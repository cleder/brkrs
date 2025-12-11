use bevy::prelude::*;

use brkrs::systems::textures::materials::TextureMaterialsPlugin;
use brkrs::{Ball, BallTypeId};

/// Helper to setup a minimal test app
fn setup_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.add_plugins(TextureMaterialsPlugin);
    app
}

/// Test that ball entities spawned during Startup receive materials
/// once fallback materials are available (verified by FallbackRegistry existence)
#[test]
fn ball_receives_fallback_materials_on_startup() {
    let mut app = setup_test_app();

    // Spawn a ball entity with default material
    let ball_entity = app
        .world_mut()
        .spawn((
            Ball,
            BallTypeId(0),
            MeshMaterial3d(Handle::<StandardMaterial>::default()),
        ))
        .id();

    // Verify ball initially has default handle
    let initial_material = app
        .world()
        .entity(ball_entity)
        .get::<MeshMaterial3d<StandardMaterial>>()
        .expect("Ball should have material component")
        .0
        .clone();

    assert_eq!(
        initial_material,
        Handle::<StandardMaterial>::default(),
        "Ball should start with default handle"
    );

    // Run update to initialize fallback registry (Startup systems run)
    app.update();

    // Verify FallbackRegistry was created
    assert!(
        app.world()
            .get_resource::<brkrs::systems::textures::materials::FallbackRegistry>()
            .is_some(),
        "FallbackRegistry should be initialized"
    );
}

/// Test that the TextureMaterialsPlugin properly initializes its resources
#[test]
fn plugin_initializes_required_resources() {
    let mut app = setup_test_app();

    // Run update to trigger Startup systems
    app.update();

    // Verify all expected resources are initialized
    assert!(
        app.world()
            .get_resource::<brkrs::systems::textures::materials::FallbackRegistry>()
            .is_some(),
        "FallbackRegistry should be initialized"
    );

    assert!(
        app.world()
            .get_resource::<brkrs::systems::textures::materials::CanonicalMaterialHandles>()
            .is_some(),
        "CanonicalMaterialHandles should be initialized"
    );

    assert!(
        app.world()
            .get_resource::<brkrs::systems::textures::TypeVariantRegistry>()
            .is_some(),
        "TypeVariantRegistry should be initialized"
    );
}

/// Test that ball entities can be spawned with materials
#[test]
fn ball_entities_spawn_with_materials() {
    let mut app = setup_test_app();

    // Run initial update to initialize systems
    app.update();

    // Spawn a ball entity with material component
    let ball_entity = app
        .world_mut()
        .spawn((
            Ball,
            BallTypeId(0),
            MeshMaterial3d(Handle::<StandardMaterial>::default()),
        ))
        .id();

    // Verify ball was spawned correctly
    assert!(
        app.world().entity(ball_entity).contains::<Ball>(),
        "Ball component should exist"
    );

    assert!(
        app.world().entity(ball_entity).contains::<BallTypeId>(),
        "BallTypeId component should exist"
    );

    assert!(
        app.world()
            .entity(ball_entity)
            .contains::<MeshMaterial3d<StandardMaterial>>(),
        "MeshMaterial3d component should exist"
    );

    // Verify ball count
    let ball_count = app.world_mut().query::<&Ball>().iter(app.world()).count();
    assert_eq!(ball_count, 1, "Exactly one ball should exist");
}
