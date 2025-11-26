use std::collections::HashSet;
use std::fs;
use std::path::Path;

use brkrs::systems::textures::loader::{ObjectClass, RawTextureManifest, TextureManifest};

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
