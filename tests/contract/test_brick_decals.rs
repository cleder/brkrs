//! Contract test for brick-type-decals: verifies all brick types in the test level have correct decals assigned.

use bevy::prelude::*;
use brkrs::level_format::brick_types::{BrickType, Decal};
use brkrs::level_loader::LevelLoaderPlugin;
use brkrs::systems::brick_decals::{assign_brick_decals, assign_brick_decals_fallback};

#[test]
fn all_brick_types_have_correct_decals() {
    // Setup a minimal Bevy app for testing
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(LevelLoaderPlugin);

    // Load the test level
    let level_path = "assets/levels/test_decals.ron";
    let asset_server = app.world().resource::<AssetServer>();
    let level_handle: Handle<brkrs::level_format::LevelDefinition> = asset_server.load(level_path);

    // Wait for assets to load (in a real test, we'd use a proper async setup)
    // For now, we'll simulate the level loading by manually spawning bricks

    // Spawn bricks of different types to test decal assignment
    app.world_mut().spawn((BrickType::Standard,));
    app.world_mut().spawn((BrickType::Indestructible,));
    app.world_mut().spawn((BrickType::MultiHit,));

    // Run the decal assignment systems
    app.add_systems(Update, assign_brick_decals);
    app.add_systems(Update, assign_brick_decals_fallback);

    // Update the app to run systems
    app.update();

    // Query all bricks and check for decal assignment
    let mut query = app.world_mut().query::<(&BrickType, &Decal)>();
    let mut found_types = std::collections::HashSet::new();

    for (brick_type, decal) in query.iter(app.world()) {
        assert!(decal.is_valid_for_type(brick_type),
            "Decal not valid for brick type: {:?}, decal brick_type: {:?}",
            brick_type, decal.brick_type);
        assert!(decal.has_normal_map(),
            "Decal for brick type {:?} should have a normal map", brick_type);
        found_types.insert(brick_type);
    }

    // Verify we found all expected brick types
    assert!(found_types.contains(&BrickType::Standard),
        "Standard brick type not found with decal");
    assert!(found_types.contains(&BrickType::Indestructible),
        "Indestructible brick type not found with decal");
    assert!(found_types.contains(&BrickType::MultiHit),
        "MultiHit brick type not found with decal");
}
