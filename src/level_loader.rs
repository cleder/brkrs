use crate::systems::level_switch::{LevelSwitchRequested, LevelSwitchState};
use crate::systems::respawn::{RespawnEntityKind, RespawnHandle, SpawnPoints, SpawnTransform};
#[cfg(feature = "texture_manifest")]
use crate::systems::textures::{
    baseline_material_handle, brick_type_material_handle, BaselineMaterialKind,
    CanonicalMaterialHandles, FallbackRegistry, LevelPresentation, TextureManifest,
    TypeVariantRegistry,
};
#[cfg(feature = "texture_manifest")]
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use ron::de::from_str;
use serde::Deserialize;

use crate::{
    Ball, BallTypeId, Brick, BrickTypeId, GameProgress, GravityConfig, LowerGoal, Paddle,
    BALL_RADIUS, CELL_HEIGHT, CELL_WIDTH, PADDLE_HEIGHT, PADDLE_RADIUS, PLANE_H, PLANE_W,
};
use bevy_rapier3d::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelAdvanceSet;

/// Bundled texture-related resources to reduce system parameter count.
#[cfg(feature = "texture_manifest")]
#[derive(SystemParam)]
pub struct TextureResources<'w> {
    pub canonical: Option<Res<'w, CanonicalMaterialHandles>>,
    pub fallback: Option<ResMut<'w, FallbackRegistry>>,
    pub type_registry: Option<Res<'w, TypeVariantRegistry>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LevelDefinition {
    pub number: u32,
    /// Optional gravity override for this level (x,y,z). If omitted the existing GravityConfig value is used.
    pub gravity: Option<(f32, f32, f32)>,
    pub matrix: Vec<Vec<u8>>, // expect 20 x 20
    #[cfg(feature = "texture_manifest")]
    #[serde(default)]
    pub presentation: Option<crate::systems::textures::loader::LevelTextureSet>,
}

#[derive(Resource, Debug)]
pub struct CurrentLevel(pub LevelDefinition);

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GravityConfig>();
        app.add_systems(Startup, (load_level, spawn_level_entities).chain());
        #[cfg(feature = "texture_manifest")]
        app.add_systems(
            Update,
            (
                advance_level_when_cleared,
                handle_level_advance_delay,
                finalize_level_advance.after(handle_level_advance_delay),
                spawn_fade_overlay_if_needed,
                update_fade_overlay,
                restart_level_on_key,
                destroy_all_bricks_on_key,
                process_level_switch_requests,
                sync_level_presentation,
            )
                .in_set(LevelAdvanceSet),
        );
        #[cfg(not(feature = "texture_manifest"))]
        app.add_systems(
            Update,
            (
                advance_level_when_cleared,
                handle_level_advance_delay,
                finalize_level_advance.after(handle_level_advance_delay),
                spawn_fade_overlay_if_needed,
                update_fade_overlay,
                restart_level_on_key,
                destroy_all_bricks_on_key,
                process_level_switch_requests,
            ),
        );
    }
}

/// State machine for delayed level advancement and growth animation.
#[derive(Resource)]
pub struct LevelAdvanceState {
    pub timer: Timer,                     // initial delay before spawning growth paddle
    pub active: bool,                     // transition in progress
    pub growth_spawned: bool,             // tiny paddle+ball spawned, waiting for growth completion
    pub pending: Option<LevelDefinition>, // next level definition awaiting brick spawn
}

impl Default for LevelAdvanceState {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            active: false,
            growth_spawned: false,
            pending: None,
        }
    }
}

/// Full-screen UI overlay used for fade in/out during level transitions.
#[derive(Component)]
struct FadeOverlay;

fn paddle_spawn_transform(position: Vec3) -> SpawnTransform {
    SpawnTransform::new(position, Quat::from_rotation_x(-std::f32::consts::PI / 2.0))
}

fn ball_spawn_transform(position: Vec3) -> SpawnTransform {
    SpawnTransform::new(position, Quat::IDENTITY)
}

fn paddle_respawn_handle(position: Vec3) -> RespawnHandle {
    RespawnHandle {
        spawn: paddle_spawn_transform(position),
        kind: RespawnEntityKind::Paddle,
    }
}

fn ball_respawn_handle(position: Vec3) -> RespawnHandle {
    RespawnHandle {
        spawn: ball_spawn_transform(position),
        kind: RespawnEntityKind::Ball,
    }
}

/// Normalize matrix to 20x20 dimensions with padding/truncation
fn normalize_matrix(mut matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    const TARGET_ROWS: usize = 20;
    const TARGET_COLS: usize = 20;

    let original_rows = matrix.len();
    let original_cols = matrix.first().map_or(0, |r| r.len());

    // Log warning if dimensions don't match
    if original_rows != TARGET_ROWS || original_cols != TARGET_COLS {
        warn!(
            "Level matrix wrong dimensions; expected 20x20, got {}x{}",
            original_rows, original_cols
        );
    }

    // Pad rows if needed
    while matrix.len() < TARGET_ROWS {
        matrix.push(vec![0; TARGET_COLS]);
    }

    // Truncate rows if needed
    if matrix.len() > TARGET_ROWS {
        warn!(
            "Level matrix has {} rows; truncating to {}",
            matrix.len(),
            TARGET_ROWS
        );
        matrix.truncate(TARGET_ROWS);
    }

    // Pad/truncate columns
    for (i, row) in matrix.iter_mut().enumerate() {
        let original_row_len = row.len();

        // Pad columns if needed
        while row.len() < TARGET_COLS {
            row.push(0);
        }

        // Truncate columns if needed
        if original_row_len > TARGET_COLS {
            warn!(
                "Row {} has {} columns; truncating to {}",
                i, original_row_len, TARGET_COLS
            );
            row.truncate(TARGET_COLS);
        }
    }

    matrix
}

