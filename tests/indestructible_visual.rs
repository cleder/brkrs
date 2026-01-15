use bevy::{app::App, prelude::*, MinimalPlugins};
use brkrs::BrickTypeId;

fn test_app() -> App {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));
    // Collision events are delivered via the global CollisionEvent message resource
    app.add_message::<bevy_rapier3d::prelude::CollisionEvent>();
    app.insert_resource(brkrs::GameProgress::default());
    app.insert_resource(brkrs::level_loader::LevelAdvanceState::default());
    app.insert_resource(brkrs::systems::respawn::SpawnPoints::default());
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(bevy::input::ButtonInput::<bevy::prelude::KeyCode>::default());
    // need rapier config entity for physics queries used by level systems
    app.world_mut()
        .spawn(bevy_rapier3d::prelude::RapierConfiguration::new(1.0));
    app.add_plugins(brkrs::systems::LevelSwitchPlugin);
    app.add_plugins(brkrs::level_loader::LevelLoaderPlugin);
    brkrs::register_brick_collision_systems(&mut app);
    app
}

#[test]
fn indestructible_bricks_have_material_component() {
    let mut app = test_app();

    // Load the sample mixed level (level_997.ron / test_mixed_indestructible.ron)
    std::env::set_var("BK_LEVEL", "997");
    app.update();
    app.update();

    // Ensure at least one indestructible brick (type 90) has a MeshMaterial3d component.
    let world = &mut app.world_mut();
    let mut found = false;
    // query exists below; no separate iterator required
    for (type_id, _mat) in world
        .query::<(&BrickTypeId, &MeshMaterial3d<StandardMaterial>)>()
        .iter(world)
    {
        if type_id.0 == 90 {
            found = true;
            break;
        }
    }

    assert!(
        found,
        "Expected at least one indestructible brick (type 90) with a material component"
    );
    std::env::remove_var("BK_LEVEL");
}
