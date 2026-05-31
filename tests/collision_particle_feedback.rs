use std::time::Duration;

use bevy::prelude::*;

use brkrs::game_state::GameState;
use brkrs::signals::{CollisionFeedbackTargetKind, CollisionFeedbackTriggered};
use brkrs::systems::collision_feedback::{
    spawn_collision_feedback_effect, update_feedback_effect_lifetimes, CollisionFeedbackParticle,
    FeedbackEffectInstance, FeedbackProfile,
};

fn make_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(bevy::state::app::StatesPlugin)
        .init_state::<GameState>()
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<FeedbackProfile>()
        .init_resource::<brkrs::systems::collision_feedback::CollisionFeedbackVisuals>()
        .add_message::<CollisionFeedbackTriggered>()
        .add_observer(spawn_collision_feedback_effect)
        .add_systems(Update, update_feedback_effect_lifetimes);

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);
    app.update();

    app
}

fn advance_time(app: &mut App, delta_secs: f32) {
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(Duration::from_secs_f32(delta_secs));
}

fn trigger_collision(app: &mut App, kind: CollisionFeedbackTargetKind, contact_point: Vec3) {
    let ball = app.world_mut().spawn((Transform::default(),)).id();
    let target = app.world_mut().spawn((Transform::default(),)).id();

    app.world_mut()
        .resource_mut::<Messages<CollisionFeedbackTriggered>>()
        .write(CollisionFeedbackTriggered {
            ball_entity: ball,
            target_entity: target,
            target_kind: kind,
            contact_point,
            fallback_contact_point: Some(contact_point),
            brick_destroyed_on_impact: kind == CollisionFeedbackTargetKind::Brick,
        });
}

fn effect_instances(world: &mut World) -> Vec<FeedbackEffectInstance> {
    let mut q = world.query::<&FeedbackEffectInstance>();
    q.iter(world).copied().collect()
}

#[test]
fn wall_collision_spawns_effect_same_frame() {
    let mut app = make_test_app();
    trigger_collision(
        &mut app,
        CollisionFeedbackTargetKind::Wall,
        Vec3::new(2.0, 0.0, 1.0),
    );
    app.update();

    let effects = effect_instances(app.world_mut());
    assert_eq!(effects.len(), 1, "Expected one wall feedback effect");
}

#[test]
fn paddle_collision_spawns_effect_same_frame() {
    let mut app = make_test_app();
    trigger_collision(
        &mut app,
        CollisionFeedbackTargetKind::Paddle,
        Vec3::new(0.0, 0.0, -1.0),
    );
    app.update();

    let effects = effect_instances(app.world_mut());
    assert_eq!(effects.len(), 1, "Expected one paddle feedback effect");
}

#[test]
fn brick_collision_spawns_effect_same_frame() {
    let mut app = make_test_app();
    trigger_collision(
        &mut app,
        CollisionFeedbackTargetKind::Brick,
        Vec3::new(-1.0, 0.0, 3.0),
    );
    app.update();

    let effects = effect_instances(app.world_mut());
    assert_eq!(effects.len(), 1, "Expected one brick feedback effect");
}

#[test]
fn effect_spawns_at_exact_contact_point() {
    let mut app = make_test_app();
    let point = Vec3::new(3.25, 0.0, -2.5);

    trigger_collision(&mut app, CollisionFeedbackTargetKind::Brick, point);
    app.update();

    let mut q = app
        .world_mut()
        .query::<(&FeedbackEffectInstance, &Transform)>();
    let (effect, transform) = q
        .iter(app.world_mut())
        .next()
        .expect("Expected one feedback effect");

    assert_eq!(effect.origin_contact_point, point);
    assert_eq!(transform.translation, point);
}

#[test]
fn resolve_contact_point_falls_back_to_valid_point() {
    use brkrs::systems::collision_feedback::resolve_contact_point;

    let invalid = Vec3::new(f32::NAN, 0.0, 0.0);
    let fallback = Vec3::new(1.0, 2.0, 3.0);

    let resolved = resolve_contact_point(invalid, fallback);
    assert_eq!(resolved, fallback);
}

