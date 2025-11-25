use crate::systems::respawn::{RespawnEntityKind, RespawnHandle, SpawnPoints, SpawnTransform};
use bevy::prelude::*;
use ron::de::from_str;
use serde::Deserialize;

use crate::{
    Ball, Brick, GameProgress, GravityConfig, Paddle, BALL_RADIUS, CELL_HEIGHT, CELL_WIDTH,
    PADDLE_HEIGHT, PADDLE_RADIUS, PLANE_H, PLANE_W,
};
use bevy_rapier3d::prelude::*;

#[derive(Deserialize, Debug)]
pub struct LevelDefinition {
    pub number: u32,
    /// Optional gravity override for this level (x,y,z). If omitted the existing GravityConfig value is used.
    pub gravity: Option<(f32, f32, f32)>,
    pub matrix: Vec<Vec<u8>>, // expect 22 x 22
}

#[derive(Resource, Debug)]
pub struct CurrentLevel(pub LevelDefinition);

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_level, spawn_level_entities).chain());
        app.add_systems(
            Update,
            (
                advance_level_when_cleared,
                handle_level_advance_delay,
                finalize_level_advance,
                spawn_fade_overlay_if_needed,
                update_fade_overlay,
                restart_level_on_key,
            ),
        );
    }
}

/// State machine for delayed level advancement and growth animation.
#[derive(Resource)]
pub struct LevelAdvanceState {
    pub timer: Timer,                     // initial delay before spawning growth paddle
    pub active: bool,                     // transition in progress
    pub growth_spawned: bool,             // tiny paddle+ball spawned, waiting for growth completion
    pub pending: Option<LevelDefinition>, // next level definition awaiting brick spawn
}

impl Default for LevelAdvanceState {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            active: false,
            growth_spawned: false,
            pending: None,
        }
    }
}

/// Full-screen UI overlay used for fade in/out during level transitions.
#[derive(Component)]
struct FadeOverlay;

fn paddle_spawn_transform(position: Vec3) -> SpawnTransform {
    SpawnTransform::new(position, Quat::from_rotation_x(-std::f32::consts::PI / 2.0))
}

fn ball_spawn_transform(position: Vec3) -> SpawnTransform {
    SpawnTransform::new(position, Quat::IDENTITY)
}

fn paddle_respawn_handle(position: Vec3) -> RespawnHandle {
    RespawnHandle {
        spawn: paddle_spawn_transform(position),
        kind: RespawnEntityKind::Paddle,
    }
}

fn ball_respawn_handle(position: Vec3) -> RespawnHandle {
    RespawnHandle {
        spawn: ball_spawn_transform(position),
        kind: RespawnEntityKind::Ball,
    }
}

