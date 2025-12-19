use bevy::prelude::ImageNode;
use bevy::prelude::*;

use crate::systems::cheat_mode::CheatModeToggled;

#[derive(Component)]
pub struct CheatModeIndicator;

/// Cached cheat mode indicator texture (loaded once at startup).
/// Constitution VIII: Asset Handle Reuse — load assets once, reuse everywhere.
#[derive(Resource)]
pub struct CheatIndicatorTexture {
    pub handle: Handle<Image>,
}

/// Spawns or removes the cheat mode indicator in response to CheatModeToggled events
pub fn handle_cheat_indicator(
    mut commands: Commands,
    mut events: MessageReader<CheatModeToggled>,
    existing: Query<Entity, With<CheatModeIndicator>>,
    cached_texture: Option<Res<CheatIndicatorTexture>>,
) {
    for event in events.read() {
        if event.active {
            if !existing.is_empty() {
                // already exists
                return;
            }

            let Some(texture) = cached_texture.as_ref() else {
                warn!("CheatIndicatorTexture resource missing; skipping indicator spawn");
                return;
            };

            // Use cached texture handle (loaded once at startup)
            // Constitution VIII: Asset Handle Reuse — no per-toggle asset loading
            let handle = texture.handle.clone();

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
