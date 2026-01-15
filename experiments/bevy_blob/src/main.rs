use bevy::{post_process::bloom::Bloom, prelude::*, render::view::Hdr};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (spawn_blob, blob_lifetime, animate_blob))
        .run();
}

// --- Components ---

#[derive(Component)]
struct BlobProjectile;

#[derive(Component)]
struct Lifetime {
    timer: Timer,
}

#[derive(Component)]
struct BlobPart {
    original_dir: Vec3,
    phase: f32,
    speed: f32,
}

// --- Systems ---

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 1. Camera
    commands.spawn((
        Camera3d::default(),
        Hdr,
        Bloom::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 2. Light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // 3. Floor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        RigidBody::Fixed,
        Collider::cuboid(10.0, 0.01, 10.0),
    ));

    // 4. Target Wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(4.0, 4.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.1, 0.1))),
        Transform::from_xyz(0.0, 2.0, -10.0),
        RigidBody::Fixed,
        Collider::cuboid(2.0, 2.0, 0.5),
    ));
}

fn spawn_blob(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        // --- ASSETS ---
        // A smaller sphere that we will cluster together
        let lump_mesh = meshes.add(Sphere::new(0.25).mesh().uv(16, 8));

        // Alien "Goo" Material (Green/Blue Plasma)
        let goo_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.9, 0.5),
            emissive: LinearRgba::rgb(2.0, 9.0, 5.0),
            perceptual_roughness: 0.1,
            metallic: 0.5,
            ..default()
        });

        let throw_speed = 15.0;
        let spin_speed = 5.0;
        let lump_count = 30;

        // --- PARENT ENTITY ---
        commands
            .spawn((
                Transform::from_xyz(0.0, 2.0, 0.0),
                Visibility::default(),
                BlobProjectile,
                Lifetime {
                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                },
                RigidBody::Dynamic,
                Collider::ball(0.5),
                Velocity {
                    linvel: Vec3::NEG_Z * throw_speed,
                    angvel: Vec3::new(spin_speed, spin_speed, 0.0),
                },
                GravityScale(0.0),
                Ccd::enabled(),
                Restitution::coefficient(0.2),
            ))
            .with_children(|parent| {
                // --- GENERATE LUMPS ---
                let phi = PI * (3.0 - 5.0_f32.sqrt());

                for i in 0..lump_count {
                    let y = 1.0 - (i as f32 / (lump_count - 1) as f32) * 2.0;
                    let radius = (1.0 - y * y).sqrt();
                    let theta = phi * i as f32;

                    let x = theta.cos() * radius;
                    let z = theta.sin() * radius;
                    let dir = Vec3::new(x, y, z);

                    parent.spawn((
                        Mesh3d(lump_mesh.clone()),
                        MeshMaterial3d(goo_material.clone()),
                        Transform::from_translation(dir * 0.3),
                        BlobPart {
                            original_dir: dir,
                            phase: i as f32 * 0.5,
                            speed: 5.0 + (i % 3) as f32 * 2.0,
                        },
                    ));
                }
            });
    }
}

// FIXED SYSTEM: Uses elapsed_secs()
fn animate_blob(time: Res<Time>, mut query: Query<(&mut Transform, &BlobPart)>) {
    for (mut transform, part) in &mut query {
        // FIXED: .elapsed_secs() instead of .elapsed_seconds()
        let wobble = (time.elapsed_secs() * part.speed + part.phase).sin();

        let current_radius = 0.3 + wobble * 0.15;

        transform.translation = part.original_dir * current_radius;
    }
}

fn blob_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.timer.tick(time.delta());

        if lifetime.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
