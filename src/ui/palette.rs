//! Designer palette UI for brick selection and placement.
//!
//! Purpose
//! - Provides a lightweight in-game UI to select brick types and preview their appearance.
//! - Supports placing bricks onto the playfield grid via mouse drag while a type is selected.
//!
//! User flow
//! - Toggle visibility with `P` (`toggle_palette`).
//! - When opened, `ensure_palette_ui` spawns a simple UI (idempotent) showing entries for common
//!   bricks (e.g., simple and indestructible) and small color previews derived from materials.
//! - Click a preview to select a type (`handle_palette_selection`), which updates `SelectedBrick`.
//! - A "ghost" preview follows the cursor over the grid (`update_ghost_preview`).
//! - Hold the left mouse button and drag over grid cells to place bricks (`place_bricks_on_drag`).
//!
//! Integration details
//! - Uses `TypeVariantRegistry` when available to resolve `StandardMaterial` handles for previews;
//!   falls back to simple colors if materials are not yet ready.
//! - Text uses `UiFonts` when present; if missing (e.g., early WASM frames), the UI still spawns
//!   with default font handles.
//! - Grid conversion uses camera raycasting to map the cursor to world and grid coordinates.
//!
//! Scheduling summary (Update)
//! - `toggle_palette` updates `PaletteState`.
//! - `ensure_palette_ui` spawns/despawns UI when `PaletteState` changes.
//! - `handle_palette_selection` reacts to button `Interaction` changes.
//! - `update_palette_selection_feedback` highlights the selected preview.
//! - `update_ghost_preview` and `place_bricks_on_drag` manage visual feedback and placement.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::level_format::{INDESTRUCTIBLE_BRICK, SIMPLE_BRICK};
use crate::systems::textures::loader::ObjectClass;
use crate::systems::textures::TypeVariantRegistry;
use crate::ui::fonts::UiFonts;
use crate::{
    Brick, BrickTypeId, CountsTowardsCompletion, CELL_HEIGHT, CELL_WIDTH, GRID_HEIGHT, GRID_WIDTH,
    PLANE_H, PLANE_W,
};
use bevy_rapier3d::prelude::*;

#[derive(Resource, Default)]
pub struct PaletteState {
    pub open: bool,
}

/// Tracks the currently selected brick type from the palette.
/// When Some, designers can click/drag to place bricks in the level.
#[derive(Resource, Default)]
pub struct SelectedBrick {
    pub type_id: Option<u8>,
}

/// Cached ghost preview material handle (loaded once at startup).
/// Used as fallback when `TypeVariantRegistry` doesn't have a material for the selected brick type.
/// Constitution VIII: Asset Handle Reuse — load once, reuse everywhere.
#[derive(Resource)]
pub struct GhostPreviewMaterial {
    pub handle: Handle<StandardMaterial>,
}

/// Marker component for the root node of the designer palette UI.
///
/// Used to identify and manage the palette container entity for spawning/despawning.
#[derive(Component)]
pub struct PaletteRoot;

/// A small marker attached to spawned previews in the palette; stores the resolved
/// material handle when available so tests and systems can inspect the preview.
#[derive(Component, Debug)]
pub struct PalettePreview {
    pub type_id: u8,
    pub material: Option<Handle<StandardMaterial>>,
}

/// Marker for ghost preview brick that follows cursor during placement.
/// Constitution VIII: Required Components — all 3D entities require Transform + Visibility.
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct GhostPreview;

// (duplicate removed)

/// 3D preview viewport marker — small entity that stores mesh & material handles for a mini-preview.
/// Constitution VIII: Required Components — all 3D entities require Transform + Visibility.
#[derive(Component, Debug)]
#[require(Transform, Visibility)]
pub struct PreviewViewport {
    pub type_id: u8,
    pub mesh: Handle<Mesh>,
    pub material: Option<Handle<StandardMaterial>>,
}

pub fn toggle_palette(_keyboard: Res<ButtonInput<KeyCode>>, _state: ResMut<PaletteState>) {
    // Binding for 'P' removed to reserve the key for previous-level control in cheat mode.
    // Palette can still be toggled via UI or other explicit commands if needed.
}

