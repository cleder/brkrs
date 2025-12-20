use bevy::prelude::*;
use brkrs::systems::spawning::{
    spawn_camera, spawn_ground_plane, spawn_light, GroundPlane, MainCamera,
};

#[test]
fn test_spawn_camera() {
    let mut app = App::new();
    app.add_systems(Startup, spawn_camera);
    app.update();

    let mut query = app
        .world_mut()
        .query_filtered::<Entity, (With<Camera3d>, With<MainCamera>)>();
    let camera = query.iter(app.world()).next();
    assert!(camera.is_some(), "MainCamera should be spawned");

    let entity = camera.unwrap();
    let transform = app.world().get::<Transform>(entity).unwrap();
    assert_eq!(transform.translation, Vec3::new(0.0, 37.0, 0.0));
    // We can't easily check looking_at without matrix math, but position is a good start.
}

#[test]
fn test_spawn_ground_plane() {
    let mut app = App::new();
    app.add_plugins(AssetPlugin::default()); // Needed for mesh/material assets
    app.add_plugins(MaterialPlugin::<StandardMaterial>::default());
    app.init_asset::<Mesh>();

    app.add_systems(Startup, spawn_ground_plane);
    app.update();

    let mut query = app.world_mut().query_filtered::<Entity, (
        With<GroundPlane>,
        With<Mesh3d>,
        With<MeshMaterial3d<StandardMaterial>>,
    )>();
    let plane = query.iter(app.world()).next();
    assert!(
        plane.is_some(),
        "GroundPlane should be spawned with mesh and material"
    );
}

#[test]
fn test_spawn_light() {
    let mut app = App::new();
    app.add_systems(Startup, spawn_light);
    app.update();

    let mut query = app
        .world_mut()
        .query_filtered::<&PointLight, With<PointLight>>();
    let light = query.iter(app.world()).next();
    assert!(light.is_some(), "PointLight should be spawned");

    let light_component = light.unwrap();
    assert!(light_component.shadows_enabled, "Shadows should be enabled");
    assert_eq!(light_component.intensity, 10_000_000.0);
    assert_eq!(light_component.range, 100.0);
}
