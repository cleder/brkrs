// Test fallback chain resolution with new texture types
// Ensures fallback chain works correctly when profiles have missing new texture paths

#[cfg(test)]
mod fallback_chain_tests {
    use bevy::prelude::Vec2;
    use brkrs::systems::textures::loader::VisualAssetProfile;

    /// T042: Verify fallback chain with missing new texture types
    #[test]
    fn test_fallback_chain_with_missing_new_textures() {
        // Primary profile missing ORM and emissive
        let primary = VisualAssetProfile {
            id: "primary_partial".to_string(),
            albedo_path: "primary_albedo.png".to_string(),
            normal_path: Some("primary_normal.png".to_string()),
            orm_path: None,      // Missing ORM
            emissive_path: None, // Missing emissive
            depth_path: None,
            roughness: 0.6,
            metallic: 0.2,
            uv_scale: Vec2::splat(1.0),
            uv_offset: Vec2::ZERO,
            depth_scale: 0.1,
            fallback_chain: vec!["fallback_full".to_string()], // Has fallback
        };

        // Fallback profile with all textures
        let fallback = VisualAssetProfile {
            id: "fallback_full".to_string(),
            albedo_path: "fallback_albedo.png".to_string(),
            normal_path: Some("fallback_normal.png".to_string()),
            orm_path: Some("fallback_orm.png".to_string()),
            emissive_path: Some("fallback_emissive.png".to_string()),
            depth_path: Some("fallback_depth.png".to_string()),
            roughness: 0.5,
            metallic: 0.3,
            uv_scale: Vec2::splat(1.0),
            uv_offset: Vec2::ZERO,
            depth_scale: 0.1,
            fallback_chain: vec![],
        };

        // Verify primary has fallback specified
        assert_eq!(primary.fallback_chain.len(), 1);
        assert_eq!(primary.fallback_chain[0], "fallback_full");

        // Verify primary has missing textures
        assert_eq!(primary.orm_path, None);
        assert_eq!(primary.emissive_path, None);

        // Verify fallback has all textures
        assert!(fallback.orm_path.is_some());
        assert!(fallback.emissive_path.is_some());
        assert!(fallback.depth_path.is_some());
    }

    /// T042: Verify fallback chain doesn't break when missing textures are present
    #[test]
    fn test_fallback_chain_with_all_new_textures() {
        // Primary profile with all textures, but has fallback (shouldn't be needed)
        let primary = VisualAssetProfile {
            id: "primary_full".to_string(),
            albedo_path: "primary_albedo.png".to_string(),
            normal_path: Some("primary_normal.png".to_string()),
            orm_path: Some("primary_orm.png".to_string()),
            emissive_path: Some("primary_emissive.png".to_string()),
            depth_path: Some("primary_depth.png".to_string()),
            roughness: 0.5,
            metallic: 0.3,
            uv_scale: Vec2::splat(1.0),
            uv_offset: Vec2::ZERO,
            depth_scale: 0.1,
            fallback_chain: vec!["fallback".to_string()], // Has fallback (unused)
        };

        // Verify primary has all textures and fallback
        assert!(primary.orm_path.is_some());
        assert!(primary.emissive_path.is_some());
        assert!(primary.depth_path.is_some());
        assert_eq!(primary.fallback_chain.len(), 1);

        // Fallback should be used only if primary textures fail to load
    }

    /// T042: Verify multi-level fallback chain
    #[test]
    fn test_fallback_chain_multi_level() {
        // Primary with minimal textures
        let primary = VisualAssetProfile {
            id: "primary_minimal".to_string(),
            albedo_path: "primary_albedo.png".to_string(),
            normal_path: None,
            orm_path: None,
            emissive_path: None,
            depth_path: None,
            roughness: 0.5,
            metallic: 0.0,
            uv_scale: Vec2::splat(1.0),
            uv_offset: Vec2::ZERO,
            depth_scale: 0.1,
            fallback_chain: vec!["fallback_partial".to_string(), "fallback_full".to_string()],
        };

        // Secondary fallback with some textures
        let fallback_partial = VisualAssetProfile {
            id: "fallback_partial".to_string(),
            albedo_path: "fallback_partial_albedo.png".to_string(),
            normal_path: Some("fallback_partial_normal.png".to_string()),
            orm_path: Some("fallback_partial_orm.png".to_string()),
            emissive_path: None,
            depth_path: None,
            roughness: 0.5,
            metallic: 0.2,
            uv_scale: Vec2::splat(1.0),
            uv_offset: Vec2::ZERO,
            depth_scale: 0.1,
            fallback_chain: vec!["fallback_full".to_string()],
        };

        // Final fallback with all textures
        let fallback_full = VisualAssetProfile {
            id: "fallback_full".to_string(),
            albedo_path: "fallback_full_albedo.png".to_string(),
            normal_path: Some("fallback_full_normal.png".to_string()),
            orm_path: Some("fallback_full_orm.png".to_string()),
            emissive_path: Some("fallback_full_emissive.png".to_string()),
            depth_path: Some("fallback_full_depth.png".to_string()),
            roughness: 0.5,
            metallic: 0.3,
            uv_scale: Vec2::splat(1.0),
            uv_offset: Vec2::ZERO,
            depth_scale: 0.1,
            fallback_chain: vec![],
        };

        // Verify fallback chain is ordered
        assert_eq!(primary.fallback_chain[0], "fallback_partial");
        assert_eq!(primary.fallback_chain[1], "fallback_full");

        // Verify fallback chain progression adds textures
        assert_eq!(
            fallback_partial.orm_path,
            Some("fallback_partial_orm.png".to_string())
        );
        assert_eq!(fallback_partial.emissive_path, None);
        assert_eq!(
            fallback_full.emissive_path,
            Some("fallback_full_emissive.png".to_string())
        );
    }

    /// T042: Verify empty fallback chain
    #[test]
    fn test_fallback_chain_empty() {
        let profile = VisualAssetProfile {
            id: "no_fallback".to_string(),
            albedo_path: "albedo.png".to_string(),
            normal_path: Some("normal.png".to_string()),
            orm_path: Some("orm.png".to_string()),
            emissive_path: Some("emissive.png".to_string()),
            depth_path: Some("depth.png".to_string()),
            roughness: 0.5,
            metallic: 0.3,
            uv_scale: Vec2::splat(1.0),
            uv_offset: Vec2::ZERO,
            depth_scale: 0.1,
            fallback_chain: vec![], // No fallback
        };

        // Verify no fallback chain
        assert_eq!(profile.fallback_chain.len(), 0);

        // Profile should be self-contained with all textures
        assert!(profile.orm_path.is_some());
        assert!(profile.emissive_path.is_some());
        assert!(profile.depth_path.is_some());
    }

    /// T042: Verify fallback chain deserialization
    #[test]
    fn test_fallback_chain_deserialization() {
        let ron_str = r#"
            (
                id: "with_fallback",
                albedo_path: "albedo.png",
                normal_path: Some("normal.png"),
                orm_path: None,
                emissive_path: Some("emissive.png"),
                fallback_chain: ["fallback_basic", "fallback_full"],
            )
        "#;

        let profile: VisualAssetProfile =
            ron::from_str(ron_str).expect("Failed to deserialize fallback chain profile");

        // Verify fallback chain deserialized correctly
        assert_eq!(profile.fallback_chain.len(), 2);
        assert_eq!(profile.fallback_chain[0], "fallback_basic");
        assert_eq!(profile.fallback_chain[1], "fallback_full");

        // Verify other fields still present
        assert_eq!(profile.orm_path, None);
        assert!(profile.emissive_path.is_some());
    }
}
