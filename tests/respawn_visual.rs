use bevy::app::App;
use bevy::ecs::entity::EntityRow;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::time::Time;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::{CollisionEvent, Velocity};

use brkrs::systems::respawn::{
    InputLocked, RespawnFadeOverlay, RespawnPlugin, RespawnSchedule, RespawnScheduled,
    RespawnVisualState, SpawnPoints,
};
use brkrs::{Ball, BallFrozen, Paddle};

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

fn advance_time(app: &mut App, delta_secs: f32) {
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(Duration::from_secs_f32(delta_secs));
}

fn send_respawn_event(app: &mut App, ball: Entity, paddle: Option<Entity>) {
    let mut events = app.world_mut().resource_mut::<Messages<RespawnScheduled>>();
    events.write(RespawnScheduled {
        ball,
        paddle,
        completes_at: 0.0,
        remaining_lives: 2,
    });
}

fn finish_overlay_timer(app: &mut App) {
    let overlay_entity = {
        let mut query = app
            .world_mut()
            .query_filtered::<Entity, With<RespawnFadeOverlay>>();
        query
            .iter(app.world())
            .next()
            .expect("respawn overlay should exist")
    };
    let duration = {
        let world = app.world();
        world
            .entity(overlay_entity)
            .get::<RespawnFadeOverlay>()
            .expect("respawn overlay missing")
            .timer()
            .duration()
    };
    app.world_mut()
        .entity_mut(overlay_entity)
        .get_mut::<RespawnFadeOverlay>()
        .expect("respawn overlay missing")
        .timer_mut()
        .tick(duration + Duration::from_millis(100));
}

fn overlay_exists(app: &mut App) -> bool {
    let mut query = app
        .world_mut()
        .query_filtered::<Entity, With<RespawnFadeOverlay>>();
    query.iter(app.world()).next().is_some()
}

#[test]
fn overlay_spawns_and_clears_after_duration() {
    let mut app = test_app();

    send_respawn_event(
        &mut app,
        Entity::from_row(EntityRow::from_raw_u32(1).unwrap()),
        None,
    );
    advance_time(&mut app, 0.016);
    app.update();

    assert!(overlay_exists(&mut app));
    assert!(app.world().resource::<RespawnVisualState>().is_active());

    finish_overlay_timer(&mut app);
    advance_time(&mut app, 0.0);
    app.update();

    assert!(!overlay_exists(&mut app));
    assert!(
        !app.world().resource::<RespawnVisualState>().is_active(),
        "visual state should clear once overlay despawns",
    );
}

#[test]
fn controls_unlock_only_after_overlay_finishes() {
    let mut app = test_app();

    let paddle = app.world_mut().spawn((Paddle, InputLocked)).id();
    let ball = app
        .world_mut()
        .spawn((Ball, BallFrozen, Velocity::zero()))
        .id();

    send_respawn_event(&mut app, ball, Some(paddle));
    advance_time(&mut app, 0.016);
    app.update();

    {
        let world = app.world();
        assert!(world.entity(paddle).contains::<InputLocked>());
        assert!(world.entity(ball).contains::<BallFrozen>());
    }

    {
        let mut schedule = app.world_mut().resource_mut::<RespawnSchedule>();
        schedule.pending = None;
        schedule.queue.clear();
    }
    advance_time(&mut app, 0.016);
    app.update();

    assert!(
        app.world().entity(paddle).contains::<InputLocked>(),
        "controls should remain locked while overlay is active",
    );

    finish_overlay_timer(&mut app);
    advance_time(&mut app, 0.0);
    app.update();

    let world = app.world();
    assert!(
        !world.entity(paddle).contains::<InputLocked>(),
        "paddle unlocks after overlay completes",
    );
    assert!(
        !world.entity(ball).contains::<BallFrozen>(),
        "ball defrosts after overlay completes",
    );
}
