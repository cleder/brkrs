use bevy::pbr::wireframe::WireframeConfig;
use bevy::prelude::*;
use brkrs::systems::grid_debug::toggle_grid_visibility;
use brkrs::GridOverlay;

#[derive(Resource, Default)]
struct ChangeResult(bool);

fn verify_change(
    query: Query<Ref<Visibility>, With<GridOverlay>>,
    mut result: ResMut<ChangeResult>,
) {
    for vis in query.iter() {
        if vis.is_changed() {
            result.0 = true;
        }
    }
}

#[test]
fn toggle_grid_visibility_only_runs_on_config_change() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.init_resource::<WireframeConfig>();
    app.init_resource::<ChangeResult>();
    app.add_systems(Update, toggle_grid_visibility);
    app.add_systems(PostUpdate, verify_change);

    // Spawn a grid entity
    let entity = app
        .world_mut()
        .spawn((GridOverlay, Visibility::Hidden))
        .id();

    // Initial update - should set visibility based on default config (false -> Hidden)
    app.update();

    // Reset result
    app.world_mut().resource_mut::<ChangeResult>().0 = false;

    // Update again with NO config change
    app.update();

    // Verify NO change detected
    assert!(
        !app.world().resource::<ChangeResult>().0,
        "Visibility should not change when config is unchanged"
    );

    // Change config
    app.world_mut().resource_mut::<WireframeConfig>().global = true;
    // Reset result
    app.world_mut().resource_mut::<ChangeResult>().0 = false;

    app.update();

    // Verify change detected
    assert!(
        app.world().resource::<ChangeResult>().0,
        "Visibility should change when config changes"
    );

    // Verify value
    let entity_ref = app.world().entity(entity);
    let visibility = entity_ref.get::<Visibility>().unwrap();
    assert_eq!(*visibility, Visibility::Visible);
}
