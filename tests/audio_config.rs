//! Audio configuration tests
//!
//! Tests for the AudioConfig resource and related audio system components.

use brkrs::systems::{AudioConfig, SoundType};

#[test]
fn audio_config_default_has_full_volume() {
    let config = AudioConfig::default();
    assert_eq!(config.master_volume, 1.0);
    assert!(!config.muted);
    assert!(config.is_valid());
}

#[test]
fn audio_config_clamps_volume_in_constructor() {
    // Test upper bound
    let config = AudioConfig::new(1.5, false);
    assert_eq!(config.master_volume, 1.0);

    // Test lower bound
    let config = AudioConfig::new(-0.5, false);
    assert_eq!(config.master_volume, 0.0);

    // Test valid range
    let config = AudioConfig::new(0.5, true);
    assert_eq!(config.master_volume, 0.5);
    assert!(config.muted);
}

#[test]
fn audio_config_set_volume_clamps() {
    let mut config = AudioConfig::default();

    config.set_volume(2.0);
    assert_eq!(config.master_volume, 1.0);

    config.set_volume(-1.0);
    assert_eq!(config.master_volume, 0.0);

    config.set_volume(0.75);
    assert_eq!(config.master_volume, 0.75);
}

#[test]
fn audio_config_toggle_mute_returns_new_state() {
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
fn audio_config_is_valid_checks_volume_range() {
    let valid = AudioConfig::new(0.5, false);
    assert!(valid.is_valid());

    let at_zero = AudioConfig::new(0.0, false);
    assert!(at_zero.is_valid());

    let at_one = AudioConfig::new(1.0, false);
    assert!(at_one.is_valid());
}

#[test]
fn sound_type_has_all_expected_variants() {
    // Test that all variants exist and are distinct
    let variants = [
        SoundType::BrickDestroy,
        SoundType::MultiHitImpact,
        SoundType::WallBounce,
        SoundType::PaddleHit,
        SoundType::PaddleWallHit,
        SoundType::PaddleBrickHit,
        SoundType::LevelStart,
        SoundType::LevelComplete,
    ];

    // Verify all 8 variants are present
    assert_eq!(variants.len(), 8);

    // Verify variants are distinct
    for (i, a) in variants.iter().enumerate() {
        for (j, b) in variants.iter().enumerate() {
            if i != j {
                assert_ne!(
                    a, b,
                    "Sound types at index {} and {} should be different",
                    i, j
                );
            }
        }
    }
}

#[test]
fn sound_type_is_hashable() {
    use std::collections::HashMap;

    let mut map: HashMap<SoundType, &str> = HashMap::new();
    map.insert(SoundType::BrickDestroy, "destroy");
    map.insert(SoundType::WallBounce, "bounce");

    assert_eq!(map.get(&SoundType::BrickDestroy), Some(&"destroy"));
    assert_eq!(map.get(&SoundType::WallBounce), Some(&"bounce"));
    assert_eq!(map.get(&SoundType::LevelStart), None);
}