fn ensure_lower_goal_sensor(commands: &mut Commands, existing: &Query<Entity, With<LowerGoal>>) {
    if !existing.is_empty() {
        return;
    }

    let half_thickness = 0.25;
    let half_height = 2.5;
    let half_width = PLANE_W / 2.0;
    let sensor_x = PLANE_H / 2.0 + half_thickness;

    commands.spawn((
        Transform::from_xyz(sensor_x, 0.0, 0.0),
        GlobalTransform::default(),
        Collider::cuboid(half_thickness, half_height, half_width),
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
        LowerGoal,
    ));
}

fn load_level(
    mut commands: Commands,
    mut gravity_cfg: ResMut<GravityConfig>,
    mut rapier_config: Query<&mut RapierConfiguration>,
) {
    #[cfg(not(target_arch = "wasm32"))]
    let chosen_path = {
        use std::env;
        if let Ok(num) = env::var("BK_LEVEL") {
            let trimmed = num.trim();
            if let Ok(n) = trimmed.parse::<u32>() {
                format!("assets/levels/level_{:03}.ron", n)
            } else {
                warn!("BK_LEVEL='{}' not a number; defaulting to level_001", num);
                "assets/levels/level_001.ron".to_string()
            }
        } else {
            "assets/levels/level_001.ron".to_string()
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    let level_str: &str = match std::fs::read_to_string(&chosen_path) {
        Ok(s) => {
            info!("Loading level file: {}", chosen_path);
            Box::leak(s.into_boxed_str())
        }
        Err(e) => {
            warn!(
                "Failed to read level file '{}': {e}. Falling back to empty level",
                chosen_path
            );
            "LevelDefinition(number:0,matrix:[])"
        }
    };
    #[cfg(target_arch = "wasm32")]
    let level_str: &str = include_str!("../assets/levels/level_001.ron"); // initial level only; subsequent loads use embedded helper

    match from_str::<LevelDefinition>(level_str) {
        Ok(mut def) => {
            // Normalize matrix to 20x20 with padding/truncation
            def.matrix = normalize_matrix(def.matrix);
            info!("Loaded level {}", def.number);
            // Apply per-level gravity if present
            if let Some((x, y, z)) = def.gravity {
                gravity_cfg.normal = Vec3::new(x, y, z);
                if let Ok(mut config) = rapier_config.single_mut() {
                    config.gravity = gravity_cfg.normal;
                }
                info!("Level gravity set to {:?}", gravity_cfg.normal);
            } else {
                debug!(
                    "Level has no gravity override; using existing {:?}",
                    gravity_cfg.normal
                );
            }
            commands.insert_resource(CurrentLevel(def));
        }
        Err(e) => {
            warn!("Failed to parse level: {e}");
        }
    }
}

fn spawn_level_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_points: ResMut<SpawnPoints>,
    lower_goal: Query<Entity, With<LowerGoal>>,
    level: Option<Res<CurrentLevel>>,
    #[cfg(feature = "texture_manifest")] canonical: Option<Res<CanonicalMaterialHandles>>,
    #[cfg(feature = "texture_manifest")] mut fallback: Option<ResMut<FallbackRegistry>>,
    #[cfg(feature = "texture_manifest")] type_registry: Option<Res<TypeVariantRegistry>>,
) {
    let Some(level) = level else {
        return;
    };
    ensure_lower_goal_sensor(&mut commands, &lower_goal);
    spawn_level_entities_impl(
        &level.0,
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut spawn_points,
        #[cfg(feature = "texture_manifest")]
        canonical.as_deref(),
        #[cfg(feature = "texture_manifest")]
        fallback.as_deref_mut(),
        #[cfg(feature = "texture_manifest")]
        type_registry.as_deref(),
    );
}

fn spawn_level_entities_impl(
    def: &LevelDefinition,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    spawn_points: &mut ResMut<SpawnPoints>,
    #[cfg(feature = "texture_manifest")] canonical: Option<&CanonicalMaterialHandles>,
    #[cfg(feature = "texture_manifest")] mut fallback: Option<&mut FallbackRegistry>,
    #[cfg(feature = "texture_manifest")] type_registry: Option<&TypeVariantRegistry>,
) {
    debug!("Spawning entities for level {}", def.number);
    // Shared material
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        unlit: false,
        ..default()
    });
    let default_brick_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.1, 0.1),
        unlit: false,
        ..default()
    });

    #[cfg(feature = "texture_manifest")]
    let canonical_handles = canonical;

    let paddle_material = {
        #[cfg(feature = "texture_manifest")]
        {
            baseline_material_handle(
                canonical_handles,
                fallback.as_deref_mut(),
                BaselineMaterialKind::Paddle,
                "level_loader.spawn_level_entities.paddle",
            )
            .unwrap_or_else(|| debug_material.clone())
        }
        #[cfg(not(feature = "texture_manifest"))]
        {
            debug_material.clone()
        }
    };

    let ball_material = {
        #[cfg(feature = "texture_manifest")]
        {
            baseline_material_handle(
                canonical_handles,
                fallback.as_deref_mut(),
                BaselineMaterialKind::Ball,
                "level_loader.spawn_level_entities.ball",
            )
            .unwrap_or_else(|| debug_material.clone())
        }
        #[cfg(not(feature = "texture_manifest"))]
        {
            debug_material.clone()
        }
    };

    let brick_material = {
        #[cfg(feature = "texture_manifest")]
        {
            baseline_material_handle(
                canonical_handles,
                fallback.as_deref_mut(),
                BaselineMaterialKind::Brick,
                "level_loader.spawn_level_entities.brick",
            )
            .unwrap_or_else(|| default_brick_material.clone())
        }
        #[cfg(not(feature = "texture_manifest"))]
        {
            default_brick_material.clone()
        }
    };

    let mut paddle_spawned = false;
    let mut ball_spawned = false;

    spawn_points.paddle = None;
    spawn_points.ball = None;

    for (row, row_data) in def.matrix.iter().enumerate() {
        for (col, value) in row_data.iter().enumerate() {
            let x = -PLANE_H / 2.0 + (row as f32 + 0.5) * CELL_HEIGHT;
            let z = -PLANE_W / 2.0 + (col as f32 + 0.5) * CELL_WIDTH;
            match value {
                0 => {}
                1 => {
                    // Paddle
                    if !paddle_spawned {
                        paddle_spawned = true;
                        let position = Vec3::new(x, 2.0, z);
                        spawn_points.paddle = Some(position);
                        commands
                            .spawn((
                                Mesh3d(
                                    meshes.add(Capsule3d::new(PADDLE_RADIUS, PADDLE_HEIGHT).mesh()),
                                ),
                                MeshMaterial3d(paddle_material.clone()),
                                Transform::from_xyz(x, 2.0, z).with_rotation(
                                    Quat::from_rotation_x(-std::f32::consts::PI / 2.0),
                                ),
                                Paddle,
                                RigidBody::KinematicPositionBased,
                                GravityScale(0.0),
                                CollidingEntities::default(),
                                Collider::capsule_y(PADDLE_HEIGHT / 2.0, PADDLE_RADIUS),
                                LockedAxes::TRANSLATION_LOCKED_Y,
                                KinematicCharacterController::default(),
                                Ccd::enabled(),
                                Friction {
                                    coefficient: 2.0,
                                    combine_rule: CoefficientCombineRule::Max,
                                },
                            ))
                            .insert(paddle_respawn_handle(position));
                    }
                }
                2 => {
                    // Ball
                    if !ball_spawned {
                        ball_spawned = true;
                        let position = Vec3::new(x, 2.0, z);
                        spawn_points.ball = Some(position);
                        commands
                            .spawn((
                                Mesh3d(meshes.add(Sphere::new(BALL_RADIUS).mesh())),
                                MeshMaterial3d(ball_material.clone()),
                                Transform::from_xyz(x, 2.0, z),
                                Ball,
                                RigidBody::Dynamic,
                                CollidingEntities::default(),
                                ActiveEvents::COLLISION_EVENTS,
                                Collider::ball(BALL_RADIUS),
                                Restitution {
                                    coefficient: 0.9,
                                    combine_rule: CoefficientCombineRule::Max,
                                },
                                Friction {
                                    coefficient: 2.0,
                                    combine_rule: CoefficientCombineRule::Max,
                                },
                                Damping {
                                    linear_damping: 0.5,
                                    angular_damping: 0.5,
                                },
                            ))
                            .insert((
                                LockedAxes::TRANSLATION_LOCKED_Y,
                                Ccd::enabled(),
                                ExternalImpulse::default(),
                                GravityScale(1.0),
                            ))
                            .insert(BallTypeId(0)) // Default ball type
                            .insert(ball_respawn_handle(position));
                    }
                }
                brick_type @ 3..=255 => {
                    // Brick with type ID (value 3+ maps to brick types)
                    let brick_type_id = *brick_type;
                    #[cfg(feature = "texture_manifest")]
                    let brick_mat = {
                        // Try type-variant lookup first, then fall back to canonical brick
                        type_registry
                            .and_then(|reg| {
                                reg.get(crate::systems::textures::ObjectClass::Brick, brick_type_id)
                            })
                            .or_else(|| {
                                brick_type_material_handle(
                                    type_registry,
                                    fallback.as_deref_mut(),
                                    brick_type_id,
                                    "level_loader.spawn_level_entities.brick",
                                )
                            })
                            .unwrap_or_else(|| brick_material.clone())
                    };
                    #[cfg(not(feature = "texture_manifest"))]
                    let brick_mat = brick_material.clone();

                    commands.spawn((
                        Mesh3d(meshes.add(Cuboid::new(CELL_HEIGHT * 0.9, 0.5, CELL_WIDTH * 0.9))),
                        MeshMaterial3d(brick_mat),
                        Transform::from_xyz(x, 2.0, z),
                        Brick,
                        BrickTypeId(brick_type_id),
                        RigidBody::Fixed,
                        Collider::cuboid(CELL_HEIGHT * 0.9 / 2.0, 0.25, CELL_WIDTH * 0.9 / 2.0),
                        Restitution {
                            coefficient: 1.0,
                            combine_rule: CoefficientCombineRule::Max,
                        },
                        CollidingEntities::default(),
                        ActiveEvents::COLLISION_EVENTS,
                    ));
                }
            }
        }
    }

    if !paddle_spawned {
        warn!("No paddle found in level matrix; spawning fallback paddle.");
        let x = 0.0;
        let z = 0.0;
        let position = Vec3::new(x, 2.0, z);
        spawn_points.paddle = Some(position);
        commands
            .spawn((
                Mesh3d(meshes.add(Capsule3d::new(PADDLE_RADIUS, PADDLE_HEIGHT).mesh())),
                MeshMaterial3d(paddle_material.clone()),
                Transform::from_xyz(x, 2.0, z)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
                Paddle,
                RigidBody::KinematicPositionBased,
                GravityScale(0.0),
                CollidingEntities::default(),
                Collider::capsule_y(PADDLE_HEIGHT / 2.0, PADDLE_RADIUS),
                LockedAxes::TRANSLATION_LOCKED_Y,
                KinematicCharacterController::default(),
                Ccd::enabled(),
                Friction {
                    coefficient: 2.0,
                    combine_rule: CoefficientCombineRule::Max,
                },
            ))
            .insert(paddle_respawn_handle(position));
    }
    if !ball_spawned {
        warn!("No ball found in level matrix; spawning fallback ball.");
        let position = Vec3::new(0.0, 2.0, 0.0);
        spawn_points.ball = Some(position);
        commands
            .spawn((
                Mesh3d(meshes.add(Sphere::new(BALL_RADIUS).mesh())),
                MeshMaterial3d(ball_material.clone()),
                Transform::from_xyz(0.0, 2.0, 0.0),
                Ball,
                RigidBody::Dynamic,
                CollidingEntities::default(),
                ActiveEvents::COLLISION_EVENTS,
                Collider::ball(BALL_RADIUS),
                Restitution {
                    coefficient: 0.9,
                    combine_rule: CoefficientCombineRule::Max,
                },
                Friction {
                    coefficient: 2.0,
                    combine_rule: CoefficientCombineRule::Max,
                },
                Damping {
                    linear_damping: 0.5,
                    angular_damping: 0.5,
                },
            ))
            .insert((
                LockedAxes::TRANSLATION_LOCKED_Y,
                Ccd::enabled(),
                ExternalImpulse::default(),
                GravityScale(1.0),
            ))
            .insert(ball_respawn_handle(position));
    }
}

