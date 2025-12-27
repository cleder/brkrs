use crate::level_format::{normalize_matrix_simple, INDESTRUCTIBLE_BRICK};
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
    Ball, BallTypeId, Brick, BrickTypeId, CountsTowardsCompletion, GameProgress, GravityConfig,
    LowerGoal, Paddle, BALL_RADIUS, CELL_HEIGHT, CELL_WIDTH, PADDLE_HEIGHT, PADDLE_RADIUS, PLANE_H,
    PLANE_W,
};
use bevy_rapier3d::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelAdvanceSystems;

/// Deprecated compatibility alias.
///
/// Previously this project exported `LevelAdvanceSet`. It was renamed to
/// `LevelAdvanceSystems` for clarity; provide a deprecated type alias so
/// external code (or older branches) can still compile while maintainers
/// migrate callers.
#[deprecated(note = "Use LevelAdvanceSystems instead")]
pub type LevelAdvanceSet = LevelAdvanceSystems;

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
    /// Optional description documenting the level's design intent, unique features, or gameplay characteristics.
    /// This is for documentation purposes only and is not displayed during gameplay.
    #[serde(default)]
    pub description: Option<String>,
    /// Optional author field for contributor attribution.
    /// Supports plain text names (e.g., "Jane Smith") or markdown links (e.g., "[Jane Smith](mailto:jane@example.com)").
    /// When using markdown format, only the display name is extracted.
    #[serde(default)]
    pub author: Option<String>,
}

#[derive(Resource, Debug)]
pub struct CurrentLevel(pub LevelDefinition);

/// Extract display name from author field (handles plain text and markdown link formats)
///
/// Converts:
/// - Plain text: "Jane Smith" → "Jane Smith"
/// - Markdown link: "[Jane Smith](mailto:jane@example.com)" → "Jane Smith"
/// - Markdown link: "[Team](https://github.com/team)" → "Team"
///
/// If the author string doesn't match the markdown pattern, it's returned as-is.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(extract_author_name("Jane Smith"), "Jane Smith");
/// assert_eq!(extract_author_name("[Jane Smith](mailto:jane@example.com)"), "Jane Smith");
/// ```
pub fn extract_author_name(author: &str) -> &str {
    let trimmed = author.trim();
    if trimmed.starts_with('[') {
        if let Some(end_bracket) = trimmed.find("](") {
            return trimmed[1..end_bracket].trim();
        }
    }
    trimmed
}

impl LevelDefinition {
    /// Returns true if the level has a non-empty description
    ///
    /// Returns `false` if the description field is `None` or contains only whitespace.
    pub fn has_description(&self) -> bool {
        self.description
            .as_ref()
            .is_some_and(|s| !s.trim().is_empty())
    }

    /// Returns true if the level has a non-empty author
    ///
    /// Returns `false` if the author field is `None` or contains only whitespace.
    pub fn has_author(&self) -> bool {
        self.author.as_ref().is_some_and(|s| !s.trim().is_empty())
    }

