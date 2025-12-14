//! Lives counter HUD display for tracking remaining lives.

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

    let font = ui_fonts
        .as_ref()
        .map(|f| f.orbitron.clone())
        .unwrap_or_default();

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

    for mut text in counter_query.iter_mut() {
        **text = format!("Lives: {}", lives_state.lives_remaining);
    }
}
