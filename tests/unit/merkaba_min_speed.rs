//! Unit tests for merkaba minimum forward speed enforcement (US2: T021)
//!
//! Tests that merkaba z-velocity is clamped to a minimum threshold.

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

/// T021: Min z-speed clamp ≥ 3.0 u/s enforced.
///
/// Merkaba MUST maintain a minimum forward speed threshold of 3.0 units/second
/// for movement in the z direction. If z-velocity drops below this threshold,
/// it MUST be clamped to ±3.0 u/s to prevent the merkaba from appearing stuck.
#[test]
#[ignore = "RED: T021 - Implement min z-speed enforcement (T025)"]
fn t021_minimum_z_speed_clamped_to_3_0() {
    let mut app = test_app();

    // Test case 1: Positive z-velocity below threshold
    let merkaba1 = app
        .world_mut()
        .spawn((
            Merkaba,
            Transform::default(),
            Velocity::linear(Vec3::new(0.5, 0.0, 1.5)), // Z below 3.0
        ))
        .id();

    // Test case 2: Negative z-velocity below threshold (magnitude-wise)
    let merkaba2 = app
        .world_mut()
        .spawn((
            Merkaba,
            Transform::default(),
            Velocity::linear(Vec3::new(-0.5, 0.0, -1.0)), // Z magnitude < 3.0
        ))
        .id();

    // Test case 3: Z-velocity already at or above threshold
    let merkaba3 = app
        .world_mut()
        .spawn((
            Merkaba,
            Transform::default(),
            Velocity::linear(Vec3::new(0.2, 0.0, 5.0)), // Z already >= 3.0
        ))
        .id();

    app.update();

    // Verify all cases after min-speed enforcement
    let vel1 = app.world().entity(merkaba1).get::<Velocity>().unwrap();
    assert!(
        vel1.linvel.z.abs() >= 3.0,
        "Merkaba 1 z-speed should be clamped to >=3.0, got {}",
        vel1.linvel.z
    );
    assert_eq!(
        vel1.linvel.z, 3.0,
        "Positive z-velocity below 3.0 should be clamped to 3.0"
    );

    let vel2 = app.world().entity(merkaba2).get::<Velocity>().unwrap();
    assert!(
        vel2.linvel.z.abs() >= 3.0,
        "Merkaba 2 z-speed magnitude should be >=3.0, got {}",
        vel2.linvel.z
    );
    assert_eq!(
        vel2.linvel.z, -3.0,
        "Negative z-velocity below -3.0 should be clamped to -3.0"
    );

    let vel3 = app.world().entity(merkaba3).get::<Velocity>().unwrap();
    assert_eq!(
        vel3.linvel.z, 5.0,
        "Z-velocity already >=3.0 should remain unchanged"
    );
}
