//! Audio system event integration tests
//!
//! Tests for audio event types and graceful degradation behavior.

use bevy::prelude::*;
use brkrs::signals::BrickDestroyed;
use brkrs::systems::audio::AudioAssets;
use brkrs::systems::{AudioConfig, AudioPlugin, BallWallHit, LevelCompleted, LevelStarted};

#[test]
fn brick_destroyed_message_has_correct_fields() {
    let event = BrickDestroyed {
        brick_entity: Entity::PLACEHOLDER,
        brick_type: 20,
        destroyed_by: None,
    };
    assert_eq!(event.brick_type, 20);
}

#[test]
fn ball_wall_hit_event_has_correct_fields() {
    let event = BallWallHit {
        ball_entity: Entity::PLACEHOLDER,
        wall_entity: Entity::PLACEHOLDER,
    };
    assert_eq!(event.ball_entity, Entity::PLACEHOLDER);
    assert_eq!(event.wall_entity, Entity::PLACEHOLDER);
}

#[test]
fn level_started_event_has_correct_fields() {
    let event = LevelStarted { level_index: 1 };
    assert_eq!(event.level_index, 1);
}

#[test]
fn level_completed_event_has_correct_fields() {
    let event = LevelCompleted { level_index: 5 };
    assert_eq!(event.level_index, 5);
}

#[test]
fn audio_config_default_is_valid() {
    let config = AudioConfig::default();
    assert!(config.is_valid());
    assert!(!config.muted);
    assert_eq!(config.master_volume, 1.0);
}

#[test]
fn audio_config_volume_clamps_correctly() {
    let mut config = AudioConfig::default();

    // Test upper bound
    config.set_volume(1.5);
    assert_eq!(config.master_volume, 1.0);

    // Test lower bound
    config.set_volume(-0.5);
    assert_eq!(config.master_volume, 0.0);

    // Test mid-range
    config.set_volume(0.5);
    assert_eq!(config.master_volume, 0.5);
}

#[test]
fn audio_config_mute_toggle_works() {
    let mut config = AudioConfig::default();
    assert!(!config.muted);

    let new_state = config.toggle_mute();
    assert!(new_state);
    assert!(config.muted);

    let new_state = config.toggle_mute();
    assert!(!new_state);
    assert!(!config.muted);
}

#[test]
fn audio_events_are_cloneable() {
    let brick_destroyed = BrickDestroyed {
        brick_entity: Entity::PLACEHOLDER,
        brick_type: 20,
        destroyed_by: None,
    };
    let cloned = brick_destroyed;
    assert_eq!(cloned.brick_type, 20);

    let ball_wall_hit = BallWallHit {
        entity: Entity::PLACEHOLDER,
        impulse: Vec3::new(1.0, 2.0, 3.0),
    };
    let cloned = ball_wall_hit.clone();
    assert_eq!(cloned.impulse, Vec3::new(1.0, 2.0, 3.0));

    let level_started = LevelStarted { level_index: 3 };
    let cloned = level_started.clone();
    assert_eq!(cloned.level_index, 3);

    let level_completed = LevelCompleted { level_index: 7 };
    let cloned = level_completed.clone();
    assert_eq!(cloned.level_index, 7);
}

#[test]
fn graceful_degradation_app_initializes_without_audio_assets() {
    // Ensure that initializing the app with the AudioPlugin does not panic
    // when the audio manifest or assets are missing. This mirrors the
    // graceful-degradation behavior specified in the feature plan.
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_plugins(AudioPlugin)
        .init_resource::<AudioConfig>();

    // Run update to execute startup systems that load config/assets
    app.update();

    // AudioAssets resource should exist even if empty
    let assets = app.world().resource::<AudioAssets>();
    // AudioAssets resource should exist even if empty; accessing its fields
    // must be safe and deterministic.
    assert!(assets.sounds.is_empty());
}
