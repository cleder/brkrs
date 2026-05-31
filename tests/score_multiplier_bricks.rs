use bevy::{app::App, ecs::message::Messages, prelude::*, MinimalPlugins};

fn test_app() -> App {
    let mut app = App::new();
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

#[test]
fn active_multiplier_persists_across_multiple_frames_until_reset() {
    let mut app = test_app();

    app.world_mut()
        .resource_mut::<Messages<brkrs::signals::BrickDestroyed>>()
        .write(brkrs::signals::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(500).expect("entity id should construct"),
            brick_type: 29,
            brick_position: Vec3::ZERO,
            destroyed_by: None,
        });

    app.update();

    for _ in 0..10 {
        app.update();
    }

    app.world_mut()
        .resource_mut::<Messages<brkrs::signals::BrickDestroyed>>()
        .write(brkrs::signals::BrickDestroyed {
            brick_entity: Entity::from_raw_u32(501).expect("entity id should construct"),
            brick_type: 20,
            brick_position: Vec3::ZERO,
            destroyed_by: None,
        });

    app.update();

    let score = app
        .world()
        .resource::<brkrs::systems::scoring::ScoreState>()
        .current_score;
    assert_eq!(
        score, 125,
        "times-4 brick should award base 25, and the later simple brick should award 100"
    );
}
