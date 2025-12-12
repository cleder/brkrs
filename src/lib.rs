//!
//! You can toggle wireframes with the space bar except on wasm. Wasm does not support
//! `POLYGON_MODE_LINE` on the gpu.
//!
//! Keyboard commands:
//! - R: Restart current level
//! - L: Switch to next level
//! - K: Destroy all bricks (for testing level transitions)
//! - ESC: Pause game (click to resume)

pub mod level_format;
pub mod level_loader;
pub mod pause;
pub mod systems;
pub mod ui;

pub use level_loader::extract_author_name;

#[cfg(feature = "texture_manifest")]
use crate::systems::TextureManifestPlugin;
use crate::systems::{
    AudioPlugin, InputLocked, LevelSwitchPlugin, LevelSwitchRequested, RespawnPlugin,
    RespawnSystems,
};

#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
#[cfg(not(target_arch = "wasm32"))]
use bevy::window::MonitorSelection;
use bevy::{
    asset::RenderAssetUsages,
    color::palettes::{basic::SILVER, css::RED},
    ecs::message::{MessageReader, MessageWriter},
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::{CursorGrabMode, CursorOptions, PrimaryWindow, Window, WindowMode, WindowPlugin},
};
use bevy_rapier3d::prelude::*;

const BALL_RADIUS: f32 = 0.3;
const PADDLE_RADIUS: f32 = 0.3;
const PADDLE_HEIGHT: f32 = 3.0;
const PLANE_H: f32 = 30.0;
const PLANE_W: f32 = 40.0;

// Paddle size powerup constants
const PADDLE_BASE_WIDTH: f32 = 20.0;
const PADDLE_MIN_WIDTH: f32 = 10.0;
const PADDLE_MAX_WIDTH: f32 = 30.0;
const PADDLE_SHRINK_MULTIPLIER: f32 = 0.7;
const PADDLE_ENLARGE_MULTIPLIER: f32 = 1.5;
const PADDLE_SIZE_EFFECT_DURATION: f32 = 10.0;

// Bounce/impulse tuning
// How strongly the wall collision pushes the ball (ExternalImpulse on balls)
const BALL_WALL_IMPULSE_FACTOR: f32 = 0.001;
// How strongly the paddle bounces back when hitting a wall
const PADDLE_BOUNCE_WALL_FACTOR: f32 = 0.03;
// How strongly the paddle bounces back when hitting a brick (separate from walls)
const PADDLE_BOUNCE_BRICK_FACTOR: f32 = 0.02;
// Maximum ball velocity (can be made ball-type dependent in the future)
const MAX_BALL_VELOCITY: f32 = 20.0;
// Camera shake parameters
const CAMERA_SHAKE_DURATION: f32 = 0.15;
const CAMERA_SHAKE_IMPULSE_SCALE: f32 = 0.005; // Scale factor for impulse to shake intensity
const CAMERA_SHAKE_MIN_INTENSITY: f32 = 0.05;
const CAMERA_SHAKE_MAX_INTENSITY: f32 = 10.0;
// Paddle growth animation duration
const PADDLE_GROWTH_DURATION: f32 = 2.0;

// Grid debug overlay constants (20x20 grid covering PLANE_H × PLANE_W)
const GRID_WIDTH: usize = 20; // Columns (Z-axis)
const GRID_HEIGHT: usize = 20; // Rows (X-axis)
const CELL_WIDTH: f32 = PLANE_W / GRID_WIDTH as f32; // 2.0 (Z dimension)
const CELL_HEIGHT: f32 = PLANE_H / GRID_HEIGHT as f32; // 1.5 (X dimension)
                                                       // Cell aspect ratio: CELL_HEIGHT / CELL_WIDTH = 30/40 * 20/20 = 3/4 = 0.75
/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
pub struct Paddle;
#[derive(Component)]
pub struct Ball;

/// Type ID for ball variants (used by texture manifest type_variants).
/// When changed, the ball-type watcher system will swap materials accordingly.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BallTypeId(pub u8);

/// Marker component for sidewall/border entities.
/// Used by per-level texture override system to apply custom sidewall materials.
#[derive(Component)]
pub struct Border;

/// Marker component for the ground plane entity.
/// Used by per-level texture override system to apply custom ground materials.
#[derive(Component)]
pub struct GroundPlane;

#[derive(Component)]
pub struct LowerGoal;
#[derive(Component)]
pub struct GridOverlay;
#[derive(Component)]
pub struct Brick;

