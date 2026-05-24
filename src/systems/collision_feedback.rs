//! Collision particle feedback systems.
//!
//! This module provides immediate, observer-driven collision feedback effects for
//! wall, paddle, and brick impacts.

use bevy::prelude::*;
use rand::RngExt;
use tracing::debug;

use crate::game_state::GameState;
use crate::signals::{CollisionFeedbackTargetKind, CollisionFeedbackTriggered};

const DEFAULT_MIN_PARTICLES: u8 = 8;
const DEFAULT_MAX_PARTICLES: u8 = 16;
const DEFAULT_MIN_LIFETIME: f32 = 0.20;
const DEFAULT_MAX_LIFETIME: f32 = 0.35;
const STYLE_FAMILY_SPARKLY: u8 = 1;

#[derive(Resource, Debug, Clone)]
pub struct CollisionFeedbackVisuals {
    pub particle_mesh: Handle<Mesh>,
    pub wall_material: Handle<StandardMaterial>,
    pub paddle_material: Handle<StandardMaterial>,
    pub brick_material: Handle<StandardMaterial>,
}

/// Tunable parameters for collision feedback spawning.
#[derive(Resource, Debug, Clone)]
pub struct FeedbackProfile {
    pub min_particles: u8,
    pub max_particles: u8,
    pub min_lifetime_seconds: f32,
    pub max_lifetime_seconds: f32,
}

impl Default for FeedbackProfile {
    fn default() -> Self {
        Self {
            min_particles: DEFAULT_MIN_PARTICLES,
            max_particles: DEFAULT_MAX_PARTICLES,
            min_lifetime_seconds: DEFAULT_MIN_LIFETIME,
            max_lifetime_seconds: DEFAULT_MAX_LIFETIME,
        }
    }
}

impl FeedbackProfile {
    fn is_valid(&self) -> bool {
        self.min_particles <= self.max_particles
            && self.min_lifetime_seconds <= self.max_lifetime_seconds
            && self.min_particles >= DEFAULT_MIN_PARTICLES
            && self.max_particles <= DEFAULT_MAX_PARTICLES
            && self.min_lifetime_seconds >= DEFAULT_MIN_LIFETIME
            && self.max_lifetime_seconds <= DEFAULT_MAX_LIFETIME
    }

    fn sample_particle_count(&self, rng: &mut rand::rngs::ThreadRng) -> u8 {
        rng.random_range(self.min_particles..=self.max_particles)
    }

    fn sample_lifetime_seconds(&self, rng: &mut rand::rngs::ThreadRng) -> f32 {
        rng.random_range(self.min_lifetime_seconds..=self.max_lifetime_seconds)
    }
}

/// One active effect instance spawned from a collision trigger.
#[derive(Component, Debug, Clone, Copy)]
pub struct FeedbackEffectInstance {
    pub elapsed_seconds: f32,
    pub lifetime_seconds: f32,
    pub particle_count: u8,
    pub origin_contact_point: Vec3,
    pub source_kind: CollisionFeedbackTargetKind,
    pub source_brick_destroyed: bool,
    pub style_family_id: u8,
    pub style_variant: f32,
}

/// Particle marker for entities spawned by one feedback instance.
#[derive(Component, Debug, Clone, Copy)]
pub struct CollisionFeedbackParticle {
    pub source_effect: Entity,
    pub velocity: Vec3,
}

/// Returns whether feedback spawning is allowed for the current game state.
///
/// If no state resource is present (e.g. isolated tests), feedback is allowed.
pub fn feedback_allowed_for_state_opt(game_state: &Option<Res<State<GameState>>>) -> bool {
    match game_state {
        Some(state) => *state.get() == GameState::Playing,
        None => true,
    }
}

/// Resolves contact point with finite fallback.
pub fn resolve_contact_point(contact_point: Vec3, fallback_contact_point: Vec3) -> Vec3 {
    if contact_point.is_finite() {
        contact_point
    } else if fallback_contact_point.is_finite() {
        fallback_contact_point
    } else {
        Vec3::ZERO
    }
}

/// Offsets a contact point slightly toward the ball to keep feedback particles in front of
/// opaque geometry such as walls and indestructible bricks.
pub fn offset_contact_toward_ball(
    contact_point: Vec3,
    source_center: Vec3,
    ball_position: Vec3,
) -> Vec3 {
    let direction = (ball_position - source_center).normalize_or_zero();
    if direction.is_finite() {
        contact_point + direction * 0.25
    } else {
        contact_point
    }
}

fn sample_style_variant(kind: CollisionFeedbackTargetKind, rng: &mut rand::rngs::ThreadRng) -> f32 {
    match kind {
        CollisionFeedbackTargetKind::Wall => rng.random_range(0.90..=1.10),
        CollisionFeedbackTargetKind::Paddle => rng.random_range(0.92..=1.08),
        CollisionFeedbackTargetKind::Brick => rng.random_range(0.88..=1.12),
    }
}

