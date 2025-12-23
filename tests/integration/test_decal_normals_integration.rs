//! Integration test for brick-type-decals normal mapping: verifies 3D effect of decals is consistent from different angles.

use bevy::prelude::*;
use brkrs::level_format::brick_types::{BrickType, Decal};
use brkrs::systems::brick_decals::{assign_brick_decals, apply_decal_normal_maps};
use bevy::render::mesh::MeshMaterial3d;

#[test]
fn decal_3d_effect_consistent_from_different_angles() {
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

    // Spawn bricks with different decal types
    let standard_brick = app.world_mut().spawn((
        BrickType::Standard,
        MeshMaterial3d(material_handle.clone()),
    )).id();

    let indestructible_brick = app.world_mut().spawn((
        BrickType::Indestructible,
        MeshMaterial3d(material_handle.clone()),
    )).id();

    let multihit_brick = app.world_mut().spawn((
        BrickType::MultiHit,
        MeshMaterial3d(material_handle.clone()),
    )).id();

    // Add the decal systems
    app.add_systems(Update, assign_brick_decals);
    app.add_systems(Update, apply_decal_normal_maps);

    // Update the app to run systems
    app.update();

    // Verify all decals have consistent 3D effects
    let standard_decal = app.world().get::<Decal>(standard_brick)
        .expect("Standard brick should have decal");
    let indestructible_decal = app.world().get::<Decal>(indestructible_brick)
        .expect("Indestructible brick should have decal");
    let multihit_decal = app.world().get::<Decal>(multihit_brick)
        .expect("MultiHit brick should have decal");

    // Check that 3D effects are consistent from different angles
    assert!(standard_decal.effect_consistent_from_different_angles(),
        "Standard brick decal 3D effect should be consistent from different angles");
    assert!(indestructible_decal.effect_consistent_from_different_angles(),
        "Indestructible brick decal 3D effect should be consistent from different angles");
    assert!(multihit_decal.effect_consistent_from_different_angles(),
        "MultiHit brick decal 3D effect should be consistent from different angles");

    // Verify normal maps are applied to materials
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    let material = materials.get(&material_handle).expect("Material should exist");

    // Normal mapping should be applied
    assert!(material.normal_map_texture.is_some(),
        "Material should have normal map texture for 3D effects");
}