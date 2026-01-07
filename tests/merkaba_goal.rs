//! Integration tests for merkaba goal despawn (US2: T022)
//!
//! Tests that merkaba is despawned when it contacts the goal area.

use bevy::app::App;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::{CollisionEvent, Velocity};
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use brkrs::systems::merkaba::Merkaba;
use brkrs::systems::respawn::LivesState;
use brkrs::systems::textures::{ObjectClass, TypeVariantRegistry};
use brkrs::LowerGoal;

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(Assets::<Mesh>::default())
        .insert_resource(Assets::<StandardMaterial>::default())
        .insert_resource(LivesState {
            lives_remaining: 3,
            on_last_life: false,
        })
        .init_resource::<TypeVariantRegistry>()
        .add_message::<CollisionEvent>()
        .add_message::<brkrs::signals::SpawnMerkabaMessage>()
        .add_message::<brkrs::signals::MerkabaWallCollision>()
        .add_message::<brkrs::signals::MerkabaBrickCollision>()
        .add_plugins(brkrs::systems::merkaba::MerkabaPlugin);

    // Populate registry with dummies
    let mut registry = app.world_mut().resource_mut::<TypeVariantRegistry>();
    let mat = Handle::<StandardMaterial>::default();
    registry.insert_for_tests(ObjectClass::Merkaba, 0, mat.clone());
    registry.insert_for_tests(ObjectClass::Merkaba, 1, mat);

    app
}

fn trigger_collision(app: &mut App, e1: Entity, e2: Entity) {
    app.world_mut()
        .resource_mut::<Messages<CollisionEvent>>()
        .write(CollisionEvent::Started(
            e1,
            e2,
            CollisionEventFlags::empty(),
        ));
}

/// T022: Goal area contact → merkaba despawn (100% success rate).
///
/// When merkaba contacts the goal area (typically the bottom boundary where
/// the ball falls off), the merkaba MUST be despawned immediately. This behavior
/// ensures merkabas do not persist indefinitely and clutter the world.
#[test]
fn t022_merkaba_despawns_on_goal_contact() {
    let mut app = test_app();

    let merkaba = app
        .world_mut()
        .spawn((
            Merkaba,
            Transform::from_translation(Vec3::new(0.0, -5.0, 0.0)),
            Velocity::linear(Vec3::new(0.0, -10.0, 0.0)), // Moving toward goal
        ))
        .id();
    let goal = app.world_mut().spawn(LowerGoal).id();

    // Trigger goal contact collision
    trigger_collision(&mut app, merkaba, goal);
    app.update();

    // Verify merkaba is despawned
    assert!(
        !app.world().entities().contains(merkaba),
        "Merkaba should despawn when contacting goal area"
    );
}
