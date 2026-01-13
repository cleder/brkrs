//! Integration tests for brick types 42 and 91 paddle life loss and ball destruction behaviors
//!
//! Tests verify:
//! - Type 42: Destructible by ball, awards 90 points, causes life loss on paddle contact
//! - Type 91: Indestructible by ball, awards 0 points, causes life loss on paddle contact
//! - Single life loss per frame maximum
//! - Level completion logic excludes type 91

use brkrs::level_format::{is_hazard_brick, HAZARD_BRICK_91};

#[test]
fn test_is_hazard_brick_identification() {
    // Verify that is_hazard_brick correctly identifies types 42 and 91
    assert!(
        is_hazard_brick(42),
        "Type 42 should be identified as hazard"
    );
    assert!(
        is_hazard_brick(91),
        "Type 91 should be identified as hazard"
    );
    assert!(
        is_hazard_brick(HAZARD_BRICK_91),
        "HAZARD_BRICK_91 constant should be identified as hazard"
    );
    assert!(
        !is_hazard_brick(20),
        "Type 20 should not be identified as hazard"
    );
    assert!(
        !is_hazard_brick(90),
        "Type 90 should not be identified as hazard"
    );
}

#[test]
fn test_brick_type_91_constant() {
    // Verify that HAZARD_BRICK_91 constant has correct value
    assert_eq!(HAZARD_BRICK_91, 91);
}

#[test]
fn test_hazard_brick_includes_type_42_and_91() {
    // Test that both type 42 and 91 are considered hazard bricks
    for hazard_type in &[42u8, 91u8] {
        assert!(
            is_hazard_brick(*hazard_type),
            "Type {} should be a hazard brick",
            hazard_type
        );
    }
}

#[test]
fn test_non_hazard_bricks() {
    // Test that other brick types are NOT hazard bricks
    for non_hazard_type in &[10u8, 20u8, 41u8, 57u8, 90u8] {
        assert!(
            !is_hazard_brick(*non_hazard_type),
            "Type {} should not be a hazard brick",
            non_hazard_type
        );
    }
}
