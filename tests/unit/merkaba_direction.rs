//! Unit tests for merkaba initial direction (US1: T012)
//!
//! Tests that merkaba initial velocity is initialized with ±20° angle
//! variance from pure horizontal (y-direction).

use std::time::Duration;

use bevy::app::App;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::{CollisionEvent, Velocity};
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use brkrs::register_brick_collision_systems;
use brkrs::signals::SpawnMerkabaMessage;
use brkrs::systems::merkaba::Merkaba;
use brkrs::{Ball, Brick, BrickTypeId, CountsTowardsCompletion};

const ROTOR_BRICK_INDEX: u8 = 36;

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(Assets::<Mesh>::default())
        .insert_resource(Assets::<StandardMaterial>::default())
        .add_message::<CollisionEvent>()
        .add_message::<SpawnMerkabaMessage>()
        .add_plugins(brkrs::systems::merkaba::MerkabaPlugin);

    register_brick_collision_systems(&mut app);
    app
}

fn trigger_collision(app: &mut App, e1: Entity, e2: Entity) {
    app.world_mut()
        .resource_mut::<Messages<CollisionEvent>>()
        .write(CollisionEvent::Started(
            e1,
            e2,
            CollisionEventFlags::empty(),
        ));
}

fn advance_time(app: &mut App, delta_secs: f32) {
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(Duration::from_secs_f32(delta_secs));
}

/// T012: Assert initial velocity in y-direction with ±20° random angle variance.
///
/// Merkaba MUST spawn with initial velocity in the horizontal (y) direction
/// with a random angle variance of ±20 degrees from pure horizontal.
/// This ensures variability in spawn behavior while keeping movement within bounds.
#[test]
fn t012_initial_velocity_angle_variance_within_20_degrees() {
    let mut app = test_app();

    let ball = app.world_mut().spawn(Ball).id();
    let rotor_brick = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(ROTOR_BRICK_INDEX),
            CountsTowardsCompletion,
        ))
        .id();

    // Trigger spawn path and advance past the 0.5s delay
    trigger_collision(&mut app, ball, rotor_brick);
    advance_time(&mut app, 0.6);
    app.update();

    // Collect merkabas with velocities
    let mut query = app
        .world()
        .query::<(&Velocity, &Transform), With<Merkaba>>();
    let merkabas: Vec<(Velocity, Transform)> = query
        .iter(app.world())
        .map(|(v, t)| (v.clone(), t.clone()))
        .collect();

    assert!(
        !merkabas.is_empty(),
        "Merkaba should spawn and carry initial velocity"
    );

    let mut saw_nonzero_variance = false;
    for (velocity, _transform) in merkabas.iter() {
        let angle_rad = velocity.linvel.y.atan2(velocity.linvel.x);
        let angle_deg = angle_rad.to_degrees();
        let variance = angle_deg.abs();
        if variance > 0.0 {
            saw_nonzero_variance = true;
        }
        assert!(
            variance <= 20.0,
            "Initial y-direction variance should be within ±20°, got {:.2}°",
            variance
        );
    }

    assert!(
        saw_nonzero_variance,
        "At least one merkaba should have a non-zero angle variance to demonstrate randomness"
    );
}
