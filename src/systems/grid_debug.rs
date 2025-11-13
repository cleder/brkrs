//! Grid debug overlay system for visualizing the 22x22 game grid
//!
//! This module provides a wireframe grid overlay that is visible only when
//! wireframe mode is enabled, helping with alignment and debugging.

use bevy::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::WireframeConfig;

use crate::{GridOverlay, CELL_HEIGHT, CELL_WIDTH, GRID_HEIGHT, GRID_WIDTH, PLANE_H, PLANE_W};

/// Spawns the 22x22 grid wireframe overlay
/// The grid is initially hidden and only becomes visible when wireframe mode is enabled
pub fn spawn_grid_overlay(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a thin wireframe material for the grid lines
    let grid_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.5, 0.5, 0.5, 0.3),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Grid covers the entire playing field (PLANE_W × PLANE_H)
    // Calculate starting position (center the grid at origin)
    let start_x = -PLANE_H / 2.0; // X-axis (vertical on screen)
    let start_z = -PLANE_W / 2.0; // Z-axis (horizontal on screen)

    // Create vertical lines (along X axis) - these span the height
    for i in 0..=GRID_WIDTH {
        let z_pos = start_z + (i as f32 * CELL_WIDTH);

        let line_mesh = meshes.add(Cuboid::new(PLANE_H, 0.02, 0.02));

        commands.spawn((
            Mesh3d(line_mesh),
            MeshMaterial3d(grid_material.clone()),
            Transform::from_xyz(0.0, 2.0, z_pos),
            GridOverlay,
            Visibility::Hidden, // Initially hidden
        ));
    }

    // Create horizontal lines (along Z axis) - these span the width
    for i in 0..=GRID_HEIGHT {
        let x_pos = start_x + (i as f32 * CELL_HEIGHT);

        let line_mesh = meshes.add(Cuboid::new(0.02, 0.02, PLANE_W));

        commands.spawn((
            Mesh3d(line_mesh),
            MeshMaterial3d(grid_material.clone()),
            Transform::from_xyz(x_pos, 2.0, 0.0),
            GridOverlay,
            Visibility::Hidden, // Initially hidden
        ));
    }
}

/// Toggles grid overlay visibility based on wireframe mode
/// Grid is visible when wireframe is enabled, hidden otherwise
#[cfg(not(target_arch = "wasm32"))]
pub fn toggle_grid_visibility(
    wireframe_config: Res<WireframeConfig>,
    mut grid_query: Query<&mut Visibility, With<GridOverlay>>,
) {
    let target_visibility = if wireframe_config.global {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    for mut visibility in grid_query.iter_mut() {
        *visibility = target_visibility;
    }
}

/// No-op version for WASM (wireframe not supported)
#[cfg(target_arch = "wasm32")]
pub fn toggle_grid_visibility() {
    // WASM doesn't support wireframe mode, so grid stays hidden
}