/// Type ID for brick variants (used by texture manifest type_variants).
/// Applied during brick spawn based on level matrix values.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BrickTypeId(pub u8);

#[derive(Component)]
struct MarkedForDespawn;
#[derive(Component)]
/// Marker component attached to bricks that should count toward level completion
/// (i.e. destructible bricks). Indestructible bricks MUST NOT have this component.
pub struct CountsTowardsCompletion;
#[derive(Component)]
struct CameraShake {
    timer: Timer,
    intensity: f32,
    original_position: Vec3,
}

#[derive(Component)]
pub struct PaddleGrowing {
    pub timer: Timer,
    pub target_scale: Vec3,
    pub start_scale: Vec3,
}

#[derive(Component)]
pub struct BallFrozen;

/// Marker component for brick type 30 (triggers paddle shrink).
///
/// When a ball collides with a brick marked with this component,
/// the paddle will shrink to 70% of its base size (14 units) for 10 seconds.
#[derive(Component)]
pub struct BrickType30;

/// Marker component for brick type 32 (triggers paddle enlarge).
///
/// When a ball collides with a brick marked with this component,
/// the paddle will enlarge to 150% of its base size (30 units) for 10 seconds.
#[derive(Component)]
pub struct BrickType32;

/// Component that tracks an active paddle size effect.
///
/// This component is added to paddle entities when they should have a temporary
/// size modification. The effect is automatically removed after the duration expires.
///
/// # Examples
///
/// ```ignore
/// commands.entity(paddle_entity).insert(PaddleSizeEffect {
///     effect_type: SizeEffectType::Shrink,
///     remaining_duration: 10.0,
///     target_width: 14.0,
/// });
/// ```
#[derive(Component)]
pub struct PaddleSizeEffect {
    /// Type of the size effect (shrink or enlarge)
    pub effect_type: SizeEffectType,
    /// Time remaining for this effect in seconds
    pub remaining_duration: f32,
    /// Target width for this effect (clamped between 10 and 30 units)
    pub target_width: f32,
}

/// Type of paddle size modification effect.
///
/// - `Shrink`: Reduces paddle to 70% of base size (14 units)
/// - `Enlarge`: Increases paddle to 150% of base size (30 units)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeEffectType {
    /// Shrink effect (70% of base size)
    Shrink,
    /// Enlarge effect (150% of base size)
    Enlarge,
}

/// Emitted when the paddle collides with a wall boundary.
/// Used by the audio system to play paddle-wall collision sounds.
#[derive(Event)]
pub struct WallHit {
    /// The collision impulse.
    pub impulse: Vec3,
}

/// Emitted when the paddle collides with a brick.
/// Used by the audio system to play paddle-brick collision sounds.
#[derive(Event)]
pub struct BrickHit {
    /// The collision impulse.
    pub impulse: Vec3,
}

/// Emitted when the paddle collides with the ball.
/// Used by the audio system to play paddle hit sounds.
#[derive(Event)]
pub struct BallHit {
    /// The collision impulse.
    pub impulse: Vec3,
    /// The ball entity that was hit.
    pub ball: Entity,
}

/// Event emitted when a paddle size effect is applied (for audio/visual feedback).
///
/// This event is triggered when a ball collides with a powerup brick (type 30 or 32),
/// allowing audio and visual systems to respond to the effect activation.
#[derive(Event)]
pub struct PaddleSizeEffectApplied {
    /// The paddle entity affected
    pub paddle: Entity,
    /// Type of effect applied (shrink or enlarge)
    pub effect_type: SizeEffectType,
    /// The new target width (clamped between 10 and 30 units)
    pub target_width: f32,
}

/// Stores configurable gravity values (normal gameplay gravity, etc.)
#[derive(Resource)]
struct GravityConfig {
    normal: Vec3,
}

impl Default for GravityConfig {
    fn default() -> Self {
        Self {
            normal: Vec3::new(2.0, 0.0, 0.0),
        }
    }
}

#[derive(Resource, Default)]
pub struct GameProgress {
    finished: bool,
}