fn create_collision_feedback_visuals(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> CollisionFeedbackVisuals {
    let particle_mesh = meshes.add(Cuboid::new(0.12, 0.12, 0.12).mesh());

    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.95, 0.35),
        unlit: true,
        ..default()
    });
    let paddle_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.95, 1.0),
        unlit: true,
        ..default()
    });
    let brick_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.45, 0.25),
        unlit: true,
        ..default()
    });

    CollisionFeedbackVisuals {
        particle_mesh,
        wall_material,
        paddle_material,
        brick_material,
    }
}

fn material_for_kind(
    target_kind: CollisionFeedbackTargetKind,
    visuals: &CollisionFeedbackVisuals,
) -> Handle<StandardMaterial> {
    match target_kind {
        CollisionFeedbackTargetKind::Wall => visuals.wall_material.clone(),
        CollisionFeedbackTargetKind::Paddle => visuals.paddle_material.clone(),
        CollisionFeedbackTargetKind::Brick => visuals.brick_material.clone(),
    }
}

/// Observer: spawn one feedback effect for each qualifying collision trigger.
pub fn spawn_collision_feedback_effect(
    trigger: On<CollisionFeedbackTriggered>,
    mut commands: Commands,
    profile: Option<Res<FeedbackProfile>>,
    game_state: Option<Res<State<GameState>>>,
    visuals: Option<Res<CollisionFeedbackVisuals>>,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<StandardMaterial>>>,
) {
    if !feedback_allowed_for_state_opt(&game_state) {
        return;
    }

    let event = trigger.event();
    let profile = profile.map(|p| p.clone()).unwrap_or_default();
    if !profile.is_valid() {
        return;
    }

    let mut rng = rand::rng();
    let particle_count = profile.sample_particle_count(&mut rng);
    let lifetime_seconds = profile.sample_lifetime_seconds(&mut rng);
    let style_variant = sample_style_variant(event.target_kind, &mut rng);
    let contact_point = resolve_contact_point(
        event.contact_point,
        event.fallback_contact_point.unwrap_or(Vec3::ZERO),
    );

    let effect_entity = commands
        .spawn((
            FeedbackEffectInstance {
                elapsed_seconds: 0.0,
                lifetime_seconds,
                particle_count,
                origin_contact_point: contact_point,
                source_kind: event.target_kind,
                source_brick_destroyed: event.brick_destroyed_on_impact,
                style_family_id: STYLE_FAMILY_SPARKLY,
                style_variant,
            },
            Transform::from_translation(contact_point),
            GlobalTransform::default(),
        ))
        .id();

    debug!(
        source_kind = ?event.target_kind,
        contact_point = ?contact_point,
        particle_count,
        lifetime_seconds,
        "spawned collision feedback effect"
    );

    let visuals = if let Some(visuals) = visuals {
        Some(visuals.clone())
    } else if let (Some(meshes), Some(materials)) = (meshes.as_mut(), materials.as_mut()) {
        let created = create_collision_feedback_visuals(meshes.as_mut(), materials.as_mut());
        commands.insert_resource(created.clone());
        Some(created)
    } else {
        None
    };

    let particle_material = visuals
        .as_ref()
        .map(|v| material_for_kind(event.target_kind, v));

    for _ in 0..particle_count {
        let angle = rng.random_range(0.0..std::f32::consts::TAU);
        let speed = rng.random_range(0.8..1.8) * style_variant;
        let velocity = Vec3::new(angle.cos() * speed, 0.0, angle.sin() * speed);

        let mut particle_entity = commands.spawn((
            CollisionFeedbackParticle {
                source_effect: effect_entity,
                velocity,
            },
            Transform::from_translation(contact_point),
            GlobalTransform::default(),
            Visibility::Visible,
        ));

        if let (Some(visuals), Some(material)) = (visuals.as_ref(), particle_material.as_ref()) {
            particle_entity.insert((
                Mesh3d(visuals.particle_mesh.clone()),
                MeshMaterial3d(material.clone()),
            ));
        }
    }
}

/// Advance effect lifetimes and cleanup expired effect/particle entities.
pub fn update_feedback_effect_lifetimes(
    time: Res<Time>,
    mut commands: Commands,
    mut effects: Query<(Entity, &mut FeedbackEffectInstance)>,
    mut particles: ParamSet<(
        Query<(Entity, &CollisionFeedbackParticle)>,
        Query<(&CollisionFeedbackParticle, &mut Transform)>,
    )>,
) {
    let dt = time.delta_secs();

    for (particle, mut transform) in &mut particles.p1() {
        transform.translation += particle.velocity * dt;
    }

    let mut expired = Vec::new();
    for (entity, mut effect) in &mut effects {
        effect.elapsed_seconds += dt;
        if effect.elapsed_seconds >= effect.lifetime_seconds {
            expired.push(entity);
        }
    }

    if expired.is_empty() {
        return;
    }

    for effect in &expired {
        commands.entity(*effect).despawn();
    }

    for (particle_entity, particle) in particles.p0().iter() {
        if expired.contains(&particle.source_effect) {
            commands.entity(particle_entity).despawn();
        }
    }
}
