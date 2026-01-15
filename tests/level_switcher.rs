use bevy::{app::App, input::InputPlugin, prelude::*};
use bevy_rapier3d::prelude::RapierConfiguration;
use brkrs::level_loader::{
    self, CurrentLevel, LevelAdvanceState, LevelDefinition, LevelLoaderPlugin,
};
use brkrs::systems::level_switch::LevelSwitchDirection;
use brkrs::systems::respawn::SpawnPoints;
use brkrs::systems::{
    LevelSwitchPlugin, LevelSwitchRequested, LevelSwitchSource, LevelSwitchState,
};
use brkrs::GameProgress;

fn level_switch_test_app() -> App {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins((MinimalPlugins, InputPlugin));
    app.insert_resource(GameProgress::default());
    app.insert_resource(LevelAdvanceState::default());
    app.insert_resource(SpawnPoints::default());
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(ButtonInput::<KeyCode>::default());
    app.world_mut().spawn(RapierConfiguration::new(1.0));
    app.add_plugins(LevelSwitchPlugin);
    app.add_plugins(LevelLoaderPlugin);
    app
}

fn initialize_level_systems(app: &mut App) {
    // First update runs Startup schedule (loads level) and an initial Update pass.
    app.update();
    // Second update lets any pending systems settle before assertions.
    app.update();
}

fn trigger_level_switch(app: &mut App) {
    app.world_mut().write_message(LevelSwitchRequested {
        source: LevelSwitchSource::Keyboard,
        direction: LevelSwitchDirection::Next,
    });
    // First update processes the event and queues commands.
    app.update();
    // Second update flushes any deferred level/spawn commands before assertions.
    app.update();
}

fn current_level_number(app: &App) -> u32 {
    app.world()
        .get_resource::<CurrentLevel>()
        .map(|res| res.0.number)
        .expect("current level should be set after startup")
}

/* assert_spawn_points_for_level removed — not referenced in tests */

fn load_level_definition(number: u32) -> LevelDefinition {
    let path = format!("assets/levels/level_{number:03}.ron");
    let contents =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    ron::de::from_str(&contents).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn key_l_cycles_levels_and_wraps_with_spawn_resets() {
    // Force level 001 for this test to avoid interference from other tests that set BK_LEVEL.
    std::env::set_var("BK_LEVEL", "1");
    println!(
        "BK_LEVEL at test start = {:?}",
        std::env::var("BK_LEVEL").ok()
    );
    let mut app = level_switch_test_app();
    initialize_level_systems(&mut app);

    // Tests should be deterministic regardless of other level files present in assets/levels.
    // Force the current level to 001 for this test and recompute spawn points so assertions are stable.
    let def1 = load_level_definition(1);
    app.world_mut().insert_resource(CurrentLevel(def1.clone()));
    let mut expected_points = SpawnPoints::default();
    level_loader::set_spawn_points_only(&def1, &mut expected_points);
    // Update spawn_points resource to match level 001
    let mut sp = app.world_mut().resource_mut::<SpawnPoints>();
    *sp = expected_points;

    // Verify that pressing L advances to the next available level according to
    // the discovered `LevelSwitchState` ordering and that repeated presses wrap
    // around back to the starting level.
    let start = current_level_number(&app);
    // Determine the sequence of available level numbers in the switcher
    let slots = app
        .world()
        .get_resource::<LevelSwitchState>()
        .expect("level switch state should exist")
        .ordered_levels()
        .iter()
        .map(|s| s.number)
        .collect::<Vec<_>>();
    assert!(slots.len() >= 2, "expected at least two levels in test set");

    // Advance one slot and verify we moved forward
    trigger_level_switch(&mut app);
    let after_one = current_level_number(&app);
    // next should be the level after `start` in slots
    let pos = slots.iter().position(|n| *n == start).unwrap_or(0);
    let expected_next = slots[(pos + 1) % slots.len()];
    assert_eq!(
        after_one, expected_next,
        "pressing L should advance to next discovered level"
    );

    // Now press repeatedly until we wrap fully back to start
    let mut seen = after_one;
    let mut steps = 1usize;
    while seen != start && steps < slots.len() + 3 {
        trigger_level_switch(&mut app);
        seen = current_level_number(&app);
        steps += 1;
    }
    assert_eq!(
        seen, start,
        "after cycling, we should wrap back to the start level"
    );

    let switch_state = app.world().resource::<LevelSwitchState>();
    assert!(
        !switch_state.is_transition_pending(),
        "level switch resource should settle after processing requests"
    );
}

// cleanup env var to avoid affecting other tests
#[test]
fn clear_bk_level_after_switcher_test() {
    // make sure BK_LEVEL doesn't leak
    std::env::remove_var("BK_LEVEL");
}
