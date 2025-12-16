use bevy::{app::App, ecs::message::Messages, prelude::*, MinimalPlugins};

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_message::<brkrs::systems::scoring::BrickDestroyed>();
    app.insert_resource(brkrs::systems::scoring::ScoreState::default());
    app.add_systems(Update, brkrs::systems::scoring::award_points_system);
    app
}

#[test]
fn scoring_accumulates_points_and_question_in_range() {
    let mut app = test_app();

    // Write a set of destruction events: Simple Stone (25), Multi-hit (50), Question (25-300)
    {
        let mut msgs = app
            .world_mut()
            .resource_mut::<Messages<brkrs::systems::scoring::BrickDestroyed>>();
        msgs.write(brkrs::systems::scoring::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(1).expect("entity id should construct"),
            brick_type: 20, // Simple Stone => 25
            destroyed_by: None,
        });
        msgs.write(brkrs::systems::scoring::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(2).expect("entity id should construct"),
            brick_type: 10, // Multi-hit => 50
            destroyed_by: None,
        });
        msgs.write(brkrs::systems::scoring::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(3).expect("entity id should construct"),
            brick_type: 53, // Question => 25-300
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
            .resource_mut::<Messages<brkrs::systems::scoring::BrickDestroyed>>();
        msgs.write(brkrs::systems::scoring::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(10).expect("entity id should construct"),
            brick_type: 41, // Extra Ball => 0 points
            destroyed_by: None,
        });
        msgs.write(brkrs::systems::scoring::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(11).expect("entity id should construct"),
            brick_type: 55, // Magnet enabled => 0 points
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
