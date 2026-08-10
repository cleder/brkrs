use bevy::ecs::message::MessageReader;
use bevy::math::primitives::Torus;
use bevy::prelude::*;
use bevy_hanabi::prelude::{
    Attribute, ColorOverLifetimeModifier, EffectAsset, ExprWriter, Gradient, HanabiPlugin,
    OrientMode, OrientModifier, ParticleEffect, SetAttributeModifier, SetPositionSphereModifier,
    SetVelocitySphereModifier, ShapeDimension, SizeOverLifetimeModifier, SpawnerSettings,
};

use crate::signals::SeaMineExplosionTriggered;
use crate::systems::sea_mine::SeaMineSystems;

#[derive(Resource, Default)]
pub struct SeaMineParticleAssets {
    pub explosion_effect: Option<Handle<EffectAsset>>,
}

#[derive(Component)]
pub struct SeaMineExplosionBurst;

#[derive(Component)]
struct SeaMineExplosionLifetime {
    timer: Timer,
}

#[derive(Component)]
struct SeaMineExplosionEffectLifetime {
    timer: Timer,
}

#[derive(Component)]
struct SeaMineExplosionShockwave {
    timer: Timer,
    start_major_radius: f32,
    end_major_radius: f32,
    start_minor_radius: f32,
    end_minor_radius: f32,
}

const EXPLOSION_BURST_LIFETIME_SECONDS: f32 = 0.22;
const EXPLOSION_SHOCKWAVE_LIFETIME_SECONDS: f32 = 0.34;
const EXPLOSION_PARTICLE_CAPACITY: u32 = 8192;
const EXPLOSION_PARTICLE_COUNT: f32 = 96.0;
const EXPLOSION_PARTICLE_LIFETIME: f32 = 0.42;
const EXPLOSION_PARTICLE_RADIUS: f32 = 0.28;
const EXPLOSION_PARTICLE_SPEED_MIN: f32 = 14.0;
const EXPLOSION_PARTICLE_SPEED_RANGE: f32 = 18.0;
const SHOCKWAVE_BASE_ALPHA: f32 = 0.38;

/// Damage radius aligned with the maximum visual reach of the burst particles,
/// reduced to half the previous effective size.
pub const SEA_MINE_EXPLOSION_DAMAGE_RADIUS: f32 = 0.5
    * (EXPLOSION_PARTICLE_RADIUS
        + (EXPLOSION_PARTICLE_SPEED_MIN + EXPLOSION_PARTICLE_SPEED_RANGE)
            * EXPLOSION_PARTICLE_LIFETIME);

pub struct ParticleFxPlugin;

impl Plugin for ParticleFxPlugin {
    fn build(&self, app: &mut App) {
        // Headless/minimal test apps may not include render shader assets.
        // In that case we still run burst-marker logic without booting Hanabi.
        if app
            .world()
            .contains_resource::<Assets<bevy::shader::Shader>>()
        {
            app.add_plugins(HanabiPlugin);
        }

        app.init_resource::<SeaMineParticleAssets>();
        app.add_systems(
            Update,
            (
                trigger_sea_mine_explosion_burst.after(SeaMineSystems::ResolveDetonation),
                animate_sea_mine_explosion_burst,
            ),
        );
    }
}

fn trigger_sea_mine_explosion_burst(
    mut triggers: MessageReader<SeaMineExplosionTriggered>,
    mut commands: Commands,
    mut particle_assets: ResMut<SeaMineParticleAssets>,
    mut effect_assets: Option<ResMut<Assets<EffectAsset>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if particle_assets.explosion_effect.is_none() {
        if let Some(effect_assets) = effect_assets.as_mut() {
            particle_assets.explosion_effect = Some(build_sea_mine_explosion_effect(effect_assets));
        }
    }

    for event in triggers.read() {
        if let Some(effect) = particle_assets.explosion_effect.clone() {
            commands.spawn((
                Name::new("SeaMineExplosionEffect"),
                ParticleEffect::new(effect),
                SeaMineExplosionEffectLifetime {
                    timer: Timer::from_seconds(EXPLOSION_PARTICLE_LIFETIME, TimerMode::Once),
                },
                Transform::from_translation(event.position),
                GlobalTransform::default(),
            ));
        }

        let burst_material = materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.45, 0.08, 0.9),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });

        let start_major_radius = event.radius * 0.02;
        let end_major_radius = event.radius;
        let start_minor_radius = event.radius * 0.008;
        let end_minor_radius = event.radius * 0.003;

        let shockwave_mesh = meshes.add(
            Torus {
                major_radius: start_major_radius,
                minor_radius: start_minor_radius,
            }
            .mesh(),
        );
        let shockwave_material = materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.72, 0.28, SHOCKWAVE_BASE_ALPHA),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            ..default()
        });

        commands.spawn((
            Name::new("SeaMineExplosionBurst"),
            SeaMineExplosionBurst,
            SeaMineExplosionLifetime {
                timer: Timer::from_seconds(EXPLOSION_BURST_LIFETIME_SECONDS, TimerMode::Once),
            },
            Mesh3d(meshes.add(Sphere::new(0.4).mesh())),
            MeshMaterial3d(burst_material),
            Transform::from_translation(event.position).with_scale(Vec3::splat(0.5)),
            GlobalTransform::default(),
        ));

        commands.spawn((
            Name::new("SeaMineExplosionShockwave"),
            SeaMineExplosionShockwave {
                timer: Timer::from_seconds(EXPLOSION_SHOCKWAVE_LIFETIME_SECONDS, TimerMode::Once),
                start_major_radius,
                end_major_radius,
                start_minor_radius,
                end_minor_radius,
            },
            Mesh3d(shockwave_mesh),
            MeshMaterial3d(shockwave_material),
            Transform::from_translation(event.position + Vec3::Y * 0.03),
            GlobalTransform::default(),
        ));
    }
}

