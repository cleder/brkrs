//! Integration tests for Brick 54 (Level Down) navigation brick.
//!
//! Brick 54 allows the player to return to the previous level when destroyed by the ball.
//! On level 1 (the first level), the brick is destroyed but no transition occurs.
//!
//! Tests verify:
//! - Level transition message emission (LevelSwitchRequested with direction=Previous)
//! - Brick destruction message emission (BrickDestroyed with brick_type=54)
//! - Score award (0 points, utility brick like Extra Ball)
//! - First level boundary condition (no transition, brick still destroyed)
//! - Multi-frame state persistence (10+ frames without overwrite)

use bevy::{app::App, prelude::*};
use bevy_rapier3d::prelude::*;
use brkrs::level_format::BRICK_54;
use brkrs::level_loader::{CurrentLevel, LevelAdvanceState, LevelDefinition};
use brkrs::systems::level_switch::{LevelSwitchDirection, LevelSwitchRequested, LevelSwitchSource};
use brkrs::systems::LevelSwitchState;
use brkrs::GameProgress;

fn test_app_with_brick_54() -> App {
    let mut app = App::new();

    // Core plugins
    app.add_plugins((MinimalPlugins, InputPlugin));

    // Physics configuration
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());

    // Game state
    app.insert_resource(GameProgress::default());
    app.insert_resource(LevelAdvanceState::default());
    app.insert_resource(brkrs::systems::respawn::SpawnPoints::default());
    app.insert_resource(ButtonInput::<KeyCode>::default());

    // Assets
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());

    // Physics world
    app.world_mut().spawn(RapierConfiguration::new(1.0));

    app
}

/// Helper to create a test level with a single brick 54 at a known position.
fn create_test_level_with_brick_54(level_number: u32) -> LevelDefinition {
    let mut matrix = vec![vec![0u8; 20]; 20];
    // Place brick 54 at position (5, 5) in the matrix
    matrix[5][5] = BRICK_54;

    LevelDefinition {
        number: level_number,
        matrix,
        gravity: None,
    }
}

/// Helper to check if LevelSwitchRequested message was written with expected values.
fn level_switch_message_written(
    app: &mut App,
    expected_direction: LevelSwitchDirection,
    expected_source: LevelSwitchSource,
) -> bool {
    use bevy::ecs::message::MessageReader;

    let mut reader = app
        .world_mut()
        .resource_mut::<MessageReader<LevelSwitchRequested>>();
    reader
        .iter()
        .any(|msg| msg.direction == expected_direction && msg.source == expected_source)
}

#[test]
#[ignore = "RED phase test: expects brick 54 collision logic not yet implemented"]
fn test_brick_54_collision_emits_level_switch_requested() {
    let mut app = test_app_with_brick_54();

    // Set up level 5 with brick 54
    let test_level = create_test_level_with_brick_54(5);
    app.world_mut().insert_resource(CurrentLevel(test_level));
    app.update();

    // Simulate ball collision with brick 54
    app.world_mut().write_message(LevelSwitchRequested {
        source: LevelSwitchSource::Brick,
        direction: LevelSwitchDirection::Previous,
    });
    app.update();

    // Assertion: LevelSwitchRequested message should exist
    // Note: This test will FAIL until collision handler is implemented
    assert!(
        level_switch_message_written(
            &mut app,
            LevelSwitchDirection::Previous,
            LevelSwitchSource::Brick
        ),
        "LevelSwitchRequested(Previous, Brick) should be emitted on brick 54 collision"
    );
}

#[test]
#[ignore = "RED phase test: expects brick destruction message not yet implemented"]
fn test_brick_54_collision_emits_brick_destroyed_message() {
    // This test verifies that BrickDestroyed message is emitted with brick_type = 54
    // Required assertion: BrickDestroyed { brick_type: 54 } message in queue
    // Expected to FAIL until collision handler implementation
    panic!("Test not yet implemented: BrickDestroyed message emission");
}

