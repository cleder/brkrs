use bevy::ecs::message::Messages;
/// Integration test to verify the complete gravity brick flow:
/// 1. Brick destroyed → BrickDestroyed message sent
/// 2. brick_destruction_gravity_handler reads message → sends GravityChanged
/// 3. gravity_application_system reads GravityChanged → updates GravityConfiguration
/// 4. apply_gravity_to_physics reads GravityConfiguration → updates RapierConfiguration
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use brkrs::signals::BrickDestroyed;
use brkrs::systems::gravity::GravityChanged;
use brkrs::{GravityBrick, GravityConfiguration};

#[test]
fn test_complete_gravity_flow() {
    // Create minimal app with only the systems we need
    let mut app = App::new();

    // Add minimal plugins - we DON'T add RapierPhysicsPlugin to avoid all its dependencies
    app.add_plugins(bevy::MinimalPlugins);

    // Add message types
    app.add_message::<BrickDestroyed>();
    app.add_message::<GravityChanged>();

    // Add resource
    app.init_resource::<GravityConfiguration>();

    // Spawn an entity with RapierConfiguration component (simulating what RapierPhysicsPlugin would do)
    let mut rapier_config = RapierConfiguration::new(1.0);
    rapier_config.gravity = Vec3::ZERO;
    app.world_mut().spawn(rapier_config);

    // Add the gravity systems in correct order
    app.add_systems(
        Update,
        brkrs::systems::gravity::brick_destruction_gravity_handler,
    );
    app.add_systems(
        Update,
        brkrs::systems::gravity::gravity_application_system
            .after(brkrs::systems::gravity::brick_destruction_gravity_handler),
    );
    app.add_systems(
        Update,
        brkrs::systems::gravity::apply_gravity_to_physics
            .after(brkrs::systems::gravity::gravity_application_system),
    );

    // Spawn a gravity brick entity
    let brick_entity = app
        .world_mut()
        .spawn(GravityBrick {
            index: 23,
            gravity: Vec3::new(10.0, 0.0, 0.0),
        })
        .id();

    // Verify initial state - gravity should be default (zero)
    let initial_gravity = app.world().resource::<GravityConfiguration>().current;
    assert_eq!(
        initial_gravity,
        Vec3::ZERO,
        "Initial gravity should be zero"
    );

    // Send BrickDestroyed message for this gravity brick
    {
        let mut msgs = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();
        msgs.write(BrickDestroyed {
            brick_entity,
            brick_type: 23,
            destroyed_by: None,
        });
    }

    // Run one update cycle - this should process all systems
    app.update();

    // Verify GravityConfiguration was updated
    let updated_gravity = app.world().resource::<GravityConfiguration>().current;
    assert_eq!(
        updated_gravity,
        Vec3::new(10.0, 0.0, 0.0),
        "GravityConfiguration should be updated to 10 on X"
    );

    // Verify RapierConfiguration was also updated
    let rapier_gravity = {
        let mut query = app.world_mut().query::<&RapierConfiguration>();
        let mut iter = query.iter(app.world());
        let first = iter
            .next()
            .expect("Expected at least one RapierConfiguration");
        assert!(
            iter.next().is_none(),
            "Expected exactly one RapierConfiguration"
        );
        first.gravity
    };

    assert_eq!(
        rapier_gravity,
        Vec3::new(10.0, 0.0, 0.0),
        "RapierConfiguration should match GravityConfiguration"
    );
}

