use bevy::{post_process::bloom::Bloom, prelude::*, render::view::Hdr};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (spawn_merkaba, projectile_lifetime))
        .run();
}

// --- Components ---

#[derive(Component)]
struct Merkaba;

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

fn projectile_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.timer.tick(time.delta());

        if lifetime.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

//////////////////////////////////

fn spawn_merkaba(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let tetra_mesh = meshes.add(Tetrahedron::default());

        // Material 1 (Blue) - Standard
        let mat_blue = materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.4, 0.9),
            metallic: 0.8,
            perceptual_roughness: 0.1,
            ..default()
        });

        // Material 2 (Gold) - MUST have cull_mode: None
        let mat_gold = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.8, 0.1),
            metallic: 0.8,
            perceptual_roughness: 0.1,
            // IMPORTANT: Negative scale flips normals, making the object "inside out".
            // Disabling culling ensures we can still see it.
            cull_mode: None,
            ..default()
        });

        let throw_speed = 15.0;
        let spin_speed = 3.0;

        commands
            .spawn((
                Transform::from_xyz(0.0, 2.0, 0.0),
                Visibility::default(),
                Merkaba,
                Lifetime {
                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                },
                RigidBody::Dynamic,
                Collider::ball(0.8),
                Velocity {
                    linvel: Vec3::NEG_Z * throw_speed,
                    angvel: Vec3::new(spin_speed, spin_speed, 0.0),
                },
                GravityScale(0.0),
                Ccd::enabled(),
                Restitution::coefficient(0.8),
            ))
            .with_children(|parent| {
                let scale = 1.5;

                // 1. UPRIGHT TETRAHEDRON (Blue)
                parent.spawn((
                    Mesh3d(tetra_mesh.clone()),
                    MeshMaterial3d(mat_blue),
                    Transform::from_scale(Vec3::splat(scale)),
                ));

                // 2. INVERTED TETRAHEDRON (Gold)
                // We use NEGATIVE scale. This performs a "Point Inversion",
                // creating the exact dual shape needed for the star.
                parent.spawn((
                    Mesh3d(tetra_mesh),
                    MeshMaterial3d(mat_gold),
                    Transform::from_scale(Vec3::splat(-scale)), // Negative Scale!
                ));
            });
    }
}
