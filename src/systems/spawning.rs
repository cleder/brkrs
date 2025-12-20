//! Systems for spawning game entities (camera, lights, ground).

use crate::{PLANE_H, PLANE_W};
use bevy::color::palettes::basic::SILVER;
use bevy::prelude::*;

/// Marker component for the main camera.
#[derive(Component)]
pub struct MainCamera;

/// Marker component for the ground plane entity.
/// Used by per-level texture override system to apply custom ground materials.
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct GroundPlane;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 37., 0.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        MainCamera,
    ));
}

pub fn spawn_ground_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(PLANE_H, PLANE_W)
                    .subdivisions(4),
            ),
        ),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
        GroundPlane,
    ));
}

pub fn spawn_light(mut commands: Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(-4.0, 20.0, 2.0),
    ));
}
