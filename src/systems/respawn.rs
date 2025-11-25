use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    Ball, BallFrozen, LowerGoal, Paddle, PaddleGrowing, BALL_RADIUS, PADDLE_GROWTH_DURATION,
    PADDLE_HEIGHT, PADDLE_RADIUS,
};

/// Tracks pending respawn operations and their timer state.
#[derive(Resource)]
pub struct RespawnState {
    pub timer: Timer,
    pub active: bool,
    pub last_lost_ball: Option<Entity>,
}

impl Default for RespawnState {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            active: false,
            last_lost_ball: None,
        }
    }
}

/// Cached spawn transforms populated during level loading.
#[derive(Resource, Default)]
pub struct InitialPositions {
    pub paddle_pos: Option<Vec3>,
    pub ball_pos: Option<Vec3>,
}

/// Plugin wiring the respawn flow through ordered system sets.
pub struct RespawnPlugin;

#[derive(Event, Debug, Clone, Copy)]
pub struct LifeLostEvent {
    pub ball: Entity,
}

/// Marker component used to disable paddle input while respawn settles.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct InputLocked;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RespawnSystems {
    Detect,
    Schedule,
    Execute,
    Control,
}

impl Plugin for RespawnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RespawnState>()
            .init_resource::<InitialPositions>()
            .add_event::<LifeLostEvent>()
            .configure_sets(
                Update,
                (
                    RespawnSystems::Detect,
                    RespawnSystems::Schedule,
                    RespawnSystems::Execute,
                    RespawnSystems::Control,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    detect_ball_loss.in_set(RespawnSystems::Detect),
                    schedule_respawn_timer.in_set(RespawnSystems::Schedule),
                    legacy_respawn_executor.in_set(RespawnSystems::Execute),
                    restore_paddle_control.in_set(RespawnSystems::Control),
                ),
            );
    }
}

fn detect_ball_loss(
    mut collision_events: EventReader<CollisionEvent>,
    balls: Query<Entity, With<Ball>>,
    lower_goals: Query<Entity, With<LowerGoal>>,
    paddles: Query<Entity, With<Paddle>>,
    mut commands: Commands,
    mut life_lost_events: EventWriter<LifeLostEvent>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let e1_is_ball = balls.get(*e1).is_ok();
            let e2_is_ball = balls.get(*e2).is_ok();
            let e1_is_lower = lower_goals.get(*e1).is_ok();
            let e2_is_lower = lower_goals.get(*e2).is_ok();

            if (e1_is_ball && e2_is_lower) || (e2_is_ball && e1_is_lower) {
                let ball_entity = if e1_is_ball { *e1 } else { *e2 };

                life_lost_events.write(LifeLostEvent { ball: ball_entity });
                commands.entity(ball_entity).despawn();

                for paddle in paddles.iter() {
                    commands.entity(paddle).despawn();
                }
            }
        }
    }
}

fn schedule_respawn_timer(
    mut respawn_state: ResMut<RespawnState>,
    mut events: EventReader<LifeLostEvent>,
) {
    let mut triggered = false;

    for event in events.read() {
        respawn_state.last_lost_ball = Some(event.ball);
        triggered = true;
    }

    if triggered {
        respawn_state.timer.reset();
        respawn_state.active = true;
    }
}

fn legacy_respawn_executor(
    time: Res<Time>,
    mut respawn_state: ResMut<RespawnState>,
    initial_positions: Res<InitialPositions>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    if !respawn_state.active {
        return;
    }

    respawn_state.timer.tick(time.delta());

    if respawn_state.timer.finished() {
        respawn_state.active = false;

        let debug_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            unlit: false,
            ..default()
        });

        if let Some(paddle_pos) = initial_positions.paddle_pos {
            commands
                .spawn((
                    Mesh3d(meshes.add(Capsule3d::new(PADDLE_RADIUS, PADDLE_HEIGHT).mesh())),
                    MeshMaterial3d(debug_material.clone()),
                    Transform::from_xyz(paddle_pos.x, paddle_pos.y, paddle_pos.z)
                        .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0))
                        .with_scale(Vec3::splat(0.01)),
                    Paddle,
                    PaddleGrowing {
                        timer: Timer::from_seconds(PADDLE_GROWTH_DURATION, TimerMode::Once),
                        target_scale: Vec3::ONE,
                    },
                    InputLocked,
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
                });
        }

        if let Some(ball_pos) = initial_positions.ball_pos {
            commands
                .spawn((
                    Mesh3d(meshes.add(Sphere::new(BALL_RADIUS).mesh())),
                    MeshMaterial3d(debug_material.clone()),
                    Transform::from_xyz(ball_pos.x, ball_pos.y, ball_pos.z),
                    Ball,
                    BallFrozen,
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
                ));
        }
    }
}

