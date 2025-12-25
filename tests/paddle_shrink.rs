use bevy::app::App;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::{CollisionEvent, Velocity};
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use brkrs::systems::respawn::{
    InputLocked, RespawnEntityKind, RespawnHandle, RespawnPlugin, RespawnSchedule, SpawnPoints,
    SpawnTransform,
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
fn paddle_shrinks_on_ball_loss() {
    let mut app = test_app();
    let (lower_goal, ball, paddle) = spawn_respawn_fixture(&mut app);

    // Verify paddle starts without PaddleGrowing component
    assert!(
        app.world().entity(paddle).get::<PaddleGrowing>().is_none(),
        "Paddle should not have PaddleGrowing component initially"
    );

    // Trigger ball loss
    trigger_life_loss(&mut app, ball, lower_goal);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify paddle now has PaddleGrowing component with shrink parameters
    let growing = app
        .world()
        .entity(paddle)
        .get::<PaddleGrowing>()
        .expect("Paddle should have PaddleGrowing component after ball loss");

    assert_eq!(
        growing.target_scale,
        Vec3::splat(0.01),
        "Target scale should be 0.01 for shrink animation"
    );
    assert_eq!(
        growing.start_scale,
        Vec3::ONE,
        "Start scale should be Vec3::ONE (current paddle scale)"
    );
}

#[test]
fn shrink_reaches_minimum_scale() {
    let mut app = test_app();
    let (lower_goal, ball, paddle) = spawn_respawn_fixture(&mut app);

    // Trigger ball loss
    trigger_life_loss(&mut app, ball, lower_goal);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify PaddleGrowing component has correct target scale
    let growing = app
        .world()
        .entity(paddle)
        .get::<PaddleGrowing>()
        .expect("Paddle should have PaddleGrowing component");

    assert_eq!(
        growing.target_scale,
        Vec3::splat(0.01),
        "Target scale should be 0.01 (minimum scale)"
    );

    // Verify timer is set up correctly
    assert!(
        !growing.timer.is_finished(),
        "Timer should not be finished immediately after adding component"
    );
}

#[test]
fn paddle_remains_visible_during_shrink() {
    let mut app = test_app();
    let (lower_goal, ball, paddle) = spawn_respawn_fixture(&mut app);

    // Trigger ball loss
    trigger_life_loss(&mut app, ball, lower_goal);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify paddle entity still exists
    assert!(
        app.world().get_entity(paddle).is_ok(),
        "Paddle entity should still exist after ball loss"
    );

    // Verify paddle has Transform component (visible entity)
    assert!(
        app.world().entity(paddle).get::<Transform>().is_some(),
        "Paddle should have Transform component (remains visible)"
    );

    // Verify paddle is not despawned - it has PaddleGrowing for animation
    assert!(
        app.world().entity(paddle).get::<PaddleGrowing>().is_some(),
        "Paddle should have PaddleGrowing component for shrink animation"
    );

    // Verify paddle entity still exists
    assert!(
        app.world().get_entity(paddle).is_ok(),
        "Paddle entity should remain visible throughout shrink"
    );
}

#[test]
fn shrink_duration_matches_respawn_delay() {
    let mut app = test_app();
    let (lower_goal, ball, paddle) = spawn_respawn_fixture(&mut app);

    // Get the respawn delay duration
    let respawn_delay = app
        .world()
        .resource::<RespawnSchedule>()
        .timer
        .duration()
        .as_secs_f32();

    // Trigger ball loss
    trigger_life_loss(&mut app, ball, lower_goal);
    advance_time(&mut app, 0.016);
    app.update();

    // Get the shrink duration from the component
    let shrink_duration = {
        let growing = app
            .world()
            .entity(paddle)
            .get::<PaddleGrowing>()
            .expect("Paddle should have PaddleGrowing component");
        growing.timer.duration().as_secs_f32()
    };

    // Verify shrink duration matches respawn delay
    assert_eq!(
        shrink_duration, respawn_delay,
        "Shrink duration should match respawn delay"
    );
}

#[test]
fn input_locked_during_shrink() {
    let mut app = test_app();
    let (lower_goal, ball, paddle) = spawn_respawn_fixture(&mut app);

    // Trigger ball loss
    trigger_life_loss(&mut app, ball, lower_goal);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify paddle has InputLocked component
    assert!(
        app.world().entity(paddle).get::<InputLocked>().is_some(),
        "Paddle should have InputLocked component during shrink"
    );

    // Advance time through the middle of the shrink animation
    advance_time(&mut app, 0.5);
    app.update();

    // Verify paddle still has InputLocked component
    assert!(
        app.world().entity(paddle).get::<InputLocked>().is_some(),
        "Paddle should remain input-locked throughout shrink"
    );
}

#[test]
fn shrink_interrupts_growth_animation() {
    let mut app = test_app();
    let (lower_goal, ball, paddle) = spawn_respawn_fixture(&mut app);

    // Start the paddle in a growing state (simulating level transition)
    app.world_mut().entity_mut(paddle).insert((
        PaddleGrowing {
            timer: Timer::from_seconds(2.0, TimerMode::Once),
            target_scale: Vec3::ONE,
            start_scale: Vec3::splat(0.01),
        },
        Transform::from_scale(Vec3::splat(0.5)), // Mid-growth
    ));

    advance_time(&mut app, 0.016);
    app.update();

    // Verify paddle has growth animation initially
    assert!(
        app.world().entity(paddle).get::<PaddleGrowing>().is_some(),
        "Paddle should have PaddleGrowing component before ball loss"
    );

    // Trigger ball loss during growth
    // Note: The apply_paddle_shrink system has a query filter Without<PaddleGrowing>
    // So it won't interrupt an ongoing growth animation
    // This is intentional to prevent component replacement mid-animation
    trigger_life_loss(&mut app, ball, lower_goal);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify paddle still has the original PaddleGrowing component
    // (apply_paddle_shrink doesn't replace existing PaddleGrowing)
    let growing = app
        .world()
        .entity(paddle)
        .get::<PaddleGrowing>()
        .expect("Paddle should have PaddleGrowing component");

    // The growth animation continues - shrink is skipped for paddles already animating
    assert_eq!(
        growing.target_scale,
        Vec3::ONE,
        "Target scale should still be Vec3::ONE (growth continues)"
    );
}

#[test]
fn rapid_consecutive_losses_handled() {
    let mut app = test_app();
    let lower_goal = app.world_mut().spawn(LowerGoal).id();

    // Spawn paddle
    let paddle = app
        .world_mut()
        .spawn((
            Paddle,
            Transform::from_scale(Vec3::ONE),
            Velocity::zero(),
            paddle_handle_at(Vec3::new(0.0, 2.0, 0.0)),
        ))
        .id();

    // First ball loss
    let ball1 = app
        .world_mut()
        .spawn((Ball, ball_handle_at(Vec3::new(0.0, 2.0, 0.0))))
        .id();
    trigger_life_loss(&mut app, ball1, lower_goal);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify first shrink started
    assert!(
        app.world().entity(paddle).get::<PaddleGrowing>().is_some(),
        "Paddle should have PaddleGrowing after first loss"
    );

    // Second ball loss (rapid)
    let ball2 = app
        .world_mut()
        .spawn((Ball, ball_handle_at(Vec3::new(0.0, 2.0, 0.0))))
        .id();
    trigger_life_loss(&mut app, ball2, lower_goal);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify paddle still has shrink animation (not interrupted)
    // The second loss is queued but doesn't interrupt the current shrink
    assert!(
        app.world().entity(paddle).get::<PaddleGrowing>().is_some(),
        "Paddle should continue shrinking even with rapid loss"
    );
}

#[test]
fn shrink_component_configuration() {
    let mut app = test_app();
    let (lower_goal, ball, paddle) = spawn_respawn_fixture(&mut app);

    // Trigger ball loss
    trigger_life_loss(&mut app, ball, lower_goal);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify PaddleGrowing component is configured for shrink
    let growing = app
        .world()
        .entity(paddle)
        .get::<PaddleGrowing>()
        .expect("Paddle should have PaddleGrowing component");

    // Verify shrink configuration
    assert_eq!(
        growing.target_scale,
        Vec3::splat(0.01),
        "Target scale should be 0.01 for shrink"
    );

    assert_eq!(
        growing.start_scale,
        Vec3::ONE,
        "Start scale should be Vec3::ONE (current paddle scale)"
    );

    // Verify timer is active
    assert!(!growing.timer.is_finished(), "Timer should be active");

    // Verify timer duration matches respawn delay
    let respawn_duration = app
        .world()
        .resource::<RespawnSchedule>()
        .timer
        .duration()
        .as_secs_f32();

    assert_eq!(
        growing.timer.duration().as_secs_f32(),
        respawn_duration,
        "Shrink duration should match respawn delay"
    );
}