    /// Get the author display name, extracting from markdown format if needed
    ///
    /// Returns `None` if author field is empty or absent.
    /// Automatically extracts the name from markdown links like `[Name](url)`.
    pub fn author_name(&self) -> Option<&str> {
        self.author
            .as_ref()
            .filter(|s| !s.trim().is_empty())
            .map(|s| extract_author_name(s))
    }
}

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GravityConfig>();
        app.add_message::<RestartRequested>();
        app.add_systems(Startup, (load_level, spawn_level_entities).chain());
        #[cfg(feature = "texture_manifest")]
        {
            use crate::systems::sets::LevelFadeInStartSystems;
            app.add_systems(
                Update,
                (
                    advance_level_when_cleared,
                    handle_level_advance_delay,
                    // Insert LevelFadeInStartSet after handle_level_advance_delay
                    finalize_level_advance.after(handle_level_advance_delay),
                    spawn_fade_overlay_if_needed,
                )
                    .in_set(LevelAdvanceSystems),
            );
            app.configure_sets(
                Update,
                LevelFadeInStartSystems.after(handle_level_advance_delay),
            );

            app.add_systems(
                Update,
                (
                    update_fade_overlay,
                    destroy_all_bricks_on_key,
                    process_level_switch_requests,
                ),
            );
            // Run sync_level_presentation in PreUpdate, before apply_level_overrides
            app.add_systems(
                PreUpdate,
                sync_level_presentation.in_set(SyncLevelPresentationSystems),
            );

            // Use shared system set for ordering background/material override after sync_level_presentation
            use crate::systems::sets::SyncLevelPresentationSystems;
            // Register restart queue and processor
            // Run the restart producer in PreUpdate so it observes just_pressed reliably
            app.add_systems(PreUpdate, queue_restart_requests);
            // Keep the heavy restart processing in Update
            app.add_systems(Update, process_restart_requests);
        }
        #[cfg(not(feature = "texture_manifest"))]
        {
            app.add_systems(
                Update,
                (
                    advance_level_when_cleared,
                    handle_level_advance_delay,
                    finalize_level_advance.after(handle_level_advance_delay),
                    spawn_fade_overlay_if_needed,
                )
                    .in_set(LevelAdvanceSystems),
            );

            app.add_systems(
                Update,
                (
                    update_fade_overlay,
                    destroy_all_bricks_on_key,
                    process_level_switch_requests,
                ),
            );
            // Run the restart producer in PreUpdate so it observes just_pressed reliably
            app.add_systems(PreUpdate, queue_restart_requests);
            // Keep the heavy restart processing in Update
            app.add_systems(Update, process_restart_requests);
        }
    }
}

/// State machine for delayed level advancement and growth animation.
#[derive(Resource)]
pub struct LevelAdvanceState {
    pub timer: Timer,                     // initial delay before spawning growth paddle
    pub active: bool,                     // transition in progress
    pub growth_spawned: bool,             // tiny paddle+ball spawned, waiting for growth completion
    pub pending: Option<LevelDefinition>, // next level definition awaiting brick spawn
    pub unfreezing: bool, // BallFrozen removed, waiting one frame for Velocity cleanup
}

impl Default for LevelAdvanceState {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            active: false,
            growth_spawned: false,
            pending: None,
            unfreezing: false,
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
        // Allow tests to provide an explicit path via BK_LEVEL_PATH. If set, use it
        // directly (this helps tests avoid writing to repo assets/levels/).
        if let Ok(path) = env::var("BK_LEVEL_PATH") {
            path
        } else if let Ok(num) = env::var("BK_LEVEL") {
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
            def.matrix = normalize_matrix_simple(def.matrix);
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
    brick_config_res: Res<crate::physics_config::BrickPhysicsConfig>,
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
        brick_config_res,
    );

