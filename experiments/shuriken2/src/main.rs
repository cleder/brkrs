use bevy::{post_process::bloom::Bloom, prelude::*, render::view::Hdr};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

// --- Components ---
#[derive(Component)]
struct Shuriken;

#[derive(Component)]
struct Lifetime {
    timer: Timer,
}

#[derive(Component)]
struct Rotates {
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (spawn_shuriken, projectile_lifetime, rotate_system))
        .run();
}

// --- Scene Setup ---
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera with Bloom
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

// --- Systems ---

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

fn rotate_system(time: Res<Time>, mut query: Query<(&mut Transform, &Rotates)>) {
    for (mut transform, rotates) in &mut query {
        transform.rotate_y(rotates.speed * time.delta_secs());
    }
}

fn spawn_shuriken(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        // Procedural shuriken mesh
        let shuriken_mesh = meshes.add(generate_shuriken(4, 0.5, 1.0, 0.1));

        let mat_metal = materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.6, 0.7),
            metallic: 0.9,
            perceptual_roughness: 0.2,
            ..default()
        });

        let throw_speed = 20.0;
        let spin_speed = 6.0;

        commands.spawn((
            Mesh3d(shuriken_mesh),
            MeshMaterial3d(mat_metal),
            Transform::from_xyz(0.0, 2.0, 0.0),
            Shuriken,
            Lifetime {
                timer: Timer::from_seconds(5.0, TimerMode::Once),
            },
            RigidBody::Dynamic,
            Collider::ball(0.8),
            Velocity {
                linvel: Vec3::NEG_Z * throw_speed,
                angvel: Vec3::new(0.0, spin_speed, 0.0),
            },
            GravityScale(0.0),
            Ccd::enabled(),
            Restitution::coefficient(0.8),
            Rotates { speed: spin_speed },
        ));
    }
}

// --- Shuriken Mesh Generator ---
fn generate_shuriken(blades: usize, inner_r: f32, outer_r: f32, thickness: f32) -> Mesh {
    use bevy::asset::RenderAssetUsages;
    use bevy::mesh::Indices;
    use bevy::render::render_resource::PrimitiveTopology;

    let mut positions = Vec::new();
    let mut indices = Vec::new();

    // Generate 2D star profile
    for i in 0..(blades * 2) {
        let angle = i as f32 * PI / blades as f32;
        let r = if i % 2 == 0 { inner_r } else { outer_r };
        let x = r * angle.cos();
        let y = r * angle.sin();
        positions.push([x, y, 0.0]); // front face
        positions.push([x, y, thickness]); // back face
    }

    // Center points
    let front_center = positions.len();
    positions.push([0.0, 0.0, 0.0]);
    let back_center = positions.len();
    positions.push([0.0, 0.0, thickness]);

    // Front face triangles
    for i in 0..(blades * 2) {
        let next = (i + 1) % (blades * 2);
        indices.push([front_center as u32, i as u32, next as u32]);
    }

    // Back face triangles
    for i in 0..(blades * 2) {
        let next = (i + 1) % (blades * 2);
        indices.push([
            back_center as u32,
            (next + blades * 2) as u32,
            (i + blades * 2) as u32,
        ]);
    }

    // Side quads
    for i in 0..(blades * 2) {
        let next = (i + 1) % (blades * 2);
        let a = i;
        let b = next;
        let c = b + blades * 2;
        let d = a + blades * 2;
        indices.push([a as u32, b as u32, c as u32]);
        indices.push([a as u32, c as u32, d as u32]);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices.concat()));
    mesh
}
