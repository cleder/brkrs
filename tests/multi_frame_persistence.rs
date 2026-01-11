//! Multi-frame state persistence tests.
//!
//! These tests verify that runtime state changes persist across multiple frames,
//! per Constitution v1.6.0 mandate: "Tests for runtime state changes MUST verify
//! that the change persists across multiple frames by calling `app.update()`
//! repeatedly after the change."
//!
//! These tests exist to catch bugs where per-frame systems unconditionally
//! overwrite runtime state (like the gravity loader bug in 020-gravity-bricks).

use bevy::prelude::*;
use brkrs::level_loader::{CurrentLevel, LevelDefinition};
use brkrs::signals::BrickDestroyed;
use brkrs::systems::gravity::{
    brick_destruction_gravity_handler,
    gravity_application_system,
    gravity_configuration_loader_system,
    GravityChanged,
    // Import gravity constants for test assertions
    BRICK_TYPE_GRAVITY_HIGH,
    BRICK_TYPE_GRAVITY_LOW,
    BRICK_TYPE_GRAVITY_MEDIUM,
    BRICK_TYPE_GRAVITY_ZERO,
    GRAVITY_HIGH,
    GRAVITY_LOW,
    GRAVITY_MEDIUM,
    GRAVITY_ZERO,
};
use brkrs::GravityConfiguration;

/// Number of frames to run for persistence checks.
/// 10 frames is enough to catch per-frame overwrite bugs.
const PERSISTENCE_FRAMES: usize = 10;

/// Test that gravity changes from brick destruction persist across multiple frames
/// when the loader system is also running.
///
/// This is the exact scenario that caused the 020-gravity-bricks bug:
/// the loader was resetting `current` every frame, overwriting brick-induced changes.
#[test]
fn gravity_change_persists_with_loader_running() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Setup level with default gravity (2, 0, 0)
    let level_def = LevelDefinition {
        number: 1,
        gravity: Some((2.0, 0.0, 0.0)),
        matrix: vec![vec![]],
        #[cfg(feature = "texture_manifest")]
        presentation: None,
        description: None,
        author: None,
    };

    app.insert_resource(CurrentLevel(level_def));
    app.init_resource::<GravityConfiguration>();
    app.add_message::<BrickDestroyed>();
    app.add_message::<GravityChanged>();

    // Register ALL gravity-related systems (loader + brick handler + application)
    // This matches production system composition
    app.add_systems(
        Update,
        (
            gravity_configuration_loader_system,
            brick_destruction_gravity_handler,
            gravity_application_system
                .after(brick_destruction_gravity_handler)
                .after(gravity_configuration_loader_system),
        ),
    );

    // Initial update to load level gravity
    app.update();

    // Verify initial state
    let cfg = app.world().resource::<GravityConfiguration>();
    assert_eq!(
        cfg.current, GRAVITY_LOW,
        "Initial gravity should be level default"
    );
    assert_eq!(
        cfg.last_level_number,
        Some(1),
        "Loader should track level number"
    );

    // Emit gravity brick destruction (brick 23 -> gravity 10)
    {
        let mut msgs = app
            .world_mut()
            .resource_mut::<bevy::ecs::message::Messages<BrickDestroyed>>();
        msgs.write(BrickDestroyed {
            brick_entity: Entity::from_bits(100),
            brick_type: BRICK_TYPE_GRAVITY_MEDIUM,
            destroyed_by: None,
        });
    }

    // Process the brick destruction
    app.update();

    // Verify gravity changed
    let cfg = app.world().resource::<GravityConfiguration>();
    assert_eq!(
        cfg.current, GRAVITY_MEDIUM,
        "Gravity should change after brick 23 destroyed"
    );

    // Run multiple frames - the critical persistence check
    for frame in 0..PERSISTENCE_FRAMES {
        app.update();
        let cfg = app.world().resource::<GravityConfiguration>();
        assert_eq!(
            cfg.current,
            GRAVITY_MEDIUM,
            "Gravity should persist on frame {} (not revert to level default)",
            frame + 1
        );
    }
}

/// Test that gravity changes persist when no loader is present.
/// This establishes baseline behavior for the gravity application system.
#[test]
fn gravity_change_persists_without_loader() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.init_resource::<GravityConfiguration>();
    app.add_message::<BrickDestroyed>();
    app.add_message::<GravityChanged>();

    // Only register brick handler and application (no loader)
    app.add_systems(
        Update,
        (
            brick_destruction_gravity_handler,
            gravity_application_system.after(brick_destruction_gravity_handler),
        ),
    );

    app.update();

    // Initial gravity is zero (no level loaded)
    assert_eq!(
        app.world().resource::<GravityConfiguration>().current,
        Vec3::ZERO
    );

    // Emit gravity brick destruction
    {
        let mut msgs = app
            .world_mut()
            .resource_mut::<bevy::ecs::message::Messages<BrickDestroyed>>();
        msgs.write(BrickDestroyed {
            brick_entity: Entity::from_bits(100),
            brick_type: BRICK_TYPE_GRAVITY_LOW,
            destroyed_by: None,
        });
    }

    app.update();
    assert_eq!(
        app.world().resource::<GravityConfiguration>().current,
        GRAVITY_LOW
    );

    // Persistence check
    for frame in 0..PERSISTENCE_FRAMES {
        app.update();
        assert_eq!(
            app.world().resource::<GravityConfiguration>().current,
            GRAVITY_LOW,
            "Gravity should persist on frame {}",
            frame + 1
        );
    }
}

