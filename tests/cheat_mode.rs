use bevy::prelude::*;
use bevy::MinimalPlugins;

use bevy::ecs::entity::Entity;
use brkrs::systems::audio::UiBeepEvent;
use brkrs::systems::cheat_mode::CheatModeState;
use brkrs::systems::level_switch::LevelSwitchSource;
use brkrs::systems::level_switch::{LevelSwitchDirection, LevelSwitchPlugin, LevelSwitchRequested};
use brkrs::ui::game_over_overlay::GameOverOverlay;

use brkrs::level_loader::LevelLoaderPlugin;
use brkrs::systems::audio::AudioPlugin;
use brkrs::GameProgress;

#[derive(Resource, Default)]
struct BeepCount(u32);

#[derive(Resource, Default)]
struct SwitchRequests(Vec<LevelSwitchRequested>);

// Reader-based capture for UiBeepEvent messages
fn capture_beep(
    mut reader: bevy::ecs::message::MessageReader<UiBeepEvent>,
    mut counter: ResMut<BeepCount>,
) {
    for _ in reader.read() {
        counter.0 += 1;
    }
}

// Reader-based capture for LevelSwitchRequested messages
fn capture_switch(
    mut reader: bevy::ecs::message::MessageReader<LevelSwitchRequested>,
    mut rec: ResMut<SwitchRequests>,
) {
    for ev in reader.read() {
        rec.0.push(*ev);
    }
}

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));
    app.insert_resource(GameProgress::default());
    app.insert_resource(ButtonInput::<KeyCode>::default());
    // Register plugins that provide messages/events. Add AudioPlugin first so
    // message types are registered before systems that take MessageWriter params.
    app.add_plugins(AudioPlugin);
    app.add_plugins(LevelSwitchPlugin);

    // Add systems to read message queues and record observations (capture between
    // LevelSwitch and LevelLoader so we see N/P requests before level loader consumes them)
    app.add_systems(Update, (capture_beep, capture_switch));

    app.add_plugins(LevelLoaderPlugin);

    // Provide minimal asset stores and level loader resources so LevelLoader systems can run in tests
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(brkrs::level_loader::LevelAdvanceState::default());
    app.insert_resource(brkrs::systems::respawn::SpawnPoints::default());
    // Pause resource required by run-conditions used by gameplay systems
    app.init_resource::<brkrs::pause::PauseState>();
    // Minimal scoring state used by cheat toggles
    app.init_resource::<brkrs::systems::scoring::ScoreState>();
    // CheatMode resource/plugin
    app.add_plugins(brkrs::systems::cheat_mode::CheatModePlugin);

    // Insert test resources
    app.init_resource::<BeepCount>();
    app.init_resource::<SwitchRequests>();
    app
}

#[test]
fn toggling_cheat_removes_game_over_overlay() {
    let mut app = test_app();

    // Spawn game-over overlay entity, set lives to 0, and assert initial state
    {
        let mut world = app.world_mut();
        world.spawn((GameOverOverlay,));
        // Simulate player has no lives left
        world.insert_resource(brkrs::systems::respawn::LivesState {
            lives_remaining: 0,
            on_last_life: true,
        });
        let mut q = world.query_filtered::<Entity, With<GameOverOverlay>>();
        assert!(
            q.iter(&world).next().is_some(),
            "GameOverOverlay should exist before toggle"
        );
        let lives = world.resource::<brkrs::systems::respawn::LivesState>();
        assert_eq!(
            lives.lives_remaining, 0,
            "Initial lives should be 0 before toggle"
        );
    }

    // Press G to toggle cheat
    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyG);
    }

    // Run one update to process toggle
    app.update();

    // GameOverOverlay should be removed and lives reset to 3
    {
        let mut world = app.world_mut();
        let mut q = world.query_filtered::<Entity, With<GameOverOverlay>>();
        assert!(
            q.iter(&world).next().is_none(),
            "GameOverOverlay should be removed when toggling cheat mode"
        );
        let lives = world.resource::<brkrs::systems::respawn::LivesState>();
        assert_eq!(
            lives.lives_remaining, 3,
            "Lives should be reset to 3 when toggling cheat mode"
        );
    }
}

// Blocked N/P behavior is covered by unit tests in `src/systems/level_switch.rs`.

fn load_level_definition(number: u32) -> brkrs::level_loader::LevelDefinition {
    let path = format!("assets/levels/level_{number:03}.ron");
    let contents =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    ron::de::from_str(&contents).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn n_and_p_allowed_when_cheat_active_no_beep() {
    let mut app = test_app();

    // Enable cheat mode explicitly
    {
        let mut cheat = app.world_mut().resource_mut::<CheatModeState>();
        cheat.active = true;
    }

    // Press N
    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyN);
    }
    app.update();
    app.update();

    // No beep expected for allowed action
    let beep = app.world().resource::<BeepCount>();
    assert_eq!(beep.0, 0, "Allowed actions should not emit beep");

    // Clear inputs
    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.clear();
    }
    app.update();

    // Press P
    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyP);
    }
    app.update();
    app.update();

    // No beep expected for allowed P
    let beep2 = app.world().resource::<BeepCount>();
    assert_eq!(beep2.0, 0, "Allowed actions should not emit beep");
}

// Blocked restart behavior is covered by unit tests in `src/level_loader.rs`.

#[test]
fn r_allowed_when_cheat_active_no_beep() {
    let mut app = test_app();
    // Enable cheat mode
    {
        let mut cheat = app.world_mut().resource_mut::<CheatModeState>();
        cheat.active = true;
    }

    // Press R
    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyR);
    }
    app.update();
    app.update();

    // No beep emitted for allowed restart
    let beep = app.world().resource::<BeepCount>();
    assert_eq!(beep.0, 0, "Allowed restart should not emit UI beep");
}
