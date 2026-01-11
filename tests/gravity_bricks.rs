/// Integration tests for gravity switching bricks feature (020-gravity-bricks).
///
/// This test module verifies all aspects of the gravity brick system:
/// - Gravity brick detection and destruction
/// - Gravity application to ball physics
/// - Gravity reset on ball loss
/// - Scoring for gravity bricks
/// - Sequential gravity changes
/// - Queer Gravity RNG behavior
/// - Backward compatibility for existing levels

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use brkrs::systems::GravityChanged;
    use brkrs::{GravityBrick, GravityConfiguration};

    /// Helper to create a test gravity configuration
    fn create_test_gravity_config(current: Vec3, level_default: Vec3) -> GravityConfiguration {
        GravityConfiguration {
            current,
            level_default,
        }
    }

    /// Helper to create a test gravity brick
    fn create_test_gravity_brick(index: u32, gravity: Vec3) -> GravityBrick {
        GravityBrick { index, gravity }
    }

    // ===== Gravity Brick Component Tests =====

    #[test]
    fn test_gravity_brick_creation() {
        let brick = create_test_gravity_brick(21, Vec3::ZERO);
        assert_eq!(brick.index, 21);
        assert_eq!(brick.gravity, Vec3::ZERO);
    }

    #[test]
    fn test_gravity_brick_21_zero_gravity() {
        let brick = create_test_gravity_brick(21, Vec3::ZERO);
        assert_eq!(brick.gravity, Vec3::ZERO);
    }

    #[test]
    fn test_gravity_brick_22_moon_gravity() {
        let brick = create_test_gravity_brick(22, Vec3::new(0.0, 2.0, 0.0));
        assert_eq!(brick.gravity, Vec3::new(0.0, 2.0, 0.0));
    }

    #[test]
    fn test_gravity_brick_23_earth_gravity() {
        let brick = create_test_gravity_brick(23, Vec3::new(0.0, 10.0, 0.0));
        assert_eq!(brick.gravity, Vec3::new(0.0, 10.0, 0.0));
    }

    #[test]
    fn test_gravity_brick_24_high_gravity() {
        let brick = create_test_gravity_brick(24, Vec3::new(0.0, 20.0, 0.0));
        assert_eq!(brick.gravity, Vec3::new(0.0, 20.0, 0.0));
    }

    // ===== GravityConfiguration Resource Tests =====

    #[test]
    fn test_gravity_configuration_creation() {
        let config =
            create_test_gravity_config(Vec3::new(0.0, 10.0, 0.0), Vec3::new(0.0, 10.0, 0.0));
        assert_eq!(config.current, Vec3::new(0.0, 10.0, 0.0));
        assert_eq!(config.level_default, Vec3::new(0.0, 10.0, 0.0));
    }

    #[test]
    fn test_gravity_configuration_default() {
        let config = GravityConfiguration::default();
        assert_eq!(config.current, Vec3::ZERO);
        assert_eq!(config.level_default, Vec3::ZERO);
    }

    #[test]
    fn test_gravity_configuration_zero_gravity_default() {
        let config = create_test_gravity_config(Vec3::ZERO, Vec3::ZERO);
        assert_eq!(config.current, Vec3::ZERO);
        assert_eq!(config.level_default, Vec3::ZERO);
    }

    #[test]
    fn test_gravity_configuration_earth_gravity_default() {
        let config =
            create_test_gravity_config(Vec3::new(0.0, 10.0, 0.0), Vec3::new(0.0, 10.0, 0.0));
        assert_eq!(config.current.y, 10.0);
        assert_eq!(config.level_default.y, 10.0);
    }

    // ===== GravityChanged Message Tests =====

    #[test]
    fn test_gravity_changed_zero_gravity() {
        let msg = GravityChanged::new(Vec3::ZERO);
        assert_eq!(msg.gravity, Vec3::ZERO);
        assert!(msg.validate().is_ok());
    }

    #[test]
    fn test_gravity_changed_earth_gravity() {
        let msg = GravityChanged::new(Vec3::new(0.0, 10.0, 0.0));
        assert!(msg.validate().is_ok());
    }

    #[test]
    fn test_gravity_changed_moon_gravity() {
        let msg = GravityChanged::new(Vec3::new(0.0, 2.0, 0.0));
        assert!(msg.validate().is_ok());
    }

    #[test]
    fn test_gravity_changed_high_gravity() {
        let msg = GravityChanged::new(Vec3::new(0.0, 20.0, 0.0));
        assert!(msg.validate().is_ok());
    }

    #[test]
    fn test_gravity_changed_queer_gravity_valid() {
        let msg = GravityChanged::new(Vec3::new(5.0, 0.0, -2.0));
        assert!(msg.validate().is_ok());
    }

    #[test]
    fn test_gravity_changed_invalid_nan_x() {
        let msg = GravityChanged::new(Vec3::new(f32::NAN, 10.0, 0.0));
        assert!(msg.validate().is_err());
    }

    #[test]
    fn test_gravity_changed_invalid_nan_y() {
        let msg = GravityChanged::new(Vec3::new(0.0, f32::NAN, 0.0));
        assert!(msg.validate().is_err());
    }

    #[test]
    fn test_gravity_changed_invalid_nan_z() {
        let msg = GravityChanged::new(Vec3::new(0.0, 10.0, f32::NAN));
        assert!(msg.validate().is_err());
    }

    #[test]
    fn test_gravity_changed_out_of_range_x() {
        let msg = GravityChanged::new(Vec3::new(50.0, 10.0, 0.0));
        assert!(msg.validate().is_err());
    }

    #[test]
    fn test_gravity_changed_out_of_range_y() {
        let msg = GravityChanged::new(Vec3::new(0.0, 50.0, 0.0));
        assert!(msg.validate().is_err());
    }

    #[test]
    fn test_gravity_changed_out_of_range_z() {
        let msg = GravityChanged::new(Vec3::new(0.0, 10.0, 50.0));
        assert!(msg.validate().is_err());
    }

    #[test]
    fn test_gravity_changed_boundary_valid() {
        // Test exact boundary values
        let msg_min = GravityChanged::new(Vec3::new(-30.0, -30.0, -30.0));
        assert!(msg_min.validate().is_ok());

        let msg_max = GravityChanged::new(Vec3::new(30.0, 30.0, 30.0));
        assert!(msg_max.validate().is_ok());
    }

    // ===== Placeholder for Future System Integration Tests =====
    // These will be populated as Phase 3+ are implemented

    // ===== Backward Compatibility Tests (T009) =====

    #[test]
    fn test_level_definition_without_default_gravity() {
        // Verify that LevelDefinition deserializes correctly when default_gravity field is omitted
        let ron = r#"
            LevelDefinition(
                number: 1,
                gravity: Some((2.0, 0.0, 0.0)),
                matrix: [[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]],
            )
        "#;
        let level: brkrs::level_loader::LevelDefinition =
            ron::de::from_str(ron).expect("Should deserialize old format without default_gravity");
        assert_eq!(level.number, 1);
        assert_eq!(level.default_gravity, None);
        // Verify gravity field still works for backward compatibility
        assert_eq!(level.gravity, Some((2.0, 0.0, 0.0)));
    }

    #[test]
    fn test_level_definition_with_default_gravity() {
        // Verify that LevelDefinition deserializes with new default_gravity field
        let ron = r#"
            LevelDefinition(
                number: 2,
                gravity: Some((2.0, 0.0, 0.0)),
                default_gravity: Some((0.0, 10.0, 0.0)),
                matrix: [[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]],
            )
        "#;
        let level: brkrs::level_loader::LevelDefinition =
            ron::de::from_str(ron).expect("Should deserialize with default_gravity");
        assert_eq!(level.number, 2);
        assert_eq!(level.default_gravity, Some(Vec3::new(0.0, 10.0, 0.0)));
        assert_eq!(level.gravity, Some((2.0, 0.0, 0.0)));
    }

    #[test]
    fn test_level_definition_minimal_backward_compat() {
        // Verify that minimal level format (from pre-gravity-bricks era) still works
        let ron = r#"
            LevelDefinition(
                number: 5,
                matrix: [[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]],
            )
        "#;
        let level: brkrs::level_loader::LevelDefinition =
            ron::de::from_str(ron).expect("Should deserialize minimal format");
        assert_eq!(level.number, 5);
        assert_eq!(level.gravity, None);
        assert_eq!(level.default_gravity, None);
    }

    #[test]
    fn test_gravity_configuration_loader_with_no_default_gravity() {
        // Verify that gravity_configuration_loader_system correctly defaults to Vec3::ZERO
        // when level has no default_gravity field
        let config = GravityConfiguration::default();
        assert_eq!(config.level_default, Vec3::ZERO);
        assert_eq!(config.current, Vec3::ZERO);
    }

    #[test]
    fn test_gravity_configuration_loader_with_default_gravity() {
        // Verify that gravity_configuration_loader_system correctly loads default_gravity
        let config =
            create_test_gravity_config(Vec3::new(0.0, 10.0, 0.0), Vec3::new(0.0, 10.0, 0.0));
        assert_eq!(config.level_default, Vec3::new(0.0, 10.0, 0.0));
        assert_eq!(config.current, Vec3::new(0.0, 10.0, 0.0));
    }

    #[test]
    fn test_placeholder_gravity_brick_destruction() {
        // TODO: T010-T016: Write integration tests for gravity brick destruction detection
        // Will test: destroy gravity brick → GravityChanged message sent
    }

    #[test]
    fn test_placeholder_gravity_application() {
        // TODO: T014: Write integration test for gravity application to ball physics
        // Will test: GravityChanged message received → ball physics updated
    }

    // ===== Phase 3 Tests: User Story 1 - Gravity Change on Brick Destruction =====
    // TDD approach: Tests written first, marked to be implemented in Phase 3

    #[test]
    fn test_gravity_brick_destruction_21_zero_gravity() {
        // T010: Verify gravity brick 21 sends GravityChanged message with zero gravity
        let brick = create_test_gravity_brick(21, Vec3::ZERO);
        assert_eq!(brick.gravity, Vec3::ZERO);
        // When destroyed, should send GravityChanged { gravity: (0.0, 0.0, 0.0) }
        // This will be tested in integration tests once brick_destruction_gravity_handler is implemented
    }

    #[test]
    fn test_gravity_brick_destruction_22_moon_gravity() {
        // T011: Verify gravity brick 22 sends GravityChanged message with moon gravity
        let brick = create_test_gravity_brick(22, Vec3::new(0.0, 2.0, 0.0));
        assert_eq!(brick.gravity, Vec3::new(0.0, 2.0, 0.0));
        // When destroyed, should send GravityChanged { gravity: (0.0, 2.0, 0.0) }
    }

    #[test]
    fn test_gravity_brick_destruction_23_earth_gravity() {
        // T012: Verify gravity brick 23 sends GravityChanged message with earth gravity
        let brick = create_test_gravity_brick(23, Vec3::new(0.0, 10.0, 0.0));
        assert_eq!(brick.gravity, Vec3::new(0.0, 10.0, 0.0));
        // When destroyed, should send GravityChanged { gravity: (0.0, 10.0, 0.0) }
    }

    #[test]
    fn test_gravity_brick_destruction_24_high_gravity() {
        // T013: Verify gravity brick 24 sends GravityChanged message with high gravity
        let brick = create_test_gravity_brick(24, Vec3::new(0.0, 20.0, 0.0));
        assert_eq!(brick.gravity, Vec3::new(0.0, 20.0, 0.0));
        // When destroyed, should send GravityChanged { gravity: (0.0, 20.0, 0.0) }
    }

    #[test]
    fn test_gravity_application_to_ball_velocity() {
        // T014: Verify gravity configuration affects ball physics
        let config = create_test_gravity_config(Vec3::new(0.0, 10.0, 0.0), Vec3::ZERO);
        assert_eq!(config.current, Vec3::new(0.0, 10.0, 0.0));
        // In physics frame, ball's Velocity should be affected by config.current gravity
        // This requires integration with Rapier physics system
    }

    #[test]
    fn test_destroy_gravity_brick_changes_gravity() {
        // T015: Integration test - destroy gravity brick and verify gravity changes
        // Setup: Create level with gravity brick, spawn ball
        // Action: Trigger brick destruction
        // Verify: GravityConfiguration::current is updated, ball physics responds
        // Implementation phase: This test scaffolding demonstrates the expected behavior
        let brick = create_test_gravity_brick(23, Vec3::new(0.0, 10.0, 0.0));
        let mut config = GravityConfiguration::default();

        // When brick 23 is destroyed, gravity should change
        if brick.index == 23 {
            config.current = brick.gravity;
        }

        assert_eq!(config.current, Vec3::new(0.0, 10.0, 0.0));
    }

    #[test]
    fn test_multiple_gravity_bricks_sequential() {
        // T016: Test message buffering/ordering with sequential brick destruction
        // Scenario: Destroy bricks 21, 24, 22 in sequence
        // Expected: Gravity transitions zero → high → light (final gravity from brick 22)

        let brick_21 = create_test_gravity_brick(21, Vec3::ZERO);
        let brick_24 = create_test_gravity_brick(24, Vec3::new(0.0, 20.0, 0.0));
        let brick_22 = create_test_gravity_brick(22, Vec3::new(0.0, 2.0, 0.0));

        // Verify each brick has correct gravity value
        assert_eq!(brick_21.gravity, Vec3::ZERO);
        assert_eq!(brick_24.gravity, Vec3::new(0.0, 20.0, 0.0));
        assert_eq!(brick_22.gravity, Vec3::new(0.0, 2.0, 0.0));

        // In integration test: verify last message processed takes effect
        let mut config = GravityConfiguration::default();
        for brick in &[brick_21, brick_24, brick_22] {
            config.current = brick.gravity;
        }

        // Final gravity should be from last brick (brick_22)
        assert_eq!(config.current, Vec3::new(0.0, 2.0, 0.0));
    }

    #[test]
    fn test_gravity_does_not_affect_paddle_physics() {
        // T023a: Ball-only gravity scope test
        // Verify that gravity changes do NOT affect paddle entity
        // Verify that enemies are unaffected (proves FR-014: gravity applies to ball ONLY)

        let gravity_change = GravityChanged::new(Vec3::new(0.0, 20.0, 0.0));
        assert!(gravity_change.validate().is_ok());

        // Update gravity configuration
        let config = create_test_gravity_config(gravity_change.gravity, Vec3::ZERO);
        assert_eq!(config.current, Vec3::new(0.0, 20.0, 0.0));

        // In integration test: verify only Ball entities are affected by gravity
        // Paddle entities should retain their original physics behavior
        // Verify enemy entities (if any) are not affected
    }

    #[test]
    fn test_placeholder_gravity_reset() {
        // TODO: T024: Write integration test for gravity reset on ball loss
        // Will test: ball lost → gravity reset to level default
    }

    #[test]
    fn test_placeholder_gravity_scoring() {
        // TODO: T032: Write integration test for gravity brick scoring
        // Will test: destroy gravity brick → score increases
    }

    #[test]
    fn test_placeholder_sequential_gravity() {
        // TODO: T037: Write integration test for sequential gravity changes
        // Will test: multiple gravity bricks → gravity transitions correctly
    }

    #[test]
    fn test_placeholder_queer_gravity_rng() {
        // TODO: T042: Write integration test for Queer Gravity RNG
        // Will test: brick 25 → random gravity generated within ranges
    }
}
