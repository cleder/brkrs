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
    let world = app.world_mut();
    // Find the LevelLabelText entity by scanning entities and checking for the marker component
    let mut found = None;
    for entity_ref in world.query::<EntityRef>().iter(world) {
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

    let world = app.world_mut();
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

#[test]
fn level_label_is_left_aligned() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.insert_resource(brkrs::ui::fonts::UiFonts {
        orbitron: Handle::default(),
    });

    app.add_systems(Update, brkrs::ui::level_label::spawn_level_label);
    app.update();

    // Find the LevelLabelRoot entity and assert its Node has left justification
    let world = app.world_mut();
    let mut root = None;
    for entity_ref in world.query::<EntityRef>().iter(world) {
        let id = entity_ref.id();
        if world
            .get::<brkrs::ui::level_label::LevelLabelRoot>(id)
            .is_some()
        {
            root = Some(id);
            break;
        }
    }

    let root = root.expect("LevelLabelRoot should be spawned");
    let node = world
        .get::<Node>(root)
        .expect("Root should have Node component");
    assert_eq!(node.justify_content, bevy::ui::JustifyContent::FlexStart);
}

#[test]
fn spawn_shows_current_level_if_present() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.insert_resource(brkrs::ui::fonts::UiFonts {
        orbitron: Handle::default(),
    });

    // Insert CurrentLevel before spawn
    app.insert_resource(brkrs::level_loader::CurrentLevel(
        brkrs::level_loader::LevelDefinition {
            number: 5,
            gravity: None,
            matrix: vec![vec![]],
            #[cfg(feature = "texture_manifest")]
            presentation: None,
            description: None,
            author: None,
        },
    ));

    app.add_systems(Update, brkrs::ui::level_label::spawn_level_label);

    app.update();

    // Find spawned level label and assert it shows the level number
    let world = app.world_mut();
    let mut found = None;
    for entity_ref in world.query::<EntityRef>().iter(world) {
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
    assert_eq!(text.as_str(), "Level 5");
}

#[test]
fn updates_when_current_level_changes() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.insert_resource(brkrs::ui::fonts::UiFonts {
        orbitron: Handle::default(),
    });
    app.add_systems(Update, brkrs::ui::level_label::spawn_level_label);
    app.add_systems(Update, brkrs::ui::level_label::sync_with_current_level);

    app.update();

    // Insert CurrentLevel after spawn and verify the HUD updates
    app.insert_resource(brkrs::level_loader::CurrentLevel(
        brkrs::level_loader::LevelDefinition {
            number: 7,
            gravity: None,
            matrix: vec![vec![]],
            #[cfg(feature = "texture_manifest")]
            presentation: None,
            description: None,
            author: None,
        },
    ));

    app.update();

    let world = app.world_mut();
    // find the text entity
    let mut found = None;
    for entity_ref in world.query::<EntityRef>().iter(world) {
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
    assert_eq!(text.as_str(), "Level 7");
}
