use bevy::prelude::*;
use brkrs::signals::BrickDestroyed;
use brkrs::systems::gravity::GravityChanged;
use brkrs::GravityConfiguration;

#[test]
fn test_gravity_message_sequence_applies_last() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Messages
    app.add_message::<BrickDestroyed>();
    app.add_message::<GravityChanged>();

    // Resources
    app.init_resource::<GravityConfiguration>();

    // Systems under test
    app.add_systems(
        Update,
        brkrs::systems::gravity::brick_destruction_gravity_handler,
    );
    app.add_systems(
        Update,
        brkrs::systems::gravity::gravity_application_system
            .after(brkrs::systems::gravity::brick_destruction_gravity_handler),
    );

    // Simulate two BrickDestroyed messages in same frame
    {
        let mut msgs = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();
        msgs.write(BrickDestroyed {
            brick_entity: Entity::from_bits(1),
            brick_type: 22,
            destroyed_by: None,
        });
        msgs.write(BrickDestroyed {
            brick_entity: Entity::from_bits(2),
            brick_type: 23,
            destroyed_by: None,
        });
    }

    // Run one update to process both messages
    app.update();

    let cfg = app.world().resource::<GravityConfiguration>().current;
    // Expect last processed message (brick_type 23 -> gravity 10.0 on X) to be applied
    assert_eq!(cfg, bevy::prelude::Vec3::new(10.0, 0.0, 0.0));
}
