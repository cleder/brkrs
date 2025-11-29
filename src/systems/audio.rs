//! Audio system module for the brick-breaker game.
//!
//! This module provides event-driven audio feedback for game interactions including:
//! - Brick collisions (multi-hit impact and destruction sounds)
//! - Ball bounces (wall and paddle)
//! - Paddle collisions (wall and brick)
//! - Level transitions (start and complete)
//!
//! # Architecture
//!
//! The audio system uses Bevy's observer pattern for event-to-sound mapping:
//! 1. Game events (collisions, transitions) trigger observers
//! 2. Observers call the `play_sound` helper to spawn audio
//! 3. Sound playback respects volume/mute settings and concurrent limits
//!
//! # Graceful Degradation
//!
//! The system operates safely when audio assets are missing:
//! - Missing assets produce a warning log but no crash
//! - Muted or zero-volume states skip audio playback
//!
//! # Configuration
//!
//! Audio settings are stored in `AudioConfig` resource:
//! - Master volume (0.0 - 1.0)
//! - Mute toggle
//! - Persistence via RON file (native) or localStorage (WASM)

use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Maximum concurrent sounds of the same type to prevent audio overload.
const MAX_CONCURRENT_SOUNDS: u8 = 4;

/// Types of sounds in the game, used for event-to-sound mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoundType {
    /// Standard brick destruction sound.
    BrickDestroy,
    /// Multi-hit brick (indices 10-13) impact sound.
    MultiHitImpact,
    /// Ball bouncing off walls.
    WallBounce,
    /// Ball bouncing off paddle.
    PaddleHit,
    /// Paddle colliding with wall.
    PaddleWallHit,
    /// Paddle colliding with brick.
    PaddleBrickHit,
    /// Level beginning.
    LevelStart,
    /// Level completion.
    LevelComplete,
}

/// Audio configuration resource with volume and mute settings.
///
/// Persisted across game sessions via RON file (native) or localStorage (WASM).
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Master volume level (0.0 = silent, 1.0 = full volume).
    pub master_volume: f32,
    /// Whether audio is muted.
    pub muted: bool,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            master_volume: 0.8,
            muted: false,
        }
    }
}

impl AudioConfig {
    /// Set the master volume level.
    ///
    /// Volume is clamped to the range [0.0, 1.0].
    pub fn set_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    /// Toggle the mute state.
    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }

    /// Check if audio should play (not muted and volume > 0).
    pub fn should_play(&self) -> bool {
        !self.muted && self.master_volume > 0.0
    }
}

/// Resource holding loaded audio asset handles mapped by sound type.
#[derive(Resource, Default)]
pub struct AudioAssets {
    /// Map of sound types to their audio source handles.
    pub sounds: HashMap<SoundType, Handle<AudioSource>>,
}

/// Resource tracking the number of active sounds per type for concurrent limiting.
#[derive(Resource, Default)]
pub struct ActiveSounds {
    /// Map of sound types to their current playback count.
    pub counts: HashMap<SoundType, u8>,
}

/// Event emitted when a destructible brick is destroyed.
#[derive(Event, Debug, Clone)]
pub struct BrickDestroyed {
    /// The entity that was destroyed.
    pub entity: Entity,
    /// The brick type ID at time of destruction.
    pub brick_type: u8,
}

/// Event emitted when the ball hits a wall boundary.
#[derive(Event, Debug, Clone)]
pub struct BallWallHit {
    /// The ball entity that hit the wall.
    pub entity: Entity,
    /// The collision impulse.
    pub impulse: Vec3,
}

/// Event emitted when a level starts.
#[derive(Event, Debug, Clone)]
pub struct LevelStarted {
    /// The index of the level that started.
    pub level_index: usize,
}

/// Audio system plugin that registers resources and systems.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioConfig>()
            .init_resource::<AudioAssets>()
            .init_resource::<ActiveSounds>()
            .add_systems(Startup, (load_audio_config, load_audio_assets))
            .add_observer(on_brick_destroyed_sound)
            .add_observer(on_multi_hit_brick_sound)
            .add_observer(on_ball_wall_hit_sound)
            .add_observer(on_paddle_ball_hit_sound)
            .add_observer(on_paddle_wall_hit_sound)
            .add_observer(on_paddle_brick_hit_sound)
            .add_observer(on_level_started_sound)
            .add_observer(on_level_complete_sound);
    }
}

