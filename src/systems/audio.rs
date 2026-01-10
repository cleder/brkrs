//! Audio system module for the brick-breaker game.
//!
//! This module provides a message-driven audio system that plays sounds in response
//! to game events via Constitution-compliant Message boundaries.
//!
//! # Architecture
//!
//! The audio system uses Bevy's Message pattern for clean inter-system communication:
//!
//! - [`AudioPlugin`] registers all audio resources and message consumer systems
//! - [`consume_ui_beep_messages`] reads [`UiBeep`](crate::signals::UiBeep) messages
//! - [`consume_brick_destroyed_messages`] reads [`BrickDestroyed`](crate::signals::BrickDestroyed) messages
//! - [`AudioConfig`] stores user-adjustable volume and mute settings
//! - [`AudioAssets`] holds loaded audio asset handles keyed by [`SoundType`]
//! - [`ActiveSounds`] tracks concurrent playback to limit simultaneous sounds
//!
//! # Message Boundaries (Constitution Compliance)
//!
//! The system follows the Constitution requirement for unified message boundaries:
//!
//! - **UiBeep**: Consumed by [`consume_ui_beep_messages`] system
//!   - Single message path (no observer callbacks)
//!   - Triggered by UI interactions, pause menu, game state changes
//!   - Produces: Short beep audio feedback
//!
//! - **BrickDestroyed**: Consumed by [`consume_brick_destroyed_messages`] system
//!   - Single message path (no dual producer/consumer)
//!   - Fired by collision/despawn systems with destruction context
//!   - Produces: Brick break sound effect based on brick type
//!
//! # Sound Types
//!
//! The system supports 8 distinct sound effects:
//!
//! - `BrickDestroy` - Standard brick destruction
//! - `MultiHitImpact` - Multi-hit brick damage (indices 10-13)
//! - `WallBounce` - Ball bounces off wall
//! - `PaddleHit` - Ball bounces off paddle
//! - `PaddleWallHit` - Paddle collides with wall
//! - `PaddleBrickHit` - Paddle collides with brick
//! - `LevelStart` - Level begins
//! - `LevelComplete` - Level completed
//!
//! # System Organization
//!
//! Systems are organized using the [`AudioSystems`] SystemSet enum:
//! - [`AudioSystems::Startup`]: Load manifest and asset handles
//! - [`AudioSystems::Update`]: Message consumer systems run here
//! - [`AudioSystems::Cleanup`]: Remove stale active sound tracking
//!
//! # Graceful Degradation
//!
//! The system handles missing audio assets gracefully by logging warnings
//! without crashing, allowing development and testing without audio files.
//!
//! # Example
//!
//! ```ignore
//! // Register the audio plugin in your app
//! app.add_plugins(AudioPlugin);
//!
//! // Audio will automatically play when game events occur via messages
//! // Sending a UI beep:
//! fn my_system(mut writer: MessageWriter<UiBeep>) {
//!     writer.write(UiBeep);
//! }
//! ```

use crate::signals::{
    BallWallHit, BrickDestroyed as BrickDestroyedMsg, MerkabaBrickCollision,
    MerkabaPaddleCollision, MerkabaWallCollision, UiBeep,
};
use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use ron::de::from_str;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use web_sys;
/// System set organization for audio-related systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum AudioSystems {
    /// Startup initialization (config, assets)
    Startup,
    /// Regular update consumers and management
    Update,
    /// Cleanup finished sounds / instances
    Cleanup,
}

/// Maximum number of concurrent sounds of the same type.
const MAX_CONCURRENT_SOUNDS: u8 = 4;

/// Identifies the category of sound effect for mapping and concurrent tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoundType {
    /// Standard brick destruction sound.
    BrickDestroy,
    /// Multi-hit brick damage sound (indices 10-13).
    MultiHitImpact,
    /// Ball bounces off wall boundary.
    WallBounce,
    /// Ball bounces off paddle.
    PaddleHit,
    /// Paddle collides with wall boundary.
    PaddleWallHit,
    /// Paddle collides with brick.
    PaddleBrickHit,
    /// Level begins.
    LevelStart,
    /// Level completed.
    LevelComplete,
    /// UI feedback (short soft beep)
    UiBeep,
    /// Merkaba collision with wall.
    MerkabaWall,
    /// Merkaba collision with brick.
    MerkabaBrick,
    /// Merkaba collision with paddle.
    MerkabaPaddle,
    /// Merkaba helicopter blade loop (background).
    MerkabaLoop,
    /// Brick 41 (Extra Life) unique destruction sound.
    Brick41ExtraLife,
}

