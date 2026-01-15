//! Integration tests for paddle-destroyable bricks (type 57).
//!
//! Tests verify:
//! - US1: Paddle collision destroys brick, awards 250 points
//! - US2: Ball collision bounces off without destroying brick
//! - Multi-frame persistence (10 frames minimum)

use bevy::{app::App, ecs::message::Messages, prelude::*, MinimalPlugins};
use bevy_rapier3d::{control::CharacterCollision, geometry::ShapeCastHit, prelude::*};

use brkrs::{
    level_format::{is_paddle_destroyable_brick, PADDLE_DESTROYABLE_BRICK},
    signals::BrickDestroyed,
    systems::respawn::{FrameLossState, LifeLostEvent, SpawnPoints},
    systems::scoring::{MilestoneReached, ScoreState},
    Ball, Brick, BrickTypeId, CountsTowardsCompletion, MarkedForDespawn, Paddle,
};

fn test_app() -> App {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));
    app.add_message::<CollisionEvent>();
    app.add_message::<BrickDestroyed>();
    app.add_message::<MilestoneReached>();
    app.add_message::<LifeLostEvent>();
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(bevy::input::ButtonInput::<KeyCode>::default());
    app.insert_resource(ScoreState {
        current_score: 0,
        last_milestone_reached: 0,
    });
    app.insert_resource(SpawnPoints::default());
    app.insert_resource(FrameLossState::default());
    app.world_mut().spawn(RapierConfiguration::new(1.0));

    // Register systems
    brkrs::register_brick_collision_systems(&mut app);
    app.add_systems(
        Update,
        (
            brkrs::read_character_controller_collisions,
            brkrs::systems::scoring::award_points_system,
            brkrs::systems::scoring::detect_milestone_system,
        )
            .chain()
            .after(brkrs::despawn_marked_entities),
    );

    app
}

// =============================================================================
// Helper function tests
// =============================================================================

#[test]
fn is_paddle_destroyable_brick_returns_true_for_type_57() {
    assert!(
        is_paddle_destroyable_brick(PADDLE_DESTROYABLE_BRICK),
        "Type 57 should be paddle-destroyable"
    );
}

#[test]
fn is_paddle_destroyable_brick_returns_false_for_other_types() {
    assert!(
        !is_paddle_destroyable_brick(20),
        "Type 20 (simple brick) should not be paddle-destroyable"
    );
    assert!(
        !is_paddle_destroyable_brick(0),
        "Type 0 should not be paddle-destroyable"
    );
    assert!(
        !is_paddle_destroyable_brick(255),
        "Type 255 should not be paddle-destroyable"
    );
}

// =============================================================================
// US1: Paddle destroys brick type 57, awards 250 points
// =============================================================================

#[test]
fn paddle_collision_destroys_type_57_brick() {
    let mut app = test_app();

    // Spawn paddle with controller output to feed collisions into the system
    let paddle = app
        .world_mut()
        .spawn((
            Paddle,
            KinematicCharacterController::default(),
            KinematicCharacterControllerOutput::default(),
        ))
        .id();

    // Spawn type 57 brick
    let brick = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(PADDLE_DESTROYABLE_BRICK),
            CountsTowardsCompletion,
            Transform::from_xyz(0.0, -4.0, 0.0),
        ))
        .id();

    // Verify initial state
    assert_eq!(
        app.world().resource::<ScoreState>().current_score,
        0,
        "Initial score should be 0"
    );

    // Simulate paddle collision by populating the controller output
    let mut paddle_output = app
        .world_mut()
        .get_mut::<KinematicCharacterControllerOutput>(paddle)
        .expect("paddle output component present");
    paddle_output.collisions.push(CharacterCollision {
        entity: brick,
        character_translation: Vec3::ZERO,
        character_rotation: Quat::IDENTITY,
        translation_applied: Vec3::ZERO,
        translation_remaining: Vec3::ZERO,
        // ShapeCastHit lacks Default; zeroed is sufficient for stubbed collision data in this test.
        hit: unsafe { std::mem::zeroed::<ShapeCastHit>() },
    });

    // Run update loop for multi-frame persistence (10 frames minimum)
    for _ in 0..10 {
        app.update();
    }

    // Verify brick destroyed
    assert!(
        app.world().get_entity(brick).is_err(),
        "Brick type 57 should be despawned after paddle collision"
    );

    // Verify 250 points awarded
    assert_eq!(
        app.world().resource::<ScoreState>().current_score,
        250,
        "Paddle destroying type 57 brick should award 250 points"
    );

    // Verify BrickDestroyed message emitted
    let messages = app.world().resource::<Messages<BrickDestroyed>>();
    assert!(!messages.is_empty(), "Should emit BrickDestroyed message");
}