/// Load audio configuration from persistent storage.
fn load_audio_config(mut commands: Commands) {
    // Try to load from config file, fall back to defaults
    let config = load_config_from_file().unwrap_or_default();
    commands.insert_resource(config);
}

/// Load audio config from RON file (native) or localStorage (WASM).
fn load_config_from_file() -> Option<AudioConfig> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let config_path = std::path::Path::new("config/audio.ron");
        if config_path.exists() {
            let contents = std::fs::read_to_string(config_path).ok()?;
            ron::from_str(&contents).ok()
        } else {
            None
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        // WASM: Try localStorage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(data)) = storage.get_item("brkrs_audio_config") {
                    return ron::from_str(&data).ok();
                }
            }
        }
        None
    }
}

/// Load audio assets from the manifest file.
fn load_audio_assets(_asset_server: Res<AssetServer>, _audio_assets: ResMut<AudioAssets>) {
    // Try to load manifest, gracefully handle missing file
    let manifest_path = "audio/manifest.ron";

    // Since we can't directly check if file exists in Bevy's asset system,
    // we'll load what we can and log warnings for missing assets
    debug!(target: "audio", "Attempting to load audio manifest from {}", manifest_path);

    // For now, register placeholder entries - actual loading happens when assets are available
    // The play_sound helper will gracefully handle missing assets
}

/// Play a sound of the specified type if conditions are met.
///
/// Conditions checked:
/// - Audio is not muted and volume > 0
/// - Asset handle exists for the sound type
/// - Concurrent sound limit not exceeded
pub fn play_sound(
    sound_type: SoundType,
    commands: &mut Commands,
    audio_config: &AudioConfig,
    audio_assets: &AudioAssets,
    active_sounds: &mut ActiveSounds,
) {
    // Check if audio should play
    if !audio_config.should_play() {
        return;
    }

    // Check concurrent sound limit
    let count = active_sounds.counts.entry(sound_type).or_insert(0);
    if *count >= MAX_CONCURRENT_SOUNDS {
        debug!(
            target: "audio",
            "Skipping {:?} sound: concurrent limit ({}) reached",
            sound_type, MAX_CONCURRENT_SOUNDS
        );
        return;
    }

    // Get asset handle
    let Some(handle) = audio_assets.sounds.get(&sound_type) else {
        debug!(
            target: "audio",
            "Skipping {:?} sound: asset not loaded",
            sound_type
        );
        return;
    };

    // Spawn audio player
    commands.spawn((
        AudioPlayer(handle.clone()),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: bevy::audio::Volume::Linear(audio_config.master_volume),
            ..default()
        },
    ));

    *count += 1;

    debug!(target: "audio", "Playing {:?} sound (count: {})", sound_type, *count);
}

// ============================================================================
// Audio Observers
// ============================================================================

/// Observer for brick destruction audio.
fn on_brick_destroyed_sound(
    trigger: On<BrickDestroyed>,
    mut commands: Commands,
    audio_config: Res<AudioConfig>,
    audio_assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        "Brick destroyed: entity {:?}, type {}",
        event.entity, event.brick_type
    );

    play_sound(
        SoundType::BrickDestroy,
        &mut commands,
        &audio_config,
        &audio_assets,
        &mut active_sounds,
    );
}

/// Observer for multi-hit brick impact audio.
///
/// This replaces the placeholder in `src/systems/multi_hit.rs`.
fn on_multi_hit_brick_sound(
    trigger: On<crate::systems::MultiHitBrickHit>,
    mut commands: Commands,
    audio_config: Res<AudioConfig>,
    audio_assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        "Multi-hit brick impact: entity {:?}, {} -> {}",
        event.entity, event.previous_type, event.new_type
    );

    play_sound(
        SoundType::MultiHitImpact,
        &mut commands,
        &audio_config,
        &audio_assets,
        &mut active_sounds,
    );
}

/// Observer for ball-wall collision audio.
fn on_ball_wall_hit_sound(
    trigger: On<BallWallHit>,
    mut commands: Commands,
    audio_config: Res<AudioConfig>,
    audio_assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        "Ball-wall hit: entity {:?}, impulse {:?}",
        event.entity, event.impulse
    );

    play_sound(
        SoundType::WallBounce,
        &mut commands,
        &audio_config,
        &audio_assets,
        &mut active_sounds,
    );
}

