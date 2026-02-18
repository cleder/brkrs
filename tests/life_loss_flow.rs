use bevy::prelude::*;
use std::time::Duration;

use brkrs::game_state::{GameSession, GameState, GameStatesPlugin, StateTransitionContext};

fn setup_app(lives: u32) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(bevy::state::app::StatesPlugin)
        .add_plugins(GameStatesPlugin);
    app.world_mut()
        .resource_mut::<GameSession>()
        .lives_remaining = lives;
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
fn life_loss_with_lives_remaining_transitions_to_fade_in() {
    let mut app = setup_app(2);
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);
    app.update();
    app.world_mut()
        .insert_resource(StateTransitionContext::LifeLoss);
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::FadeOut);

    let mut saw_fade_in = false;
    for _ in 0..20 {
        advance_time(&mut app, 0.1);
        app.update();
        let state = app.world().resource::<State<GameState>>();
        if *state.get() == GameState::FadeIn {
            saw_fade_in = true;
            break;
        }
    }

    assert!(saw_fade_in, "expected FadeIn transition after life loss");
}

#[test]
fn life_loss_with_no_lives_transitions_to_game_over() {
    let mut app = setup_app(0);
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);
    app.update();
    app.world_mut()
        .insert_resource(StateTransitionContext::LifeLoss);
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::FadeOut);

    for _ in 0..20 {
        advance_time(&mut app, 0.1);
        app.update();
    }

    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::GameOver);
}
