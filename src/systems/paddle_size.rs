//! Paddle size modification system triggered by special brick collisions.
//!
//! This module implements paddle size powerups:
//! - Brick Type 30: Shrinks paddle to 70% (14 units) for 10 seconds
//! - Brick Type 32: Enlarges paddle to 150% (30 units) for 10 seconds
//!
//! Effects are temporary, replace each other, and clear on level changes or life loss.
//!
//! # System Organization
//!
//! Systems are organized using the [`PaddleSizeSystems`] SystemSet enum:
//! - [`PaddleSizeSystems::Detect`]: Detect collisions / inputs
//! - [`PaddleSizeSystems::UpdateTimers`]: Update effect timers
//! - [`PaddleSizeSystems::Cleanup`]: Cleanup expired effects
//! - [`PaddleSizeSystems::Visual`]: Visual updates (change-driven)
//! - [`PaddleSizeSystems::Audio`]: Optional audio feedback
//!
//! Ordering: Detect -> UpdateTimers -> Cleanup -> Visual -> Audio

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

/// System set organization for paddle size feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum PaddleSizeSystems {
    /// Detect collisions / inputs
    Detect,
    /// Update effect timers
    UpdateTimers,
    /// Cleanup expired effects
    Cleanup,
    /// Visual updates (change-driven)
    Visual,
    /// Optional audio feedback
    Audio,
}

/// Brick type ID for shrink powerup
pub const BRICK_TYPE_30: u8 = 30;
/// Brick type ID for enlarge powerup
pub const BRICK_TYPE_32: u8 = 32;

/// Component tracking an active paddle size effect
#[derive(Component, Clone, Debug)]
pub struct PaddleSizeEffect {
    /// Type of effect: Shrink or Enlarge
    pub effect_type: SizeEffectType,
    /// Timer tracking effect duration
    pub timer: Timer,
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
        // Softer red tint for shrink
        SizeEffectType::Shrink => Color::srgb(1.0, 0.5, 0.5),
        // Extra gentle green tint for enlarge
        SizeEffectType::Enlarge => Color::srgb(0.68, 0.78, 0.68),
    }
}

/// Get emissive glow for visual feedback based on effect type
pub fn effect_to_glow(effect_type: SizeEffectType) -> LinearRgba {
    match effect_type {
        // Softer red glow to match subtler tint
        SizeEffectType::Shrink => LinearRgba::rgb(0.15, 0.0, 0.0),
        // Softer green glow to match subtler tint
        SizeEffectType::Enlarge => LinearRgba::rgb(0.0, 0.08, 0.0),
    }
}

