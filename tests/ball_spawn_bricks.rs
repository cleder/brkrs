//! Integration tests for ball spawn bricks (Red 1/2/3) feature.
//!
//! Validates spawn/despawn behavior using BrickDestroyed messages and collision
//! dedupe flow where required.

use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{CollisionEvent, Velocity};
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use brkrs::signals::BrickDestroyed;
use brkrs::{Ball, Brick, EmittedBrickDestroyed};

#[path = "ball_spawn_bricks/fixtures.rs"]
mod fixtures;
use fixtures::{count_balls, setup_test_app, spawn_test_ball, spawn_test_brick};

fn write_brick_destroyed(app: &mut App, brick_type: u8, position: Vec3, triggering_ball: Entity) {
    let mut msgs = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();
    msgs.write(BrickDestroyed {
        brick_entity: Entity::PLACEHOLDER,
        brick_type,
        brick_position: position,
        destroyed_by: Some(triggering_ball),
    });
}

fn angle_deg(v: Vec3) -> f32 {
    v.z.atan2(v.x).to_degrees()
}

fn angle_delta_deg(a: f32, b: f32) -> f32 {
    let mut delta = (a - b).abs();
    if delta > 180.0 {
        delta = 360.0 - delta;
    }
    delta
}

// ===============================
// US1: Red 2 (index 38)
// ===============================

#[test]
fn red_2_spawns_one_additional_ball() {
    let mut app = setup_test_app();
    let ball = spawn_test_ball(&mut app, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.5));

    write_brick_destroyed(&mut app, 38, Vec3::new(5.0, 0.0, 3.0), ball);
    app.update();

    assert_eq!(count_balls(&mut app), 2, "Expected original + spawned ball");
}

#[test]
fn red_2_spawned_ball_has_inverse_velocity() {
    let mut app = setup_test_app();
    let base_velocity = Vec3::new(2.0, 0.0, -3.0);
    let ball = spawn_test_ball(&mut app, Vec3::ZERO, base_velocity);

    write_brick_destroyed(&mut app, 38, Vec3::new(1.0, 0.0, 2.0), ball);
    app.update();

    let world = app.world_mut();
    let mut query = world.query_filtered::<(&Velocity, Entity), With<Ball>>();
    let mut inverse_found = false;
    for (velocity, entity) in query.iter(world) {
        if entity == ball {
            continue;
        }
        let delta = velocity.linvel + base_velocity;
        if delta.length() < 0.0001 {
            inverse_found = true;
        }
    }
    assert!(inverse_found, "Spawned ball should have inverse velocity");
}

