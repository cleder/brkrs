use super::tests::{advance_time, test_app};
use super::*;
use bevy::ecs::entity::EntityRow;
use bevy::ecs::message::Messages;
use bevy_rapier3d::prelude::{ExternalImpulse, Velocity};
use std::time::Duration;

#[test]
fn overlay_spawns_on_respawn_schedule() {
    let mut app = test_app();

    {
        let mut events = app.world_mut().resource_mut::<Messages<RespawnScheduled>>();
        events.write(RespawnScheduled {
            ball: Entity::from_row(EntityRow::from_raw_u32(1).unwrap()),
            paddle: None,
            completes_at: 0.0,
            remaining_lives: 3,
        });
    }

    app.update();

    let overlay_exists = {
        let mut query = app.world_mut().query_filtered::<Entity, With<RespawnFadeOverlay>>();
        query.iter(app.world()).next().is_some()
    };
    assert!(
        overlay_exists,
        "respawn overlay should spawn when schedule event fires"
    );
    assert!(app.world().resource::<RespawnVisualState>().active);
}

#[test]
fn controls_wait_until_overlay_finishes() {
    let mut app = test_app();
    let paddle = app.world_mut().spawn((Paddle, InputLocked)).id();
    let ball = app
        .world_mut()
        .spawn((
            Ball,
            BallFrozen,
            Velocity::zero(),
            ExternalImpulse::default(),
        ))
        .id();

    {
        let mut events = app.world_mut().resource_mut::<Messages<RespawnScheduled>>();
        events.write(RespawnScheduled {
            ball,
            paddle: Some(paddle),
            completes_at: 0.0,
            remaining_lives: 2,
        });
    }

    app.update();

    {
        let world = app.world();
        assert!(world.entity(paddle).contains::<InputLocked>());
        assert!(world.entity(ball).contains::<BallFrozen>());
    }

    // With overlay active and no respawn pending, controls should remain locked.
    {
        app.world_mut().resource_mut::<RespawnSchedule>().pending = None;
        app.update();
        let world = app.world();
        assert!(world.entity(paddle).contains::<InputLocked>());
    }

    // Finish overlay timer and ensure control unlocks afterwards.
    let overlay_entity = {
        let mut query = app.world_mut().query_filtered::<Entity, With<RespawnFadeOverlay>>();
        query.iter(app.world()).next().expect("overlay entity should exist")
    };
    {
        let world = app.world_mut();
        let mut entity = world.entity_mut(overlay_entity);
        let mut overlay = entity
            .get_mut::<RespawnFadeOverlay>()
            .expect("overlay component missing");
        let advance = overlay.timer.duration().as_secs_f32() + 0.1;
        overlay.timer.tick(Duration::from_secs_f32(advance));
    }
    advance_time(&mut app, 0.0);
    app.update();

    let world = app.world();
    assert!(
        !world.resource::<RespawnVisualState>().active,
        "overlay visual state should be inactive once the fade completes",
    );
    assert!(
        !world.entity(paddle).contains::<InputLocked>(),
        "paddle should unlock after overlay completes",
    );
    assert!(
        !world.entity(ball).contains::<BallFrozen>(),
        "ball should unfreeze when controls return",
    );
}