    // Emit LevelStarted event for audio system
    commands.trigger(crate::systems::LevelStarted {
        level_index: level.0.number,
    });
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
    brick_config_res: Res<crate::physics_config::BrickPhysicsConfig>,
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
            let z = PLANE_W / 2.0 - (col as f32 + 0.5) * CELL_WIDTH;
            match value {
                0 => {}
                2 => {
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
                1 => {
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
                    let brick_config = &*brick_config_res;
                    if let Err(err) = brick_config.validate() {
                        bevy::log::error!("Invalid BrickPhysicsConfig during brick spawn: {}", err);
                    }
                    let mut entity = commands.spawn((
                        Mesh3d(meshes.add(Cuboid::new(CELL_HEIGHT * 0.9, 0.5, CELL_WIDTH * 0.9))),
                        MeshMaterial3d(brick_mat),
                        Transform::from_xyz(x, 2.0, z),
                        Brick,
                        BrickTypeId(brick_type_id),
                        RigidBody::Fixed,
                        Collider::cuboid(CELL_HEIGHT * 0.9 / 2.0, 0.25, CELL_WIDTH * 0.9 / 2.0),
                        Restitution {
                            coefficient: brick_config.restitution,
                            combine_rule: CoefficientCombineRule::Max,
                        },
                        Friction {
                            coefficient: brick_config.friction,
                            combine_rule: CoefficientCombineRule::Max,
                        },
                        CollidingEntities::default(),
                        ActiveEvents::COLLISION_EVENTS,
                    ));
                    if brick_type_id != INDESTRUCTIBLE_BRICK {
                        entity.insert(CountsTowardsCompletion);
                    }
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
            let z = PLANE_W / 2.0 - (col as f32 + 0.5) * CELL_WIDTH;
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

            let mut entity = commands.spawn((
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
            if brick_type_id != INDESTRUCTIBLE_BRICK {
                entity.insert(crate::CountsTowardsCompletion);
            }
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
            let z = PLANE_W / 2.0 - (col as f32 + 0.5) * CELL_WIDTH;
            match value {
                2 if !paddle_set => {
                    paddle_set = true;
                    spawn_points.paddle = Some(Vec3::new(x, 2.0, z));
                }
                1 if !ball_set => {
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
    destructible_bricks: Query<Entity, (With<Brick>, With<crate::CountsTowardsCompletion>)>,
    bricks: Query<Entity, With<Brick>>,
    paddle_q: Query<Entity, With<Paddle>>,
    ball_q: Query<Entity, With<Ball>>,
    current_level: Option<Res<CurrentLevel>>,
    mut commands: Commands,
    mut game_progress: ResMut<GameProgress>,
    mut level_advance: ResMut<LevelAdvanceState>,
    ui_fonts: Option<Res<crate::ui::fonts::UiFonts>>,
) {
    let Some(curr) = current_level else {
        return;
    };
    if !destructible_bricks.is_empty() {
        return; // still destructible bricks remaining
    }
    // If already transitioning, don't restart it.
    if level_advance.active {
        return;
    }

    // Emit LevelCompleted event for audio system
    commands.trigger(crate::systems::LevelCompleted {
        level_index: curr.0.number,
    });

    // Despawn all bricks before fade-out and next level setup
    for entity in bricks.iter() {
        commands.entity(entity).despawn();
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
            if let Some(ui_fonts) = ui_fonts {
                let font = ui_fonts.orbitron.clone();
                commands.spawn((
                    Text::new("GAME COMPLETE - Press Q to Quit"),
                    TextFont { font, ..default() },
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(60.0),
                        left: Val::Px(60.0),
                        ..default()
                    },
                ));
            }
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
// Restart messaging: small producer + heavy consumer split so registration is reliable

#[derive(Message, Debug, Clone, Copy)]
pub struct RestartRequested;

/// Producer: queue restart requests when 'R' is pressed; emits UiBeep when blocked.
fn queue_restart_requests(
    keyboard: Res<ButtonInput<KeyCode>>,
    cheat: Option<Res<crate::systems::cheat_mode::CheatModeState>>,
    mut restart: Option<MessageWriter<RestartRequested>>,
    mut beep: Option<MessageWriter<crate::signals::UiBeep>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }
    if let Some(cheat) = cheat.as_ref() {
        if cheat.is_active() {
            if let Some(r) = restart.as_mut() {
                r.write(RestartRequested);
            }
        } else if let Some(b) = beep.as_mut() {
            b.write(crate::signals::UiBeep);
        }
    } else {
        // conservative: block if cheat state not present
        if let Some(b) = beep.as_mut() {
            b.write(crate::signals::UiBeep);
        }
    }
}

/// Consumer: handle restart requests and perform the heavy restart operation.
fn process_restart_requests(
    mut requests: bevy::ecs::message::MessageReader<RestartRequested>,
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
    lives_state: Option<ResMut<crate::systems::respawn::LivesState>>,
    #[cfg(feature = "texture_manifest")] mut tex_res: TextureResources,
    brick_config_res: Res<crate::physics_config::BrickPhysicsConfig>,
) {
    if requests.is_empty() {
        return;
    }

    // Only process the first request per frame
    let Some(_) = requests.read().next() else {
        requests.clear();
        return;
    };

    // Reset lives to 3 when restarting level
    if let Some(mut lives_state) = lives_state {
        lives_state.lives_remaining = 3;
    } else {
        warn!("LivesState resource missing during level restart; skipping lives reset");
    }

    let level_number = current_level.map(|cl| cl.0.number).unwrap_or(1);
    let path = format!("assets/levels/level_{:03}.ron", level_number);
    // Despawn all bricks before restarting the level
    for entity in bricks.iter() {
        commands.entity(entity).despawn();
    }
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
        brick_config_res,
    ) {
        Ok(_) => info!("Restarted level {level_number}"),
        Err(err) => warn!("Failed to restart level {level_number}: {err}"),
    }
    requests.clear();
}

/// Destroy all bricks when K is pressed (for testing level transitions).
fn destroy_all_bricks_on_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    cheat: Option<Res<crate::systems::cheat_mode::CheatModeState>>,
    // Only destroy bricks that count towards completion. Indestructible bricks (no
    // CountsTowardsCompletion) should remain in the scene even when testing with K.
    bricks: Query<Entity, (With<Brick>, With<crate::CountsTowardsCompletion>)>,
    mut commands: Commands,
) {
    // Simple key press - just K to destroy all bricks for testing
    // Input state is consumed below; nothing to log in production.

    // Accept either a single-frame `just_pressed` or continuous `pressed` state so
    // test harnesses and interactive play both trigger the test-delete behaviour.
    if keyboard.just_pressed(KeyCode::KeyK) || keyboard.pressed(KeyCode::KeyK) {
        // Check cheat mode
        if let Some(cheat) = cheat {
            if !cheat.is_active() {
                return;
            }
        } else {
            // If cheat resource is missing, default to blocked
            return;
        }

        // KeyK detected — destroy destructible bricks for testing
        for entity in bricks.iter() {
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn process_level_switch_requests(
    mut requests: bevy::ecs::message::MessageReader<LevelSwitchRequested>,
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
    brick_config_res: Res<crate::physics_config::BrickPhysicsConfig>,
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
    let Some(request) = requests.read().next() else {
        requests.clear();
        return;
    };

    let maybe_slot = match request.direction {
        crate::systems::level_switch::LevelSwitchDirection::Next => {
            switch_state.next_level_after(current_number).cloned()
        }
        crate::systems::level_switch::LevelSwitchDirection::Previous => {
            switch_state.previous_level_before(current_number).cloned()
        }
    };

    let Some(target_slot) = maybe_slot else {
        warn!(target: "level_switch", "No level entries available for switching");
        requests.clear();
        return;
    };
    switch_state.mark_transition_start();
    // Despawn all bricks before loading the new level
    for entity in bricks.iter() {
        commands.entity(entity).despawn();
    }
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
        brick_config_res,
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
    if !level_advance.timer.is_finished() {
        return;
    }
    let Some(def) = level_advance.pending.as_ref() else {
        return;
    };

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
                    start_scale: Vec3::splat(0.01),
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
    // At the end of fade-out, before fade-in, set CurrentLevel to the new level
    if let Some(def) = level_advance.pending.as_ref() {
        commands.insert_resource(CurrentLevel(def.clone()));
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
    if !level_advance.active
        || !level_advance.growth_spawned
        || level_advance.pending.is_none()
        // If paddles are still growing, wait
        || !paddles_growing.is_empty()
    {
        return;
    }

    // Check if we can actually see the spawned ball (commands have been applied)
    let ball_count = balls.iter().count();
    if ball_count == 0 {
        // Ball hasn't been spawned yet (commands not applied), wait for next frame
        return;
    }

    // Two-stage unfreezing process:
    // Stage 1: Remove BallFrozen, wait one frame for stabilize_frozen_balls to stop acting
    // Stage 2: Remove Velocity component and activate gravity

    if !level_advance.unfreezing {
        // Stage 1: Remove BallFrozen marker
        for (entity, _gravity_scale, _velocity) in balls.iter_mut() {
            commands.entity(entity).remove::<crate::BallFrozen>();
        }
        level_advance.unfreezing = true;
        return; // Wait one frame for BallFrozen removal to take effect
    }

    // Stage 2: Now BallFrozen is removed, stabilize_frozen_balls won't act anymore
    let _ = level_advance.pending.take();

    for (entity, mut gravity_scale, _velocity) in balls.iter_mut() {
        // Remove Velocity component so Rapier manages it internally (like R/L do)
        commands.entity(entity).remove::<Velocity>();
        // Give tiny impulse to wake up Rapier's internal physics state
        // (Rapier stored zero velocity internally while we were freezing the ball)
        commands.entity(entity).insert(ExternalImpulse {
            impulse: Vec3::new(0.0001, 0.0, 0.0001),
            torque_impulse: Vec3::ZERO,
        });
        gravity_scale.0 = 1.0; // Activate gravity
    }
    // Restore gravity to new level's normal.
    if let Ok(mut config) = rapier_config.single_mut() {
        config.gravity = gravity_cfg.normal;
    }
    // Reset state.
    level_advance.active = false;
    level_advance.growth_spawned = false;
    level_advance.unfreezing = false;
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
    brick_config_res: Res<crate::physics_config::BrickPhysicsConfig>,
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
        brick_config_res,
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
        "assets/levels/level_003.ron" => Some(include_str!("../assets/levels/level_003.ron")),
        "assets/levels/level_004.ron" => Some(include_str!("../assets/levels/level_004.ron")),
        "assets/levels/level_005.ron" => Some(include_str!("../assets/levels/level_005.ron")),
        "assets/levels/level_006.ron" => Some(include_str!("../assets/levels/level_006.ron")),
        "assets/levels/level_007.ron" => Some(include_str!("../assets/levels/level_007.ron")),
        "assets/levels/level_008.ron" => Some(include_str!("../assets/levels/level_008.ron")),
        "assets/levels/level_009.ron" => Some(include_str!("../assets/levels/level_009.ron")),
        "assets/levels/level_010.ron" => Some(include_str!("../assets/levels/level_010.ron")),
        "assets/levels/level_011.ron" => Some(include_str!("../assets/levels/level_011.ron")),
        "assets/levels/level_012.ron" => Some(include_str!("../assets/levels/level_012.ron")),
        "assets/levels/level_013.ron" => Some(include_str!("../assets/levels/level_013.ron")),
        "assets/levels/level_014.ron" => Some(include_str!("../assets/levels/level_014.ron")),
        "assets/levels/level_015.ron" => Some(include_str!("../assets/levels/level_015.ron")),
        "assets/levels/level_016.ron" => Some(include_str!("../assets/levels/level_016.ron")),
        "assets/levels/level_017.ron" => Some(include_str!("../assets/levels/level_017.ron")),
        "assets/levels/level_018.ron" => Some(include_str!("../assets/levels/level_018.ron")),
        "assets/levels/level_019.ron" => Some(include_str!("../assets/levels/level_019.ron")),
        "assets/levels/level_020.ron" => Some(include_str!("../assets/levels/level_020.ron")),
        "assets/levels/level_021.ron" => Some(include_str!("../assets/levels/level_021.ron")),
        "assets/levels/level_022.ron" => Some(include_str!("../assets/levels/level_022.ron")),
        "assets/levels/level_023.ron" => Some(include_str!("../assets/levels/level_023.ron")),
        "assets/levels/level_024.ron" => Some(include_str!("../assets/levels/level_024.ron")),
        "assets/levels/level_025.ron" => Some(include_str!("../assets/levels/level_025.ron")),
        "assets/levels/level_026.ron" => Some(include_str!("../assets/levels/level_026.ron")),
        "assets/levels/level_027.ron" => Some(include_str!("../assets/levels/level_027.ron")),
        "assets/levels/level_028.ron" => Some(include_str!("../assets/levels/level_028.ron")),
        "assets/levels/level_029.ron" => Some(include_str!("../assets/levels/level_029.ron")),
        "assets/levels/level_030.ron" => Some(include_str!("../assets/levels/level_030.ron")),
        "assets/levels/level_031.ron" => Some(include_str!("../assets/levels/level_031.ron")),
        "assets/levels/level_032.ron" => Some(include_str!("../assets/levels/level_032.ron")),
        "assets/levels/level_033.ron" => Some(include_str!("../assets/levels/level_033.ron")),
        "assets/levels/level_034.ron" => Some(include_str!("../assets/levels/level_034.ron")),
        "assets/levels/level_035.ron" => Some(include_str!("../assets/levels/level_035.ron")),
        "assets/levels/level_036.ron" => Some(include_str!("../assets/levels/level_036.ron")),
        "assets/levels/level_037.ron" => Some(include_str!("../assets/levels/level_037.ron")),
        "assets/levels/level_038.ron" => Some(include_str!("../assets/levels/level_038.ron")),
        "assets/levels/level_039.ron" => Some(include_str!("../assets/levels/level_039.ron")),
        "assets/levels/level_040.ron" => Some(include_str!("../assets/levels/level_040.ron")),
        "assets/levels/level_041.ron" => Some(include_str!("../assets/levels/level_041.ron")),
        "assets/levels/level_042.ron" => Some(include_str!("../assets/levels/level_042.ron")),
        "assets/levels/level_043.ron" => Some(include_str!("../assets/levels/level_043.ron")),
        "assets/levels/level_044.ron" => Some(include_str!("../assets/levels/level_044.ron")),
        "assets/levels/level_045.ron" => Some(include_str!("../assets/levels/level_045.ron")),
        "assets/levels/level_046.ron" => Some(include_str!("../assets/levels/level_046.ron")),
        "assets/levels/level_047.ron" => Some(include_str!("../assets/levels/level_047.ron")),
        "assets/levels/level_048.ron" => Some(include_str!("../assets/levels/level_048.ron")),
        "assets/levels/level_049.ron" => Some(include_str!("../assets/levels/level_049.ron")),
        "assets/levels/level_050.ron" => Some(include_str!("../assets/levels/level_050.ron")),
        "assets/levels/level_051.ron" => Some(include_str!("../assets/levels/level_051.ron")),
        "assets/levels/level_052.ron" => Some(include_str!("../assets/levels/level_052.ron")),
        "assets/levels/level_053.ron" => Some(include_str!("../assets/levels/level_053.ron")),
        "assets/levels/level_054.ron" => Some(include_str!("../assets/levels/level_054.ron")),
        "assets/levels/level_055.ron" => Some(include_str!("../assets/levels/level_055.ron")),
        "assets/levels/level_056.ron" => Some(include_str!("../assets/levels/level_056.ron")),
        "assets/levels/level_057.ron" => Some(include_str!("../assets/levels/level_057.ron")),
        "assets/levels/level_058.ron" => Some(include_str!("../assets/levels/level_058.ron")),
        "assets/levels/level_059.ron" => Some(include_str!("../assets/levels/level_059.ron")),
        "assets/levels/level_060.ron" => Some(include_str!("../assets/levels/level_060.ron")),
        "assets/levels/level_061.ron" => Some(include_str!("../assets/levels/level_061.ron")),
        "assets/levels/level_062.ron" => Some(include_str!("../assets/levels/level_062.ron")),
        "assets/levels/level_063.ron" => Some(include_str!("../assets/levels/level_063.ron")),
        "assets/levels/level_064.ron" => Some(include_str!("../assets/levels/level_064.ron")),
        "assets/levels/level_065.ron" => Some(include_str!("../assets/levels/level_065.ron")),
        "assets/levels/level_066.ron" => Some(include_str!("../assets/levels/level_066.ron")),
        "assets/levels/level_067.ron" => Some(include_str!("../assets/levels/level_067.ron")),
        "assets/levels/level_068.ron" => Some(include_str!("../assets/levels/level_068.ron")),
        "assets/levels/level_069.ron" => Some(include_str!("../assets/levels/level_069.ron")),
        "assets/levels/level_070.ron" => Some(include_str!("../assets/levels/level_070.ron")),
        "assets/levels/level_071.ron" => Some(include_str!("../assets/levels/level_071.ron")),
        "assets/levels/level_072.ron" => Some(include_str!("../assets/levels/level_072.ron")),
        "assets/levels/level_073.ron" => Some(include_str!("../assets/levels/level_073.ron")),
        "assets/levels/level_074.ron" => Some(include_str!("../assets/levels/level_074.ron")),
        "assets/levels/level_997.ron" => Some(include_str!("../assets/levels/level_997.ron")),
        "assets/levels/level_998.ron" => Some(include_str!("../assets/levels/level_998.ron")),
        "assets/levels/level_999.ron" => Some(include_str!("../assets/levels/level_999.ron")),
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
    brick_config_res: Res<crate::physics_config::BrickPhysicsConfig>,
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
        brick_config_res,
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

#[cfg(test)]
mod tests {
    use crate::level_format::normalize_matrix_simple as normalize_matrix;

    #[test]
    fn normalize_padding_rows_and_cols() {
        // 18 rows, each 19 cols -> should pad to 20x20
        let input = vec![vec![1u8; 19]; 18];
        let out = normalize_matrix(input.clone());
        assert_eq!(out.len(), 20, "row count padded to 20");
        for row in &out {
            assert_eq!(row.len(), 20, "col count padded to 20");
        }
        // Original data preserved in leading rows/cols
        for (r, row) in out.iter().enumerate().take(18) {
            for (c, &val) in row.iter().enumerate().take(19) {
                assert_eq!(val, 1, "row {r} col {c} should preserve value 1");
            }
        }
        // Padded cells zeroed
        for (r, row) in out.iter().enumerate().take(18) {
            assert_eq!(row[19], 0, "row {r} col 19 should be padded zero");
        }
        for (r, row) in out.iter().enumerate().skip(18).take(2) {
            for (c, &val) in row.iter().enumerate().take(20) {
                assert_eq!(val, 0, "row {r} col {c} should be padded zero");
            }
        }
    }

    #[test]
    fn normalize_truncates_rows_and_cols() {
        // 22 rows of 24 cols -> truncates to first 20 rows/cols
        let input = vec![vec![2u8; 24]; 22];
        let out = normalize_matrix(input.clone());
        assert_eq!(out.len(), 20);
        for row in &out {
            assert_eq!(row.len(), 20);
        }
        // Leading preserved
        for (r, row) in out.iter().enumerate().take(20) {
            for (c, &val) in row.iter().enumerate().take(20) {
                assert_eq!(val, 2, "row {r} col {c} should preserve value 2");
            }
        }
    }

    #[test]
    fn normalize_irregular_row_lengths() {
        // Mixture: some short, some long
        let mut input: Vec<Vec<u8>> = Vec::new();
        for i in 0..22 {
            // exceed target rows to test truncation
            let len = match i % 3 {
                0 => 10,
                1 => 25,
                _ => 20,
            }; // various lengths
            input.push(vec![3u8; len]);
        }
        let out = normalize_matrix(input);
        assert_eq!(out.len(), 20);
        for (r, row) in out.iter().enumerate() {
            assert_eq!(row.len(), 20, "row {} not normalized to 20 cols", r);
            let original_len = match r % 3 {
                0 => 10,
                1 => 25,
                _ => 20,
            };
            let preserved = original_len.min(20);
            for (c, &val) in row.iter().enumerate().take(preserved) {
                assert_eq!(val, 3, "row {r} col {c} should preserve value 3");
            }
            for (c, &val) in row.iter().enumerate().skip(preserved).take(20 - preserved) {
                assert_eq!(val, 0, "row {r} col {c} should be padded zero");
            }
        }
    }

    #[test]
    fn normalize_empty_matrix() {
        let out = normalize_matrix(Vec::new());
        assert_eq!(out.len(), 20);
        for row in &out {
            assert_eq!(row.len(), 20);
            for c in row {
                assert_eq!(*c, 0);
            }
        }
    }

    #[test]
    fn normalize_exact_dimensions_unchanged() {
        let mut input = vec![vec![5u8; 20]; 20];
        input[0][0] = 7;
        let out = normalize_matrix(input.clone());
        assert_eq!(out.len(), 20);
        for row in &out {
            assert_eq!(row.len(), 20);
        }
        assert_eq!(out[0][0], 7);
        // Ensure no unintended zeroing
        for (r, row) in out.iter().enumerate().take(20) {
            for (c, &val) in row.iter().enumerate().take(20) {
                assert_eq!(val, input[r][c]);
            }
        }
    }

    // Unit tests for restart gating

    use super::*;

    #[derive(Resource, Default)]
    struct BeepCount(u32);

    fn capture_beep(
        mut reader: bevy::ecs::message::MessageReader<crate::signals::UiBeep>,
        mut c: ResMut<BeepCount>,
    ) {
        for _ in reader.read() {
            c.0 += 1;
        }
    }

    fn app_with_plugins() -> App {
        let mut app = App::new();
        app.insert_resource(crate::physics_config::BallPhysicsConfig::default());
        app.insert_resource(crate::physics_config::PaddlePhysicsConfig::default());
        app.insert_resource(crate::physics_config::BrickPhysicsConfig::default());
        app.add_plugins((MinimalPlugins, bevy::input::InputPlugin));
        app.add_plugins(crate::systems::audio::AudioPlugin);
        // Register level switch plugin so LevelSwitchRequested message is initialized for processing
        app.add_plugins(crate::systems::level_switch::LevelSwitchPlugin);
        app.add_plugins(LevelLoaderPlugin);
        // Minimal resources required by level loader systems (spawn_level_entities, restarts, etc.)
        app.insert_resource(Assets::<Mesh>::default());
        app.insert_resource(Assets::<StandardMaterial>::default());
        app.insert_resource(GameProgress::default());
        app.insert_resource(LevelAdvanceState::default());
        app.insert_resource(SpawnPoints::default());
        // Capture beeps
        app.init_resource::<BeepCount>();
        // Pause state required by run conditions
        app.init_resource::<crate::pause::PauseState>();
        // Scoring state required by cheat-mode toggle
        app.init_resource::<crate::systems::scoring::ScoreState>();
        // capture after restart system to ensure we observe writes
        app.add_systems(Update, capture_beep);
        app
    }

    #[test]
    fn restart_blocks_when_cheat_inactive_emits_beep() {
        let mut app = app_with_plugins();
        app.add_plugins(crate::systems::cheat_mode::CheatModePlugin);
        // ensure cheat off
        {
            let mut cheat = app
                .world_mut()
                .resource_mut::<crate::systems::cheat_mode::CheatModeState>();
            cheat.active = false;
        }
        // press R
        {
            let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            input.press(KeyCode::KeyR);
        }
        app.update();
        app.update();
        // capture runs on second frame and should observe the beep
        let beep = app.world().resource::<BeepCount>();
        assert!(beep.0 >= 1, "Blocked restart should emit a UI beep");
    }

    #[test]
    fn restart_allowed_when_cheat_active_no_beep() {
        let mut app = app_with_plugins();
        app.add_plugins(crate::systems::cheat_mode::CheatModePlugin);
        // ensure cheat on
        {
            let mut cheat = app
                .world_mut()
                .resource_mut::<crate::systems::cheat_mode::CheatModeState>();
            cheat.active = true;
        }
        // press R
        {
            let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            input.press(KeyCode::KeyR);
        }
        app.update();
        app.update();
        let beep = app.world().resource::<BeepCount>();
        assert_eq!(beep.0, 0, "Allowed restart should not emit a UI beep");
    }
}