/// Only spawn bricks for a level (used after paddle growth completes during level advance).
fn spawn_bricks_only(
    def: &LevelDefinition,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    #[cfg(feature = "texture_manifest")] canonical: Option<&CanonicalMaterialHandles>,
    #[cfg(feature = "texture_manifest")] mut fallback: Option<&mut FallbackRegistry>,
    #[cfg(feature = "texture_manifest")] type_registry: Option<&TypeVariantRegistry>,
) {
    let default_brick_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.1, 0.1),
        unlit: false,
        ..default()
    });

    #[cfg(feature = "texture_manifest")]
    let canonical_handles = canonical;

    let brick_material = {
        #[cfg(feature = "texture_manifest")]
        {
            baseline_material_handle(
                canonical_handles,
                fallback.as_deref_mut(),
                BaselineMaterialKind::Brick,
                "level_loader.spawn_bricks_only",
            )
            .unwrap_or_else(|| default_brick_material.clone())
        }
        #[cfg(not(feature = "texture_manifest"))]
        {
            default_brick_material.clone()
        }
    };

    for (row, row_data) in def.matrix.iter().enumerate() {
        for (col, value) in row_data.iter().enumerate() {
            if *value < 3 {
                continue;
            }
            let brick_type_id = *value;
            let x = -PLANE_H / 2.0 + (row as f32 + 0.5) * CELL_HEIGHT;
            let z = -PLANE_W / 2.0 + (col as f32 + 0.5) * CELL_WIDTH;

            #[cfg(feature = "texture_manifest")]
            let brick_mat = {
                type_registry
                    .and_then(|reg| {
                        reg.get(crate::systems::textures::ObjectClass::Brick, brick_type_id)
                    })
                    .or_else(|| {
                        brick_type_material_handle(
                            type_registry,
                            fallback.as_deref_mut(),
                            brick_type_id,
                            "level_loader.spawn_bricks_only",
                        )
                    })
                    .unwrap_or_else(|| brick_material.clone())
            };
            #[cfg(not(feature = "texture_manifest"))]
            let brick_mat = brick_material.clone();

            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(CELL_HEIGHT * 0.9, 0.5, CELL_WIDTH * 0.9))),
                MeshMaterial3d(brick_mat),
                Transform::from_xyz(x, 2.0, z),
                Brick,
                BrickTypeId(brick_type_id),
                RigidBody::Fixed,
                Collider::cuboid(CELL_HEIGHT * 0.9 / 2.0, 0.25, CELL_WIDTH * 0.9 / 2.0),
                Restitution {
                    coefficient: 1.0,
                    combine_rule: CoefficientCombineRule::Max,
                },
                CollidingEntities::default(),
                ActiveEvents::COLLISION_EVENTS,
            ));
        }
    }
}

