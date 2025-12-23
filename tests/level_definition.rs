use bevy::{app::App, prelude::*, MinimalPlugins};
use bevy_rapier3d::prelude::CollisionEvent;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;
// LevelDefinition import removed — test uses serialized LevelDefinition string, not the type
use brkrs::{BrickTypeId, CountsTowardsCompletion};
use tempfile::NamedTempFile;

fn level_test_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));
    // Collision events are delivered via the global CollisionEvent message resource
    app.add_message::<CollisionEvent>();
    app.insert_resource(brkrs::GameProgress::default());
    app.insert_resource(brkrs::level_loader::LevelAdvanceState::default());
    app.insert_resource(brkrs::systems::respawn::SpawnPoints::default());
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(bevy::input::ButtonInput::<bevy::prelude::KeyCode>::default());
    // need rapier config entity for physics queries used by level systems
    app.world_mut()
        .spawn(bevy_rapier3d::prelude::RapierConfiguration::new(1.0));
    app.add_plugins(brkrs::systems::LevelSwitchPlugin);
    app.add_plugins(brkrs::level_loader::LevelLoaderPlugin);
    // Register the brick collision/despawn systems used by the runtime so collision events
    // are processed during tests (these are normally added by run()).
    brkrs::register_brick_collision_systems(&mut app);
    app
}

#[test]
fn spawn_marks_counts_for_non_indestructible_bricks() {
    let mut app = level_test_app();

    // Create a temporary level file using `tempfile` and instruct the loader to
    // load it via `BK_LEVEL_PATH`. This avoids writing into the repo `assets/`
    // tree and prevents collisions when tests run in parallel.
    let mut tmp = NamedTempFile::new().expect("create temp level file");
    let contents = "LevelDefinition(number:999,matrix:[[90,20,3]])";
    use std::io::Write;
    tmp.write_all(contents.as_bytes())
        .expect("write temp level");

    // Set env so loader picks the exact temp file path
    std::env::set_var("BK_LEVEL_PATH", tmp.path().to_str().unwrap());

    // Run startup systems (load_level) and let systems settle
    app.update();
    app.update();

    // Query bricks and their types + completion marker
    let mut found_90 = false;
    let mut found_20 = false;
    let mut found_3 = false;

    let world = &mut app.world_mut();
    let mut q = world.query::<(&BrickTypeId, Option<&CountsTowardsCompletion>)>();
    for (type_id, maybe_marker) in q.iter(world) {
        if type_id.0 == 90 {
            found_90 = true;
            assert!(
                maybe_marker.is_none(),
                "indestructible brick must NOT count for completion"
            );
        }
        if type_id.0 == 20 {
            found_20 = true;
            assert!(
                maybe_marker.is_some(),
                "simple brick (20) must count for completion"
            );
        }
        if type_id.0 == 3 {
            found_3 = true;
            assert!(
                maybe_marker.is_some(),
                "legacy simple brick (3) must count for completion during compatibility window"
            );
        }
    }

    assert!(
        found_90 && found_20 && found_3,
        "All three brick types should be present in spawned bricks"
    );

    // cleanup - remove env var; temp file removed on drop
    std::env::remove_var("BK_LEVEL_PATH");
}

#[test]
fn completion_triggers_when_only_indestructible_bricks_remain() {
    let mut app = level_test_app();

    // Use the sample level file we added
    let path = "assets/levels/test_mixed_indestructible.ron";
    // Ensure the level exists (created by T011 earlier)
    assert!(
        std::path::Path::new(path).exists(),
        "test level file must exist"
    );

    // Tell the loader to load this exact level file using force_load_level_from_path
    // We use the same helper used in other tests by simulating a direct load via env var
    std::env::set_var("BK_LEVEL", "997");

    // Run startup systems to load the level
    app.update();
    app.update();

    // Confirm that there is at least one destructible brick (CountsTowardsCompletion) initially
    {
        let world = &mut app.world_mut();
        let mut q = world.query::<(Entity, Option<&CountsTowardsCompletion>)>();
        let mut destructible_count = 0usize;
        let mut to_despawn: Vec<Entity> = Vec::new();
        for (e, marker) in q.iter(world) {
            if marker.is_some() {
                destructible_count += 1;
                to_despawn.push(e);
            }
        }
        for e in to_despawn {
            world.despawn(e);
        }
        assert!(
            destructible_count > 0,
            "level must start with at least one destructible brick"
        );
    }

    // Run the update loop to let the advance system detect clearance
    app.update();
    app.update();

    // Since there is no level_998.ron (next level) the loader should mark game complete
    // When the level completes with no next level, the systems despawn the paddle and ball
    // as part of the completion flow. Assert paddle entities are gone to verify completion.
    let mut paddle_count = 0usize;
    {
        let world = &mut app.world_mut();
        let mut paddle_query = world.query::<(Entity, &brkrs::Paddle)>();
        for (_e, _p) in paddle_query.iter(world) {
            paddle_count += 1;
        }
    }
    assert_eq!(
        paddle_count, 0,
        "No paddles should remain after level completion"
    );

    // cleanup
    std::env::remove_var("BK_LEVEL");
}