#[test]
fn test_gravity_zero_brick() {
    let mut app = App::new();

    app.add_plugins(bevy::MinimalPlugins);
    app.add_message::<BrickDestroyed>();
    app.add_message::<GravityChanged>();
    app.init_resource::<GravityConfiguration>();

    // Spawn RapierConfiguration component manually
    let mut rapier_config = RapierConfiguration::new(1.0);
    rapier_config.gravity = Vec3::ZERO;
    app.world_mut().spawn(rapier_config);

    app.add_systems(
        Update,
        brkrs::systems::gravity::brick_destruction_gravity_handler,
    );
    app.add_systems(
        Update,
        brkrs::systems::gravity::gravity_application_system
            .after(brkrs::systems::gravity::brick_destruction_gravity_handler),
    );
    app.add_systems(
        Update,
        brkrs::systems::gravity::apply_gravity_to_physics
            .after(brkrs::systems::gravity::gravity_application_system),
    );

    // Set initial gravity to something non-zero
    app.world_mut()
        .resource_mut::<GravityConfiguration>()
        .current = Vec3::new(0.0, 10.0, 0.0);

    // Spawn zero gravity brick (index 21)
    let brick_entity = app
        .world_mut()
        .spawn(GravityBrick {
            index: 21,
            gravity: Vec3::ZERO,
        })
        .id();

    // Send destruction message
    {
        let mut msgs = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();
        msgs.write(BrickDestroyed {
            brick_entity,
            brick_type: 21,
            destroyed_by: None,
        });
    }

    app.update();

    // Verify gravity changed to zero
    let gravity = app.world().resource::<GravityConfiguration>().current;
    assert_eq!(gravity, Vec3::ZERO, "Gravity should change to zero");

    let rapier_gravity = {
        let mut query = app.world_mut().query::<&RapierConfiguration>();
        let mut iter = query.iter(app.world());
        let first = iter
            .next()
            .expect("Expected at least one RapierConfiguration");
        assert!(
            iter.next().is_none(),
            "Expected exactly one RapierConfiguration"
        );
        first.gravity
    };
    assert_eq!(rapier_gravity, Vec3::ZERO, "Rapier gravity should be zero");
}

#[test]
fn test_sequential_gravity_changes() {
    let mut app = App::new();

    app.add_plugins(bevy::MinimalPlugins);
    app.add_message::<BrickDestroyed>();
    app.add_message::<GravityChanged>();
    app.init_resource::<GravityConfiguration>();

    // Spawn RapierConfiguration component manually
    let mut rapier_config = RapierConfiguration::new(1.0);
    rapier_config.gravity = Vec3::ZERO;
    app.world_mut().spawn(rapier_config);

    app.add_systems(
        Update,
        brkrs::systems::gravity::brick_destruction_gravity_handler,
    );
    app.add_systems(
        Update,
        brkrs::systems::gravity::gravity_application_system
            .after(brkrs::systems::gravity::brick_destruction_gravity_handler),
    );
    app.add_systems(
        Update,
        brkrs::systems::gravity::apply_gravity_to_physics
            .after(brkrs::systems::gravity::gravity_application_system),
    );

    // Create three different gravity bricks
    let brick_21 = app
        .world_mut()
        .spawn(GravityBrick {
            index: 21,
            gravity: Vec3::ZERO,
        })
        .id();

    let brick_23 = app
        .world_mut()
        .spawn(GravityBrick {
            index: 23,
            gravity: Vec3::new(10.0, 0.0, 0.0),
        })
        .id();

    let brick_24 = app
        .world_mut()
        .spawn(GravityBrick {
            index: 24,
            gravity: Vec3::new(20.0, 0.0, 0.0),
        })
        .id();

    // Destroy brick 21 (zero gravity)
    {
        let mut msgs = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();
        msgs.write(BrickDestroyed {
            brick_entity: brick_21,
            brick_type: 21,
            destroyed_by: None,
        });
    }
    app.update();

    let gravity1 = app.world().resource::<GravityConfiguration>().current;
    assert_eq!(gravity1, Vec3::ZERO);

    // Destroy brick 23 (earth gravity)
    {
        let mut msgs = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();
        msgs.write(BrickDestroyed {
            brick_entity: brick_23,
            brick_type: 23,
            destroyed_by: None,
        });
    }
    app.update();

    let gravity2 = app.world().resource::<GravityConfiguration>().current;
    assert_eq!(gravity2, Vec3::new(10.0, 0.0, 0.0));

    // Destroy brick 24 (high gravity) - last one wins
    {
        let mut msgs = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();
        msgs.write(BrickDestroyed {
            brick_entity: brick_24,
            brick_type: 24,
            destroyed_by: None,
        });
    }
    app.update();

    let gravity3 = app.world().resource::<GravityConfiguration>().current;
    assert_eq!(gravity3, Vec3::new(20.0, 0.0, 0.0));

    // Verify Rapier config matches final gravity
    let rapier_gravity = {
        let mut query = app.world_mut().query::<&RapierConfiguration>();
        let mut iter = query.iter(app.world());
        let first = iter
            .next()
            .expect("Expected at least one RapierConfiguration");
        assert!(
            iter.next().is_none(),
            "Expected exactly one RapierConfiguration"
        );
        first.gravity
    };
    assert_eq!(rapier_gravity, Vec3::new(20.0, 0.0, 0.0));
}