pub fn run() {
    let mut app = App::new();

    app.insert_resource(GravityConfig::default());
    app.insert_resource(GameProgress::default());
    // designer palette UI state
    app.init_resource::<ui::palette::PaletteState>();
    app.init_resource::<ui::palette::SelectedBrick>();
    app.insert_resource(level_loader::LevelAdvanceState::default());
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Brkrs".to_string(),
                    #[cfg(not(target_arch = "wasm32"))]
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    #[cfg(target_arch = "wasm32")]
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            }),
        #[cfg(not(target_arch = "wasm32"))]
        WireframePlugin::default(),
    ));
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugins(LevelSwitchPlugin);
    app.add_plugins(crate::level_loader::LevelLoaderPlugin);
    // app.add_plugins(RapierDebugRenderPlugin::default());
    app.add_plugins(RespawnPlugin);
    app.add_plugins(crate::pause::PausePlugin);
    app.add_plugins(AudioPlugin);

    #[cfg(feature = "texture_manifest")]
    {
        app.add_plugins(TextureManifestPlugin);
    }

    // Register paddle size effect event
    app.add_event::<PaddleSizeEffectApplied>();

    app.add_systems(
        Startup,
        (setup, spawn_border, systems::grid_debug::spawn_grid_overlay),
    );
    app.add_systems(
        Update,
        (
            move_paddle
                .after(RespawnSystems::Control)
                .run_if(crate::pause::not_paused),
            limit_ball_velocity,
            update_camera_shake,
            update_paddle_growth,
            stabilize_frozen_balls.before(crate::level_loader::LevelAdvanceSystems),
            restore_gravity_post_growth,
            #[cfg(not(target_arch = "wasm32"))]
            toggle_wireframe,
            #[cfg(not(target_arch = "wasm32"))]
            systems::grid_debug::toggle_grid_visibility,
            grab_mouse,
            read_character_controller_collisions,
            detect_ball_wall_collisions,
            mark_brick_on_ball_collision,
            detect_paddle_powerup_collisions,
            update_paddle_size_timers,
            remove_expired_size_effects,
            apply_paddle_size_to_transform,
            clear_paddle_effects_on_level_switch,
            clear_paddle_effects_on_respawn,
            despawn_marked_entities, // Runs after marking, allowing physics to resolve
            // display_events,
            // designer palette - toggle with P
            ui::palette::toggle_palette,
            ui::palette::ensure_palette_ui,
            ui::palette::handle_palette_selection,
            ui::palette::update_palette_selection_feedback,
            ui::palette::update_ghost_preview,
            ui::palette::place_bricks_on_drag,
            #[cfg(feature = "texture_manifest")]
            systems::multi_hit::watch_brick_type_changes,
        ),
    );
    app.add_observer(on_wall_hit);
    app.add_observer(on_paddle_ball_hit);
    app.add_observer(on_brick_hit);
    app.add_observer(start_camera_shake);
    // Note: Multi-hit brick sound observer is now registered by AudioPlugin
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rapier_config: Query<&mut RapierConfiguration>,
    gravity_cfg: Res<GravityConfig>,
) {
    let rapier_config = rapier_config.single_mut();
    // Set gravity for normal gameplay (respawn will temporarily disable it)
    rapier_config.unwrap().gravity = gravity_cfg.normal;

    let _debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    // Level entities (paddle, ball, bricks) are spawned by LevelLoaderPlugin after level parsing.

    // light
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

    // ground plane
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

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 37., 0.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        MainCamera,
    ));

    #[derive(Component)]
    struct MainCamera;

    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn((
        Text::new("Press space to toggle wire frames"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

/// Apply speed-dependent damping to control ball velocity
fn limit_ball_velocity(mut balls: Query<(&Velocity, &mut Damping), With<Ball>>) {
    for (velocity, mut damping) in balls.iter_mut() {
        let speed = velocity.linvel.length();

        // Calculate damping based on speed relative to target velocity
        // Higher speeds get more damping, lower speeds get less
        let speed_ratio = speed / MAX_BALL_VELOCITY;

        if speed_ratio > 1.0 {
            // Above target: increase damping exponentially
            damping.linear_damping = 0.5 + (speed_ratio - 1.0) * 2.0;
        } else if speed_ratio < 0.5 {
            // Below half target: reduce damping to allow acceleration
            damping.linear_damping = 0.01 + speed_ratio * 0.8;
        } else {
            // Near target: moderate damping
            damping.linear_damping = 0.1;
        }

        // Clamp damping to reasonable bounds
        damping.linear_damping = damping.linear_damping.clamp(0.1, 5.0);
    }
}

/// Update camera shake effect
fn update_camera_shake(
    time: Res<Time>,
    mut cameras: Query<(Entity, &mut Transform, Option<&mut CameraShake>)>,
    mut commands: Commands,
) {
    for (entity, mut transform, shake_opt) in cameras.iter_mut() {
        if let Some(mut shake) = shake_opt {
            shake.timer.tick(time.delta());

            if shake.timer.is_finished() {
                // Restore original position and remove shake component
                transform.translation = shake.original_position;
                commands.entity(entity).remove::<CameraShake>();
            } else {
                // Apply random offset based on intensity
                let progress = shake.timer.fraction();
                let intensity = shake.intensity * (1.0 - progress); // Fade out
                let offset = Vec3::new(
                    (time.elapsed_secs() * 50.0).sin() * intensity,
                    0.0,
                    (time.elapsed_secs() * 43.0).cos() * intensity,
                );
                transform.translation = shake.original_position + offset;
            }
        }
    }
}

/// Observer to start camera shake
fn start_camera_shake(
    trigger: On<StartCameraShake>,
    mut cameras: Query<(Entity, &Transform), (With<Camera3d>, Without<CameraShake>)>,
    mut commands: Commands,
) {
    let event = trigger.event();
    // Calculate intensity based on impulse magnitude
    let impulse_magnitude = event.impulse.length();
    let intensity = (impulse_magnitude * CAMERA_SHAKE_IMPULSE_SCALE)
        .clamp(CAMERA_SHAKE_MIN_INTENSITY, CAMERA_SHAKE_MAX_INTENSITY);

    for (entity, transform) in cameras.iter_mut() {
        commands.entity(entity).insert(CameraShake {
            timer: Timer::from_seconds(CAMERA_SHAKE_DURATION, TimerMode::Once),
            intensity,
            original_position: transform.translation,
        });
    }
}

/// Animate paddle growth over PADDLE_GROWTH_DURATION seconds
fn update_paddle_growth(
    time: Res<Time>,
    mut paddles: Query<(Entity, &mut Transform, &mut PaddleGrowing)>,
    mut rapier_config: Query<&mut RapierConfiguration>,
    gravity_cfg: Res<GravityConfig>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut growing) in paddles.iter_mut() {
        growing.timer.tick(time.delta());

        if growing.timer.is_finished() {
            // Growth complete: set final scale, enable gravity, remove component
            transform.scale = growing.target_scale;
            if let Ok(mut config) = rapier_config.single_mut() {
                config.gravity = gravity_cfg.normal;
            } else {
                warn!(
                    "Failed to restore gravity after paddle growth: RapierConfiguration not found"
                );
            }
            commands.entity(entity).remove::<PaddleGrowing>();
            info!(
                "Paddle growth completed, gravity restored to {:?}",
                gravity_cfg.normal
            );
        } else {
            // Interpolate scale from start to target
            let progress = growing.timer.fraction();
            // Use smooth easing function (ease-out cubic)
            let eased_progress = 1.0 - (1.0 - progress).powi(3);
            transform.scale = growing
                .start_scale
                .lerp(growing.target_scale, eased_progress);
        }
    }
}

/// Ensure gravity is restored if growth finished but previous restoration was missed.
/// Acts as a safety net in case the growth completion frame didn't run gravity restoration.
fn restore_gravity_post_growth(
    paddles: Query<&PaddleGrowing>,
    mut rapier_config: Query<&mut RapierConfiguration>,
    gravity_cfg: Res<GravityConfig>,
) {
    // Only restore if no paddle is growing and gravity is currently zero.
    if paddles.is_empty() {
        if let Ok(mut config) = rapier_config.single_mut() {
            if config.gravity == Vec3::ZERO {
                config.gravity = gravity_cfg.normal;
            }
        }
    }
}

/// Keep ball frozen (zero velocity, locked position) while paddle is growing
fn stabilize_frozen_balls(
    mut balls: Query<(Entity, Option<&mut Velocity>, &BallFrozen), With<Ball>>,
    mut commands: Commands,
) {
    for (entity, velocity, _frozen) in balls.iter_mut() {
        if let Some(mut vel) = velocity {
            // Ball has Velocity component, zero it out
            vel.linvel = Vec3::ZERO;
            vel.angvel = Vec3::ZERO;
        } else {
            // Ball doesn't have Velocity component, add it with zero velocity
            commands.entity(entity).insert(Velocity::zero());
        }
    }
}

fn move_paddle(
    mut query: Query<&mut Transform, (With<Paddle>, Without<InputLocked>)>,
    time: Res<Time>,
    mut controllers: Query<&mut KinematicCharacterController, (With<Paddle>, Without<InputLocked>)>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    window: Single<&Window, With<PrimaryWindow>>,
    growing: Query<&PaddleGrowing>,
) {
    if !window.focused {
        return;
    }
    // If paddle is currently growing, ignore input and movement entirely.
    if !growing.is_empty() {
        return;
    }
    let _sensitivity = 100.0 / window.height().min(window.width());
    if query.is_empty() {
        return;
    }

    for mut controller in controllers.iter_mut() {
        controller.translation = Some(
            Vec3::new(
                accumulated_mouse_motion.delta.y,
                0.0,
                -accumulated_mouse_motion.delta.x,
            ) * 0.000_4
                / time.delta_secs(),
        );
    }
    for mut transform in &mut query {
        // Allow rotation only when not growing
        transform.rotate_y(accumulated_mouse_scroll.delta.y * time.delta_secs() * 3.0);
        transform.translation.y = 2.0; // force the paddle to stay at the same height

        // Constrain paddle to play area bounds (with some padding for paddle size)
        let padding = PADDLE_HEIGHT / 2.0;
        let x_min = -PLANE_H / 2.0 + padding;
        let x_max = PLANE_H / 2.0 - padding;
        let z_min = -PLANE_W / 2.0 + padding;
        let z_max = PLANE_W / 2.0 - padding;

        transform.translation.x = transform.translation.x.clamp(x_min, x_max);
        transform.translation.z = transform.translation.z.clamp(z_min, z_max);
    }
}

fn spawn_border(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    _images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let border_material = materials.add(StandardMaterial {
        base_color: Color::from(RED),
        ..default()
    });

    // upper border
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 5.0, PLANE_W).mesh())),
        MeshMaterial3d(border_material.clone()),
        Transform::from_xyz(-15.5, 0.0, 0.0),
        Collider::cuboid(1.0, 2.5, PLANE_W / 2.0),
        Border,
    ));
    //
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(PLANE_H, 5.0, 1.0).mesh())),
        MeshMaterial3d(border_material.clone()),
        Transform::from_xyz(-0.0, 0.0, -20.5),
        Collider::cuboid(PLANE_H / 2.0, 2.5, 0.5),
        Border,
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(PLANE_H, 5.0, 1.0).mesh())),
        MeshMaterial3d(border_material.clone()),
        Transform::from_xyz(-0.0, 0.0, 20.5),
        Collider::cuboid(PLANE_H / 2.0, 2.5, 0.5),
        Border,
    ));
    //  lower border
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.0, 5.0, PLANE_W).mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.0, 0.0, 0.0, 1.0),
            //alpha_mode: AlphaMode::Mask(0.0),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(15.5, 0.0, 0.0),
        Collider::cuboid(0.0, 2.5, PLANE_W / 2.0),
        //Sensor::default(),
        Border,
    ));
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