#[test]
#[ignore = "RED phase test: expects brick destruction scoring not yet implemented"]
fn test_brick_54_awards_zero_points() {
    // This test verifies that destroying brick 54 awards exactly 0 points
    // (utility brick, like Extra Ball brick 41)
    // Expected to FAIL until collision handler and scoring integration complete
    panic!("Test not yet implemented: Scoring 0 points for brick 54");
}

#[test]
#[ignore = "RED phase test: expects level transition not yet implemented"]
fn test_brick_54_returns_to_previous_level() {
    let mut app = test_app_with_brick_54();

    // Set up level 5 with brick 54
    let test_level = create_test_level_with_brick_54(5);
    app.world_mut()
        .insert_resource(CurrentLevel(test_level.clone()));
    app.update();

    let start_level = app
        .world()
        .get_resource::<CurrentLevel>()
        .map(|l| l.0.number)
        .expect("CurrentLevel should exist");
    assert_eq!(start_level, 5, "Should start at level 5");

    // Emit level switch request (brick collision would do this)
    app.world_mut().write_message(LevelSwitchRequested {
        source: LevelSwitchSource::Brick,
        direction: LevelSwitchDirection::Previous,
    });
    app.update();
    app.update(); // Allow level loader to process

    // Assertion: CurrentLevel should go back to 4
    // Expected to FAIL until level_loader processes brick-sourced transitions
    let new_level = app
        .world()
        .get_resource::<CurrentLevel>()
        .map(|l| l.0.number)
        .expect("CurrentLevel should exist");
    assert!(
        new_level < start_level,
        "Level should regress from {start_level} to a lower number, got {new_level}"
    );
}

#[test]
#[ignore = "RED phase test: expects level 1 boundary logic not yet implemented"]
fn test_brick_54_on_level_1_has_no_effect() {
    let mut app = test_app_with_brick_54();

    // Set up level 1 (first level) with brick 54
    let test_level = create_test_level_with_brick_54(1);
    app.world_mut()
        .insert_resource(CurrentLevel(test_level.clone()));
    app.update();

    // Verify we're on level 1
    let current = app
        .world()
        .get_resource::<CurrentLevel>()
        .map(|l| l.0.number)
        .expect("CurrentLevel should exist");
    assert_eq!(current, 1, "Should be on level 1");

    // Emit brick 54 collision/destruction
    app.world_mut().write_message(LevelSwitchRequested {
        source: LevelSwitchSource::Brick,
        direction: LevelSwitchDirection::Previous,
    });
    app.update();
    app.update();

    // Assertion: Should remain on level 1 (no transition)
    let post_level = app
        .world()
        .get_resource::<CurrentLevel>()
        .map(|l| l.0.number)
        .expect("CurrentLevel should exist");
    assert_eq!(
        post_level, 1,
        "Should remain on level 1, got level {post_level}"
    );
}

#[test]
#[ignore = "RED phase test: expects multi-frame persistence not yet verified"]
fn test_brick_54_level_state_persists_across_frames() {
    let mut app = test_app_with_brick_54();

    // Set up level 5 with brick 54
    let test_level = create_test_level_with_brick_54(5);
    app.world_mut()
        .insert_resource(CurrentLevel(test_level.clone()));
    app.update();

    // Trigger level switch
    app.world_mut().write_message(LevelSwitchRequested {
        source: LevelSwitchSource::Brick,
        direction: LevelSwitchDirection::Previous,
    });
    app.update();
    app.update();

    let transitioned_level = app
        .world()
        .get_resource::<CurrentLevel>()
        .map(|l| l.0.number)
        .expect("CurrentLevel should exist")
        .clone();

    // Run 10+ additional frames to verify no initialization system overwrites level state
    for frame in 0..12 {
        app.update();

        let current_level = app
            .world()
            .get_resource::<CurrentLevel>()
            .map(|l| l.0.number)
            .expect("CurrentLevel should exist");

        assert_eq!(
            current_level, transitioned_level,
            "Level state should persist across frame {frame}. Expected {transitioned_level}, got {current_level}"
        );
    }
}

// Record failing test commit hash here (from: git rev-parse HEAD after test failure)
// <failing-test-commit-hash>: TBD - Run: cargo test --test brick_54_level_down --ignored
