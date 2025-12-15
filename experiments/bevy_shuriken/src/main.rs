use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (spawn_shuriken, shuriken_lifetime))
        .run();
}

// --- Components ---

#[derive(Component)]
struct Shuriken;

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
    // 1. Camera
    commands.spawn((
        Camera3d::default(),
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

    // 3. A Floor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        RigidBody::Fixed,
        Collider::cuboid(10.0, 0.01, 10.0),
    ));

    // 4. A Target Wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(4.0, 4.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.1, 0.1))),
        Transform::from_xyz(0.0, 2.0, -10.0),
        RigidBody::Fixed,
        Collider::cuboid(2.0, 2.0, 0.5),
    ));
}

fn spawn_shuriken(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let blade_mesh = meshes.add(Cuboid::new(0.7, 0.05, 0.15));

        let metal_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.35),
            metallic: 1.0,
            perceptual_roughness: 0.4,
            ..default()
        });

        let throw_speed = 25.0;
        let spin_speed = 40.0;

        commands
            .spawn((
                Transform::from_xyz(0.0, 2.0, 0.0),
                Visibility::default(),
                Shuriken,
                Lifetime {
                    timer: Timer::from_seconds(3.0, TimerMode::Once),
                },
                // Physics
                RigidBody::Dynamic,
                Collider::cylinder(0.025, 0.35),
                Velocity {
                    linvel: Vec3::NEG_Z * throw_speed,
                    angvel: Vec3::Y * spin_speed,
                },
                GravityScale(0.0),
                Ccd::enabled(),
                Restitution::coefficient(0.6),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Mesh3d(blade_mesh.clone()),
                    MeshMaterial3d(metal_material.clone()),
                    Transform::default(),
                ));

                parent.spawn((
                    Mesh3d(blade_mesh),
                    MeshMaterial3d(metal_material),
                    Transform::from_rotation(Quat::from_rotation_y(PI / 2.0)),
                ));
            });
    }
}

fn shuriken_lifetime(
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