fn load_level(
    mut commands: Commands,
    mut gravity_cfg: ResMut<GravityConfig>,
    mut rapier_config: Query<&mut RapierConfiguration>,
) {
    #[cfg(not(target_arch = "wasm32"))]
    let chosen_path = {
        use std::env;
        if let Ok(num) = env::var("BK_LEVEL") {
            let trimmed = num.trim();
            if let Ok(n) = trimmed.parse::<u32>() {
                format!("assets/levels/level_{:03}.ron", n)
            } else {
                warn!("BK_LEVEL='{}' not a number; defaulting to level_001", num);
                "assets/levels/level_001.ron".to_string()
            }
        } else {
            "assets/levels/level_001.ron".to_string()
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    let level_str: &str = match std::fs::read_to_string(&chosen_path) {
        Ok(s) => {
            info!("Loading level file: {}", chosen_path);
            Box::leak(s.into_boxed_str())
        }
        Err(e) => {
            warn!(
                "Failed to read level file '{}': {e}. Falling back to empty level",
                chosen_path
            );
            "LevelDefinition(number:0,matrix:[])"
        }
    };
    #[cfg(target_arch = "wasm32")]
    let level_str: &str = include_str!("../assets/levels/level_001.ron");

    match from_str::<LevelDefinition>(level_str) {
        Ok(def) => {
            // basic validation
            if def.matrix.len() != 22 || def.matrix.iter().any(|r| r.len() != 22) {
                warn!("Level matrix wrong dimensions; expected 22x22");
            }
            info!("Loaded level {}", def.number);
            // Apply per-level gravity if present
            if let Some((x, y, z)) = def.gravity {
                gravity_cfg.normal = Vec3::new(x, y, z);
                if let Ok(mut config) = rapier_config.single_mut() {
                    config.gravity = gravity_cfg.normal;
                }
                info!("Level gravity set to {:?}", gravity_cfg.normal);
            } else {
                debug!(
                    "Level has no gravity override; using existing {:?}",
                    gravity_cfg.normal
                );
            }
            commands.insert_resource(CurrentLevel(def));
        }
        Err(e) => {
            warn!("Failed to parse level: {e}");
        }
    }
}

fn spawn_level_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_points: ResMut<SpawnPoints>,
    level: Option<Res<CurrentLevel>>,
) {
    let Some(level) = level else {
        return;
    };
    spawn_level_entities_impl(
        &level.0,
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut spawn_points,
    );
}

fn spawn_level_entities_impl(
    def: &LevelDefinition,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    spawn_points: &mut ResMut<SpawnPoints>,
) {
    debug!("Spawning entities for level {}", def.number);
    // Shared material
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        unlit: false,
        ..default()
    });

    let mut paddle_spawned = false;
    let mut ball_spawned = false;

    spawn_points.paddle = None;
    spawn_points.ball = None;

    for (row, row_data) in def.matrix.iter().enumerate() {
        for (col, value) in row_data.iter().enumerate() {
            let x = -PLANE_H / 2.0 + (row as f32 + 0.5) * CELL_HEIGHT;
            let z = -PLANE_W / 2.0 + (col as f32 + 0.5) * CELL_WIDTH;
            match value {
                0 => {}
                1 => {
                    // Paddle
                    if !paddle_spawned {
                        paddle_spawned = true;
                        let position = Vec3::new(x, 2.0, z);
                        spawn_points.paddle = Some(position);
                        commands
                            .spawn((
                                Mesh3d(
                                    meshes.add(Capsule3d::new(PADDLE_RADIUS, PADDLE_HEIGHT).mesh()),
                                ),
                                MeshMaterial3d(debug_material.clone()),
                                Transform::from_xyz(x, 2.0, z).with_rotation(
                                    Quat::from_rotation_x(-std::f32::consts::PI / 2.0),
                                ),
                                Paddle,
                                RigidBody::KinematicPositionBased,
                                GravityScale(0.0),
                                CollidingEntities::default(),
                                Collider::capsule_y(PADDLE_HEIGHT / 2.0, PADDLE_RADIUS),
                                LockedAxes::TRANSLATION_LOCKED_Y,
                                KinematicCharacterController::default(),
                                Ccd::enabled(),
                                Friction {
                                    coefficient: 2.0,
                                    combine_rule: CoefficientCombineRule::Max,
                                },
                            ))
                            .insert(paddle_respawn_handle(position));
                    }
                }
                2 => {
                    // Ball
                    if !ball_spawned {
                        ball_spawned = true;
                        let position = Vec3::new(x, 2.0, z);
                        spawn_points.ball = Some(position);
                        commands
                            .spawn((
                                Mesh3d(meshes.add(Sphere::new(BALL_RADIUS).mesh())),
                                MeshMaterial3d(debug_material.clone()),
                                Transform::from_xyz(x, 2.0, z),
                                Ball,
                                RigidBody::Dynamic,
                                CollidingEntities::default(),
                                ActiveEvents::COLLISION_EVENTS,
                                Collider::ball(BALL_RADIUS),
                                Restitution {
                                    coefficient: 0.9,
                                    combine_rule: CoefficientCombineRule::Max,
                                },
                                Friction {
                                    coefficient: 2.0,
                                    combine_rule: CoefficientCombineRule::Max,
                                },
                                Damping {
                                    linear_damping: 0.5,
                                    angular_damping: 0.5,
                                },
                            ))
                            .insert((
                                LockedAxes::TRANSLATION_LOCKED_Y,
                                Ccd::enabled(),
                                ExternalImpulse::default(),
                                GravityScale(1.0),
                            ))
                            .insert(ball_respawn_handle(position));
                    }
                }
                3 => {
                    // Brick
                    commands.spawn((
                        Mesh3d(meshes.add(Cuboid::new(CELL_HEIGHT * 0.9, 0.5, CELL_WIDTH * 0.9))),
                        MeshMaterial3d(materials.add(Color::srgb(0.9, 0.1, 0.1))),
                        Transform::from_xyz(x, 2.0, z),
                        Brick,
                        RigidBody::Fixed,
                        Collider::cuboid(CELL_HEIGHT * 0.9 / 2.0, 0.25, CELL_WIDTH * 0.9 / 2.0),
                        Restitution {
                            coefficient: 1.0,
                            combine_rule: CoefficientCombineRule::Max,
                        },
                        CollidingEntities::default(),
                        ActiveEvents::COLLISION_EVENTS,
                    ));
                }
                _ => {
                    warn!("Unsupported cell value {value} at ({row},{col})");
                }
            }
        }
    }

    if !paddle_spawned {
        warn!("No paddle found in level matrix; spawning fallback paddle.");
        let x = 0.0;
        let z = 0.0;
        let position = Vec3::new(x, 2.0, z);
        spawn_points.paddle = Some(position);
        commands
            .spawn((
                Mesh3d(meshes.add(Capsule3d::new(PADDLE_RADIUS, PADDLE_HEIGHT).mesh())),
                MeshMaterial3d(debug_material.clone()),
                Transform::from_xyz(x, 2.0, z)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
                Paddle,
                RigidBody::KinematicPositionBased,
                GravityScale(0.0),
                CollidingEntities::default(),
                Collider::capsule_y(PADDLE_HEIGHT / 2.0, PADDLE_RADIUS),
                LockedAxes::TRANSLATION_LOCKED_Y,
                KinematicCharacterController::default(),
                Ccd::enabled(),
                Friction {
                    coefficient: 2.0,
                    combine_rule: CoefficientCombineRule::Max,
                },
            ))
            .insert(paddle_respawn_handle(position));
    }
    if !ball_spawned {
        warn!("No ball found in level matrix; spawning fallback ball.");
        let position = Vec3::new(0.0, 2.0, 0.0);
        spawn_points.ball = Some(position);
        commands
            .spawn((
                Mesh3d(meshes.add(Sphere::new(BALL_RADIUS).mesh())),
                MeshMaterial3d(debug_material.clone()),
                Transform::from_xyz(0.0, 2.0, 0.0),
                Ball,
                RigidBody::Dynamic,
                CollidingEntities::default(),
                ActiveEvents::COLLISION_EVENTS,
                Collider::ball(BALL_RADIUS),
                Restitution {
                    coefficient: 0.9,
                    combine_rule: CoefficientCombineRule::Max,
                },
                Friction {
                    coefficient: 2.0,
                    combine_rule: CoefficientCombineRule::Max,
                },
                Damping {
                    linear_damping: 0.5,
                    angular_damping: 0.5,
                },
            ))
            .insert((
                LockedAxes::TRANSLATION_LOCKED_Y,
                Ccd::enabled(),
                ExternalImpulse::default(),
                GravityScale(1.0),
            ))
            .insert(ball_respawn_handle(position));
    }
}

