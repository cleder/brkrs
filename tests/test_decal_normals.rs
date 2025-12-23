use bevy::prelude::*;
use brkrs::level_format::brick_types::{BrickType, Decal};

#[test]
fn normal_bump_mapping_applied_to_decals() {
    // Setup a minimal Bevy app for testing
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()));
    app.init_asset::<Image>();
    // Add asset server for loading normal maps
    let asset_server = app.world().resource::<AssetServer>();
    // Spawn bricks with decals that have normal maps
    let decal = Decal {
        brick_type: BrickType::Standard,
        normal_map_handle: Some(asset_server.load("textures/decals/standard_normal.png")),
    };
    let brick_entity = app.world_mut().spawn((BrickType::Standard, decal)).id();
    // Query for decal and check normal mapping
    let decal = app
        .world()
        .get::<Decal>(brick_entity)
        .expect("Decal missing");
    // Check that normal mapping is applied
    assert!(
        decal.has_normal_map(),
        "Decal should have normal map applied"
    );
    assert!(
        decal.normal_map_visible_under_lighting(),
        "Normal map should be visible under lighting"
    );
}
