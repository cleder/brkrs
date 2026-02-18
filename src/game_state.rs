use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::systems::game_state_transitions::{
    capture_deferred_level_change, check_fade_in_completion, check_fade_out_completion,
    despawn_hazards_on_fade_out, enter_level_transition, guard_invalid_state_transitions,
    handle_deferred_level_change, handle_life_loss_events, spawn_fade_in_overlay,
    spawn_fade_out_overlay, update_fade_overlay, DeferredLevelChange,
};
use crate::systems::ui::game_over::{despawn_game_over, handle_game_over_buttons, spawn_game_over};
use crate::systems::ui::main_menu::{despawn_main_menu, handle_main_menu_buttons, spawn_main_menu};
use bevy::state::state::{StateTransition, StateTransitionSystems};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    FadeOut,
    FadeIn,
    LevelTransition,
    GameOver,
}

#[derive(Resource, Debug, Clone)]
pub struct GameSession {
    pub current_level: u32,
    pub lives_remaining: u32,
    pub score: u32,
}

impl Default for GameSession {
    fn default() -> Self {
        Self {
            current_level: 1,
            lives_remaining: 3,
            score: 0,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub enum StateTransitionContext {
    LifeLoss,
    LevelChange { target_level: u32 },
    NewGame,
    ReturnToMenu,
}

pub struct GameStatesPlugin;

fn show_cursor_on_main_menu(mut cursor_q: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    for mut cursor_options in cursor_q.iter_mut() {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}

fn hide_cursor_on_playing(mut cursor_q: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    for mut cursor_options in cursor_q.iter_mut() {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<GameSession>()
            .init_resource::<DeferredLevelChange>()
            .add_systems(OnEnter(GameState::MainMenu), show_cursor_on_main_menu)
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(OnEnter(GameState::Playing), hide_cursor_on_playing)
            .add_systems(
                StateTransition,
                guard_invalid_state_transitions
                    .before(StateTransitionSystems::DependentTransitions),
            )
            .add_systems(
                Update,
                handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnEnter(GameState::GameOver), show_cursor_on_main_menu)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(OnExit(GameState::GameOver), despawn_game_over)
            .add_systems(
                Update,
                handle_game_over_buttons.run_if(in_state(GameState::GameOver)),
            )
            .add_systems(
                Update,
                handle_life_loss_events.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::FadeOut), spawn_fade_out_overlay)
            .add_systems(OnEnter(GameState::FadeOut), despawn_hazards_on_fade_out)
            .add_systems(OnEnter(GameState::FadeIn), spawn_fade_in_overlay)
            .add_systems(
                Update,
                update_fade_overlay
                    .run_if(in_state(GameState::FadeOut).or(in_state(GameState::FadeIn))),
            )
            .add_systems(
                Update,
                check_fade_out_completion.run_if(in_state(GameState::FadeOut)),
            )
            .add_systems(
                Update,
                check_fade_in_completion.run_if(in_state(GameState::FadeIn)),
            )
            // EC-003: Handle deferred level changes when resuming from pause
            .add_systems(
                Update,
                capture_deferred_level_change.run_if(in_state(GameState::Paused)),
            )
            .add_systems(
                Update,
                handle_deferred_level_change.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::LevelTransition), enter_level_transition);
    }
}
