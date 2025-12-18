use bevy::{app::App, prelude::*, MinimalPlugins};

#[test]
fn level_label_spawns_and_updates_on_level_started() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Provide a UiFonts resource so spawn doesn't bail out
    app.insert_resource(brkrs::ui::fonts::UiFonts {
        orbitron: Handle::default(),
    });

    // Register our spawn system and the observer
    app.add_systems(Update, brkrs::ui::level_label::spawn_level_label);
    app.insert_resource(brkrs::ui::level_label::AccessibilityAnnouncement::default());
    app.add_observer(brkrs::ui::level_label::on_level_started);

    // Run one frame so spawn_system can create the UI
    app.update();

    // There should be one LevelLabelText entity
    let world = app.world();
    // Find the LevelLabelText entity by scanning entities and checking for the marker component
    let mut found = None;
    for entity_ref in world.iter_entities() {
        let id = entity_ref.id();
        if world
            .get::<brkrs::ui::level_label::LevelLabelText>(id)
            .is_some()
        {
            found = Some(id);
            break;
        }
    }

    let entity = found.expect("Level label should be spawned");
    let text = world
        .get::<Text>(entity)
        .expect("Level label entity should have Text component");
    assert_eq!(text.as_str(), "Level");

    // Send a LevelStarted event via a trigger system and run a frame; observer should update text and announcement resource
    app.add_systems(Update, |mut commands: Commands| {
        commands.trigger(brkrs::systems::LevelStarted { level_index: 3 });
    });
    app.update();

    let world = app.world();
    let text = world
        .get::<Text>(entity)
        .expect("Level label entity should still exist");
    assert_eq!(text.as_str(), "Level 3");

    // Accessibility announcement resource should be updated
    let announcement = world
        .get_resource::<brkrs::ui::level_label::AccessibilityAnnouncement>()
        .expect("Announcement resource present");

    assert_eq!(announcement.last.as_deref(), Some("Level 3"));
}
