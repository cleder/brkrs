//! Pause overlay UI component
//!
//! This module manages the visual overlay displayed when the game is paused.
//! It provides functions to spawn and despawn the "PAUSED\nClick to Resume" message.

use bevy::prelude::*;
use bevy::text::Justify;

use crate::pause::PauseState;
use crate::ui::fonts::UiFonts;
use crate::ui::game_over_overlay::GameOverOverlay;

/// Marker component for the pause overlay UI entity.
///
/// Used to identify and despawn the pause overlay when the game resumes.
#[derive(Component, Debug)]
pub struct PauseOverlay;

/// System that spawns the pause overlay UI when the game is paused.
///
/// Only spawns if the overlay doesn't already exist (prevents duplicates).
/// Run condition: Only when PauseState is Paused.
pub fn spawn_pause_overlay(
    mut commands: Commands,
    pause_state: Res<PauseState>,
    overlay_query: Query<(), With<PauseOverlay>>,
    game_over_query: Query<(), With<GameOverOverlay>>,
    ui_fonts: Option<Res<UiFonts>>,
) {
    // Don't spawn pause overlay if game-over is active
    if !game_over_query.is_empty() {
        return;
    }

    // Only spawn if paused and overlay doesn't exist
    if matches!(*pause_state, PauseState::Paused { .. }) && overlay_query.is_empty() {
        let font = ui_fonts
            .as_ref()
            .map(|f| f.orbitron.clone())
            .unwrap_or_default();

        commands.spawn((
            Text::new("PAUSED\nClick to Resume"),
            TextFont {
                font,
                font_size: 60.0,
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
            PauseOverlay,
        ));
    }
}

/// System that despawns the pause overlay UI when the game resumes.
///
/// Run condition: Only when PauseState is Active.
pub fn despawn_pause_overlay(
    mut commands: Commands,
    pause_state: Res<PauseState>,
    overlay_query: Query<Entity, With<PauseOverlay>>,
) {
    // Only despawn if active and overlay exists
    if matches!(*pause_state, PauseState::Active) {
        for entity in overlay_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