/// User-adjustable audio settings, persisted across sessions.
///
/// # Fields
///
/// - `master_volume` - Global volume multiplier (0.0 to 1.0)
/// - `muted` - Whether audio is muted
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Global volume multiplier (0.0 to 1.0).
    #[serde(default = "default_volume")]
    pub master_volume: f32,
    /// Whether audio is muted.
    #[serde(default)]
    pub muted: bool,
}

fn default_volume() -> f32 {
    1.0
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            muted: false,
        }
    }
}

impl AudioConfig {
    /// Create a new AudioConfig with the given volume and mute state.
    pub fn new(master_volume: f32, muted: bool) -> Self {
        Self {
            master_volume: master_volume.clamp(0.0, 1.0),
            muted,
        }
    }

    /// Set the master volume (clamped to 0.0-1.0).
    pub fn set_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    /// Toggle mute state and return the new state.
    pub fn toggle_mute(&mut self) -> bool {
        self.muted = !self.muted;
        self.muted
    }

    /// Check if the audio config is valid (volume in range).
    pub fn is_valid(&self) -> bool {
        (0.0..=1.0).contains(&self.master_volume)
    }
}

/// Loaded sound asset handles, keyed by SoundType.
#[derive(Resource, Debug, Default)]
pub struct AudioAssets {
    /// Map of sound types to their loaded audio handles.
    pub sounds: HashMap<SoundType, Handle<AudioSource>>,
}

impl AudioAssets {
    /// Get the audio handle for a specific sound type.
    pub fn get(&self, sound_type: SoundType) -> Option<&Handle<AudioSource>> {
        self.sounds.get(&sound_type)
    }
}

/// Tracks concurrent playback count per sound type.
///
/// Enforces a maximum of 4 concurrent sounds per type to prevent audio distortion.
#[derive(Resource, Debug, Default)]
pub struct ActiveSounds {
    /// Active instances per sound type.
    counts: HashMap<SoundType, u8>,
}

impl ActiveSounds {
    /// Increment the count for a sound type. Returns true if under limit.
    pub fn try_increment(&mut self, sound_type: SoundType) -> bool {
        let count = self.counts.entry(sound_type).or_insert(0);
        if *count < MAX_CONCURRENT_SOUNDS {
            *count += 1;
            true
        } else {
            false
        }
    }

    /// Decrement the count for a sound type.
    pub fn decrement(&mut self, sound_type: SoundType) {
        if let Some(count) = self.counts.get_mut(&sound_type) {
            *count = count.saturating_sub(1);
        }
    }

    /// Get the current count for a sound type.
    pub fn count(&self, sound_type: SoundType) -> u8 {
        *self.counts.get(&sound_type).unwrap_or(&0)
    }
}

/// Tracks active audio entity instances so we can decrement counts when playback ends.
#[derive(Resource, Debug, Default)]
pub struct ActiveAudioInstances {
    /// Map from spawned audio entity -> SoundType
    pub instances: HashMap<Entity, SoundType>,
}

/// Audio manifest for deserializing the audio configuration file.
#[derive(Debug, Deserialize)]
struct AudioManifest {
    sounds: HashMap<SoundType, String>,
}

/// Audio plugin that registers all audio resources and systems.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioAssets>()
            .init_resource::<ActiveSounds>()
            .init_resource::<ActiveAudioInstances>()
            .init_resource::<AudioLoopState>()
            .add_message::<UiBeep>()
            .add_message::<MerkabaWallCollision>()
            .add_message::<MerkabaBrickCollision>()
            .add_systems(Startup, load_audio_config)
            .add_systems(Startup, load_audio_assets)
            .add_systems(Update, save_audio_config_on_change)
            .add_systems(Update, cleanup_finished_sounds)
            .add_observer(on_multi_hit_brick_sound)
            .add_observer(on_paddle_ball_hit_sound)
            .add_observer(on_paddle_wall_hit_sound)
            .add_observer(on_paddle_brick_hit_sound)
            .add_observer(on_ball_wall_hit_sound)
            .add_observer(on_level_started_sound)
            .add_observer(on_level_complete_sound)
            .add_observer(on_merkaba_paddle_collision_sound)
            .add_systems(Update, consume_brick_destroyed_messages)
            .add_systems(Update, consume_ui_beep_messages)
            .add_systems(Update, consume_merkaba_wall_collision_messages)
            .add_systems(Update, consume_merkaba_brick_collision_messages);
    }
}

