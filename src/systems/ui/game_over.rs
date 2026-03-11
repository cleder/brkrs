use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::game_state::{GameSession, GameState, StateTransitionContext};
use crate::systems::game_state_transitions::is_valid_transition;

#[derive(Component)]
pub struct GameOverRoot;

#[derive(Component)]
pub struct ReturnToMenuButtonMarker;

#[derive(Component)]
pub struct NewGameButtonMarker;

pub fn spawn_game_over(
    mut commands: Commands,
    cursor_options: Option<Single<&mut CursorOptions, With<PrimaryWindow>>>,
) {
    if let Some(mut cursor_options) = cursor_options {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
    // Game over root container
    let root = commands
        .spawn((
            GameOverRoot,
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
            Name::new("GameOverRoot"),
        ))
        .id();

    // Return to Menu Button
    let return_button = commands
        .spawn((
            ReturnToMenuButtonMarker,
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
            Name::new("ReturnToMenuButton"),
        ))
        .id();

    // New Game Button
    let new_game_button = commands
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

    // Title
    let title = commands
        .spawn((
            Text::new("Game Over"),
            TextFont {
                font_size: 60.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .id();

    // Return to Menu Label
    let return_label = commands
        .spawn((
            Text::new("Return to Menu"),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
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

    commands.entity(return_button).add_child(return_label);
    commands.entity(new_game_button).add_child(new_game_label);
    commands.entity(root).add_child(title);
    commands.entity(root).add_child(return_button);
    commands.entity(root).add_child(new_game_button);
}

pub fn despawn_game_over(mut commands: Commands, query: Query<Entity, With<GameOverRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_game_over_buttons(
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut session: ResMut<GameSession>,
    lives_state: Option<ResMut<crate::systems::respawn::LivesState>>,
    score_state: Option<ResMut<crate::systems::scoring::ScoreState>>,
    mut commands: Commands,
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    return_query: Query<&Interaction, (Changed<Interaction>, With<ReturnToMenuButtonMarker>)>,
    new_game_query: Query<&Interaction, (Changed<Interaction>, With<NewGameButtonMarker>)>,
) {
    let (mut return_requested, mut new_game_requested) = if let Some(keyboard) = keyboard {
        (
            keyboard.just_pressed(KeyCode::Escape),
            keyboard.just_pressed(KeyCode::Enter)
                || keyboard.just_pressed(KeyCode::NumpadEnter)
                || keyboard.just_pressed(KeyCode::Space),
        )
    } else {
        (false, false)
    };

    for interaction in return_query.iter() {
        if *interaction == Interaction::Pressed
            && is_valid_transition(current_state.get(), &GameState::MainMenu)
        {
            return_requested = true;
        }
    }

    for interaction in new_game_query.iter() {
        if *interaction == Interaction::Pressed {
            new_game_requested = true;
        }
    }

    if return_requested && is_valid_transition(current_state.get(), &GameState::MainMenu) {
        commands.insert_resource(StateTransitionContext::ReturnToMenu);
        next_state.set(GameState::MainMenu);
        return;
    }

    if new_game_requested {
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
                    "LivesState resource missing when starting new game from GameOver"
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
                    "ScoreState resource missing when starting new game from GameOver"
                );
            }
        }

        if is_valid_transition(current_state.get(), &GameState::MainMenu) {
            commands.insert_resource(StateTransitionContext::NewGame);
            next_state.set(GameState::MainMenu);
        }
    }
}
