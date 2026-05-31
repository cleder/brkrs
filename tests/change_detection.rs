use bevy::{app::App, ecs::change_detection::Ref, prelude::*, MinimalPlugins};

#[test]
fn multiplier_indicator_does_not_rewrite_when_state_is_unchanged() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(brkrs::systems::scoring::ScoreMultiplierState::default());
    app.add_systems(
        Update,
        brkrs::ui::score_display::update_multiplier_indicator_system,
    );

    let indicator_entity = app
        .world_mut()
        .spawn((
            Text::new(""),
            TextFont::default(),
            TextColor(Color::WHITE),
            Visibility::Hidden,
            brkrs::ui::score_display::ScoreMultiplierIndicatorUi,
        ))
        .id();

    {
        let mut multiplier = app
            .world_mut()
            .resource_mut::<brkrs::systems::scoring::ScoreMultiplierState>();
        multiplier.factor = 4;
    }

    // First frame applies the state change.
    app.update();
    // Second frame should not rewrite UI without a multiplier-state change.
    app.update();

    let world = app.world_mut();
    let mut query =
		world.query_filtered::<(Ref<Text>, Ref<Visibility>), With<brkrs::ui::score_display::ScoreMultiplierIndicatorUi>>();
    let (text, visibility) = query
        .get(world, indicator_entity)
        .expect("indicator entity should exist");

    assert_eq!(text.as_str(), "x4");
    assert_eq!(*visibility, Visibility::Inherited);
    assert!(
        !text.is_changed(),
        "text should not be rewritten when multiplier state is unchanged"
    );
    assert!(
        !visibility.is_changed(),
        "visibility should not be rewritten when multiplier state is unchanged"
    );
}
