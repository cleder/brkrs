use bevy::app::App;
use bevy::ecs::message::{MessageReader, Messages};
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::{CollisionEvent, Velocity};
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use brkrs::signals::SpawnSeaMineMessage;
use brkrs::systems::respawn::LifeLostEvent;
use brkrs::systems::sea_mine::SeaMine;
use brkrs::{Ball, Border, Brick, BrickTypeId, Paddle};

#[derive(Resource, Default)]
struct LifeLossCounter(u32);

fn count_life_loss_events(
    mut events: MessageReader<LifeLostEvent>,
    mut counter: ResMut<LifeLossCounter>,
) {
    for _ in events.read() {
        counter.0 += 1;
    }
}

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<LifeLossCounter>()
        .add_message::<CollisionEvent>()
        .add_message::<SpawnSeaMineMessage>()
        .add_message::<brkrs::signals::SeaMineDetonationMessage>()
        .add_plugins(brkrs::systems::respawn::RespawnPlugin)
        .add_plugins(brkrs::systems::sea_mine::SeaMinePlugin)
        .add_systems(Update, count_life_loss_events);

    app
}

#[test]
fn sea_mine_maintains_minimum_motion_over_multiple_frames() {
    let mut app = test_app();

    app.world_mut()
        .resource_mut::<Messages<SpawnSeaMineMessage>>()
        .write(SpawnSeaMineMessage {
            position: Vec3::new(0.0, 2.0, 0.0),
            brick_entity: Entity::PLACEHOLDER,
            source_brick_type: 31,
        });

    app.update();

    for _ in 0..10 {
        app.update();
        let speed_and_spin = {
            let world = app.world_mut();
            let mut query = world.query_filtered::<&Velocity, With<SeaMine>>();
            query
                .iter(world)
                .map(|v| {
                    let horizontal = Vec3::new(v.linvel.x, 0.0, v.linvel.z).length();
                    (horizontal, v.angvel.y.abs())
                })
                .next()
        };

        let Some((horizontal_speed, angular_speed)) = speed_and_spin else {
            panic!("Sea mine should exist for motion-floor test");
        };

        assert!(
            horizontal_speed >= 3.0,
            "Sea mine horizontal speed should stay >= 3.0 u/s"
        );
        assert!(
            angular_speed >= std::f32::consts::PI,
            "Sea mine angular speed should stay >= 180 deg/s"
        );
    }
}

#[test]
fn sea_mine_only_detonates_on_valid_trigger_and_emits_single_life_loss() {
    let mut app = test_app();

    let mine = app
        .world_mut()
        .spawn((SeaMine, Transform::from_xyz(0.0, 2.0, 0.0)))
        .id();
    let non_trigger_brick = app.world_mut().spawn((Brick, BrickTypeId(20))).id();
    let paddle = app.world_mut().spawn((Paddle, Transform::default())).id();
    let ball = app.world_mut().spawn((Ball, Transform::default())).id();
    let _wall = app.world_mut().spawn(Border).id();

    // Non-trigger brick contact should not detonate.
    app.world_mut()
        .resource_mut::<Messages<CollisionEvent>>()
        .write(CollisionEvent::Started(
            mine,
            non_trigger_brick,
            CollisionEventFlags::empty(),
        ));
    app.update();

    assert!(
        app.world().entities().contains(mine),
        "Sea mine should ignore collisions with non-trigger bricks"
    );

    // Paddle contact should detonate and yield exactly one life-loss event.
    app.world_mut()
        .resource_mut::<Messages<CollisionEvent>>()
        .write(CollisionEvent::Started(
            mine,
            paddle,
            CollisionEventFlags::empty(),
        ));

    // Run several frames to ensure no duplicate life-loss handoff occurs.
    for _ in 0..4 {
        app.update();
    }

    assert!(
        !app.world().entities().contains(mine),
        "Sea mine should despawn after valid detonation"
    );
    assert!(
        !app.world().entities().contains(ball),
        "Ball inside blast radius should be removed"
    );

    let counter = app.world().resource::<LifeLossCounter>();
    assert_eq!(
        counter.0, 1,
        "Paddle blast should hand off exactly one life-loss event"
    );
}

