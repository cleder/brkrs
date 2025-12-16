//! Score display UI module - renders and updates the score HUD
//!
//! This module implements the score visualization:
//! - Score display spawning at game startup
//! - Real-time score updates with change detection
//! - UI positioning and styling

use bevy::prelude::*;

/// Marker component tagging the UI entity that displays current score.
///
/// Attached to a Bevy TextBundle entity. Systems query with this marker
/// to update the score display when ScoreState changes.
#[derive(Component, Debug, Clone, Copy)]
pub struct ScoreDisplayUi;

/// Spawns the score display UI element at game startup.
///
/// Creates a TextBundle positioned in the top-right corner of the screen
/// with the Orbitron font, displaying the initial score of 0.
pub fn spawn_score_display_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Orbitron/Orbitron-Bold.ttf");

    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font,
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(20.0),
            top: Val::Px(20.0),
            ..default()
        },
        ScoreDisplayUi,
    ));
}