/// Decrement counts for audio entities that have finished playback.
fn cleanup_finished_sounds(
    mut removed: RemovedComponents<AudioPlayer>,
    mut active_instances: ResMut<ActiveAudioInstances>,
    mut active_sounds: ResMut<ActiveSounds>,
) {
    for removed_entity in removed.read() {
        if let Some(sound_type) = active_instances.instances.remove(&removed_entity) {
            active_sounds.decrement(sound_type);
            debug!(target: "audio", ?sound_type, entity = ?removed_entity, "Audio instance finished, decremented count");
        }
    }
}

// UiBeep buffered message consumed via MessageReader

/// Path to the audio config file.
const AUDIO_CONFIG_PATH: &str = "config/audio.ron";

/// Load audio configuration from disk or use defaults.
fn load_audio_config(mut commands: Commands) {
    #[cfg(not(target_arch = "wasm32"))]
    let config = {
        match std::fs::read_to_string(AUDIO_CONFIG_PATH) {
            Ok(content) => match ron::de::from_str::<AudioConfig>(&content) {
                Ok(mut loaded) => {
                    // Ensure volume is in valid range
                    loaded.master_volume = loaded.master_volume.clamp(0.0, 1.0);
                    info!(
                        target: "audio",
                        volume = loaded.master_volume,
                        muted = loaded.muted,
                        "Loaded audio config"
                    );
                    loaded
                }
                Err(e) => {
                    warn!(
                        target: "audio",
                        error = %e,
                        "Failed to parse audio config, using defaults"
                    );
                    AudioConfig::default()
                }
            },
            Err(_) => {
                info!(
                    target: "audio",
                    "Audio config not found, using defaults"
                );
                AudioConfig::default()
            }
        }
    };

    #[cfg(target_arch = "wasm32")]
    let config = {
        // On WASM, try to load from `localStorage` under the key `brkrs_audio`.
        // We store the serialized RON string in localStorage to keep parity
        // with the native `config/audio.ron` format.
        let storage_key = "brkrs_audio";
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(item)) = storage.get_item(storage_key) {
                    match ron::de::from_str::<AudioConfig>(&item) {
                        Ok(mut loaded) => {
                            loaded.master_volume = loaded.master_volume.clamp(0.0, 1.0);
                            info!(
                                target: "audio",
                                volume = loaded.master_volume,
                                muted = loaded.muted,
                                "Loaded audio config from localStorage"
                            );
                            loaded
                        }
                        Err(e) => {
                            warn!(
                                target: "audio",
                                error = %e,
                                "Failed to parse audio config from localStorage, using defaults"
                            );
                            AudioConfig::default()
                        }
                    }
                } else {
                    info!(target: "audio", "No audio config in localStorage, using defaults");
                    AudioConfig::default()
                }
            } else {
                warn!(target: "audio", "localStorage unavailable, using defaults");
                AudioConfig::default()
            }
        } else {
            warn!(target: "audio", "window object unavailable (WASM), using defaults");
            AudioConfig::default()
        }
    };

    commands.insert_resource(config);
}

/// Save audio configuration when it changes.
fn save_audio_config_on_change(config: Res<AudioConfig>) {
    if !config.is_changed() {
        return;
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Ensure config directory exists
        if let Some(parent) = std::path::Path::new(AUDIO_CONFIG_PATH).parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                warn!(
                    target: "audio",
                    error = %e,
                    "Failed to create config directory"
                );
                return;
            }
        }

        // Serialize and save
        let content = match ron::ser::to_string_pretty(&*config, ron::ser::PrettyConfig::default())
        {
            Ok(s) => s,
            Err(e) => {
                warn!(
                    target: "audio",
                    error = %e,
                    "Failed to serialize audio config"
                );
                return;
            }
        };

        if let Err(e) = std::fs::write(AUDIO_CONFIG_PATH, content) {
            warn!(
                target: "audio",
                error = %e,
                "Failed to save audio config"
            );
        } else {
            debug!(
                target: "audio",
                volume = config.master_volume,
                muted = config.muted,
                "Saved audio config"
            );
        }
    }

    // On WASM, localStorage saving would go here
    #[cfg(target_arch = "wasm32")]
    {
        // Serialize to RON and store in localStorage under `brkrs_audio`.
        let storage_key = "brkrs_audio";
        match ron::ser::to_string_pretty(&*config, ron::ser::PrettyConfig::default()) {
            Ok(serialized) => {
                if let Some(window) = web_sys::window() {
                    match window.local_storage() {
                        Ok(Some(storage)) => {
                            if let Err(e) = storage.set_item(storage_key, &serialized) {
                                warn!(
                                    target: "audio",
                                    error = ?e,
                                    "Failed to save audio config to localStorage"
                                );
                            } else {
                                debug!(
                                    target: "audio",
                                    volume = config.master_volume,
                                    muted = config.muted,
                                    "Saved audio config to localStorage"
                                );
                            }
                        }
                        _ => warn!(target: "audio", "localStorage unavailable, config not saved"),
                    }
                } else {
                    warn!(target: "audio", "window object unavailable, config not saved");
                }
            }
            Err(e) => warn!(
                target: "audio",
                error = %e,
                "Failed to serialize audio config for localStorage"
            ),
        }
    }
}

