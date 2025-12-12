use bevy::app::App;
use bevy::ecs::message::Messages;
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::CollisionEvent;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

use brkrs::systems::paddle_size::{
    PaddleSizeEffect, PaddleSizePlugin, SizeEffectType, BRICK_TYPE_30, BRICK_TYPE_32,
};
use brkrs::{Ball, Brick, BrickTypeId, Paddle};

use std::time::Duration;

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(Assets::<Mesh>::default())
        .insert_resource(Assets::<StandardMaterial>::default())
        .add_message::<CollisionEvent>()
        .add_plugins(PaddleSizePlugin);
    app
}

fn advance_time(app: &mut App, delta_secs: f32) {
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(Duration::from_secs_f32(delta_secs));
    // Note: After advancing, we need to call app.update() for systems to see the delta
}

fn trigger_collision(app: &mut App, e1: Entity, e2: Entity) {
    app.world_mut()
        .resource_mut::<Messages<CollisionEvent>>()
        .write(CollisionEvent::Started(
            e1,
            e2,
            CollisionEventFlags::empty(),
        ));
}

#[test]
fn brick_30_shrinks_paddle() {
    let mut app = test_app();

    // Spawn paddle
    let paddle = app
        .world_mut()
        .spawn((Paddle, Transform::from_scale(Vec3::ONE)))
        .id();

    // Spawn ball
    let ball = app.world_mut().spawn(Ball).id();

    // Spawn shrink brick (type 30)
    let brick = app
        .world_mut()
        .spawn((Brick, BrickTypeId(BRICK_TYPE_30)))
        .id();

    // Trigger collision between ball and brick
    trigger_collision(&mut app, ball, brick);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify paddle has PaddleSizeEffect component
    let effect = app
        .world()
        .entity(paddle)
        .get::<PaddleSizeEffect>()
        .expect("Paddle should have PaddleSizeEffect component");

    assert_eq!(
        effect.effect_type,
        SizeEffectType::Shrink,
        "Effect should be Shrink"
    );
    assert!(
        !effect.timer.is_finished(),
        "Timer should not be finished immediately"
    );

    // Verify paddle transform scale (70% = 14 units / 20 base = 0.7)
    let transform = app
        .world()
        .entity(paddle)
        .get::<Transform>()
        .expect("Paddle should have Transform");

    assert!(
        (transform.scale.y - 0.7).abs() < 0.01,
        "Paddle Y scale should be ~0.7 (70% of base), got {}",
        transform.scale.y
    );
}

#[test]
fn brick_32_enlarges_paddle() {
    let mut app = test_app();

    // Spawn paddle
    let paddle = app
        .world_mut()
        .spawn((Paddle, Transform::from_scale(Vec3::ONE)))
        .id();

    // Spawn ball
    let ball = app.world_mut().spawn(Ball).id();

    // Spawn enlarge brick (type 32)
    let brick = app
        .world_mut()
        .spawn((Brick, BrickTypeId(BRICK_TYPE_32)))
        .id();

    // Trigger collision between ball and brick
    trigger_collision(&mut app, ball, brick);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify paddle has PaddleSizeEffect component
    let effect = app
        .world()
        .entity(paddle)
        .get::<PaddleSizeEffect>()
        .expect("Paddle should have PaddleSizeEffect component");

    assert_eq!(
        effect.effect_type,
        SizeEffectType::Enlarge,
        "Effect should be Enlarge"
    );

    // Verify paddle transform scale (150% = 30 units / 20 base = 1.5)
    let transform = app
        .world()
        .entity(paddle)
        .get::<Transform>()
        .expect("Paddle should have Transform");

    assert!(
        (transform.scale.y - 1.5).abs() < 0.01,
        "Paddle Y scale should be ~1.5 (150% of base), got {}",
        transform.scale.y
    );
}

#[test]
fn effect_expires_after_duration() {
    let mut app = test_app();

    // Spawn paddle
    let paddle = app
        .world_mut()
        .spawn((Paddle, Transform::from_scale(Vec3::ONE)))
        .id();

    // Spawn ball and brick
    let ball = app.world_mut().spawn(Ball).id();
    let brick = app
        .world_mut()
        .spawn((Brick, BrickTypeId(BRICK_TYPE_30)))
        .id();

    // Trigger collision
    trigger_collision(&mut app, ball, brick);
    app.update();

    // Verify effect is active
    assert!(
        app.world()
            .entity(paddle)
            .get::<PaddleSizeEffect>()
            .is_some(),
        "Effect should be active"
    );

    // Manually tick the timer to completion (since Time delta isn't working in tests)
    app.world_mut()
        .entity_mut(paddle)
        .get_mut::<PaddleSizeEffect>()
        .unwrap()
        .timer
        .tick(Duration::from_secs_f32(10.1));
    app.update();

    // Verify effect is removed
    assert!(
        app.world()
            .entity(paddle)
            .get::<PaddleSizeEffect>()
            .is_none(),
        "Effect should be removed after timer expires"
    );

    // Verify paddle scale restored to 1.0
    let transform = app
        .world()
        .entity(paddle)
        .get::<Transform>()
        .expect("Paddle should have Transform");

    assert!(
        (transform.scale.y - 1.0).abs() < 0.01,
        "Paddle Y scale should be restored to 1.0, got {}",
        transform.scale.y
    );
}