/// Extract and set spawn points for paddle & ball from a level definition (without spawning bricks).
/// Recomputes paddle and ball spawn points from a level definition.
///
/// Exposed for integration tests to validate fallback behavior. Runtime code should
/// continue invoking this through the level loader systems.
pub fn set_spawn_points_only(def: &LevelDefinition, spawn_points: &mut SpawnPoints) {
    spawn_points.paddle = None;
    spawn_points.ball = None;
    let mut paddle_set = false;
    let mut ball_set = false;
    for (row, row_data) in def.matrix.iter().enumerate() {
        for (col, value) in row_data.iter().enumerate() {
            let x = -PLANE_H / 2.0 + (row as f32 + 0.5) * CELL_HEIGHT;
            let z = -PLANE_W / 2.0 + (col as f32 + 0.5) * CELL_WIDTH;
            match value {
                1 if !paddle_set => {
                    paddle_set = true;
                    spawn_points.paddle = Some(Vec3::new(x, 2.0, z));
                }
                2 if !ball_set => {
                    ball_set = true;
                    spawn_points.ball = Some(Vec3::new(x, 2.0, z));
                }
                _ => {}
            }
        }
    }
    if spawn_points.paddle.is_none() {
        spawn_points.paddle = Some(Vec3::new(0.0, 2.0, 0.0));
    }
    if spawn_points.ball.is_none() {
        spawn_points.ball = Some(Vec3::new(0.0, 2.0, 0.0));
    }
}

