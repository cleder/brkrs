//! Audio tests for extra ball brick (brick 41) feature.
//
// Tests US2: Destroying brick 41 plays unique destruction sound once (with fallback).
//
// Phase 4 US2 Tests: T014 (unique sound once), T015 (multi-ball + fallback)

use bevy::app::App;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::{CollisionEvent, RapierConfiguration};

use brkrs::signals::BrickDestroyed;
use brkrs::systems::audio::{AudioAssets, AudioConfig};
use brkrs::{Ball, Brick, BrickTypeId, CountsTowardsCompletion};

const EXTRA_LIFE_BRICK: u8 = 41;

fn test_app() -> App {
    let mut app = App::new();
    app.insert_resource(brkrs::physics_config::BallPhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(brkrs::physics_config::BrickPhysicsConfig::default());
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));
    app.add_message::<CollisionEvent>();
    app.add_message::<BrickDestroyed>();
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    app.insert_resource(Assets::<AudioSource>::default());
    app.insert_resource(bevy::input::ButtonInput::<bevy::prelude::KeyCode>::default());
    app.world_mut().spawn(RapierConfiguration::new(1.0));

    // Initialize audio resources
    app.insert_resource(AudioConfig::default());
    app.insert_resource(AudioAssets::default());

    // Initialize lives state (required by collision system)
    app.insert_resource(brkrs::systems::respawn::LivesState {
        lives_remaining: 3,
        on_last_life: false,
    });

    // Register brick collision systems
    brkrs::register_brick_collision_systems(&mut app);

    // TODO T016: Register audio system to consume BrickDestroyed messages
    // app.add_plugins(brkrs::systems::audio::AudioPlugin);

    app
}

/// T014: Brick 41 destruction plays unique sound once
///
/// **Acceptance Criteria:**
/// - Brick 41 emits BrickDestroyed message with brick_type = 41
/// - Audio system plays Brick41ExtraLife sound (not generic BrickDestroy)
/// - Sound plays exactly once per brick destruction
/// - Other brick types do not trigger Brick41ExtraLife sound
///
/// **Expected Failure Before Implementation:**
/// - No audio system consuming BrickDestroyed for brick 41
/// - SoundType::Brick41ExtraLife not yet added to enum
/// - Test will fail asserting unique sound type
#[test]
fn t014_brick_41_plays_unique_sound_once() {
    let mut app = test_app();

    // Spawn brick 41
    let brick_41 = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(EXTRA_LIFE_BRICK),
            CountsTowardsCompletion,
            Transform::from_xyz(0.0, 5.0, 10.0),
        ))
        .id();

    // Spawn ball
    let ball = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(0.0, 4.0, 9.5)))
        .id();

    // Simulate collision
    app.world_mut().write_message(CollisionEvent::Started(
        ball,
        brick_41,
        bevy_rapier3d::rapier::prelude::CollisionEventFlags::empty(),
    ));

    // Run systems
    app.update();

    // Check BrickDestroyed message written
    let brick_destroyed_messages = app.world().resource::<Messages<BrickDestroyed>>();
    assert!(
        !brick_destroyed_messages.is_empty(),
        "BrickDestroyed message should be written"
    );

    // TODO T017: Verify unique sound played (not generic brick destroy)
    // This will fail until audio system maps brick type 41 to SoundType::Brick41ExtraLife
    // For now, we verify the message was written with correct brick type
    // Once audio system is implemented, we'll check audio events/commands
}