fn animate_sea_mine_explosion_burst(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bursts: Query<
        (Entity, &mut SeaMineExplosionLifetime, &mut Transform),
        With<SeaMineExplosionBurst>,
    >,
    mut effect_particles: Query<
        (Entity, &mut SeaMineExplosionEffectLifetime),
        With<ParticleEffect>,
    >,
    mut shockwaves: Query<(
        Entity,
        &mut SeaMineExplosionShockwave,
        &Mesh3d,
        &MeshMaterial3d<StandardMaterial>,
    )>,
) {
    for (entity, mut lifetime, mut transform) in &mut bursts {
        lifetime.timer.tick(time.delta());

        let progress = lifetime.timer.fraction();
        let scale = 0.5 + (progress * 2.5);
        transform.scale = Vec3::splat(scale);

        if lifetime.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }

    for (entity, mut effect_lifetime) in &mut effect_particles {
        effect_lifetime.timer.tick(time.delta());
        if effect_lifetime.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }

    for (entity, mut shockwave, mesh3d, material3d) in &mut shockwaves {
        shockwave.timer.tick(time.delta());

        let progress = shockwave.timer.fraction();
        let major_radius = shockwave
            .start_major_radius
            .lerp(shockwave.end_major_radius, progress);
        let minor_radius = shockwave
            .start_minor_radius
            .lerp(shockwave.end_minor_radius, progress);

        if let Some(mesh) = meshes.get_mut(&mesh3d.0) {
            *mesh = Torus {
                major_radius,
                minor_radius,
            }
            .mesh()
            .build();
        }

        if let Some(material) = materials.get_mut(&material3d.0) {
            let alpha = SHOCKWAVE_BASE_ALPHA * (1.0 - progress).powf(1.35);
            material.base_color = Color::srgba(1.0, 0.72, 0.28, alpha);
        }

        if shockwave.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn build_sea_mine_explosion_effect(effects: &mut Assets<EffectAsset>) -> Handle<EffectAsset> {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(1.0, 0.95, 0.75, 1.0));
    color_gradient.add_key(0.16, Vec4::new(1.0, 0.42, 0.1, 0.88));
    color_gradient.add_key(0.55, Vec4::new(0.82, 0.2, 0.06, 0.28));
    color_gradient.add_key(1.0, Vec4::new(0.12, 0.03, 0.01, 0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec3::splat(0.16));
    size_gradient.add_key(0.32, Vec3::splat(0.1));
    size_gradient.add_key(1.0, Vec3::splat(0.012));

    let writer = ExprWriter::new();

    let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.0).expr());
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        writer.lit(EXPLOSION_PARTICLE_LIFETIME).expr(),
    );
    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(EXPLOSION_PARTICLE_RADIUS).expr(),
        dimension: ShapeDimension::Volume,
    };
    let velocity_speed = (writer.rand(Attribute::LIFETIME.value_type())
        * writer.lit(EXPLOSION_PARTICLE_SPEED_RANGE)
        + writer.lit(EXPLOSION_PARTICLE_SPEED_MIN))
    .expr();
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: velocity_speed,
    };

    effects.add(
        EffectAsset::new(
            EXPLOSION_PARTICLE_CAPACITY,
            SpawnerSettings::once(EXPLOSION_PARTICLE_COUNT.into()),
            writer.finish(),
        )
        .with_name("SeaMineExplosion")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .render(ColorOverLifetimeModifier::new(color_gradient))
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        })
        .render(OrientModifier::new(OrientMode::FaceCameraPosition)),
    )
}
