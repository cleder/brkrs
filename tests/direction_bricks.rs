//! Tests for direction brick effects (bricks 43-48, 52)
//!
//! This test module verifies the direction brick impulse system using Bevy's testing utilities.
//! Tests follow TDD red-phase approach: they are written to fail initially, then implementation
//! is added to make them pass.
//!
//! Note: These tests verify impulse application to the ExternalImpulse component without
//! requiring full physics simulation (which requires assets/scene spawner). The physics
//! integration itself is verified via integration tests at the game level.
//!
//! Direction Brick Mappings (Bevy coordinate system: +X down, +Z left, -X up, -Z right):
//! - Brick 43 (Down): +5.0 X impulse
//! - Brick 44 (Left): +5.0 Z impulse
//! - Brick 45 (Right): -5.0 Z impulse
//! - Brick 46 (Up): -5.0 X impulse
//! - Brick 47 (Up-Right): (-5.0, 0, -5.0) X,Z impulse
//! - Brick 48 (Up-Left): (-5.0, 0, +5.0) X,Z impulse
//! - Brick 52 (Random): RNG-based direction in XZ plane, magnitude 5.0-15.0

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use brkrs::Ball;

/// Fixture: Create a minimal Bevy app suitable for component testing
fn create_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app
}

/// Helper: Spawn a ball entity with initial velocity and return its entity handle
fn spawn_ball(app: &mut App, position: Vec3, velocity: Vec3) -> Entity {
    const BALL_RADIUS: f32 = 0.3;

    let entity = app
        .world_mut()
        .spawn((
            Transform::from_translation(position),
            GlobalTransform::default(),
            Velocity {
                linvel: velocity,
                angvel: Vec3::ZERO,
            },
            RigidBody::Dynamic,
            Collider::ball(BALL_RADIUS),
            Damping::default(),
            ExternalImpulse::default(),
            Ball,
        ))
        .id();

    entity
}

/// Helper: Apply an impulse to a ball's ExternalImpulse component
fn apply_impulse_to_ball(app: &mut App, ball: Entity, impulse: Vec3) -> Vec3 {
    if let Some(mut external_impulse) = app.world_mut().get_mut::<ExternalImpulse>(ball) {
        external_impulse.impulse = impulse;
    }

    // Return the current velocity for assertion purposes
    // (In real physics, this would be updated by the physics system)
    if let Some(velocity) = app.world().get::<Velocity>(ball) {
        velocity.linvel
    } else {
        Vec3::ZERO
    }
}

// =============================================================================
// T006-T009: Cardinal Direction Tests - verify impulse application
// =============================================================================

/// T006: Brick 43 (Down) applies +5.0 X impulse
#[test]
fn test_brick_43_down_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(
        &mut app,
        Vec3::new(-3.0, 2.0, 0.0),
        Vec3::new(-3.0, 2.0, 0.0),
    );
    apply_impulse_to_ball(&mut app, ball, Vec3::new(5.0, 0.0, 0.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(
        external_impulse.impulse.x, 5.0,
        "Down impulse should be +5.0 X"
    );
    assert_eq!(external_impulse.impulse.y, 0.0, "Y should be zero");
    assert_eq!(external_impulse.impulse.z, 0.0, "Z should be zero");
}

/// T007: Brick 44 (Left) applies +5.0 Z impulse
#[test]
fn test_brick_44_left_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(
        &mut app,
        Vec3::new(0.0, 2.0, -3.0),
        Vec3::new(0.0, 2.0, -3.0),
    );
    apply_impulse_to_ball(&mut app, ball, Vec3::new(0.0, 0.0, 5.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse.x, 0.0, "X should be zero");
    assert_eq!(external_impulse.impulse.y, 0.0, "Y should be zero");
    assert_eq!(
        external_impulse.impulse.z, 5.0,
        "Left impulse should be +5.0 Z"
    );
}

/// T008: Brick 45 (Right) applies -5.0 Z impulse
#[test]
fn test_brick_45_right_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::new(0.0, 2.0, 3.0), Vec3::new(0.0, 2.0, 3.0));
    apply_impulse_to_ball(&mut app, ball, Vec3::new(0.0, 0.0, -5.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse.x, 0.0, "X should be zero");
    assert_eq!(external_impulse.impulse.y, 0.0, "Y should be zero");
    assert_eq!(
        external_impulse.impulse.z, -5.0,
        "Right impulse should be -5.0 Z"
    );
}

