use bevy::log::{info, warn};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::{collections::VecDeque, f32::consts::PI, time::Duration};

use crate::{
    Ball, BallFrozen, LowerGoal, Paddle, PaddleGrowing, BALL_RADIUS, PADDLE_GROWTH_DURATION,
    PADDLE_HEIGHT, PADDLE_RADIUS,
};

/// Shared lives resource maintained by the lives system.
#[derive(Resource, Debug, Clone, Copy)]
pub struct LivesState {
    pub lives_remaining: u8,
    #[allow(dead_code)]
    pub on_last_life: bool,
}

#[cfg(test)]
#[path = "../../tests/common/multi_respawn.rs"]
mod multi_respawn;

impl Default for LivesState {
    fn default() -> Self {
        Self {
            lives_remaining: 3,
            on_last_life: false,
        }
    }
}

/// Tracks pending respawn operations and their timer state.
#[derive(Resource)]
pub struct RespawnSchedule {
    pub timer: Timer,
    pub pending: Option<RespawnRequest>,
    pub queue: VecDeque<RespawnRequest>,
    pub last_loss: Option<Duration>,
}

impl Default for RespawnSchedule {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            pending: None,
            queue: VecDeque::new(),
            last_loss: None,
        }
    }
}

/// Details about a single respawn request in flight.
#[derive(Debug, Clone)]
pub struct RespawnRequest {
    pub lost_ball: Entity,
    pub tracked_paddle: Option<Entity>,
    pub remaining_lives: u8,
    pub ball_spawn: Option<SpawnTransform>,
    pub paddle_spawn: Option<SpawnTransform>,
}

/// Canonical transform used when respawning an entity.
#[derive(Debug, Clone, Copy)]
pub struct SpawnTransform {
    pub translation: Vec3,
    pub rotation: Quat,
}

impl SpawnTransform {
    pub fn new(translation: Vec3, rotation: Quat) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    pub fn to_transform(self) -> Transform {
        Transform::from_translation(self.translation).with_rotation(self.rotation)
    }
}

/// Cached spawn data populated during level loading.
#[derive(Resource, Debug)]
pub struct SpawnPoints {
    pub paddle: Option<Vec3>,
    pub ball: Option<Vec3>,
    pub fallback_center: Vec3,
}

impl Default for SpawnPoints {
    fn default() -> Self {
        Self {
            paddle: None,
            ball: None,
            fallback_center: Vec3::new(0.0, 2.0, 0.0),
        }
    }
}

impl SpawnPoints {
    pub fn paddle_spawn(&self) -> SpawnTransform {
        let translation = self.paddle.unwrap_or(self.fallback_center);
        SpawnTransform::new(translation, Quat::from_rotation_x(-PI / 2.0))
    }

    pub fn ball_spawn(&self) -> SpawnTransform {
        let translation = self.ball.unwrap_or(self.fallback_center);
        SpawnTransform::new(translation, Quat::IDENTITY)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespawnEntityKind {
    Paddle,
    Ball,
}

/// Component tagging entities with their canonical respawn transform.
#[derive(Component, Debug, Clone, Copy)]
pub struct RespawnHandle {
    pub spawn: SpawnTransform,
    #[allow(dead_code)]
    pub kind: RespawnEntityKind,
}

/// Plugin wiring the respawn flow through ordered system sets.
pub struct RespawnPlugin;

#[derive(Event, Debug, Clone, Copy)]
pub struct LifeLostEvent {
    pub ball: Entity,
    pub cause: LifeLossCause,
    pub ball_spawn: SpawnTransform,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifeLossCause {
    LowerGoal,
}

#[allow(dead_code)]
#[derive(Event, Debug, Clone, Copy)]
pub struct RespawnScheduled {
    pub ball: Entity,
    pub paddle: Option<Entity>,
    pub completes_at: f64,
    pub remaining_lives: u8,
}

#[allow(dead_code)]
#[derive(Event, Debug, Clone, Copy)]
pub struct RespawnCompleted {
    pub ball: Entity,
    pub paddle: Option<Entity>,
    pub remaining_lives: u8,
}

#[allow(dead_code)]
#[derive(Event, Debug, Clone, Copy)]
pub struct GameOverRequested {
    pub remaining_lives: u8,
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
        app.init_resource::<RespawnSchedule>()
            .init_resource::<LivesState>()
            .init_resource::<SpawnPoints>()
            .add_event::<LifeLostEvent>()
            .add_event::<RespawnScheduled>()
            .add_event::<RespawnCompleted>()
            .add_event::<GameOverRequested>()
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
                    life_loss_logging
                        .in_set(RespawnSystems::Detect)
                        .after(detect_ball_loss),
                    schedule_respawn_timer.in_set(RespawnSystems::Schedule),
                    respawn_executor.in_set(RespawnSystems::Execute),
                    restore_paddle_control.in_set(RespawnSystems::Control),
                ),
            );
    }
}