/// Load audio assets from the manifest file.
/// If an `AssetServer` resource is not available (e.g., in minimal test setups),
/// gracefully skip loading and leave `AudioAssets` empty.
fn load_audio_assets(
    asset_server: Option<Res<AssetServer>>,
    mut audio_assets: ResMut<AudioAssets>,
    audio_sources: Option<Res<Assets<AudioSource>>>,
) {
    // If there's no AssetServer available, skip loading (graceful degradation).
    let asset_server = match asset_server {
        Some(s) => s,
        None => {
            warn!(target: "audio", "AssetServer missing; skipping audio asset loading");
            return;
        }
    };

    // If AudioSource is not registered (e.g. AudioPlugin not added), skip loading
    if audio_sources.is_none() {
        warn!(target: "audio", "AudioSource asset not registered; skipping audio asset loading");
        return;
    }

    // Try to read the manifest file
    #[cfg(not(target_arch = "wasm32"))]
    let manifest_content = std::fs::read_to_string("assets/audio/manifest.ron");
    #[cfg(target_arch = "wasm32")]
    // Use an absolute path based on the crate root so moving this file doesn't
    // break the include. `env!("CARGO_MANIFEST_DIR")` is evaluated at compile
    // time and `concat!` produces a literal suitable for `include_str!`.
    let manifest_content: Result<&str, &str> = Ok(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/audio/manifest.ron"
    )));

    match manifest_content {
        Ok(content) => {
            #[cfg(not(target_arch = "wasm32"))]
            let content_ref = content.as_str();
            #[cfg(target_arch = "wasm32")]
            let content_ref = content;

            match from_str::<AudioManifest>(content_ref) {
                Ok(manifest) => {
                    for (sound_type, file_name) in manifest.sounds {
                        let path = format!("audio/{}", file_name);
                        let handle: Handle<AudioSource> = asset_server.load(&path);
                        audio_assets.sounds.insert(sound_type, handle);
                        debug!(
                            target: "audio",
                            ?sound_type,
                            %path,
                            "Loaded audio asset"
                        );
                    }
                    info!(
                        target: "audio",
                        count = audio_assets.sounds.len(),
                        "Audio assets loaded from manifest"
                    );
                }
                Err(e) => {
                    warn!(
                        target: "audio",
                        error = %e,
                        "Failed to parse audio manifest, audio will be disabled"
                    );
                }
            }
        }
        Err(_e) => {
            warn!(
                target: "audio",
                "Audio manifest not found, audio will be disabled"
            );
        }
    }

    // Ensure merkaba audio handles exist even if the manifest is missing entries.
    // This reuses the placeholder assets added in Phase 1.
    let insert_placeholder = |assets: &mut AudioAssets, ty: SoundType, path: &'static str| {
        assets.sounds.entry(ty).or_insert_with(|| {
            let handle: Handle<AudioSource> = asset_server.load(path);
            handle
        });
    };

    insert_placeholder(
        &mut audio_assets,
        SoundType::MerkabaWall,
        "audio/merkaba_wall.ogg",
    );
    insert_placeholder(
        &mut audio_assets,
        SoundType::MerkabaBrick,
        "audio/merkaba_brick.ogg",
    );
    insert_placeholder(
        &mut audio_assets,
        SoundType::MerkabaPaddle,
        "audio/merkaba_paddle.ogg",
    );
    insert_placeholder(
        &mut audio_assets,
        SoundType::MerkabaLoop,
        "audio/merkaba_loop_helicopter.ogg",
    );
}

/// Tracks the helicopter loop state for merkaba hazards.
#[derive(Resource, Default, Debug)]
pub struct AudioLoopState {
    /// Handle to the loop audio asset (merkaba blades)
    pub loop_handle: Option<Handle<AudioSource>>,
    /// Active loop audio entity (if playing)
    pub active_entity: Option<Entity>,
    /// Whether the loop is currently playing
    pub is_playing: bool,
}