/// Only spawn bricks for a level (used after paddle growth completes during level advance).
fn spawn_bricks_only(
    def: &LevelDefinition,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    for (row, row_data) in def.matrix.iter().enumerate() {
        for (col, value) in row_data.iter().enumerate() {
            if *value != 3 {
                continue;
            }
            let x = -PLANE_H / 2.0 + (row as f32 + 0.5) * CELL_HEIGHT;
            let z = -PLANE_W / 2.0 + (col as f32 + 0.5) * CELL_WIDTH;
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(CELL_HEIGHT * 0.9, 0.5, CELL_WIDTH * 0.9))),
                MeshMaterial3d(materials.add(Color::srgb(0.9, 0.1, 0.1))),
                Transform::from_xyz(x, 2.0, z),
                Brick,
                RigidBody::Fixed,
                Collider::cuboid(CELL_HEIGHT * 0.9 / 2.0, 0.25, CELL_WIDTH * 0.9 / 2.0),
                Restitution {
                    coefficient: 1.0,
                    combine_rule: CoefficientCombineRule::Max,
                },
                CollidingEntities::default(),
                ActiveEvents::COLLISION_EVENTS,
            ));
        }
    }
}

/// Extract and set spawn points for paddle & ball from a level definition (without spawning bricks).
fn set_spawn_points_only(def: &LevelDefinition, spawn_points: &mut ResMut<SpawnPoints>) {
    spawn_points.paddle = None;
    spawn_points.ball = None;
    let mut paddle_set = false;
    let mut ball_set = false;
    for (row, row_data) in def.matrix.iter().enumerate() {
        for (col, value) in row_data.iter().enumerate() {
            let x = -PLANE_H / 2.0 + (row as f32 + 0.5) * CELL_HEIGHT;
            let z = -PLANE_W / 2.0 + (col as f32 + 0.5) * CELL_WIDTH;
            match value {
                1 if !paddle_set => {
                    paddle_set = true;
                    spawn_points.paddle = Some(Vec3::new(x, 2.0, z));
                }
                2 if !ball_set => {
                    ball_set = true;
                    spawn_points.ball = Some(Vec3::new(x, 2.0, z));
                }
                _ => {}
            }
        }
    }
    if spawn_points.paddle.is_none() {
        spawn_points.paddle = Some(Vec3::new(0.0, 2.0, 0.0));
    }
    if spawn_points.ball.is_none() {
        spawn_points.ball = Some(Vec3::new(0.0, 2.0, 0.0));
    }
}

