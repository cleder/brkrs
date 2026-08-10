use bevy::app::App;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::CollisionEvent;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use brkrs::level_format::{INDESTRUCTIBLE_BRICK, SEA_MINE_BRICK};
use brkrs::register_brick_collision_systems;
use brkrs::systems::sea_mine::SeaMine;
use brkrs::{Ball, Brick, BrickTypeId, CountsTowardsCompletion};

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .add_message::<CollisionEvent>()
        .add_message::<brkrs::signals::SpawnSeaMineMessage>()
        .add_message::<brkrs::signals::SeaMineDetonationMessage>()
        .add_plugins(brkrs::systems::sea_mine::SeaMinePlugin);

    register_brick_collision_systems(&mut app);
    app
}

#[test]
fn sea_mine_brick_spawns_exactly_one_mine_and_despawns_brick() {
    let mut app = test_app();

    let ball = app.world_mut().spawn(Ball).id();
    let brick = app
        .world_mut()
        .spawn((
            Brick,
            BrickTypeId(SEA_MINE_BRICK),
            CountsTowardsCompletion,
            Transform::from_xyz(2.0, 2.0, 1.0),
            GlobalTransform::default(),
        ))
        .id();

    app.world_mut()
        .resource_mut::<Messages<CollisionEvent>>()
        .write(CollisionEvent::Started(
            ball,
            brick,
            CollisionEventFlags::empty(),
        ));

    app.update();
    app.update();

    assert!(
        !app.world().entities().contains(brick),
        "Sea-mine brick should despawn after ball collision"
    );

    let mine_count = {
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<SeaMine>>();
        query.iter(world).count()
    };
    assert_eq!(
        mine_count, 1,
        "Sea-mine brick should spawn exactly one sea mine"
    );
}

#[test]
fn sea_mine_brick_index_participates_in_destructible_range() {
    assert_eq!(SEA_MINE_BRICK, 31, "Sea-mine brick index should be 31");
    assert!(
        SEA_MINE_BRICK < INDESTRUCTIBLE_BRICK,
        "Sea-mine brick should remain in destructible completion range"
    );
}