/// Play a sound of the given type, respecting volume, mute, and concurrent limits.
fn play_sound(
    sound_type: SoundType,
    config: &AudioConfig,
    assets: &AudioAssets,
    audio_sources: Option<&Assets<AudioSource>>,
    active_sounds: &mut ActiveSounds,
    active_instances: &mut ActiveAudioInstances,
    commands: &mut Commands,
) {
    // Check if muted
    if config.muted {
        return;
    }

    // Check if volume is effectively zero
    if config.master_volume <= 0.0 {
        return;
    }

    // Check concurrent limit
    if !active_sounds.try_increment(sound_type) {
        debug!(
            target: "audio",
            ?sound_type,
            "Dropped sound: concurrent limit reached"
        );
        return;
    }

    // Get the audio handle
    let Some(handle) = assets.get(sound_type) else {
        warn!(
            target: "audio",
            ?sound_type,
            "Audio asset missing"
        );
        active_sounds.decrement(sound_type);
        return;
    };

    // Check if the asset is actually loaded
    if let Some(audio_assets) = audio_sources {
        if !audio_assets.contains(handle) {
            debug!(
                target: "audio",
                ?sound_type,
                "Audio asset not yet loaded, skipping playback"
            );
            active_sounds.decrement(sound_type);
            return;
        }
    }

    // Spawn the audio player and record the spawned entity so we can
    // decrement the concurrent-count when playback finishes (entity despawn).
    let entity = commands
        .spawn((
            AudioPlayer::new(handle.clone()),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: bevy::audio::Volume::Linear(config.master_volume),
                ..default()
            },
        ))
        .id();

    // Register the spawned entity so we can detect when it is removed
    // (playback finished or entity despawned) and decrement the count.
    active_instances.instances.insert(entity, sound_type);

    debug!(
        target: "audio",
        ?sound_type,
        volume = config.master_volume,
        "Playing sound"
    );
}

// =============================================================================
// Event definitions
// =============================================================================

// BrickDestroyed moved to `crate::signals` and is now a Message.

// BallWallHit event is now defined in signals.rs and imported above

/// Emitted when a level has finished loading and is ready for play.
/// Used by audio system to play level start sound.
#[derive(Event, Debug, Clone)]
pub struct LevelStarted {
    /// Index of the level that started.
    pub level_index: u32,
}

/// Emitted when a level has been completed (all destructible bricks destroyed).
/// Used by audio system to play level complete sound.
#[derive(Event, Debug, Clone)]
pub struct LevelCompleted {
    /// Index of the level that was completed.
    pub level_index: u32,
}

// =============================================================================
// Audio observers
// =============================================================================

/// Observer for multi-hit brick impact sound.
fn on_multi_hit_brick_sound(
    trigger: On<crate::systems::multi_hit::MultiHitBrickHit>,
    config: Res<AudioConfig>,
    assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
    mut active_instances: ResMut<ActiveAudioInstances>,
    mut commands: Commands,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        entity = ?event.entity,
        previous_type = event.previous_type,
        new_type = event.new_type,
        "Multi-hit brick impact"
    );
    play_sound(
        SoundType::MultiHitImpact,
        &config,
        &assets,
        None,
        &mut active_sounds,
        &mut active_instances,
        &mut commands,
    );
}

/// Consumer for brick destruction messages to play destruction sound.
fn consume_brick_destroyed_messages(
    reader: Option<MessageReader<BrickDestroyedMsg>>,
    config: Option<Res<AudioConfig>>,
    assets: Option<Res<AudioAssets>>,
    active_sounds: Option<ResMut<ActiveSounds>>,
    active_instances: Option<ResMut<ActiveAudioInstances>>,
    mut brick41_available: Local<Option<bool>>,
    mut commands: Commands,
) {
    let Some(mut reader) = reader else {
        return;
    };
    let Some(config) = config else {
        return;
    };
    let Some(assets) = assets else {
        return;
    };
    let Some(mut active_sounds) = active_sounds else {
        return;
    };
    let Some(mut active_instances) = active_instances else {
        return;
    };

    for event in reader.read() {
        // Don't play destruction sound for multi-hit bricks (they use MultiHitImpact)
        if crate::level_format::is_multi_hit_brick(event.brick_type) {
            continue;
        }
        debug!(
            target: "audio",
            entity = ?event.brick_entity,
            brick_type = event.brick_type,
            "Brick destroyed"
        );

        // Brick 41 (Extra Life) has unique sound; cache availability to avoid repeated lookups/logs
        if brick41_available.is_none() {
            let available = assets.get(SoundType::Brick41ExtraLife).is_some();
            if !available {
                warn!(
                    target: "audio",
                    "Brick 41 unique sound handle missing; falling back to generic BrickDestroy"
                );
            }
            *brick41_available = Some(available);
        }

        let brick41_sound_available = brick41_available.unwrap_or(false);

        let sound_type = if event.brick_type == crate::level_format::EXTRA_LIFE_BRICK
            && brick41_sound_available
        {
            SoundType::Brick41ExtraLife
        } else {
            SoundType::BrickDestroy
        };

        play_sound(
            sound_type,
            &config,
            &assets,
            None,
            &mut active_sounds,
            &mut active_instances,
            &mut commands,
        );
    }
}

