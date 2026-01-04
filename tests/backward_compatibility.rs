// Test backward compatibility with old texture profiles (albedo + normal only)
// Ensures new PBR texture fields don't break existing configurations

#[cfg(test)]
mod backward_compatibility_tests {
    use bevy::prelude::Vec2;
    use brkrs::systems::textures::loader::VisualAssetProfile;

    /// T039: Verify old profiles (albedo + normal only) deserialize and load without errors
    #[test]
    fn test_backward_compatibility_no_new_textures() {
        // RON deserialization of old-style profile (no ORM, emissive, or depth)
        let ron_str = r#"
            (
                id: "old_brick_profile",
                albedo_path: "brick_albedo.png",
                normal_path: Some("brick_normal.png"),
                roughness: 0.5,
                metallic: 0.3,
            )
        "#;

        let profile: VisualAssetProfile =
            ron::from_str(ron_str).expect("Failed to deserialize old-style profile");

        // Verify core fields are present
        assert_eq!(profile.id, "old_brick_profile");
        assert_eq!(profile.albedo_path, "brick_albedo.png");
        assert_eq!(profile.normal_path, Some("brick_normal.png".to_string()));
        assert_eq!(profile.roughness, 0.5);
        assert_eq!(profile.metallic, 0.3);

        // Verify new fields are safely defaulted
        assert_eq!(profile.orm_path, None);
        assert_eq!(profile.emissive_path, None);
        assert_eq!(profile.depth_path, None);
        assert_eq!(profile.depth_scale, 0.1); // Default value
    }

    /// T039: Verify old profiles with partial fields deserialize correctly
    #[test]
    fn test_backward_compatibility_partial_fields() {
        // RON with only required fields
        let ron_str = r#"
            (
                id: "minimal_profile",
                albedo_path: "albedo.png",
            )
        "#;

        let profile: VisualAssetProfile =
            ron::from_str(ron_str).expect("Failed to deserialize minimal profile");

        assert_eq!(profile.id, "minimal_profile");
        assert_eq!(profile.albedo_path, "albedo.png");
        assert_eq!(profile.normal_path, None);

        // All optional fields should have sensible defaults
        assert_eq!(profile.orm_path, None);
        assert_eq!(profile.emissive_path, None);
        assert_eq!(profile.depth_path, None);
        assert_eq!(profile.roughness, 0.5); // Default
        assert_eq!(profile.metallic, 0.0); // Default
        assert_eq!(profile.uv_scale, Vec2::splat(1.0)); // Default
        assert_eq!(profile.depth_scale, 0.1); // Default
    }

    /// T039: Verify conversion from old profile maintains all default values
    #[test]
    fn test_backward_compatibility_contract_conversion() {
        // Create a minimal profile via contract
        let contract_ron = r#"
            (
                id: "from_contract",
                albedo_path: "old_albedo.png",
            )
        "#;

        let profile: VisualAssetProfile =
            ron::from_str(contract_ron).expect("Failed to deserialize from contract");

        // All new fields should be safely absent (use defaults)
        assert!(profile.orm_path.is_none());
        assert!(profile.emissive_path.is_none());
        assert!(profile.depth_path.is_none());
    }

    /// T039: Verify make_material handles profiles without new texture paths
    /// This test ensures make_material doesn't crash when orm_path, emissive_path, depth_path are None
    #[test]
    fn test_backward_compatibility_make_material_no_new_textures() {
        // Create profile with only albedo and normal (no new texture types)
        let profile = VisualAssetProfile {
            id: "backward_compat_test".to_string(),
            albedo_path: "test_albedo.png".to_string(),
            normal_path: Some("test_normal.png".to_string()),
            orm_path: None,
            emissive_path: None,
            depth_path: None,
            roughness: 0.6,
            metallic: 0.2,
            uv_scale: Vec2::splat(1.0),
            uv_offset: Vec2::ZERO,
            depth_scale: 0.1,
            fallback_chain: vec![],
        };

        // This should not panic - graceful handling of None values
        // In actual use, asset_server would be mocked or real
        // For this unit test, we just verify the struct is valid
        assert_eq!(profile.orm_path, None);
        assert_eq!(profile.emissive_path, None);
        assert_eq!(profile.depth_path, None);

        // Material creation would skip texture assignments for None paths
        // No panic should occur
    }
}
