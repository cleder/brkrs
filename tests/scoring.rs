use bevy::{app::App, ecs::message::Messages, prelude::*, MinimalPlugins};

fn test_app() -> App {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins(MinimalPlugins);
    app.add_message::<brkrs::signals::BrickDestroyed>();
    app.insert_resource(brkrs::systems::scoring::ScoreState::default());
    app.add_systems(Update, brkrs::systems::scoring::award_points_system);
    app
}

fn multiplier_test_app() -> App {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins(MinimalPlugins);
    app.add_message::<brkrs::signals::BrickDestroyed>();
    app.add_message::<brkrs::systems::respawn::LifeLostEvent>();
    app.insert_resource(brkrs::systems::scoring::ScoreState::default());
    app.insert_resource(brkrs::systems::scoring::ScoreMultiplierState::default());
    app.add_systems(
        Update,
        (
            brkrs::systems::scoring::award_points_system,
            brkrs::systems::scoring::reset_multiplier_on_life_loss_system,
        )
            .chain(),
    );
    app
}

fn milestone_test_app() -> App {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins(MinimalPlugins);
    app.add_message::<brkrs::signals::BrickDestroyed>();
    app.add_message::<brkrs::systems::scoring::MilestoneReached>();
    app.insert_resource(brkrs::systems::scoring::ScoreState::default());
    // Chain systems: award points first, then detect milestones
    app.add_systems(
        Update,
        (
            brkrs::systems::scoring::award_points_system,
            brkrs::systems::scoring::detect_milestone_system,
        )
            .chain(),
    );
    app
}

#[test]
fn scoring_accumulates_points_and_question_in_range() {
    let mut app = test_app();

    // Write a set of destruction events: Simple Stone (25), Multi-hit (50), Question (25-300)
    {
        let mut msgs = app
            .world_mut()
            .resource_mut::<Messages<brkrs::signals::BrickDestroyed>>();
        msgs.write(brkrs::signals::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(1).expect("entity id should construct"),
            brick_type: 20, // Simple Stone => 25
            brick_position: Vec3::ZERO,
            destroyed_by: None,
        });
        msgs.write(brkrs::signals::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(2).expect("entity id should construct"),
            brick_type: 10, // Multi-hit => 50
            brick_position: Vec3::ZERO,
            destroyed_by: None,
        });
        msgs.write(brkrs::signals::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(3).expect("entity id should construct"),
            brick_type: 53, // Question => 25-300
            brick_position: Vec3::ZERO,
            destroyed_by: None,
        });
    }

    app.update();

    let score = app
        .world()
        .resource::<brkrs::systems::scoring::ScoreState>()
        .current_score;

    // Expected range: 25 (simple) + 50 (multi-hit) + 25..=300 (question)
    assert!(
        (100..=375).contains(&score),
        "Score {} outside expected accumulation range (100..=375)",
        score
    );
}

#[test]
fn scoring_ignores_zero_value_bricks() {
    let mut app = test_app();

    {
        let mut msgs = app
            .world_mut()
            .resource_mut::<Messages<brkrs::signals::BrickDestroyed>>();
        msgs.write(brkrs::signals::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(10).expect("entity id should construct"),
            brick_type: 41, // Extra Ball => 0 points
            brick_position: Vec3::ZERO,
            destroyed_by: None,
        });
        msgs.write(brkrs::signals::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(11).expect("entity id should construct"),
            brick_type: 55, // Magnet enabled => 0 points
            brick_position: Vec3::ZERO,
            destroyed_by: None,
        });
    }

    app.update();

    let score = app
        .world()
        .resource::<brkrs::systems::scoring::ScoreState>()
        .current_score;

    assert_eq!(score, 0, "Zero-value bricks should not change score");
}

#[test]
fn milestone_detection_emits_event_at_5000() {
    let mut app = milestone_test_app();

    // Set score to 4900, then add points to cross first milestone
    {
        let mut score_state = app
            .world_mut()
            .resource_mut::<brkrs::systems::scoring::ScoreState>();
        score_state.current_score = 4900;
        score_state.last_milestone_reached = 0;
    }

    {
        let mut msgs = app
            .world_mut()
            .resource_mut::<Messages<brkrs::signals::BrickDestroyed>>();
        // Add 4 bricks of 25 points each = 100 points total
        // 4900 + 100 = 5000 (exactly crosses first milestone)
        for i in 0..4 {
            msgs.write(brkrs::signals::BrickDestroyed {
                brick_entity: Entity::from_raw_u32(100 + i).expect("entity id should construct"),
                brick_type: 20, // Simple Stone => 25 points each
                brick_position: Vec3::ZERO,
                destroyed_by: None,
            });
        }
    }

    app.update();

    // Verify score crossed 5000 threshold (4900 + 100 = 5000)
    let score_state = app
        .world()
        .resource::<brkrs::systems::scoring::ScoreState>();
    assert_eq!(score_state.current_score, 5000);
    assert_eq!(
        score_state.last_milestone_reached, 1,
        "First milestone tier should be recorded"
    );

    // Verify milestone event was emitted
    let msgs = app
        .world()
        .resource::<Messages<brkrs::systems::scoring::MilestoneReached>>();
    assert!(
        !msgs.is_empty(),
        "Milestone event should be emitted when crossing 5000"
    );
}

