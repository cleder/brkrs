use bevy::app::App;
use bevy::asset::AssetPlugin;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::CollisionEvent;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use brkrs::signals::SeaMineDetonationMessage;
use brkrs::systems::particle_fx::SeaMineExplosionBurst;
use brkrs::systems::sea_mine::SeaMine;
use brkrs::{Ball, Border, Paddle};

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(AssetPlugin::default())
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .add_message::<CollisionEvent>()
        .add_message::<brkrs::signals::SpawnSeaMineMessage>()
        .add_message::<SeaMineDetonationMessage>()
        .add_message::<brkrs::signals::SeaMineExplosionTriggered>()
        .add_plugins(brkrs::systems::particle_fx::ParticleFxPlugin)
        .add_plugins(brkrs::systems::sea_mine::SeaMinePlugin);
    app
}

#[test]
fn sea_mine_wall_trigger_emits_single_burst_marker() {
    let mut app = test_app();

    let mine = app
        .world_mut()
        .spawn((SeaMine, Transform::from_xyz(0.0, 2.0, 0.0)))
        .id();
    let wall = app.world_mut().spawn(Border).id();

    app.world_mut()
        .resource_mut::<Messages<CollisionEvent>>()
        .write(CollisionEvent::Started(
            mine,
            wall,
            CollisionEventFlags::empty(),
        ));

    app.update();

    let burst_count = {
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<SeaMineExplosionBurst>>();
        query.iter(world).count()
    };

    assert_eq!(
        burst_count, 1,
        "Wall-triggered detonation should produce one explosion burst marker"
    );
}

#[test]
fn sea_mine_blast_radius_only_destroys_entities_inside_radius() {
    let mut app = test_app();

    let mine = app
        .world_mut()
        .spawn((SeaMine, Transform::from_xyz(0.0, 2.0, 0.0)))
        .id();
    let near_ball = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(5.0, 2.0, 0.0)))
        .id();
    let far_ball = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(40.0, 2.0, 0.0)))
        .id();
    let near_paddle = app
        .world_mut()
        .spawn((Paddle, Transform::from_xyz(0.0, 2.0, 10.0)))
        .id();
    let far_paddle = app
        .world_mut()
        .spawn((Paddle, Transform::from_xyz(0.0, 2.0, 45.0)))
        .id();

    app.world_mut()
        .resource_mut::<Messages<SeaMineDetonationMessage>>()
        .write(SeaMineDetonationMessage {
            entity: mine,
            position: Vec3::new(0.0, 2.0, 0.0),
            cause: brkrs::signals::SeaMineTriggerCause::Wall,
            radius: 30.0,
        });

    app.update();

    assert!(
        !app.world().entities().contains(near_ball),
        "Ball inside blast radius should be removed"
    );
    assert!(
        app.world().entities().contains(far_ball),
        "Ball outside blast radius should remain"
    );
    assert!(
        !app.world().entities().contains(near_paddle),
        "Paddle inside blast radius should be removed"
    );
    assert!(
        app.world().entities().contains(far_paddle),
        "Paddle outside blast radius should remain"
    );
}
