//! Contract test for brick-type-decals normal mapping: verifies normal/bump mapping is applied and visible under lighting.

use bevy::prelude::*;
use brkrs::level_format::brick_types::{BrickType, Decal};
use brkrs::systems::brick_decals::{assign_brick_decals, apply_decal_normal_maps};
use bevy::render::mesh::MeshMaterial3d;

#[test]
fn normal_mapping_is_applied_and_visible_under_lighting() {
    // Setup a minimal Bevy app for testing
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());

    // Create a test material
    let mut materials = app.world_mut().resource_mut::<Assets<StandardMaterial>>();
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0), // Red brick
        ..default()
    });

    // Spawn a brick with decal
    let brick = app.world_mut().spawn((
        BrickType::Standard,
        MeshMaterial3d(material_handle.clone()),
    )).id();

    // Add the decal systems
    app.add_systems(Update, assign_brick_decals);
    app.add_systems(Update, apply_decal_normal_maps);

    // Update the app to run systems
    app.update();

    // Verify decal was assigned and has normal map
    let decal = app.world().get::<Decal>(brick).expect("Brick should have decal");
    assert!(decal.has_normal_map(), "Decal should have normal map");
    assert!(decal.normal_map_visible_under_lighting(),
        "Normal map should be visible under lighting");

    // Verify material was updated with normal mapping
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    let material = materials.get(&material_handle).expect("Material should exist");

    // Check that normal_map_texture was set
    assert!(material.normal_map_texture.is_some(),
        "Material should have normal map texture applied");
}