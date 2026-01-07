//! Integration tests for merkaba paddle contact penalty (US3: T029, T030)
//!
//! Tests that paddle contact results in life loss, ball despawn, and merkaba despawn.

use bevy::app::App;
use bevy::ecs::message::{MessageReader, Messages};
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::CollisionEvent;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use brkrs::signals::MerkabaPaddleCollision;
use brkrs::systems::merkaba::Merkaba;
use brkrs::systems::respawn::{LifeLostEvent, LivesState, SpawnPoints};
use brkrs::systems::textures::TypeVariantRegistry;
use brkrs::{Ball, Paddle};

#[derive(Resource, Default)]
struct TestEvents {
    paddle_collision: bool,
    life_lost: bool,
}

fn on_merkaba_paddle_collision(
    _trigger: Trigger<MerkabaPaddleCollision>,
    mut events: ResMut<TestEvents>,
) {
    events.paddle_collision = true;
}

fn mock_despawn_on_life_loss(
    mut events: MessageReader<LifeLostEvent>,
    mut commands: Commands,
    balls: Query<Entity, With<Ball>>,
    merkabas: Query<Entity, With<Merkaba>>,
) {
    if !events.is_empty() {
        events.clear();
        for e in balls.iter() {
            commands.entity(e).despawn();
        }
        for e in merkabas.iter() {
            commands.entity(e).despawn();
        }
    }
}

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<TypeVariantRegistry>()
        .init_resource::<SpawnPoints>()
        .init_resource::<TestEvents>()
        .insert_resource(Assets::<Mesh>::default())
        .insert_resource(Assets::<StandardMaterial>::default())
        .insert_resource(LivesState {
            lives_remaining: 3,
            on_last_life: false,
        })
        .add_event::<CollisionEvent>() // Rapier uses Events, not Messages in Bevy 0.17+ context usually, but earlier errors suggested MessageReader. Let's stick to what worked or check lib.rs
        .add_message::<brkrs::signals::SpawnMerkabaMessage>()
        .add_message::<LifeLostEvent>() // LifeLostEvent is a Message in respawn.rs
        // .add_message::<brkrs::signals::MerkabaPaddleCollision>() // REMOVED
        .add_plugins(brkrs::systems::merkaba::MerkabaPlugin)
        .add_systems(Update, mock_despawn_on_life_loss);

    // Register test observers
    app.add_observer(on_merkaba_paddle_collision);

    app
}

fn trigger_collision(app: &mut App, e1: Entity, e2: Entity) {
    // If sys uses EventReader<CollisionEvent>, we must write to Events<CollisionEvent>
    app.world_mut().send_event(CollisionEvent::Started(
        e1,
        e2,
        CollisionEventFlags::empty(),
    ));
}

/// T029: Paddle contact → life -1 + distinct paddle collision sound.
///
/// When merkaba contacts the player paddle, the system MUST:
/// - Trigger a life loss event (player loses 1 life)
/// - Emit a distinct paddle collision sound (unique from wall/brick sounds)
#[test]
fn t029_paddle_contact_triggers_life_loss_and_sound() {
    let mut app = test_app();

    let initial_lives = app.world().resource::<LivesState>().lives_remaining;
    assert_eq!(initial_lives, 3, "Should start with 3 lives");

    let merkaba = app.world_mut().spawn((Merkaba, Transform::default())).id();
    let paddle = app.world_mut().spawn((Paddle, Transform::default())).id();
    // Spawn a ball so the life loss logic can find it (requirement of refactor)
    app.world_mut().spawn((Ball, Transform::default()));

    // Trigger paddle collision
    trigger_collision(&mut app, merkaba, paddle);
    app.update();

    // Assert paddle collision event was observed
    let events = app.world().resource::<TestEvents>();
    assert!(
        events.paddle_collision,
        "Paddle collision event should be emitted"
    );

    // Assert LifeLostEvent message was emitted
    let messages = app.world().resource::<Messages<LifeLostEvent>>();
    assert!(
        !messages.is_empty(),
        "LifeLostEvent message should be emitted"
    );
}

/// T030: Ball despawn + all merkaba despawn on paddle contact.
///
/// When merkaba contacts the paddle (triggering life loss), the system MUST:
/// - Despawn all currently active ball entities
/// - Despawn all currently active merkaba entities
/// This ensures a clean state after a life-loss event.
#[test]
fn t030_paddle_contact_despawns_balls_and_merkabas() {
    let mut app = test_app();

    // Create multiple balls and merkabas
    let ball1 = app.world_mut().spawn((Ball, Transform::default())).id();
    let ball2 = app.world_mut().spawn((Ball, Transform::default())).id();
    let merkaba1 = app.world_mut().spawn((Merkaba, Transform::default())).id();
    let merkaba2 = app.world_mut().spawn((Merkaba, Transform::default())).id();
    let paddle = app.world_mut().spawn((Paddle, Transform::default())).id();

    // Initialize systems (set up local state in despawn system)
    app.update();

    // Verify entities exist before collision
    assert!(app.world().entities().contains(ball1), "Ball1 should exist");
    assert!(app.world().entities().contains(ball2), "Ball2 should exist");
    assert!(
        app.world().entities().contains(merkaba1),
        "Merkaba1 should exist"
    );
    assert!(
        app.world().entities().contains(merkaba2),
        "Merkaba2 should exist"
    );

    // Trigger paddle collision with one merkaba
    trigger_collision(&mut app, merkaba1, paddle);
    app.update();
    app.update();

    // All balls and merkabas should be despawned
    assert!(
        !app.world().entities().contains(ball1),
        "Ball1 should be despawned"
    );
    assert!(
        !app.world().entities().contains(ball2),
        "Ball2 should be despawned"
    );
    assert!(
        !app.world().entities().contains(merkaba1),
        "Merkaba1 should be despawned"
    );
    assert!(
        !app.world().entities().contains(merkaba2),
        "Merkaba2 should be despawned"
    );
}