#[test]
fn red_2_spawns_from_multiple_balls() {
    let mut app = setup_test_app();
    let ball_a = spawn_test_ball(&mut app, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0));
    let _ball_b = spawn_test_ball(&mut app, Vec3::new(2.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let _ball_c = spawn_test_ball(
        &mut app,
        Vec3::new(-2.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    write_brick_destroyed(&mut app, 38, Vec3::new(3.0, 0.0, -1.0), ball_a);
    app.update();

    assert_eq!(count_balls(&mut app), 4, "Expected 4 balls after spawn");
}

#[test]
fn red_2_spawns_at_brick_position() {
    let mut app = setup_test_app();
    let ball = spawn_test_ball(&mut app, Vec3::ZERO, Vec3::new(1.0, 0.0, 1.0));
    let brick_pos = Vec3::new(5.0, 2.0, 3.0);

    write_brick_destroyed(&mut app, 38, brick_pos, ball);
    app.update();

    assert_eq!(count_balls(&mut app), 2, "Expected spawned ball to exist");

    let world = app.world_mut();
    let mut query = world.query_filtered::<&Transform, With<Ball>>();
    let mut found = false;
    for transform in query.iter(world) {
        if transform.translation.distance(brick_pos) < 0.0001 {
            found = true;
        }
    }
    assert!(found, "Spawned ball should appear at brick position");
}

#[test]
fn red_2_spawned_ball_persists_10_frames() {
    let mut app = setup_test_app();
    let ball = spawn_test_ball(&mut app, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0));
    let brick_pos = Vec3::new(2.0, 0.0, 2.0);

    write_brick_destroyed(&mut app, 38, brick_pos, ball);
    app.update();

    let world = app.world_mut();
    let mut query = world.query_filtered::<&Transform, With<Ball>>();
    let initial_positions: Vec<Vec3> = query.iter(world).map(|t| t.translation).collect();

    for _ in 0..10 {
        app.update();
    }

    let world = app.world_mut();
    let mut query = world.query_filtered::<&Transform, With<Ball>>();
    let later_positions: Vec<Vec3> = query.iter(world).map(|t| t.translation).collect();

    assert_eq!(initial_positions.len(), later_positions.len());
    let moved = initial_positions
        .iter()
        .zip(later_positions.iter())
        .any(|(a, b)| a.distance(*b) > 0.0001);
    assert!(moved, "Spawned ball should persist and move across frames");
}

#[test]
fn red_2_uses_brick_destroyed_message() {
    let mut app = setup_test_app();
    let ball = spawn_test_ball(&mut app, Vec3::ZERO, Vec3::new(0.5, 0.0, 0.5));

    assert_eq!(count_balls(&mut app), 1);
    write_brick_destroyed(&mut app, 38, Vec3::new(1.0, 0.0, 1.0), ball);
    app.update();

    assert_eq!(
        count_balls(&mut app),
        2,
        "BrickDestroyed message should trigger spawn"
    );
}

// ===============================
// US2: Red 3 (index 39)
// ===============================

#[test]
fn red_3_spawns_two_additional_balls() {
    let mut app = setup_test_app();
    let ball = spawn_test_ball(&mut app, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0));

    write_brick_destroyed(&mut app, 39, Vec3::new(1.0, 0.0, 1.0), ball);
    app.update();

    assert_eq!(
        count_balls(&mut app),
        3,
        "Expected 3 balls after Red 3 spawn"
    );
}

#[test]
fn red_3_spawns_y_shaped_pattern() {
    let mut app = setup_test_app();
    let base_velocity = Vec3::new(1.0, 0.0, 0.0);
    let ball = spawn_test_ball(&mut app, Vec3::ZERO, base_velocity);

    write_brick_destroyed(&mut app, 39, Vec3::new(3.0, 0.0, 3.0), ball);
    app.update();

    let base_angle = angle_deg(base_velocity);
    let world = app.world_mut();
    let mut query = world.query_filtered::<(&Velocity, Entity), With<Ball>>();
    let mut deltas = Vec::new();
    for (velocity, entity) in query.iter(world) {
        if entity == ball {
            continue;
        }
        deltas.push(angle_delta_deg(angle_deg(velocity.linvel), base_angle));
    }

    assert_eq!(deltas.len(), 2, "Expected two spawned balls");
    for delta in deltas {
        assert!(
            (delta - 37.5).abs() < 0.5,
            "Expected ~37.5° spread, got {delta}"
        );
    }

    // EDGE CASE: Near-zero velocity still spawns distinct balls
    let mut app = setup_test_app();
    let slow_velocity = Vec3::new(0.01, 0.0, 0.0);
    let ball = spawn_test_ball(&mut app, Vec3::ZERO, slow_velocity);
    write_brick_destroyed(&mut app, 39, Vec3::new(1.0, 0.0, 1.0), ball);
    app.update();

    let world = app.world_mut();
    let mut query = world.query_filtered::<(&Velocity, Entity), With<Ball>>();
    let speeds: Vec<f32> = query
        .iter(world)
        .filter(|(_, entity)| *entity != ball)
        .map(|(velocity, _)| velocity.linvel.length())
        .collect();
    assert_eq!(speeds.len(), 2);
    for speed in speeds {
        assert!((speed - slow_velocity.length()).abs() < 0.001);
    }
}

#[test]
fn red_3_spawns_from_multiple_balls() {
    let mut app = setup_test_app();
    let ball_a = spawn_test_ball(&mut app, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0));
    let _ball_b = spawn_test_ball(&mut app, Vec3::new(2.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));

    write_brick_destroyed(&mut app, 39, Vec3::new(2.0, 0.0, 2.0), ball_a);
    app.update();

    assert_eq!(
        count_balls(&mut app),
        4,
        "Expected 4 balls after Red 3 spawn"
    );
}