/// Test that multiple gravity changes in sequence all persist correctly.
/// Each change should "stick" until the next one occurs.
#[test]
fn sequential_gravity_changes_persist() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    let level_def = LevelDefinition {
        number: 1,
        gravity: Some((0.0, 0.0, 0.0)),
        matrix: vec![vec![]],
        #[cfg(feature = "texture_manifest")]
        presentation: None,
        description: None,
        author: None,
    };

    app.insert_resource(CurrentLevel(level_def));
    app.init_resource::<GravityConfiguration>();
    app.add_message::<BrickDestroyed>();
    app.add_message::<GravityChanged>();

    app.add_systems(
        Update,
        (
            gravity_configuration_loader_system,
            brick_destruction_gravity_handler,
            gravity_application_system
                .after(brick_destruction_gravity_handler)
                .after(gravity_configuration_loader_system),
        ),
    );

    app.update(); // Load level

    // Sequence: 21 (0) -> 22 (2) -> 23 (10) -> 24 (20) -> 21 (0)
    let sequence = [
        (BRICK_TYPE_GRAVITY_ZERO, GRAVITY_ZERO),
        (BRICK_TYPE_GRAVITY_LOW, GRAVITY_LOW),
        (BRICK_TYPE_GRAVITY_MEDIUM, GRAVITY_MEDIUM),
        (BRICK_TYPE_GRAVITY_HIGH, GRAVITY_HIGH),
        (BRICK_TYPE_GRAVITY_ZERO, GRAVITY_ZERO),
    ];

    for (i, (brick_type, expected_gravity)) in sequence.iter().enumerate() {
        // Emit brick destruction
        {
            let mut msgs = app
                .world_mut()
                .resource_mut::<bevy::ecs::message::Messages<BrickDestroyed>>();
            msgs.write(BrickDestroyed {
                brick_entity: Entity::from_bits(100 + i as u64),
                brick_type: *brick_type,
                destroyed_by: None,
            });
        }

        app.update();

        // Verify immediate change
        assert_eq!(
            app.world().resource::<GravityConfiguration>().current,
            *expected_gravity,
            "Step {}: gravity should be {:?} after brick {}",
            i + 1,
            expected_gravity,
            brick_type
        );

        // Verify persistence
        for frame in 0..5 {
            app.update();
            assert_eq!(
                app.world().resource::<GravityConfiguration>().current,
                *expected_gravity,
                "Step {}, frame {}: gravity {:?} should persist",
                i + 1,
                frame + 1,
                expected_gravity
            );
        }
    }
}

/// Test that level transitions properly reset gravity while preserving
/// the idempotence behavior for same-level updates.
#[test]
fn level_transition_resets_gravity_then_persists() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    let level_1 = LevelDefinition {
        number: 1,
        gravity: Some((2.0, 0.0, 0.0)),
        matrix: vec![vec![]],
        #[cfg(feature = "texture_manifest")]
        presentation: None,
        description: None,
        author: None,
    };

    app.insert_resource(CurrentLevel(level_1));
    app.init_resource::<GravityConfiguration>();
    app.add_message::<BrickDestroyed>();
    app.add_message::<GravityChanged>();

    app.add_systems(
        Update,
        (
            gravity_configuration_loader_system,
            brick_destruction_gravity_handler,
            gravity_application_system
                .after(brick_destruction_gravity_handler)
                .after(gravity_configuration_loader_system),
        ),
    );

    app.update();
    assert_eq!(
        app.world().resource::<GravityConfiguration>().current,
        GRAVITY_LOW
    );

    // Change gravity via brick
    {
        let mut msgs = app
            .world_mut()
            .resource_mut::<bevy::ecs::message::Messages<BrickDestroyed>>();
        msgs.write(BrickDestroyed {
            brick_entity: Entity::from_bits(100),
            brick_type: BRICK_TYPE_GRAVITY_MEDIUM,
            destroyed_by: None,
        });
    }
    app.update();
    assert_eq!(
        app.world().resource::<GravityConfiguration>().current,
        GRAVITY_MEDIUM
    );

    // Transition to level 2 with different gravity
    let level_2 = LevelDefinition {
        number: 2,
        gravity: Some((5.0, 0.0, 0.0)),
        matrix: vec![vec![]],
        #[cfg(feature = "texture_manifest")]
        presentation: None,
        description: None,
        author: None,
    };
    app.insert_resource(CurrentLevel(level_2));

    app.update();

    // Should reset to level 2's gravity
    let cfg = app.world().resource::<GravityConfiguration>();
    assert_eq!(
        cfg.current,
        Vec3::new(5.0, 0.0, 0.0),
        "Gravity should reset to level 2 default"
    );
    assert_eq!(
        cfg.last_level_number,
        Some(2),
        "Should track new level number"
    );

    // Verify persistence on new level
    for frame in 0..PERSISTENCE_FRAMES {
        app.update();
        assert_eq!(
            app.world().resource::<GravityConfiguration>().current,
            Vec3::new(5.0, 0.0, 0.0),
            "Level 2 gravity should persist on frame {}",
            frame + 1
        );
    }
}
