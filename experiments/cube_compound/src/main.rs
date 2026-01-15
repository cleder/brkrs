use bevy::{post_process::bloom::Bloom, prelude::*, render::view::Hdr};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (spawn_cube_compound, projectile_lifetime))
        .run();
}

// --- Components ---

#[derive(Component)]
struct Projectile;

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
    // 1. Camera setup
    commands.spawn((
        Camera3d::default(),
        // HDR must be enabled for Bloom to work
        Hdr,
        // In latest dev versions, 'Bloom' is the component name
        Bloom::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 2. Light
    commands.spawn((
        PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            range: 50.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // 3. Floor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.2, 0.25))),
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

fn spawn_cube_compound(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        // --- GEOMETRY ---
        // A perfect cube, 1.0 units on all sides
        let cube_mesh = meshes.add(Cuboid::from_size(Vec3::splat(1.0)));

        // --- MATERIALS ---

        // Material 1: Glowing Purple Neon
        let mat_neon = materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.0, 1.0),
            // High emissive value creates the "Bloom" glow
            emissive: LinearRgba::rgb(4.0, 0.0, 10.0),
            ..default()
        });

        // Material 2: Reflective Chrome
        let mat_chrome = materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.9, 0.95),
            metallic: 1.0,
            perceptual_roughness: 0.1,
            ..default()
        });

        // --- PHYSICS ---
        let throw_speed = 15.0;
        let spin_speed = 4.0;

        commands
            .spawn((
                Transform::from_xyz(0.0, 2.0, 0.0),
                Visibility::default(),
                Projectile,
                Lifetime {
                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                },
                RigidBody::Dynamic,
                Collider::ball(1.2),
                Velocity {
                    linvel: Vec3::NEG_Z * throw_speed,
                    angvel: Vec3::new(spin_speed, spin_speed, spin_speed),
                },
                GravityScale(0.0),
                Ccd::enabled(),
                Restitution::coefficient(0.8),
            ))
            .with_children(|parent| {
                let scale = 1.0;

                // 1. STANDARD CUBE (Chrome)
                parent.spawn((
                    Mesh3d(cube_mesh.clone()),
                    MeshMaterial3d(mat_chrome),
                    Transform::from_scale(Vec3::splat(scale)),
                ));

                // 2. ROTATED CUBE (Neon Purple)
                // Rotated 45 degrees on X and Y to create the compound shape
                parent.spawn((
                    Mesh3d(cube_mesh),
                    MeshMaterial3d(mat_neon),
                    Transform::from_scale(Vec3::splat(scale)).with_rotation(
                        Quat::from_rotation_x(PI / 4.0) * Quat::from_rotation_y(PI / 4.0),
                    ),
                ));
            });
    }
}

fn projectile_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.timer.tick(time.delta());

        // FIXED: Use is_finished() to silence warning
        if lifetime.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