#[test]
fn sea_mine_detonation_despawns_child_entities_recursively() {
    let mut app = test_app();

    let mine = app
        .world_mut()
        .spawn((SeaMine, Transform::from_xyz(0.0, 2.0, 0.0)))
        .id();
    let child = app
        .world_mut()
        .spawn(Transform::from_xyz(0.0, 0.0, 1.0))
        .id();
    app.world_mut().entity_mut(mine).add_child(child);

    app.world_mut()
        .resource_mut::<Messages<brkrs::signals::SeaMineDetonationMessage>>()
        .write(brkrs::signals::SeaMineDetonationMessage {
            entity: mine,
            position: Vec3::new(0.0, 2.0, 0.0),
            cause: brkrs::signals::SeaMineTriggerCause::Wall,
            radius: 10.0,
        });

    app.update();

    assert!(
        !app.world().entities().contains(child),
        "Sea-mine child entities should be removed when the mine detonates"
    );
}

#[test]
fn sea_mine_detonations_for_same_ball_only_emit_single_life_loss() {
    let mut app = test_app();

    let mine = app
        .world_mut()
        .spawn((SeaMine, Transform::from_xyz(0.0, 2.0, 0.0)))
        .id();
    let ball = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(1.0, 2.0, 0.0)))
        .id();
    let paddle = app
        .world_mut()
        .spawn((Paddle, Transform::from_xyz(100.0, 2.0, 0.0)))
        .id();

    app.world_mut()
        .resource_mut::<Messages<brkrs::signals::SeaMineDetonationMessage>>()
        .write(brkrs::signals::SeaMineDetonationMessage {
            entity: mine,
            position: Vec3::new(0.0, 2.0, 0.0),
            cause: brkrs::signals::SeaMineTriggerCause::Wall,
            radius: 10.0,
        });
    app.world_mut()
        .resource_mut::<Messages<brkrs::signals::SeaMineDetonationMessage>>()
        .write(brkrs::signals::SeaMineDetonationMessage {
            entity: mine,
            position: Vec3::new(0.0, 2.0, 0.0),
            cause: brkrs::signals::SeaMineTriggerCause::Wall,
            radius: 10.0,
        });

    for _ in 0..4 {
        app.update();
    }

    assert!(
        !app.world().entities().contains(ball),
        "Ball inside blast radius should be removed"
    );
    assert!(
        app.world().entities().contains(paddle),
        "Paddle outside blast radius should remain"
    );

    let counter = app.world().resource::<LifeLossCounter>();
    assert_eq!(
        counter.0, 1,
        "Repeated detonation messages for the same frame should not double-count a life loss"
    );
}

#[test]
fn sea_mine_last_ball_explosion_emits_life_loss_without_paddle_destruction() {
    let mut app = test_app();

    let mine = app
        .world_mut()
        .spawn((SeaMine, Transform::from_xyz(0.0, 2.0, 0.0)))
        .id();
    let ball = app
        .world_mut()
        .spawn((Ball, Transform::from_xyz(1.0, 2.0, 0.0)))
        .id();
    let paddle = app
        .world_mut()
        .spawn((Paddle, Transform::from_xyz(100.0, 2.0, 0.0)))
        .id();

    app.world_mut()
        .resource_mut::<Messages<brkrs::signals::SeaMineDetonationMessage>>()
        .write(brkrs::signals::SeaMineDetonationMessage {
            entity: mine,
            position: Vec3::new(0.0, 2.0, 0.0),
            cause: brkrs::signals::SeaMineTriggerCause::Wall,
            radius: 10.0,
        });

    for _ in 0..4 {
        app.update();
    }

    assert!(
        !app.world().entities().contains(ball),
        "Last ball inside blast radius should be removed"
    );
    assert!(
        app.world().entities().contains(paddle),
        "Paddle outside blast radius should remain"
    );

    let counter = app.world().resource::<LifeLossCounter>();
    assert_eq!(
        counter.0, 1,
        "Destroying the final ball with a sea mine should emit exactly one life-loss event"
    );
}