/// Observer for ball wall hit sound.
pub(crate) fn on_ball_wall_hit_sound(
    trigger: On<BallWallHit>,
    config: Res<AudioConfig>,
    assets: Res<AudioAssets>,
    audio_sources: Option<Res<Assets<AudioSource>>>,
    mut active_sounds: ResMut<ActiveSounds>,
    mut active_instances: ResMut<ActiveAudioInstances>,
    mut commands: Commands,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        ball_entity = ?event.ball_entity,
        wall_entity = ?event.wall_entity,
        "Ball wall hit"
    );
    play_sound(
        SoundType::WallBounce,
        &config,
        &assets,
        audio_sources.as_deref(),
        &mut active_sounds,
        &mut active_instances,
        &mut commands,
    );
}

/// Observer for paddle-ball hit sound (ball bounces off paddle).
fn on_paddle_ball_hit_sound(
    trigger: On<crate::BallHit>,
    config: Res<AudioConfig>,
    assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
    mut active_instances: ResMut<ActiveAudioInstances>,
    mut commands: Commands,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        ball = ?event.ball,
        impulse = ?event.impulse,
        "Paddle-ball hit"
    );
    play_sound(
        SoundType::PaddleHit,
        &config,
        &assets,
        None,
        &mut active_sounds,
        &mut active_instances,
        &mut commands,
    );
}

/// Observer for paddle-wall hit sound.
fn on_paddle_wall_hit_sound(
    trigger: On<crate::WallHit>,
    config: Res<AudioConfig>,
    assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
    mut active_instances: ResMut<ActiveAudioInstances>,
    mut commands: Commands,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        impulse = ?event.impulse,
        "Paddle-wall hit"
    );
    play_sound(
        SoundType::PaddleWallHit,
        &config,
        &assets,
        None,
        &mut active_sounds,
        &mut active_instances,
        &mut commands,
    );
}

#[cfg(test)]
mod brick41_audio_tests {
    use super::*;
    use bevy::{audio::AudioSource, MinimalPlugins};

    fn make_handle() -> Handle<AudioSource> {
        Handle::default()
    }

    fn setup_app(include_brick41: bool) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_message::<BrickDestroyedMsg>();
        app.insert_resource(AudioConfig::default());

        let mut audio_assets = AudioAssets::default();
        audio_assets
            .sounds
            .insert(SoundType::BrickDestroy, make_handle());
        if include_brick41 {
            audio_assets
                .sounds
                .insert(SoundType::Brick41ExtraLife, make_handle());
        }
        app.insert_resource(audio_assets);
        app.insert_resource(ActiveSounds::default());
        app.insert_resource(ActiveAudioInstances::default());

        app.add_systems(Update, consume_brick_destroyed_messages);
        app
    }

    #[test]
    fn brick_41_uses_unique_sound_when_available() {
        let mut app = setup_app(true);

        let brick = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(BrickDestroyedMsg {
            brick_entity: brick,
            brick_type: crate::level_format::EXTRA_LIFE_BRICK,
            destroyed_by: None,
        });

        app.update();

        let counts = app.world().resource::<ActiveSounds>();
        assert_eq!(
            1,
            counts.count(SoundType::Brick41ExtraLife),
            "Brick 41 should play its unique sound when available"
        );
        assert_eq!(
            0,
            counts.count(SoundType::BrickDestroy),
            "Brick 41 should not fall back when unique sound exists"
        );
    }

    #[test]
    fn brick_41_falls_back_when_unique_sound_missing() {
        let mut app = setup_app(false);

        let brick = app.world_mut().spawn_empty().id();
        app.world_mut().write_message(BrickDestroyedMsg {
            brick_entity: brick,
            brick_type: crate::level_format::EXTRA_LIFE_BRICK,
            destroyed_by: None,
        });

        app.update();

        let counts = app.world().resource::<ActiveSounds>();
        assert_eq!(
            0,
            counts.count(SoundType::Brick41ExtraLife),
            "Brick 41 should not attempt unique sound when handle is missing"
        );
        assert_eq!(
            1,
            counts.count(SoundType::BrickDestroy),
            "Brick 41 should fall back to generic destroy sound when unique handle missing"
        );
    }
}