/// Spawn or despawn the palette UI based on `PaletteState`.
pub fn ensure_palette_ui(
    state: Res<PaletteState>,
    mut commands: Commands,
    existing: Query<Entity, With<PaletteRoot>>,
    registry: Option<Res<'_, TypeVariantRegistry>>,
    materials_res: Option<Res<'_, Assets<StandardMaterial>>>,
    mut meshes_res: Option<ResMut<'_, Assets<Mesh>>>,
    ui_fonts: Option<Res<UiFonts>>,
    // meshes/materials optional (not present in every test harness) — keep function small for tests
) {
    if !state.is_changed() {
        return;
    }

    if state.open {
        if !existing.is_empty() {
            return;
        }
        // Root node - minimal layout so we don't depend on many style types in tests.
        // compute materials and colors ahead of mutably borrowing `commands`.
        let material_20 = registry
            .as_ref()
            .and_then(|r| r.get(ObjectClass::Brick, SIMPLE_BRICK));
        let base_color_20 = material_20.as_ref().and_then(|h| {
            materials_res
                .as_ref()
                .and_then(|m| m.get(h).map(|mat| mat.base_color))
        });

        let material_90 = registry
            .as_ref()
            .and_then(|r| r.get(ObjectClass::Brick, INDESTRUCTIBLE_BRICK));
        let base_color_90 = material_90.as_ref().and_then(|h| {
            materials_res
                .as_ref()
                .and_then(|m| m.get(h).map(|mat| mat.base_color))
        });
        // Use the project's lightweight text components (Text, TextFont, TextColor)
        // — this avoids pulling heavier UI style types into the test harness.
        commands
            .spawn((Node { ..default() }, PaletteRoot))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Designer Palette"),
                    TextFont {
                        font: ui_fonts
                            .as_ref()
                            .map(|f| f.orbitron.clone())
                            .unwrap_or_default(),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                // Simple brick preview (type 20)
                parent.spawn((
                    Text::new(format!("{} — Simple Brick", SIMPLE_BRICK)),
                    TextFont {
                        font: ui_fonts
                            .as_ref()
                            .map(|f| f.orbitron.clone())
                            .unwrap_or_default(),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                // small preview UI node for type 20. Background color is derived from material when available.

                parent.spawn((
                    Node {
                        width: Val::Px(48.0),
                        height: Val::Px(24.0),
                        margin: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(base_color_20.unwrap_or(Color::srgba(0.5, 0.5, 0.5, 1.0))),
                    PalettePreview {
                        type_id: SIMPLE_BRICK,
                        material: material_20.clone(),
                    },
                    Button,
                ));

                // Indestructible preview (type 90)
                parent.spawn((
                    Text::new(format!(
                        "{} — Indestructible (won't count toward completion)",
                        INDESTRUCTIBLE_BRICK
                    )),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgba(1.0, 0.84, 0.0, 1.0)),
                ));

                // base_color_90 and material_90 captured from outer scope

                parent.spawn((
                    Node {
                        width: Val::Px(48.0),
                        height: Val::Px(24.0),
                        margin: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(base_color_90.unwrap_or(Color::srgba(0.5, 0.5, 0.5, 1.0))),
                    PalettePreview {
                        type_id: INDESTRUCTIBLE_BRICK,
                        material: material_90.clone(),
                    },
                    Button,
                ));

                // 3D previews will be spawned after the UI node is created to avoid conflicting
                // mutable borrows of `commands` in the same scope.
            });

        // Spawn lightweight 3D preview entities (non-UI) when a Mesh asset store is available.
        if let Some(meshes) = meshes_res.as_mut() {
            let mesh_20 = meshes.add(Cuboid::new(0.5, 0.2, 0.5));
            if let Some(mat) = material_20.clone() {
                commands.spawn((
                    Mesh3d(mesh_20.clone()),
                    MeshMaterial3d(mat.clone()),
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    PreviewViewport {
                        type_id: SIMPLE_BRICK,
                        mesh: mesh_20.clone(),
                        material: Some(mat.clone()),
                    },
                ));
            } else {
                commands.spawn((
                    Mesh3d(mesh_20.clone()),
                    PreviewViewport {
                        type_id: SIMPLE_BRICK,
                        mesh: mesh_20.clone(),
                        material: None,
                    },
                ));
            }

            let mesh_90 = meshes.add(Cuboid::new(0.5, 0.2, 0.5));
            if let Some(mat) = material_90.clone() {
                commands.spawn((
                    Mesh3d(mesh_90.clone()),
                    MeshMaterial3d(mat.clone()),
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    PreviewViewport {
                        type_id: INDESTRUCTIBLE_BRICK,
                        mesh: mesh_90.clone(),
                        material: Some(mat.clone()),
                    },
                ));
            } else {
                commands.spawn((
                    Mesh3d(mesh_90.clone()),
                    PreviewViewport {
                        type_id: INDESTRUCTIBLE_BRICK,
                        mesh: mesh_90.clone(),
                        material: None,
                    },
                ));
            }
        }
    } else {
        // closed — remove existing
        for e in existing.iter() {
            commands.entity(e).despawn();
        }
    }
}

/// Detect clicks on palette preview buttons and update SelectedBrick.
pub fn handle_palette_selection(
    interactions: Query<(&Interaction, &PalettePreview), Changed<Interaction>>,
    mut selected: ResMut<SelectedBrick>,
) {
    for (interaction, preview) in interactions.iter() {
        if *interaction == Interaction::Pressed {
            selected.type_id = Some(preview.type_id);
            info!("Selected brick type {}", preview.type_id);
        }
    }
}

/// Resolve the base color to use for a palette preview, falling back to grey when the material is missing.
fn base_color_for(
    material: &Option<Handle<StandardMaterial>>,
    materials_res: &Option<Res<Assets<StandardMaterial>>>,
) -> Color {
    material
        .as_ref()
        .and_then(|h| {
            materials_res
                .as_ref()
                .and_then(|m| m.get(h).map(|mat| mat.base_color))
        })
        .unwrap_or(Color::srgba(0.5, 0.5, 0.5, 1.0))
}

/// Update visual feedback for selected palette item.
pub fn update_palette_selection_feedback(
    selected: Res<SelectedBrick>,
    mut param_set: ParamSet<(
        Query<(&PalettePreview, &mut BackgroundColor)>,
        Query<(&PalettePreview, &mut BackgroundColor), Added<PalettePreview>>,
    )>,
    materials_res: Option<Res<Assets<StandardMaterial>>>,
) {
    // Constitution VIII: Change-driven updates — only run when SelectedBrick changes or new previews spawn
    let selection_changed = selected.is_changed();
    let has_new_previews = !param_set.p1().is_empty();

    if !selection_changed && !has_new_previews {
        return; // No change, skip per-frame work
    }

    // Update existing previews if selection changed
    // Only process new previews when selection is unchanged to avoid double-processing
    if selection_changed {
        for (preview, mut bg_color) in param_set.p0().iter_mut() {
            let base = base_color_for(&preview.material, &materials_res);
            if Some(preview.type_id) == selected.type_id {
                // Highlight selected item with brighter yellow
                *bg_color = BackgroundColor(Color::srgba(1.0, 1.0, 0.0, 1.0));
            } else {
                // Restore original color from material
                *bg_color = BackgroundColor(base);
            }
        }
    } else if has_new_previews {
        // Initialize colors for newly spawned previews only (selection unchanged)
        for (preview, mut bg_color) in param_set.p1().iter_mut() {
            let base = base_color_for(&preview.material, &materials_res);
            if Some(preview.type_id) == selected.type_id {
                *bg_color = BackgroundColor(Color::srgba(1.0, 1.0, 0.0, 1.0));
            } else {
                *bg_color = BackgroundColor(base);
            }
        }
    }
}

/// Convert cursor position to grid coordinates on the ground plane.
/// Returns (grid_x, grid_z) indices if cursor is over the play area, or None if outside bounds.
fn cursor_to_grid(
    cursor_pos: Vec2,
    _window: &Window,
    camera_transform: &GlobalTransform,
    camera: &Camera,
) -> Option<(usize, usize)> {
    // Use Camera::viewport_to_world to get ray from cursor
    let ray = camera
        .viewport_to_world(camera_transform, cursor_pos)
        .ok()?;

    // Intersect with ground plane (y=0)
    let ray_direction = *ray.direction;
    if ray_direction.y.abs() < 0.001 {
        return None; // Ray is parallel to ground
    }

    let t = -ray.origin.y / ray_direction.y;
    if t < 0.0 {
        return None; // Intersection behind camera
    }

    let intersection = ray.origin + ray_direction * t;

    // Convert world position to grid coordinates
    // World space: X ∈ [-PLANE_H/2, PLANE_H/2], Z ∈ [-PLANE_W/2, PLANE_W/2]
    // Grid space: X ∈ [0, GRID_HEIGHT), Z ∈ [0, GRID_WIDTH)
    let x_normalized = (intersection.x + PLANE_H / 2.0) / PLANE_H;
    let z_normalized = (intersection.z + PLANE_W / 2.0) / PLANE_W;

    if !(0.0..1.0).contains(&x_normalized) || !(0.0..1.0).contains(&z_normalized) {
        return None; // Outside play area
    }

    let grid_x = (x_normalized * GRID_HEIGHT as f32).floor() as usize;
    let grid_z = (z_normalized * GRID_WIDTH as f32).floor() as usize;

    Some((grid_x, grid_z))
}

/// Update ghost preview position to follow cursor on grid.
pub fn update_ghost_preview(
    mut commands: Commands,
    selected: Res<SelectedBrick>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&GlobalTransform, &Camera), With<Camera3d>>,
    ghost: Query<Entity, With<GhostPreview>>,
    registry: Option<Res<TypeVariantRegistry>>,
    cached_material: Option<Res<GhostPreviewMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Ok(window) = window.single() else {
        return;
    };

    let Ok((camera_transform, camera)) = camera_query.single() else {
        return;
    };

    // Get cursor position
    let Some(cursor_pos) = window.cursor_position() else {
        // No cursor - remove ghost
        for entity in ghost.iter() {
            commands.entity(entity).despawn();
        }
        return;
    };

    // Check if brick type is selected
    let Some(type_id) = selected.type_id else {
        // No selection - remove ghost
        for entity in ghost.iter() {
            commands.entity(entity).despawn();
        }
        return;
    };

    // Convert cursor to grid position
    let Some((grid_x, grid_z)) = cursor_to_grid(cursor_pos, window, camera_transform, camera)
    else {
        // Cursor outside play area - remove ghost
        for entity in ghost.iter() {
            commands.entity(entity).despawn();
        }
        return;
    };

    // Calculate world position from grid coordinates
    let world_x = -PLANE_H / 2.0 + (grid_x as f32 + 0.5) * CELL_HEIGHT;
    let world_z = -PLANE_W / 2.0 + (grid_z as f32 + 0.5) * CELL_WIDTH;
    let world_pos = Vec3::new(world_x, 0.5, world_z);

    // Get material for this brick type from registry or use cached fallback
    // Constitution VIII: Asset Handle Reuse — no per-frame material allocation
    let Some(material) = registry
        .as_ref()
        .and_then(|r| r.get(ObjectClass::Brick, type_id))
        .or_else(|| cached_material.as_ref().map(|c| c.handle.clone()))
    else {
        // No material available; log warning and remove ghost to avoid rendering with invalid handle
        warn!(
            "No material available for ghost preview (type {}); registry and cached fallback both missing",
            type_id
        );
        for entity in ghost.iter() {
            commands.entity(entity).despawn();
        }
        return;
    };

    // Update existing ghost or spawn new one
    if let Some(ghost_entity) = ghost.iter().next() {
        commands
            .entity(ghost_entity)
            .insert(Transform::from_translation(world_pos));
    } else {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(CELL_HEIGHT * 0.9, 0.4, CELL_WIDTH * 0.9))),
            MeshMaterial3d(material),
            Transform::from_translation(world_pos),
            GhostPreview,
        ));
    }
}

