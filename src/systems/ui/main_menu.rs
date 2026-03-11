use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::game_state::{GameSession, GameState, StateTransitionContext};
use crate::systems::game_state_transitions::is_valid_transition;

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct NewGameButtonMarker;

#[derive(Component)]
pub struct QuitButtonMarker;

pub fn spawn_main_menu(
    mut commands: Commands,
    cursor_options: Option<Single<&mut CursorOptions, With<PrimaryWindow>>>,
) {
    if let Some(mut cursor_options) = cursor_options {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }

    // Main menu root container
    let root = commands
        .spawn((
            MainMenuRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            Name::new("MainMenuRoot"),
        ))
        .id();

    // New Game Button
    let new_game = commands
        .spawn((
            NewGameButtonMarker,
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            Name::new("NewGameButton"),
        ))
        .id();

    // Quit Button
    let quit = commands
        .spawn((
            QuitButtonMarker,
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            Name::new("QuitButton"),
        ))
        .id();

    // New Game Label
    let new_game_label = commands
        .spawn((
            Text::new("New Game"),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .id();

    // Quit Label
    let quit_label = commands
        .spawn((
            Text::new("Quit"),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .id();

    commands.entity(new_game).add_child(new_game_label);
    commands.entity(quit).add_child(quit_label);
    commands.entity(root).add_child(new_game);
    commands.entity(root).add_child(quit);
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_main_menu_buttons(
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut session: ResMut<GameSession>,
    lives_state: Option<ResMut<crate::systems::respawn::LivesState>>,
    score_state: Option<ResMut<crate::systems::scoring::ScoreState>>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    new_game_query: Query<&Interaction, (Changed<Interaction>, With<NewGameButtonMarker>)>,
    quit_query: Query<&Interaction, (Changed<Interaction>, With<QuitButtonMarker>)>,
) {
    let (mut start_requested, mut quit_requested) = if let Some(keyboard) = keyboard {
        (
            keyboard.just_pressed(KeyCode::Enter)
                || keyboard.just_pressed(KeyCode::NumpadEnter)
                || keyboard.just_pressed(KeyCode::Space),
            keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::KeyQ),
        )
    } else {
        (false, false)
    };

    for interaction in new_game_query.iter() {
        if *interaction == Interaction::Pressed {
            start_requested = true;
        }
    }

    for interaction in quit_query.iter() {
        if *interaction == Interaction::Pressed {
            quit_requested = true;
        }
    }

    // Prioritize quit over start to avoid ambiguous state when both are requested
    if quit_requested {
        exit.write(AppExit::Success);
        return;
    }

    if start_requested && is_valid_transition(current_state.get(), &GameState::FadeOut) {
        // Starting a new game must reset canonical gameplay resources before transitioning.
        session.current_level = 1;
        session.lives_remaining = 3;
        session.score = 0;

        match lives_state {
            Some(mut lives_state) => {
                crate::systems::respawn::reset_lives(lives_state.as_mut());
            }
            None => {
                error!(
                    target: "game_state",
                    "LivesState resource missing when starting new game from MainMenu"
                );
            }
        }

        match score_state {
            Some(mut score_state) => {
                crate::systems::scoring::reset_score(&mut score_state);
            }
            None => {
                error!(
                    target: "game_state",
                    "ScoreState resource missing when starting new game from MainMenu"
                );
            }
        }

        let target_level = session.current_level;
        info!(
            target: "game_state",
            "Starting new game: transitioning from MainMenu to FadeOut, target_level={}",
            target_level
        );
        commands.insert_resource(StateTransitionContext::LevelChange { target_level });
        next_state.set(GameState::FadeOut);
    }
}
