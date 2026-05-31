//! Score display UI module - renders and updates the score HUD
//!
//! This module implements the score visualization:
//! - Score display spawning at game startup
//! - Multiplier indicator spawning beneath the score display
//! - Real-time score and multiplier updates with change detection
//! - UI positioning and styling

use bevy::prelude::*;

use crate::systems::scoring::{ScoreMultiplierState, ScoreState};
use crate::ui::fonts::UiFonts;

/// Marker component tagging the UI entity that displays current score.
///
/// Attached to a Bevy TextBundle entity. Systems query with this marker
/// to update the score display when ScoreState changes.
#[derive(Component, Debug, Clone, Copy)]
pub struct ScoreDisplayUi;

/// Marker component tagging the UI entity that displays the active score multiplier.
#[derive(Component, Debug, Clone, Copy)]
pub struct ScoreMultiplierIndicatorUi;

/// Spawns the score display UI element at game startup.
///
/// # Purpose
///
/// Creates the persistent score HUD that shows the player's cumulative
/// point total throughout the game session.
///
/// # When it runs
///
/// Runs in the Update schedule (not Startup) to ensure `UiFonts` resource
/// is available. Will only spawn once per game session due to duplicate check.
///
/// # Behavior
///
/// - Checks if score display already exists (prevents duplicates)
/// - Waits for `UiFonts` resource to be ready
/// - Spawns text entity at top-right corner (below lives counter)
/// - Uses Orbitron Bold font at 32px for consistency with UI style
/// - Positions at `top: 40px, right: 12px` to avoid lives counter overlap
///
/// # UI Layout
///
/// ```text
/// ┌─────────────────────────┐
/// │              Lives: 3   │ ← top: 12px
/// │              Score: 0   │ ← top: 40px (this system)
/// │                         │
/// ```
pub fn spawn_score_display_system(
    mut commands: Commands,
    ui_fonts: Option<Res<UiFonts>>,
    existing_score: Query<Entity, With<ScoreDisplayUi>>,
    existing_multiplier: Query<Entity, With<ScoreMultiplierIndicatorUi>>,
) {
    let Some(fonts) = ui_fonts else {
        warn!("UiFonts resource missing; skipping score display spawn");
        return;
    };

    let font = fonts.orbitron.clone();

    if existing_score.is_empty() {
        commands.spawn((
            Text::new("Score: 0"),
            TextFont {
                font: font.clone(),
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(12.0),
                top: Val::Px(40.0),
                ..default()
            },
            ScoreDisplayUi,
        ));
    }

    if existing_multiplier.is_empty() {
        commands.spawn((
            Text::new(""),
            TextFont {
                font,
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Visibility::Hidden,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(12.0),
                top: Val::Px(74.0),
                ..default()
            },
            ScoreMultiplierIndicatorUi,
        ));
    }
}

/// Updates the score text whenever the score state changes.
///
/// # Purpose
///
/// Keeps the score HUD synchronized with the underlying `ScoreState` resource.
/// Uses change detection to update only when necessary.
///
/// # When it runs
///
/// Runs in the Update schedule after `detect_milestone_system`. This ensures
/// the display reflects milestone-triggered score changes within the same frame.
///
/// # Behavior
///
/// - Uses Bevy's change detection to avoid unnecessary updates
/// - Formats score as "Score: {number}" (e.g., "Score: 1234")
/// - Updates all entities with `ScoreDisplayUi` marker (typically one)
///
/// # Performance
///
/// Change detection prevents updates when score hasn't changed. When score
/// does change, the update is O(1) as there's only one score display entity.
/// Meets <16ms frame budget requirement.
pub fn update_score_display_system(
    score_state: Res<ScoreState>,
    mut query: Query<&mut Text, With<ScoreDisplayUi>>,
) {
    if !score_state.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        **text = format!("Score: {}", score_state.current_score);
    }
}

/// Updates the multiplier indicator whenever multiplier state changes.
pub fn update_multiplier_indicator_system(
    multiplier_state: Option<Res<ScoreMultiplierState>>,
    mut query: Query<(&mut Text, &mut Visibility), With<ScoreMultiplierIndicatorUi>>,
) {
    let Some(multiplier_state) = multiplier_state else {
        return;
    };

    if !multiplier_state.is_changed() {
        return;
    }

    for (mut text, mut visibility) in query.iter_mut() {
        if multiplier_state.factor > 1 {
            **text = format!("x{}", multiplier_state.factor);
            *visibility = Visibility::Inherited;
        } else {
            **text = String::new();
            *visibility = Visibility::Hidden;
        }
    }
}