#[test]
fn red_3_spawns_once_per_destruction() {
    let mut app = setup_test_app();
    app.init_resource::<EmittedBrickDestroyed>();
    brkrs::register_brick_collision_systems(&mut app);

    let brick = spawn_test_brick(&mut app, 39, Vec3::new(5.0, 0.0, 5.0));
    let ball1 = spawn_test_ball(&mut app, Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let ball2 = spawn_test_ball(
        &mut app,
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    app.world_mut().write_message(CollisionEvent::Started(
        ball1,
        brick,
        CollisionEventFlags::empty(),
    ));
    app.world_mut().write_message(CollisionEvent::Started(
        ball2,
        brick,
        CollisionEventFlags::empty(),
    ));

    app.update();

    // Two new balls + two originals = 4 total
    assert_eq!(count_balls(&mut app), 4, "Expected single Red 3 spawn set");
}

#[test]
fn red_3_spawned_balls_persist_10_frames() {
    let mut app = setup_test_app();
    let ball = spawn_test_ball(&mut app, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0));

    write_brick_destroyed(&mut app, 39, Vec3::new(4.0, 0.0, 4.0), ball);
    app.update();

    assert_eq!(count_balls(&mut app), 3, "Expected spawned balls to exist");

    for _ in 0..10 {
        app.update();
    }

    assert_eq!(
        count_balls(&mut app),
        3,
        "Spawned balls should persist across frames"
    );
}

// ===============================
// US3: Red 1 (index 37)
// ===============================

#[test]
fn red_1_despawns_all_except_triggering() {
    let mut app = setup_test_app();
    let triggering_ball = spawn_test_ball(&mut app, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0));
    let _ball_b = spawn_test_ball(&mut app, Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let _ball_c = spawn_test_ball(
        &mut app,
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    let _ball_d = spawn_test_ball(&mut app, Vec3::new(2.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let _ball_e = spawn_test_ball(
        &mut app,
        Vec3::new(-2.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    write_brick_destroyed(&mut app, 37, Vec3::new(1.0, 0.0, 1.0), triggering_ball);
    app.update();

    assert_eq!(count_balls(&mut app), 1);
    assert!(app.world().get_entity(triggering_ball).is_ok());
}

#[test]
fn red_1_with_single_ball_unchanged() {
    let mut app = setup_test_app();
    let triggering_ball = spawn_test_ball(&mut app, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0));

    write_brick_destroyed(&mut app, 37, Vec3::new(0.0, 0.0, 0.0), triggering_ball);
    app.update();

    assert_eq!(count_balls(&mut app), 1);
    assert!(app.world().get_entity(triggering_ball).is_ok());
}

#[test]
fn red_1_despawns_off_screen_balls() {
    let mut app = setup_test_app();
    let triggering_ball =
        spawn_test_ball(&mut app, Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let _ball_b = spawn_test_ball(
        &mut app,
        Vec3::new(100.0, 0.0, 100.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    let _ball_c = spawn_test_ball(
        &mut app,
        Vec3::new(-100.0, 0.0, -100.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    write_brick_destroyed(&mut app, 37, Vec3::new(1.0, 0.0, 1.0), triggering_ball);
    app.update();

    assert_eq!(count_balls(&mut app), 1);
    assert!(app.world().get_entity(triggering_ball).is_ok());
}

#[test]
fn red_1_no_respawn_after_despawn() {
    let mut app = setup_test_app();
    let triggering_ball = spawn_test_ball(&mut app, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0));
    let _ball_b = spawn_test_ball(&mut app, Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));

    write_brick_destroyed(&mut app, 37, Vec3::new(1.0, 0.0, 1.0), triggering_ball);
    app.update();

    for _ in 0..10 {
        app.update();
    }

    assert_eq!(count_balls(&mut app), 1);
    assert!(app.world().get_entity(triggering_ball).is_ok());
}

// ===============================
// Phase 6: Cross-cutting tests
// ===============================

#[test]
fn all_three_bricks_award_100_points() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_message::<BrickDestroyed>();
    app.insert_resource(brkrs::systems::scoring::ScoreState::default());
    app.add_systems(Update, brkrs::systems::scoring::award_points_system);

    let mut msgs = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();
    for brick_type in [37u8, 38u8, 39u8] {
        msgs.write(BrickDestroyed {
            brick_entity: Entity::PLACEHOLDER,
            brick_type,
            brick_position: Vec3::ZERO,
            destroyed_by: None,
        });
    }

    app.update();

    let score = app
        .world()
        .resource::<brkrs::systems::scoring::ScoreState>()
        .current_score;
    assert_eq!(score, 300, "Expected 100 points per brick");
}

#[test]
fn rapid_consecutive_triggers() {
    let mut app = setup_test_app();
    let ball = spawn_test_ball(&mut app, Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0));

    write_brick_destroyed(&mut app, 38, Vec3::new(1.0, 0.0, 1.0), ball);
    app.update();
    assert_eq!(count_balls(&mut app), 2);

    write_brick_destroyed(&mut app, 39, Vec3::new(2.0, 0.0, 2.0), ball);
    app.update();
    assert_eq!(count_balls(&mut app), 4);

    write_brick_destroyed(&mut app, 37, Vec3::new(3.0, 0.0, 3.0), ball);
    app.update();
    assert_eq!(count_balls(&mut app), 1);
}

#[test]
fn ball_spawn_bricks_count_toward_level_completion() {
    use std::fs;
    use std::path::PathBuf;
    use std::sync::Mutex;

    use brkrs::level_loader::{LevelAdvanceState, LevelLoaderPlugin};
    use brkrs::systems::level_switch::LevelSwitchPlugin;
    use brkrs::systems::LevelCompleted;
    use brkrs::{CountsTowardsCompletion, GameProgress};

    static LEVEL_ENV_LOCK: Mutex<()> = Mutex::new(());
    let _guard = LEVEL_ENV_LOCK.lock().expect("lock level env");

    let mut matrix = vec![vec![0u8; 20]; 20];
    matrix[0][0] = 37;
    matrix[0][1] = 38;
    matrix[0][2] = 39;

    let rows: Vec<String> = matrix
        .iter()
        .map(|row| {
            let row_str = row
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{row_str}]")
        })
        .collect();
    let matrix_ron = format!("[{}]", rows.join(", "));
    let ron = format!(
        "LevelDefinition(number: 999, gravity: None, matrix: {matrix_ron}, description: None, author: None)"
    );
    let mut path = PathBuf::from(std::env::temp_dir());
    path.push("brkrs_ball_spawn_level_999.ron");
    fs::write(&path, ron).expect("write temp level");

    let prev_level_path = std::env::var("BK_LEVEL_PATH").ok();
    std::env::set_var("BK_LEVEL_PATH", &path);

    #[derive(Resource, Default)]
    struct CompletedCount(u32);

    fn capture_completed(trigger: On<LevelCompleted>, mut count: ResMut<CompletedCount>) {
        let _event = trigger.event();
        count.0 += 1;
    }

    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins(LevelSwitchPlugin);
    app.add_plugins(LevelLoaderPlugin);
    app.add_observer(capture_completed);
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(GameProgress::default());
    app.insert_resource(LevelAdvanceState::default());
    app.insert_resource(brkrs::systems::respawn::SpawnPoints::default());
    app.init_resource::<CompletedCount>();
    app.init_resource::<brkrs::pause::PauseState>();
    app.init_resource::<brkrs::systems::scoring::ScoreState>();

    app.update(); // run startup: load and spawn level

    // Ensure bricks spawned and counted
    let brick_entities: Vec<Entity> = {
        let world = app.world_mut();
        let mut counts_query =
            world.query_filtered::<Entity, (With<Brick>, With<CountsTowardsCompletion>)>();
        counts_query.iter(world).collect()
    };
    assert!(
        !brick_entities.is_empty(),
        "Expected destructible bricks to be counted"
    );

    // Despawn all counted bricks
    for entity in brick_entities {
        app.world_mut().entity_mut(entity).despawn();
    }

    app.update(); // advance_level_when_cleared should fire

    let completed = app.world().resource::<CompletedCount>().0;
    assert!(completed > 0, "Expected level completion event");

    // Cleanup env var
    if let Some(prev) = prev_level_path {
        std::env::set_var("BK_LEVEL_PATH", prev);
    } else {
        std::env::remove_var("BK_LEVEL_PATH");
    }
    let _ = fs::remove_file(&path);
}
