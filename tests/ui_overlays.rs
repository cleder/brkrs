//! Tests for UI overlay precedence behavior.
//!
//! Purpose: Verify that game-over overlay takes precedence over pause overlay,
//! ensuring players see critical game-over state even when pause is triggered.
//!
//! Constitution VII: Red test commit required before implementation.

use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::window::WindowMode;

use brkrs::pause::PauseState;
use brkrs::systems::respawn::{GameOverRequested, LivesState};
use brkrs::ui::fonts::UiFonts;
use brkrs::ui::game_over_overlay::{spawn_game_over_overlay, GameOverOverlay};
use brkrs::ui::pause_overlay::{spawn_pause_overlay, PauseOverlay};

/// Test that pause overlay does NOT spawn when game-over overlay exists.
///
/// Scenario:
/// 1. Game is in game-over state (GameOverOverlay exists)
/// 2. Player pauses the game (PauseState::Paused)
/// 3. Expected: Pause overlay should NOT spawn (game-over takes precedence)
#[test]
fn pause_overlay_does_not_spawn_when_game_over_active() {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());

    // Add required resources
    app.add_message::<GameOverRequested>();
    app.insert_resource(PauseState::Active);
    app.insert_resource(LivesState {
        lives_remaining: 0,
        on_last_life: false,
    });
    app.insert_resource(UiFonts {
        orbitron: Handle::default(),
    });

    // Register systems
    app.add_systems(Update, (spawn_game_over_overlay, spawn_pause_overlay));

    // Step 1: Trigger game over (lives = 0, emit GameOverRequested)
    app.world_mut()
        .resource_mut::<Messages<GameOverRequested>>()
        .write(GameOverRequested { remaining_lives: 0 });
    app.update();

    // Verify game-over overlay spawned
    let game_over_count = app
        .world_mut()
        .query_filtered::<(), With<GameOverOverlay>>()
        .iter(app.world())
        .count();
    assert_eq!(
        game_over_count, 1,
        "Game-over overlay should spawn when lives = 0"
    );

    // Step 2: Pause the game
    #[cfg(not(target_arch = "wasm32"))]
    app.insert_resource(PauseState::Paused {
        window_mode_before_pause: WindowMode::Windowed,
    });
    #[cfg(target_arch = "wasm32")]
    app.insert_resource(PauseState::Paused {});
    app.update();

    // Verify pause overlay did NOT spawn (game-over takes precedence)
    let pause_count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    assert_eq!(
        pause_count, 0,
        "Pause overlay should NOT spawn when game-over overlay is active"
    );
}

/// Test that game-over overlay can coexist with existing pause overlay.
///
/// Scenario:
/// 1. Game is paused (PauseOverlay exists)
/// 2. Player loses last life (GameOverRequested triggered)
/// 3. Expected: Game-over overlay spawns alongside pause overlay
#[test]
fn game_over_spawns_even_when_paused() {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());

    // Add required resources
    app.add_message::<GameOverRequested>();
    #[cfg(not(target_arch = "wasm32"))]
    app.insert_resource(PauseState::Paused {
        window_mode_before_pause: WindowMode::Windowed,
    });
    #[cfg(target_arch = "wasm32")]
    app.insert_resource(PauseState::Paused {});
    app.insert_resource(LivesState {
        lives_remaining: 1,
        on_last_life: false,
    }); // Start with 1 life
    app.insert_resource(UiFonts {
        orbitron: Handle::default(),
    });

    // Register systems
    app.add_systems(Update, (spawn_game_over_overlay, spawn_pause_overlay));

    // Step 1: Pause the game (spawn pause overlay)
    app.update();

    // Verify pause overlay spawned
    let pause_count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    assert_eq!(pause_count, 1, "Pause overlay should spawn when paused");

    // Step 2: Lose last life (trigger game over while paused)
    app.insert_resource(LivesState {
        lives_remaining: 0,
        on_last_life: false,
    });
    app.world_mut()
        .resource_mut::<Messages<GameOverRequested>>()
        .write(GameOverRequested { remaining_lives: 0 });
    app.update();

    // Verify game-over overlay spawned
    let game_over_count = app
        .world_mut()
        .query_filtered::<(), With<GameOverOverlay>>()
        .iter(app.world())
        .count();
    assert_eq!(
        game_over_count, 1,
        "Game-over overlay should spawn even when paused"
    );

    // Both overlays should coexist (game-over doesn't prevent pause)
    let pause_count_after = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    assert_eq!(
        pause_count_after, 1,
        "Pause overlay should remain when game-over spawns"
    );
}