/// Observer for paddle-brick hit sound.
fn on_paddle_brick_hit_sound(
    trigger: On<crate::BrickHit>,
    config: Res<AudioConfig>,
    assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
    mut active_instances: ResMut<ActiveAudioInstances>,
    mut commands: Commands,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        impulse = ?event.impulse,
        "Paddle-brick hit"
    );
    play_sound(
        SoundType::PaddleBrickHit,
        &config,
        &assets,
        None,
        &mut active_sounds,
        &mut active_instances,
        &mut commands,
    );
}

/// Observer for level started sound.
fn on_level_started_sound(
    trigger: On<LevelStarted>,
    config: Res<AudioConfig>,
    assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
    mut active_instances: ResMut<ActiveAudioInstances>,
    mut commands: Commands,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        level_index = event.level_index,
        "Level started"
    );
    play_sound(
        SoundType::LevelStart,
        &config,
        &assets,
        None,
        &mut active_sounds,
        &mut active_instances,
        &mut commands,
    );
}

/// Observer for level complete sound.
fn on_level_complete_sound(
    trigger: On<LevelCompleted>,
    config: Res<AudioConfig>,
    assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
    mut active_instances: ResMut<ActiveAudioInstances>,
    mut commands: Commands,
) {
    let event = trigger.event();
    debug!(
        target: "audio",
        level_index = event.level_index,
        "Level completed"
    );
    play_sound(
        SoundType::LevelComplete,
        &config,
        &assets,
        None,
        &mut active_sounds,
        &mut active_instances,
        &mut commands,
    );
}

/// UI beep observer - plays a short soft beep when requested
fn consume_ui_beep_messages(
    reader: Option<MessageReader<UiBeep>>,
    config: Option<Res<AudioConfig>>,
    assets: Option<Res<AudioAssets>>,
    active_sounds: Option<ResMut<ActiveSounds>>,
    active_instances: Option<ResMut<ActiveAudioInstances>>,
    mut commands: Commands,
) {
    let Some(mut reader) = reader else { return };
    let Some(config) = config else { return };
    let Some(assets) = assets else { return };
    let Some(mut active_sounds) = active_sounds else {
        return;
    };
    let Some(mut active_instances) = active_instances else {
        return;
    };

    let mut count = 0u32;
    for _ in reader.read() {
        count += 1;
        play_sound(
            SoundType::UiBeep,
            &config,
            &assets,
            None,
            &mut active_sounds,
            &mut active_instances,
            &mut commands,
        );
    }
    if count > 0 {
        debug!(target: "audio", messages = count, "UI beep messages consumed");
    }
}

/// Message consumer for merkaba-wall collision sounds.
fn consume_merkaba_wall_collision_messages(
    reader: Option<MessageReader<MerkabaWallCollision>>,
    config: Option<Res<AudioConfig>>,
    assets: Option<Res<AudioAssets>>,
    active_sounds: Option<ResMut<ActiveSounds>>,
    active_instances: Option<ResMut<ActiveAudioInstances>>,
    mut commands: Commands,
) {
    let Some(mut reader) = reader else { return };
    let Some(config) = config else { return };
    let Some(assets) = assets else { return };
    let Some(mut active_sounds) = active_sounds else {
        return;
    };
    let Some(mut active_instances) = active_instances else {
        return;
    };

    for event in reader.read() {
        debug!(
            target: "audio",
            merkaba_entity = ?event.merkaba_entity,
            wall_entity = ?event.wall_entity,
            "Merkaba wall collision"
        );
        play_sound(
            SoundType::MerkabaWall,
            &config,
            &assets,
            None,
            &mut active_sounds,
            &mut active_instances,
            &mut commands,
        );
    }
}