/// Advance to the next level when all bricks have been cleared.
fn advance_level_when_cleared(
    bricks: Query<Entity, With<Brick>>,
    paddle_q: Query<Entity, With<Paddle>>,
    ball_q: Query<Entity, With<Ball>>,
    current_level: Option<Res<CurrentLevel>>,
    mut commands: Commands,
    mut game_progress: ResMut<GameProgress>,
    mut level_advance: ResMut<LevelAdvanceState>,
) {
    let Some(curr) = current_level else {
        return;
    };
    if !bricks.is_empty() {
        return; // still bricks remaining
    }
    // If already transitioning, don't restart it.
    if level_advance.active {
        return;
    }
    let next_number = curr.0.number + 1;
    let path = format!("assets/levels/level_{:03}.ron", next_number);
    if !std::path::Path::new(&path).exists() {
        if !game_progress.finished {
            info!(
                "All bricks cleared; no next level file {}. Game complete.",
                path
            );
            game_progress.finished = true;
            // Despawn remaining paddle and ball to freeze gameplay
            for p in paddle_q.iter() {
                commands.entity(p).despawn();
            }
            for b in ball_q.iter() {
                commands.entity(b).despawn();
            }
            // Spawn completion text (desktop only UI style similar to wireframe text)
            #[cfg(not(target_arch = "wasm32"))]
            commands.spawn((
                Text::new("GAME COMPLETE - Press Q to Quit"),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(60.0),
                    left: Val::Px(60.0),
                    ..default()
                },
            ));
        }
        return;
    }
    // Parse and store next level; delay spawning via LevelAdvanceState.
    match std::fs::read_to_string(&path) {
        Ok(content) => match from_str::<LevelDefinition>(&content) {
            Ok(def) => {
                info!("Preparing advancement to level {} (delayed)", def.number);
                level_advance.timer.reset();
                level_advance.active = true;
                level_advance.growth_spawned = false;
                level_advance.pending = Some(def);
                // Despawn paddle & ball now to show empty field during delay.
                for p in paddle_q.iter() {
                    commands.entity(p).despawn();
                }
                for b in ball_q.iter() {
                    commands.entity(b).despawn();
                }
            }
            Err(e) => warn!("Failed to parse next level '{}': {e}", path),
        },
        Err(e) => warn!("Failed to read next level file '{}': {e}", path),
    }
}

