use bevy::prelude::*;
use bevy::text::Justify;

use crate::systems::cheat_mode::CheatModeToggled;
use crate::ui::fonts::UiFonts;

#[derive(Component)]
pub struct CheatModeIndicator;

/// Spawns or removes the cheat mode indicator in response to CheatModeToggled events
pub fn handle_cheat_indicator(
    mut commands: Commands,
    mut events: MessageReader<CheatModeToggled>,
    existing: Query<Entity, With<CheatModeIndicator>>,
    ui_fonts: Option<Res<UiFonts>>,
) {
    for event in events.read() {
        if event.active {
            if !existing.is_empty() {
                // already exists
                return;
            }

            let Some(fonts) = ui_fonts.as_ref() else {
                warn!("UiFonts missing; skipping cheat mode indicator spawn");
                return;
            };

            let font = fonts.orbitron.clone();

            commands.spawn((
                Text::new("CHEAT MODE"),
                TextFont {
                    font,
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(12.0),
                    bottom: Val::Px(12.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                CheatModeIndicator,
            ));
        } else {
            // deactivate: remove existing indicator(s)
            for entity in existing.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}
