use super::tests::{advance_time, ball_handle_at, paddle_handle_at, test_app};
use super::*;
use bevy::ecs::event::Events;
use bevy_rapier3d::prelude::CollisionEvent;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;
use std::time::Duration;

#[test]
fn sequential_life_losses_complete_in_order() {
    let mut app = test_app();

    let lower_goal = app.world_mut().spawn(LowerGoal).id();
    let ball_a = app
        .world_mut()
        .spawn((Ball, ball_handle_at(Vec3::new(0.0, 2.0, 0.0))))
        .id();
    let ball_b = app
        .world_mut()
        .spawn((Ball, ball_handle_at(Vec3::new(1.0, 2.0, 0.0))))
        .id();
    app.world_mut().spawn((
        Paddle,
        Transform::default(),
        paddle_handle_at(Vec3::new(0.0, 2.0, 0.0)),
    ));

    let mut collisions = app.world_mut().resource_mut::<Events<CollisionEvent>>();
    collisions.send(CollisionEvent::Started(
        ball_a,
        lower_goal,
        CollisionEventFlags::SENSOR,
    ));
    drop(collisions);

    advance_time(&mut app, 0.016);
    app.update();

    {
        let schedule = app.world().resource::<RespawnSchedule>();
        assert_eq!(schedule.queue.len(), 0);
        assert_eq!(schedule.pending.as_ref().unwrap().lost_ball, ball_a);
    }

    let mut collisions = app.world_mut().resource_mut::<Events<CollisionEvent>>();
    collisions.send(CollisionEvent::Started(
        ball_b,
        lower_goal,
        CollisionEventFlags::SENSOR,
    ));
    drop(collisions);

    advance_time(&mut app, 0.016);
    app.update();

    {
        let schedule = app.world().resource::<RespawnSchedule>();
        assert_eq!(schedule.queue.len(), 1);
    }

    {
        let mut schedule = app.world_mut().resource_mut::<RespawnSchedule>();
        let duration = schedule.timer.duration();
        schedule.timer.tick(duration + Duration::from_millis(100));
    }
    app.update();
    {
        let schedule = app.world().resource::<RespawnSchedule>();
        assert_eq!(schedule.queue.len(), 1);
        assert!(schedule.pending.is_none());
    }
    app.update();

    {
        let schedule = app.world().resource::<RespawnSchedule>();
        assert_eq!(schedule.queue.len(), 0);
        assert_eq!(schedule.pending.as_ref().unwrap().lost_ball, ball_b);
    }

    let completions = app.world().resource::<Events<RespawnCompleted>>();
    assert!(
        !completions.is_empty(),
        "expected at least one respawn completion event"
    );
}

#[test]
fn game_over_halts_additional_respawns() {
    let mut app = test_app();

    {
        let mut lives = app.world_mut().resource_mut::<LivesState>();
        lives.lives_remaining = 1;
    }

    let lower_goal = app.world_mut().spawn(LowerGoal).id();
    let ball_a = app
        .world_mut()
        .spawn((Ball, ball_handle_at(Vec3::new(0.0, 2.0, 0.0))))
        .id();
    let ball_b = app
        .world_mut()
        .spawn((Ball, ball_handle_at(Vec3::new(1.0, 2.0, 0.0))))
        .id();
    app.world_mut().spawn((
        Paddle,
        Transform::default(),
        paddle_handle_at(Vec3::new(0.0, 2.0, 0.0)),
    ));

    {
        let mut collisions = app.world_mut().resource_mut::<Events<CollisionEvent>>();
        collisions.send(CollisionEvent::Started(
            ball_a,
            lower_goal,
            CollisionEventFlags::SENSOR,
        ));
    }
    advance_time(&mut app, 0.016);
    app.update();

    {
        let mut lives = app.world_mut().resource_mut::<LivesState>();
        lives.lives_remaining = 0;
    }

    {
        let mut collisions = app.world_mut().resource_mut::<Events<CollisionEvent>>();
        collisions.send(CollisionEvent::Started(
            ball_b,
            lower_goal,
            CollisionEventFlags::SENSOR,
        ));
    }

    advance_time(&mut app, 0.016);
    app.update();

    {
        let schedule = app.world().resource::<RespawnSchedule>();
        assert_eq!(schedule.queue.len(), 0);
        assert!(schedule.pending.is_some());
        assert_eq!(schedule.pending.as_ref().unwrap().lost_ball, ball_a);
    }

    let game_over_events = app.world().resource::<Events<GameOverRequested>>();
    assert!(
        !game_over_events.is_empty(),
        "expected GameOverRequested to fire when lives hit zero"
    );
}
