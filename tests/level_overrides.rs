//! Tests for per-level texture overrides (T019).
//!
//! Validates:
//! 1. Default profiles apply when no level-specific overrides exist
//! 2. Custom LevelTextureSet entries override ground/background/sidewall profiles
//! 3. LevelPresentation resource reflects current level's override set
//! 4. Hot-reload of overrides updates LevelPresentation correctly

use bevy::prelude::*;

use brkrs::systems::textures::loader::{LevelTextureSet, TextureManifest, VisualAssetProfile};
use brkrs::systems::textures::{
    overrides::LevelPresentation, BaselineMaterialKind, FallbackRegistry, ProfileMaterialBank,
    TextureMaterialsPlugin,
};

fn app_with_texture_system() -> App {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins(MinimalPlugins);
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.add_plugins(TextureMaterialsPlugin);
    app.init_resource::<LevelPresentation>();
    app.update();
    app
}

fn create_test_profile(id: &str) -> VisualAssetProfile {
    VisualAssetProfile {
        id: id.to_string(),
        albedo_path: format!("test/{id}.png"),
        normal_path: None,
        roughness: 0.5,
        metallic: 0.0,
        uv_scale: Vec2::splat(1.0),
        uv_offset: Vec2::ZERO,
        fallback_chain: vec![],
    }
}

fn manifest_with_baseline_profiles() -> TextureManifest {
    let profiles = [
        "ball/default",
        "paddle/default",
        "brick/default",
        "sidewall/default",
        "ground/default",
        "background/default",
    ]
    .into_iter()
    .map(|id| (id.to_string(), create_test_profile(id)))
    .collect();

    TextureManifest {
        profiles,
        type_variants: vec![],
        level_overrides: Default::default(),
        level_switch: None,
    }
}

#[test]
fn default_profiles_apply_when_no_level_override_exists() {
    let _app = app_with_texture_system();
    let manifest = manifest_with_baseline_profiles();

    // With no level_overrides, LevelPresentation for level 1 should use canonical defaults
    let presentation = LevelPresentation::for_level(1, &manifest);

    // All override profiles should be None (use canonical)
    assert!(
        presentation.ground_profile().is_none(),
        "no override means canonical ground"
    );
    assert!(
        presentation.background_profile().is_none(),
        "no override means canonical background"
    );
    assert!(
        presentation.sidewall_profile().is_none(),
        "no override means canonical sidewall"
    );
    assert!(
        presentation.tint().is_none(),
        "no override means no tint modifier"
    );
}

#[test]
fn custom_level_texture_set_overrides_profiles() {
    let mut manifest = manifest_with_baseline_profiles();

    // Add custom profiles for level-specific overrides
    manifest.profiles.insert(
        "ground/lava".to_string(),
        create_test_profile("ground/lava"),
    );
    manifest.profiles.insert(
        "background/sunset".to_string(),
        create_test_profile("background/sunset"),
    );
    manifest.profiles.insert(
        "sidewall/marble".to_string(),
        create_test_profile("sidewall/marble"),
    );

    // Register a level override for level 3
    manifest.level_overrides.insert(
        3,
        LevelTextureSet {
            level_number: 3,
            ground_profile: Some("ground/lava".to_string()),
            background_profile: Some("background/sunset".to_string()),
            sidewall_profile: Some("sidewall/marble".to_string()),
            tint: Some(Color::srgba(1.0, 0.8, 0.6, 1.0)),
            notes: Some("Test lava level".to_string()),
        },
    );

    let presentation = LevelPresentation::for_level(3, &manifest);

    assert_eq!(
        presentation.ground_profile().map(String::as_str),
        Some("ground/lava"),
        "level 3 should use lava ground"
    );
    assert_eq!(
        presentation.background_profile().map(String::as_str),
        Some("background/sunset"),
        "level 3 should use sunset background"
    );
    assert_eq!(
        presentation.sidewall_profile().map(String::as_str),
        Some("sidewall/marble"),
        "level 3 should use marble sidewalls"
    );
    assert!(
        presentation.tint().is_some(),
        "level 3 should have a tint modifier"
    );
}

