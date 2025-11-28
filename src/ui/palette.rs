use bevy::prelude::*;

use crate::systems::textures::loader::ObjectClass;
use crate::systems::textures::TypeVariantRegistry;

#[derive(Resource, Default)]
pub struct PaletteState {
    pub open: bool,
}

#[derive(Component)]
pub struct PaletteRoot;

/// A small marker attached to spawned previews in the palette; stores the resolved
/// material handle when available so tests and systems can inspect the preview.
#[derive(Component, Debug)]
pub struct PalettePreview {
    pub type_id: u8,
    pub material: Option<Handle<StandardMaterial>>,
}

// (duplicate removed)

/// 3D preview viewport marker — small entity that stores mesh & material handles for a mini-preview.
#[derive(Component, Debug)]
pub struct PreviewViewport {
    pub type_id: u8,
    pub mesh: Handle<Mesh>,
    pub material: Option<Handle<StandardMaterial>>,
}

pub fn toggle_palette(keyboard: Res<ButtonInput<KeyCode>>, mut state: ResMut<PaletteState>) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        state.open = !state.open;
    }
}

/// Spawn or despawn the palette UI based on `PaletteState`.
pub fn ensure_palette_ui(
    state: Res<PaletteState>,
    mut commands: Commands,
    existing: Query<Entity, With<PaletteRoot>>,
    registry: Option<Res<'_, TypeVariantRegistry>>,
    materials_res: Option<Res<'_, Assets<StandardMaterial>>>,
    mut meshes_res: Option<ResMut<'_, Assets<Mesh>>>,
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
            .and_then(|r| r.get(ObjectClass::Brick, 20));
        let base_color_20 = material_20.as_ref().and_then(|h| {
            materials_res
                .as_ref()
                .and_then(|m| m.get(h).map(|mat| mat.base_color))
        });

        let material_90 = registry
            .as_ref()
            .and_then(|r| r.get(ObjectClass::Brick, 90));
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
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                // Simple brick preview (type 20)
                parent.spawn((
                    Text::new("20 — Simple Brick"),
                    TextFont {
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
                        type_id: 20,
                        material: material_20.clone(),
                    },
                ));

                // Indestructible preview (type 90)
                parent.spawn((
                    Text::new("90 — Indestructible (won't count toward completion)"),
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
                        type_id: 90,
                        material: material_90.clone(),
                    },
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
                        type_id: 20,
                        mesh: mesh_20.clone(),
                        material: Some(mat.clone()),
                    },
                ));
            } else {
                commands.spawn((
                    Mesh3d(mesh_20.clone()),
                    PreviewViewport {
                        type_id: 20,
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
                        type_id: 90,
                        mesh: mesh_90.clone(),
                        material: Some(mat.clone()),
                    },
                ));
            } else {
                commands.spawn((
                    Mesh3d(mesh_90.clone()),
                    PreviewViewport {
                        type_id: 90,
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
