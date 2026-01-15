use bevy::{app::App, prelude::*, MinimalPlugins};

fn test_app() -> (App, Entity) {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(brkrs::systems::scoring::ScoreState::default());
    app.add_systems(
        Update,
        brkrs::ui::score_display::update_score_display_system,
    );

    let entity = app
        .world_mut()
        .spawn((
            Text::new("Score: 0"),
            TextFont::default(),
            TextColor(Color::WHITE),
            brkrs::ui::score_display::ScoreDisplayUi,
        ))
        .id();

    (app, entity)
}

#[test]
fn score_display_updates_on_same_frame_as_score_change() {
    let (mut app, score_entity) = test_app();

    {
        let mut score = app
            .world_mut()
            .resource_mut::<brkrs::systems::scoring::ScoreState>();
        score.current_score = 1_234;
    }

    // Run one frame; UI should update in the same frame as the score mutation.
    app.update();

    let world = app.world();
    let text = world
        .get::<Text>(score_entity)
        .expect("score display entity should exist");

    assert_eq!(text.as_str(), "Score: 1234");
}