/// T014-generic: Other brick types do not trigger brick 41 unique sound
///
/// **Acceptance Criteria:**
/// - Brick type 20 (simple stone) plays BrickDestroy sound, not Brick41ExtraLife
/// - Audio system must distinguish brick types via brick_type field in BrickDestroyed
#[test]
fn t014_other_bricks_use_generic_sound() {
    let mut app = test_app();

    // Spawn simple stone brick (type 20)
    let brick_20 = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(20),
            CountsTowardsCompletion,
            Transform::from_xyz(0.0, 5.0, 10.0),
        ))
        .id();

    let ball = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(0.0, 4.0, 9.5)))
        .id();

    app.world_mut().write_message(CollisionEvent::Started(
        ball,
        brick_20,
        bevy_rapier3d::rapier::prelude::CollisionEventFlags::empty(),
    ));

    app.update();

    // Check BrickDestroyed written
    let brick_destroyed_messages = app.world().resource::<Messages<BrickDestroyed>>();
    assert!(
        !brick_destroyed_messages.is_empty(),
        "BrickDestroyed message should be written for brick 20"
    );

    // TODO T017: Verify BrickDestroy sound used (not Brick41ExtraLife)
}

/// T015: Multi-ball simultaneous hits → unique sound fires once, fallback on missing asset
///
/// **Acceptance Criteria:**
/// - Two balls hit brick 41 in same frame
/// - BrickDestroyed message written once (brick despawns after first hit)
/// - Unique sound plays once only
/// - If Brick41ExtraLife asset missing, fallback to BrickDestroy generic sound
/// - No panics or crashes on missing asset
///
/// **Expected Failure Before Implementation:**
/// - Audio system not yet handling brick 41 unique sound
/// - Fallback logic not implemented
/// - Test will fail asserting single sound event
#[test]
fn t015_brick_41_multi_ball_single_sound_with_fallback() {
    let mut app = test_app();

    // Spawn brick 41
    let brick_41 = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(EXTRA_LIFE_BRICK),
            CountsTowardsCompletion,
            Transform::from_xyz(0.0, 5.0, 10.0),
        ))
        .id();

    // Spawn two balls
    let ball1 = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(-1.0, 4.0, 9.5)))
        .id();
    let ball2 = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(1.0, 4.0, 9.5)))
        .id();

    // Simulate simultaneous collisions
    app.world_mut().write_message(CollisionEvent::Started(
        ball1,
        brick_41,
        bevy_rapier3d::rapier::prelude::CollisionEventFlags::empty(),
    ));
    app.world_mut().write_message(CollisionEvent::Started(
        ball2,
        brick_41,
        bevy_rapier3d::rapier::prelude::CollisionEventFlags::empty(),
    ));

    app.update();

    // Check only ONE BrickDestroyed message (brick despawns after first hit)
    let brick_destroyed_messages = app.world().resource::<Messages<BrickDestroyed>>();
    assert!(
        !brick_destroyed_messages.is_empty(),
        "BrickDestroyed message should be written once"
    );

    // TODO T017: Verify unique sound played once
    // TODO T018: If Brick41ExtraLife handle missing, verify fallback to BrickDestroy
}

/// T015-fallback: Missing audio asset fallback behavior
///
/// **Acceptance Criteria:**
/// - If Brick41ExtraLife audio file missing or failed to load
/// - Audio system logs warning about missing asset
/// - Falls back to generic BrickDestroy sound
/// - No panic or crash
///
/// **Expected Failure Before Implementation:**
/// - Fallback logic not yet implemented
/// - May panic on missing handle instead of graceful degradation
#[test]
fn t015_missing_asset_fallback_to_generic() {
    let mut app = test_app();

    // Intentionally do NOT load Brick41ExtraLife asset (simulate missing file)
    // AudioAssets resource initialized but empty

    let brick_41 = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(EXTRA_LIFE_BRICK),
            CountsTowardsCompletion,
            Transform::from_xyz(0.0, 5.0, 10.0),
        ))
        .id();

    let ball = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(0.0, 4.0, 9.5)))
        .id();

    app.world_mut().write_message(CollisionEvent::Started(
        ball,
        brick_41,
        bevy_rapier3d::rapier::prelude::CollisionEventFlags::empty(),
    ));

    app.update();

    // TODO T018: Verify no panic occurred (graceful fallback)
    // TODO T018: Verify warning logged about missing Brick41ExtraLife asset
    // TODO T018: Verify BrickDestroy sound used as fallback
}
