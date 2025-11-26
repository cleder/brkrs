use bevy::{app::App, input::InputPlugin, prelude::*};
use bevy_rapier3d::prelude::RapierConfiguration;
use brkrs::level_loader::{
    self, CurrentLevel, LevelAdvanceState, LevelDefinition, LevelLoaderPlugin,
};
use brkrs::systems::respawn::SpawnPoints;
use brkrs::systems::{
    LevelSwitchPlugin, LevelSwitchRequested, LevelSwitchSource, LevelSwitchState,
};
use brkrs::GameProgress;

fn level_switch_test_app() -> App {
    let mut app = App::new();
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
    app.world_mut().send_event(LevelSwitchRequested {
        source: LevelSwitchSource::Keyboard,
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

fn assert_spawn_points_for_level(app: &App, level_number: u32) {
    let def = load_level_definition(level_number);
    let mut expected_points = SpawnPoints::default();
    level_loader::set_spawn_points_only(&def, &mut expected_points);
    let actual_points = app.world().resource::<SpawnPoints>();
    assert_eq!(
        actual_points.paddle, expected_points.paddle,
        "paddle spawn should match level {level_number:03} definition"
    );
    assert_eq!(
        actual_points.ball, expected_points.ball,
        "ball spawn should match level {level_number:03} definition"
    );
}

fn load_level_definition(number: u32) -> LevelDefinition {
    let path = format!("assets/levels/level_{number:03}.ron");
    let contents =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    ron::de::from_str(&contents).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn key_l_cycles_levels_and_wraps_with_spawn_resets() {
    let mut app = level_switch_test_app();
    initialize_level_systems(&mut app);

    assert_eq!(
        current_level_number(&app),
        1,
        "level 001 should load first by default"
    );
    assert_spawn_points_for_level(&app, 1);

    trigger_level_switch(&mut app);
    assert_eq!(
        current_level_number(&app),
        2,
        "pressing L should advance to level 002"
    );
    assert_spawn_points_for_level(&app, 2);

    trigger_level_switch(&mut app);
    assert_eq!(
        current_level_number(&app),
        1,
        "pressing L again should wrap back to level 001",
    );
    assert_spawn_points_for_level(&app, 1);

    let switch_state = app.world().resource::<LevelSwitchState>();
    assert!(
        !switch_state.is_transition_pending(),
        "level switch resource should settle after processing requests"
    );
}