#[test]
fn offset_contact_toward_ball_moves_point_forward() {
    use brkrs::systems::collision_feedback::offset_contact_toward_ball;

    let contact_point = Vec3::new(0.0, 0.0, 0.0);
    let source_center = Vec3::new(-1.0, 0.0, 0.0);
    let ball_position = Vec3::new(1.0, 0.0, 0.0);

    let offset = offset_contact_toward_ball(contact_point, source_center, ball_position);
    assert!(offset.x > contact_point.x);
}

#[test]
fn feedback_allowed_without_state_resource() {
    use brkrs::systems::collision_feedback::feedback_allowed_for_state_opt;
    assert!(feedback_allowed_for_state_opt(&None));
}

#[test]
fn particles_move_over_time() {
    let mut app = make_test_app();
    trigger_collision(
        &mut app,
        CollisionFeedbackTargetKind::Wall,
        Vec3::new(0.0, 0.0, 0.0),
    );
    app.update();

    let initial_positions: Vec<Vec3> = app
        .world_mut()
        .query::<(&CollisionFeedbackParticle, &Transform)>()
        .iter(app.world_mut())
        .map(|(_particle, transform)| transform.translation)
        .collect();

    advance_time(&mut app, 0.1);
    app.update();

    let later_positions: Vec<Vec3> = app
        .world_mut()
        .query::<(&CollisionFeedbackParticle, &Transform)>()
        .iter(app.world_mut())
        .map(|(_particle, transform)| transform.translation)
        .collect();

    assert_eq!(initial_positions.len(), later_positions.len());
    assert!(initial_positions
        .iter()
        .zip(later_positions.iter())
        .any(|(before, after)| *before != *after));
}

#[test]
fn brick_destroyed_on_impact_sets_flag() {
    let mut app = make_test_app();
    trigger_collision(
        &mut app,
        CollisionFeedbackTargetKind::Brick,
        Vec3::new(1.0, 0.0, 1.0),
    );
    app.update();

    let effects = effect_instances(app.world_mut());
    assert_eq!(effects.len(), 1);
    assert!(effects[0].source_brick_destroyed);
}

#[test]
fn lifetime_stays_within_required_window() {
    let mut app = make_test_app();
    trigger_collision(&mut app, CollisionFeedbackTargetKind::Wall, Vec3::ZERO);
    app.update();

    let effects = effect_instances(app.world_mut());
    assert_eq!(effects.len(), 1);

    let lifetime = effects[0].lifetime_seconds;
    assert!((0.20..=0.35).contains(&lifetime));
}

#[test]
fn particle_count_stays_within_required_window() {
    let mut app = make_test_app();
    trigger_collision(&mut app, CollisionFeedbackTargetKind::Paddle, Vec3::ZERO);
    app.update();

    let effects = effect_instances(app.world_mut());
    assert_eq!(effects.len(), 1);

    let count = effects[0].particle_count;
    assert!((8..=16).contains(&count));

    let mut q = app.world_mut().query::<&CollisionFeedbackParticle>();
    let particle_entities = q.iter(app.world_mut()).count();
    assert_eq!(particle_entities as u8, count);
}

#[test]
fn pause_suppresses_spawns_and_resume_does_not_replay() {
    let mut app = make_test_app();

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Paused);
    app.update();

    trigger_collision(&mut app, CollisionFeedbackTargetKind::Wall, Vec3::ZERO);
    app.update();

    assert!(effect_instances(app.world_mut()).is_empty());

    app.world_mut()
        .resource_mut::<NextState<GameState>>()
        .set(GameState::Playing);
    app.update();

    assert!(
        effect_instances(app.world_mut()).is_empty(),
        "Suppressed effects must not replay after resume"
    );
}

#[test]
fn repeated_collisions_cleanup_all_effects_and_particles() {
    let mut app = make_test_app();

    for i in 0..12 {
        trigger_collision(
            &mut app,
            CollisionFeedbackTargetKind::Brick,
            Vec3::new(i as f32, 0.0, 0.0),
        );
    }
    app.update();

    assert_eq!(effect_instances(app.world_mut()).len(), 12);

    advance_time(&mut app, 1.0);
    app.update();

    assert!(effect_instances(app.world_mut()).is_empty());
    let mut q = app.world_mut().query::<&CollisionFeedbackParticle>();
    assert_eq!(q.iter(app.world()).count(), 0);
}

