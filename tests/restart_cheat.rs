use bevy::prelude::*;
use bevy::MinimalPlugins;

use brkrs::level_loader::LevelLoaderPlugin;
use brkrs::signals::UiBeep;
use brkrs::systems::audio::AudioPlugin;
use brkrs::systems::cheat_mode::CheatModePlugin;
use brkrs::systems::level_switch::LevelSwitchPlugin;
use brkrs::systems::respawn::LivesState;

#[derive(Resource, Default)]
struct BeepCount(u32);

fn capture_beeps(mut reader: bevy::ecs::message::MessageReader<UiBeep>, mut c: ResMut<BeepCount>) {
    for _ in reader.read() {
        c.0 += 1;
    }
}

fn make_app() -> App {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));

    // Register audio first so UiBeep is available, then level loader
    app.add_plugins(AudioPlugin);
    app.add_plugins(LevelLoaderPlugin);
    // Level loader expects LevelSwitchRequested message to exist; register level switch plugin
    app.add_plugins(LevelSwitchPlugin);

    // Minimal resources used by level loader
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(brkrs::level_loader::LevelAdvanceState::default());
    app.insert_resource(brkrs::systems::respawn::SpawnPoints::default());
    app.insert_resource(brkrs::GameProgress::default());
    app.init_resource::<brkrs::pause::PauseState>();
    // Scoring state required by cheat-mode toggle
    app.init_resource::<brkrs::systems::scoring::ScoreState>();
    // Capture beeps
    app.init_resource::<BeepCount>();
    app.add_systems(Update, capture_beeps);

    app
}

#[test]
fn restart_with_cheat_active_resets_lives_and_no_beep() {
    let mut app = make_app();
    // Add cheat plugin and LivesState
    app.add_plugins(CheatModePlugin);
    app.insert_resource(LivesState {
        lives_remaining: 1,
        on_last_life: false,
    });

    // Enable cheat mode
    {
        let mut cheat = app
            .world_mut()
            .resource_mut::<brkrs::systems::cheat_mode::CheatModeState>();
        cheat.active = true;
    }
    // Sanity: cheat state should be active
    let cheat_state = app
        .world()
        .resource::<brkrs::systems::cheat_mode::CheatModeState>();
    assert!(
        cheat_state.is_active(),
        "Cheat state must be active for this test"
    );

    // Press R
    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyR);
    }

    // Run two frames (producer + consumer)
    app.update();
    app.update();

    // Lives should be reset to 3 and no beep emitted
    let lives = app.world().resource::<LivesState>();
    assert_eq!(
        lives.lives_remaining, 3,
        "Restart with cheat should reset lives to 3"
    );

    let beeps = app.world().resource::<BeepCount>();
    assert_eq!(beeps.0, 0, "Restart with cheat should not emit a beep");
}

#[test]
fn restart_with_cheat_inactive_blocked_and_beeps() {
    let mut app = make_app();
    // Add cheat plugin and LivesState
    app.add_plugins(CheatModePlugin);
    app.insert_resource(LivesState {
        lives_remaining: 1,
        on_last_life: false,
    });

    // Ensure cheat mode is off
    {
        let mut cheat = app
            .world_mut()
            .resource_mut::<brkrs::systems::cheat_mode::CheatModeState>();
        cheat.active = false;
    }

    // Press R
    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyR);
    }

    // Run frames
    app.update();
    app.update();

    // Lives should remain unchanged and beep emitted
    let lives = app.world().resource::<LivesState>();
    assert_eq!(
        lives.lives_remaining, 1,
        "Blocked restart should not change lives"
    );

    let beeps = app.world().resource::<BeepCount>();
    assert!(beeps.0 >= 1, "Blocked restart should emit a UI beep");
}
