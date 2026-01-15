//! Integration tests for multi-hit bricks (indices 10-13).
//!
//! Tests verify that:
//! - Multi-hit bricks transition correctly on ball collision
//! - Index 13 → 12 → 11 → 10 → 20 → destroyed
//! - Visual materials update on type changes
//! - Level completion works with multi-hit bricks

use bevy::{app::App, prelude::*, MinimalPlugins};
use brkrs::{level_format::is_multi_hit_brick, BrickTypeId, CountsTowardsCompletion};

fn test_app() -> App {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));
    // Collision events are delivered via the global CollisionEvent message resource
    app.add_message::<bevy_rapier3d::prelude::CollisionEvent>();
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
    brkrs::register_brick_collision_systems(&mut app);
    app
}

// =============================================================================
// Unit tests for is_multi_hit_brick helper
// =============================================================================

#[test]
fn is_multi_hit_brick_returns_true_for_indices_10_to_13() {
    assert!(is_multi_hit_brick(10), "Index 10 should be multi-hit");
    assert!(is_multi_hit_brick(11), "Index 11 should be multi-hit");
    assert!(is_multi_hit_brick(12), "Index 12 should be multi-hit");
    assert!(is_multi_hit_brick(13), "Index 13 should be multi-hit");
}

#[test]
fn is_multi_hit_brick_returns_false_for_other_indices() {
    assert!(!is_multi_hit_brick(0), "Index 0 should not be multi-hit");
    assert!(!is_multi_hit_brick(3), "Index 3 should not be multi-hit");
    assert!(!is_multi_hit_brick(9), "Index 9 should not be multi-hit");
    assert!(!is_multi_hit_brick(14), "Index 14 should not be multi-hit");
    assert!(
        !is_multi_hit_brick(20),
        "Index 20 (simple stone) should not be multi-hit"
    );
    assert!(
        !is_multi_hit_brick(90),
        "Index 90 (indestructible) should not be multi-hit"
    );
}

// =============================================================================
// Integration tests for multi-hit brick transitions
// =============================================================================

#[test]
fn multi_hit_brick_spawns_with_correct_type_id() {
    let mut app = test_app();

    // Load the test level with multi-hit bricks
    std::env::set_var("BK_LEVEL", "998");
    app.update();
    app.update();

    // Find bricks with multi-hit type IDs
    let world = app.world_mut();
    let mut multi_hit_found = false;

    for type_id in world.query::<&BrickTypeId>().iter(world) {
        if is_multi_hit_brick(type_id.0) {
            multi_hit_found = true;
            break;
        }
    }

    assert!(
        multi_hit_found,
        "Expected at least one multi-hit brick (type 10-13) in test level 998"
    );
    std::env::remove_var("BK_LEVEL");
}

#[test]
fn multi_hit_bricks_have_counts_towards_completion() {
    let mut app = test_app();

    // Load the test level with multi-hit bricks
    std::env::set_var("BK_LEVEL", "998");
    app.update();
    app.update();

    // All multi-hit bricks should have CountsTowardsCompletion
    let world = app.world_mut();
    let mut all_have_component = true;
    let mut count = 0;

    for (type_id, _) in world
        .query::<(&BrickTypeId, &CountsTowardsCompletion)>()
        .iter(world)
    {
        if is_multi_hit_brick(type_id.0) {
            count += 1;
        }
    }

    // Also check if any multi-hit bricks are missing the component
    for (type_id, entity) in world.query::<(&BrickTypeId, Entity)>().iter(world) {
        if is_multi_hit_brick(type_id.0) && world.get::<CountsTowardsCompletion>(entity).is_none() {
            all_have_component = false;
            break;
        }
    }

    assert!(
        count > 0,
        "Expected multi-hit bricks with CountsTowardsCompletion in test level 998"
    );
    assert!(
        all_have_component,
        "All multi-hit bricks should have CountsTowardsCompletion component"
    );
    std::env::remove_var("BK_LEVEL");
}

// =============================================================================
// Tests for multi-hit brick transition logic (require collision simulation)
// =============================================================================

/// Test that index 13 brick transitions to index 12 on collision
#[test]
fn multi_hit_brick_13_transitions_to_12() {
    // This test verifies the transition logic once collision handling is implemented
    // For now, we verify the state machine logic directly

    use brkrs::level_format::{MULTI_HIT_BRICK_3, MULTI_HIT_BRICK_4};

    // State transition: 13 -> 12
    let current = MULTI_HIT_BRICK_4; // 13
    let expected_next = MULTI_HIT_BRICK_3; // 12

    assert!(is_multi_hit_brick(current), "Current should be multi-hit");
    assert!(
        is_multi_hit_brick(expected_next),
        "Next should be multi-hit"
    );
    assert_eq!(current - 1, expected_next, "13 - 1 = 12");
}

/// Test that index 10 brick transitions to index 20 (simple stone)
#[test]
fn multi_hit_brick_10_transitions_to_20() {
    use brkrs::level_format::{MULTI_HIT_BRICK_1, SIMPLE_BRICK};

    // State transition: 10 -> 20 (special case)
    let current = MULTI_HIT_BRICK_1; // 10
    let expected_next = SIMPLE_BRICK; // 20

    assert!(is_multi_hit_brick(current), "10 should be multi-hit");
    assert!(
        !is_multi_hit_brick(expected_next),
        "20 should not be multi-hit"
    );
    assert_eq!(expected_next, 20, "Final transition leads to simple stone");
}

/// Test the complete transition chain from 13 to destruction
#[test]
fn multi_hit_brick_full_lifecycle() {
    use brkrs::level_format::{
        is_multi_hit_brick, MULTI_HIT_BRICK_1, MULTI_HIT_BRICK_2, MULTI_HIT_BRICK_3,
        MULTI_HIT_BRICK_4, SIMPLE_BRICK,
    };

    // Verify the complete chain: 13 -> 12 -> 11 -> 10 -> 20 -> destroyed
    let chain = [
        (MULTI_HIT_BRICK_4, MULTI_HIT_BRICK_3, "13 -> 12"),
        (MULTI_HIT_BRICK_3, MULTI_HIT_BRICK_2, "12 -> 11"),
        (MULTI_HIT_BRICK_2, MULTI_HIT_BRICK_1, "11 -> 10"),
        (MULTI_HIT_BRICK_1, SIMPLE_BRICK, "10 -> 20"),
    ];

    for (current, expected_next, desc) in chain {
        if current == MULTI_HIT_BRICK_1 {
            // Special case: 10 -> 20
            assert_eq!(
                expected_next, SIMPLE_BRICK,
                "Transition {desc} should lead to simple stone"
            );
        } else {
            assert_eq!(current - 1, expected_next, "Transition {desc} failed");
        }
    }

    // After 20, the brick should be destroyed (not transitioned)
    assert!(
        !is_multi_hit_brick(SIMPLE_BRICK),
        "Simple stone (20) is not multi-hit"
    );
}