#[test]
fn cross_surface_uses_same_style_family() {
    let mut app = make_test_app();

    trigger_collision(
        &mut app,
        CollisionFeedbackTargetKind::Wall,
        Vec3::new(0.0, 0.0, 0.0),
    );
    trigger_collision(
        &mut app,
        CollisionFeedbackTargetKind::Paddle,
        Vec3::new(1.0, 0.0, 0.0),
    );
    trigger_collision(
        &mut app,
        CollisionFeedbackTargetKind::Brick,
        Vec3::new(2.0, 0.0, 0.0),
    );
    app.update();

    let effects = effect_instances(app.world_mut());
    assert_eq!(effects.len(), 3);
    assert!(effects.iter().all(|e| e.style_family_id == 1));
}

#[test]
fn style_variation_is_bounded_for_all_target_kinds() {
    let mut app = make_test_app();

    for _ in 0..6 {
        trigger_collision(
            &mut app,
            CollisionFeedbackTargetKind::Wall,
            Vec3::new(0.0, 0.0, 0.0),
        );
        trigger_collision(
            &mut app,
            CollisionFeedbackTargetKind::Paddle,
            Vec3::new(1.0, 0.0, 0.0),
        );
        trigger_collision(
            &mut app,
            CollisionFeedbackTargetKind::Brick,
            Vec3::new(2.0, 0.0, 0.0),
        );
    }
    app.update();

    let effects = effect_instances(app.world_mut());
    assert!(!effects.is_empty());

    for effect in effects {
        assert!((0.85..=1.15).contains(&effect.style_variant));
    }
}

#[test]
fn burst_collisions_spawn_one_effect_per_collision_no_cap() {
    let mut app = make_test_app();

    for i in 0..25 {
        trigger_collision(
            &mut app,
            CollisionFeedbackTargetKind::Brick,
            Vec3::new(i as f32, 0.0, 0.0),
        );
    }
    app.update();

    assert_eq!(effect_instances(app.world_mut()).len(), 25);
}

#[test]
fn supported_target_kinds_are_all_spawned() {
    let mut app = make_test_app();

    trigger_collision(
        &mut app,
        CollisionFeedbackTargetKind::Wall,
        Vec3::new(0.0, 0.0, 0.0),
    );
    trigger_collision(
        &mut app,
        CollisionFeedbackTargetKind::Paddle,
        Vec3::new(1.0, 0.0, 0.0),
    );
    trigger_collision(
        &mut app,
        CollisionFeedbackTargetKind::Brick,
        Vec3::new(2.0, 0.0, 0.0),
    );
    app.update();

    let effects = effect_instances(app.world_mut());
    assert!(effects
        .iter()
        .any(|effect| effect.source_kind == CollisionFeedbackTargetKind::Wall));
    assert!(effects
        .iter()
        .any(|effect| effect.source_kind == CollisionFeedbackTargetKind::Paddle));
    assert!(effects
        .iter()
        .any(|effect| effect.source_kind == CollisionFeedbackTargetKind::Brick));
}

#[test]
fn feedback_particles_do_not_create_hierarchy_links() {
    let mut app = make_test_app();
    trigger_collision(&mut app, CollisionFeedbackTargetKind::Brick, Vec3::ZERO);
    app.update();

    let mut particle_q = app
        .world_mut()
        .query_filtered::<(Option<&ChildOf>, Option<&Children>), With<CollisionFeedbackParticle>>();
    for (child_of, children) in particle_q.iter(app.world()) {
        assert!(child_of.is_none());
        assert!(children.is_none());
    }

    let mut effect_q = app
        .world_mut()
        .query_filtered::<(Option<&ChildOf>, Option<&Children>), With<FeedbackEffectInstance>>();
    for (child_of, children) in effect_q.iter(app.world()) {
        assert!(child_of.is_none());
        assert!(children.is_none());
    }
}
