use bevy::prelude::*;
use brkrs::signals::BrickDestroyed;

use brkrs::EmittedBrickDestroyed;

#[derive(Resource, Default)]
struct Count(pub usize);

fn collect_brick_destroyed(mut reader: MessageReader<BrickDestroyed>, mut cnt: ResMut<Count>) {
    for _msg in reader.read() {
        cnt.0 += 1;
    }
}

#[test]
fn test_duplicate_brick_destroy_emissions_are_deduped() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_message::<BrickDestroyed>();
    app.add_message::<bevy_rapier3d::prelude::CollisionEvent>();
    app.init_resource::<EmittedBrickDestroyed>();
    app.insert_resource(Count::default());

    // Spawn an entity and mark it for despawn
    let entity = app
        .world_mut()
        .spawn((
            brkrs::Brick,
            brkrs::BrickTypeId(23),
            brkrs::MarkedForDespawn,
        ))
        .id();

    // Simulate an immediate emission for this entity (as rotor path would do)
    {
        let mut emitted = app.world_mut().resource_mut::<EmittedBrickDestroyed>();
        assert!(emitted.0.insert(entity));
    }

    {
        let mut writer = app.world_mut().resource_mut::<Messages<BrickDestroyed>>();
        writer.write(BrickDestroyed {
            brick_entity: entity,
            brick_type: 23,
            destroyed_by: None,
        });
    }

    // Register the brick collision + despawn chain and a collector to count messages
    brkrs::register_brick_collision_systems(&mut app);
    // Run collection in PostUpdate so it runs after Update chain
    app.add_systems(PostUpdate, collect_brick_destroyed);

    // Run one frame - despawn system should run but skip emitting duplicate and collector will count
    app.update();

    let count = app.world().resource::<Count>().0;
    assert_eq!(
        count, 1,
        "Expected only one BrickDestroyed message after dedupe, got {}",
        count
    );
}
