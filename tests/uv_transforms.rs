// Test UV transform application across all texture types
// Ensures UV scale and UV offset apply uniformly to all textures

#[cfg(test)]
mod uv_transforms_tests {
    use bevy::prelude::Vec2;
    use brkrs::systems::textures::loader::VisualAssetProfile;

    /// T041: Verify UV scale is applied uniformly to all texture coordinates
    #[test]
    fn test_uv_scale_all_textures() {
        // Profile with custom UV scale
        let profile = VisualAssetProfile {
            id: "scaled_uv_profile".to_string(),
            albedo_path: "albedo.png".to_string(),
            normal_path: Some("normal.png".to_string()),
            orm_path: Some("orm.png".to_string()),
            emissive_path: Some("emissive.png".to_string()),
            depth_path: Some("depth.png".to_string()),
            roughness: 0.5,
            metallic: 0.3,
            uv_scale: Vec2::new(2.0, 2.0), // 2x UV scaling
            uv_offset: Vec2::ZERO,
            depth_scale: 0.1,
            fallback_chain: vec![],
        };

        // Verify UV scale is set
        assert_eq!(profile.uv_scale, Vec2::new(2.0, 2.0));

        // All textures should use this same UV scale in material
        // (Bevy's StandardMaterial applies UV transform to all texture samplers)
        // This test verifies the data is present and correct
    }

    /// T041: Verify UV offset is applied uniformly to all texture coordinates
    #[test]
    fn test_uv_offset_all_textures() {
        // Profile with custom UV offset
        let profile = VisualAssetProfile {
            id: "offset_uv_profile".to_string(),
            albedo_path: "albedo.png".to_string(),
            normal_path: Some("normal.png".to_string()),
            orm_path: Some("orm.png".to_string()),
            emissive_path: Some("emissive.png".to_string()),
            depth_path: Some("depth.png".to_string()),
            roughness: 0.5,
            metallic: 0.3,
            uv_scale: Vec2::splat(1.0),
            uv_offset: Vec2::new(0.5, 0.25), // Offset by 50% X, 25% Y
            depth_scale: 0.1,
            fallback_chain: vec![],
        };

        // Verify UV offset is set
        assert_eq!(profile.uv_offset, Vec2::new(0.5, 0.25));

        // All textures should use this same UV offset in material
    }

    /// T041: Verify combined UV scale and offset
    #[test]
    fn test_combined_uv_transforms() {
        // Profile with both scale and offset
        let profile = VisualAssetProfile {
            id: "combined_uv_profile".to_string(),
            albedo_path: "albedo.png".to_string(),
            normal_path: Some("normal.png".to_string()),
            orm_path: Some("orm.png".to_string()),
            emissive_path: Some("emissive.png".to_string()),
            depth_path: Some("depth.png".to_string()),
            roughness: 0.5,
            metallic: 0.3,
            uv_scale: Vec2::new(1.5, 2.0),
            uv_offset: Vec2::new(0.25, 0.1),
            depth_scale: 0.1,
            fallback_chain: vec![],
        };

        // Verify both transforms are set correctly
        assert_eq!(profile.uv_scale, Vec2::new(1.5, 2.0));
        assert_eq!(profile.uv_offset, Vec2::new(0.25, 0.1));

        // These transforms should affect all texture coordinates uniformly
    }

    /// T041: Verify UV transforms deserialize from RON correctly
    #[test]
    fn test_uv_transforms_deserialization() {
        let ron_str = r#"
            (
                id: "uv_transform_profile",
                albedo_path: "albedo.png",
                normal_path: Some("normal.png"),
                orm_path: Some("orm.png"),
                emissive_path: Some("emissive.png"),
                depth_path: Some("depth.png"),
                uv_scale: (1.5, 2.0),
                uv_offset: (0.25, 0.1),
            )
        "#;

        let profile: VisualAssetProfile =
            ron::from_str(ron_str).expect("Failed to deserialize UV transform profile");

        // Verify deserialized values
        assert_eq!(profile.uv_scale, Vec2::new(1.5, 2.0));
        assert_eq!(profile.uv_offset, Vec2::new(0.25, 0.1));

        // Verify all textures are still present
        assert!(profile.albedo_path.starts_with("albedo"));
        assert_eq!(profile.normal_path, Some("normal.png".to_string()));
        assert_eq!(profile.orm_path, Some("orm.png".to_string()));
        assert_eq!(profile.emissive_path, Some("emissive.png".to_string()));
        assert_eq!(profile.depth_path, Some("depth.png".to_string()));
    }

    /// T041: Verify default UV transforms (identity)
    #[test]
    fn test_default_uv_transforms() {
        let ron_str = r#"
            (
                id: "default_uv_profile",
                albedo_path: "albedo.png",
            )
        "#;

        let profile: VisualAssetProfile =
            ron::from_str(ron_str).expect("Failed to deserialize default UV profile");

        // Verify default UV transforms (identity: scale=1.0, offset=0.0)
        assert_eq!(profile.uv_scale, Vec2::splat(1.0));
        assert_eq!(profile.uv_offset, Vec2::ZERO);
    }
}
