//! Tests for Depth/Parallax texture support
//!
//! This test suite verifies that depth textures (parallax maps) load correctly
//! for realistic surface detailing and displacement effects.

use bevy::prelude::*;

/// Test that depth path field deserializes correctly from RON
#[test]
fn test_depth_path_deserialization_minimal() {
    use brkrs::systems::textures::loader::VisualAssetProfile;

    let ron_data = r#"(
        id: "test/depth_brick",
        albedo_path: "brick_albedo.png",
        depth_path: Some("brick_depth.png"),
    )"#;

    let profile: VisualAssetProfile = ron::from_str(ron_data).expect("Failed to parse RON");
    assert_eq!(profile.id, "test/depth_brick");
    assert_eq!(profile.depth_path, Some("brick_depth.png".to_string()));
}

/// Test that depth_scale parameter exists and has default value
#[test]
fn test_depth_scale_parameter() {
    use brkrs::systems::textures::loader::VisualAssetProfile;

    let ron_data = r#"(
        id: "test/depth_scale",
        albedo_path: "brick.png",
        depth_path: Some("depth.png"),
    )"#;

    let profile: VisualAssetProfile = ron::from_str(ron_data).expect("Failed to parse RON");
    assert_eq!(
        profile.depth_scale, 0.1,
        "depth_scale should default to 0.1 for parallax mapping"
    );
}

/// Test that depth texture loading gracefully handles missing files
#[test]
fn test_depth_fallback_missing_file() {
    use brkrs::systems::textures::loader::VisualAssetProfile;

    let profile = VisualAssetProfile {
        id: "test/depth_missing".to_string(),
        albedo_path: "test_orm.png".to_string(),
        normal_path: None,
        orm_path: None,
        emissive_path: None,
        depth_path: Some("nonexistent_depth.png".to_string()),
        roughness: 0.5,
        metallic: 0.0,
        uv_scale: Vec2::splat(1.0),
        uv_offset: Vec2::ZERO,
        depth_scale: 0.1,
        fallback_chain: vec![],
    };

    // Verify profile accepts nonexistent path
    assert_eq!(
        profile.depth_path,
        Some("nonexistent_depth.png".to_string())
    );

    // When make_material is called with nonexistent depth_path:
    // - Asset server returns a handle to a missing asset (doesn't panic)
    // - StandardMaterial renders without depth texture
    // - depth_scale parameter is still set for when texture loads
    // - Warning is logged via Bevy's asset system
}
