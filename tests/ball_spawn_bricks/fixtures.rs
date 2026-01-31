use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::{CollisionEvent, Velocity};

use brkrs::physics_config::{BallPhysicsConfig, BrickPhysicsConfig, PaddlePhysicsConfig};
use brkrs::signals::BrickDestroyed;
use brkrs::systems::ball_spawn_bricks::ball_spawn_system;
use brkrs::systems::BallSpawnBricksPlugin;
use brkrs::{Ball, Brick, BrickTypeId, CountsTowardsCompletion, EmittedBrickDestroyed};

/// Build a minimal app with ball spawn systems and message types registered.
pub fn setup_test_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));
    app.add_message::<BrickDestroyed>();
    app.add_message::<CollisionEvent>();
    app.init_resource::<EmittedBrickDestroyed>();
    app.insert_resource(BallPhysicsConfig::default());
    app.insert_resource(PaddlePhysicsConfig::default());
    app.insert_resource(BrickPhysicsConfig::default());
    app.add_plugins(BallSpawnBricksPlugin);
    app.add_systems(Update, advance_ball_positions.after(ball_spawn_system));
    app
}

/// Spawn a test ball with explicit position and velocity.
pub fn spawn_test_ball(app: &mut App, position: Vec3, velocity: Vec3) -> Entity {
    app.world_mut()
        .spawn((
            Ball,
            Transform::from_translation(position),
            GlobalTransform::from_translation(position),
            bevy_rapier3d::prelude::RigidBody::Dynamic,
            bevy_rapier3d::prelude::Velocity::linear(velocity),
        ))
        .id()
}

/// Spawn a test brick with a type index at a given position.
pub fn spawn_test_brick(app: &mut App, brick_type: u8, position: Vec3) -> Entity {
    app.world_mut()
        .spawn((
            Brick,
            BrickTypeId(brick_type),
            CountsTowardsCompletion,
            Transform::from_translation(position),
            GlobalTransform::from_translation(position),
        ))
        .id()
}

/// Count all active balls in the world.
pub fn count_balls(app: &mut App) -> usize {
    let world = app.world_mut();
    let mut query = world.query_filtered::<Entity, With<Ball>>();
    query.iter(world).count()
}

fn advance_ball_positions(
    time: Res<Time>,
    mut balls: Query<(&mut Transform, &Velocity), With<Ball>>,
) {
    let mut delta = time.delta_secs();
    if delta == 0.0 {
        delta = 1.0;
    }
    for (mut transform, velocity) in balls.iter_mut() {
        transform.translation += velocity.linvel * delta;
    }
}
