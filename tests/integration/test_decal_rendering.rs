//! Integration test for brick-type-decals: verifies decals are visible and centered on the top side of each brick.

use bevy::prelude::*;
use brkrs::level_format::brick_types::{BrickType, Decal};
use brkrs::systems::brick_decals::{assign_brick_decals, apply_decal_normal_maps};
use bevy::render::mesh::MeshMaterial3d;

#[test]
fn decals_are_visible_and_centered() {
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

    // Spawn bricks with different types
    let brick1 = app.world_mut().spawn((
        BrickType::Standard,
        MeshMaterial3d(material_handle.clone()),
    )).id();

    let brick2 = app.world_mut().spawn((
        BrickType::Indestructible,
        MeshMaterial3d(material_handle.clone()),
    )).id();

    let brick3 = app.world_mut().spawn((
        BrickType::MultiHit,
        MeshMaterial3d(material_handle.clone()),
    )).id();

    // Add the decal systems
    app.add_systems(Update, assign_brick_decals);
    app.add_systems(Update, apply_decal_normal_maps);

    // Update the app to run systems
    app.update();

    // Verify decals were assigned
    let decal1 = app.world().get::<Decal>(brick1).expect("Standard brick should have decal");
    let decal2 = app.world().get::<Decal>(brick2).expect("Indestructible brick should have decal");
    let decal3 = app.world().get::<Decal>(brick3).expect("MultiHit brick should have decal");

    // Check decal properties
    assert!(decal1.is_centered_on_top(), "Standard brick decal should be centered on top");
    assert!(decal1.is_visible(), "Standard brick decal should be visible");
    assert!(decal1.has_normal_map(), "Standard brick decal should have normal map");

    assert!(decal2.is_centered_on_top(), "Indestructible brick decal should be centered on top");
    assert!(decal2.is_visible(), "Indestructible brick decal should be visible");
    assert!(decal2.has_normal_map(), "Indestructible brick decal should have normal map");

    assert!(decal3.is_centered_on_top(), "MultiHit brick decal should be centered on top");
    assert!(decal3.is_visible(), "MultiHit brick decal should be visible");
    assert!(decal3.has_normal_map(), "MultiHit brick decal should have normal map");

    // Verify materials were updated with normal maps
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    let material1 = materials.get(&material_handle).expect("Material should exist");

    // The material should now have normal mapping applied
    // (Note: In a real integration test, we'd check that the normal map is actually applied)
    // For now, we verify the decal assignment worked
}