/// Mark bricks for despawn when hit by the ball, or transition multi-hit bricks.
///
/// Multi-hit bricks (indices 10-13) transition to the next lower index instead of
/// being despawned immediately. When a multi-hit brick at index 10 is hit, it
/// transitions to index 20 (simple stone), which can then be destroyed on the next hit.
///
/// This allows the physics collision response to complete before removal.
fn mark_brick_on_ball_collision(
    mut collision_events: MessageReader<CollisionEvent>,
    balls: Query<Entity, With<Ball>>,
    // Query bricks with their type ID for multi-hit handling
    mut bricks: Query<
        (Entity, &mut BrickTypeId),
        (
            With<Brick>,
            With<CountsTowardsCompletion>,
            Without<MarkedForDespawn>,
        ),
    >,
    mut commands: Commands,
) {
    use crate::level_format::{is_multi_hit_brick, MULTI_HIT_BRICK_1, SIMPLE_BRICK};

    for event in collision_events.read() {
        // collision event processed
        if let CollisionEvent::Started(e1, e2, _) = event {
            let e1_is_ball = balls.get(*e1).is_ok();
            let e2_is_ball = balls.get(*e2).is_ok();

            // Determine which entity is the brick (if any)
            let brick_entity = if e1_is_ball {
                bricks.get_mut(*e2).ok()
            } else if e2_is_ball {
                bricks.get_mut(*e1).ok()
            } else {
                None
            };

            if let Some((entity, mut brick_type)) = brick_entity {
                let current_type = brick_type.0;

                if is_multi_hit_brick(current_type) {
                    // Multi-hit brick: transition to next state
                    let new_type = if current_type == MULTI_HIT_BRICK_1 {
                        // Index 10 transitions to index 20 (simple stone)
                        SIMPLE_BRICK
                    } else {
                        // Index 13, 12, 11 transition to index - 1
                        current_type - 1
                    };

                    // Emit event for audio/scoring integration
                    commands.trigger(systems::MultiHitBrickHit {
                        entity,
                        previous_type: current_type,
                        new_type,
                    });

                    // Update the brick type (this triggers watch_brick_type_changes for visual update)
                    brick_type.0 = new_type;

                    debug!(
                        "Multi-hit brick {:?} transitioned: {} -> {}",
                        entity, current_type, new_type
                    );
                } else {
                    // Regular brick: mark for despawn
                    commands.entity(entity).insert(MarkedForDespawn);
                }
            }
        }
    }
}

