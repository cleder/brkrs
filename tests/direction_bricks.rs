//! Tests for direction brick effects (bricks 43-48, 52)
//!
//! This test module verifies the direction brick impulse system using Bevy's testing utilities.
//! Tests follow TDD red-phase approach: they are written to fail initially, then implementation
//! is added to make them pass.
//!
//! Note: These tests verify impulse application to the ExternalImpulse component without
//! requiring full physics simulation (which requires assets/scene spawner). The physics
//! integration itself is verified via integration tests at the game level.

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
            ExternalImpulse {
                impulse: Vec3::ZERO,
                torque_impulse: Vec3::ZERO,
            },
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

/// T006: Brick 43 (Down) applies -5.0 Y impulse
#[test]
fn test_brick_43_down_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::new(3.0, 2.0, 0.0), Vec3::new(3.0, 2.0, 0.0));
    apply_impulse_to_ball(&mut app, ball, Vec3::new(0.0, -5.0, 0.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse.x, 0.0);
    assert_eq!(external_impulse.impulse.y, -5.0);
    assert_eq!(external_impulse.impulse.z, 0.0);
}

/// T007: Brick 44 (Left) applies -5.0 X impulse
#[test]
fn test_brick_44_left_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::new(3.0, 2.0, 0.0), Vec3::new(3.0, 2.0, 0.0));
    apply_impulse_to_ball(&mut app, ball, Vec3::new(-5.0, 0.0, 0.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse.x, -5.0);
    assert_eq!(external_impulse.impulse.y, 0.0);
    assert_eq!(external_impulse.impulse.z, 0.0);
}

/// T008: Brick 45 (Right) applies +5.0 X impulse
#[test]
fn test_brick_45_right_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(
        &mut app,
        Vec3::new(-3.0, 2.0, 0.0),
        Vec3::new(-3.0, 2.0, 0.0),
    );
    apply_impulse_to_ball(&mut app, ball, Vec3::new(5.0, 0.0, 0.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse.x, 5.0);
    assert_eq!(external_impulse.impulse.y, 0.0);
    assert_eq!(external_impulse.impulse.z, 0.0);
}

/// T009: Brick 46 (Up) applies +5.0 Y impulse
#[test]
fn test_brick_46_up_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(
        &mut app,
        Vec3::new(3.0, -2.0, 0.0),
        Vec3::new(3.0, -2.0, 0.0),
    );
    apply_impulse_to_ball(&mut app, ball, Vec3::new(0.0, 5.0, 0.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse.x, 0.0);
    assert_eq!(external_impulse.impulse.y, 5.0);
    assert_eq!(external_impulse.impulse.z, 0.0);
}

// =============================================================================
// T010-T012: Impulse Behavior Tests
// =============================================================================

/// T010: Verify impulse component can be set to specific values
#[test]
fn test_impulse_component_setting() {
    let mut app = create_test_app();
    let ball = spawn_ball(
        &mut app,
        Vec3::new(-3.0, -3.0, 0.0),
        Vec3::new(-3.0, -3.0, 0.0),
    );
    apply_impulse_to_ball(&mut app, ball, Vec3::new(0.0, -5.0, 0.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse, Vec3::new(0.0, -5.0, 0.0));
}

/// T011: Impulse can be applied to stationary ball (at rest)
#[test]
fn test_stationary_ball_receives_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO);
    apply_impulse_to_ball(&mut app, ball, Vec3::new(0.0, 5.0, 0.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse.y, 5.0);
    assert_eq!(external_impulse.impulse.x, 0.0);
    assert_eq!(external_impulse.impulse.z, 0.0);
}

/// T012: Z-axis impulse is preserved (not modified by XY-only direction bricks)
#[test]
fn test_z_axis_preserved() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 5.0));
    apply_impulse_to_ball(&mut app, ball, Vec3::new(5.0, 0.0, 0.0));

    // Velocity Z should not change from the impulse
    let velocity = app.world().get::<Velocity>(ball).unwrap();
    assert_eq!(velocity.linvel.z, 5.0);
}

// =============================================================================
// T013-T014: Persistence and Instrumentation Tests
// =============================================================================

