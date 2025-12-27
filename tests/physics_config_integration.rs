//! End-to-end tests for physics config usage in entity spawns

use bevy::prelude::*;
use bevy_rapier3d::prelude::{Damping, Friction, Restitution};
use brkrs::physics_config::{BallPhysicsConfig, PaddlePhysicsConfig};
use brkrs::{Ball, Paddle};

fn setup_app_with_ball_config(config: BallPhysicsConfig) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(config);
    // Insert other required resources for ball spawning
    app.insert_resource(brkrs::systems::respawn::SpawnPoints::default());
    app.insert_resource(bevy::asset::Assets::<bevy::prelude::Mesh>::default());
    app.insert_resource(bevy::asset::Assets::<bevy::prelude::StandardMaterial>::default());
    app.add_plugins(brkrs::systems::respawn::RespawnPlugin);
    app
}

#[test]
fn ball_spawn_uses_config() {
    let config = BallPhysicsConfig {
        restitution: 1.1,
        friction: 0.7,
        linear_damping: 0.2,
        angular_damping: 0.3,
        ..Default::default()
    };
    let mut app = setup_app_with_ball_config(config.clone());
    // Queue a respawn so a ball is spawned
    let lost_ball = {
        let world = app.world_mut();
        world.spawn_empty().id()
    };
    let timer_duration = {
        let world = app.world_mut();
        let schedule = world.resource::<brkrs::systems::respawn::RespawnSchedule>();
        schedule.timer.duration()
    };
    {
        let world = app.world_mut();
        let mut schedule = world.resource_mut::<brkrs::systems::respawn::RespawnSchedule>();
        schedule.pending = Some(brkrs::systems::respawn::RespawnRequest {
            lost_ball,
            tracked_paddle: None,
            remaining_lives: 3,
            ball_spawn: None,
            paddle_spawn: None,
        });
        schedule.timer.set_elapsed(timer_duration);
    }
    // Advance the timer and update the app multiple times to allow the respawn system to run
    for _ in 0..10 {
        // Advance time by 0.2 seconds per frame
        let mut time = app.world_mut().resource_mut::<bevy::time::Time>();
        time.advance_by(std::time::Duration::from_millis(200));
        drop(time);
        app.update();
    }
    let world = app.world_mut();
    let mut query = world.query::<(&Restitution, &Friction, &Damping, &Ball)>();
    let (restitution, friction, damping, _ball) = query
        .iter(world)
        .next()
        .expect("expected at least one Ball with physics components to be spawned");
    assert!((restitution.coefficient - config.restitution).abs() < f32::EPSILON);
    assert!((friction.coefficient - config.friction).abs() < f32::EPSILON);
    assert!((damping.linear_damping - config.linear_damping).abs() < f32::EPSILON);
    assert!((damping.angular_damping - config.angular_damping).abs() < f32::EPSILON);
}

fn setup_app_with_paddle_config(config: PaddlePhysicsConfig) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(config);
    app.insert_resource(brkrs::systems::respawn::SpawnPoints::default());
    app.insert_resource(bevy::asset::Assets::<bevy::prelude::Mesh>::default());
    app.insert_resource(bevy::asset::Assets::<bevy::prelude::StandardMaterial>::default());
    app.add_plugins(brkrs::systems::respawn::RespawnPlugin);
    app
}

#[test]
fn paddle_spawn_uses_config() {
    let config = PaddlePhysicsConfig {
        restitution: 0.8,
        friction: 0.6,
        linear_damping: 0.1,
        angular_damping: 0.2,
        ..Default::default()
    };
    let mut app = setup_app_with_paddle_config(config.clone());
    // Queue a respawn so a paddle is spawned
    let lost_ball = {
        let world = app.world_mut();
        world.spawn_empty().id()
    };
    let timer_duration = {
        let world = app.world_mut();
        let schedule = world.resource::<brkrs::systems::respawn::RespawnSchedule>();
        schedule.timer.duration()
    };
    {
        let world = app.world_mut();
        let mut schedule = world.resource_mut::<brkrs::systems::respawn::RespawnSchedule>();
        schedule.pending = Some(brkrs::systems::respawn::RespawnRequest {
            lost_ball,
            tracked_paddle: None,
            remaining_lives: 3,
            ball_spawn: None,
            paddle_spawn: None,
        });
        schedule.timer.set_elapsed(timer_duration);
    }
    // Advance the timer and update the app multiple times to allow the respawn system to run
    for _ in 0..10 {
        // Advance time by 0.2 seconds per frame
        let mut time = app.world_mut().resource_mut::<bevy::time::Time>();
        time.advance_by(std::time::Duration::from_millis(200));
        drop(time);
        app.update();
    }
    let world = app.world_mut();
    let mut query = world.query::<(&Restitution, &Friction, &Damping, &Paddle)>();
    let (restitution, friction, damping, _paddle) = query
        .iter(world)
        .next()
        .expect("expected at least one Paddle with physics components to be spawned");
    assert!((restitution.coefficient - config.restitution).abs() < f32::EPSILON);
    assert!((friction.coefficient - config.friction).abs() < f32::EPSILON);
    assert!((damping.linear_damping - config.linear_damping).abs() < f32::EPSILON);
    assert!((damping.angular_damping - config.angular_damping).abs() < f32::EPSILON);
}

// For Brick, you may need to use the level loader or a direct brick spawn system if available.
