//! Game-over overlay UI component
//!
//! Displays "Game over" message when the player runs out of lives.

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
    mut events: MessageReader<GameOverRequested>,
    existing: Query<Entity, With<GameOverOverlay>>,
    lives_state: Res<LivesState>,
    ui_fonts: Option<Res<UiFonts>>,
) {
    // Only spawn if we receive a GameOverRequested event
    if events.read().next().is_none() {
        return;
    }

    // Only spawn if overlay doesn't exist and lives are actually 0
    if !existing.is_empty() || lives_state.lives_remaining != 0 {
        return;
    }

    let font = ui_fonts
        .as_ref()
        .map(|f| f.orbitron.clone())
        .unwrap_or_default();

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
