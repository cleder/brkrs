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

fn spawn_respawn_fixture(app: &mut App) -> (Entity, Entity, Entity) {
    let lower_goal = app.world_mut().spawn(LowerGoal).id();
    let ball = app
        .world_mut()
        .spawn((Ball, ball_handle_at(Vec3::new(0.0, 2.0, 0.0))))
        .id();
    let paddle = app
        .world_mut()
        .spawn((
            Paddle,
            Transform::from_scale(Vec3::ONE), // Start at full size
            Velocity::zero(),
            paddle_handle_at(Vec3::new(0.0, 2.0, 0.0)),
        ))
        .id();
    (lower_goal, ball, paddle)
}

fn trigger_life_loss(app: &mut App, ball: Entity, lower_goal: Entity) {
    app.world_mut()
        .resource_mut::<Messages<CollisionEvent>>()
        .write(CollisionEvent::Started(
            ball,
            lower_goal,
            CollisionEventFlags::SENSOR,
        ));
}

#[test]
fn paddle_does_not_shrink_when_multiple_balls_in_play() {
    let mut app = test_app();
    let (lower_goal, ball1, paddle) = spawn_respawn_fixture(&mut app);

    // Spawn a second ball
    let _ball2 = app
        .world_mut()
        .spawn((Ball, ball_handle_at(Vec3::new(2.0, 2.0, 0.0))))
        .id();

    // Verify paddle starts without PaddleGrowing component
    assert!(
        app.world().entity(paddle).get::<PaddleGrowing>().is_none(),
        "Paddle should not have PaddleGrowing component initially"
    );

    // Trigger loss of first ball (but second ball still exists)
    trigger_life_loss(&mut app, ball1, lower_goal);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify paddle does NOT have PaddleGrowing component
    // (because there are still other balls in play)
    assert!(
        app.world().entity(paddle).get::<PaddleGrowing>().is_none(),
        "Paddle should NOT shrink when other balls are still in play"
    );
}

#[test]
fn paddle_shrinks_when_last_ball_is_lost() {
    let mut app = test_app();
    let (lower_goal, ball1, paddle) = spawn_respawn_fixture(&mut app);

    // Spawn a second ball
    let ball2 = app
        .world_mut()
        .spawn((Ball, ball_handle_at(Vec3::new(2.0, 2.0, 0.0))))
        .id();

    // Trigger loss of first ball (lives still at 3)
    trigger_life_loss(&mut app, ball1, lower_goal);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify paddle still has no PaddleGrowing (lives > 1, respawn will happen)
    assert!(
        app.world().entity(paddle).get::<PaddleGrowing>().is_none(),
        "Paddle should not shrink when lives > 1 (respawn will happen)"
    );

    // Set lives to 1 so the next ball loss will be the last one
    {
        let mut lives = app
            .world_mut()
            .resource_mut::<brkrs::systems::respawn::LivesState>();
        lives.lives_remaining = 1;
    }

    // Now lose the last ball with lives == 1
    trigger_life_loss(&mut app, ball2, lower_goal);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify paddle NOW has PaddleGrowing component (lives == 1, no more respawns)
    let growing = app
        .world()
        .entity(paddle)
        .get::<PaddleGrowing>()
        .expect("Paddle should have PaddleGrowing component when lives == 1");

    assert_eq!(
        growing.target_scale,
        Vec3::splat(0.01),
        "Target scale should be 0.01 for shrink animation"
    );
}

#[test]
fn paddle_shrinks_when_single_ball_lost_on_last_life() {
    let mut app = test_app();
    let (lower_goal, ball, paddle) = spawn_respawn_fixture(&mut app);

    // Set lives to 1 so the next ball loss will be game over
    {
        let mut lives = app
            .world_mut()
            .resource_mut::<brkrs::systems::respawn::LivesState>();
        lives.lives_remaining = 1;
    }

    // Losing the ball when lives == 1 should shrink the paddle (last life)
    trigger_life_loss(&mut app, ball, lower_goal);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify paddle has PaddleGrowing component
    let growing =
        app.world().entity(paddle).get::<PaddleGrowing>().expect(
            "Paddle should have PaddleGrowing component when single ball is lost on last life",
        );

    assert_eq!(
        growing.target_scale,
        Vec3::splat(0.01),
        "Target scale should be 0.01 for shrink animation"
    );
}

#[test]
fn no_visual_effects_when_multiple_balls_in_play() {
    let mut app = test_app();
    let (lower_goal, ball1, paddle) = spawn_respawn_fixture(&mut app);

    // Spawn a second ball
    let _ball2 = app
        .world_mut()
        .spawn((Ball, ball_handle_at(Vec3::new(2.0, 2.0, 0.0))))
        .id();

    // Trigger loss of first ball (but second ball still exists)
    trigger_life_loss(&mut app, ball1, lower_goal);
    advance_time(&mut app, 0.016);
    app.update();

    // Check that paddle does NOT have shrink animation
    // (respawn may be queued internally, but no visual effects should trigger)
    assert!(
        app.world().entity(paddle).get::<PaddleGrowing>().is_none(),
        "Paddle should NOT shrink when other balls are still in play"
    );
}