fn detect_ball_loss(
    mut collision_events: EventReader<CollisionEvent>,
    balls: Query<Entity, With<Ball>>,
    ball_handles: Query<&RespawnHandle, With<Ball>>,
    lower_goals: Query<Entity, With<LowerGoal>>,
    spawn_points: Res<SpawnPoints>,
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
                let ball_spawn = match ball_handles.get(ball_entity) {
                    Ok(handle) => handle.spawn,
                    Err(_) => {
                        warn!(
                            "Ball entity {:?} missing RespawnHandle; using fallback",
                            ball_entity
                        );
                        spawn_points.ball_spawn()
                    }
                };
                life_lost_events.write(LifeLostEvent {
                    ball: ball_entity,
                    cause: LifeLossCause::LowerGoal,
                    ball_spawn,
                });
                commands.entity(ball_entity).despawn();
            }
        }
    }
}

fn life_loss_logging(mut life_lost_events: EventReader<LifeLostEvent>) {
    for event in life_lost_events.read() {
        let spawn = event.ball_spawn.translation;
        info!(
            "life lost: ball={:?} cause={:?} spawn=({:.2}, {:.2}, {:.2})",
            event.ball, event.cause, spawn.x, spawn.y, spawn.z
        );
    }
}

fn acquire_primary_paddle(
    paddles: &mut Query<(Entity, Option<&mut Velocity>), With<Paddle>>,
    commands: &mut Commands,
) -> Option<Entity> {
    let mut iter = paddles.iter_mut();
    if let Some((entity, maybe_velocity)) = iter.next() {
        if let Some(mut velocity) = maybe_velocity {
            velocity.linvel = Vec3::ZERO;
            velocity.angvel = Vec3::ZERO;
        }
        commands.entity(entity).insert(InputLocked);
        Some(entity)
    } else {
        None
    }
}

fn resolve_paddle_spawn(
    tracked_paddle: Option<Entity>,
    paddle_handles: &Query<&RespawnHandle, With<Paddle>>,
    spawn_points: &SpawnPoints,
) -> SpawnTransform {
    if let Some(entity) = tracked_paddle {
        if let Ok(handle) = paddle_handles.get(entity) {
            return handle.spawn;
        }
    }
    spawn_points.paddle_spawn()
}

fn start_pending_request(
    respawn_schedule: &mut RespawnSchedule,
    request: RespawnRequest,
    time: &Time,
    respawn_scheduled_events: &mut EventWriter<RespawnScheduled>,
) {
    respawn_schedule.timer.reset();
    respawn_schedule.pending = Some(request);
    respawn_schedule.last_loss = Some(time.elapsed());

    if let Some(active) = respawn_schedule.pending.as_ref() {
        let completes_at =
            time.elapsed().as_secs_f64() + respawn_schedule.timer.duration().as_secs_f64();
        respawn_scheduled_events.write(RespawnScheduled {
            ball: active.lost_ball,
            paddle: active.tracked_paddle,
            completes_at,
            remaining_lives: active.remaining_lives,
        });
    }
}

