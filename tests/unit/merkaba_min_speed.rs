//! Unit tests for merkaba minimum horizontal speed enforcement (US2: T021)
//!
//! Tests that merkaba x-velocity is clamped to a minimum threshold.

use bevy::app::App;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::Velocity;

use brkrs::systems::merkaba::Merkaba;

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_message::<brkrs::signals::SpawnMerkabaMessage>()
        .add_message::<brkrs::signals::MerkabaWallCollision>()
        .add_message::<brkrs::signals::MerkabaBrickCollision>()
        .add_plugins(brkrs::systems::merkaba::MerkabaPlugin);
    app
}

/// T021: Min x-speed clamp ≥ 3.0 u/s enforced.
///
/// Merkaba MUST maintain a minimum horizontal speed threshold of 3.0 units/second
/// for movement in the x direction. If x-velocity drops below this threshold,
/// it MUST be clamped to ±3.0 u/s to prevent the merkaba from appearing stuck.
#[test]
#[ignore = "RED: T021 - Implement min x-speed enforcement (T025)"]
fn t021_minimum_x_speed_clamped_to_3_0() {
    let mut app = test_app();

    // Test case 1: Positive y-velocity below threshold
    let merkaba1 = app
        .world_mut()
        .spawn((
            Merkaba,
            Transform::default(),
            Velocity::linear(Vec3::new(1.5, 0.5, 0.0)), // X below 3.0
        ))
        .id();

    // Test case 2: Negative y-velocity above threshold (magnitude-wise)
    let merkaba2 = app
        .world_mut()
        .spawn((
            Merkaba,
            Transform::default(),
            Velocity::linear(Vec3::new(-1.0, -0.5, 0.0)), // X magnitude < 3.0
        ))
        .id();

    // Test case 3: Y-velocity already at or above threshold
    let merkaba3 = app
        .world_mut()
        .spawn((
            Merkaba,
            Transform::default(),
            Velocity::linear(Vec3::new(5.0, 0.2, 0.0)), // Already >= 3.0
        ))
        .id();

    app.update();

    // Verify all cases after min-speed enforcement
    let vel1 = app.world().entity(merkaba1).get::<Velocity>().unwrap();
    assert!(
        vel1.linvel.x.abs() >= 3.0,
        "Merkaba 1 x-speed should be clamped to >=3.0, got {}",
        vel1.linvel.x
    );
    assert_eq!(
        vel1.linvel.x, 3.0,
        "Positive x-velocity below 3.0 should be clamped to 3.0"
    );

    let vel2 = app.world().entity(merkaba2).get::<Velocity>().unwrap();
    assert!(
        vel2.linvel.x.abs() >= 3.0,
        "Merkaba 2 x-speed magnitude should be >=3.0, got {}",
        vel2.linvel.x
    );
    assert_eq!(
        vel2.linvel.x, -3.0,
        "Negative x-velocity above -3.0 should be clamped to -3.0"
    );

    let vel3 = app.world().entity(merkaba3).get::<Velocity>().unwrap();
    assert_eq!(
        vel3.linvel.x, 5.0,
        "X-velocity already >=3.0 should remain unchanged"
    );
}