/// Detect ball-wall collisions and emit BallWallHit events for audio.
fn detect_ball_wall_collisions(
    mut collision_events: MessageReader<CollisionEvent>,
    balls: Query<(Entity, &Velocity), With<Ball>>,
    borders: Query<Entity, With<Border>>,
    mut commands: Commands,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            // Check if one entity is a ball and the other is a border
            let ball_data = balls.get(*e1).ok().or_else(|| balls.get(*e2).ok());
            let is_border = borders.get(*e1).is_ok() || borders.get(*e2).is_ok();

            if let (Some((ball_entity, velocity)), true) = (ball_data, is_border) {
                // Emit BallWallHit event for audio system
                commands.trigger(systems::BallWallHit {
                    entity: ball_entity,
                    impulse: velocity.linvel,
                });
            }
        }
    }
}

/// Detect ball collisions with powerup bricks (types 30 and 32) and apply paddle size effects.
///
/// This system listens for collision events between balls and bricks marked with
/// `BrickType30` (shrink) or `BrickType32` (enlarge) components. When a collision
/// is detected, it applies the corresponding size effect to all paddles in the game.
///
/// The system handles:
/// - Effect replacement: New effects override existing ones
/// - Timer reset: Hitting the same brick type resets the duration to 10 seconds
/// - Size clamping: Target widths are clamped between 10 and 30 units
///
/// # Size Effects
/// - **Brick 30 (Shrink)**: Paddle width becomes 14 units (70% of base 20 units)
/// - **Brick 32 (Enlarge)**: Paddle width becomes 30 units (150% of base 20 units)
fn detect_paddle_powerup_collisions(
    mut collision_events: MessageReader<CollisionEvent>,
    balls: Query<Entity, With<Ball>>,
    brick30: Query<Entity, With<BrickType30>>,
    brick32: Query<Entity, With<BrickType32>>,
    mut paddles: Query<(Entity, &Transform), With<Paddle>>,
    mut commands: Commands,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let e1_is_ball = balls.get(*e1).is_ok();
            let e2_is_ball = balls.get(*e2).is_ok();

            // Determine which entity is the ball and which is the brick
            let (ball_entity, brick_entity) = if e1_is_ball {
                (*e1, *e2)
            } else if e2_is_ball {
                (*e2, *e1)
            } else {
                continue;
            };

            // Check if brick is type 30 (shrink) or type 32 (enlarge)
            let effect_type = if brick30.get(brick_entity).is_ok() {
                Some(SizeEffectType::Shrink)
            } else if brick32.get(brick_entity).is_ok() {
                Some(SizeEffectType::Enlarge)
            } else {
                None
            };

            if let Some(effect_type) = effect_type {
                // Apply effect to all paddles (typically just one)
                for (paddle_entity, _transform) in paddles.iter_mut() {
                    let target_width = match effect_type {
                        SizeEffectType::Shrink => (PADDLE_BASE_WIDTH * PADDLE_SHRINK_MULTIPLIER)
                            .clamp(PADDLE_MIN_WIDTH, PADDLE_MAX_WIDTH),
                        SizeEffectType::Enlarge => (PADDLE_BASE_WIDTH * PADDLE_ENLARGE_MULTIPLIER)
                            .clamp(PADDLE_MIN_WIDTH, PADDLE_MAX_WIDTH),
                    };

                    // Remove any existing effect and add new one
                    commands
                        .entity(paddle_entity)
                        .remove::<PaddleSizeEffect>()
                        .insert(PaddleSizeEffect {
                            effect_type,
                            remaining_duration: PADDLE_SIZE_EFFECT_DURATION,
                            target_width,
                        });

                    // Emit event for audio/visual feedback
                    commands.trigger(PaddleSizeEffectApplied {
                        paddle: paddle_entity,
                        effect_type,
                        target_width,
                    });

                    debug!(
                        "Applied {:?} effect to paddle {:?}, target width: {}",
                        effect_type, paddle_entity, target_width
                    );
                }
            }
        }
    }
}