/// Advance to the next level when all bricks have been cleared.
fn advance_level_when_cleared(
    bricks: Query<Entity, With<Brick>>,
    paddle_q: Query<Entity, With<Paddle>>,
    ball_q: Query<Entity, With<Ball>>,
    current_level: Option<Res<CurrentLevel>>,
    mut commands: Commands,
    mut game_progress: ResMut<GameProgress>,
    mut level_advance: ResMut<LevelAdvanceState>,
) {
    let Some(curr) = current_level else {
        return;
    };
    if !bricks.is_empty() {
        return; // still bricks remaining
    }
    // If already transitioning, don't restart it.
    if level_advance.active {
        return;
    }
    let next_number = curr.0.number + 1;
    let path = format!("assets/levels/level_{:03}.ron", next_number);
    #[cfg(not(target_arch = "wasm32"))]
    let level_exists = std::path::Path::new(&path).exists();
    #[cfg(target_arch = "wasm32")]
    let level_exists = embedded_level_str(&path).is_some();
    if !level_exists {
        if !game_progress.finished {
            info!(
                "All bricks cleared; no next level file {}. Game complete.",
                path
            );
            game_progress.finished = true;
            // Despawn remaining paddle and ball to freeze gameplay
            for p in paddle_q.iter() {
                commands.entity(p).despawn();
            }
            for b in ball_q.iter() {
                commands.entity(b).despawn();
            }
            // Spawn completion text (desktop only UI style similar to wireframe text)
            #[cfg(not(target_arch = "wasm32"))]
            commands.spawn((
                Text::new("GAME COMPLETE - Press Q to Quit"),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(60.0),
                    left: Val::Px(60.0),
                    ..default()
                },
            ));
        }
        return;
    }
    // Parse and store next level; delay spawning via LevelAdvanceState.
    #[cfg(not(target_arch = "wasm32"))]
    let file_content_result = std::fs::read_to_string(&path);
    #[cfg(target_arch = "wasm32")]
    let file_content_result = embedded_level_str(&path)
        .map(|s| s.to_string())
        .ok_or_else(|| format!("failed to read level file '{path}': embedded asset missing"));
    match file_content_result {
        Ok(content) => match from_str::<LevelDefinition>(&content) {
            Ok(def) => {
                info!("Preparing advancement to level {} (delayed)", def.number);
                level_advance.timer.reset();
                level_advance.active = true;
                level_advance.growth_spawned = false;
                level_advance.pending = Some(def);
                // Despawn paddle & ball now to show empty field during fade-out.
                for p in paddle_q.iter() {
                    commands.entity(p).despawn();
                }
                for b in ball_q.iter() {
                    commands.entity(b).despawn();
                }
            }
            Err(e) => warn!("Failed to parse next level '{}': {e}", path),
        },
        Err(e) => warn!("Failed to read next level file '{}': {e}", path),
    }
}

/// Restart the current level when the user presses R.
fn restart_level_on_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_level: Option<Res<CurrentLevel>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_points: ResMut<SpawnPoints>,
    mut gravity_cfg: ResMut<GravityConfig>,
    mut rapier_config: Query<&mut RapierConfiguration>,
    bricks: Query<Entity, With<Brick>>,
    paddle_q: Query<Entity, With<Paddle>>,
    ball_q: Query<Entity, With<Ball>>,
    mut game_progress: ResMut<GameProgress>,
    mut level_advance: ResMut<LevelAdvanceState>,
    #[cfg(feature = "texture_manifest")] mut tex_res: TextureResources,
) {
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }
    info!("R key pressed - restarting level");
    let level_number = current_level.map(|cl| cl.0.number).unwrap_or(1);
    let path = format!("assets/levels/level_{:03}.ron", level_number);
    match force_load_level_from_path(
        &path,
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut spawn_points,
        &mut gravity_cfg,
        &mut rapier_config,
        &bricks,
        &paddle_q,
        &ball_q,
        &mut game_progress,
        &mut level_advance,
        #[cfg(feature = "texture_manifest")]
        tex_res.canonical.as_deref(),
        #[cfg(feature = "texture_manifest")]
        tex_res.fallback.as_deref_mut(),
        #[cfg(feature = "texture_manifest")]
        tex_res.type_registry.as_deref(),
    ) {
        Ok(_) => info!("Restarted level {level_number}"),
        Err(err) => warn!("Failed to restart level {level_number}: {err}"),
    }
}

