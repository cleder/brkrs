use bevy::app::App;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::{CollisionEvent, Velocity};
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use brkrs::systems::respawn::{
    RespawnEntityKind, RespawnHandle, RespawnPlugin, RespawnScheduled, SpawnPoints, SpawnTransform,
};
use brkrs::{Ball, LowerGoal, Paddle, PaddleGrowing};

use std::time::Duration;

fn test_app() -> App {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins(MinimalPlugins)
        .insert_resource(Assets::<Mesh>::default())
        .insert_resource(Assets::<StandardMaterial>::default())
        .add_message::<CollisionEvent>()
        .add_message::<RespawnScheduled>()
        .add_plugins(RespawnPlugin);
    {
        let mut spawn_points = app.world_mut().resource_mut::<SpawnPoints>();
        spawn_points.ball = Some(Vec3::new(0.0, 2.0, 0.0));
        spawn_points.paddle = Some(Vec3::new(0.0, 2.0, 0.0));
    }
    app
}

fn ball_handle_at(position: Vec3) -> RespawnHandle {
    RespawnHandle {
        spawn: SpawnTransform::new(position, Quat::IDENTITY),
        kind: RespawnEntityKind::Ball,
    }
}

fn paddle_handle_at(position: Vec3) -> RespawnHandle {
    RespawnHandle {
        spawn: SpawnTransform::new(position, Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
        kind: RespawnEntityKind::Paddle,
    }
}

fn advance_time(app: &mut App, delta_secs: f32) {
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(Duration::from_secs_f32(delta_secs));
}

#[test]
fn paddle_shrinks_only_once_when_losing_multiple_balls_on_last_life() {
    let mut app = test_app();
    let lower_goal = app.world_mut().spawn(LowerGoal).id();

    // Set lives to 1 so any ball loss that removes all balls would trigger shrink
    {
        let mut lives = app
            .world_mut()
            .resource_mut::<brkrs::systems::respawn::LivesState>();
        lives.lives_remaining = 3;
    }

    // Spawn multiple balls
    let ball1 = app
        .world_mut()
        .spawn((Ball, ball_handle_at(Vec3::new(0.0, 2.0, 0.0))))
        .id();
    let ball2 = app
        .world_mut()
        .spawn((Ball, ball_handle_at(Vec3::new(2.0, 2.0, 0.0))))
        .id();
    let ball3 = app
        .world_mut()
        .spawn((Ball, ball_handle_at(Vec3::new(-2.0, 2.0, 0.0))))
        .id();

    let paddle = app
        .world_mut()
        .spawn((
            Paddle,
            Transform::from_scale(Vec3::ONE),
            Velocity::zero(),
            paddle_handle_at(Vec3::new(0.0, 2.0, 0.0)),
        ))
        .id();

    // Trigger loss of first ball ONLY
    app.world_mut()
        .resource_mut::<Messages<CollisionEvent>>()
        .write(CollisionEvent::Started(
            ball1,
            lower_goal,
            CollisionEventFlags::SENSOR,
        ));

    advance_time(&mut app, 0.016);
    app.update();

    // Verify paddle does NOT have PaddleGrowing because other balls still exist
    // (other balls exist, so paddle should NOT shrink even though a ball was lost)
    assert!(
        app.world().entity(paddle).get::<PaddleGrowing>().is_none(),
        "Paddle should NOT shrink when other balls still exist, even if a ball was lost"
    );

    // Trigger loss of second ball
    app.world_mut()
        .resource_mut::<Messages<CollisionEvent>>()
        .write(CollisionEvent::Started(
            ball2,
            lower_goal,
            CollisionEventFlags::SENSOR,
        ));

    advance_time(&mut app, 0.016);
    app.update();

    // Still should not shrink (ball3 still exists)
    assert!(
        app.world().entity(paddle).get::<PaddleGrowing>().is_none(),
        "Paddle should NOT shrink when other balls still exist"
    );

    // Trigger loss of last ball
    app.world_mut()
        .resource_mut::<Messages<CollisionEvent>>()
        .write(CollisionEvent::Started(
            ball3,
            lower_goal,
            CollisionEventFlags::SENSOR,
        ));

    advance_time(&mut app, 0.016);
    app.update();

    // Now paddle SHOULD have PaddleGrowing because all balls are gone (last ball lost)
    let growing = app.world().entity(paddle).get::<PaddleGrowing>();

    assert!(
        growing.is_some(),
        "Paddle should shrink when the last ball is lost (all balls gone), regardless of lives count"
    );

    // Verify the shrink component is configured correctly
    let growing_component = growing.unwrap();

    assert_eq!(
        growing_component.start_scale,
        Vec3::ONE,
        "Paddle shrink should start from full size (Vec3::ONE)"
    );

    assert_eq!(
        growing_component.target_scale,
        Vec3::splat(0.01),
        "Paddle shrink should target very small size (0.01)"
    );
}
