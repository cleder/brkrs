use bevy::{app::App, prelude::*, MinimalPlugins};
use bevy_rapier3d::prelude::CollisionEvent;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;
// LevelDefinition import removed — test uses serialized LevelDefinition string, not the type
use brkrs::{BrickTypeId, CountsTowardsCompletion};

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

    // Prepare a temporary level file under assets/levels using a per-process unique
    // numeric suffix to avoid collisions when tests run in parallel.
    let pid = std::process::id();
    // Keep index in the 900-989 range to avoid colliding with reserved test files.
    let level_index: u32 = 900 + (pid % 90);
    let path = format!("assets/levels/level_{:03}.ron", level_index);
    let contents = format!("LevelDefinition(number:{},matrix:[[90,20,3]])", level_index);
    std::fs::write(&path, contents).expect("write test level");

    // Set env so loader picks the test file
    std::env::set_var("BK_LEVEL", level_index.to_string());

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

    // cleanup
    let _ = std::fs::remove_file(path);
    std::env::remove_var("BK_LEVEL");
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
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyK);
    }

    // Update to process destruction (two frames to stabilize input state)
    app.update();
    app.update();

    // All destructible bricks should be gone; indestructible remain
    let world_ref = app.world();
    for e in destructible {
        assert!(
            !world_ref.entities().contains(e),
            "destructible brick should be removed by K"
        );
    }
    for e in indestructible {
        assert!(
            world_ref.entities().contains(e),
            "indestructible brick should remain after K"
        );
    }
    std::env::remove_var("BK_LEVEL");
}
