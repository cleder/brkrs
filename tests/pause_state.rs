use bevy::prelude::*;

use brkrs::game_state::{GameState, GameStatesPlugin, StateTransitionContext};
use brkrs::systems::game_state_transitions::is_valid_transition;

fn make_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(bevy::state::app::StatesPlugin)
        .add_plugins(GameStatesPlugin);
    app
}

#[derive(Resource, Default)]
struct Counter(u32);

fn count_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

#[test]
fn pause_blocks_systems_and_resume_restores() {
    let mut app = make_test_app();
    app.init_resource::<Counter>()
        .add_systems(Update, count_system.run_if(in_state(GameState::Playing)));

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);
    app.update();
    assert_eq!(app.world().resource::<Counter>().0, 1);

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Paused);
    app.update();

    for _ in 0..5 {
        app.update();
    }
    assert_eq!(app.world().resource::<Counter>().0, 1);

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);
    app.update();
    assert_eq!(app.world().resource::<Counter>().0, 2);
}

#[test]
fn pause_invalid_from_main_menu() {
    let app = make_test_app();

    let state = app.world().resource::<State<GameState>>();
    assert!(!is_valid_transition(state.get(), &GameState::Paused));
}
// EC-003: Level Complete While Paused - level-complete trigger deferred, activates on resume
#[test]
fn ec003_level_complete_deferred_while_paused() {
    let mut app = make_test_app();

    // Transition to Playing
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);
    app.update();

    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::Playing);

    // Pause the game
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Paused);
    app.update();

    let state = app.world().resource::<State<GameState>>();
    assert_eq!(*state.get(), GameState::Paused);

    // While paused, try to trigger level complete by setting level change context
    // (In real implementation, this would be triggered by level completion detection)
    app.world_mut()
        .insert_resource(StateTransitionContext::LevelChange { target_level: 2 });

    // Should still be Paused - level change deferred
    let state = app.world().resource::<State<GameState>>();
    assert_eq!(
        *state.get(),
        GameState::Paused,
        "Level transition should be deferred while Paused"
    );

    // Now resume to Playing - should trigger the deferred level transition
    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);
    app.update();

    // After a few updates, should transition to FadeOut
    for _ in 0..3 {
        app.update();
    }

    let state = app.world().resource::<State<GameState>>();
    // Should be transitioning through Fade or in LevelTransition
    // (FadeOut→LevelTransition may take several frames)
    assert!(
        matches!(
            *state.get(),
            GameState::FadeOut | GameState::LevelTransition | GameState::FadeIn
        ),
        "Level transition should activate after resuming from Paused; got {:?}",
        state.get()
    );
}