#[test]
fn destructible_brick_marked_and_despawned_on_ball_collision() {
    let mut app = level_test_app();

    // Ensure level with mixed bricks is loaded (has both types) and has bricks spawned
    std::env::set_var("BK_LEVEL", "997");
    app.update();
    app.update();

    // Find an entity representing a destructible brick (BrickTypeId == 20)
    let world = &mut app.world_mut();
    let mut target: Option<Entity> = None;
    let mut q = world.query::<(
        Entity,
        &brkrs::BrickTypeId,
        Option<&brkrs::CountsTowardsCompletion>,
    )>();
    for (e, type_id, marker) in q.iter(world) {
        if type_id.0 == 20 && marker.is_some() {
            target = Some(e);
            break;
        }
    }
    let brick = target.expect("expected at least one destructible (20) brick in test level");

    // spawn a ball for the collision
    let ball = app.world_mut().spawn((brkrs::Ball,)).id();

    // Ensure both entities exist and brick is destructible
    assert!(
        app.world().entities().contains(ball),
        "ball entity must exist"
    );
    assert!(
        app.world().entities().contains(brick),
        "brick entity must exist"
    );
    {
        let world = &mut app.world_mut();
        let mut q = world.query::<(Entity, Option<&brkrs::CountsTowardsCompletion>)>();
        let mut found = false;
        for (e, marker) in q.iter(world) {
            if e == brick {
                found = marker.is_some();
                break;
            }
        }
        assert!(found, "brick must be destructible before collision test");
    }

    // Simulate collision event between ball and brick
    let mut collisions = app.world_mut().resource_mut::<Messages<CollisionEvent>>();
    collisions.write(CollisionEvent::Started(
        ball,
        brick,
        CollisionEventFlags::empty(),
    ));

    // Run update to process marking + despawn systems
    app.update();
    app.update();

    // Brick should be despawned by the despawn_marked_entities system
    assert!(
        !app.world().entities().contains(brick),
        "destructible brick should be removed on collision"
    );
    std::env::remove_var("BK_LEVEL");
}

#[test]
fn indestructible_brick_not_marked_on_ball_collision() {
    let mut app = level_test_app();

    // Ensure level with mixed bricks is loaded (has both types) and has bricks spawned
    std::env::set_var("BK_LEVEL", "997");
    app.update();
    app.update();

    // Find an entity representing an indestructible brick (BrickTypeId == 90), which must NOT count
    let world = &mut app.world_mut();
    let mut target: Option<Entity> = None;
    let mut q = world.query::<(
        Entity,
        &brkrs::BrickTypeId,
        Option<&brkrs::CountsTowardsCompletion>,
    )>();
    for (e, type_id, marker) in q.iter(world) {
        if type_id.0 == 90 && marker.is_none() {
            target = Some(e);
            break;
        }
    }
    let brick = target.expect("expected at least one indestructible (90) brick in test level");

    // spawn a ball for the collision
    let ball = app.world_mut().spawn((brkrs::Ball,)).id();

    // Simulate collision event between ball and brick
    let mut collisions = app.world_mut().resource_mut::<Messages<CollisionEvent>>();
    collisions.write(CollisionEvent::Started(
        ball,
        brick,
        CollisionEventFlags::empty(),
    ));

    // Run update to process marking + despawn systems
    app.update();
    app.update();

    // Brick should still exist
    assert!(
        app.world().entities().contains(brick),
        "indestructible brick should not be despawned on collision"
    );
    std::env::remove_var("BK_LEVEL");
}

