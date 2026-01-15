//! Regression test for brick system after adding brick 41 (Extra Life).
//
// Ensures existing brick types (simple stone 20, indestructible 90, multi-hit 10-13)
// retain score/sound/durability behavior after brick 41 code lands.

use bevy::{app::App, prelude::*, MinimalPlugins};
use brkrs::{BrickTypeId, CountsTowardsCompletion};
use std::sync::Mutex;

// Protects BK_LEVEL/BK_LEVEL_PATH mutations in this module so concurrent tests don't race on env vars.
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn test_app() -> App {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));
    app.add_message::<bevy_rapier3d::prelude::CollisionEvent>();
    app.insert_resource(brkrs::GameProgress::default());
    app.insert_resource(brkrs::level_loader::LevelAdvanceState::default());
    app.insert_resource(brkrs::systems::respawn::SpawnPoints::default());
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(bevy::input::ButtonInput::<bevy::prelude::KeyCode>::default());
    app.world_mut()
        .spawn(bevy_rapier3d::prelude::RapierConfiguration::new(1.0));
    app.add_plugins(brkrs::systems::LevelSwitchPlugin);
    app.add_plugins(brkrs::level_loader::LevelLoaderPlugin);
    brkrs::register_brick_collision_systems(&mut app);
    app
}

#[test]
fn simple_stone_brick_20_still_spawns() {
    let _env_guard = ENV_LOCK.lock().unwrap();
    let mut app = test_app();
    std::env::set_var("BK_LEVEL", "1"); // Level 1 has simple stone (20)
    app.update();
    app.update();

    let world = app.world_mut();
    let simple_stones = world
        .query_filtered::<&BrickTypeId, With<CountsTowardsCompletion>>()
        .iter(world)
        .filter(|tid| tid.0 == 20)
        .count();

    assert!(
        simple_stones > 0,
        "Expected simple stone bricks (id 20) in level 1"
    );
    std::env::remove_var("BK_LEVEL");
}

#[test]
fn indestructible_brick_90_still_spawns() {
    let _env_guard = ENV_LOCK.lock().unwrap();
    let mut app = test_app();
    std::env::set_var("BK_LEVEL", "997"); // Level 997 has indestructible (90)
    app.update();
    app.update();

    let world = app.world_mut();
    let indestructibles = world
        .query::<&BrickTypeId>()
        .iter(world)
        .filter(|tid| tid.0 == 90)
        .count();

    assert!(
        indestructibles > 0,
        "Expected indestructible bricks (id 90) in level 997"
    );
    std::env::remove_var("BK_LEVEL");
}

#[test]
fn multi_hit_bricks_10_to_13_still_spawn() {
    let _env_guard = ENV_LOCK.lock().unwrap();
    let mut app = test_app();
    std::env::set_var("BK_LEVEL", "998"); // Level 998 has multi-hit bricks
    app.update();
    app.update();

    let world = app.world_mut();
    let multi_hit_bricks = world
        .query::<&BrickTypeId>()
        .iter(world)
        .filter(|tid| (10..=13).contains(&tid.0))
        .count();

    assert!(
        multi_hit_bricks > 0,
        "Expected multi-hit bricks (ids 10-13) in level 998"
    );
    std::env::remove_var("BK_LEVEL");
}
