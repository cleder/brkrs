use bevy::{post_process::bloom::Bloom, prelude::*, render::view::Hdr};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (spawn_ophanim, ophanim_lifetime))
        .run();
}

// --- Components ---

#[derive(Component)]
struct Ophanim;

#[derive(Component)]
struct Lifetime {
    timer: Timer,
}

// --- Systems ---

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 1. Camera with Bloom
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

fn spawn_ophanim(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        // --- ASSETS ---
        // 1. Core Sphere
        let core_mesh = meshes.add(Sphere::new(0.4).mesh().uv(32, 16));

        // 2. Spike Cone (Thinner now, to fit more of them)
        // Radius: 0.04 (Thin), Height: 0.7 (Long)
        let spike_mesh = meshes.add(Cone::new(0.04, 0.7));

        // 3. Glowing Material
        let glow_material = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.3, 0.1),
            emissive: LinearRgba::rgb(8.0, 2.0, 0.2),
            ..default()
        });

        // --- CONFIGURATION ---
        let throw_speed = 20.0;
        let spin_speed = 10.0;
        let spike_count = 42; // You can change this to 100 for a porcupine look!

        commands
            .spawn((
                Transform::from_xyz(0.0, 2.0, 0.0),
                Visibility::default(),
                Ophanim,
                Lifetime {
                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                },
                RigidBody::Dynamic,
                Collider::ball(0.6),
                Velocity {
                    linvel: Vec3::NEG_Z * throw_speed,
                    angvel: Vec3::new(spin_speed, spin_speed, spin_speed),
                },
                GravityScale(0.0),
                Ccd::enabled(),
                Restitution::coefficient(0.8),
            ))
            .with_children(|parent| {
                // Spawn Core
                parent.spawn((
                    Mesh3d(core_mesh),
                    MeshMaterial3d(glow_material.clone()),
                    Transform::default(),
                ));

                // --- FIBONACCI SPHERE ALGORITHM ---
                // This distributes points evenly on a sphere surface
                let phi = PI * (3.0 - 5.0_f32.sqrt()); // Golden Angle (~2.399 radians)

                for i in 0..spike_count {
                    let y = 1.0 - (i as f32 / (spike_count - 1) as f32) * 2.0; // y goes from 1 to -1
                    let radius = (1.0 - y * y).sqrt(); // radius at y
                    let theta = phi * i as f32; // Golden angle increment

                    let x = theta.cos() * radius;
                    let z = theta.sin() * radius;

                    // The direction vector for this spike
                    let dir = Vec3::new(x, y, z);

                    // Rotate the cone to point in that direction
                    let rotation = Quat::from_rotation_arc(Vec3::Y, dir);
                    let offset = dir * 0.35;

                    parent.spawn((
                        Mesh3d(spike_mesh.clone()),
                        MeshMaterial3d(glow_material.clone()),
                        Transform::from_translation(offset).with_rotation(rotation),
                    ));
                }
            });
    }
}

fn ophanim_lifetime(
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