/// Update paddle size effect timers.
///
/// Decrements the remaining duration of all active paddle size effects based on
/// the elapsed time since the last frame. Durations are clamped to a minimum of 0.
fn update_paddle_size_timers(mut effects: Query<&mut PaddleSizeEffect>, time: Res<Time>) {
    for mut effect in effects.iter_mut() {
        effect.remaining_duration -= time.delta_secs();
        effect.remaining_duration = effect.remaining_duration.max(0.0);
    }
}

/// Remove expired paddle size effects and restore paddle to base size.
///
/// Checks all paddles with active size effects and removes the component when
/// the remaining duration reaches zero. The paddle's transform will automatically
/// be restored to base scale by `apply_paddle_size_to_transform` in the next frame.
fn remove_expired_size_effects(
    mut paddles: Query<(Entity, &PaddleSizeEffect)>,
    mut commands: Commands,
) {
    for (entity, effect) in paddles.iter_mut() {
        if effect.remaining_duration <= 0.0 {
            commands.entity(entity).remove::<PaddleSizeEffect>();
            debug!("Removed expired size effect from paddle {:?}", entity);
        }
    }
}

/// Apply paddle size effect to the paddle's transform scale.
///
/// Updates the transform scale of all paddles based on their active size effects.
/// The scale is applied to the Z-axis (width) while keeping X and Y axes at 1.0.
///
/// - With effect: Scale = (target_width / base_width) in Z-axis
/// - Without effect: Scale = Vec3::ONE (base size)
///
/// The base paddle width is 20 units, so:
/// - Shrink effect (14 units) = 0.7 scale
/// - Enlarge effect (30 units) = 1.5 scale
fn apply_paddle_size_to_transform(
    mut paddles: Query<(&mut Transform, Option<&PaddleSizeEffect>), With<Paddle>>,
) {
    for (mut transform, effect) in paddles.iter_mut() {
        let target_scale = if let Some(effect) = effect {
            // Calculate scale based on target width relative to base width
            let scale_factor = effect.target_width / PADDLE_BASE_WIDTH;
            Vec3::new(1.0, 1.0, scale_factor)
        } else {
            // No effect active, use base scale
            Vec3::ONE
        };

        // Smoothly interpolate to target scale (or just set it directly for immediate effect)
        transform.scale = target_scale;
    }
}

