//! Integration tests for Brick 50 (Level Up) and Brick 54 (Level Down) navigation bricks.
//!
//! These bricks allow navigation between levels.
//! Brick 50 advances to the next level when destroyed.
//! Brick 54 returns to the previous level when destroyed.

use bevy::{app::App, input::InputPlugin, prelude::*};
use bevy_rapier3d::prelude::RapierConfiguration;
use brkrs::level_format::{BRICK_50, BRICK_54};
use brkrs::level_loader::{CurrentLevel, LevelAdvanceState, LevelDefinition, LevelLoaderPlugin};
use brkrs::systems::level_switch::{LevelSwitchDirection, LevelSwitchRequested, LevelSwitchSource};
use brkrs::systems::respawn::SpawnPoints;
use brkrs::systems::LevelSwitchPlugin;
use brkrs::GameProgress;

fn test_app() -> App {
    let mut app = App::new();

    // Core plugins
    app.add_plugins((
        MinimalPlugins,
        InputPlugin,
        LevelSwitchPlugin,
        LevelLoaderPlugin,
    ));

    // Physics configuration
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());

    // Game state
    app.insert_resource(GameProgress::default());
    app.insert_resource(LevelAdvanceState::default());
    app.insert_resource(SpawnPoints::default());
    app.insert_resource(ButtonInput::<KeyCode>::default());

    // Assets
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());

    // Physics world
    app.world_mut().spawn(RapierConfiguration::new(1.0));

    app
}

/// Create a test level with a specific brick type at a known position.
fn create_test_level_with_brick(level_number: u32, brick_type: u8) -> LevelDefinition {
    let mut matrix = vec![vec![0u8; 20]; 20];
    matrix[5][5] = brick_type;

    LevelDefinition {
        number: level_number,
        matrix,
        gravity: None,
        #[cfg(feature = "texture_manifest")]
        presentation: None,
        author: Some("test".to_string()),
        description: Some(format!("Test level with brick {brick_type}")),
    }
}

#[test]
fn brick_50_level_up_message_can_be_written() {
    let mut app = test_app();
    app.update();

    // Write level switch message for brick 50
    app.world_mut().write_message(LevelSwitchRequested {
        source: LevelSwitchSource::Brick,
        direction: LevelSwitchDirection::Next,
    });
    app.update();

    // Verify the app continues without panicking - test passes if we reach here
}

#[test]
fn brick_54_level_down_message_can_be_written() {
    let mut app = test_app();
    app.update();

    // Write level switch message for brick 54
    app.world_mut().write_message(LevelSwitchRequested {
        source: LevelSwitchSource::Brick,
        direction: LevelSwitchDirection::Previous,
    });
    app.update();

    // Verify the app continues without panicking - test passes if we reach here
}

#[test]
fn level_definition_with_brick_50_loads() {
    let mut app = test_app();

    let level = create_test_level_with_brick(5, BRICK_50);
    app.world_mut().insert_resource(CurrentLevel(level.clone()));
    app.update();

    // Verify level is set correctly
    let current_level = app
        .world()
        .get_resource::<CurrentLevel>()
        .expect("CurrentLevel should exist");
    assert_eq!(current_level.0.number, 5, "Level should be 5");
    assert_eq!(
        current_level.0.matrix[5][5], BRICK_50,
        "Brick 50 should be at (5, 5)"
    );
}

#[test]
fn level_definition_with_brick_54_loads() {
    let mut app = test_app();

    let level = create_test_level_with_brick(7, BRICK_54);
    app.world_mut().insert_resource(CurrentLevel(level.clone()));
    app.update();

    // Verify level is set correctly
    let current_level = app
        .world()
        .get_resource::<CurrentLevel>()
        .expect("CurrentLevel should exist");
    assert_eq!(current_level.0.number, 7, "Level should be 7");
    assert_eq!(
        current_level.0.matrix[5][5], BRICK_54,
        "Brick 54 should be at (5, 5)"
    );
}