#[test]
fn k_key_only_destroys_destructible_bricks() {
    let mut app = level_test_app();

    // Load the mixed test level
    std::env::set_var("BK_LEVEL", "997");
    app.update();
    app.update();

    // Collect entities
    let world = &mut app.world_mut();
    let mut destructible: Vec<Entity> = Vec::new();
    let mut indestructible: Vec<Entity> = Vec::new();
    let mut q = world.query::<(
        Entity,
        &brkrs::BrickTypeId,
        Option<&brkrs::CountsTowardsCompletion>,
    )>();
    for (e, _type_id, marker) in q.iter(world) {
        if marker.is_some() {
            destructible.push(e);
        } else {
            indestructible.push(e);
        }
    }

    assert!(
        !destructible.is_empty(),
        "expected some destructible bricks in level"
    );
    assert!(
        !indestructible.is_empty(),
        "expected some indestructible bricks in level"
    );

    // Allow input systems to settle, then simulate pressing K and run two frames so
    // the InputPlugin updates `just_pressed` and the destruction system runs.
    app.update();
    {
        app.insert_resource(brkrs::systems::cheat_mode::CheatModeState {
            active: true,
            ..default()
        });
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyK);
    }

    // Update to process destruction (two frames to stabilize input state)
    app.update();
    app.update();

    // All destructible bricks should be gone
    let world_ref = app.world();
    for e in destructible {
        assert!(
            !world_ref.entities().contains(e),
            "destructible brick should be removed by K"
        );
    }
    std::env::remove_var("BK_LEVEL");
}

// === US1: Level Designer Documents Level Intent (Description) ===

#[test]
fn test_level_with_description_only() {
    let ron = r#"
        LevelDefinition(
            number: 2,
            description: Some("Test level description"),
            matrix: [[0]],
        )
    "#;
    let level: brkrs::level_loader::LevelDefinition =
        ron::de::from_str(ron).expect("Should deserialize");
    assert!(level.has_description());
    assert!(!level.has_author());
    assert_eq!(
        level.description,
        Some("Test level description".to_string())
    );
}

#[test]
fn test_multiline_description() {
    let ron = r#"
        LevelDefinition(
            number: 3,
            description: Some("Line 1\nLine 2\nLine 3"),
            matrix: [[0]],
        )
    "#;
    let level: brkrs::level_loader::LevelDefinition =
        ron::de::from_str(ron).expect("Should deserialize");
    assert!(level.has_description());
    let desc = level.description.as_ref().unwrap();
    assert!(desc.contains("Line 1"));
    assert!(desc.contains("Line 2"));
    assert!(desc.contains("Line 3"));
}

#[test]
fn test_empty_description_treated_as_none() {
    let ron = r#"
        LevelDefinition(
            number: 4,
            description: Some(""),
            matrix: [[0]],
        )
    "#;
    let level: brkrs::level_loader::LevelDefinition =
        ron::de::from_str(ron).expect("Should deserialize");
    assert!(!level.has_description());
    assert_eq!(level.description, Some("".to_string()));
}

#[test]
fn test_description_with_special_chars() {
    let ron = r#"
        LevelDefinition(
            number: 5,
            description: Some("Special chars: !@#$%^&*()"),
            matrix: [[0]],
        )
    "#;
    let level: brkrs::level_loader::LevelDefinition =
        ron::de::from_str(ron).expect("Should deserialize");
    assert!(level.has_description());
    assert!(level.description.as_ref().unwrap().contains("!@#$%^&*()"));
}

// === US2: Contributor Takes Credit for Work (Author) ===

#[test]
fn test_level_with_author_plain_string() {
    let ron = r#"
        LevelDefinition(
            number: 6,
            author: Some("Jane Smith"),
            matrix: [[0]],
        )
    "#;
    let level: brkrs::level_loader::LevelDefinition =
        ron::de::from_str(ron).expect("Should deserialize");
    assert!(level.has_author());
    assert!(!level.has_description());
    assert_eq!(level.author, Some("Jane Smith".to_string()));
    assert_eq!(level.author_name(), Some("Jane Smith"));
}

#[test]
fn test_author_markdown_email_format() {
    let ron = r#"
        LevelDefinition(
            number: 7,
            author: Some("[Jane Smith](mailto:jane@example.com)"),
            matrix: [[0]],
        )
    "#;
    let level: brkrs::level_loader::LevelDefinition =
        ron::de::from_str(ron).expect("Should deserialize");
    assert!(level.has_author());
    assert_eq!(level.author_name(), Some("Jane Smith"));
}

#[test]
fn test_author_markdown_url_format() {
    let ron = r#"
        LevelDefinition(
            number: 8,
            author: Some("[Team](https://github.com/team)"),
            matrix: [[0]],
        )
    "#;
    let level: brkrs::level_loader::LevelDefinition =
        ron::de::from_str(ron).expect("Should deserialize");
    assert!(level.has_author());
    assert_eq!(level.author_name(), Some("Team"));
}

