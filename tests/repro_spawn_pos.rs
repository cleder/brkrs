use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy_rapier3d::prelude::CollisionEvent;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;
use brkrs::{
    mark_brick_on_ball_collision,
    systems::merkaba::{queue_merkaba_spawns, PendingMerkabaSpawns},
    Ball, Brick, BrickTypeId, CountsTowardsCompletion,
};

#[test]
fn test_spawn_at_correct_location() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_message::<CollisionEvent>()
        .add_message::<brkrs::signals::SpawnMerkabaMessage>()
        .add_message::<brkrs::signals::BrickDestroyed>()
        .init_resource::<PendingMerkabaSpawns>()
        .add_systems(
            Update,
            (mark_brick_on_ball_collision, queue_merkaba_spawns).chain(),
        );

    // Spawn a Ball
    let ball = app
        .world_mut()
        .spawn((Ball, GlobalTransform::default(), Transform::default()))
        .id();

    // Spawn Brick A at (-8.25, 2.0, 9.0) [Row 5, Col 6]
    let brick_a_pos = Vec3::new(-8.25, 2.0, 9.0);
    let brick_a = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(36),
            CountsTowardsCompletion,
            Transform::from_translation(brick_a_pos),
            GlobalTransform::from_translation(brick_a_pos),
        ))
        .id();

    // Spawn Brick B at (-8.25, 2.0, 1.0) [Row 5, Col 10]
    let brick_b_pos = Vec3::new(-8.25, 2.0, 1.0);
    let _brick_b = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(36),
            CountsTowardsCompletion,
            Transform::from_translation(brick_b_pos),
            GlobalTransform::from_translation(brick_b_pos),
        ))
        .id();

    // Trigger collision with *Brick A* (Col 6)
    app.world_mut()
        .resource_mut::<Messages<CollisionEvent>>()
        .write(CollisionEvent::Started(
            ball,
            brick_a,
            CollisionEventFlags::empty(),
        ));

    app.update();

    // Check PendingMerkabaSpawns
    let pending = app.world().resource::<PendingMerkabaSpawns>();
    assert_eq!(
        pending.entries.len(),
        1,
        "Should have 1 pending merkaba spawn"
    );

    let spawn_pos = pending.entries[0].position;
    println!("Spawned at: {:?} (Expected: {:?})", spawn_pos, brick_a_pos);

    // If bug exists, it might spawn at brick_b_pos (Col 10)
    if spawn_pos == brick_b_pos {
        panic!("Bug Reproduced: Spawned at Brick B position (Col 10) instead of Brick A (Col 6)!");
    }

    assert_eq!(
        spawn_pos, brick_a_pos,
        "Merkaba should spawn at brick A position"
    );
}