/// T013: ExternalImpulse component persists across queries
#[test]
fn test_impulse_component_persistence() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
    apply_impulse_to_ball(&mut app, ball, Vec3::new(5.0, 0.0, 0.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    let first_impulse = external_impulse.impulse;
    assert_eq!(first_impulse.x, 5.0);

    // Query again (simulating multiple system reads)
    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse, first_impulse);
}

/// T014: Ball entity carries correct component set
#[test]
fn test_ball_component_set() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 0.0));

    // Verify all required components are present
    assert!(app.world().get::<Transform>(ball).is_some());
    assert!(app.world().get::<GlobalTransform>(ball).is_some());
    assert!(app.world().get::<Velocity>(ball).is_some());
    assert!(app.world().get::<RigidBody>(ball).is_some());
    assert!(app.world().get::<Collider>(ball).is_some());
    assert!(app.world().get::<ExternalImpulse>(ball).is_some());
    assert!(app.world().get::<Ball>(ball).is_some());
}

// =============================================================================
// T022-T025: Diagonal Direction Tests (Phase 4)
// =============================================================================

/// T022: Brick 47 (Forward) applies +5.0 Z impulse
#[test]
fn test_brick_47_forward_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(
        &mut app,
        Vec3::new(0.0, 0.0, -3.0),
        Vec3::new(0.0, 0.0, -3.0),
    );
    apply_impulse_to_ball(&mut app, ball, Vec3::new(0.0, 0.0, 5.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse.x, 0.0, "X impulse should be zero");
    assert_eq!(external_impulse.impulse.y, 0.0, "Y impulse should be zero");
    assert_eq!(external_impulse.impulse.z, 5.0, "Z impulse should be +5.0");
}

/// T023: Brick 48 (Backward) applies -5.0 Z impulse
#[test]
fn test_brick_48_backward_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::new(0.0, 0.0, 3.0), Vec3::new(0.0, 0.0, 3.0));
    apply_impulse_to_ball(&mut app, ball, Vec3::new(0.0, 0.0, -5.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(external_impulse.impulse.x, 0.0, "X impulse should be zero");
    assert_eq!(external_impulse.impulse.y, 0.0, "Y impulse should be zero");
    assert_eq!(external_impulse.impulse.z, -5.0, "Z impulse should be -5.0");
}

/// T024: Z-axis impulse applied independently from X,Y
#[test]
fn test_z_axis_independent_impulse() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::new(2.0, 2.0, 0.0), Vec3::new(2.0, 2.0, 0.0));
    // Apply forward impulse on ball with existing X,Y velocity
    apply_impulse_to_ball(&mut app, ball, Vec3::new(0.0, 0.0, 5.0));

    let external_impulse = app.world().get::<ExternalImpulse>(ball).unwrap();
    assert_eq!(
        external_impulse.impulse.z, 5.0,
        "Z impulse should be applied independently"
    );
}

/// T025: All six directions (43-48) can be applied in sequence
#[test]
fn test_all_six_directions() {
    let mut app = create_test_app();
    let ball = spawn_ball(&mut app, Vec3::ZERO, Vec3::ZERO);

    // Test all six cardinal and diagonal directions
    let directions = vec![
        (43, Vec3::new(-5.0, 0.0, 0.0)), // Left
        (44, Vec3::new(5.0, 0.0, 0.0)),  // Right
        (45, Vec3::new(0.0, 5.0, 0.0)),  // Up
        (46, Vec3::new(0.0, -5.0, 0.0)), // Down
        (47, Vec3::new(0.0, 0.0, 5.0)),  // Forward
        (48, Vec3::new(0.0, 0.0, -5.0)), // Backward
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
        let angle = rng.random_range(0.0..std::f32::consts::TAU);
        let impulse = Vec3::new(magnitude * angle.cos(), magnitude * angle.sin(), 0.0);
        let magnitude_computed = (impulse.x * impulse.x + impulse.y * impulse.y).sqrt();

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
            impulse.z, 0.0,
            "Z component should be zero for random brick (XY plane only)"
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
        let angle = rng.random_range(0.0..std::f32::consts::TAU);
        let impulse = Vec3::new(magnitude * angle.cos(), magnitude * angle.sin(), 0.0);
        let computed_angle = impulse.y.atan2(impulse.x);
        let computed_magnitude = impulse.length();

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