/// Restart the current level when the user presses R.
fn restart_level_on_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_level: Option<Res<CurrentLevel>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_points: ResMut<SpawnPoints>,
    mut gravity_cfg: ResMut<GravityConfig>,
    mut rapier_config: Query<&mut RapierConfiguration>,
    bricks: Query<Entity, With<Brick>>,
    paddle_q: Query<Entity, With<Paddle>>,
    ball_q: Query<Entity, With<Ball>>,
    mut game_progress: ResMut<GameProgress>,
    mut level_advance: ResMut<LevelAdvanceState>,
) {
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }
    let level_number = current_level.map(|cl| cl.0.number).unwrap_or(1);
    let path = format!("assets/levels/level_{:03}.ron", level_number);
    info!("Restarting level {} from {}", level_number, path);

    // Despawn existing entities
    for e in bricks.iter() {
        commands.entity(e).despawn();
    }
    for e in paddle_q.iter() {
        commands.entity(e).despawn();
    }
    for e in ball_q.iter() {
        commands.entity(e).despawn();
    }

    // Reset progress if we were finished
    game_progress.finished = false;
    level_advance.active = false;
    level_advance.pending = None;
    level_advance.growth_spawned = false;

    // Reset spawn points
    spawn_points.paddle = None;
    spawn_points.ball = None;

    match std::fs::read_to_string(&path) {
        Ok(content) => match from_str::<LevelDefinition>(&content) {
            Ok(def) => {
                // Apply gravity override
                if let Some((x, y, z)) = def.gravity {
                    gravity_cfg.normal = Vec3::new(x, y, z);
                    if let Ok(mut config) = rapier_config.single_mut() {
                        config.gravity = gravity_cfg.normal;
                    }
                    info!("Level gravity set to {:?}", gravity_cfg.normal);
                } else {
                    // Ensure rapier gravity matches current stored config
                    if let Ok(mut config) = rapier_config.single_mut() {
                        config.gravity = gravity_cfg.normal;
                    }
                }
                // Spawn entities then update resource
                spawn_level_entities_impl(
                    &def,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut spawn_points,
                );
                commands.insert_resource(CurrentLevel(def));
            }
            Err(e) => warn!("Failed to parse level on restart '{}': {e}", path),
        },
        Err(e) => warn!("Failed to read level file on restart '{}': {e}", path),
    }
}

/// After delay, spawn tiny paddle + frozen ball for growth animation (similar to respawn), defer bricks.
fn handle_level_advance_delay(
    time: Res<Time>,
    mut level_advance: ResMut<LevelAdvanceState>,
    mut spawn_points: ResMut<SpawnPoints>,
    mut gravity_cfg: ResMut<GravityConfig>,
    mut rapier_config: Query<&mut RapierConfiguration>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !level_advance.active || level_advance.pending.is_none() || level_advance.growth_spawned {
        return;
    }
    level_advance.timer.tick(time.delta());
    if !level_advance.timer.finished() {
        return;
    }
    let def = level_advance.pending.as_ref().unwrap();
    // Apply per-level gravity (or keep current) immediately so it is ready when the ball unfreezes.
    let target_gravity = if let Some((x, y, z)) = def.gravity {
        let vec = Vec3::new(x, y, z);
        gravity_cfg.normal = vec;
        vec
    } else {
        gravity_cfg.normal
    };
    if let Ok(mut config) = rapier_config.single_mut() {
        config.gravity = target_gravity;
    }
    // Set initial positions (used by spawn below and later systems).
    set_spawn_points_only(def, &mut spawn_points);
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        unlit: false,
        ..default()
    });
    if let Some(paddle_pos) = spawn_points.paddle {
        commands
            .spawn((
                Mesh3d(meshes.add(Capsule3d::new(PADDLE_RADIUS, PADDLE_HEIGHT).mesh())),
                MeshMaterial3d(debug_material.clone()),
                Transform::from_xyz(paddle_pos.x, paddle_pos.y, paddle_pos.z)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0))
                    .with_scale(Vec3::splat(0.01)),
                Paddle,
                crate::PaddleGrowing {
                    timer: Timer::from_seconds(crate::PADDLE_GROWTH_DURATION, TimerMode::Once),
                    target_scale: Vec3::ONE,
                },
                RigidBody::KinematicPositionBased,
                GravityScale(0.0),
                CollidingEntities::default(),
                Collider::capsule_y(PADDLE_HEIGHT / 2.0, PADDLE_RADIUS),
                LockedAxes::TRANSLATION_LOCKED_Y,
                KinematicCharacterController::default(),
                Ccd::enabled(),
            ))
            .insert(Friction {
                coefficient: 2.0,
                combine_rule: CoefficientCombineRule::Max,
            })
            .insert(paddle_respawn_handle(paddle_pos));
    }
    if let Some(ball_pos) = spawn_points.ball {
        commands
            .spawn((
                Mesh3d(meshes.add(Sphere::new(BALL_RADIUS).mesh())),
                MeshMaterial3d(debug_material.clone()),
                Transform::from_xyz(ball_pos.x, ball_pos.y, ball_pos.z),
                Ball,
                crate::BallFrozen,
                RigidBody::Dynamic,
                Velocity::zero(),
                CollidingEntities::default(),
                ActiveEvents::COLLISION_EVENTS,
                Collider::ball(BALL_RADIUS),
                Restitution {
                    coefficient: 0.9,
                    combine_rule: CoefficientCombineRule::Max,
                },
                Friction {
                    coefficient: 2.0,
                    combine_rule: CoefficientCombineRule::Max,
                },
                Damping {
                    linear_damping: 0.5,
                    angular_damping: 0.5,
                },
            ))
            .insert((
                LockedAxes::TRANSLATION_LOCKED_Y,
                Ccd::enabled(),
                ExternalImpulse::default(),
                GravityScale(1.0),
            ))
            .insert(ball_respawn_handle(ball_pos));
    }
    level_advance.growth_spawned = true;
}