/// Destroy all bricks when K is pressed (for testing level transitions).
fn destroy_all_bricks_on_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    bricks: Query<Entity, With<Brick>>,
    mut commands: Commands,
) {
    // Simple key press - just K to destroy all bricks for testing
    if keyboard.just_pressed(KeyCode::KeyK) {
        info!("K key pressed");
        let count = bricks.iter().len();
        if count > 0 {
            info!("Destroying {} brick(s) for level transition testing", count);
            for entity in bricks.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub(crate) fn process_level_switch_requests(
    mut requests: EventReader<LevelSwitchRequested>,
    mut switch_state: ResMut<LevelSwitchState>,
    current_level: Option<Res<CurrentLevel>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_points: ResMut<SpawnPoints>,
    mut gravity_cfg: ResMut<GravityConfig>,
    mut rapier_config: Query<&mut RapierConfiguration>,
    bricks: Query<Entity, With<Brick>>,
    paddle_q: Query<Entity, With<Paddle>>,
    ball_q: Query<Entity, With<Ball>>,
    mut game_progress: ResMut<GameProgress>,
    mut level_advance: ResMut<LevelAdvanceState>,
    #[cfg(feature = "texture_manifest")] mut tex_res: TextureResources,
) {
    if requests.is_empty() {
        return;
    }
    if switch_state.is_transition_pending() || level_advance.active {
        info!(
            target: "level_switch",
            "Level transition already active; ignoring switch request"
        );
        requests.clear();
        return;
    }
    let current_number = current_level.map(|c| c.0.number).unwrap_or(0);
    let Some(target_slot) = switch_state.next_level_after(current_number).cloned() else {
        warn!(target: "level_switch", "No level entries available for switching");
        requests.clear();
        return;
    };
    switch_state.mark_transition_start();
    match force_load_level_from_path(
        &target_slot.path,
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut spawn_points,
        &mut gravity_cfg,
        &mut rapier_config,
        &bricks,
        &paddle_q,
        &ball_q,
        &mut game_progress,
        &mut level_advance,
        #[cfg(feature = "texture_manifest")]
        tex_res.canonical.as_deref(),
        #[cfg(feature = "texture_manifest")]
        tex_res.fallback.as_deref_mut(),
        #[cfg(feature = "texture_manifest")]
        tex_res.type_registry.as_deref(),
    ) {
        Ok(def) => info!(
            target: "level_switch",
            number = def.number,
            path = %target_slot.path,
            "Level switch completed"
        ),
        Err(err) => warn!(
            target: "level_switch",
            path = %target_slot.path,
            "Failed to switch levels: {err}"
        ),
    }
    switch_state.mark_transition_end();
    requests.clear();
}

/// After delay, spawn tiny paddle + frozen ball for growth animation (similar to respawn), defer bricks.
fn handle_level_advance_delay(
    time: Res<Time>,
    mut level_advance: ResMut<LevelAdvanceState>,
    mut spawn_points: ResMut<SpawnPoints>,
    mut gravity_cfg: ResMut<GravityConfig>,
    mut rapier_config: Query<&mut RapierConfiguration>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    #[cfg(feature = "texture_manifest")] canonical: Option<Res<CanonicalMaterialHandles>>,
    #[cfg(feature = "texture_manifest")] mut fallback: Option<ResMut<FallbackRegistry>>,
    #[cfg(feature = "texture_manifest")] type_registry: Option<Res<TypeVariantRegistry>>,
) {
    if !level_advance.active || level_advance.pending.is_none() || level_advance.growth_spawned {
        return;
    }
    level_advance.timer.tick(time.delta());
    if !level_advance.timer.finished() {
        return;
    }
    let def = level_advance.pending.as_ref().unwrap();

    // Spawn bricks at peak of fade (when screen is fully black)
    #[cfg(feature = "texture_manifest")]
    let canonical_handles = canonical.as_deref();

    spawn_bricks_only(
        def,
        &mut commands,
        &mut meshes,
        &mut materials,
        #[cfg(feature = "texture_manifest")]
        canonical_handles,
        #[cfg(feature = "texture_manifest")]
        fallback.as_deref_mut(),
        #[cfg(feature = "texture_manifest")]
        type_registry.as_deref(),
    );

    // Apply per-level gravity (or keep current) immediately so it is ready when the ball unfreezes.
    let target_gravity = if let Some((x, y, z)) = def.gravity {
        let vec = Vec3::new(x, y, z);
        gravity_cfg.normal = vec;
        info!(
            "Level {} gravity set to {:?} in handle_level_advance_delay",
            def.number, vec
        );
        vec
    } else {
        info!(
            "Level {} using existing gravity {:?}",
            def.number, gravity_cfg.normal
        );
        gravity_cfg.normal
    };
    if let Ok(mut config) = rapier_config.single_mut() {
        config.gravity = target_gravity;
        info!("Rapier config gravity set to {:?}", config.gravity);
    }
    // Set initial positions (used by spawn below and later systems).
    set_spawn_points_only(def, spawn_points.as_mut());
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        unlit: false,
        ..default()
    });

    #[cfg(feature = "texture_manifest")]
    let canonical_handles = canonical.as_deref();

    let paddle_material = {
        #[cfg(feature = "texture_manifest")]
        {
            baseline_material_handle(
                canonical_handles,
                fallback.as_deref_mut(),
                BaselineMaterialKind::Paddle,
                "level_loader.handle_level_advance_delay.paddle",
            )
            .unwrap_or_else(|| debug_material.clone())
        }
        #[cfg(not(feature = "texture_manifest"))]
        {
            debug_material.clone()
        }
    };

    let ball_material = {
        #[cfg(feature = "texture_manifest")]
        {
            baseline_material_handle(
                canonical_handles,
                fallback.as_deref_mut(),
                BaselineMaterialKind::Ball,
                "level_loader.handle_level_advance_delay.ball",
            )
            .unwrap_or_else(|| debug_material.clone())
        }
        #[cfg(not(feature = "texture_manifest"))]
        {
            debug_material.clone()
        }
    };
    if let Some(paddle_pos) = spawn_points.paddle {
        commands
            .spawn((
                Mesh3d(meshes.add(Capsule3d::new(PADDLE_RADIUS, PADDLE_HEIGHT).mesh())),
                MeshMaterial3d(paddle_material.clone()),
                Transform::from_xyz(paddle_pos.x, paddle_pos.y, paddle_pos.z)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0))
                    .with_scale(Vec3::splat(0.01)),
                Paddle,
                crate::PaddleGrowing {
                    timer: Timer::from_seconds(crate::PADDLE_GROWTH_DURATION, TimerMode::Once),
                    target_scale: Vec3::ONE,
                },
                RigidBody::KinematicPositionBased,
                GravityScale(0.0),
                CollidingEntities::default(),
                Collider::capsule_y(PADDLE_HEIGHT / 2.0, PADDLE_RADIUS),
                LockedAxes::TRANSLATION_LOCKED_Y,
                KinematicCharacterController::default(),
                Ccd::enabled(),
            ))
            .insert(Friction {
                coefficient: 2.0,
                combine_rule: CoefficientCombineRule::Max,
            })
            .insert(paddle_respawn_handle(paddle_pos));
    }
    if let Some(ball_pos) = spawn_points.ball {
        info!(
            "Spawning ball during level advance at {:?} with BallFrozen marker",
            ball_pos
        );
        commands
            .spawn((
                Mesh3d(meshes.add(Sphere::new(BALL_RADIUS).mesh())),
                MeshMaterial3d(ball_material.clone()),
                Transform::from_xyz(ball_pos.x, ball_pos.y, ball_pos.z),
                Ball,
                crate::BallFrozen,
                RigidBody::Dynamic,
                CollidingEntities::default(),
                ActiveEvents::COLLISION_EVENTS,
                Collider::ball(BALL_RADIUS),
                Restitution {
                    coefficient: 0.9,
                    combine_rule: CoefficientCombineRule::Max,
                },
                Friction {
                    coefficient: 2.0,
                    combine_rule: CoefficientCombineRule::Max,
                },
                Damping {
                    linear_damping: 0.5,
                    angular_damping: 0.5,
                },
            ))
            .insert((
                LockedAxes::TRANSLATION_LOCKED_Y,
                Ccd::enabled(),
                ExternalImpulse::default(),
                GravityScale(0.0), // Keep at 0.0 until paddle growth completes
            ))
            .insert(ball_respawn_handle(ball_pos));
    }
    level_advance.growth_spawned = true;
}