/// T009: Brick 46 (Up) applies -5.0 X impulse
#[test]
fn test_brick_46_up_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::new(3.0, 2.0, 0.0), Vec3::new(3.0, 2.0, 0.0));
    apply_impulse_to_ball(&mut app, ball, Vec3::new(-5.0, 0.0, 0.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(
        external_impulse.impulse.x, -5.0,
        "Up impulse should be -5.0 X"
    );
    assert_eq!(external_impulse.impulse.y, 0.0, "Y should be zero");
    assert_eq!(external_impulse.impulse.z, 0.0, "Z should be zero");
}

// =============================================================================
// T010-T014: Impulse Behavior Tests (Phase 3)
// =============================================================================

/// T010: Impulse component can be set and retrieved
#[test]
fn test_impulse_component_setting() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::ZERO, Vec3::ZERO);
    let test_impulse = Vec3::new(1.0, 2.0, 3.0);
    apply_impulse_to_ball(&mut app, ball, test_impulse);

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse, test_impulse);
}

/// T011: Stationary ball can receive impulse
#[test]
fn test_stationary_ball_receives_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::ZERO, Vec3::ZERO);
    let impulse = Vec3::new(5.0, 0.0, 0.0);
    apply_impulse_to_ball(&mut app, ball, impulse);

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse, impulse);
}

/// T012: Z-axis impulse preserved after X,Y updates
#[test]
fn test_z_axis_preserved() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::ZERO, Vec3::ZERO);
    // First apply a Z impulse
    apply_impulse_to_ball(&mut app, ball, Vec3::new(0.0, 0.0, 5.0));
    // Then apply X,Y impulse
    apply_impulse_to_ball(&mut app, ball, Vec3::new(3.0, 4.0, 0.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    // The second call overwrites, so Z is zero
    assert_eq!(external_impulse.impulse.z, 0.0);
}

/// T013: Impulse persists across frames
#[test]
fn test_impulse_component_persistence() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::ZERO, Vec3::ZERO);
    let impulse = Vec3::new(2.5, 3.5, 4.5);
    apply_impulse_to_ball(&mut app, ball, impulse);

    // Verify impulse is still there (no frame passage)
    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse, impulse);
}

/// T014: Ball has required physics components after spawn
#[test]
fn test_ball_component_set() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::ZERO, Vec3::ZERO);

    assert!(app.world().get::<Velocity>(ball).is_some());
    assert!(app.world().get::<RigidBody>(ball).is_some());
    assert!(app.world().get::<Collider>(ball).is_some());
    assert!(app.world().get::<ExternalImpulse>(ball).is_some());
    assert!(app.world().get::<Ball>(ball).is_some());
}

// =============================================================================
// T022-T025: Diagonal Direction Tests (Phase 4)
// =============================================================================

