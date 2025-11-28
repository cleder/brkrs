use bevy::{app::App, prelude::*, MinimalPlugins};
use brkrs::level_loader::LevelDefinition;
use brkrs::{BrickTypeId, CountsTowardsCompletion};

fn level_test_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));
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
    app
}

#[test]
fn spawn_marks_counts_for_non_indestructible_bricks() {
    let mut app = level_test_app();

    // Prepare a temporary level file under assets/levels
    let path = "assets/levels/level_999.ron";
    let contents = r#"LevelDefinition(number:999,matrix:[[90,20,3]])"#;
    std::fs::write(path, contents).expect("write test level");

    // Set env so loader picks the test file
    std::env::set_var("BK_LEVEL", "999");

    // Run startup systems (load_level) and let systems settle
    app.update();
    app.update();

    // Query bricks and their types + completion marker
    let mut found_90 = false;
    let mut found_20 = false;
    let mut found_3 = false;

    let world = &mut app.world_mut();
    let mut q = world.query::<(&BrickTypeId, Option<&CountsTowardsCompletion>)>();
    for (type_id, maybe_marker) in q.iter(world) {
        if type_id.0 == 90 {
            found_90 = true;
            assert!(
                maybe_marker.is_none(),
                "indestructible brick must NOT count for completion"
            );
        }
        if type_id.0 == 20 {
            found_20 = true;
            assert!(
                maybe_marker.is_some(),
                "simple brick (20) must count for completion"
            );
        }
        if type_id.0 == 3 {
            found_3 = true;
            assert!(
                maybe_marker.is_some(),
                "legacy simple brick (3) must count for completion during compatibility window"
            );
        }
    }

    assert!(
        found_90 && found_20 && found_3,
        "All three brick types should be present in spawned bricks"
    );

    // cleanup
    let _ = std::fs::remove_file(path);
}
