//! Game-over overlay UI
//!
//! Purpose
//! - Shows a centered "Game over" message when the player runs out of lives.
//!
//! When it spawns
//! - `spawn_game_over_overlay` listens for `GameOverRequested` and only spawns when:
//!   - The event is received, and
//!   - `LivesState.lives_remaining == 0`, and
//!   - No existing `GameOverOverlay` entity is present (idempotent), and
//!   - `UiFonts` is available (logs a warning and defers otherwise).
//!
//! Scheduling and integration
//! - Registered in Update after `RespawnSystems::Schedule`, ensuring lives and respawn logic have
//!   finished before the overlay is evaluated and potentially spawned.
//! - Uses the same `UiFonts` resource as the HUD to render text consistently across platforms.
//!
//! Relationship to other UI
//! - The lives counter continues to exist; the overlay becomes the primary focus once visible.
//!   Any future logic to hide/disable the counter during game-over can be added in the overlay
//!   system if desired.

use bevy::prelude::*;
use bevy::text::Justify;

use crate::systems::respawn::{GameOverRequested, LivesState};
use crate::ui::fonts::UiFonts;

/// Marker component for the game-over overlay UI entity.
#[derive(Component, Debug)]
pub struct GameOverOverlay;

/// System that spawns the game-over overlay when GameOverRequested is emitted.
///
/// Displays "Game over" centered on screen with large white text.
pub fn spawn_game_over_overlay(
    mut commands: Commands,
    events: Option<MessageReader<GameOverRequested>>,
    existing: Query<Entity, With<GameOverOverlay>>,
    lives_state: Option<Res<LivesState>>,
    ui_fonts: Option<Res<UiFonts>>,
) {
    // Only spawn if we receive a GameOverRequested event
    let Some(mut events) = events else {
        return;
    };
    if events.read().next().is_none() {
        return;
    }

    // Only spawn if overlay doesn't exist and lives are actually 0
    let Some(lives_state) = lives_state else {
        warn!("LivesState resource missing; skipping game over overlay spawn");
        return;
    };
    if !existing.is_empty() || lives_state.lives_remaining != 0 {
        return;
    }

    let Some(fonts) = ui_fonts else {
        warn!("UiFonts resource missing; skipping game over overlay spawn");
        return;
    };

    let font = fonts.orbitron.clone();

    commands.spawn((
        Text::new("Game over"),
        TextFont {
            font,
            font_size: 80.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        GameOverOverlay,
    ));
}