/// After paddle growth ends, spawn bricks and finalize level change.
fn finalize_level_advance(
    paddles_growing: Query<&crate::PaddleGrowing>,
    mut level_advance: ResMut<LevelAdvanceState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: Query<&mut RapierConfiguration>,
    gravity_cfg: Res<GravityConfig>,
) {
    if !level_advance.active
        || !level_advance.growth_spawned
        || level_advance.pending.is_none()
        || !paddles_growing.is_empty()
    {
        return;
    }
    let def = level_advance.pending.take().unwrap();
    // Spawn bricks now.
    spawn_bricks_only(&def, &mut commands, &mut meshes, &mut materials);
    // Restore gravity to new level's normal.
    if let Ok(mut config) = rapier_config.single_mut() {
        config.gravity = gravity_cfg.normal;
    }
    // Update CurrentLevel resource.
    commands.insert_resource(CurrentLevel(def));
    // Reset state.
    level_advance.active = false;
    level_advance.growth_spawned = false;
}

/// Spawn fade overlay once when level advancement begins.
fn spawn_fade_overlay_if_needed(
    level_advance: Res<LevelAdvanceState>,
    existing: Query<Entity, With<FadeOverlay>>,
    mut commands: Commands,
) {
    if !level_advance.active || level_advance.pending.is_none() || !existing.is_empty() {
        return;
    }
    // Spawn a full-screen black Node with 0 alpha; animated by update_fade_overlay.
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        FadeOverlay,
    ));
}

/// Animate fade overlay alpha based on delay (fade in) and paddle growth (fade out).
fn update_fade_overlay(
    level_advance: Res<LevelAdvanceState>,
    mut overlays: Query<(Entity, &mut BackgroundColor), With<FadeOverlay>>,
    paddles_growing: Query<&crate::PaddleGrowing>,
    mut commands: Commands,
) {
    if let Ok((entity, mut color)) = overlays.single_mut() {
        if !level_advance.active {
            commands.entity(entity).despawn();
            return;
        }
        if !level_advance.growth_spawned {
            let alpha: f32 = level_advance.timer.fraction();
            let a = alpha.clamp(0.0, 1.0);
            color.0 = Color::srgba(0.0, 0.0, 0.0, a);
        } else if let Ok(growing) = paddles_growing.single() {
            let progress: f32 = growing.timer.fraction();
            let alpha: f32 = 1.0 - progress;
            let a = alpha.clamp(0.0, 1.0);
            color.0 = Color::srgba(0.0, 0.0, 0.0, a);
        } else {
            // Growth finished; remove overlay.
            commands.entity(entity).despawn();
        }
    }
}
