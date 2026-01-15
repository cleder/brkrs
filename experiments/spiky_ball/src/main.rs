use bevy::{
    math::primitives::{Cone, Cuboid, Plane3d, Sphere},
    post_process::bloom::Bloom,
    prelude::*,
    render::view::Hdr,
};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct SpikyBall;

#[derive(Component)]
struct Lifetime {
    timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (spawn_spiky_ball, projectile_lifetime))
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Hdr,
        Bloom::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Floor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        RigidBody::Fixed,
        Collider::cuboid(10.0, 0.01, 10.0),
    ));

    // Target Wall
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
        if lifetime.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_spiky_ball(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        // Sphere base
        let sphere_mesh = meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap());

        let mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.8),
            metallic: 0.9,
            perceptual_roughness: 0.3,
            ..default()
        });

        commands
            .spawn((
                Mesh3d(sphere_mesh),
                MeshMaterial3d(mat),
                Transform::from_xyz(0.0, 2.0, 0.0),
                SpikyBall,
                Lifetime {
                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                },
                RigidBody::Dynamic,
                Collider::ball(1.0),
                Velocity {
                    linvel: Vec3::NEG_Z * 15.0,
                    angvel: Vec3::new(3.0, 2.0, 1.0),
                },
                GravityScale(0.0),
                Ccd::enabled(),
                Restitution::coefficient(0.6),
            ))
            .with_children(|parent| {
                // Create 16 pronounced spikes distributed around the sphere
                let spike_mesh = meshes.add(
                    Cone {
                        radius: 0.3,
                        height: 1.2,
                    }
                    .mesh(),
                );
                let spike_mat = materials.add(Color::srgb(0.9, 0.9, 0.2));

                // Fibonacci sphere distribution for even spike placement
                let golden_ratio = (1.0 + 5.0_f32.sqrt()) / 2.0;
                for i in 0..16 {
                    let theta = 2.0 * std::f32::consts::PI * i as f32 / golden_ratio;
                    let phi = ((2 * i + 1) as f32 / 16.0 - 1.0).acos();

                    let x = phi.sin() * theta.cos();
                    let y = phi.sin() * theta.sin();
                    let z = phi.cos();

                    let direction = Vec3::new(x, y, z);
                    let position = direction * 0.9; // Place at sphere surface

                    // Rotate cone to point outward
                    let rotation = Quat::from_rotation_arc(Vec3::Y, direction);

                    parent.spawn((
                        Mesh3d(spike_mesh.clone()),
                        MeshMaterial3d(spike_mat.clone()),
                        Transform::from_translation(position).with_rotation(rotation),
                    ));
                }
            });
    }
}
