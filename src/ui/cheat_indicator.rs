use bevy::prelude::ImageNode;
use bevy::prelude::*;

use crate::systems::cheat_mode::CheatModeToggled;

#[derive(Component)]
pub struct CheatModeIndicator;

/// Spawns or removes the cheat mode indicator in response to CheatModeToggled events
pub fn handle_cheat_indicator(
    mut commands: Commands,
    mut events: MessageReader<CheatModeToggled>,
    existing: Query<Entity, With<CheatModeIndicator>>,
    asset_server: Option<Res<AssetServer>>,
) {
    for event in events.read() {
        if event.active {
            if !existing.is_empty() {
                // already exists
                return;
            }

            let Some(assets) = asset_server.as_ref() else {
                warn!("AssetServer missing; skipping cheat mode indicator spawn");
                return;
            };

            // Load the cheat mode texture from the project's assets folder. The asset path
            // is relative to the `assets/` directory.
            let handle = assets.load("textures/default/cheat-mode-128.png");

            commands.spawn((
                // ImageNode is the UI image component in this Bevy version; create one from the
                // texture handle and position it using a `Node`-like component for absolute
                // positioning.
                ImageNode::new(handle),
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(12.0),
                    bottom: Val::Px(12.0),
                    ..default()
                },
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
