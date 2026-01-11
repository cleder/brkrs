//! Unit tests for gravity indicator UI (021-gravity-bricks).
//!
//! Tests cover:
//! - Gravity mapping logic and tolerance
//! - Gravity level to asset name mapping
//! - Edge cases and tolerance boundaries

use bevy::prelude::*;
use brkrs::ui::gravity_indicator::{map_gravity_to_level, GravityLevel};

// ============================================================================
// Unit Tests: map_gravity_to_level
// ============================================================================

#[test]
fn test_map_gravity_exact_values() {
    assert_eq!(
        map_gravity_to_level(Vec3::new(0.0, 0.0, 0.0)),
        GravityLevel::L0
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(2.0, 5.0, 0.0)),
        GravityLevel::L2
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(10.0, 0.0, 0.0)),
        GravityLevel::L10
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(20.0, 0.0, 0.0)),
        GravityLevel::L20
    );
}

#[test]
fn test_map_gravity_tolerance_within() {
    // Within ±0.5 tolerance
    assert_eq!(
        map_gravity_to_level(Vec3::new(2.3, 0.0, 0.0)),
        GravityLevel::L2
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(1.5, 0.0, 0.0)),
        GravityLevel::L2
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(2.49, 0.0, 0.0)),
        GravityLevel::L2
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(10.4, 0.0, 0.0)),
        GravityLevel::L10
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(9.6, 0.0, 0.0)),
        GravityLevel::L10
    );
}

#[test]
fn test_map_gravity_tolerance_outside() {
    // Outside ±0.5 tolerance (0.6 away)
    assert_eq!(
        map_gravity_to_level(Vec3::new(2.6, 0.0, 0.0)),
        GravityLevel::Unknown
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(1.4, 0.0, 0.0)),
        GravityLevel::Unknown
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(9.4, 0.0, 0.0)),
        GravityLevel::Unknown
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(10.51, 0.0, 0.0)),
        GravityLevel::Unknown
    );
}

#[test]
fn test_map_gravity_mixed_axes_highest_wins() {
    // X=2, Z=10 → highest is 10
    assert_eq!(
        map_gravity_to_level(Vec3::new(2.0, 0.0, 10.0)),
        GravityLevel::L10
    );

    // X=10, Z=2 → highest is 10
    assert_eq!(
        map_gravity_to_level(Vec3::new(10.0, 0.0, 2.0)),
        GravityLevel::L10
    );

    // X=20, Z=10 → highest is 20
    assert_eq!(
        map_gravity_to_level(Vec3::new(20.0, 0.0, 10.0)),
        GravityLevel::L20
    );

    // X=0, Z=20 → highest is 20
    assert_eq!(
        map_gravity_to_level(Vec3::new(0.0, 0.0, 20.0)),
        GravityLevel::L20
    );
}

#[test]
fn test_map_gravity_y_axis_ignored() {
    // Y should be completely ignored (always Y-locked in game)
    assert_eq!(
        map_gravity_to_level(Vec3::new(10.0, 100.0, 0.0)),
        GravityLevel::L10
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(10.0, -50.0, 0.0)),
        GravityLevel::L10
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(10.0, 0.0, 0.0)),
        GravityLevel::L10
    );
}

#[test]
fn test_map_gravity_unknown() {
    assert_eq!(
        map_gravity_to_level(Vec3::new(5.0, 0.0, 0.0)),
        GravityLevel::Unknown
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(15.0, 0.0, 0.0)),
        GravityLevel::Unknown
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(25.0, 0.0, 0.0)),
        GravityLevel::Unknown
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(1.0, 0.0, 1.0)),
        GravityLevel::Unknown
    );
}

#[test]
fn test_map_gravity_negative_values() {
    // Absolute value used for comparison
    assert_eq!(
        map_gravity_to_level(Vec3::new(-10.0, 0.0, 0.0)),
        GravityLevel::L10
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(0.0, 0.0, -20.0)),
        GravityLevel::L20
    );
    assert_eq!(
        map_gravity_to_level(Vec3::new(-2.0, 0.0, -10.0)),
        GravityLevel::L10
    );
}

#[test]
fn test_gravity_level_asset_names() {
    assert_eq!(GravityLevel::L0.asset_name(), "weight-0");
    assert_eq!(GravityLevel::L2.asset_name(), "weight-2");
    assert_eq!(GravityLevel::L10.asset_name(), "weight-10");
    assert_eq!(GravityLevel::L20.asset_name(), "weight-20");
    assert_eq!(GravityLevel::Unknown.asset_name(), "weight-question");
}

#[test]
fn test_gravity_level_equality() {
    assert_eq!(GravityLevel::L0, GravityLevel::L0);
    assert_ne!(GravityLevel::L0, GravityLevel::L2);
    assert_ne!(GravityLevel::Unknown, GravityLevel::L10);
}
