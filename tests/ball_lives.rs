use bevy::app::App;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;

use bevy_rapier3d::rapier::prelude::CollisionEventFlags;
use brkrs::systems::respawn::{
    GameOverRequested, LifeLossCause, LifeLostEvent, LivesState, RespawnEntityKind, RespawnHandle,
    RespawnPlugin, SpawnTransform,
};
use brkrs::{Ball, LowerGoal};

fn test_app() -> App {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins(MinimalPlugins)
        .insert_resource(Assets::<Mesh>::default())
        .insert_resource(Assets::<StandardMaterial>::default())
        .add_message::<bevy_rapier3d::prelude::CollisionEvent>()
        .add_plugins(RespawnPlugin);
    app
}

fn spawn_ball_with_respawn(app: &mut App, position: Vec3) -> Entity {
    let spawn = SpawnTransform::new(position, Quat::IDENTITY);
    app.world_mut()
        .spawn((
            Ball,
            Transform::from_translation(position),
            GlobalTransform::from_translation(position),
            RespawnHandle {
                spawn,
                kind: RespawnEntityKind::Ball,
            },
        ))
        .id()
}

fn spawn_lower_goal(app: &mut App) -> Entity {
    app.world_mut().spawn(LowerGoal).id()
}

#[test]
fn lives_start_at_3() {
    let app = test_app();
    let lives = app.world().resource::<LivesState>();
    assert_eq!(lives.lives_remaining, 3);
}

#[test]
fn lives_decrement_on_life_lost_event() {
    let mut app = test_app();

    // Initial state: 3 lives
    assert_eq!(app.world().resource::<LivesState>().lives_remaining, 3);

    // Trigger first life lost
    app.world_mut()
        .resource_mut::<Messages<LifeLostEvent>>()
        .write(LifeLostEvent {
            ball: Entity::PLACEHOLDER,
            cause: LifeLossCause::LowerGoal,
            ball_spawn: SpawnTransform::new(Vec3::ZERO, Quat::IDENTITY),
        });

    app.update();

    // Should be 2 lives
    assert_eq!(app.world().resource::<LivesState>().lives_remaining, 2);

    // Trigger second life lost
    app.world_mut()
        .resource_mut::<Messages<LifeLostEvent>>()
        .write(LifeLostEvent {
            ball: Entity::PLACEHOLDER,
            cause: LifeLossCause::LowerGoal,
            ball_spawn: SpawnTransform::new(Vec3::ZERO, Quat::IDENTITY),
        });

    app.update();

    // Should be 1 life
    assert_eq!(app.world().resource::<LivesState>().lives_remaining, 1);
}

#[test]
fn lives_clamp_at_zero() {
    let mut app = test_app();

    // Set lives to 1
    app.world_mut().resource_mut::<LivesState>().lives_remaining = 1;

    // Trigger life lost
    app.world_mut()
        .resource_mut::<Messages<LifeLostEvent>>()
        .write(LifeLostEvent {
            ball: Entity::PLACEHOLDER,
            cause: LifeLossCause::LowerGoal,
            ball_spawn: SpawnTransform::new(Vec3::ZERO, Quat::IDENTITY),
        });

    app.update();

    // Should be 0 lives
    assert_eq!(app.world().resource::<LivesState>().lives_remaining, 0);

    // Trigger another life lost (should not go negative)
    app.world_mut()
        .resource_mut::<Messages<LifeLostEvent>>()
        .write(LifeLostEvent {
            ball: Entity::PLACEHOLDER,
            cause: LifeLossCause::LowerGoal,
            ball_spawn: SpawnTransform::new(Vec3::ZERO, Quat::IDENTITY),
        });

    app.update();

    // Should still be 0 lives (clamped)
    assert_eq!(app.world().resource::<LivesState>().lives_remaining, 0);
}

#[test]
fn multiple_events_same_frame_decrement_individually() {
    let mut app = test_app();

    // Set lives to 3
    assert_eq!(app.world().resource::<LivesState>().lives_remaining, 3);

    // Write 2 LifeLostEvent in same frame with different non-LowerGoal causes
    // (LowerGoal events are aggregated, but other causes like MerkabaCollision and PaddleHazard
    // should each decrement lives independently)
    let mut messages = app.world_mut().resource_mut::<Messages<LifeLostEvent>>();
    messages.write(LifeLostEvent {
        ball: Entity::PLACEHOLDER,
        cause: LifeLossCause::MerkabaCollision,
        ball_spawn: SpawnTransform::new(Vec3::ZERO, Quat::IDENTITY),
    });
    messages.write(LifeLostEvent {
        ball: Entity::PLACEHOLDER,
        cause: LifeLossCause::PaddleHazard,
        ball_spawn: SpawnTransform::new(Vec3::new(1.0, 0.0, 0.0), Quat::IDENTITY),
    });

    app.update();

    // Should decrement twice: 3 -> 2 -> 1
    assert_eq!(app.world().resource::<LivesState>().lives_remaining, 1);
}

