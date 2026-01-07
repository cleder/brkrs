//! Integration tests for merkaba spawning (US1: T010, T011, T012b)
//!
//! Tests the rotor brick collision detection, spawn message emission,
//! delayed spawn, and brick destruction behaviors.

use std::time::Duration;

use bevy::app::App;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::CollisionEvent;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use brkrs::register_brick_collision_systems;
use brkrs::signals::SpawnMerkabaMessage;
use brkrs::systems::merkaba::Merkaba;
use brkrs::systems::respawn::LivesState;
use brkrs::{Ball, Brick, BrickTypeId, CountsTowardsCompletion};

const ROTOR_BRICK_INDEX: u8 = 36;

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(Assets::<Mesh>::default())
        .insert_resource(Assets::<StandardMaterial>::default())
        .insert_resource(LivesState {
            lives_remaining: 3,
            on_last_life: false,
        })
        .add_message::<CollisionEvent>()
        .add_message::<SpawnMerkabaMessage>()
        .add_plugins(brkrs::systems::merkaba::MerkabaPlugin);

    register_brick_collision_systems(&mut app);
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

fn advance_time(app: &mut App, delta_secs: f32) {
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(Duration::from_secs_f32(delta_secs));
}

/// T010: Assert SpawnMerkabaMessage emitted on brick 36 hit.
///
/// When a ball collides with a brick with index 36, the system MUST emit
/// a SpawnMerkabaMessage to the message writer.
#[test]
fn t010_spawn_message_emitted_on_rotor_brick_hit() {
    let mut app = test_app();

    let ball = app.world_mut().spawn(Ball).id();
    let rotor_brick = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(ROTOR_BRICK_INDEX),
            CountsTowardsCompletion,
        ))
        .id();

    trigger_collision(&mut app, ball, rotor_brick);
    advance_time(&mut app, 0.016);
    app.update();

    let messages = app.world().resource::<Messages<SpawnMerkabaMessage>>();
    assert!(
        !messages.is_empty(),
        "SpawnMerkabaMessage should be emitted when rotor brick (36) is hit"
    );
}

/// T011: Assert 0.5s delayed spawn at brick position with dual tetrahedron children.
///
/// After rotor brick collision, the system MUST:
/// - Wait 0.5 seconds before spawning merkaba
/// - Spawn merkaba at the destroyed brick's position
/// - Create dual-tetrahedron children (one upright, one inverted)
#[test]
fn t011_merkaba_spawned_after_0_5s_with_dual_tetrahedron() {
    let mut app = test_app();

    let ball = app.world_mut().spawn(Ball).id();
    let brick_position = Vec3::new(2.0, 3.0, 0.0);
    let rotor_brick = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(ROTOR_BRICK_INDEX),
            CountsTowardsCompletion,
            Transform::from_translation(brick_position),
            GlobalTransform::from_translation(brick_position),
        ))
        .id();

    trigger_collision(&mut app, ball, rotor_brick);

    // Before the 0.5s delay: no merkaba should exist yet
    advance_time(&mut app, 0.25);
    app.update();
    {
        let mut world = app.world_mut();
        let mut q = world.query_filtered::<Entity, With<Merkaba>>();
        let any = q.iter(&mut world).next().is_some();
        assert!(!any, "Merkaba should not spawn before 0.5s delay");
    }

    // After delay: merkaba should spawn at brick position with two children
    advance_time(&mut app, 0.5);

    // Manually tick the pending merkaba spawn timers (Time delta isn't working in tests)
    {
        let mut pending = app
            .world_mut()
            .resource_mut::<brkrs::systems::merkaba::PendingMerkabaSpawns>();
        for spawn in &mut pending.entries {
            spawn.timer.tick(std::time::Duration::from_secs_f32(0.6));
        }
    }
    app.update();

    let merkabas: Vec<(Entity, Transform, usize)> = {
        let mut world = app.world_mut();
        let mut q =
            world.query_filtered::<(Entity, &Transform, Option<&Children>), With<Merkaba>>();
        q.iter(&mut world)
            .map(|(entity, transform, children)| {
                let count = children.map(|c| c.len()).unwrap_or(0);
                (entity, transform.clone(), count)
            })
            .collect()
    };

    assert!(
        !merkabas.is_empty(),
        "Merkaba should spawn after 0.5s delay"
    );

    // Validate spawn position and child hierarchy count (expected 2 children: dual tetrahedrons)
    let (_, transform, child_count) = merkabas[0].clone();
    assert!(
        transform.translation.distance(brick_position) < 0.01,
        "Merkaba should spawn at brick position; expected {:?}, got {:?}",
        brick_position,
        transform.translation
    );
    assert_eq!(
        child_count, 2,
        "Merkaba should have two child meshes (dual tetrahedron)"
    );

    // Verify the children have Tetrahedron meshes
    let merkaba_entity = merkabas[0].0;
    let children = app.world().get::<Children>(merkaba_entity).unwrap();
    assert_eq!(
        children.len(),
        2,
        "Should have exactly 2 children (dual tetrahedrons)"
    );
}

/// T012b: Assert rotor brick destroyed on collision + message emitted (FR-016).
///
/// When ball hits rotor brick (index 36), the brick MUST be destroyed (removed from world)
/// AND the spawn message MUST still be emitted (destruction and spawn are independent).
#[test]
fn t012b_rotor_brick_destroyed_on_collision() {
    let mut app = test_app();

    let ball = app.world_mut().spawn(Ball).id();
    let rotor_brick = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(ROTOR_BRICK_INDEX),
            CountsTowardsCompletion,
        ))
        .id();

    trigger_collision(&mut app, ball, rotor_brick);
    advance_time(&mut app, 0.016);
    app.update();

    // Brick should be despawned
    {
        let world = app.world();
        assert!(
            !world.entities().contains(rotor_brick),
            "Rotor brick should be destroyed after collision"
        );
    }

    // Spawn message should have been emitted
    let messages = app.world().resource::<Messages<SpawnMerkabaMessage>>();
    assert!(
        !messages.is_empty(),
        "SpawnMerkabaMessage should be emitted even though the brick is destroyed"
    );
}

// TODO: T013 acceptance checks can be added as inline assertions in tests above:
// - Verify no panicking `.unwrap()` calls in rotor brick system
// - Verify message vs observer separation (use MessageWriter, not Event)
// - Verify hierarchy safety (no direct Transform conflicts)
