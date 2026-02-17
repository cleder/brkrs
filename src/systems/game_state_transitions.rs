use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::time::Duration;

use crate::game_state::{GameSession, GameState, StateTransitionContext};
use crate::systems::level_switch::LevelSwitchState;
use crate::systems::merkaba::Merkaba;
use crate::systems::respawn::{RespawnEntityKind, RespawnHandle, SpawnPoints};
use crate::{Ball, BallTypeId, Brick, Paddle, BALL_RADIUS};

pub fn is_valid_transition(current: &GameState, target: &GameState) -> bool {
    use GameState::*;
    let valid = matches!(
        (current, target),
        (MainMenu, Playing)
            | (MainMenu, FadeOut)
            | (Playing, Paused)
            | (Playing, FadeOut)
            | (Paused, Playing)
            | (FadeOut, FadeIn)
            | (FadeOut, LevelTransition)
            | (FadeOut, GameOver)
            | (LevelTransition, FadeIn)
            | (FadeIn, Playing)
            | (GameOver, MainMenu)
    );

    if !valid {
        warn!("Invalid transition: {:?} -> {:?}", current, target);
    }

    valid
}

#[derive(Component)]
pub struct FadeTimer {
    pub timer: Timer,
}

impl FadeTimer {
    pub fn new(duration_secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration_secs, TimerMode::Once),
        }
    }

    pub fn tick(&mut self, delta: Duration) -> &Self {
        self.timer.tick(delta);
        self
    }

    pub fn finished(&self) -> bool {
        self.timer.is_finished()
    }

    pub fn fraction(&self) -> f32 {
        self.timer.fraction()
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FadeDirection {
    In,
    Out,
}

#[derive(Resource, Debug, Clone)]
pub struct RespawnBallVisuals {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

pub fn spawn_fade_out_overlay(mut commands: Commands) {
    info!(target: "game_state", "Spawning FadeOut overlay");
    commands.spawn((
        Node::default(),
        BackgroundColor(Color::BLACK),
        FadeTimer::new(0.75),
        FadeDirection::Out,
    ));
}

pub fn spawn_fade_in_overlay(mut commands: Commands) {
    info!(target: "game_state", "Spawning FadeIn overlay");
    commands.spawn((
        Node::default(),
        BackgroundColor(Color::BLACK),
        FadeTimer::new(0.75),
        FadeDirection::In,
    ));
}

pub fn update_fade_overlay(
    time: Res<Time>,
    mut query: Query<(&mut BackgroundColor, &mut FadeTimer, &FadeDirection)>,
) {
    for (mut color, mut timer, direction) in query.iter_mut() {
        timer.tick(time.delta());
        let progress = timer.fraction();
        let alpha = match direction {
            FadeDirection::Out => progress,
            FadeDirection::In => 1.0 - progress,
        };
        color.0.set_alpha(alpha);
    }
}

pub fn check_fade_out_completion(
    context: Option<Res<StateTransitionContext>>,
    mut session: ResMut<GameSession>,
    lives_state: Option<Res<crate::systems::respawn::LivesState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    query: Query<&FadeTimer>,
) {
    let finished = query.iter().all(|timer| timer.finished());
    if !finished && query.iter().next().is_some() {
        return;
    }

    let Some(ctx) = context else {
        warn!(target: "game_state", "FadeOut complete but no StateTransitionContext");
        return;
    };

    info!(target: "game_state", "FadeOut complete with context: {:?}", *ctx);

    match *ctx {
        StateTransitionContext::LifeLoss => {
            // Check the actual lives state (source of truth from respawn system),
            // falling back to GameSession when LivesState is not available (tests).
            let current_lives = lives_state
                .map(|ls| ls.lives_remaining as u32)
                .unwrap_or(session.lives_remaining);

            if current_lives > 0 {
                // Sync GameSession with LivesState
                session.lives_remaining = current_lives;
                next_state.set(GameState::FadeIn);
            } else {
                session.lives_remaining = 0;
                next_state.set(GameState::GameOver);
            }
            // Remove context after using it for LifeLoss
            commands.remove_resource::<StateTransitionContext>();
        }
        StateTransitionContext::LevelChange { target_level } => {
            info!(
                target: "game_state",
                "FadeOut->LevelTransition for level {}",
                target_level
            );
            next_state.set(GameState::LevelTransition);
        }
        _ => {}
    }

    // Don't remove StateTransitionContext here - enter_level_transition needs it
}

pub fn check_fade_in_completion(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<&FadeTimer>,
    balls: Query<Entity, With<Ball>>,
    paddles: Query<Entity, With<Paddle>>,
    spawn_points: Option<Res<SpawnPoints>>,
    ball_config: Option<Res<crate::physics_config::BallPhysicsConfig>>,
    meshes: Option<ResMut<Assets<Mesh>>>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
    visuals: Option<ResMut<RespawnBallVisuals>>,
    mut commands: Commands,
    overlay_entities: Query<Entity, With<FadeTimer>>,
) {
    let finished = query.iter().all(|timer| timer.finished());
    if !finished && query.iter().next().is_some() {
        return;
    }

    for entity in overlay_entities.iter() {
        commands.entity(entity).despawn();
    }

    info!(
        target: "game_state",
        "FadeIn complete: balls={}, paddles={}",
        balls.iter().count(),
        paddles.iter().count()
    );

    if balls.is_empty() {
        let Some(spawn_points) = spawn_points else {
            error!(
                target: "game_state",
                "Ball respawn failed: missing SpawnPoints resource. Proceeding to Playing without ball."
            );
            next_state.set(GameState::Playing);
            return;
        };
        let Some(mut meshes) = meshes else {
            error!(
                target: "game_state",
                "Ball respawn failed: missing Assets<Mesh> resource. Proceeding to Playing without ball."
            );
            next_state.set(GameState::Playing);
            return;
        };
        let Some(mut materials) = materials else {
            error!(
                target: "game_state",
                "Ball respawn failed: missing Assets<StandardMaterial> resource. Proceeding to Playing without ball."
            );
            next_state.set(GameState::Playing);
            return;
        };

        let (mesh_handle, material_handle) = if let Some(visuals) = visuals.as_ref() {
            (visuals.mesh.clone(), visuals.material.clone())
        } else {
            let mesh = meshes.add(Sphere::new(BALL_RADIUS).mesh());
            let material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.9, 0.9, 0.9),
                ..default()
            });
            commands.insert_resource(RespawnBallVisuals {
                mesh: mesh.clone(),
                material: material.clone(),
            });
            (mesh, material)
        };

        let spawn = spawn_points.ball_spawn();
        let physics = ball_config
            .as_ref()
            .map(|config| config.as_ref().clone())
            .unwrap_or_default();

        // Split spawn into two parts to avoid tuple size limits
        let ball = commands
            .spawn((
                Mesh3d(mesh_handle),
                MeshMaterial3d(material_handle),
                Transform::from_translation(spawn.translation),
                Ball,
                RigidBody::Dynamic,
                Velocity::zero(),
                CollidingEntities::default(),
                ActiveEvents::COLLISION_EVENTS,
                Collider::ball(BALL_RADIUS),
                Restitution {
                    coefficient: physics.restitution,
                    combine_rule: CoefficientCombineRule::Max,
                },
                Friction {
                    coefficient: physics.friction,
                    combine_rule: CoefficientCombineRule::Max,
                },
            ))
            .id();

        commands.entity(ball).insert((
            Damping {
                linear_damping: physics.linear_damping,
                angular_damping: physics.angular_damping,
            },
            LockedAxes::TRANSLATION_LOCKED_Y,
            Ccd::enabled(),
            ExternalImpulse::default(),
            GravityScale(1.0),
            BallTypeId(0),
            RespawnHandle {
                spawn,
                kind: RespawnEntityKind::Ball,
            },
        ));
    }

    info!(
        target: "game_state",
        "FadeIn complete, transitioning to Playing"
    );
    next_state.set(GameState::Playing);
}