fn schedule_respawn_timer(
    mut respawn_schedule: ResMut<RespawnSchedule>,
    mut events: EventReader<LifeLostEvent>,
    lives_state: Res<LivesState>,
    time: Res<Time>,
    spawn_points: Res<SpawnPoints>,
    mut respawn_scheduled_events: EventWriter<RespawnScheduled>,
    mut game_over_events: EventWriter<GameOverRequested>,
    mut paddles: Query<(Entity, Option<&mut Velocity>), With<Paddle>>,
    paddle_handles: Query<&RespawnHandle, With<Paddle>>,
    mut commands: Commands,
) {
    if respawn_schedule.pending.is_none() {
        if let Some(mut next_request) = respawn_schedule.queue.pop_front() {
            if next_request.tracked_paddle.is_none() {
                next_request.tracked_paddle = acquire_primary_paddle(&mut paddles, &mut commands);
            }
            if next_request.paddle_spawn.is_none() {
                next_request.paddle_spawn = Some(resolve_paddle_spawn(
                    next_request.tracked_paddle,
                    &paddle_handles,
                    &spawn_points,
                ));
            }
            if next_request.ball_spawn.is_none() {
                next_request.ball_spawn = Some(spawn_points.ball_spawn());
            }
            start_pending_request(
                &mut respawn_schedule,
                next_request,
                &time,
                &mut respawn_scheduled_events,
            );
        }
    }

    let mut saw_event = false;
    for event in events.read().copied() {
        saw_event = true;

        if lives_state.lives_remaining == 0 {
            game_over_events.write(GameOverRequested {
                remaining_lives: lives_state.lives_remaining,
            });
            continue;
        }

        if respawn_schedule.pending.is_some() {
            let tracked_paddle = respawn_schedule
                .pending
                .as_ref()
                .and_then(|request| request.tracked_paddle);
            let paddle_spawn = resolve_paddle_spawn(tracked_paddle, &paddle_handles, &spawn_points);
            respawn_schedule.queue.push_back(RespawnRequest {
                lost_ball: event.ball,
                tracked_paddle,
                remaining_lives: lives_state.lives_remaining,
                ball_spawn: Some(event.ball_spawn),
                paddle_spawn: Some(paddle_spawn),
            });
            warn!(
                "respawn already pending; queued additional LifeLostEvent (queue_len={})",
                respawn_schedule.queue.len()
            );
        } else {
            let tracked_paddle = acquire_primary_paddle(&mut paddles, &mut commands);
            let paddle_spawn = resolve_paddle_spawn(tracked_paddle, &paddle_handles, &spawn_points);
            start_pending_request(
                &mut respawn_schedule,
                RespawnRequest {
                    lost_ball: event.ball,
                    tracked_paddle,
                    remaining_lives: lives_state.lives_remaining,
                    ball_spawn: Some(event.ball_spawn),
                    paddle_spawn: Some(paddle_spawn),
                },
                &time,
                &mut respawn_scheduled_events,
            );
        }
    }

    if saw_event {
        respawn_schedule.last_loss = Some(time.elapsed());
    }
}

