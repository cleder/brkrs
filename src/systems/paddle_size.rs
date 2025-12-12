//! Paddle size modification system triggered by special brick collisions.
//!
//! This module implements paddle size powerups:
//! - Brick Type 30: Shrinks paddle to 70% (14 units) for 10 seconds
//! - Brick Type 32: Enlarges paddle to 150% (30 units) for 10 seconds
//!
//! Effects are temporary, replace each other, and clear on level changes or life loss.

use bevy::ecs::message::{Message, MessageReader, MessageWriter};
use bevy::prelude::*;
use bevy_rapier3d::prelude::CollisionEvent;

use crate::systems::level_switch::LevelSwitchRequested;
use crate::systems::respawn::LifeLostEvent;
use crate::{Ball, Brick, BrickTypeId, Paddle};

/// Base paddle width in units
pub const PADDLE_BASE_WIDTH: f32 = 20.0;
/// Shrink multiplier (70%)
pub const SHRINK_MULTIPLIER: f32 = 0.7;
/// Enlarge multiplier (150%)
pub const ENLARGE_MULTIPLIER: f32 = 1.5;
/// Effect duration in seconds
pub const EFFECT_DURATION: f32 = 10.0;
/// Minimum paddle width
pub const MIN_PADDLE_WIDTH: f32 = 10.0;
/// Maximum paddle width
pub const MAX_PADDLE_WIDTH: f32 = 30.0;

/// Brick type ID for shrink powerup
pub const BRICK_TYPE_30: u8 = 30;
/// Brick type ID for enlarge powerup
pub const BRICK_TYPE_32: u8 = 32;

/// Component tracking an active paddle size effect
#[derive(Component, Clone, Debug)]
pub struct PaddleSizeEffect {
    /// Type of effect: Shrink or Enlarge
    pub effect_type: SizeEffectType,
    /// Time remaining in seconds before effect expires
    pub remaining_duration: f32,
    /// Original paddle width before effect (always 20.0)
    pub base_width: f32,
}

/// Type of size effect
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SizeEffectType {
    /// Shrink paddle to 70% of base size
    Shrink,
    /// Enlarge paddle to 150% of base size
    Enlarge,
}

/// Message emitted when a size effect is applied to the paddle
#[derive(Message, Debug, Clone, Copy)]
pub struct PaddleSizeEffectApplied {
    /// The paddle entity affected
    pub paddle_entity: Entity,
    /// The type of effect applied
    pub effect_type: SizeEffectType,
    /// The new width after effect
    pub new_width: f32,
}

/// Calculate clamped paddle width based on effect type
pub fn calculate_paddle_width(base_width: f32, effect_type: SizeEffectType) -> f32 {
    let multiplier = match effect_type {
        SizeEffectType::Shrink => SHRINK_MULTIPLIER,
        SizeEffectType::Enlarge => ENLARGE_MULTIPLIER,
    };
    (base_width * multiplier).clamp(MIN_PADDLE_WIDTH, MAX_PADDLE_WIDTH)
}

/// Get color for visual feedback based on effect type
pub fn effect_to_color(effect_type: SizeEffectType) -> Color {
    match effect_type {
        SizeEffectType::Shrink => Color::srgb(1.0, 0.3, 0.3), // Red tint
        SizeEffectType::Enlarge => Color::srgb(0.3, 1.0, 0.3), // Green tint
    }
}

/// Get emissive glow for visual feedback based on effect type
pub fn effect_to_glow(effect_type: SizeEffectType) -> LinearRgba {
    match effect_type {
        SizeEffectType::Shrink => LinearRgba::rgb(0.3, 0.0, 0.0), // Red glow
        SizeEffectType::Enlarge => LinearRgba::rgb(0.0, 0.3, 0.0), // Green glow
    }
}

/// System to detect ball-brick collisions and apply paddle size effects
pub fn detect_powerup_brick_collisions(
    mut collision_events: MessageReader<CollisionEvent>,
    balls: Query<Entity, With<Ball>>,
    bricks: Query<&BrickTypeId, With<Brick>>,
    mut paddles: Query<(Entity, &mut Transform), With<Paddle>>,
    mut commands: Commands,
    mut effect_applied_events: MessageWriter<PaddleSizeEffectApplied>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            // Check if one entity is a ball
            let ball_entity = if balls.contains(*e1) {
                Some(*e1)
            } else if balls.contains(*e2) {
                Some(*e2)
            } else {
                None
            };

            if ball_entity.is_none() {
                continue;
            }

            // Check if the other entity is a powerup brick
            let brick_type = if balls.contains(*e1) {
                bricks.get(*e2).ok()
            } else {
                bricks.get(*e1).ok()
            };

            if let Some(brick_type_id) = brick_type {
                let effect_type = match brick_type_id.0 {
                    BRICK_TYPE_30 => Some(SizeEffectType::Shrink),
                    BRICK_TYPE_32 => Some(SizeEffectType::Enlarge),
                    _ => None,
                };

                if let Some(effect_type) = effect_type {
                    // Apply effect to all paddles (typically just one)
                    for (paddle_entity, mut transform) in paddles.iter_mut() {
                        let new_width = calculate_paddle_width(PADDLE_BASE_WIDTH, effect_type);

                        // Remove any existing effect and insert new one
                        commands.entity(paddle_entity).remove::<PaddleSizeEffect>();
                        commands.entity(paddle_entity).insert(PaddleSizeEffect {
                            effect_type,
                            remaining_duration: EFFECT_DURATION,
                            base_width: PADDLE_BASE_WIDTH,
                        });

                        // Update paddle transform scale (X-axis is width in this game)
                        let scale_factor = new_width / PADDLE_BASE_WIDTH;
                        transform.scale.x = scale_factor;

                        // Emit event for audio/visual feedback
                        effect_applied_events.write(PaddleSizeEffectApplied {
                            paddle_entity,
                            effect_type,
                            new_width,
                        });

                        debug!(
                            "Applied {:?} effect to paddle: width = {}",
                            effect_type, new_width
                        );
                    }
                }
            }
        }
    }
}

