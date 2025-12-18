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
    current_level: Option<Res<crate::level_loader::CurrentLevel>>,
) {
    if !existing.is_empty() {
        return;
    }

    let Some(fonts) = ui_fonts else {
        warn!("UiFonts resource missing; skipping level label spawn");
        return;
    };

    let font = fonts.orbitron.clone();

    // Determine initial label: prefer current level if available
    let initial_label = current_level
        .as_ref()
        .map(|c| format!("Level {}", c.0.number))
        .unwrap_or_else(|| "Level".to_string());

    // Root node spans the full width and aligns content to the left so the label is placed at top-left
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            LevelLabelRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(initial_label),
                TextFont {
                    font,
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect {
                        top: Val::Px(12.0),
                        left: Val::Px(12.0),
                        ..default()
                    },
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
    current_level: Option<Res<crate::level_loader::CurrentLevel>>,
) {
    let event = trigger.event();
    info!("LevelStarted event received: {}", event.level_index);

    if let Some(curr) = current_level.as_ref() {
        if curr.0.number != event.level_index {
            warn!(
                "LevelStarted index ({}) differs from CurrentLevel.number ({})",
                event.level_index, curr.0.number
            );
        }
    }

    let label = format!("Level {}", event.level_index);

    if let Ok(mut text) = query.single_mut() {
        **text = label.clone();
    }

    // Record announcement for platform integration or tests
    announcement.last = Some(label.clone());
    info!("Accessibility announcement queued: {}", label);
}

/// Sync HUD label to CurrentLevel when resource changes.
pub fn sync_with_current_level(
    current_level: Option<Res<crate::level_loader::CurrentLevel>>,
    mut query: Query<&mut Text, With<LevelLabelText>>,
    announcement: Option<ResMut<AccessibilityAnnouncement>>,
) {
    let Some(curr) = current_level else {
        return;
    };

    if !curr.is_changed() {
        return;
    }

    let label = format!("Level {}", curr.0.number);
    if let Ok(mut text) = query.single_mut() {
        **text = label.clone();
    }

    if let Some(mut ann) = announcement {
        ann.last = Some(label.clone());
        info!("Accessibility announcement queued (sync): {}", label);
    } else {
        debug!(
            "AccessibilityAnnouncement resource missing; queued label: {} (not recorded)",
            label
        );
    }
}

/// Minimal test helper: update nothing when no change
pub fn noop() {}
