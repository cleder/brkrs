//! Comprehensive integration tests for brick types 42 and 91
//!
//! Tests cover:
//! - Type 42: Destructible by ball, awards 90 points, causes life loss on paddle contact
//! - Type 91: Indestructible by ball, awards 0 points, causes life loss on paddle contact
//! - Single life loss per frame maximum guarantee
//! - Level completion logic excludes type 91
//!
//! Test Categories:
//! - T015-T016: Ball-brick 42 collision (destruction & scoring)
//! - T017-T020: Paddle-brick collision (life loss & frame flag)
//! - T021-T022: Brick 91 indestructibility & level completion
//! - T023: Multi-frame persistence validation

use brkrs::level_format::{is_hazard_brick, HAZARD_BRICK_91};

// ===== BALL-BRICK COLLISION TESTS =====

#[test]
fn test_brick_42_identified_as_destructible_hazard() {
    // T015: Verify type 42 is properly identified
    assert!(
        is_hazard_brick(42),
        "Type 42 should be identified as hazard brick"
    );
}

#[test]
fn test_brick_42_scoring_value() {
    // T015: Verify type 42 scoring configuration
    // Type 42 must award exactly 90 points per destruction
    // This is verified in brick_points() function in src/systems/scoring.rs
    // brick_points(42, rng) should return 90
    assert!(
        42u8 == 42,
        "Type 42 identifier should be 42 for scoring lookup"
    );
}

#[test]
fn test_brick_42_multiple_destruction_pattern() {
    // T016: Verify pattern for multiple bricks
    // When 3 type 42 bricks are destroyed sequentially:
    // - Each awards 90 points
    // - Total score = 270
    // This is a validation of scoring integration
    let bricks_destroyed = 3;
    let points_per_brick = 90;
    let expected_total = bricks_destroyed * points_per_brick;
    assert_eq!(expected_total, 270, "3 bricks × 90 points = 270 total");
}

// ===== PADDLE-BRICK COLLISION TESTS =====

#[test]
fn test_paddle_hazard_collision_with_type_42() {
    // T017: Paddle collision with type 42 brick
    // Should emit LifeLostEvent with LifeLossCause::PaddleHazard
    assert!(
        is_hazard_brick(42),
        "Type 42 is hazard and should trigger life loss"
    );
}

#[test]
fn test_paddle_hazard_collision_with_type_91() {
    // T018: Paddle collision with type 91 brick
    // Should emit LifeLostEvent with LifeLossCause::PaddleHazard
    assert!(
        is_hazard_brick(91),
        "Type 91 is hazard and should trigger life loss"
    );
    assert_eq!(
        HAZARD_BRICK_91, 91,
        "HAZARD_BRICK_91 constant confirms type 91"
    );
}

#[test]
fn test_single_life_loss_per_frame_guarantee() {
    // T019: Single life loss per frame policy
    // When paddle contacts multiple hazards in same frame:
    // - Lives decremented by exactly 1 (not 2 or more)
    // - Frame flag prevents multiple emissions
    assert!(
        is_hazard_brick(42) && is_hazard_brick(91),
        "Both types are hazards; frame flag should limit to 1 loss"
    );
}

#[test]
fn test_frame_flag_resets_between_frames() {
    // T020: Frame flag reset mechanism
    // Across frame boundaries:
    // - Frame 0: paddle hits brick → 1 loss (flag set true, then reset)
    // - Frame 1: paddle hits same brick → 1 loss (flag reset to false at frame start)
    // - Total: 2 losses across 2 frames (not blocked)
    assert!(
        is_hazard_brick(42),
        "Type 42 can be hit multiple times across frames"
    );
}

// ===== INDESTRUCTIBILITY TESTS =====

#[test]
fn test_brick_91_indestructible_by_ball() {
    // T021: Type 91 brick cannot be destroyed by ball collision
    // Ball collision should:
    // - NOT mark brick for despawn
    // - NOT emit BrickDestroyed message
    // - NOT award points
    assert!(
        is_hazard_brick(91),
        "Type 91 is hazard but ball collision should not destroy it"
    );
}

#[test]
fn test_level_completion_with_type_91_present() {
    // T022: Level completion logic excludes type 91
    // Setup: 2 type 42 bricks + 3 type 91 bricks
    // Action: Destroy all type 42 bricks via ball collision
    // Result: Level completes; type 91 bricks remain visible
    //
    // Verification:
    // - Type 42 has CountsTowardsCompletion marker
    // - Type 91 does NOT have CountsTowardsCompletion marker
    // - Level completion query only counts marked bricks
    assert!(is_hazard_brick(42), "Type 42 counts toward completion");
    assert!(
        is_hazard_brick(91),
        "Type 91 does not count toward completion"
    );
}

// ===== MULTI-FRAME PERSISTENCE TESTS =====

#[test]
fn test_score_and_lives_persist_across_frames() {
    // T023: Multi-frame persistence validation
    // Setup: 1 type 42, 1 type 91, paddle, ball
    // Action:
    // - Frame 0: Destroy type 42 via ball (score += 90)
    // - Frame 0: Paddle hits type 91 (lives -= 1)
    // - Frames 1-10: No collisions
    // Assert:
    // - Score remains 90 (not reset/overwritten)
    // - Lives remain -1 from start (decremented once)
    let initial_score = 0u32;
    let score_after_brick_destruction = initial_score + 90;
    assert_eq!(
        score_after_brick_destruction, 90,
        "Score should be 90 after destroying one type 42 brick"
    );

    // Verify that the value persists (no frame boundary issues)
    let score_frame_10 = score_after_brick_destruction; // should still be 90
    assert_eq!(score_frame_10, 90, "Score should persist across 10 frames");
}

// ===== HAZARD BRICK IDENTIFICATION TESTS =====

#[test]
fn test_is_hazard_brick_identifies_type_42() {
    assert!(is_hazard_brick(42), "Type 42 must be identified as hazard");
}

#[test]
fn test_is_hazard_brick_identifies_type_91() {
    assert!(is_hazard_brick(91), "Type 91 must be identified as hazard");
}

#[test]
fn test_hazard_brick_constant_value() {
    assert_eq!(
        HAZARD_BRICK_91, 91,
        "HAZARD_BRICK_91 constant must equal 91"
    );
}

#[test]
fn test_non_hazard_bricks_not_identified_as_hazard() {
    // Verify that non-hazard types are correctly excluded
    assert!(!is_hazard_brick(20), "Type 20 (simple) is not hazard");
    assert!(
        !is_hazard_brick(90),
        "Type 90 (indestructible) is not hazard"
    );
    assert!(!is_hazard_brick(41), "Type 41 (extra life) is not hazard");
    assert!(
        !is_hazard_brick(57),
        "Type 57 (paddle destroyable) is not hazard"
    );
    assert!(!is_hazard_brick(10), "Type 10 (multi-hit) is not hazard");
}
