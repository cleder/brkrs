use bevy::{app::App, prelude::*, MinimalPlugins};

fn test_app() -> (App, Entity) {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(brkrs::systems::scoring::ScoreState::default());
    app.insert_resource(brkrs::systems::scoring::ScoreMultiplierState::default());
    app.add_systems(
        Update,
        (
            brkrs::ui::score_display::update_score_display_system,
            brkrs::ui::score_display::update_multiplier_indicator_system,
        ),
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

fn indicator_test_app() -> (App, Entity) {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(brkrs::systems::scoring::ScoreState::default());
    app.insert_resource(brkrs::systems::scoring::ScoreMultiplierState::default());
    app.add_systems(
        Update,
        brkrs::ui::score_display::update_multiplier_indicator_system,
    );

    let entity = app
        .world_mut()
        .spawn((
            Text::new(""),
            TextFont::default(),
            TextColor(Color::WHITE),
            Visibility::Hidden,
            brkrs::ui::score_display::ScoreMultiplierIndicatorUi,
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

    app.update();

    let world = app.world();
    let text = world
        .get::<Text>(score_entity)
        .expect("score display entity should exist");

    assert_eq!(text.as_str(), "Score: 1234");
}

#[test]
fn multiplier_indicator_shows_active_multiplier_and_hides_at_one_x() {
    let (mut app, indicator_entity) = indicator_test_app();

    {
        let mut multiplier = app
            .world_mut()
            .resource_mut::<brkrs::systems::scoring::ScoreMultiplierState>();
        multiplier.factor = 3;
    }

    app.update();

    let world = app.world();
    let text = world
        .get::<Text>(indicator_entity)
        .expect("indicator entity should exist");
    let visibility = world
        .get::<Visibility>(indicator_entity)
        .expect("indicator visibility should exist");

    assert_eq!(text.as_str(), "x3");
    assert_eq!(*visibility, Visibility::Inherited);

    let _ = world;

    {
        let mut multiplier = app
            .world_mut()
            .resource_mut::<brkrs::systems::scoring::ScoreMultiplierState>();
        multiplier.factor = 1;
    }

    app.update();

    let world = app.world();
    let visibility = world
        .get::<Visibility>(indicator_entity)
        .expect("indicator visibility should exist");
    assert_eq!(*visibility, Visibility::Hidden);
}

#[test]
fn multiplier_indicator_stays_stable_without_resource_changes() {
    let (mut app, indicator_entity) = indicator_test_app();

    {
        let mut multiplier = app
            .world_mut()
            .resource_mut::<brkrs::systems::scoring::ScoreMultiplierState>();
        multiplier.factor = 2;
    }

    app.update();
    app.update();

    let world = app.world_mut();
    let mut query = world.query_filtered::<(
        bevy::ecs::change_detection::Ref<Text>,
        bevy::ecs::change_detection::Ref<Visibility>,
    ), With<brkrs::ui::score_display::ScoreMultiplierIndicatorUi>>();
    let (text, visibility) = query
        .get(world, indicator_entity)
        .expect("indicator entity should exist");

    assert_eq!(text.as_str(), "x2");
    assert_eq!(*visibility, Visibility::Inherited);
    assert!(!text.is_changed());
    assert!(!visibility.is_changed());
}