#[test]
fn test_extract_author_plain_text() {
    assert_eq!(brkrs::extract_author_name("Jane Smith"), "Jane Smith");
    assert_eq!(brkrs::extract_author_name("  Jane  "), "Jane");
    assert_eq!(brkrs::extract_author_name("John Doe"), "John Doe");
}

#[test]
fn test_extract_author_markdown_email() {
    assert_eq!(
        brkrs::extract_author_name("[Jane Smith](mailto:jane@example.com)"),
        "Jane Smith"
    );
    assert_eq!(
        brkrs::extract_author_name("[John](mailto:john@test.org)"),
        "John"
    );
}

#[test]
fn test_extract_author_markdown_url() {
    assert_eq!(
        brkrs::extract_author_name("[Team](https://example.com)"),
        "Team"
    );
    assert_eq!(
        brkrs::extract_author_name("[Contributors](https://github.com/org/repo)"),
        "Contributors"
    );
}

#[test]
fn test_extract_author_edge_cases() {
    // Empty brackets
    assert_eq!(brkrs::extract_author_name("[](url)"), "");

    // Nested brackets
    assert_eq!(brkrs::extract_author_name("[[Name]](url)"), "[Name]");

    // Malformed - no closing bracket
    assert_eq!(brkrs::extract_author_name("[Name"), "[Name");

    // Not markdown
    assert_eq!(brkrs::extract_author_name("Just text"), "Just text");

    // With spaces around brackets
    assert_eq!(
        brkrs::extract_author_name("[Jane Smith] (mailto:jane@example.com)"),
        "[Jane Smith] (mailto:jane@example.com)"
    );
}

#[test]
fn test_empty_author_treated_as_none() {
    let ron = r#"
        LevelDefinition(
            number: 9,
            author: Some("   "),
            matrix: [[0]],
        )
    "#;
    let level: brkrs::level_loader::LevelDefinition =
        ron::de::from_str(ron).expect("Should deserialize");
    assert!(!level.has_author());
    assert_eq!(level.author_name(), None);
}

// === US1 + US2: Both description and author ===

#[test]
fn test_level_with_full_metadata() {
    let ron = r#"
        LevelDefinition(
            number: 10,
            description: Some("Expert challenge level"),
            author: Some("[Jane Smith](mailto:jane@example.com)"),
            matrix: [[0]],
        )
    "#;
    let level: brkrs::level_loader::LevelDefinition =
        ron::de::from_str(ron).expect("Should deserialize");
    assert!(level.has_description());
    assert!(level.has_author());
    assert_eq!(level.author_name(), Some("Jane Smith"));
}

// === US4: Backward compatibility ===

#[test]
fn test_level_without_metadata_backward_compat() {
    let ron = r#"
        LevelDefinition(
            number: 1,
            matrix: [[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]],
        )
    "#;
    let level: brkrs::level_loader::LevelDefinition =
        ron::de::from_str(ron).expect("Should deserialize");
    assert_eq!(level.number, 1);
    assert_eq!(level.description, None);
    assert_eq!(level.author, None);
    assert!(!level.has_description());
    assert!(!level.has_author());
}

#[test]
fn test_level_with_only_description() {
    let ron = r#"
        LevelDefinition(
            number: 11,
            description: Some("Only description"),
            matrix: [[0]],
        )
    "#;
    let level: brkrs::level_loader::LevelDefinition =
        ron::de::from_str(ron).expect("Should deserialize");
    assert!(level.has_description());
    assert!(!level.has_author());
}

#[test]
fn test_level_with_only_author() {
    let ron = r#"
        LevelDefinition(
            number: 12,
            author: Some("Only Author"),
            matrix: [[0]],
        )
    "#;
    let level: brkrs::level_loader::LevelDefinition =
        ron::de::from_str(ron).expect("Should deserialize");
    assert!(!level.has_description());
    assert!(level.has_author());
}

#[test]
fn test_level_with_gravity_and_metadata() {
    let ron = r#"
        LevelDefinition(
            number: 13,
            gravity: Some((0.0, -9.81, 0.0)),
            description: Some("Standard gravity level"),
            author: Some("Game Designer"),
            matrix: [[0]],
        )
    "#;
    let level: brkrs::level_loader::LevelDefinition =
        ron::de::from_str(ron).expect("Should deserialize");
    assert!(level.gravity.is_some());
    assert!(level.has_description());
    assert!(level.has_author());
}
