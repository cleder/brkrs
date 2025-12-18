//! Level label HUD
//!
//! Spawns a top-center HUD label showing the current level number and updates it on `LevelStarted` events.

use bevy::prelude::*;

use crate::ui::fonts::UiFonts;

/// Marker component for the level label root node.
#[derive(Component)]
pub struct LevelLabelRoot;

/// Marker component for the actual text entity.
#[derive(Component)]
pub struct LevelLabelText;

/// Resource to capture last accessibility announcement (testable, can be wired to platform hooks).
#[derive(Resource, Default, Debug, Clone)]
pub struct AccessibilityAnnouncement {
    pub last: Option<String>,
}

/// Spawns top-center level label HUD if it doesn't exist.
pub fn spawn_level_label(
    mut commands: Commands,
    existing: Query<Entity, With<LevelLabelRoot>>,
    ui_fonts: Option<Res<UiFonts>>,
) {
    if !existing.is_empty() {
        return;
    }

    let Some(fonts) = ui_fonts else {
        warn!("UiFonts resource missing; skipping level label spawn");
        return;
    };

    let font = fonts.orbitron.clone();

    commands
        .spawn((Node { ..default() }, LevelLabelRoot))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Level"),
                TextFont {
                    font,
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Relative,
                    ..default()
                },
                LevelLabelText,
            ));
        });
}

/// Observer for `LevelStarted` — updates the HUD text and records an accessibility announcement.
pub fn on_level_started(
    trigger: On<crate::systems::LevelStarted>,
    mut query: Query<&mut Text, With<LevelLabelText>>,
    mut announcement: ResMut<AccessibilityAnnouncement>,
) {
    let event = trigger.event();
    let label = format!("Level {}", event.level_index);

    if let Ok(mut text) = query.single_mut() {
        **text = label.clone();
    }

    // Record announcement for platform integration or tests
    announcement.last = Some(label.clone());
    info!("Accessibility announcement queued: {}", label);
}

/// Minimal test helper: update nothing when no change
pub fn noop() {}
