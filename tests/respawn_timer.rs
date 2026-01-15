use bevy::app::App;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::{CollisionEvent, Velocity};
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use brkrs::systems::respawn::{
    InputLocked, RespawnCompleted, RespawnEntityKind, RespawnFadeOverlay, RespawnHandle,
    RespawnPlugin, RespawnSchedule, RespawnVisualState, SpawnPoints, SpawnTransform,
};
use brkrs::{Ball, BallFrozen, LowerGoal, Paddle, PaddleGrowing};

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
            Transform::default(),
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
fn respawn_timer_finishes_within_frame_tolerance() {
    let mut app = test_app();
    let (lower_goal, ball, _) = spawn_respawn_fixture(&mut app);

    trigger_life_loss(&mut app, ball, lower_goal);

    advance_time(&mut app, 0.016);
    app.update();

    let frame_tolerance = Duration::from_secs_f32(0.016);
    let total_duration = app.world().resource::<RespawnSchedule>().timer.duration();

    {
        let mut schedule = app.world_mut().resource_mut::<RespawnSchedule>();
        schedule.timer.tick(total_duration - frame_tolerance);
        assert!(
            !schedule.timer.is_finished(),
            "timer should remain pending right before frame tolerance boundary",
        );
    }

    advance_time(&mut app, 0.0);
    app.update();
    {
        let schedule = app.world().resource::<RespawnSchedule>();
        assert!(
            schedule.pending.is_some(),
            "respawn should still be pending after partial tick",
        );
    }

    {
        let mut schedule = app.world_mut().resource_mut::<RespawnSchedule>();
        schedule
            .timer
            .tick(frame_tolerance + Duration::from_millis(1));
    }

    advance_time(&mut app, 0.0);
    app.update();
    {
        let schedule = app.world().resource::<RespawnSchedule>();
        assert!(
            schedule.pending.is_none(),
            "respawn should complete within tolerance once timer elapses",
        );
    }

    let completions = app.world().resource::<Messages<RespawnCompleted>>();
    assert!(
        !completions.is_empty(),
        "respawn completion event should fire when timer elapses",
    );
}

#[test]
fn ball_remains_frozen_until_launch_unlock() {
    let mut app = test_app();
    let (lower_goal, ball, paddle) = spawn_respawn_fixture(&mut app);

    trigger_life_loss(&mut app, ball, lower_goal);

    advance_time(&mut app, 0.016);
    app.update();

    {
        let total_duration = app.world().resource::<RespawnSchedule>().timer.duration();
        let mut schedule = app.world_mut().resource_mut::<RespawnSchedule>();
        schedule
            .timer
            .tick(total_duration + Duration::from_millis(10));
    }
    advance_time(&mut app, 0.0);
    app.update();

    let respawned_ball = {
        let mut query = app
            .world_mut()
            .query_filtered::<Entity, (With<Ball>, With<BallFrozen>)>();
        query
            .iter(app.world())
            .next()
            .expect("respawned ball should exist")
    };

    {
        let world = app.world();
        let velocity = world.entity(respawned_ball).get::<Velocity>().unwrap();
        assert_eq!(velocity.linvel, Vec3::ZERO);
        assert!(world.entity(paddle).contains::<InputLocked>());
    }

    advance_time(&mut app, 0.5);
    app.update();
    {
        let world = app.world();
        let velocity = world.entity(respawned_ball).get::<Velocity>().unwrap();
        assert_eq!(velocity.linvel, Vec3::ZERO);
    }

    {
        let world = app.world_mut();
        world.entity_mut(paddle).remove::<PaddleGrowing>();
    }
    app.update();

    {
        let world = app.world();
        assert!(
            world.resource::<RespawnVisualState>().is_active(),
            "visual overlay should be active before fade completes",
        );
        assert!(world.entity(paddle).contains::<InputLocked>());
    }

    {
        let overlay_entity = {
            let mut query = app
                .world_mut()
                .query_filtered::<Entity, With<RespawnFadeOverlay>>();
            query
                .iter(app.world())
                .next()
                .expect("respawn overlay should exist while fade is active")
        };
        let world = app.world_mut();
        {
            let mut entity = world.entity_mut(overlay_entity);
            let mut overlay = entity
                .get_mut::<RespawnFadeOverlay>()
                .expect("overlay component missing");
            let remaining = overlay.timer().remaining_secs();
            overlay
                .timer_mut()
                .tick(Duration::from_secs_f32(remaining + 0.1));
        }
    }
    advance_time(&mut app, 0.0);
    app.update();

    {
        let world = app.world();
        assert!(
            !world.resource::<RespawnVisualState>().is_active(),
            "overlay should clear after fade completes",
        );
        assert!(
            !world.entity(respawned_ball).contains::<BallFrozen>(),
            "ball should unfreeze once controls unlock",
        );
        assert!(
            !world.entity(paddle).contains::<InputLocked>(),
            "paddle controls should unlock after fade",
        );
    }
}