/// System to detect ball-brick collisions and apply paddle size effects
pub fn detect_powerup_brick_collisions(
    collision_events: Option<MessageReader<CollisionEvent>>,
    balls: Query<Entity, With<Ball>>,
    bricks: Query<&BrickTypeId, With<Brick>>,
    mut paddles: Query<(Entity, &mut Transform), With<Paddle>>,
    mut commands: Commands,
    mut effect_applied_events: MessageWriter<PaddleSizeEffectApplied>,
) {
    let Some(mut collision_events) = collision_events else {
        return;
    };

    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            // Determine which entity is the ball and which is the brick
            let (_ball_entity, brick_entity) = if balls.contains(*e1) {
                (*e1, *e2)
            } else if balls.contains(*e2) {
                (*e2, *e1)
            } else {
                continue; // Neither entity is a ball
            };

            // Check if the brick entity is a powerup brick
            let brick_type = bricks.get(brick_entity).ok();

            if let Some(brick_type_id) = brick_type {
                let effect_type = match brick_type_id.0 {
                    BRICK_TYPE_30 => Some(SizeEffectType::Shrink),
                    BRICK_TYPE_32 => Some(SizeEffectType::Enlarge),
                    _ => None,
                };

                if let Some(effect_type) = effect_type {
                    // Apply effect to all paddles
                    // Note: Game design assumes single paddle, but implementation supports multiple
                    for (paddle_entity, mut transform) in paddles.iter_mut() {
                        let new_width = calculate_paddle_width(PADDLE_BASE_WIDTH, effect_type);

                        // Remove any existing effect and insert new one
                        commands.entity(paddle_entity).remove::<PaddleSizeEffect>();
                        commands.entity(paddle_entity).insert(PaddleSizeEffect {
                            effect_type,
                            timer: Timer::from_seconds(EFFECT_DURATION, TimerMode::Once),
                            base_width: PADDLE_BASE_WIDTH,
                        });

                        // Update paddle length: the capsule's height axis maps to world Z after rotation, so scale Y
                        let scale_factor = new_width / PADDLE_BASE_WIDTH;
                        transform.scale.y = scale_factor;

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
pub fn update_effect_timers(mut paddles: Query<&mut PaddleSizeEffect>, time: Res<Time>) {
    for mut effect in paddles.iter_mut() {
        effect.timer.tick(time.delta());
    }
}

/// System to remove expired effects and restore paddle size
pub fn remove_expired_effects(
    mut paddles: Query<(Entity, &PaddleSizeEffect, &mut Transform), With<Paddle>>,
    mut commands: Commands,
) {
    for (entity, effect, mut transform) in paddles.iter_mut() {
        if effect.timer.is_finished() {
            // Restore paddle to base length
            transform.scale.y = 1.0;

            // Remove effect component
            commands.entity(entity).remove::<PaddleSizeEffect>();

            debug!("Removed expired paddle size effect");
        }
    }
}

/// System to update paddle visual feedback based on active effect
/// Only runs when PaddleSizeEffect changes (reactive updates)
pub fn update_paddle_visual_feedback(
    paddles: Query<
        (&PaddleSizeEffect, &MeshMaterial3d<StandardMaterial>),
        Changed<PaddleSizeEffect>,
    >,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
) {
    let Some(mut materials) = materials else {
        return;
    };
    for (effect, material_handle) in paddles.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.base_color = effect_to_color(effect.effect_type);
            material.emissive = effect_to_glow(effect.effect_type);
        }
    }
}

/// System to restore paddle visual appearance when effect is removed
/// Uses RemovedComponents for event-driven restoration (only runs when effect removed)
pub fn restore_paddle_visual(
    mut removed_effects: RemovedComponents<PaddleSizeEffect>,
    paddles: Query<&MeshMaterial3d<StandardMaterial>, With<Paddle>>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
) {
    let Some(mut materials) = materials else {
        return;
    };
    for entity in removed_effects.read() {
        // Check if entity still exists and has a paddle material
        if let Ok(material_handle) = paddles.get(entity) {
            if let Some(material) = materials.get_mut(&material_handle.0) {
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
    level_switch_events: Option<MessageReader<LevelSwitchRequested>>,
    mut paddles: Query<(Entity, &mut Transform), (With<Paddle>, With<PaddleSizeEffect>)>,
    mut commands: Commands,
) {
    if let Some(mut events) = level_switch_events {
        for _event in events.read() {
            for (entity, mut transform) in paddles.iter_mut() {
                // Restore paddle to base length
                transform.scale.y = 1.0;

                // Remove effect component
                commands.entity(entity).remove::<PaddleSizeEffect>();

                debug!("Cleared paddle size effect on level change");
            }
        }
    }
}

/// System to clear paddle size effects on life loss
/// Note: Uses Option<MessageReader> for test compatibility where messages may not be registered
pub fn clear_effects_on_life_loss(
    life_lost_events: Option<MessageReader<LifeLostEvent>>,
    mut paddles: Query<(Entity, &mut Transform), (With<Paddle>, With<PaddleSizeEffect>)>,
    mut commands: Commands,
) {
    if let Some(mut events) = life_lost_events {
        for _event in events.read() {
            for (entity, mut transform) in paddles.iter_mut() {
                // Restore paddle to base length
                transform.scale.y = 1.0;

                // Remove effect component
                commands.entity(entity).remove::<PaddleSizeEffect>();

                debug!("Cleared paddle size effect on life loss");
            }
        }
    }
}

/// Plugin to register paddle size systems
pub struct PaddleSizePlugin;

impl Plugin for PaddleSizePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PaddleSizeEffectApplied>();

        // Register systems with explicit ordering to ensure deterministic execution
        // Ordering: collision detection → timer updates → effect removal → visual feedback
        app.add_systems(Update, detect_powerup_brick_collisions);
        app.add_systems(
            Update,
            update_effect_timers.after(detect_powerup_brick_collisions),
        );
        app.add_systems(Update, remove_expired_effects.after(update_effect_timers));
        app.add_systems(
            Update,
            update_paddle_visual_feedback.after(remove_expired_effects),
        );
        app.add_systems(Update, restore_paddle_visual.after(remove_expired_effects));

        // Event-driven cleanup systems can run independently
        app.add_systems(Update, clear_effects_on_level_change);

        // clear_effects_on_life_loss must run before paddle shrink animation captures start_scale
        // to ensure powerup effects are cleared and paddle returns to base scale first
        app.add_systems(
            Update,
            clear_effects_on_life_loss.before(crate::systems::respawn::RespawnSystems::Detect),
        );

        app.add_systems(Update, play_effect_audio);
    }
}
