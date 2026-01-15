//! Unit tests for merkaba z-plane constraint (US2: T022c)
//!
//! Tests that merkaba z-position remains within tolerance bounds (0 ± 0.01 units)
//! under collisions and rotation.

use bevy::app::App;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::Velocity;

use brkrs::systems::merkaba::Merkaba;

const Z_PLANE_TOLERANCE: f32 = 0.01;

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_message::<brkrs::signals::SpawnMerkabaMessage>()
        .add_message::<brkrs::signals::MerkabaWallCollision>()
        .add_message::<brkrs::signals::MerkabaBrickCollision>()
        .add_plugins(brkrs::systems::merkaba::MerkabaPlugin);
    app
}

/// T022c: Z-position remains within tolerance (0 ± 0.01 units) under collisions/rotation.
///
/// Merkaba MUST stay constrained to the gaming plane (z ≈ 0). If z-position
/// drifts beyond ±0.01 units due to physics or rotation, the system MUST
/// enforce correction via collision constraint or clamping.
#[test]
#[ignore = "RED: T022c - Implement z-plane constraint (T026)"]
fn t022c_merkaba_z_plane_constrained_to_tolerance() {
    let mut app = test_app();

    let merkaba = app
        .world_mut()
        .spawn((
            Merkaba,
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            Velocity::linear(Vec3::new(0.0, 3.0, 0.1)), // Small z-velocity drift
        ))
        .id();

    // Run for multiple frames to test z-constraint enforcement
    for frame in 0..10 {
        app.update();

        let transform = app.world().entity(merkaba).get::<Transform>().unwrap();
        let z_abs = transform.translation.z.abs();
        assert!(
            z_abs <= Z_PLANE_TOLERANCE,
            "Frame {}: Merkaba z-position exceeded tolerance: {}",
            frame,
            z_abs
        );
    }
}