/// Place bricks when mouse is held and dragged over grid cells.
pub fn place_bricks_on_drag(
    mut commands: Commands,
    selected: Res<SelectedBrick>,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&GlobalTransform, &Camera), With<Camera3d>>,
    registry: Option<Res<TypeVariantRegistry>>,
    mut meshes: ResMut<Assets<Mesh>>,
    existing_bricks: Query<&Transform, With<Brick>>,
) {
    // Only place when left mouse button is held
    if !mouse.pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = window.single() else {
        return;
    };

    let Ok((camera_transform, camera)) = camera_query.single() else {
        return;
    };

    // Check if brick type is selected
    let Some(type_id) = selected.type_id else {
        return;
    };

    // Get cursor position
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // Convert cursor to grid position
    let Some((grid_x, grid_z)) = cursor_to_grid(cursor_pos, window, camera_transform, camera)
    else {
        return;
    };

    // Calculate world position
    let world_x = -PLANE_H / 2.0 + (grid_x as f32 + 0.5) * CELL_HEIGHT;
    let world_z = -PLANE_W / 2.0 + (grid_z as f32 + 0.5) * CELL_WIDTH;
    let world_pos = Vec3::new(world_x, 0.5, world_z);

    // Check if brick already exists at this position (within small tolerance)
    const POSITION_TOLERANCE: f32 = 0.1;
    for existing_transform in existing_bricks.iter() {
        if existing_transform.translation.distance(world_pos) < POSITION_TOLERANCE {
            return; // Brick already exists here
        }
    }

    // Get material for this brick type
    let material = registry
        .as_ref()
        .and_then(|r| r.get(ObjectClass::Brick, type_id));

    // Spawn brick with appropriate components
    let mut brick_entity = commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(CELL_HEIGHT * 0.9, 1.0, CELL_WIDTH * 0.9))),
        Transform::from_translation(world_pos),
        Collider::cuboid(CELL_HEIGHT * 0.45, 0.5, CELL_WIDTH * 0.45),
        Brick,
        BrickTypeId(type_id),
    ));

    // Add material if available
    if let Some(mat) = material {
        brick_entity.insert(MeshMaterial3d(mat));
    }

    // Indestructible bricks (type 90) should NOT count towards completion
    if type_id != 90 {
        brick_entity.insert(CountsTowardsCompletion);
    }

    info!(
        "Placed brick type {} at grid ({}, {})",
        type_id, grid_x, grid_z
    );
}
