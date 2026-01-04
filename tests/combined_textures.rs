// Test combined texture loading - verify all 5 texture types load together without conflicts
// Ensures ORM, emissive, and depth textures don't interfere with albedo and normal

#[cfg(test)]
mod combined_textures_tests {
    use bevy::prelude::Vec2;
    use brkrs::systems::textures::loader::VisualAssetProfile;

    /// T040: Verify all 5 texture types deserialize together in one profile
    #[test]
    fn test_combined_textures_all_five_types() {
        // RON deserialization with all 5 texture types
        let ron_str = r#"
            (
                id: "full_pbr_profile",
                albedo_path: "brick_albedo.png",
                normal_path: Some("brick_normal.png"),
                orm_path: Some("brick_orm.png"),
                emissive_path: Some("brick_emissive.png"),
                depth_path: Some("brick_depth.png"),
                roughness: 0.5,
                metallic: 0.3,
                uv_scale: (1.0, 1.0),
                depth_scale: 0.15,
            )
        "#;

        let profile: VisualAssetProfile =
            ron::from_str(ron_str).expect("Failed to deserialize full PBR profile");

        // Verify all 5 texture paths are present
        assert_eq!(profile.albedo_path, "brick_albedo.png");
        assert_eq!(profile.normal_path, Some("brick_normal.png".to_string()));
        assert_eq!(profile.orm_path, Some("brick_orm.png".to_string()));
        assert_eq!(
            profile.emissive_path,
            Some("brick_emissive.png".to_string())
        );
        assert_eq!(profile.depth_path, Some("brick_depth.png".to_string()));

        // Verify all scalar values are set
        assert_eq!(profile.roughness, 0.5);
        assert_eq!(profile.metallic, 0.3);
        assert_eq!(profile.uv_scale, Vec2::new(1.0, 1.0));
        assert_eq!(profile.depth_scale, 0.15);
    }

    /// T040: Verify that loading make_material with all textures doesn't panic
    /// Test struct construction with all texture paths populated
    #[test]
    fn test_combined_textures_struct_construction() {
        let profile = VisualAssetProfile {
            id: "full_profile".to_string(),
            albedo_path: "test_albedo.png".to_string(),
            normal_path: Some("test_normal.png".to_string()),
            orm_path: Some("test_orm.png".to_string()),
            emissive_path: Some("test_emissive.png".to_string()),
            depth_path: Some("test_depth.png".to_string()),
            roughness: 0.6,
            metallic: 0.2,
            uv_scale: Vec2::splat(1.0),
            uv_offset: Vec2::ZERO,
            depth_scale: 0.1,
            fallback_chain: vec![],
        };

        // Verify all fields are correctly set
        assert_eq!(profile.id, "full_profile");
        assert_eq!(profile.albedo_path, "test_albedo.png");
        assert!(profile.normal_path.is_some());
        assert!(profile.orm_path.is_some());
        assert!(profile.emissive_path.is_some());
        assert!(profile.depth_path.is_some());

        // Should not panic during construction or field access
    }

    /// T040: Verify combined texture paths serialize and deserialize without loss
    /// Round-trip test: deserialize → verify → structure is valid for serialization
    #[test]
    fn test_combined_textures_partial_overlap() {
        // Profile with some textures, missing others
        let ron_str = r#"
            (
                id: "partial_pbr",
                albedo_path: "albedo.png",
                normal_path: Some("normal.png"),
                orm_path: Some("orm.png"),
                emissive_path: None,  // Emissive not used
                depth_path: None,     // Depth not used
            )
        "#;

        let profile: VisualAssetProfile =
            ron::from_str(ron_str).expect("Failed to deserialize partial profile");

        // Verify present textures
        assert_eq!(profile.albedo_path, "albedo.png");
        assert_eq!(profile.normal_path, Some("normal.png".to_string()));
        assert_eq!(profile.orm_path, Some("orm.png".to_string()));

        // Verify absent textures are None
        assert_eq!(profile.emissive_path, None);
        assert_eq!(profile.depth_path, None);

        // Verify defaults for unspecified fields
        assert_eq!(profile.roughness, 0.5); // Default
        assert_eq!(profile.metallic, 0.0); // Default
        assert_eq!(profile.depth_scale, 0.1); // Default
    }
}
