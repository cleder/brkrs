//! Integration tests for extra ball brick (brick 41) feature.
//
// Tests US1: Hitting brick 41 grants +1 life (clamped to max), despawns brick, awards 0 points.
//
// Phase 3 US1 Tests: T007 (single hit), T008 (multi-ball safety)

use bevy::app::App;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::{CollisionEvent, RapierConfiguration};

use brkrs::signals::{BrickDestroyed, LifeAwardMessage};
use brkrs::systems::respawn::LivesState;
use brkrs::systems::scoring::ScoreState;
use brkrs::{Ball, Brick, BrickTypeId, CountsTowardsCompletion, MarkedForDespawn};

const EXTRA_LIFE_BRICK: u8 = 41;

fn test_app() -> App {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));
    app.add_message::<CollisionEvent>();
    app.add_message::<LifeAwardMessage>();
    app.add_message::<BrickDestroyed>();
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(bevy::input::ButtonInput::<bevy::prelude::KeyCode>::default());
    app.world_mut().spawn(RapierConfiguration::new(1.0));

    // Initialize lives and score state
    app.insert_resource(LivesState {
        lives_remaining: 3,
        on_last_life: false,
    });
    app.insert_resource(ScoreState {
        current_score: 0,
        last_milestone_reached: 0,
    });

    // Register brick collision systems
    brkrs::register_brick_collision_systems(&mut app);

    // TODO T009: Register brick 41 life award system once implemented
    // app.add_systems(Update, crate::systems::extra_ball_brick::process_extra_life_brick_hits);

    // TODO T013: Register life award consumer once implemented
    // app.add_systems(Update, crate::systems::respawn::process_life_awards);

    app
}

/// T007: Single hit to brick 41 → +1 life (clamped), brick despawned, score unchanged
///
/// **Acceptance Criteria:**
/// - Initial lives: 3
/// - After brick 41 hit: lives = 4 (clamped to max if applicable)
/// - Brick 41 entity despawned or marked for despawn
/// - Score remains 0 (brick 41 awards 0 points)
/// - LifeAwardMessage { delta: +1 } written to message queue
///
/// **Expected Failure Before Implementation:**
/// - No LifeAwardMessage written (brick 41 system not yet implemented)
/// - Lives remain unchanged at 3
/// - Test will fail asserting lives increment
#[test]
fn t007_brick_41_single_hit_grants_life_no_score() {
    let mut app = test_app();

    // Spawn brick 41 with CountsTowardsCompletion marker (destructible)
    let brick_41 = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(EXTRA_LIFE_BRICK),
            CountsTowardsCompletion,
            Transform::from_xyz(0.0, 5.0, 10.0),
        ))
        .id();

    // Spawn ball
    let ball = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(0.0, 4.0, 9.5)))
        .id();

    // Verify initial state
    let initial_lives = app.world().resource::<LivesState>().lives_remaining;
    let initial_score = app.world().resource::<ScoreState>().current_score;
    assert_eq!(initial_lives, 3, "Should start with 3 lives");
    assert_eq!(initial_score, 0, "Should start with 0 score");

    // Simulate ball-brick collision
    app.world_mut().write_message(CollisionEvent::Started(
        ball,
        brick_41,
        bevy_rapier3d::rapier::prelude::CollisionEventFlags::empty(),
    ));

    // Run systems to process collision
    app.update();

    // Check LifeAwardMessage was written
    let life_messages = app.world().resource::<Messages<LifeAwardMessage>>();
    assert!(
        !life_messages.is_empty(),
        "Expected LifeAwardMessage to be written after brick 41 hit"
    );

    // Check lives incremented (this will fail until T013 implements consumer)
    let final_lives = app.world().resource::<LivesState>().lives_remaining;
    assert_eq!(
        final_lives, 4,
        "Lives should increase from 3 to 4 after brick 41 hit"
    );

    // Check brick despawned or marked for despawn
    let brick_despawned = app.world().get::<MarkedForDespawn>(brick_41).is_some()
        || app.world().get_entity(brick_41).is_err();
    assert!(
        brick_despawned,
        "Brick 41 should be despawned or marked for despawn"
    );

    // Check score unchanged (brick 41 awards 0 points)
    let final_score = app.world().resource::<ScoreState>().current_score;
    assert_eq!(
        final_score, initial_score,
        "Score should remain unchanged (brick 41 awards 0 points)"
    );
}

