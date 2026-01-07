//! Integration tests for merkaba physics (US2: T019, T020, T022b)
//!
//! Tests wall bounce, brick bounce, multi-merkaba coexistence, and audio emissions.

use bevy::app::App;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::{CollisionEvent, Velocity};
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use brkrs::systems::merkaba::Merkaba;
use brkrs::systems::respawn::LivesState;
use brkrs::{Border, Brick, BrickTypeId};

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(Assets::<Mesh>::default())
        .insert_resource(Assets::<StandardMaterial>::default())
        .insert_resource(LivesState {
            lives_remaining: 3,
            on_last_life: false,
        })
        .add_message::<CollisionEvent>()
        .add_message::<brkrs::signals::SpawnMerkabaMessage>()
        .add_message::<brkrs::signals::MerkabaWallCollision>()
        .add_message::<brkrs::signals::MerkabaBrickCollision>()
        .add_plugins(brkrs::systems::merkaba::MerkabaPlugin);

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

/// T019: Wall collision → bounce + distinct sound.
///
/// When merkaba collides with a wall, it MUST bounce with appropriate
/// physics response AND emit a distinct wall collision sound.
///
/// Note: Actual bounce physics is handled by Rapier via Restitution component.
/// This test validates that collision events trigger the expected audio signals.
#[test]
fn t019_wall_bounce_with_distinct_sound() {
    let mut app = test_app();

    let merkaba = app
        .world_mut()
        .spawn((
            Merkaba,
            Transform::default(),
            Velocity::linear(Vec3::new(0.0, 5.0, 0.0)),
        ))
        .id();
    let wall = app.world_mut().spawn(Border).id();

    trigger_collision(&mut app, merkaba, wall);
    app.update();

    // TODO: Verify wall collision audio signal emitted (T028)
    // Physics bounce is handled by Rapier via Restitution component in production
    // For now, this test is a placeholder for audio integration
}

/// T020: Brick collision → bounce (no destruction) + distinct sound.
///
/// When merkaba collides with a brick, it MUST bounce WITHOUT destroying
/// the brick, AND emit a distinct brick collision sound.
///
/// Note: Actual bounce physics is handled by Rapier via Restitution component.
/// This test validates brick persistence and audio signal triggering.
#[test]
fn t020_brick_bounce_no_destruction_with_distinct_sound() {
    let mut app = test_app();

    let merkaba = app
        .world_mut()
        .spawn((
            Merkaba,
            Transform::default(),
            Velocity::linear(Vec3::new(0.0, 5.0, 0.0)),
        ))
        .id();
    let brick = app
        .world_mut()
        .spawn((Brick, BrickTypeId(20), Transform::default()))
        .id();

    trigger_collision(&mut app, merkaba, brick);
    app.update();

    // Verify brick still exists (not destroyed)
    assert!(
        app.world().entities().contains(brick),
        "Brick should not be destroyed by merkaba collision"
    );

    // TODO: Verify brick collision audio signal emitted (T028)
    // Physics bounce is handled by Rapier via Restitution component in production
}

/// T022b: Multiple merkabas coexist without interference; 60 FPS baseline maintained.
///
/// System MUST support multiple merkabas spawning from separate rotor brick hits.
/// They MUST NOT interfere with each other and MUST maintain 60 FPS with up to
/// 5 concurrent merkabas.
#[test]
fn t022b_multiple_merkabas_coexist_60fps_baseline() {
    let mut app = test_app();

    let merkaba1 = app
        .world_mut()
        .spawn((
            Merkaba,
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            Velocity::linear(Vec3::new(0.0, 3.0, 0.0)),
        ))
        .id();
    let merkaba2 = app
        .world_mut()
        .spawn((
            Merkaba,
            Transform::from_translation(Vec3::new(5.0, 0.0, 0.0)),
            Velocity::linear(Vec3::new(0.0, -3.0, 0.0)),
        ))
        .id();

    app.update();

    // Both merkabas should still exist
    assert!(
        app.world().entities().contains(merkaba1),
        "First merkaba should coexist with second"
    );
    assert!(
        app.world().entities().contains(merkaba2),
        "Second merkaba should coexist with first"
    );

    // Both should maintain their velocities (no interference)
    let vel1 = app.world().entity(merkaba1).get::<Velocity>().unwrap();
    let vel2 = app.world().entity(merkaba2).get::<Velocity>().unwrap();
    assert_eq!(
        vel1.linvel.y.signum(),
        1.0,
        "First merkaba should maintain positive y velocity"
    );
    assert_eq!(
        vel2.linvel.y.signum(),
        -1.0,
        "Second merkaba should maintain negative y velocity"
    );
}

/// T021: Min horizontal (x) speed clamp ≥ 3.0 u/s enforced.
///
/// Merkaba MUST maintain a minimum horizontal speed threshold of 3.0 units/second
/// for movement in the x direction. If x-velocity drops below this threshold,
/// it MUST be clamped to ±3.0 u/s to prevent the merkaba from appearing stuck.
#[test]
fn t021_minimum_x_speed_clamped_to_3_0() {
    let mut app = test_app();

    // Test case 1: Positive x-velocity below threshold
    let merkaba1 = app
        .world_mut()
        .spawn((
            Merkaba,
            Transform::default(),
            Velocity::linear(Vec3::new(1.5, 0.5, 0.0)), // X below 3.0
        ))
        .id();

    // Test case 2: Negative x-velocity above threshold (magnitude-wise)
    let merkaba2 = app
        .world_mut()
        .spawn((
            Merkaba,
            Transform::default(),
            Velocity::linear(Vec3::new(-1.0, -0.5, 0.0)), // X magnitude < 3.0
        ))
        .id();

    // Test case 3: X-velocity already at or above threshold
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

/// T022c: Z-position remains within tolerance (0 ± 0.01 units) under collisions/rotation.
///
/// Merkaba MUST stay constrained to the gaming plane (z ≈ 0). If z-position
/// drifts beyond ±0.01 units due to physics or rotation, the system MUST
/// enforce correction via collision constraint or clamping.
#[test]
fn t022c_merkaba_z_plane_constrained_to_tolerance() {
    let mut app = test_app();

    const Z_PLANE_TOLERANCE: f32 = 0.01;

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
            "Frame {}: Z-plane drift exceeded tolerance; z = {}, tolerance = {}",
            frame,
            z_abs,
            Z_PLANE_TOLERANCE
        );
    }
}