fn restore_paddle_control(
    respawn_state: Res<RespawnState>,
    mut paddles: Query<(Entity, Option<&PaddleGrowing>), (With<Paddle>, With<InputLocked>)>,
    mut commands: Commands,
) {
    if respawn_state.active {
        return;
    }

    for (entity, maybe_growing) in paddles.iter_mut() {
        if maybe_growing.is_none() {
            commands.entity(entity).remove::<InputLocked>();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy::ecs::event::Events;
    use bevy::time::Time;
    use bevy::MinimalPlugins;
    use bevy_rapier3d::prelude::CollisionEvent;
    use bevy_rapier3d::rapier::prelude::CollisionEventFlags;
    use std::time::Duration;

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(Assets::<Mesh>::default())
            .insert_resource(Assets::<StandardMaterial>::default())
            .add_event::<CollisionEvent>()
            .add_plugins(RespawnPlugin);
        app
    }

    fn advance_time(app: &mut App, delta_secs: f32) {
        let mut time = app.world_mut().resource_mut::<Time>();
        time.advance_by(Duration::from_secs_f32(delta_secs));
    }

    #[test]
    fn collision_triggers_respawn_state() {
        let mut app = test_app();

        let ball = app.world_mut().spawn(Ball).id();
        let lower_goal = app.world_mut().spawn(LowerGoal).id();
        let paddle = app.world_mut().spawn(Paddle).id();

        app.world_mut()
            .resource_mut::<Events<CollisionEvent>>()
            .send(CollisionEvent::Started(
                ball,
                lower_goal,
                CollisionEventFlags::SENSOR,
            ));

        advance_time(&mut app, 0.016);
        app.update();

        let respawn_state = app.world().resource::<RespawnState>();
        assert!(respawn_state.active);
        assert_eq!(respawn_state.last_lost_ball, Some(ball));
        let world = app.world();
        assert!(!world.entities().contains(ball));
        assert!(!world.entities().contains(paddle));
    }

    #[test]
    fn executor_spawns_entities_after_delay() {
        let mut app = test_app();

        {
            let mut positions = app.world_mut().resource_mut::<InitialPositions>();
            positions.ball_pos = Some(Vec3::new(1.0, 2.0, 3.0));
            positions.paddle_pos = Some(Vec3::new(-1.0, 2.0, 0.0));
        }

        {
            let mut state = app.world_mut().resource_mut::<RespawnState>();
            state.active = true;
            state.timer.reset();
            let duration = state.timer.duration();
            state
                .timer
                .tick(Duration::from_secs_f32(duration.as_secs_f32() + 0.1));
        }

        app.update();

        let world = app.world();
        let ball_count = world
            .iter_entities()
            .filter(|entity| entity.contains::<Ball>())
            .count();
        assert!(ball_count > 0);

        let paddle_count = world
            .iter_entities()
            .filter(|entity| entity.contains::<Paddle>())
            .count();
        assert!(paddle_count > 0);
    }

    #[test]
    fn control_stage_unlocks_paddle_after_growth() {
        let mut app = test_app();

        let paddle = app.world_mut().spawn((Paddle, InputLocked)).id();

        app.update();

        let world = app.world();
        assert!(!world.entity(paddle).contains::<InputLocked>());
    }

    #[test]
    fn control_stage_waits_for_growth_or_timer() {
        let mut app = test_app();

        {
            let mut state = app.world_mut().resource_mut::<RespawnState>();
            state.active = true;
        }

        let paddle = app
            .world_mut()
            .spawn((
                Paddle,
                InputLocked,
                PaddleGrowing {
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                    target_scale: Vec3::ONE,
                },
            ))
            .id();

        app.update();

        let world = app.world();
        assert!(world.entity(paddle).contains::<InputLocked>());
    }
}
