use super::tests::{advance_time, ball_handle_at, paddle_handle_at, test_app};
use super::*;
use bevy::ecs::event::Events;
use bevy_rapier3d::prelude::{CollisionEvent, Velocity};
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;
use std::time::Duration;

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
        .resource_mut::<Events<CollisionEvent>>()
        .send(CollisionEvent::Started(
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
            !schedule.timer.finished(),
            "timer should remain pending right before frame tolerance boundary",
        );
    }

    advance_time(&mut app, 0.0);
    app.update();
    {
        let schedule = app.world().resource::<RespawnSchedule>();
        assert!(
            schedule.pending.is_some(),
            "respawn should still be pending after partial tick"
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
            "respawn should complete within ±16ms tolerance after 1s"
        );
    }

    let completions = app.world().resource::<Events<RespawnCompleted>>();
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

    let respawned_ball = app
        .world()
        .iter_entities()
        .find(|entity| entity.contains::<Ball>() && entity.contains::<BallFrozen>())
        .map(|entity| entity.id())
        .expect("respawned ball should exist");

    {
        let world = app.world();
        let velocity = world.entity(respawned_ball).get::<Velocity>().unwrap();
        assert_eq!(velocity.linvel, Vec3::ZERO);
        assert_eq!(velocity.angvel, Vec3::ZERO);
        assert!(world.entity(respawned_ball).contains::<BallFrozen>());
        assert!(world.entity(paddle).contains::<InputLocked>());
    }

    advance_time(&mut app, 0.5);
    app.update();
    {
        let world = app.world();
        let velocity = world.entity(respawned_ball).get::<Velocity>().unwrap();
        assert_eq!(
            velocity.linvel,
            Vec3::ZERO,
            "ball must stay stationary while frozen"
        );
        assert!(world.entity(respawned_ball).contains::<BallFrozen>());
    }

    {
        let world = app.world_mut();
        world.entity_mut(paddle).remove::<PaddleGrowing>();
    }
    app.update();

    {
        let world = app.world();
        assert!(
            world.entity(respawned_ball).contains::<BallFrozen>(),
            "ball should remain frozen while the overlay fade runs",
        );
        assert!(
            world.resource::<RespawnVisualState>().active,
            "visual overlay stays active until fade completes",
        );
        assert!(
            world.entity(paddle).contains::<InputLocked>(),
            "paddle control must stay locked until animation finishes",
        );
    }

    {
        let world = app.world_mut();
        let overlay_entity = world
            .iter_entities()
            .find(|entity| entity.contains::<RespawnFadeOverlay>())
            .map(|entity| entity.id())
            .expect("respawn overlay should exist while visual state is active");
        let mut entity = world.entity_mut(overlay_entity);
        let mut overlay = entity
            .get_mut::<RespawnFadeOverlay>()
            .expect("overlay component missing timer");
        let remaining = overlay.timer.remaining_secs();
        overlay.timer.tick(Duration::from_secs_f32(remaining + 0.1));
    }
    advance_time(&mut app, 0.0);
    app.update();

    {
        let world = app.world();
        assert!(
            !world.resource::<RespawnVisualState>().active,
            "overlay state should clear once fade completes",
        );
        assert!(
            !world.entity(respawned_ball).contains::<BallFrozen>(),
            "ball should unfreeze once paddle is ready and overlay finishes",
        );
        assert!(
            !world.entity(paddle).contains::<InputLocked>(),
            "paddle should regain control after growth and fade",
        );
    }
}