// =============================================================================
// US2: Ball collision bounces off without destroying brick
// =============================================================================

#[test]
fn ball_collision_does_not_destroy_type_57_brick() {
    let mut app = test_app();

    // Spawn ball
    let ball = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(0.0, 5.0, 0.0)))
        .id();

    // Spawn type 57 brick
    let brick = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(PADDLE_DESTROYABLE_BRICK),
            CountsTowardsCompletion,
            Transform::from_xyz(0.0, 4.0, 0.0),
        ))
        .id();

    // Verify initial score
    assert_eq!(app.world().resource::<ScoreState>().current_score, 0);

    // Simulate ball-brick collision
    app.world_mut().write_message(CollisionEvent::Started(
        ball,
        brick,
        bevy_rapier3d::rapier::prelude::CollisionEventFlags::empty(),
    ));

    // Run update loop for multi-frame persistence
    for _ in 0..10 {
        app.update();
    }

    // Verify brick still exists
    assert!(
        app.world().get_entity(brick).is_ok(),
        "Brick type 57 should NOT be despawned by ball collision"
    );

    // Verify no points awarded
    assert_eq!(
        app.world().resource::<ScoreState>().current_score,
        0,
        "Ball hitting type 57 brick should not award points"
    );

    // Verify brick NOT marked for despawn
    assert!(
        app.world().get::<MarkedForDespawn>(brick).is_none(),
        "Brick type 57 should NOT have MarkedForDespawn after ball collision"
    );
}

#[test]
fn ball_collision_no_brick_destroyed_message() {
    let mut app = test_app();

    let ball = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(0.0, 5.0, 0.0)))
        .id();

    let brick = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(PADDLE_DESTROYABLE_BRICK),
            CountsTowardsCompletion,
            Transform::from_xyz(0.0, 4.0, 0.0),
        ))
        .id();

    // Simulate ball-brick collision
    app.world_mut().write_message(CollisionEvent::Started(
        ball,
        brick,
        bevy_rapier3d::rapier::prelude::CollisionEventFlags::empty(),
    ));

    // Run updates
    for _ in 0..10 {
        app.update();
    }

    // Verify NO BrickDestroyed message
    let messages = app.world().resource::<Messages<BrickDestroyed>>();
    assert!(
        messages.is_empty(),
        "Ball collision with type 57 should NOT emit BrickDestroyed message"
    );
}

// =============================================================================
// Paddle-Hazard Brick Collision (Types 42/91)
// =============================================================================

#[test]
fn paddle_hazard_collision_integration_setup() {
    // Integration test: Verify paddle-hazard collision system infrastructure
    //
    // This test validates that the core hazard brick types and life-loss
    // systems are properly initialized and integrated:
    // - Hazard brick types (42, 91) are properly identified
    // - FrameLossState resource exists and initializes correctly
    // - LifeLostEvent message type is registered
    // - SpawnPoints resource is available for respawning
    //
    // Note: Full collision simulation requires extensive physics setup.
    // This test validates the infrastructure is in place.

    use brkrs::level_format::{is_hazard_brick, HAZARD_BRICK_42};

    let mut app = test_app();

    // Verify hazard brick types are properly identified
    assert!(
        is_hazard_brick(HAZARD_BRICK_42),
        "Type 42 must be identified as hazard"
    );
    assert!(is_hazard_brick(91), "Type 91 must be identified as hazard");

    // Verify FrameLossState is initialized
    let frame_loss_state = app.world().resource::<FrameLossState>();
    assert!(
        !frame_loss_state.hazard_loss_emitted,
        "FrameLossState should initialize with flag = false"
    );

    // Verify LifeLostEvent message system is available
    let messages = app.world().resource::<Messages<LifeLostEvent>>();
    assert!(
        messages.is_empty(),
        "LifeLostEvent messages should be empty initially"
    );

    // Verify SpawnPoints resource is available
    let spawn_points = app.world().resource::<SpawnPoints>();
    let _ = spawn_points.ball_spawn();

    // Test passes - infrastructure is properly set up
}