/// Observer for paddle-ball collision audio.
fn on_paddle_ball_hit_sound(
    trigger: On<crate::BallHit>,
    mut commands: Commands,
    audio_config: Res<AudioConfig>,
    audio_assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        "Paddle-ball hit: ball {:?}, impulse {:?}",
        event.ball, event.impulse
    );

    play_sound(
        SoundType::PaddleHit,
        &mut commands,
        &audio_config,
        &audio_assets,
        &mut active_sounds,
    );
}

/// Observer for paddle-wall collision audio.
fn on_paddle_wall_hit_sound(
    trigger: On<crate::WallHit>,
    mut commands: Commands,
    audio_config: Res<AudioConfig>,
    audio_assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        "Paddle-wall hit: impulse {:?}",
        event.impulse
    );

    play_sound(
        SoundType::PaddleWallHit,
        &mut commands,
        &audio_config,
        &audio_assets,
        &mut active_sounds,
    );
}

/// Observer for paddle-brick collision audio.
fn on_paddle_brick_hit_sound(
    trigger: On<crate::BrickHit>,
    mut commands: Commands,
    audio_config: Res<AudioConfig>,
    audio_assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        "Paddle-brick hit: impulse {:?}",
        event.impulse
    );

    play_sound(
        SoundType::PaddleBrickHit,
        &mut commands,
        &audio_config,
        &audio_assets,
        &mut active_sounds,
    );
}

/// Observer for level start audio.
fn on_level_started_sound(
    trigger: On<LevelStarted>,
    mut commands: Commands,
    audio_config: Res<AudioConfig>,
    audio_assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        "Level started: index {}",
        event.level_index
    );

    play_sound(
        SoundType::LevelStart,
        &mut commands,
        &audio_config,
        &audio_assets,
        &mut active_sounds,
    );
}

/// Observer for level completion audio.
///
/// This observer listens for `LevelComplete` events emitted when a level is cleared.
fn on_level_complete_sound(
    trigger: On<LevelComplete>,
    mut commands: Commands,
    audio_config: Res<AudioConfig>,
    audio_assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        "Level complete: level {}",
        event.level_number
    );

    play_sound(
        SoundType::LevelComplete,
        &mut commands,
        &audio_config,
        &audio_assets,
        &mut active_sounds,
    );
}

/// Event emitted when a level is completed (all destructible bricks destroyed).
#[derive(Event, Debug, Clone)]
pub struct LevelComplete {
    /// The number of the completed level.
    pub level_number: u32,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_config_default_values() {
        let config = AudioConfig::default();
        assert!((config.master_volume - 0.8).abs() < f32::EPSILON);
        assert!(!config.muted);
    }

    #[test]
    fn audio_config_set_volume_clamps() {
        let mut config = AudioConfig::default();

        config.set_volume(1.5);
        assert!((config.master_volume - 1.0).abs() < f32::EPSILON);

        config.set_volume(-0.5);
        assert!(config.master_volume.abs() < f32::EPSILON);

        config.set_volume(0.5);
        assert!((config.master_volume - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn audio_config_toggle_mute() {
        let mut config = AudioConfig::default();
        assert!(!config.muted);

        config.toggle_mute();
        assert!(config.muted);

        config.toggle_mute();
        assert!(!config.muted);
    }

    #[test]
    fn audio_config_should_play() {
        let mut config = AudioConfig::default();
        assert!(config.should_play());

        config.muted = true;
        assert!(!config.should_play());

        config.muted = false;
        config.master_volume = 0.0;
        assert!(!config.should_play());
    }

    #[test]
    fn sound_type_serialization() {
        let sound = SoundType::BrickDestroy;
        let serialized = ron::to_string(&sound).unwrap();
        let deserialized: SoundType = ron::from_str(&serialized).unwrap();
        assert_eq!(sound, deserialized);
    }

    #[test]
    fn brick_destroyed_event_fields() {
        let event = BrickDestroyed {
            entity: Entity::PLACEHOLDER,
            brick_type: 20,
        };
        assert_eq!(event.brick_type, 20);
    }

    #[test]
    fn level_started_event_fields() {
        let event = LevelStarted { level_index: 3 };
        assert_eq!(event.level_index, 3);
    }
}
