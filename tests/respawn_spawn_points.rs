use bevy::prelude::*;
use brkrs::level_loader::{set_spawn_points_only, LevelDefinition};
use brkrs::systems::respawn::SpawnPoints;

const GRID_DIM: usize = 20;
const PLANE_H: f32 = 30.0;
const PLANE_W: f32 = 40.0;
const CELL_HEIGHT: f32 = PLANE_H / GRID_DIM as f32;
const CELL_WIDTH: f32 = PLANE_W / GRID_DIM as f32;

fn level_with_markers(
    paddle_marker: Option<(usize, usize)>,
    ball_marker: Option<(usize, usize)>,
) -> LevelDefinition {
    let mut matrix = vec![vec![0_u8; GRID_DIM]; GRID_DIM];
    if let Some((row, col)) = paddle_marker {
        matrix[row][col] = 1;
        // Duplicate markers should be ignored once the first is recorded.
        if row + 1 < GRID_DIM {
            matrix[row + 1][col] = 1;
        }
    }
    if let Some((row, col)) = ball_marker {
        matrix[row][col] = 2;
        if row + 1 < GRID_DIM {
            matrix[row + 1][col] = 2;
        }
    }
    LevelDefinition {
        number: 1,
        gravity: None,
        matrix,
        #[cfg(feature = "texture_manifest")]
        presentation: None,
    }
}

fn expected_position(row: usize, col: usize) -> Vec3 {
    let x = -PLANE_H / 2.0 + (row as f32 + 0.5) * CELL_HEIGHT;
    let z = -PLANE_W / 2.0 + (col as f32 + 0.5) * CELL_WIDTH;
    Vec3::new(x, 2.0, z)
}

#[test]
fn extracts_first_marker_for_each_entity() {
    let def = level_with_markers(Some((3, 4)), Some((10, 18)));
    let mut world = World::new();
    world.insert_resource(SpawnPoints::default());

    {
        let mut spawn_points = world.resource_mut::<SpawnPoints>();
        set_spawn_points_only(&def, &mut spawn_points);
    }

    let spawn_points = world.resource::<SpawnPoints>();
    assert_eq!(
        spawn_points.paddle,
        Some(expected_position(3, 4)),
        "paddle spawn should track the first matrix marker",
    );
    assert_eq!(
        spawn_points.ball,
        Some(expected_position(10, 18)),
        "ball spawn should track the first matrix marker",
    );
}

#[test]
fn falls_back_to_center_when_markers_missing() {
    let def = level_with_markers(None, None);
    let mut world = World::new();
    world.insert_resource(SpawnPoints::default());

    {
        let mut spawn_points = world.resource_mut::<SpawnPoints>();
        set_spawn_points_only(&def, &mut spawn_points);
    }

    let spawn_points = world.resource::<SpawnPoints>();
    let center = Vec3::new(0.0, 2.0, 0.0);
    assert_eq!(
        spawn_points.paddle,
        Some(center),
        "paddle spawn should fall back to the board center",
    );
    assert_eq!(
        spawn_points.ball,
        Some(center),
        "ball spawn should fall back to the board center",
    );
}
