//! Multi-hit brick system module.
//!
//! This module provides systems and events for handling bricks that require
//! multiple ball collisions to destroy (indices 10-13). Each hit transitions
//! the brick to the next lower index until it becomes a simple stone (index 20),
//! which can then be destroyed with one more hit.
//!
//! # Brick Lifecycle
//!
//! ```text
//! Index 13 (4 hits) → Index 12 (3 hits) → Index 11 (2 hits) → Index 10 (1 hit) → Index 20 (stone) → Destroyed
//! ```
//!
//! # Usage
//!
//! The [`MultiHitBrickHit`] event is emitted when a multi-hit brick is damaged.
//! Subscribe to this event via an observer for audio feedback or scoring integration.
//!
//! The [`watch_brick_type_changes`] system automatically updates brick visuals
//! when their [`BrickTypeId`](crate::BrickTypeId) changes.

use bevy::prelude::*;

use crate::BrickTypeId;

/// Event emitted when a multi-hit brick (indices 10-13) is hit by the ball.
///
/// This event is triggered when a multi-hit brick collision is detected,
/// allowing observer systems to react for audio feedback or scoring.
///
/// # Fields
///
/// * `entity` - The brick entity that was hit
/// * `previous_type` - The brick's type ID before the hit (10-13)
/// * `new_type` - The brick's type ID after the hit (10-12 or 20)
///
/// # Example
///
/// ```ignore
/// fn on_multi_hit(trigger: On<MultiHitBrickHit>) {
///     let event = trigger.event();
///     info!("Brick {:?} hit: {} -> {}", event.entity, event.previous_type, event.new_type);
/// }
///
/// // In app setup:
/// app.add_observer(on_multi_hit);
/// ```
#[derive(Event, Debug, Clone)]
pub struct MultiHitBrickHit {
    /// The brick entity that was hit.
    pub entity: Entity,
    /// The brick's type ID before the hit (10-13).
    pub previous_type: u8,
    /// The brick's type ID after the hit (10-12 or 20 for final transition).
    pub new_type: u8,
}

/// System that watches for brick type changes and swaps materials accordingly.
///
/// This system detects changes to `BrickTypeId` components and updates the
/// brick's material from the type variant registry. It mirrors the existing
/// `watch_ball_type_changes` pattern from the textures module.
///
/// Runs every frame and checks for bricks whose `BrickTypeId` changed since last frame.
#[cfg(feature = "texture_manifest")]
pub fn watch_brick_type_changes(
    mut bricks: Query<
        (&BrickTypeId, &mut MeshMaterial3d<StandardMaterial>),
        (With<crate::Brick>, Changed<BrickTypeId>),
    >,
    type_registry: Res<crate::systems::textures::TypeVariantRegistry>,
    fallback: Option<Res<crate::systems::textures::FallbackRegistry>>,
) {
    use crate::systems::textures::ObjectClass;

    for (brick_type, mut material) in bricks.iter_mut() {
        if let Some(handle) = type_registry.get(ObjectClass::Brick, brick_type.0) {
            debug!(
                target: "textures::brick_type",
                type_id = brick_type.0,
                "Swapping brick material for type variant"
            );
            material.0 = handle;
        } else if let Some(fb) = &fallback {
            debug!(
                target: "textures::brick_type",
                type_id = brick_type.0,
                "No type variant for brick; using fallback"
            );
            material.0 = fb.brick.clone();
        }
    }
}

// NOTE: Audio observer for `MultiHitBrickHit` events has been moved to
// `src/systems/audio.rs` and is registered via `AudioPlugin`.
//
// The old placeholder logging observer was intentionally removed to avoid
// duplicate audio handlers. If you need to add non-audio side-effects for
// multi-hit events, add a separate observer here with a distinct name.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_hit_brick_hit_event_fields() {
        let event = MultiHitBrickHit {
            entity: Entity::PLACEHOLDER,
            previous_type: 13,
            new_type: 12,
        };

        assert_eq!(event.previous_type, 13);
        assert_eq!(event.new_type, 12);
    }

    #[test]
    fn multi_hit_brick_hit_event_clone() {
        let event = MultiHitBrickHit {
            entity: Entity::PLACEHOLDER,
            previous_type: 10,
            new_type: 20,
        };
        let cloned = event.clone();

        assert_eq!(cloned.previous_type, event.previous_type);
        assert_eq!(cloned.new_type, event.new_type);
    }
}
