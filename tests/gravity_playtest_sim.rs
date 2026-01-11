use bevy::prelude::*;
use brkrs::signals::BrickDestroyed;
use brkrs::systems::gravity::{
    apply_gravity_to_physics, brick_destruction_gravity_handler, gravity_application_system,
};
use brkrs::GravityConfiguration;

#[test]
fn gravity_playtest_sequence_simulation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_message::<BrickDestroyed>();
    app.add_message::<brkrs::systems::gravity::GravityChanged>();
    app.init_resource::<GravityConfiguration>();

    // Register gravity systems
    app.add_systems(Update, brick_destruction_gravity_handler);
    app.add_systems(
        Update,
        gravity_application_system.after(brick_destruction_gravity_handler),
    );
    app.add_systems(
        Update,
        apply_gravity_to_physics.after(gravity_application_system),
    );

    // Initial gravity should be zero
    assert_eq!(
        app.world().resource::<GravityConfiguration>().current,
        Vec3::ZERO
    );

    // Sequence: non-gravity brick (20) -> gravity brick 23 -> wait -> non-gravity 20 -> gravity brick 21
    // Step 1: emit 20 (no effect)
    {
        let mut msgs = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();
        msgs.write(BrickDestroyed {
            brick_entity: Entity::from_bits(100),
            brick_type: 20,
            destroyed_by: None,
        });
    }
    app.update();
    assert_eq!(
        app.world().resource::<GravityConfiguration>().current,
        Vec3::ZERO
    );

    // Step 2: emit 23 -> 10 on X
    {
        let mut msgs = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();
        msgs.write(BrickDestroyed {
            brick_entity: Entity::from_bits(101),
            brick_type: 23,
            destroyed_by: None,
        });
    }
    app.update();
    assert_eq!(
        app.world().resource::<GravityConfiguration>().current,
        Vec3::new(10.0, 0.0, 0.0)
    );

    // Step a few frames to ensure no revert
    for _ in 0..5 {
        app.update();
    }
    assert_eq!(
        app.world().resource::<GravityConfiguration>().current,
        Vec3::new(10.0, 0.0, 0.0)
    );

    // Step 3: emit 20 again (should not affect)
    {
        let mut msgs = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();
        msgs.write(BrickDestroyed {
            brick_entity: Entity::from_bits(102),
            brick_type: 20,
            destroyed_by: None,
        });
    }
    app.update();
    assert_eq!(
        app.world().resource::<GravityConfiguration>().current,
        Vec3::new(10.0, 0.0, 0.0)
    );

    // Step 4: emit 21 -> zero gravity
    {
        let mut msgs = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();
        msgs.write(BrickDestroyed {
            brick_entity: Entity::from_bits(103),
            brick_type: 21,
            destroyed_by: None,
        });
    }
    app.update();
    assert_eq!(
        app.world().resource::<GravityConfiguration>().current,
        Vec3::ZERO
    );

    // Step a few frames to ensure no revert back to 10
    for _ in 0..5 {
        app.update();
    }
    assert_eq!(
        app.world().resource::<GravityConfiguration>().current,
        Vec3::ZERO
    );
}
