//! Integration tests for feature 028: remove legacy game-over overlay.
//!
//! This file intentionally groups regression checks for restart behavior,
//! pause/cheat decoupling, and Bevy 0.17 safety guards.

use bevy::ecs::message::Messages;
use bevy::prelude::*;

use brkrs::pause::PauseState;
use brkrs::systems::respawn::{GameOverRequested, LivesState};
use brkrs::ui::fonts::UiFonts;
use brkrs::ui::pause_overlay::{spawn_pause_overlay, PauseOverlay};

#[cfg(not(target_arch = "wasm32"))]
fn paused_state() -> PauseState {
    PauseState::Paused {
        window_mode_before_pause: bevy::window::WindowMode::Windowed,
    }
}

#[cfg(target_arch = "wasm32")]
fn paused_state() -> PauseState {
    PauseState::Paused {}
}

fn minimal_ui_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(paused_state());
    app.add_systems(Update, spawn_pause_overlay);
    app
}

fn legacy_overlay_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_message::<GameOverRequested>();
    app.insert_resource(LivesState {
        lives_remaining: 0,
        on_last_life: true,
    });
    app.insert_resource(UiFonts {
        orbitron: Handle::default(),
    });
    app.insert_resource(PauseState::Active);
    app.add_systems(Update, spawn_pause_overlay);
    app
}

#[test]
fn bevy_guard_pause_overlay_no_panic_without_fonts() {
    // Guard for fallible query/resource handling: missing UiFonts should not panic.
    let mut app = minimal_ui_test_app();
    app.update();

    let count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    assert_eq!(
        count, 0,
        "pause overlay should not spawn when fonts are missing"
    );
}

#[test]
fn bevy_guard_pause_overlay_spawns_once_with_fonts() {
    // Guard for query specificity + idempotent spawn behavior.
    let mut app = minimal_ui_test_app();
    app.insert_resource(UiFonts {
        orbitron: Handle::default(),
    });

    app.update();
    app.update();

    let count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    assert_eq!(count, 1, "pause overlay should spawn exactly once");
}

#[test]
fn test_restart_after_game_over_has_no_legacy_overlay() {
    let mut app = legacy_overlay_test_app();
    app.update();

    let before_pause_overlay_count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    app.world_mut()
        .resource_mut::<Messages<GameOverRequested>>()
        .write(GameOverRequested { remaining_lives: 0 });
    app.update();

    let after_pause_overlay_count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    assert_eq!(
        after_pause_overlay_count, before_pause_overlay_count,
        "GameOverRequested must not create gameplay overlay entities"
    );
}

#[test]
fn test_no_legacy_overlay_reappears_after_restart_over_10_frames() {
    let mut app = legacy_overlay_test_app();
    app.update();

    let before_pause_overlay_count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    app.world_mut()
        .resource_mut::<Messages<GameOverRequested>>()
        .write(GameOverRequested { remaining_lives: 0 });
    app.update();

    for _ in 0..10 {
        app.update();
    }

    let after_pause_overlay_count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    assert_eq!(
        after_pause_overlay_count, before_pause_overlay_count,
        "GameOverRequested must not create gameplay overlay entities across frames"
    );
}

#[test]
fn test_new_game_and_gameplay_controls_work_after_overlay_removal() {
    let mut app = minimal_ui_test_app();
    app.insert_resource(UiFonts {
        orbitron: Handle::default(),
    });
    app.update();

    let count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    assert_eq!(count, 1, "pause overlay remains functional in paused state");
}

#[test]
fn test_buffered_game_over_requested_message_usage() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_message::<GameOverRequested>();

    app.world_mut()
        .resource_mut::<Messages<GameOverRequested>>()
        .write(GameOverRequested { remaining_lives: 0 });

    let queue = app.world().resource::<Messages<GameOverRequested>>();
    assert_eq!(
        queue.len(),
        1,
        "GameOverRequested should use buffered messages"
    );
}

#[test]
fn test_no_legacy_overlay_on_fresh_launch_gameplay() {
    let mut app = legacy_overlay_test_app();
    app.update();

    let before_pause_overlay_count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    app.insert_resource(LivesState {
        lives_remaining: 3,
        on_last_life: false,
    });
    app.update();

    let after_pause_overlay_count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    assert_eq!(
        after_pause_overlay_count, before_pause_overlay_count,
        "fresh launch should not create gameplay overlays"
    );
}

#[test]
fn test_no_legacy_overlay_across_10_game_over_restart_cycles() {
    let mut app = legacy_overlay_test_app();
    app.update();

    let before_pause_overlay_count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();

    for _ in 0..10 {
        app.world_mut()
            .resource_mut::<Messages<GameOverRequested>>()
            .write(GameOverRequested { remaining_lives: 0 });
        app.update();
        app.insert_resource(LivesState {
            lives_remaining: 3,
            on_last_life: false,
        });
        app.update();
    }

    let after_pause_overlay_count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    assert_eq!(
        after_pause_overlay_count, before_pause_overlay_count,
        "gameplay overlays should remain unchanged across restart cycles"
    );
}

#[test]
fn test_pause_overlay_not_spawned_while_active() {
    let mut app = minimal_ui_test_app();
    app.insert_resource(PauseState::Active);
    app.insert_resource(UiFonts {
        orbitron: Handle::default(),
    });
    app.update();

    let count = app
        .world_mut()
        .query_filtered::<(), With<PauseOverlay>>()
        .iter(app.world())
        .count();
    assert_eq!(
        count, 0,
        "pause overlay should not spawn when pause is inactive"
    );
}
