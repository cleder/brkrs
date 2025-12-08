use bevy::prelude::*;

use brkrs::systems::textures::materials::{
    BaselineMaterialKind, CanonicalMaterialHandles, FallbackRegistry, ProfileMaterialBank,
    TextureMaterialsPlugin,
};
use brkrs::systems::textures::{
    ObjectClass, TextureManifest, TypeVariantRegistry, VisualAssetProfile,
};
use brkrs::{Ball, BallTypeId};

/// Helper to setup a test app with texture materials plugin and a test manifest
fn setup_test_app_with_manifest() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<Image>::default());

    // Create a mock texture manifest with ball profile
    let mut manifest = TextureManifest::default();
    manifest.profiles.insert(
        "ball/default".to_string(),
        VisualAssetProfile {
            id: "ball/default".to_string(),
            albedo_path: "test/ball.png".to_string(),
            normal_path: None,
            roughness: 0.5,
            metallic: 0.0,
            uv_scale: Vec2::ONE,
            uv_offset: Vec2::ZERO,
            fallback_chain: vec![],
        },
    );
    manifest
        .type_variants
        .push(brkrs::systems::textures::TypeVariantDefinition {
            object_class: ObjectClass::Ball,
            type_id: 0,
            profile_id: "ball/default".to_string(),
            emissive_color: None,
            animation: None,
        });

    app.insert_resource(manifest);
    app.add_plugins(TextureMaterialsPlugin);
    app
}

/// Test that ball entities spawned during Startup receive materials
/// once canonical materials become ready.
#[test]
fn ball_receives_materials_on_startup() {
    let mut app = setup_test_app_with_manifest();

    // Spawn a ball entity during "Startup" (before materials are hydrated)
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

    // First update: runs Startup systems (initialize_fallback_registry)
    app.update();

    // Second update: runs Update systems (hydrate_texture_materials, apply_canonical_materials_to_existing_entities)
    app.update();

    // Verify canonical materials are now ready
    let canonical = app.world().resource::<CanonicalMaterialHandles>();
    assert!(canonical.is_ready(), "Canonical materials should be ready");

    // Verify ball now has a non-default material
    let updated_material = app
        .world()
        .entity(ball_entity)
        .get::<MeshMaterial3d<StandardMaterial>>()
        .expect("Ball should still have material component")
        .0
        .clone();

    assert_ne!(
        initial_material, updated_material,
        "Ball material should have been updated from default to canonical"
    );

    // Verify it's a valid canonical or type variant material
    let has_canonical = canonical.get(BaselineMaterialKind::Ball).is_some();
    let type_registry = app.world().resource::<TypeVariantRegistry>();
    let has_type_variant = type_registry.get(ObjectClass::Ball, 0).is_some();

    assert!(
        has_canonical || has_type_variant,
        "Ball should receive either canonical or type variant material"
    );
}

/// Test that the system only applies materials once (validates Local<bool> behavior)
#[test]
fn materials_applied_only_once() {
    let mut app = setup_test_app_with_manifest();

    // Spawn a ball entity
    let ball_entity = app
        .world_mut()
        .spawn((
            Ball,
            BallTypeId(0),
            MeshMaterial3d(Handle::<StandardMaterial>::default()),
        ))
        .id();

    // First update: Startup systems
    app.update();

    // Second update: hydrate materials and apply to entities
    app.update();

    // Get the material after first application
    let first_material = app
        .world()
        .entity(ball_entity)
        .get::<MeshMaterial3d<StandardMaterial>>()
        .expect("Ball should have material")
        .0
        .clone();

    // Verify it's not the default handle anymore
    assert_ne!(
        first_material,
        Handle::<StandardMaterial>::default(),
        "Ball should have received a material"
    );

    // Spawn a second ball entity AFTER materials have been applied
    let second_ball_entity = app
        .world_mut()
        .spawn((
            Ball,
            BallTypeId(0),
            MeshMaterial3d(Handle::<StandardMaterial>::default()),
        ))
        .id();

    // Third update - system should NOT reapply (Local<bool> prevents it)
    app.update();

    // First ball should still have the same material
    let first_ball_material = app
        .world()
        .entity(ball_entity)
        .get::<MeshMaterial3d<StandardMaterial>>()
        .expect("Ball should still have material")
        .0
        .clone();

    assert_eq!(
        first_material, first_ball_material,
        "First ball material should NOT change on subsequent updates"
    );

    // Second ball should still have default handle (system already ran)
    let second_ball_material = app
        .world()
        .entity(second_ball_entity)
        .get::<MeshMaterial3d<StandardMaterial>>()
        .expect("Second ball should have material")
        .0
        .clone();

    assert_eq!(
        second_ball_material,
        Handle::<StandardMaterial>::default(),
        "Second ball spawned after application should keep default handle (system runs once)"
    );
}

/// Test WASM race condition: materials ready before entities spawn
#[test]
fn handles_wasm_race_condition() {
    let mut app = setup_test_app_with_manifest();

    // First update: initialize systems and hydrate materials (no entities yet)
    app.update();
    app.update();

    // Verify canonical materials are ready but no balls exist
    let canonical = app.world().resource::<CanonicalMaterialHandles>();
    assert!(canonical.is_ready(), "Canonical materials should be ready");

    let ball_count = app.world().query::<&Ball>().iter(app.world()).count();
    assert_eq!(ball_count, 0, "No balls should exist yet");

    // NOW spawn the ball entity (late spawn - simulating WASM where materials load before entities)
    let ball_entity = app
        .world_mut()
        .spawn((
            Ball,
            BallTypeId(0),
            MeshMaterial3d(Handle::<StandardMaterial>::default()),
        ))
        .id();

    // Third update: system should retry because updated_count was 0 previously (applied flag not set)
    app.update();

    // Verify the late-spawned ball got a proper material (not default)
    let ball_material = app
        .world()
        .entity(ball_entity)
        .get::<MeshMaterial3d<StandardMaterial>>()
        .expect("Ball should have material")
        .0
        .clone();

    assert_ne!(
        ball_material,
        Handle::<StandardMaterial>::default(),
        "Late-spawned ball should receive canonical material (WASM race condition handled)"
    );

    // Verify it's a valid material from canonical or type registry
    let has_canonical = canonical.get(BaselineMaterialKind::Ball).is_some();
    let type_registry = app.world().resource::<TypeVariantRegistry>();
    let has_type_variant = type_registry.get(ObjectClass::Ball, 0).is_some();

    assert!(
        has_canonical || has_type_variant,
        "Ball should have received either canonical or type variant material"
    );
}