/// After paddle growth ends, spawn bricks and finalize level change.
fn finalize_level_advance(
    paddles_growing: Query<&crate::PaddleGrowing>,
    mut level_advance: ResMut<LevelAdvanceState>,
    mut commands: Commands,
    mut rapier_config: Query<&mut RapierConfiguration>,
    gravity_cfg: Res<GravityConfig>,
    mut balls: Query<(Entity, &mut GravityScale, Option<&Velocity>), With<Ball>>,
) {
    let paddle_count = paddles_growing.iter().count();
    if !level_advance.active || !level_advance.growth_spawned || level_advance.pending.is_none() {
        return;
    }

    // If paddles are still growing, wait
    if paddle_count > 0 {
        info!(
            "finalize_level_advance: waiting for {} paddle(s) to finish growing",
            paddle_count
        );
        return;
    }

    // Check if we can actually see the spawned ball (commands have been applied)
    let ball_count = balls.iter().count();
    if ball_count == 0 {
        // Ball hasn't been spawned yet (commands not applied), wait for next frame
        info!("finalize_level_advance: waiting for ball to spawn (commands not yet applied)");
        return;
    }

    info!("finalize_level_advance: paddle growth complete, activating ball physics");
    let def = level_advance.pending.take().unwrap();
    // Bricks were spawned at peak of fade (in handle_level_advance_delay).
    // Now activate ball physics after paddle growth completes.
    info!(
        "Finalizing level advance: found {} ball(s), target gravity={:?}",
        ball_count, gravity_cfg.normal
    );
    for (entity, mut gravity_scale, velocity) in balls.iter_mut() {
        let vel_str = velocity
            .map(|v| format!("{:?}", v.linvel))
            .unwrap_or_else(|| "None".to_string());
        info!(
            "Removing BallFrozen from {:?}, current velocity={}",
            entity, vel_str
        );
        commands.entity(entity).remove::<crate::BallFrozen>();
        // Remove Velocity component so Rapier manages it internally (like R/L do)
        commands.entity(entity).remove::<Velocity>();
        gravity_scale.0 = 1.0; // Activate gravity
        info!(
            "Ball {:?} gravity activated: scale=1.0, world_gravity={:?}, removed Velocity component",
            entity, gravity_cfg.normal
        );
    }
    // Restore gravity to new level's normal.
    if let Ok(mut config) = rapier_config.single_mut() {
        config.gravity = gravity_cfg.normal;
        info!("Rapier gravity restored to: {:?}", config.gravity);
    }
    // Update CurrentLevel resource.
    commands.insert_resource(CurrentLevel(def));
    // Reset state.
    level_advance.active = false;
    level_advance.growth_spawned = false;
}

/// Spawn fade overlay once when level advancement begins.
fn spawn_fade_overlay_if_needed(
    level_advance: Res<LevelAdvanceState>,
    existing: Query<Entity, With<FadeOverlay>>,
    mut commands: Commands,
) {
    if !level_advance.active || level_advance.pending.is_none() || !existing.is_empty() {
        return;
    }
    // Spawn a full-screen black Node with 0 alpha; animated by update_fade_overlay.
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        FadeOverlay,
    ));
}

/// Animate fade overlay alpha based on delay (fade in) and paddle growth (fade out).
fn update_fade_overlay(
    level_advance: Res<LevelAdvanceState>,
    mut overlays: Query<(Entity, &mut BackgroundColor), With<FadeOverlay>>,
    paddles_growing: Query<&crate::PaddleGrowing>,
    mut commands: Commands,
) {
    if let Ok((entity, mut color)) = overlays.single_mut() {
        if !level_advance.active {
            commands.entity(entity).despawn();
            return;
        }
        if !level_advance.growth_spawned {
            let alpha: f32 = level_advance.timer.fraction();
            let a = alpha.clamp(0.0, 1.0);
            color.0 = Color::srgba(0.0, 0.0, 0.0, a);
        } else if let Ok(growing) = paddles_growing.single() {
            let progress: f32 = growing.timer.fraction();
            let alpha: f32 = 1.0 - progress;
            let a = alpha.clamp(0.0, 1.0);
            color.0 = Color::srgba(0.0, 0.0, 0.0, a);
        } else {
            // Growth finished; remove overlay.
            commands.entity(entity).despawn();
        }
    }
}

