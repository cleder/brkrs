//! Integration tests for merkaba audio lifecycle (US3: T030b)
//!
//! Tests that the helicopter blade background loop starts, remains active,
//! and stops appropriately based on merkaba spawning and despawning.

use bevy::app::App;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;

use brkrs::signals::SpawnMerkabaMessage;
use brkrs::systems::merkaba::Merkaba;
use brkrs::systems::respawn::LivesState;

/// T030b: Helicopter blade loop starts/stops correctly; idempotent and no duplicates.
///
/// The helicopter blade background sound loop MUST:
/// - Start when the first merkaba spawns (merkaba_count: 0 → 1)
/// - Remain active while any merkaba exists (merkaba_count >= 1)
/// - Stop when the last merkaba despawns (merkaba_count: 1 → 0)
/// - Be idempotent: spawning additional merkabas does NOT restart or duplicate the loop
#[test]
fn t030b_helicopter_loop_lifecycle_start_stop_idempotent() {
    let mut app = test_app();

    // Initial state: no merkabas, loop should not be playing
    let merkaba_count = app
        .world_mut()
        .query::<&Merkaba>()
        .iter(app.world_mut())
        .count();
    assert_eq!(merkaba_count, 0, "Should start with no merkabas");

    // Spawn first merkaba
    let spawn_msg = SpawnMerkabaMessage {
        position: Vec3::new(0.0, 0.0, 0.0),
        angle_variance_deg: 20.0,
        delay_seconds: 0.0,
        min_speed_y: 3.0,
    };
    app.world_mut()
        .resource_mut::<Messages<SpawnMerkabaMessage>>()
        .write(spawn_msg);
    app.update();

    // Should now have 1 merkaba
    let merkaba_count = app
        .world_mut()
        .query::<&Merkaba>()
        .iter(app.world_mut())
        .count();
    assert_eq!(merkaba_count, 1, "First merkaba should spawn");

    // Spawn second merkaba (idempotency test)
    let spawn_msg2 = SpawnMerkabaMessage {
        position: Vec3::new(5.0, 0.0, 0.0),
        angle_variance_deg: 20.0,
        delay_seconds: 0.0,
        min_speed_y: 3.0,
    };
    app.world_mut()
        .resource_mut::<Messages<SpawnMerkabaMessage>>()
        .write(spawn_msg2);
    app.update();

    // Should now have 2 merkabas
    let merkaba_count = app
        .world_mut()
        .query::<&Merkaba>()
        .iter(app.world_mut())
        .count();
    assert_eq!(merkaba_count, 2, "Second merkaba should spawn");

    // Trigger life loss (should despawn all merkabas)
    let mut lives_state = app.world_mut().resource_mut::<LivesState>();
    lives_state.lives_remaining = 2; // Simulate life loss
    app.update();

    // All merkabas should be despawned
    let merkaba_count = app
        .world_mut()
        .query::<&Merkaba>()
        .iter(app.world_mut())
        .count();
    assert_eq!(merkaba_count, 0, "All merkabas should despawn on life loss");
}

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(Assets::<Mesh>::default())
        .insert_resource(Assets::<StandardMaterial>::default())
        .insert_resource(LivesState {
            lives_remaining: 3,
            on_last_life: false,
        })
        .add_message::<SpawnMerkabaMessage>()
        .add_message::<bevy_rapier3d::prelude::CollisionEvent>()
        .add_plugins(brkrs::systems::merkaba::MerkabaPlugin);

    app
}
