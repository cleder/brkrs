use super::tests::{advance_time, ball_handle_at, paddle_handle_at, test_app};
use super::*;
use bevy::ecs::message::Messages;
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

    {
        let mut collisions = app.world_mut().resource_mut::<Messages<CollisionEvent>>();
        collisions.write(CollisionEvent::Started(
            ball_a,
            lower_goal,
            CollisionEventFlags::SENSOR,
        ));
    }

    advance_time(&mut app, 0.016);
    app.update();

    {
        let schedule = app.world().resource::<RespawnSchedule>();
        assert_eq!(schedule.queue.len(), 0);
        assert!(schedule.pending.is_none());
    }

    {
        let mut collisions = app.world_mut().resource_mut::<Messages<CollisionEvent>>();
        collisions.write(CollisionEvent::Started(
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
        assert_eq!(schedule.pending.as_ref().unwrap().lost_ball, ball_b);
    }

    {
        let mut schedule = app.world_mut().resource_mut::<RespawnSchedule>();
        let duration = schedule.timer.duration();
        schedule.timer.tick(duration + Duration::from_millis(100));
    }
    app.update();
    {
        let schedule = app.world().resource::<RespawnSchedule>();
        assert_eq!(schedule.queue.len(), 0);
        assert!(schedule.pending.is_none());
    }
    app.update();

    let completions = app.world().resource::<Messages<RespawnCompleted>>();
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
        lives.lives_remaining = 1; // Start with 1 life so losing all balls triggers game over
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
        let mut collisions = app.world_mut().resource_mut::<Messages<CollisionEvent>>();
        collisions.write(CollisionEvent::Started(
            ball_a,
            lower_goal,
            CollisionEventFlags::SENSOR,
        ));
    }
    advance_time(&mut app, 0.016);
    app.update();

    // After first collision, lives should NOT be decremented because ball_b is still active
    {
        let lives = app.world().resource::<LivesState>();
        assert_eq!(
            lives.lives_remaining, 1,
            "lives should not decrement when other balls remain"
        );
    }

    {
        let mut collisions = app.world_mut().resource_mut::<Messages<CollisionEvent>>();
        collisions.write(CollisionEvent::Started(
            ball_b,
            lower_goal,
            CollisionEventFlags::SENSOR,
        ));
    }

    advance_time(&mut app, 0.016);
    app.update();

    // After second collision (all balls lost), lives should be decremented from 1 to 0
    {
        let lives = app.world().resource::<LivesState>();
        assert_eq!(
            lives.lives_remaining, 0,
            "lives should hit zero when all balls are lost"
        );
    }

    {
        let schedule = app.world().resource::<RespawnSchedule>();
        assert_eq!(schedule.queue.len(), 0);
        assert!(schedule.pending.is_none());
    }

    let game_over_events = app.world().resource::<Messages<GameOverRequested>>();
    assert!(
        !game_over_events.is_empty(),
        "expected GameOverRequested to fire when lives hit zero"
    );
}
