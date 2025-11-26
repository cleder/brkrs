use std::collections::HashSet;
use std::fs;
use std::path::Path;

use bevy::prelude::Color;

use brkrs::systems::textures::loader::{
    LevelSwitchState, LevelTextureSet, ObjectClass, RawTextureManifest, TextureManifest,
};
use brkrs::systems::textures::TextureManifestContract;

fn load_manifest_from_disk() -> RawTextureManifest {
    let path = Path::new("assets/textures/manifest.ron");
    let contents =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {:?}: {}", path, err));
    ron::de::from_str(&contents).unwrap_or_else(|err| panic!("failed to parse {:?}: {}", path, err))
}

#[test]
fn manifest_contains_required_profiles_and_fallbacks() {
    let raw = load_manifest_from_disk();
    let profile_ids: HashSet<_> = raw
        .profiles
        .iter()
        .map(|profile| profile.id.as_str())
        .collect();
    for required in [
        "ball/default",
        "ball/fallback",
        "paddle/default",
        "brick/default",
        "sidewall/default",
        "ground/default",
        "background/default",
    ] {
        assert!(
            profile_ids.contains(required),
            "manifest missing required profile {required}"
        );
    }

    let ball_default = raw
        .profiles
        .iter()
        .find(|profile| profile.id == "ball/default")
        .expect("ball/default profile not found");
    assert_eq!(ball_default.fallback_chain, [String::from("ball/fallback")]);

    let sidewall_default = raw
        .profiles
        .iter()
        .find(|profile| profile.id == "sidewall/default")
        .expect("sidewall/default profile not found");
    assert!(
        sidewall_default
            .fallback_chain
            .iter()
            .all(|fallback| profile_ids.contains(fallback.as_str())),
        "every fallback entry must reference a valid profile"
    );
}

#[test]
fn runtime_manifest_indexes_profiles_and_variants() {
    let raw = load_manifest_from_disk();

    let manifest = TextureManifest::from_raw(raw.clone());

    assert_eq!(manifest.profiles.len(), raw.profiles.len());
    assert_eq!(manifest.type_variants.len(), raw.type_variants.len());
    assert!(manifest.level_overrides.is_empty());

    let ball_profile = manifest
        .profiles
        .get("ball/default")
        .expect("ball/default profile should be indexed by id");
    assert_eq!(ball_profile.albedo_path, "fallback/ball_base.png");

    let resolved_variant = manifest
        .type_variants
        .iter()
        .find(|variant| variant.object_class == ObjectClass::Ball && variant.type_id == 0)
        .expect("expected a ball/default variant entry");
    assert_eq!(resolved_variant.profile_id, "ball/default");
}

#[test]
fn contract_view_matches_runtime_manifest() {
    let mut raw = load_manifest_from_disk();
    raw.level_overrides = vec![LevelTextureSet {
        level_number: 42,
        ground_profile: Some(String::from("ground/default")),
        background_profile: Some(String::from("background/default")),
        sidewall_profile: None,
        tint: Some(Color::srgba(0.25, 0.5, 0.75, 1.0)),
        notes: Some(String::from("test entry")),
    }];
    raw.level_switch = Some(LevelSwitchState {
        ordered_levels: vec![1, 42],
        current_index: 0,
        pending_switch: false,
    });
    if let Some(first_variant) = raw.type_variants.first_mut() {
        first_variant.emissive_color = Some(Color::srgba(1.0, 0.0, 0.0, 1.0));
    }

    let manifest = TextureManifest::from_raw(raw);
    let contract = TextureManifestContract::from(&manifest);

    assert_eq!(contract.profiles.len(), manifest.profiles.len());
    assert_eq!(contract.type_variants.len(), manifest.type_variants.len());
    assert_eq!(contract.level_overrides.len(), 1);
    let override_entry = &contract.level_overrides[0];
    assert_eq!(override_entry.level_number, 42);
    assert_eq!(
        override_entry.ground_profile.as_deref(),
        Some("ground/default")
    );
    assert!(override_entry.tint.is_some());

    let level_switch = contract
        .level_switch
        .expect("level switch should be present");
    assert_eq!(level_switch.ordered_levels, vec![1, 42]);
    assert!(!level_switch.pending_switch);

    let variant = &contract.type_variants[0];
    assert!(variant.emissive_color.is_some());
}