#[test]
fn new_effect_replaces_old_effect() {
    let mut app = test_app();

    // Spawn paddle
    let paddle = app
        .world_mut()
        .spawn((Paddle, Transform::from_scale(Vec3::ONE)))
        .id();

    // Spawn ball
    let ball = app.world_mut().spawn(Ball).id();

    // Spawn shrink brick (type 30)
    let brick_shrink = app
        .world_mut()
        .spawn((Brick, BrickTypeId(BRICK_TYPE_30)))
        .id();

    // First collision: shrink
    trigger_collision(&mut app, ball, brick_shrink);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify shrink effect
    let effect = app
        .world()
        .entity(paddle)
        .get::<PaddleSizeEffect>()
        .expect("Should have shrink effect");
    assert_eq!(effect.effect_type, SizeEffectType::Shrink);

    // Spawn enlarge brick (type 32)
    let brick_enlarge = app
        .world_mut()
        .spawn((Brick, BrickTypeId(BRICK_TYPE_32)))
        .id();

    // Second collision: enlarge (should replace shrink)
    trigger_collision(&mut app, ball, brick_enlarge);
    advance_time(&mut app, 0.016);
    app.update();

    // Verify enlarge effect replaced shrink
    let effect = app
        .world()
        .entity(paddle)
        .get::<PaddleSizeEffect>()
        .expect("Should have enlarge effect");
    assert_eq!(
        effect.effect_type,
        SizeEffectType::Enlarge,
        "Enlarge should replace Shrink"
    );

    // Verify paddle scale is now 1.5 (enlarged)
    let transform = app
        .world()
        .entity(paddle)
        .get::<Transform>()
        .expect("Paddle should have Transform");
    assert!(
        (transform.scale.y - 1.5).abs() < 0.01,
        "Paddle should be enlarged to 1.5, got {}",
        transform.scale.y
    );
}

#[test]
fn timer_resets_on_same_brick_type() {
    let mut app = test_app();

    // Spawn paddle
    let paddle = app
        .world_mut()
        .spawn((Paddle, Transform::from_scale(Vec3::ONE)))
        .id();

    // Spawn ball
    let ball = app.world_mut().spawn(Ball).id();

    // First shrink brick
    let brick1 = app
        .world_mut()
        .spawn((Brick, BrickTypeId(BRICK_TYPE_30)))
        .id();

    // First collision
    trigger_collision(&mut app, ball, brick1);
    app.update();

    // Manually tick timer by 5 seconds
    app.world_mut()
        .entity_mut(paddle)
        .get_mut::<PaddleSizeEffect>()
        .unwrap()
        .timer
        .tick(Duration::from_secs_f32(5.0));
    app.update();

    // Verify effect still active with some time remaining
    let effect = app
        .world()
        .entity(paddle)
        .get::<PaddleSizeEffect>()
        .expect("Effect should still be active");
    assert!(!effect.timer.is_finished(), "Timer should still be running");
    let remaining_secs = effect.timer.remaining_secs();
    assert!(
        remaining_secs < 5.1 && remaining_secs > 4.9,
        "Should have ~5 seconds remaining, got {}",
        remaining_secs
    );

    // Second shrink brick
    let brick2 = app
        .world_mut()
        .spawn((Brick, BrickTypeId(BRICK_TYPE_32)))
        .id();

    // Second collision - should reset timer with new effect type
    trigger_collision(&mut app, ball, brick2);
    app.update();

    // Verify timer reset to full duration with enlarge effect
    let effect = app
        .world()
        .entity(paddle)
        .get::<PaddleSizeEffect>()
        .expect("Effect should still be active");
    assert_eq!(
        effect.effect_type,
        SizeEffectType::Enlarge,
        "Effect type should be Enlarge now"
    );
    let remaining_secs = effect.timer.remaining_secs();
    assert!(
        remaining_secs > 9.9 && remaining_secs <= 10.0,
        "Timer should reset to ~10 seconds, got {}",
        remaining_secs
    );
}