/// Clear paddle size effects when a level switch is requested.
///
/// Removes all active paddle size effects when transitioning between levels,
/// ensuring the paddle starts each level at its base size.
fn clear_paddle_effects_on_level_switch(
    mut level_switch_events: MessageReader<systems::LevelSwitchRequested>,
    paddles: Query<Entity, (With<Paddle>, With<PaddleSizeEffect>)>,
    mut commands: Commands,
) {
    if level_switch_events.read().next().is_some() {
        for paddle_entity in paddles.iter() {
            commands.entity(paddle_entity).remove::<PaddleSizeEffect>();
            debug!(
                "Cleared paddle size effect on level switch for paddle {:?}",
                paddle_entity
            );
        }
    }
}

/// Clear paddle size effects when ball respawn is scheduled (player loses life).
///
/// Removes all active paddle size effects when the ball hits the lower goal,
/// ensuring the paddle is reset to base size when the player loses a life.
fn clear_paddle_effects_on_respawn(
    mut collision_events: MessageReader<CollisionEvent>,
    balls: Query<Entity, With<Ball>>,
    lower_goals: Query<Entity, With<LowerGoal>>,
    paddles: Query<Entity, (With<Paddle>, With<PaddleSizeEffect>)>,
    mut commands: Commands,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let e1_is_ball = balls.get(*e1).is_ok();
            let e2_is_ball = balls.get(*e2).is_ok();
            let e1_is_goal = lower_goals.get(*e1).is_ok();
            let e2_is_goal = lower_goals.get(*e2).is_ok();

            // If ball hits lower goal, clear paddle effects
            if (e1_is_ball && e2_is_goal) || (e2_is_ball && e1_is_goal) {
                for paddle_entity in paddles.iter() {
                    commands.entity(paddle_entity).remove::<PaddleSizeEffect>();
                    debug!(
                        "Cleared paddle size effect on ball loss for paddle {:?}",
                        paddle_entity
                    );
                }
            }
        }
    }
}