fn force_load_level_from_path(
    path: &str,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    spawn_points: &mut ResMut<SpawnPoints>,
    gravity_cfg: &mut ResMut<GravityConfig>,
    rapier_config: &mut Query<&mut RapierConfiguration>,
    bricks: &Query<Entity, With<Brick>>,
    paddle_q: &Query<Entity, With<Paddle>>,
    ball_q: &Query<Entity, With<Ball>>,
    game_progress: &mut ResMut<GameProgress>,
    level_advance: &mut ResMut<LevelAdvanceState>,
    #[cfg(feature = "texture_manifest")] canonical: Option<&CanonicalMaterialHandles>,
    #[cfg(feature = "texture_manifest")] fallback: Option<&mut FallbackRegistry>,
    #[cfg(feature = "texture_manifest")] type_registry: Option<&TypeVariantRegistry>,
) -> Result<LevelDefinition, String> {
    reset_level_state(
        commands,
        bricks,
        paddle_q,
        ball_q,
        spawn_points,
        game_progress,
        level_advance,
    );
    #[cfg(not(target_arch = "wasm32"))]
    let content = std::fs::read_to_string(path)
        .map_err(|err| format!("failed to read level file '{path}': {err}"))?;
    #[cfg(target_arch = "wasm32")]
    let content = embedded_level_str(path)
        .ok_or_else(|| format!("failed to read level file '{path}': embedded asset missing"))?;
    let def = from_str::<LevelDefinition>(&content)
        .map_err(|err| format!("failed to parse level '{path}': {err}"))?;
    apply_level_definition(
        &def,
        commands,
        meshes,
        materials,
        spawn_points,
        gravity_cfg,
        rapier_config,
        #[cfg(feature = "texture_manifest")]
        canonical,
        #[cfg(feature = "texture_manifest")]
        fallback,
        #[cfg(feature = "texture_manifest")]
        type_registry,
    );
    commands.insert_resource(CurrentLevel(def.clone()));
    Ok(def)
}

// Embedded level RON contents for WASM builds (no filesystem access).
#[cfg(target_arch = "wasm32")]
fn embedded_level_str(path: &str) -> Option<&'static str> {
    match path {
        "assets/levels/level_001.ron" => Some(include_str!("../assets/levels/level_001.ron")),
        "assets/levels/level_002.ron" => Some(include_str!("../assets/levels/level_002.ron")),
        _ => None,
    }
}

fn reset_level_state(
    commands: &mut Commands,
    bricks: &Query<Entity, With<Brick>>,
    paddle_q: &Query<Entity, With<Paddle>>,
    ball_q: &Query<Entity, With<Ball>>,
    spawn_points: &mut ResMut<SpawnPoints>,
    game_progress: &mut ResMut<GameProgress>,
    level_advance: &mut ResMut<LevelAdvanceState>,
) {
    for entity in bricks.iter() {
        commands.entity(entity).despawn();
    }
    for entity in paddle_q.iter() {
        commands.entity(entity).despawn();
    }
    for entity in ball_q.iter() {
        commands.entity(entity).despawn();
    }
    spawn_points.paddle = None;
    spawn_points.ball = None;
    game_progress.finished = false;
    level_advance.active = false;
    level_advance.pending = None;
    level_advance.growth_spawned = false;
}

fn apply_level_definition(
    def: &LevelDefinition,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    spawn_points: &mut ResMut<SpawnPoints>,
    gravity_cfg: &mut ResMut<GravityConfig>,
    rapier_config: &mut Query<&mut RapierConfiguration>,
    #[cfg(feature = "texture_manifest")] canonical: Option<&CanonicalMaterialHandles>,
    #[cfg(feature = "texture_manifest")] fallback: Option<&mut FallbackRegistry>,
    #[cfg(feature = "texture_manifest")] type_registry: Option<&TypeVariantRegistry>,
) {
    if let Some((x, y, z)) = def.gravity {
        gravity_cfg.normal = Vec3::new(x, y, z);
        if let Ok(mut config) = rapier_config.single_mut() {
            config.gravity = gravity_cfg.normal;
        }
        info!("Level gravity set to {:?}", gravity_cfg.normal);
    } else if let Ok(mut config) = rapier_config.single_mut() {
        config.gravity = gravity_cfg.normal;
    }
    spawn_level_entities_impl(
        def,
        commands,
        meshes,
        materials,
        spawn_points,
        #[cfg(feature = "texture_manifest")]
        canonical,
        #[cfg(feature = "texture_manifest")]
        fallback,
        #[cfg(feature = "texture_manifest")]
        type_registry,
    );
}

/// Sync `LevelPresentation` resource whenever `CurrentLevel` changes.
///
/// Looks up the current level number in the texture manifest's `level_overrides`
/// and updates `LevelPresentation` with any per-level texture overrides.
#[cfg(feature = "texture_manifest")]
fn sync_level_presentation(
    current_level: Option<Res<CurrentLevel>>,
    manifest: Option<Res<TextureManifest>>,
    presentation: Option<ResMut<LevelPresentation>>,
) {
    let Some(level) = current_level else {
        return;
    };
    let Some(mut presentation) = presentation else {
        // LevelPresentation not available (e.g., no TextureManifestPlugin)
        return;
    };
    // Only update when CurrentLevel changes
    if !level.is_changed() {
        return;
    }
    let level_number = level.0.number;
    if let Some(manifest) = manifest {
        presentation.update_from_level_and_manifest(&level.0, &manifest);
        debug!(
            target: "level_loader::presentation",
            level = level_number,
            ground = ?presentation.ground_profile(),
            background = ?presentation.background_profile(),
            sidewall = ?presentation.sidewall_profile(),
            "Updated level presentation"
        );
    } else {
        // No manifest loaded yet; reset to defaults
        presentation.reset();
    }
}