#[allow(private_interfaces)]
pub fn enter_level_transition(
    context: Option<Res<StateTransitionContext>>,
    mut session: ResMut<GameSession>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    ctx: Option<crate::level_loader::LevelContext>,
    mut rapier_config: Query<&mut RapierConfiguration>,
    bricks: Query<Entity, With<Brick>>,
    paddles: Query<Entity, With<Paddle>>,
    balls: Query<Entity, With<Ball>>,
    merkabas: Query<Entity, With<Merkaba>>,
    mut pending_merkaba_spawns: Option<ResMut<crate::systems::merkaba::PendingMerkabaSpawns>>,
    #[cfg(feature = "texture_manifest")] mut tex_res: crate::level_loader::TextureResources,
    brick_config_res: Option<Res<crate::physics_config::BrickPhysicsConfig>>,
    switch_state: Option<Res<LevelSwitchState>>,
) {
    info!(target: "game_state", "enter_level_transition called");

    let Some(context) = context else {
        warn!(target: "game_state", "enter_level_transition: No StateTransitionContext");
        return;
    };

    let Some(mut ctx) = ctx else {
        warn!(target: "game_state", "enter_level_transition: No LevelContext");
        return;
    };

    let StateTransitionContext::LevelChange { target_level } = *context else {
        warn!(target: "game_state", "enter_level_transition: StateTransitionContext is not LevelChange");
        return;
    };

    let Some(brick_config_res) = brick_config_res else {
        warn!(target: "game_state", "enter_level_transition: No BrickPhysicsConfig");
        return;
    };

    info!(target: "game_state", "enter_level_transition: Loading level {}", target_level);

    let result = crate::level_loader::load_level_for_state_transition(
        target_level,
        &mut commands,
        &mut ctx,
        &mut rapier_config,
        &bricks,
        &paddles,
        &balls,
        &merkabas,
        &mut pending_merkaba_spawns,
        #[cfg(feature = "texture_manifest")]
        &mut tex_res,
        brick_config_res,
        switch_state.as_deref(),
    );

    match result {
        Ok(def) => {
            session.current_level = def.number;
            info!(
                target: "game_state",
                "Level {} loaded during state transition, setting FadeIn",
                def.number
            );
            next_state.set(GameState::FadeIn);
        }
        Err(err) => {
            warn!(target = "level_switch", "Level transition failed: {err}");
            next_state.set(GameState::FadeIn);
        }
    }

    commands.remove_resource::<StateTransitionContext>();
}

