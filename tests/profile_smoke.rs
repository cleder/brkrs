use bevy::{app::App, prelude::*, MinimalPlugins};
use std::time::Instant;

fn make_test_app() -> App {
    let mut app = App::new();
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
fn profile_smoke_loop() {
    // This is a lightweight smoke 'profiling' test which runs a tiny headless loop to
    // surface the cost of level spawning and update loops. It prints average frame time
    // but does not assert strict thresholds to avoid flaky failures in CI.

    // Build a tiny runtime harness (reuse existing test helpers) — we just ensure the app
    // can load a level and run multiple update frames quickly.
    let mut app = make_test_app();

    // Force load an example level (level_001.ron) via BK_LEVEL env var
    std::env::set_var("BK_LEVEL", "001");

    // run startup systems
    app.update();
    app.update();

    let iterations = 200usize;
    let start = Instant::now();
    for _ in 0..iterations {
        app.update();
    }
    let dur = start.elapsed();
    let avg_ms = dur.as_secs_f64() * 1000.0 / (iterations as f64);
    println!("profile_smoke: total={:?} avg_ms={:.3}", dur, avg_ms);

    // cleanup
    let _ = std::env::remove_var("BK_LEVEL");
}
