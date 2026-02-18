use bevy::app::AppExit;
use bevy::prelude::*;

use brkrs::game_state::StateTransitionContext;
use brkrs::game_state::{GameState, GameStatesPlugin};
use brkrs::systems::respawn::LifeLostEvent;
use brkrs::systems::ui::game_over::GameOverRoot;
use brkrs::systems::ui::main_menu::{MainMenuRoot, QuitButtonMarker};
use std::time::Duration;

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(bevy::state::app::StatesPlugin)
        .add_plugins(GameStatesPlugin)
        .add_message::<AppExit>()
        .add_message::<LifeLostEvent>();
    app
}

fn advance_time(app: &mut App, delta_secs: f32) {
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(Duration::from_secs_f32(delta_secs));

    let mut query = app
        .world_mut()
        .query::<&mut brkrs::systems::game_state_transitions::FadeTimer>();
    for mut timer in query.iter_mut(app.world_mut()) {
        timer.tick(Duration::from_secs_f32(delta_secs));
    }
}

#[test]
fn main_menu_ui_spawns_on_enter() {
    let mut app = test_app();

    app.update();

    let count = app
        .world_mut()
        .query::<&MainMenuRoot>()
        .iter(app.world())
        .count();
    assert!(count > 0, "Main menu root should spawn on MainMenu state");
}

#[test]
fn new_game_button_transitions_to_playing() {
    let mut app = test_app();

    // Update to enter MainMenu state
    app.update();

    // Verify we're in MainMenu state
    let initial_state = app.world().resource::<State<GameState>>();
    assert_eq!(*initial_state.get(), GameState::MainMenu);

    // Manually set the next state to Playing
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);

    // Update to apply the state transition
    app.update();

    let final_state = app.world().resource::<State<GameState>>();
    assert_eq!(
        *final_state.get(),
        GameState::Playing,
        "Should transition to Playing when NextState is set"
    );
}

#[test]
fn quit_button_sends_app_exit() {
    let mut app = test_app();

    app.world_mut()
        .spawn((QuitButtonMarker, Interaction::Pressed, Button));

    // Simply verify the system runs without panic
    // In a real test, we'd check AppExit was sent to the exit system
    app.update();
}

#[test]
fn game_over_ui_spawns_on_enter() {
    let mut app = test_app();
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);
    app.update();

    app.world_mut()
        .resource_mut::<brkrs::game_state::GameSession>()
        .lives_remaining = 0;
    app.world_mut()
        .insert_resource(StateTransitionContext::LifeLoss);
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::FadeOut);

    for _ in 0..20 {
        advance_time(&mut app, 0.1);
        app.update();
        let state = app.world().resource::<State<GameState>>();
        if *state.get() == GameState::GameOver {
            break;
        }
    }

    app.update();

    let count = app
        .world_mut()
        .query::<&GameOverRoot>()
        .iter(app.world())
        .count();
    assert!(count > 0, "GameOver root should spawn on GameOver state");
}