#[test]
fn game_over_requested_when_lives_reach_zero() {
    let mut app = test_app();

    // Set lives to 1
    app.world_mut().resource_mut::<LivesState>().lives_remaining = 1;

    // Trigger life lost
    app.world_mut()
        .resource_mut::<Messages<LifeLostEvent>>()
        .write(LifeLostEvent {
            ball: Entity::PLACEHOLDER,
            cause: LifeLossCause::LowerGoal,
            ball_spawn: SpawnTransform::new(Vec3::ZERO, Quat::IDENTITY),
        });

    app.update();

    // Check GameOverRequested was emitted
    let events = app.world().resource::<Messages<GameOverRequested>>();
    assert!(
        !events.is_empty(),
        "GameOverRequested should be emitted when lives reach 0"
    );
}

#[test]
fn no_game_over_requested_when_lives_above_zero() {
    let mut app = test_app();

    // Initial state: 3 lives
    assert_eq!(app.world().resource::<LivesState>().lives_remaining, 3);

    // Trigger life lost
    app.world_mut()
        .resource_mut::<Messages<LifeLostEvent>>()
        .write(LifeLostEvent {
            ball: Entity::PLACEHOLDER,
            cause: LifeLossCause::LowerGoal,
            ball_spawn: SpawnTransform::new(Vec3::ZERO, Quat::IDENTITY),
        });

    app.update();

    // Should be 2 lives, no game-over
    assert_eq!(app.world().resource::<LivesState>().lives_remaining, 2);

    // Check no GameOverRequested was emitted
    let events = app.world().resource::<Messages<GameOverRequested>>();
    assert!(
        events.is_empty(),
        "GameOverRequested should not be emitted when lives are above 0"
    );
}

#[test]
fn lives_can_be_reset_manually() {
    let mut app = test_app();

    // Reduce lives to 0
    app.world_mut().resource_mut::<LivesState>().lives_remaining = 0;
    assert_eq!(app.world().resource::<LivesState>().lives_remaining, 0);

    // Reset to 3 (simulating level restart)
    app.world_mut().resource_mut::<LivesState>().lives_remaining = 3;
    assert_eq!(app.world().resource::<LivesState>().lives_remaining, 3);
}

#[test]
fn life_lost_only_when_all_balls_gone_sequential() {
    let mut app = test_app();

    let lower_goal = spawn_lower_goal(&mut app);
    let ball_a = spawn_ball_with_respawn(&mut app, Vec3::new(0.0, 2.0, 0.0));
    let ball_b = spawn_ball_with_respawn(&mut app, Vec3::new(1.0, 2.0, 0.0));

    // First ball lost: lives should not decrement because another ball remains
    app.world_mut()
        .resource_mut::<Messages<bevy_rapier3d::prelude::CollisionEvent>>()
        .write(bevy_rapier3d::prelude::CollisionEvent::Started(
            ball_a,
            lower_goal,
            CollisionEventFlags::SENSOR,
        ));
    app.update();
    assert_eq!(app.world().resource::<LivesState>().lives_remaining, 3);

    // Second ball lost: now all balls are gone, life should decrement once
    app.world_mut()
        .resource_mut::<Messages<bevy_rapier3d::prelude::CollisionEvent>>()
        .write(bevy_rapier3d::prelude::CollisionEvent::Started(
            ball_b,
            lower_goal,
            CollisionEventFlags::SENSOR,
        ));
    app.update();
    assert_eq!(app.world().resource::<LivesState>().lives_remaining, 2);
}

#[test]
fn life_lost_only_once_when_all_balls_gone_same_frame() {
    let mut app = test_app();

    let lower_goal = spawn_lower_goal(&mut app);
    let ball_a = spawn_ball_with_respawn(&mut app, Vec3::new(0.0, 2.0, 0.0));
    let ball_b = spawn_ball_with_respawn(&mut app, Vec3::new(1.0, 2.0, 0.0));

    // Both balls lost in the same frame
    let mut collisions = app
        .world_mut()
        .resource_mut::<Messages<bevy_rapier3d::prelude::CollisionEvent>>();
    collisions.write(bevy_rapier3d::prelude::CollisionEvent::Started(
        ball_a,
        lower_goal,
        CollisionEventFlags::SENSOR,
    ));
    collisions.write(bevy_rapier3d::prelude::CollisionEvent::Started(
        ball_b,
        lower_goal,
        CollisionEventFlags::SENSOR,
    ));
    drop(collisions);

    app.update();
    assert_eq!(app.world().resource::<LivesState>().lives_remaining, 2);
}