fn respawn_executor(
    time: Res<Time>,
    mut respawn_schedule: ResMut<RespawnSchedule>,
    spawn_points: Res<SpawnPoints>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut paddles: Query<(Entity, &mut Transform, Option<&mut Velocity>), With<Paddle>>,
    mut respawn_completed_events: EventWriter<RespawnCompleted>,
    mut commands: Commands,
) {
    if respawn_schedule.pending.is_none() {
        return;
    }

    respawn_schedule.timer.tick(time.delta());
    if !respawn_schedule.timer.finished() {
        return;
    }

    let request = respawn_schedule.pending.take().unwrap();
    respawn_schedule.timer.reset();

    let paddle_spawn = request
        .paddle_spawn
        .unwrap_or_else(|| spawn_points.paddle_spawn());
    let ball_spawn = request
        .ball_spawn
        .unwrap_or_else(|| spawn_points.ball_spawn());

    let debug_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        unlit: false,
        ..default()
    });

    let mut respawn_paddle_entity = None;
    if let Some(tracked) = request.tracked_paddle {
        if let Ok((entity, mut transform, maybe_velocity)) = paddles.get_mut(tracked) {
            let mut target_transform = paddle_spawn.to_transform();
            target_transform.scale = Vec3::splat(0.01);
            *transform = target_transform;
            if let Some(mut velocity) = maybe_velocity {
                velocity.linvel = Vec3::ZERO;
                velocity.angvel = Vec3::ZERO;
            }
            commands.entity(entity).insert((
                PaddleGrowing {
                    timer: Timer::from_seconds(PADDLE_GROWTH_DURATION, TimerMode::Once),
                    target_scale: Vec3::ONE,
                },
                RespawnHandle {
                    spawn: paddle_spawn,
                    kind: RespawnEntityKind::Paddle,
                },
            ));
            respawn_paddle_entity = Some(entity);
        }
    }

    if respawn_paddle_entity.is_none() {
        let mut transform = paddle_spawn.to_transform();
        transform.scale = Vec3::splat(0.01);
        let new_entity = commands
            .spawn((
                Mesh3d(meshes.add(Capsule3d::new(PADDLE_RADIUS, PADDLE_HEIGHT).mesh())),
                MeshMaterial3d(debug_material.clone()),
                transform,
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
                RespawnHandle {
                    spawn: paddle_spawn,
                    kind: RespawnEntityKind::Paddle,
                },
            ))
            .insert(Friction {
                coefficient: 2.0,
                combine_rule: CoefficientCombineRule::Max,
            })
            .id();
        respawn_paddle_entity = Some(new_entity);
    }

    if let Some(entity) = respawn_paddle_entity {
        commands.entity(entity).insert(InputLocked);
    }

    let ball_transform = ball_spawn.to_transform();
    let respawned_ball = commands
        .spawn((
            Mesh3d(meshes.add(Sphere::new(BALL_RADIUS).mesh())),
            MeshMaterial3d(debug_material.clone()),
            ball_transform,
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
            RespawnHandle {
                spawn: ball_spawn,
                kind: RespawnEntityKind::Ball,
            },
        ))
        .insert((
            LockedAxes::TRANSLATION_LOCKED_Y,
            Ccd::enabled(),
            ExternalImpulse::default(),
            GravityScale(1.0),
        ))
        .id();

    respawn_completed_events.write(RespawnCompleted {
        ball: respawned_ball,
        paddle: respawn_paddle_entity,
        remaining_lives: request.remaining_lives,
    });
}

