//! Pause and resume system for the game
//!
//! This module implements the pause/resume functionality including:
//! - Pause state management (PauseState resource)
//! - Input handling (ESC to pause, mouse click to resume)
//! - Physics control (freeze/resume via RapierConfiguration)
//! - Window mode switching (fullscreen ↔ windowed on native platforms)
//!
//! The pause system is implemented as a Bevy plugin that can be added to the app.

use bevy::prelude::*;
use bevy::window::{CursorOptions, PrimaryWindow};
#[cfg(not(target_arch = "wasm32"))]
use bevy::window::{Window, WindowMode};
use bevy_rapier3d::prelude::*;

use crate::level_loader::LevelAdvanceState;
use crate::ui::pause_overlay::{despawn_pause_overlay, spawn_pause_overlay};

/// Global pause state resource.
///
/// Controls whether the game is actively running or paused.
/// On native platforms, stores the window mode before pause to enable restoration.
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseState {
    /// Normal gameplay - physics running, no overlay
    #[default]
    Active,
    /// Paused - physics frozen, overlay visible
    Paused {
        /// Window mode before pause (native only, for restoration on resume)
        #[cfg(not(target_arch = "wasm32"))]
        window_mode_before_pause: WindowMode,
    },
}

/// Plugin that manages pause/resume functionality.
///
/// Registers the `PauseState` resource and all pause-related systems.
pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        // Register pause state resource
        app.init_resource::<PauseState>();

        // Hide cursor on startup
        app.add_systems(Startup, hide_cursor_on_startup);

        // Register pause/resume systems with explicit ordering
        // Execution order: input handling → state effects (physics, window, cursor) → UI updates
        // Physics control runs after level loader systems to ensure pause state takes precedence
        app.add_systems(
            Update,
            (
                // Input handling systems (can run in parallel)
                (handle_pause_input, handle_resume_input),
                // State-dependent systems (run after input, before UI)
                // Physics control runs after LevelAdvanceSystems to avoid race conditions
                apply_pause_to_physics.after(crate::level_loader::LevelAdvanceSystems),
                apply_pause_to_window_mode,
                apply_pause_to_cursor,
                // UI systems (run last, after all state changes)
                (spawn_pause_overlay, despawn_pause_overlay),
            )
                .chain(),
        );
    }
}

/// System that handles ESC key input to pause the game.
///
/// Transitions from Active to Paused state when ESC is pressed.
/// On native platforms, captures the current window mode for restoration on resume.
/// Blocked during level transitions (when LevelAdvanceState.active is true).
fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut pause_state: ResMut<PauseState>,
    level_advance: Res<LevelAdvanceState>,
    lives_state: Res<crate::systems::respawn::LivesState>,
    #[cfg(not(target_arch = "wasm32"))] window: Single<&Window, With<PrimaryWindow>>,
) {
    // Disable pause input when game is over (lives = 0)
    if lives_state.lives_remaining == 0 {
        return;
    }

    // Only allow pause if:
    // 1. ESC was just pressed (frame-level debouncing via just_pressed)
    // 2. Game is currently Active (not already paused)
    // 3. No level transition in progress (FR-012)
    if keyboard.just_pressed(KeyCode::Escape)
        && matches!(*pause_state, PauseState::Active)
        && !level_advance.active
    {
        #[cfg(not(target_arch = "wasm32"))]
        {
            *pause_state = PauseState::Paused {
                window_mode_before_pause: window.mode,
            };
        }
        #[cfg(target_arch = "wasm32")]
        {
            *pause_state = PauseState::Paused {};
        }
    }
}

/// System that controls physics simulation based on pause state.
///
/// Freezes physics when paused, resumes when active.
/// Uses Rapier's physics_pipeline_active flag to preserve all physics state.
fn apply_pause_to_physics(
    pause_state: Res<PauseState>,
    mut rapier_config: Query<&mut RapierConfiguration>,
) {
    let Ok(mut config) = rapier_config.single_mut() else {
        return;
    };
    match *pause_state {
        PauseState::Active => {
            config.physics_pipeline_active = true;
        }
        PauseState::Paused { .. } => {
            config.physics_pipeline_active = false;
        }
    }
}

/// System that handles mouse click input to resume the game.
///
/// Transitions from Paused to Active state when left mouse button is clicked.
fn handle_resume_input(mouse: Res<ButtonInput<MouseButton>>, mut pause_state: ResMut<PauseState>) {
    // Only allow resume if:
    // 1. Left mouse button was just pressed (frame-level debouncing via just_pressed)
    // 2. Game is currently Paused
    if mouse.just_pressed(MouseButton::Left) && matches!(*pause_state, PauseState::Paused { .. }) {
        *pause_state = PauseState::Active;
    }
}

/// System that switches window mode based on pause state (native only).
///
/// When pausing from fullscreen, switches to windowed mode.
/// When resuming, restores the original window mode.
/// WASM variant is a no-op since WASM doesn't support window mode switching.
#[cfg(not(target_arch = "wasm32"))]
fn apply_pause_to_window_mode(
    pause_state: Res<PauseState>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut previous_state: Local<Option<PauseState>>,
) {
    // Only run when pause state changes
    if !pause_state.is_changed() {
        return;
    }

    let current_state = *pause_state;
    let prev = *previous_state;

    match (prev, current_state) {
        // Transitioning from Active to Paused
        (
            _,
            PauseState::Paused {
                window_mode_before_pause,
            },
        ) => {
            // Game just paused - switch to windowed if was fullscreen
            match window_mode_before_pause {
                WindowMode::BorderlessFullscreen(_) | WindowMode::Fullscreen { .. } => {
                    window.mode = WindowMode::Windowed;
                }
                WindowMode::Windowed => {
                    // Already windowed, no change (FR-010)
                }
            }
        }
        // Transitioning from Paused to Active
        (
            Some(PauseState::Paused {
                window_mode_before_pause,
            }),
            PauseState::Active,
        ) => {
            // Game just resumed - restore original window mode
            window.mode = window_mode_before_pause;
        }
        _ => {
            // No transition or already active
        }
    }

    // Update previous state
    *previous_state = Some(current_state);
}

/// WASM variant of window mode switching (no-op).
///
/// WASM doesn't support window mode switching, so this is a no-op placeholder.
#[cfg(target_arch = "wasm32")]
fn apply_pause_to_window_mode(_pause_state: Res<PauseState>) {
    // No-op: WASM does not support window mode switching
}

/// System that controls cursor visibility based on pause state.
///
/// Hides cursor during active gameplay, shows cursor when paused.
fn apply_pause_to_cursor(
    pause_state: Res<PauseState>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    // Only run when pause state changes
    if !pause_state.is_changed() {
        return;
    }

    match *pause_state {
        PauseState::Active => {
            // Game active - hide cursor for gameplay
            cursor_options.visible = false;
        }
        PauseState::Paused { .. } => {
            // Game paused - show cursor for UI interaction
            cursor_options.visible = true;
        }
    }
}

/// Startup system to hide cursor when game launches.
fn hide_cursor_on_startup(mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    cursor_options.visible = false;
}

/// Run condition that returns true when the game is not paused.
///
/// Use this to prevent gameplay systems (like paddle movement) from running during pause.
pub fn not_paused(pause_state: Res<PauseState>) -> bool {
    matches!(*pause_state, PauseState::Active)
}