/// Message consumer for merkaba-brick collision sounds.
fn consume_merkaba_brick_collision_messages(
    reader: Option<MessageReader<MerkabaBrickCollision>>,
    config: Option<Res<AudioConfig>>,
    assets: Option<Res<AudioAssets>>,
    active_sounds: Option<ResMut<ActiveSounds>>,
    active_instances: Option<ResMut<ActiveAudioInstances>>,
    mut commands: Commands,
) {
    let Some(mut reader) = reader else { return };
    let Some(config) = config else { return };
    let Some(assets) = assets else { return };
    let Some(mut active_sounds) = active_sounds else {
        return;
    };
    let Some(mut active_instances) = active_instances else {
        return;
    };

    for event in reader.read() {
        debug!(
            target: "audio",
            merkaba_entity = ?event.merkaba_entity,
            brick_entity = ?event.brick_entity,
            "Merkaba brick collision"
        );
        play_sound(
            SoundType::MerkabaBrick,
            &config,
            &assets,
            None,
            &mut active_sounds,
            &mut active_instances,
            &mut commands,
        );
    }
}

/// Observer for merkaba-paddle collision sounds.
fn on_merkaba_paddle_collision_sound(
    trigger: On<MerkabaPaddleCollision>,
    config: Res<AudioConfig>,
    assets: Res<AudioAssets>,
    mut active_sounds: ResMut<ActiveSounds>,
    mut active_instances: ResMut<ActiveAudioInstances>,
    mut commands: Commands,
) {
    debug!(
        target: "audio",
        merkaba_entity = ?trigger.event().merkaba_entity,
        paddle_entity = ?trigger.event().paddle_entity,
        "Merkaba paddle collision"
    );
    play_sound(
        SoundType::MerkabaPaddle,
        &config,
        &assets,
        None,
        &mut active_sounds,
        &mut active_instances,
        &mut commands,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_config_default_values() {
        let config = AudioConfig::default();
        assert_eq!(config.master_volume, 1.0);
        assert!(!config.muted);
        assert!(config.is_valid());
    }

    #[test]
    fn audio_config_new_clamps_volume() {
        let config = AudioConfig::new(1.5, false);
        assert_eq!(config.master_volume, 1.0);

        let config = AudioConfig::new(-0.5, false);
        assert_eq!(config.master_volume, 0.0);
    }

    #[test]
    fn audio_config_set_volume_clamps() {
        let mut config = AudioConfig::default();
        config.set_volume(2.0);
        assert_eq!(config.master_volume, 1.0);

        config.set_volume(-1.0);
        assert_eq!(config.master_volume, 0.0);

        config.set_volume(0.5);
        assert_eq!(config.master_volume, 0.5);
    }

    #[test]
    fn audio_config_toggle_mute() {
        let mut config = AudioConfig::default();
        assert!(!config.muted);

        let muted = config.toggle_mute();
        assert!(muted);
        assert!(config.muted);

        let muted = config.toggle_mute();
        assert!(!muted);
        assert!(!config.muted);
    }

    #[test]
    fn active_sounds_respects_limit() {
        let mut active = ActiveSounds::default();

        // First 4 should succeed
        for _ in 0..4 {
            assert!(active.try_increment(SoundType::BrickDestroy));
        }

        // 5th should fail
        assert!(!active.try_increment(SoundType::BrickDestroy));
        assert_eq!(active.count(SoundType::BrickDestroy), 4);

        // Decrement and try again
        active.decrement(SoundType::BrickDestroy);
        assert_eq!(active.count(SoundType::BrickDestroy), 3);
        assert!(active.try_increment(SoundType::BrickDestroy));
    }

    #[test]
    fn active_sounds_tracks_types_independently() {
        let mut active = ActiveSounds::default();

        // Fill up one type
        for _ in 0..4 {
            assert!(active.try_increment(SoundType::BrickDestroy));
        }

        // Other type should still work
        assert!(active.try_increment(SoundType::WallBounce));
        assert_eq!(active.count(SoundType::WallBounce), 1);
    }

    #[test]
    fn brick_destroyed_message_fields() {
        let event = BrickDestroyedMsg {
            brick_entity: Entity::PLACEHOLDER,
            brick_type: 20,
            destroyed_by: None,
        };
        assert_eq!(event.brick_type, 20);
    }

    #[test]
    fn ball_wall_hit_event_fields() {
        let event = BallWallHit {
            ball_entity: Entity::PLACEHOLDER,
            wall_entity: Entity::PLACEHOLDER,
        };
        assert_eq!(event.ball_entity, Entity::PLACEHOLDER);
        assert_eq!(event.wall_entity, Entity::PLACEHOLDER);
    }

    #[test]
    fn level_started_event_fields() {
        let event = LevelStarted { level_index: 5 };
        assert_eq!(event.level_index, 5);
    }

    #[test]
    fn level_completed_event_fields() {
        let event = LevelCompleted { level_index: 3 };
        assert_eq!(event.level_index, 3);
    }
}