/// System to countdown effect timers
pub fn update_effect_timers(
    mut paddles: Query<&mut PaddleSizeEffect>,
    time: Res<Time>,
) {
    for mut effect in paddles.iter_mut() {
        effect.remaining_duration -= time.delta_secs();
        if effect.remaining_duration < 0.0 {
            effect.remaining_duration = 0.0;
        }
    }
}

/// System to remove expired effects and restore paddle size
pub fn remove_expired_effects(
    mut paddles: Query<(Entity, &PaddleSizeEffect, &mut Transform), With<Paddle>>,
    mut commands: Commands,
) {
    for (entity, effect, mut transform) in paddles.iter_mut() {
        if effect.remaining_duration <= 0.0 {
            // Restore paddle to base width
            transform.scale.x = 1.0;

            // Remove effect component
            commands.entity(entity).remove::<PaddleSizeEffect>();

            debug!("Removed expired paddle size effect");
        }
    }
}

/// System to update paddle visual feedback based on active effect
pub fn update_paddle_visual_feedback(
    paddles: Query<(&PaddleSizeEffect, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (effect, material_handle) in paddles.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.base_color = effect_to_color(effect.effect_type);
            material.emissive = effect_to_glow(effect.effect_type);
        }
    }
}

/// System to restore paddle visual appearance when effect is removed
pub fn restore_paddle_visual(
    paddles: Query<&MeshMaterial3d<StandardMaterial>, (With<Paddle>, Without<PaddleSizeEffect>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for material_handle in paddles.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            // Only restore if it's currently tinted (to avoid resetting on every frame)
            if material.base_color != Color::WHITE {
                material.base_color = Color::WHITE;
                material.emissive = LinearRgba::BLACK;
            }
        }
    }
}

/// System to play audio feedback when effect is applied
pub fn play_effect_audio(
    mut effect_applied_events: MessageReader<PaddleSizeEffectApplied>,
    // TODO: Add audio assets and playback when audio system is integrated
) {
    for event in effect_applied_events.read() {
        debug!(
            "Audio: Play {:?} sound for paddle {:?}",
            event.effect_type, event.paddle_entity
        );
        // Placeholder for audio playback integration
        // This will be connected to the audio system later
    }
}

/// System to clear paddle size effects on level change
pub fn clear_effects_on_level_change(
    mut level_switch_events: MessageReader<LevelSwitchRequested>,
    mut paddles: Query<(Entity, &mut Transform), (With<Paddle>, With<PaddleSizeEffect>)>,
    mut commands: Commands,
) {
    for _event in level_switch_events.read() {
        for (entity, mut transform) in paddles.iter_mut() {
            // Restore paddle to base width
            transform.scale.x = 1.0;

            // Remove effect component
            commands.entity(entity).remove::<PaddleSizeEffect>();

            debug!("Cleared paddle size effect on level change");
        }
    }
}

/// System to clear paddle size effects on life loss
pub fn clear_effects_on_life_loss(
    mut life_lost_events: MessageReader<LifeLostEvent>,
    mut paddles: Query<(Entity, &mut Transform), (With<Paddle>, With<PaddleSizeEffect>)>,
    mut commands: Commands,
) {
    for _event in life_lost_events.read() {
        for (entity, mut transform) in paddles.iter_mut() {
            // Restore paddle to base width
            transform.scale.x = 1.0;

            // Remove effect component
            commands.entity(entity).remove::<PaddleSizeEffect>();

            debug!("Cleared paddle size effect on life loss");
        }
    }
}

/// Plugin to register paddle size systems
pub struct PaddleSizePlugin;

impl Plugin for PaddleSizePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PaddleSizeEffectApplied>();

        // Register all systems
        app.add_systems(Update, clear_effects_on_level_change);
        app.add_systems(Update, clear_effects_on_life_loss);
        app.add_systems(Update, detect_powerup_brick_collisions);
        app.add_systems(Update, update_effect_timers);
        app.add_systems(Update, remove_expired_effects);
        app.add_systems(Update, update_paddle_visual_feedback);
        app.add_systems(Update, restore_paddle_visual);
        app.add_systems(Update, play_effect_audio);
    }
}
