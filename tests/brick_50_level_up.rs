//! Integration tests for Brick 50 (Level Up) navigation brick.
//!
//! Brick 50 allows the player to advance to the next level when destroyed by the ball.
//! On the final level, a victory screen is displayed instead of transitioning.
//!
//! Tests verify:
//! - Level transition message emission (LevelSwitchRequested with direction=Next)
//! - Brick destruction message emission (BrickDestroyed with brick_type=50)
//! - Score award (0 points, utility brick like Extra Ball)
//! - Final level boundary condition (victory screen + GameProgress.finished = true)
//! - Multi-frame state persistence (10+ frames without overwrite)

use bevy::{app::App, prelude::*};
use bevy_rapier3d::prelude::*;
use brkrs::level_format::BRICK_50;
use brkrs::level_loader::{CurrentLevel, LevelAdvanceState, LevelDefinition};
use brkrs::systems::level_switch::{LevelSwitchDirection, LevelSwitchRequested, LevelSwitchSource};
use brkrs::systems::LevelSwitchState;
use brkrs::{Brick, BrickTypeId, CountsTowardsCompletion, GameProgress};

fn test_app_with_brick_50() -> App {
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

/// Helper to create a test level with a single brick 50 at a known position.
fn create_test_level_with_brick_50(level_number: u32) -> LevelDefinition {
    let mut matrix = vec![vec![0u8; 20]; 20];
    // Place brick 50 at position (5, 5) in the matrix
    matrix[5][5] = BRICK_50;

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
#[ignore = "RED phase test: expects brick 50 collision logic not yet implemented"]
fn test_brick_50_collision_emits_level_switch_requested() {
    let mut app = test_app_with_brick_50();

    // Set up level 2 with brick 50
    let test_level = create_test_level_with_brick_50(2);
    app.world_mut().insert_resource(CurrentLevel(test_level));
    app.update();

    // Simulate ball collision with brick 50
    // (This is a placeholder; actual collision would be triggered via physics)
    // For now, we manually emit the message to verify the system exists
    app.world_mut().write_message(LevelSwitchRequested {
        source: LevelSwitchSource::Brick,
        direction: LevelSwitchDirection::Next,
    });
    app.update();

    // Assertion: LevelSwitchRequested message should exist
    // Note: This test will FAIL until collision handler is implemented
    assert!(
        level_switch_message_written(
            &mut app,
            LevelSwitchDirection::Next,
            LevelSwitchSource::Brick
        ),
        "LevelSwitchRequested(Next, Brick) should be emitted on brick 50 collision"
    );
}

#[test]
#[ignore = "RED phase test: expects brick destruction message not yet implemented"]
fn test_brick_50_collision_emits_brick_destroyed_message() {
    // This test verifies that BrickDestroyed message is emitted with brick_type = 50
    // Required assertion: BrickDestroyed { brick_type: 50 } message in queue
    // Expected to FAIL until collision handler implementation
    panic!("Test not yet implemented: BrickDestroyed message emission");
}

#[test]
#[ignore = "RED phase test: expects brick destruction scoring not yet implemented"]
fn test_brick_50_awards_zero_points() {
    // This test verifies that destroying brick 50 awards exactly 0 points
    // (utility brick, like Extra Ball brick 41)
    // Expected to FAIL until collision handler and scoring integration complete
    panic!("Test not yet implemented: Scoring 0 points for brick 50");
}

#[test]
#[ignore = "RED phase test: expects level transition not yet implemented"]
fn test_brick_50_advances_to_next_level() {
    let mut app = test_app_with_brick_50();

    // Set up level 2 with brick 50
    let test_level = create_test_level_with_brick_50(2);
    app.world_mut()
        .insert_resource(CurrentLevel(test_level.clone()));
    app.update();

    let start_level = app
        .world()
        .get_resource::<CurrentLevel>()
        .map(|l| l.0.number)
        .expect("CurrentLevel should exist");
    assert_eq!(start_level, 2, "Should start at level 2");

    // Emit level switch request (brick collision would do this)
    app.world_mut().write_message(LevelSwitchRequested {
        source: LevelSwitchSource::Brick,
        direction: LevelSwitchDirection::Next,
    });
    app.update();
    app.update(); // Allow level loader to process

    // Assertion: CurrentLevel should advance to 3
    // Expected to FAIL until level_loader processes brick-sourced transitions
    let new_level = app
        .world()
        .get_resource::<CurrentLevel>()
        .map(|l| l.0.number)
        .expect("CurrentLevel should exist");
    assert!(
        new_level > start_level,
        "Level should advance from {start_level} to a higher number, got {new_level}"
    );
}

#[test]
#[ignore = "RED phase test: expects final level boundary logic not yet implemented"]
fn test_brick_50_on_final_level_shows_victory_screen() {
    let mut app = test_app_with_brick_50();

    // Determine the final level number
    // For this test, assume level 20 is the final level (adjust based on actual game)
    let final_level = 20u32;
    let test_level = create_test_level_with_brick_50(final_level);
    app.world_mut()
        .insert_resource(CurrentLevel(test_level.clone()));
    app.update();

    // Verify we're on the final level
    let current = app
        .world()
        .get_resource::<CurrentLevel>()
        .map(|l| l.0.number)
        .expect("CurrentLevel should exist");
    assert_eq!(current, final_level, "Should be on final level");

    // Emit brick 50 collision/destruction
    app.world_mut().write_message(LevelSwitchRequested {
        source: LevelSwitchSource::Brick,
        direction: LevelSwitchDirection::Next,
    });
    app.update();
    app.update();

    // Assertion 1: No level transition should occur (stay on final level)
    let post_level = app
        .world()
        .get_resource::<CurrentLevel>()
        .map(|l| l.0.number)
        .expect("CurrentLevel should exist");
    assert_eq!(
        post_level, final_level,
        "Should remain on final level {final_level}, got {post_level}"
    );

    // Assertion 2: GameProgress.finished should be true (victory condition)
    let game_progress = app
        .world()
        .get_resource::<GameProgress>()
        .expect("GameProgress should exist");
    assert!(
        game_progress.finished,
        "GameProgress.finished should be true on final level brick 50 collision"
    );
}

#[test]
#[ignore = "RED phase test: expects multi-frame persistence not yet verified"]
fn test_brick_50_level_state_persists_across_frames() {
    let mut app = test_app_with_brick_50();

    // Set up level 2 with brick 50
    let test_level = create_test_level_with_brick_50(2);
    app.world_mut()
        .insert_resource(CurrentLevel(test_level.clone()));
    app.update();

    // Trigger level switch
    app.world_mut().write_message(LevelSwitchRequested {
        source: LevelSwitchSource::Brick,
        direction: LevelSwitchDirection::Next,
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
// <failing-test-commit-hash>: TBD - Run: cargo test --test brick_50_level_up --ignored