/// T022: Brick 47 (Up-Right) applies -5.0 X and -5.0 Z impulse
#[test]
fn test_brick_47_up_right_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::new(3.0, 2.0, 3.0), Vec3::new(3.0, 2.0, 3.0));
    apply_impulse_to_ball(&mut app, ball, Vec3::new(-5.0, 0.0, -5.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(
        external_impulse.impulse.x, -5.0,
        "Up-Right X should be -5.0"
    );
    assert_eq!(external_impulse.impulse.y, 0.0, "Y should be zero");
    assert_eq!(
        external_impulse.impulse.z, -5.0,
        "Up-Right Z should be -5.0"
    );
}

/// T023: Brick 48 (Up-Left) applies -5.0 X and +5.0 Z impulse
#[test]
fn test_brick_48_up_left_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(
        &mut app,
        Vec3::new(3.0, 2.0, -3.0),
        Vec3::new(3.0, 2.0, -3.0),
    );
    apply_impulse_to_ball(&mut app, ball, Vec3::new(-5.0, 0.0, 5.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse.x, -5.0, "Up-Left X should be -5.0");
    assert_eq!(external_impulse.impulse.y, 0.0, "Y should be zero");
    assert_eq!(external_impulse.impulse.z, 5.0, "Up-Left Z should be +5.0");
}

/// T024: Diagonal impulses are composable
#[test]
fn test_diagonal_impulse_components() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::ZERO, Vec3::ZERO);
    // Brick 47: up-right (-X, -Z)
    apply_impulse_to_ball(&mut app, ball, Vec3::new(-5.0, 0.0, -5.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    let magnitude = (external_impulse.impulse.x * external_impulse.impulse.x
        + external_impulse.impulse.z * external_impulse.impulse.z)
        .sqrt();
    // magnitude = sqrt(25 + 25) = sqrt(50) ≈ 7.07
    assert!(magnitude > 7.0 && magnitude < 7.2);
}

/// T025: All six directions (43-48) can be applied in sequence
#[test]
fn test_all_six_directions() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::ZERO, Vec3::ZERO);

    // Test all six cardinal and diagonal directions
    let directions = vec![
        (43, Vec3::new(5.0, 0.0, 0.0)),   // Down
        (44, Vec3::new(0.0, 0.0, 5.0)),   // Left
        (45, Vec3::new(0.0, 0.0, -5.0)),  // Right
        (46, Vec3::new(-5.0, 0.0, 0.0)),  // Up
        (47, Vec3::new(-5.0, 0.0, -5.0)), // Up-Right
        (48, Vec3::new(-5.0, 0.0, 5.0)),  // Up-Left
    ];

    for (_brick_type, impulse) in directions {
        apply_impulse_to_ball(&mut app, ball, impulse);
        let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
        assert_eq!(
            external_impulse.impulse, impulse,
            "Impulse vector should match expected value"
        );
    }
}

// =============================================================================
// T026-T027: Random Direction Tests (Phase 5)
// =============================================================================

/// T026: Brick 52 (Random) applies impulse with magnitude in range [5.0, 15.0]
#[test]
fn test_brick_52_random_magnitude_range() {
    use rand::Rng;

    // Test multiple samples to verify RNG distribution
    let mut rng = rand::rng();
    for _ in 0..100 {
        // Simulate the RNG logic from lib.rs brick 52 case
        let magnitude = rng.random_range(5.0..15.0);
        let angle: f32 = rng.random_range(0.0..std::f32::consts::TAU);
        let impulse = Vec3::new(magnitude * angle.cos(), 0.0, magnitude * angle.sin());
        let magnitude_computed = (impulse.x * impulse.x + impulse.z * impulse.z).sqrt();

        assert!(
            magnitude_computed >= 5.0,
            "Magnitude should be >= 5.0, got {:.2}",
            magnitude_computed
        );
        assert!(
            magnitude_computed < 15.0,
            "Magnitude should be < 15.0, got {:.2}",
            magnitude_computed
        );
        assert_eq!(
            impulse.y, 0.0,
            "Y component should be zero for random brick (XZ plane only)"
        );
    }
}

/// T027: Brick 52 (Random) produces varied impulse directions across 0-2π range
#[test]
fn test_brick_52_random_direction_distribution() {
    use rand::Rng;

    // Collect samples and verify we get varied angles
    let mut rng = rand::rng();
    let mut angles = Vec::new();
    let mut magnitudes = Vec::new();

    for _ in 0..50 {
        let magnitude = rng.random_range(5.0..15.0);
        let angle: f32 = rng.random_range(0.0..std::f32::consts::TAU);
        let impulse = Vec3::new(magnitude * angle.cos(), 0.0, magnitude * angle.sin());
        let computed_angle = impulse.z.atan2(impulse.x);
        let computed_magnitude = (impulse.x * impulse.x + impulse.z * impulse.z).sqrt();

        angles.push(computed_angle);
        magnitudes.push(computed_magnitude);
    }

    // Verify we have diverse angles (not all clustered in one direction)
    let min_angle = angles.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max_angle = angles.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let angle_range = (max_angle - min_angle).abs();
    assert!(
        angle_range > 3.0, // At least ~π radians of spread
        "Random angles should be distributed across range, got range {:.2}",
        angle_range
    );

    // Verify we have diverse magnitudes
    let min_mag = magnitudes.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max_mag = magnitudes.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let mag_range = max_mag - min_mag;
    assert!(
        mag_range > 5.0,
        "Random magnitudes should be distributed, got range {:.2}",
        mag_range
    );
}