fn restore_paddle_control(
    respawn_schedule: Res<RespawnSchedule>,
    mut frozen_balls: Query<(Entity, &mut Velocity), (With<Ball>, With<BallFrozen>)>,
    mut paddles: Query<(Entity, Option<&PaddleGrowing>), (With<Paddle>, With<InputLocked>)>,
    mut commands: Commands,
) {
    if respawn_schedule.pending.is_some() || !respawn_schedule.queue.is_empty() {
        return;
    }

    let mut locked_remaining = false;
    for (entity, maybe_growing) in paddles.iter_mut() {
        if maybe_growing.is_none() {
            commands.entity(entity).remove::<InputLocked>();
        } else {
            locked_remaining = true;
        }
    }

    if locked_remaining {
        return;
    }

    for (entity, mut velocity) in frozen_balls.iter_mut() {
        velocity.linvel = Vec3::ZERO;
        velocity.angvel = Vec3::ZERO;
        commands.entity(entity).remove::<BallFrozen>();
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
    use std::f32::consts::PI;
    use std::time::Duration;

    pub(super) fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(Assets::<Mesh>::default())
            .insert_resource(Assets::<StandardMaterial>::default())
            .add_event::<CollisionEvent>()
            .add_plugins(RespawnPlugin);
        {
            let mut spawn_points = app.world_mut().resource_mut::<SpawnPoints>();
            spawn_points.ball = Some(Vec3::new(0.0, 2.0, 0.0));
            spawn_points.paddle = Some(Vec3::new(0.0, 2.0, 0.0));
        }
        app
    }

    pub(super) fn ball_handle_at(position: Vec3) -> RespawnHandle {
        RespawnHandle {
            spawn: SpawnTransform::new(position, Quat::IDENTITY),
            kind: RespawnEntityKind::Ball,
        }
    }

    pub(super) fn paddle_handle_at(position: Vec3) -> RespawnHandle {
        RespawnHandle {
            spawn: SpawnTransform::new(position, Quat::from_rotation_x(-PI / 2.0)),
            kind: RespawnEntityKind::Paddle,
        }
    }

    pub(super) fn advance_time(app: &mut App, delta_secs: f32) {
        let mut time = app.world_mut().resource_mut::<Time>();
        time.advance_by(Duration::from_secs_f32(delta_secs));
    }

    #[test]
    fn collision_triggers_respawn_schedule() {
        let mut app = test_app();

        let ball = app
            .world_mut()
            .spawn((Ball, ball_handle_at(Vec3::new(0.0, 2.0, 0.0))))
            .id();
        let lower_goal = app.world_mut().spawn(LowerGoal).id();
        let paddle = app
            .world_mut()
            .spawn((
                Paddle,
                Transform::default(),
                paddle_handle_at(Vec3::new(0.0, 2.0, 0.0)),
            ))
            .id();

        app.world_mut()
            .resource_mut::<Events<CollisionEvent>>()
            .send(CollisionEvent::Started(
                ball,
                lower_goal,
                CollisionEventFlags::SENSOR,
            ));

        advance_time(&mut app, 0.016);
        app.update();

        let respawn_schedule = app.world().resource::<RespawnSchedule>();
        assert!(respawn_schedule.pending.is_some());
        assert_eq!(respawn_schedule.pending.as_ref().unwrap().lost_ball, ball);
        let world = app.world();
        assert!(!world.entities().contains(ball));
        assert!(world.entity(paddle).contains::<InputLocked>());
    }

    #[test]
    fn executor_respawns_paddle_and_ball() {
        let mut app = test_app();

        let paddle = app
            .world_mut()
            .spawn((
                Paddle,
                Transform::default(),
                Velocity::zero(),
                paddle_handle_at(Vec3::new(-1.0, 2.0, 0.0)),
            ))
            .id();

        {
            let mut schedule = app.world_mut().resource_mut::<RespawnSchedule>();
            schedule.pending = Some(RespawnRequest {
                lost_ball: Entity::from_raw(999),
                tracked_paddle: Some(paddle),
                remaining_lives: 2,
                ball_spawn: Some(SpawnTransform::new(
                    Vec3::new(1.0, 2.0, 3.0),
                    Quat::IDENTITY,
                )),
                paddle_spawn: Some(SpawnTransform::new(
                    Vec3::new(-1.0, 2.0, 0.0),
                    Quat::from_rotation_x(-PI / 2.0),
                )),
            });
            schedule.timer.reset();
            let duration = schedule.timer.duration();
            schedule
                .timer
                .tick(Duration::from_secs_f32(duration.as_secs_f32() + 0.1));
        }

        app.update();

        let world = app.world();
        let paddle_transform = world.entity(paddle).get::<Transform>().unwrap();
        assert_eq!(paddle_transform.translation, Vec3::new(-1.0, 2.0, 0.0));

        let ball_count = world
            .iter_entities()
            .filter(|entity| entity.contains::<Ball>() && entity.contains::<BallFrozen>())
            .count();
        assert_eq!(ball_count, 1);
    }

    #[test]
    fn control_stage_unlocks_paddle_after_growth() {
        let mut app = test_app();
        app.world_mut().resource_mut::<RespawnSchedule>().pending = None;

        let paddle = app.world_mut().spawn((Paddle, InputLocked)).id();

        app.update();

        let world = app.world();
        assert!(!world.entity(paddle).contains::<InputLocked>());
    }

    #[test]
    fn control_stage_waits_for_growth_or_timer() {
        let mut app = test_app();

        {
            let mut schedule = app.world_mut().resource_mut::<RespawnSchedule>();
            schedule.pending = Some(RespawnRequest {
                lost_ball: Entity::from_raw(1),
                tracked_paddle: None,
                remaining_lives: 1,
                ball_spawn: Some(SpawnTransform::new(Vec3::ZERO, Quat::IDENTITY)),
                paddle_spawn: Some(SpawnTransform::new(
                    Vec3::ZERO,
                    Quat::from_rotation_x(-PI / 2.0),
                )),
            });
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

    #[test]
    fn ball_unlocks_only_after_paddle_ready() {
        let mut app = test_app();

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

        let ball = app
            .world_mut()
            .spawn((
                Ball,
                BallFrozen,
                Velocity::zero(),
                ExternalImpulse::default(),
            ))
            .id();

        app.update();

        let world = app.world();
        assert!(world.entity(paddle).contains::<InputLocked>());
        assert!(world.entity(ball).contains::<BallFrozen>());

        // Simulate paddle ready
        app.world_mut().entity_mut(paddle).remove::<PaddleGrowing>();

        app.update();

        let world = app.world();
        assert!(!world.entity(paddle).contains::<InputLocked>());
        assert!(!world.entity(ball).contains::<BallFrozen>());
    }

    #[test]
    fn scheduler_emits_game_over_when_no_lives() {
        let mut app = test_app();
        app.world_mut().insert_resource(LivesState {
            lives_remaining: 0,
            on_last_life: true,
        });

        let ball = app
            .world_mut()
            .spawn((Ball, ball_handle_at(Vec3::new(0.0, 2.0, 0.0))))
            .id();
        let lower_goal = app.world_mut().spawn(LowerGoal).id();

        app.world_mut()
            .resource_mut::<Events<CollisionEvent>>()
            .send(CollisionEvent::Started(
                ball,
                lower_goal,
                CollisionEventFlags::SENSOR,
            ));

        advance_time(&mut app, 0.016);
        app.update();

        let respawn_schedule = app.world().resource::<RespawnSchedule>();
        assert!(respawn_schedule.pending.is_none());

        let events = app.world().resource::<Events<GameOverRequested>>();
        assert!(!events.is_empty());
    }

    #[test]
    fn queued_life_losses_run_after_pending_respawn() {
        let mut app = test_app();

        {
            let mut spawn_points = app.world_mut().resource_mut::<SpawnPoints>();
            spawn_points.ball = Some(Vec3::new(0.0, 2.0, 0.0));
            spawn_points.paddle = Some(Vec3::new(0.0, 2.0, 0.0));
        }

        let lower_goal = app.world_mut().spawn(LowerGoal).id();
        let ball_a = app
            .world_mut()
            .spawn((Ball, ball_handle_at(Vec3::new(0.0, 2.0, 0.0))))
            .id();
        let ball_b = app
            .world_mut()
            .spawn((Ball, ball_handle_at(Vec3::new(0.0, 2.0, 0.0))))
            .id();
        app.world_mut().spawn((
            Paddle,
            Transform::default(),
            paddle_handle_at(Vec3::new(0.0, 2.0, 0.0)),
        ));

        app.world_mut()
            .resource_mut::<Events<CollisionEvent>>()
            .send(CollisionEvent::Started(
                ball_a,
                lower_goal,
                CollisionEventFlags::SENSOR,
            ));

        advance_time(&mut app, 0.016);
        app.update();

        {
            let schedule = app.world().resource::<RespawnSchedule>();
            assert!(schedule.pending.is_some());
            assert_eq!(schedule.queue.len(), 0);
        }

        app.world_mut()
            .resource_mut::<Events<CollisionEvent>>()
            .send(CollisionEvent::Started(
                ball_b,
                lower_goal,
                CollisionEventFlags::SENSOR,
            ));

        advance_time(&mut app, 0.016);
        app.update();

        {
            let schedule = app.world().resource::<RespawnSchedule>();
            assert!(schedule.pending.is_some());
            assert_eq!(schedule.queue.len(), 1);
        }

        {
            let mut schedule = app.world_mut().resource_mut::<RespawnSchedule>();
            schedule.pending = None;
        }

        app.update();

        {
            let schedule = app.world().resource::<RespawnSchedule>();
            assert!(schedule.pending.is_some());
            assert_eq!(schedule.queue.len(), 0);
            assert_eq!(schedule.pending.as_ref().unwrap().lost_ball, ball_b);
        }
    }
}