#[test]
fn milestone_detection_multiple_milestones_one_update() {
    let mut app = milestone_test_app();

    // Set score to 3000, then jump to 11250 (crossing 5000 and 10000)
    {
        let mut score_state = app
            .world_mut()
            .resource_mut::<brkrs::systems::scoring::ScoreState>();
        score_state.current_score = 3000;
        score_state.last_milestone_reached = 0;
    }

    {
        let mut msgs = app
            .world_mut()
            .resource_mut::<Messages<brkrs::signals::BrickDestroyed>>();
        // Add 8250 points worth of bricks (33 x 250-point bricks)
        for i in 0..33 {
            msgs.write(brkrs::signals::BrickDestroyed {
                brick_entity: Entity::from_raw_u32(200 + i).expect("entity id should construct"),
                brick_type: 25, // Gravity brick type 25 => 250 points
                brick_position: Vec3::ZERO,
                destroyed_by: None,
            });
        }
    }

    app.update();

    // Verify score crossed both milestones (3000 + 8250 = 11250)
    let score_state = app
        .world()
        .resource::<brkrs::systems::scoring::ScoreState>();
    assert_eq!(score_state.current_score, 11250);
    assert_eq!(
        score_state.last_milestone_reached, 2,
        "Should record highest milestone tier reached"
    );

    // Verify both milestone events were emitted
    let msgs = app
        .world()
        .resource::<Messages<brkrs::systems::scoring::MilestoneReached>>();
    assert!(
        !msgs.is_empty(),
        "Milestone events should be emitted when crossing multiple thresholds"
    );
}

#[test]
fn milestone_detection_no_duplicate_events() {
    let mut app = milestone_test_app();

    // Set score at 5500 (already passed milestone 1)
    {
        let mut score_state = app
            .world_mut()
            .resource_mut::<brkrs::systems::scoring::ScoreState>();
        score_state.current_score = 5500;
        score_state.last_milestone_reached = 1;
    }

    {
        let mut msgs = app
            .world_mut()
            .resource_mut::<Messages<brkrs::signals::BrickDestroyed>>();
        msgs.write(brkrs::signals::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(300).expect("entity id should construct"),
            brick_type: 22, // Limestone => 75 points
            brick_position: Vec3::ZERO,
            destroyed_by: None,
        });
    }

    app.update();

    // Verify score increased but stays below next milestone (5500 + 75 = 5575)
    let score_state = app
        .world()
        .resource::<brkrs::systems::scoring::ScoreState>();
    assert_eq!(score_state.current_score, 5575);
    assert_eq!(
        score_state.last_milestone_reached, 1,
        "Milestone tier should not change"
    );

    // Verify NO milestone events emitted
    let msgs = app
        .world()
        .resource::<Messages<brkrs::systems::scoring::MilestoneReached>>();
    assert!(
        msgs.is_empty(),
        "Should not emit duplicate milestone events"
    );
}

#[test]
fn multiplier_bricks_only_affect_following_brick_hits() {
    let mut app = multiplier_test_app();

    {
        let mut msgs = app
            .world_mut()
            .resource_mut::<Messages<brkrs::signals::BrickDestroyed>>();
        msgs.write(brkrs::signals::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(400).expect("entity id should construct"),
            brick_type: 27,
            brick_position: Vec3::ZERO,
            destroyed_by: None,
        });
        msgs.write(brkrs::signals::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(401).expect("entity id should construct"),
            brick_type: 20,
            brick_position: Vec3::ZERO,
            destroyed_by: None,
        });
    }

    app.update();

    let score = app
        .world()
        .resource::<brkrs::systems::scoring::ScoreState>()
        .current_score;
    let multiplier = app
        .world()
        .resource::<brkrs::systems::scoring::ScoreMultiplierState>();

    assert_eq!(
        score, 75,
        "brick 27 should award base 25 and the following 25-point brick should award 50 at 2x"
    );
    assert_eq!(multiplier.factor, 2);
}

#[test]
fn multiplier_reset_only_on_actual_life_loss() {
    let mut app = multiplier_test_app();

    app.world_mut()
        .resource_mut::<brkrs::systems::scoring::ScoreMultiplierState>()
        .factor = 4;

    app.world_mut()
        .resource_mut::<Messages<brkrs::systems::respawn::LifeLostEvent>>()
        .write(brkrs::systems::respawn::LifeLostEvent {
            ball: Entity::from_raw_u32(402).expect("entity id should construct"),
            cause: brkrs::systems::respawn::LifeLossCause::LowerGoal,
            ball_spawn: brkrs::systems::respawn::SpawnTransform::new(Vec3::ZERO, Quat::IDENTITY),
        });

    app.update();

    let multiplier = app
        .world()
        .resource::<brkrs::systems::scoring::ScoreMultiplierState>();
    assert_eq!(
        multiplier.factor, 1,
        "life loss should reset active multiplier to 1x"
    );
}

#[test]
fn unified_brick_destroyed_signal_consumed_by_scoring() {
    let mut app = test_app();

    // Write a BrickDestroyed message using the shared type
    {
        let mut msgs = app
            .world_mut()
            .resource_mut::<Messages<brkrs::signals::BrickDestroyed>>();
        msgs.write(brkrs::signals::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(999).expect("entity id should construct"),
            brick_type: 20, // Simple Stone => 25 points
            brick_position: Vec3::ZERO,
            destroyed_by: None,
        });
    }

    app.update();

    let score = app
        .world()
        .resource::<brkrs::systems::scoring::ScoreState>()
        .current_score;

    assert_eq!(
        score, 25,
        "Scoring system should consume unified BrickDestroyed message"
    );
}
