//! Lives counter HUD for tracking remaining lives.
//!
//! Purpose
//! - Renders a small text HUD showing the player's remaining lives (e.g., `Lives: 3`).
//!
//! When it spawns
//! - `spawn_lives_counter` runs every Update and creates the HUD once (idempotent) when:
//!   - No existing `LivesCounter` entity is present, and
//!   - The `UiFonts` resource is available (desktop loads at Startup, WASM provides it later).
//! - If `UiFonts` is not yet available (common on WASM during early frames), the system logs a
//!   warning and defers spawning until the fonts resource appears on a subsequent frame.
//!
//! How it updates
//! - `update_lives_counter` updates the text only when `LivesState` changes, and it is explicitly
//!   ordered to run after `RespawnSystems::Schedule`, ensuring the display reflects the latest
//!   respawn logic before rendering.
//!
//! Relationship to game over
//! - Both the lives counter and the game-over overlay observe the same `LivesState`.
//! - When lives reach zero, `ui::game_over_overlay::spawn_game_over_overlay` (scheduled after
//!   `RespawnSystems::Schedule`) handles presenting the game-over UI. The lives counter can
//!   coexist; the overlay typically becomes the primary focus. Any future hide/remove behavior can
//!   be handled by that overlay system if desired.
//!
//! Scheduling summary
//! - Spawn attempt: every Update, idempotent, waits for `UiFonts`.
//! - Text updates: after `RespawnSystems::Schedule`, only on `LivesState` changes.

use bevy::prelude::*;

use crate::systems::respawn::LivesState;
use crate::ui::fonts::UiFonts;

/// Marker component for the lives counter UI element.
#[derive(Component)]
pub struct LivesCounter;

/// Spawns the lives counter HUD if it doesn't exist.
pub fn spawn_lives_counter(
    mut commands: Commands,
    existing: Query<Entity, With<LivesCounter>>,
    lives_state: Res<LivesState>,
    ui_fonts: Option<Res<UiFonts>>,
) {
    // Only spawn if it doesn't already exist
    if !existing.is_empty() {
        return;
    }

    let Some(fonts) = ui_fonts else {
        warn!("UiFonts resource missing; skipping lives counter spawn");
        return;
    };

    let font = fonts.orbitron.clone();

    commands.spawn((
        Text::new(format!("Lives: {}", lives_state.lives_remaining)),
        TextFont {
            font,
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            right: Val::Px(12.0),
            ..default()
        },
        LivesCounter,
    ));
}

/// Updates the lives counter text when the lives state changes.
pub fn update_lives_counter(
    lives_state: Res<LivesState>,
    mut counter_query: Query<&mut Text, With<LivesCounter>>,
) {
    // Only update if lives state actually changed
    if !lives_state.is_changed() {
        return;
    }

    if let Ok(mut text) = counter_query.single_mut() {
        **text = format!("Lives: {}", lives_state.lives_remaining);
    }
}