pub fn handle_life_loss_events(
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    reader: Option<MessageReader<crate::systems::respawn::LifeLostEvent>>,
) {
    let Some(mut reader) = reader else {
        return;
    };
    let mut saw_event = false;
    for _event in reader.read() {
        saw_event = true;
    }
    if !saw_event {
        return;
    }

    if is_valid_transition(current_state.get(), &GameState::FadeOut) {
        commands.insert_resource(StateTransitionContext::LifeLoss);
        next_state.set(GameState::FadeOut);
    }
}

/// Reject invalid state transitions by resetting `NextState`.
/// Covers EC-001 (fade guards) and EC-005 (invalid transition rejection).
pub fn guard_invalid_state_transitions(
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let current = *current_state.get();
    let NextState::Pending(target) = *next_state else {
        return;
    };

    if current == target {
        return;
    }

    if !validate_transition_with_logging(&current, &target) {
        warn!(
            target: "game_state",
            "Rejecting invalid state transition: {:?} -> {:?}",
            current,
            target
        );
        next_state.reset();
    }
}

pub fn despawn_hazards_on_fade_out(
    mut commands: Commands,
    balls: Query<Entity, With<Ball>>,
    merkabas: Query<Entity, With<Merkaba>>,
) {
    for entity in balls.iter() {
        commands.entity(entity).despawn();
    }
    for entity in merkabas.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn get_single_component<'a, T: Component>(query: &'a Query<'_, '_, &'a T>) -> Option<&'a T> {
    query.single().ok()
}
// ============================================================================
// EC-001: Guard against transitions during FadeOut and FadeIn
// ============================================================================

/// Validation helper to check if a transition is allowed from fade states.
/// EC-001 requires that transitions during FadeOut/FadeIn are rejected and logged.
pub fn is_transition_allowed_from_fade(current: &GameState, target: &GameState) -> bool {
    use GameState::*;
    // Only allow predefined fade transitions
    matches!(
        (current, target),
        (FadeOut, FadeIn | LevelTransition | GameOver) | (FadeIn, Playing)
    )
}

// ============================================================================
// EC-002: Ensure idempotent pause/resume transitions
// ============================================================================

/// Enhanced validation for idempotent transitions.
/// A transition is valid if:
/// 1. It's an allowed path according to the state machine, AND
/// 2. It's not a self-loop (same state transition) for idempotent states
pub fn is_valid_transition_idempotent(current: &GameState, target: &GameState) -> bool {
    use GameState::*;

    // Idempotent transitions (same state is always a no-op, not an error)
    if current == target {
        return true;
    }

    // Allowed state machine transitions
    let valid = matches!(
        (current, target),
        (MainMenu, Playing)
            | (MainMenu, FadeOut)
            | (Playing, Paused)
            | (Playing, FadeOut)
            | (Paused, Playing)
            | (FadeOut, FadeIn)
            | (FadeOut, LevelTransition)
            | (FadeOut, GameOver)
            | (LevelTransition, FadeIn)
            | (FadeIn, Playing)
            | (GameOver, MainMenu)
    );

    if !valid {
        error!(
            target: "game_state",
            "Invalid state transition: {:?} -> {:?}. Valid transitions from {:?}: MainMenu->Playing/FadeOut, Playing->Paused/FadeOut, Paused->Playing, FadeOut->FadeIn/LevelTransition/GameOver, LevelTransition->FadeIn, FadeIn->Playing, GameOver->MainMenu",
            current, target, current
        );
    }

    valid
}

// ============================================================================
// EC-003: Deferred level-complete while Paused
// ============================================================================

#[derive(Resource, Default, Debug, Clone)]
pub struct DeferredLevelChange {
    pub pending: bool,
    pub target_level: u32,
}

/// Capture level-change requests while Paused for later execution.
pub fn capture_deferred_level_change(
    state: Res<State<GameState>>,
    context: Option<Res<StateTransitionContext>>,
    mut deferred: ResMut<DeferredLevelChange>,
    mut commands: Commands,
) {
    if *state.get() != GameState::Paused {
        return;
    }

    let Some(context) = context else {
        return;
    };

    let StateTransitionContext::LevelChange { target_level } = *context else {
        return;
    };

    deferred.pending = true;
    deferred.target_level = target_level;
    commands.remove_resource::<StateTransitionContext>();
}

/// Handle deferred level-complete when resuming from Paused.
/// If a level-complete was triggered while paused, initiate FadeOut now.
pub fn handle_deferred_level_change(
    state: Res<State<GameState>>,
    context: Option<Res<StateTransitionContext>>,
    mut deferred: ResMut<DeferredLevelChange>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    if *state.get() != GameState::Playing {
        return;
    }

    let mut target_level = None;
    let mut needs_context_insert = false;

    if deferred.pending {
        target_level = Some(deferred.target_level);
        needs_context_insert = true;
        deferred.pending = false;
    } else if let Some(context) = context {
        if let StateTransitionContext::LevelChange {
            target_level: pending,
        } = *context
        {
            target_level = Some(pending);
        }
    }

    let Some(target_level) = target_level else {
        return;
    };

    info!(
        target: "game_state",
        "Resuming from Paused with deferred level change to {}; initiating FadeOut",
        target_level
    );

    if needs_context_insert {
        commands.insert_resource(StateTransitionContext::LevelChange { target_level });
    }
    next_state.set(GameState::FadeOut);
}

// ============================================================================
// EC-004: Entity cleanup on level transition
// ============================================================================

/// Clean up level-specific entities when exiting LevelTransition state.
/// This ensures no orphaned entities remain from the previous level.
pub fn cleanup_level_entities_on_transition(
    mut commands: Commands,
    bricks: Query<Entity, With<Brick>>,
    balls: Query<Entity, With<Ball>>,
    merkabas: Query<Entity, With<Merkaba>>,
) {
    // Capture entity counts BEFORE despawning for accurate logging
    let brick_count = bricks.iter().count();
    let ball_count = balls.iter().count();
    let merkaba_count = merkabas.iter().count();

    // Despawn all bricks from the previous level
    for entity in bricks.iter() {
        commands.entity(entity).despawn();
    }

    // Despawn all balls
    for entity in balls.iter() {
        commands.entity(entity).despawn();
    }

    // Despawn all merkabas
    for entity in merkabas.iter() {
        commands.entity(entity).despawn();
    }

    info!(
        target: "game_state",
        "Cleaned up {} bricks, {} balls, {} merkabas during level transition",
        brick_count,
        ball_count,
        merkaba_count
    );
}

// ============================================================================
// EC-005: Invalid transition rejection with detailed error logs
// ============================================================================

/// Enhanced validation helper for state transitions with detailed error logging.
/// Checks if a transition is valid and logs detailed error messages if not.
pub fn validate_transition_with_logging(current: &GameState, target: &GameState) -> bool {
    let valid = is_valid_transition_idempotent(current, target);

    if !valid {
        error!(
            target: "game_state",
            source_state = ?current,
            target_state = ?target,
            "Rejecting invalid state transition"
        );
    }

    valid
}
