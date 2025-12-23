use bevy::prelude::*;
use brkrs::level_format::brick_types::{BrickType, Decal};

#[test]
fn decal_3d_effect_consistent_from_angles() {
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
    // Query for decal and check 3D effect consistency
    let decal = app
        .world()
        .get::<Decal>(brick_entity)
        .expect("Decal missing");
    // Check that 3D effect is consistent
    assert!(
        decal.effect_consistent_from_different_angles(),
        "3D effect should be consistent from different angles"
    );
}