/// Despawn entities marked for removal (runs after physics step).
/// Emits BrickDestroyed events for audio system integration.
fn despawn_marked_entities(
    marked: Query<(Entity, Option<&BrickTypeId>), With<MarkedForDespawn>>,
    mut commands: Commands,
) {
    for (entity, brick_type) in marked.iter() {
        // Emit BrickDestroyed event for audio system
        if let Some(brick_type) = brick_type {
            commands.trigger(systems::BrickDestroyed {
                entity,
                brick_type: brick_type.0,
            });
        }
        commands.entity(entity).despawn();
    }
}

/// Public helper to register the brick collision + despawn systems on an arbitrary App.
/// Tests can call this to mimic the runtime configuration used by the main app.
pub fn register_brick_collision_systems(app: &mut App) {
    app.add_systems(
        Update,
        (mark_brick_on_ball_collision, despawn_marked_entities),
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn grab_mouse(
    window: Single<&Window, With<PrimaryWindow>>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    if !window.focused {
        return;
    }
    if mouse.just_pressed(MouseButton::Left) {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }

    if key.just_pressed(KeyCode::KeyQ) {
        app_exit.write(AppExit::Success);
    }
}

/* Read the character controller collisions stored in the character controller’s output. */
fn read_character_controller_collisions(
    paddle_outputs: Query<&KinematicCharacterControllerOutput, With<Paddle>>,
    walls: Query<Entity, With<Border>>,
    bricks: Query<Entity, With<Brick>>,
    balls: Query<Entity, With<Ball>>,
    time: Res<Time>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut commands: Commands,
) {
    let output = match paddle_outputs.single() {
        Ok(controller) => controller,
        Err(_) => return,
    };
    for collision in output.collisions.iter() {
        // paddle collides with the walls
        for wall in walls.iter() {
            if collision.entity == wall {
                commands.trigger(WallHit {
                    impulse: (collision.translation_applied + collision.translation_remaining)
                        / time.delta_secs(),
                });
            }
        }
    }
    for collision in output.collisions.iter() {
        // paddle collides with the bricks: emit BrickHit (separate from walls)
        for brick in bricks.iter() {
            if collision.entity == brick {
                commands.trigger(BrickHit {
                    impulse: (collision.translation_applied + collision.translation_remaining)
                        / time.delta_secs(),
                });
            }
        }
    }
    for collision in output.collisions.iter() {
        // paddle collides with the balls
        for ball in balls.iter() {
            if collision.entity == ball {
                // println!("hit ball {:?}", ball);
                println!("collision {:?}", collision);
                commands.trigger(BallHit {
                    impulse: Vec3::new(
                        accumulated_mouse_motion.delta.y,
                        0.0,
                        -accumulated_mouse_motion.delta.x,
                    ) / time.delta_secs(),
                    ball,
                });
            }
        }
    }
}

fn on_wall_hit(
    trigger: On<WallHit>,
    mut balls: Query<&mut ExternalImpulse, With<Ball>>,
    mut controllers: Query<&mut KinematicCharacterController, With<Paddle>>,
    mut commands: Commands,
) {
    let event = trigger.event();

    // give the balls an impulse
    for mut impulse in balls.iter_mut() {
        impulse.impulse = event.impulse * BALL_WALL_IMPULSE_FACTOR;
    }

    // let the paddle bounce back on wall collisions as well
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(-event.impulse * PADDLE_BOUNCE_WALL_FACTOR);
    }

    // Trigger camera shake with impulse-based intensity
    commands.trigger(StartCameraShake {
        impulse: event.impulse,
    });
}

#[derive(Event)]
struct StartCameraShake {
    impulse: Vec3,
}

fn on_brick_hit(
    trigger: On<BrickHit>,
    mut controllers: Query<&mut KinematicCharacterController, With<Paddle>>,
    mut commands: Commands,
) {
    let event = trigger.event();

    // let the paddle bounce back on brick collisions only
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(-event.impulse * PADDLE_BOUNCE_BRICK_FACTOR);
    }

    // Trigger camera shake with impulse-based intensity
    commands.trigger(StartCameraShake {
        impulse: event.impulse,
    });
}

fn on_paddle_ball_hit(
    trigger: On<BallHit>,
    mut balls: Query<(Entity, &mut ExternalImpulse), With<Ball>>,
) {
    let event = trigger.event();
    println!("Received ball hit event: {:?}", event.impulse);

    // give the balls an impulse with "english" - paddle rotation affects ball trajectory
    // Tuned multiplier for noticeable but controlled steering effect
    for (ball, mut impulse) in balls.iter_mut() {
        if ball == event.ball {
            impulse.impulse = event.impulse * 0.001; // Increased from 0.000_2 for more noticeable effect
        }
    }
}