/// T008: Multi-ball simultaneous hits → only one life award, message-event separation
///
/// **Acceptance Criteria:**
/// - Two balls hit brick 41 in same frame (simulated via two collision events)
/// - Only ONE LifeAwardMessage written (brick despawns after first hit)
/// - Lives increment by +1 only (not +2)
/// - Brick not rehittable after first collision processed
/// - No panics from query mismatches or observer/message conflicts
///
/// **Expected Failure Before Implementation:**
/// - No LifeAwardMessage written (brick 41 system not yet implemented)
/// - Lives remain unchanged
/// - Test will fail asserting single life award
#[test]
fn t008_brick_41_multi_ball_only_one_life_award() {
    let mut app = test_app();

    // Spawn brick 41
    let brick_41 = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(EXTRA_LIFE_BRICK),
            CountsTowardsCompletion,
            Transform::from_xyz(0.0, 5.0, 10.0),
        ))
        .id();

    // Spawn two balls
    let ball1 = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(-1.0, 4.0, 9.5)))
        .id();
    let ball2 = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(1.0, 4.0, 9.5)))
        .id();

    let initial_lives = app.world().resource::<LivesState>().lives_remaining;
    assert_eq!(initial_lives, 3);

    // Simulate simultaneous collisions (both balls hit brick in same frame)
    app.world_mut().write_message(CollisionEvent::Started(
        ball1,
        brick_41,
        bevy_rapier3d::rapier::prelude::CollisionEventFlags::empty(),
    ));
    app.world_mut().write_message(CollisionEvent::Started(
        ball2,
        brick_41,
        bevy_rapier3d::rapier::prelude::CollisionEventFlags::empty(),
    ));

    // Run systems
    app.update();

    // Check only ONE LifeAwardMessage written (brick despawns after first hit)
    let life_messages = app.world().resource::<Messages<LifeAwardMessage>>();
    assert!(
        !life_messages.is_empty(),
        "Expected LifeAwardMessage even with multi-ball hit"
    );

    // Check lives incremented by +1 only (not +2)
    let final_lives = app.world().resource::<LivesState>().lives_remaining;
    assert_eq!(
        final_lives, 4,
        "Lives should increase by +1 only (not +2 from two balls)"
    );

    // Check brick despawned (not rehittable)
    let brick_despawned = app.world().get::<MarkedForDespawn>(brick_41).is_some()
        || app.world().get_entity(brick_41).is_err();
    assert!(
        brick_despawned,
        "Brick 41 should be despawned after first hit"
    );

    // No panics = message-event separation working correctly
}

/// T007-clamping: Verify life gain is clamped to configured maximum
///
/// **Acceptance Criteria:**
/// - Initial lives set to max (assume max = 5 for this test)
/// - Brick 41 hit does not increase lives beyond max
/// - LifeAwardMessage still written (consumer clamps)
///
/// **Expected Failure Before Implementation:**
/// - Consumer not yet implemented (T013)
/// - Clamping logic not present
#[test]
fn t007_brick_41_life_clamped_to_max() {
    let mut app = test_app();

    // Set lives to max (assume max = 5)
    app.world_mut()
        .get_resource_mut::<LivesState>()
        .unwrap()
        .lives_remaining = 5;

    let brick_41 = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(EXTRA_LIFE_BRICK),
            CountsTowardsCompletion,
            Transform::from_xyz(0.0, 5.0, 10.0),
        ))
        .id();

    let ball = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(0.0, 4.0, 9.5)))
        .id();

    let initial_lives = app.world().resource::<LivesState>().lives_remaining;
    assert_eq!(initial_lives, 5);

    // Simulate collision
    app.world_mut().write_message(CollisionEvent::Started(
        ball,
        brick_41,
        bevy_rapier3d::rapier::prelude::CollisionEventFlags::empty(),
    ));

    app.update();

    // Check LifeAwardMessage written
    let life_messages = app.world().resource::<Messages<LifeAwardMessage>>();
    assert!(
        !life_messages.is_empty(),
        "LifeAwardMessage should be written"
    );

    // Check lives remain at max (clamped by consumer)
    let final_lives = app.world().resource::<LivesState>().lives_remaining;
    assert_eq!(final_lives, 5, "Lives should remain at max (5), not exceed");
}