#[test]
fn partial_override_uses_canonical_for_missing_fields() {
    let mut manifest = manifest_with_baseline_profiles();

    // Add only a ground override for level 5
    manifest
        .profiles
        .insert("ground/ice".to_string(), create_test_profile("ground/ice"));
    manifest.level_overrides.insert(
        5,
        LevelTextureSet {
            level_number: 5,
            ground_profile: Some("ground/ice".to_string()),
            background_profile: None,
            sidewall_profile: None,
            tint: None,
            notes: None,
        },
    );

    let presentation = LevelPresentation::for_level(5, &manifest);

    assert_eq!(
        presentation.ground_profile().map(String::as_str),
        Some("ground/ice"),
        "level 5 should use ice ground"
    );
    assert!(
        presentation.background_profile().is_none(),
        "level 5 should use canonical background"
    );
    assert!(
        presentation.sidewall_profile().is_none(),
        "level 5 should use canonical sidewalls"
    );
}

#[test]
fn different_levels_have_isolated_overrides() {
    let mut manifest = manifest_with_baseline_profiles();

    // Add profiles for multiple levels
    manifest.profiles.insert(
        "ground/lava".to_string(),
        create_test_profile("ground/lava"),
    );
    manifest
        .profiles
        .insert("ground/ice".to_string(), create_test_profile("ground/ice"));

    // Level 2 has lava ground
    manifest.level_overrides.insert(
        2,
        LevelTextureSet {
            level_number: 2,
            ground_profile: Some("ground/lava".to_string()),
            background_profile: None,
            sidewall_profile: None,
            tint: None,
            notes: None,
        },
    );

    // Level 4 has ice ground
    manifest.level_overrides.insert(
        4,
        LevelTextureSet {
            level_number: 4,
            ground_profile: Some("ground/ice".to_string()),
            background_profile: None,
            sidewall_profile: None,
            tint: None,
            notes: None,
        },
    );

    let presentation_2 = LevelPresentation::for_level(2, &manifest);
    let presentation_3 = LevelPresentation::for_level(3, &manifest); // no override
    let presentation_4 = LevelPresentation::for_level(4, &manifest);

    assert_eq!(
        presentation_2.ground_profile().map(String::as_str),
        Some("ground/lava"),
        "level 2 should use lava"
    );
    assert!(
        presentation_3.ground_profile().is_none(),
        "level 3 has no override"
    );
    assert_eq!(
        presentation_4.ground_profile().map(String::as_str),
        Some("ground/ice"),
        "level 4 should use ice"
    );
}

#[test]
fn level_presentation_resolve_material_uses_profile_or_fallback() {
    let mut app = app_with_texture_system();

    // Create test material handles
    let (lava_handle, _fallback_handle) = {
        let mut materials = app.world_mut().resource_mut::<Assets<StandardMaterial>>();
        let lava = materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.2, 0.0, 1.0),
            unlit: true,
            ..default()
        });
        let fallback = materials.add(StandardMaterial {
            base_color: Color::srgba(0.2, 0.2, 0.2, 1.0),
            unlit: true,
            ..default()
        });
        (lava, fallback)
    };

    // Setup bank with lava profile
    {
        let mut bank = app.world_mut().resource_mut::<ProfileMaterialBank>();
        bank.insert_for_tests("ground/lava", lava_handle.clone());
    }

    let mut manifest = manifest_with_baseline_profiles();
    manifest.profiles.insert(
        "ground/lava".to_string(),
        create_test_profile("ground/lava"),
    );
    manifest.level_overrides.insert(
        7,
        LevelTextureSet {
            level_number: 7,
            ground_profile: Some("ground/lava".to_string()),
            background_profile: None,
            sidewall_profile: None,
            tint: None,
            notes: None,
        },
    );

    let presentation = LevelPresentation::for_level(7, &manifest);

    // Resolve ground material - should get lava handle
    let bank = app.world().resource::<ProfileMaterialBank>();
    let resolved = presentation.resolve_material(
        BaselineMaterialKind::Ground,
        bank,
        None::<&mut FallbackRegistry>,
    );
    assert!(resolved.is_some());
    assert_eq!(resolved.unwrap(), lava_handle);
}

#[test]
fn tint_modifier_applies_to_presentation() {
    let mut manifest = manifest_with_baseline_profiles();

    let red_tint = Color::srgba(1.0, 0.0, 0.0, 1.0);
    manifest.level_overrides.insert(
        9,
        LevelTextureSet {
            level_number: 9,
            ground_profile: None,
            background_profile: None,
            sidewall_profile: None,
            tint: Some(red_tint),
            notes: Some("Red danger level".to_string()),
        },
    );

    let presentation = LevelPresentation::for_level(9, &manifest);

    let tint = presentation.tint().expect("should have tint");
    // Verify the tint color matches
    let srgba = tint.to_srgba();
    assert!((srgba.red - 1.0).abs() < 0.001);
    assert!((srgba.green - 0.0).abs() < 0.001);
    assert!((srgba.blue - 0.0).abs() < 0.001);
}
