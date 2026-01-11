pub mod physics_config;
// brkrs: see README.md for usage and controls.

pub mod level_format;
pub mod level_loader;
pub mod pause;
pub mod signals;
pub mod systems;
pub mod ui;

pub use level_loader::extract_author_name;

#[cfg(feature = "texture_manifest")]
use crate::systems::TextureManifestPlugin;
use crate::systems::{
    AudioPlugin, InputLocked, LevelSwitchPlugin, MerkabaPlugin, PaddleSizePlugin, RespawnPlugin,
    RespawnSystems,
};

#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
#[cfg(not(target_arch = "wasm32"))]
use bevy::window::MonitorSelection;
use bevy::{
    color::palettes::css::RED,
    ecs::message::MessageWriter,
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow, Window, WindowMode, WindowPlugin},
};
use bevy_rapier3d::prelude::*;

pub(crate) const BALL_RADIUS: f32 = 0.3;
pub(crate) const PADDLE_RADIUS: f32 = 0.3;
pub(crate) const PADDLE_HEIGHT: f32 = 3.0;
pub(crate) const PLANE_H: f32 = 30.0;
pub(crate) const PLANE_W: f32 = 40.0;

// Bounce/impulse tuning
// How strongly the paddle/wall collision pushes the ball (ExternalImpulse on balls)
const BALL_WALL_IMPULSE_FACTOR: f32 = 0.001;
// How strongly the paddle/brick collision pushes the ball (ExternalImpulse on balls)
const BALL_BRICK_IMPULSE_FACTOR: f32 = 0.000_5;
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
#[require(Transform, Visibility)]
pub struct Paddle;
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Ball;

/// Type ID for ball variants (used by texture manifest type_variants).
/// When changed, the ball-type watcher system will swap materials accordingly.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BallTypeId(pub u8);

/// Marker component for sidewall/border entities.
/// Used by per-level texture override system to apply custom sidewall materials.
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Border;

pub use systems::spawning::{GroundPlane, MainCamera};

#[derive(Component)]
pub struct LowerGoal;
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct GridOverlay;
#[derive(Component)]
pub struct Brick;

/// Type ID for brick variants (used by texture manifest type_variants).
/// Applied during brick spawn based on level matrix values.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BrickTypeId(pub u8);

#[derive(Component)]
pub struct MarkedForDespawn;
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

/// Stores configurable gravity values (normal gameplay gravity, etc.)
#[derive(Resource)]
pub(crate) struct GravityConfig {
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
///
/// When a brick with this component is destroyed, the gravity immediately
/// changes to the value specified in the component. The `GravityChanged` message
/// is sent to communicate the change to the physics system.
///
/// **Brick Indices**:
/// - 21: Zero Gravity (0.0, 0.0, 0.0)
/// - 22: Moon Gravity (0.0, 2.0, 0.0)
/// - 23: Earth Gravity (0.0, 10.0, 0.0)
/// - 24: High Gravity (0.0, 20.0, 0.0)
/// - 25: Queer Gravity (random X, Y=0.0, random Z)
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct GravityBrick {
    /// Brick index (21-25)
    pub index: u32,
    /// Gravity vector to apply when this brick is destroyed
    pub gravity: Vec3,
}

/// Tracks the current and default gravity for the level.
///
/// This resource is updated when gravity bricks are destroyed and reset
/// when the player loses a ball. The physics system reads `current` to
/// apply gravity to the ball's rigid body.
///
/// **Coordinate System**: Bevy standard (Y = up, X = right, Z = back)
#[derive(Resource, Clone, Copy, Debug)]
pub struct GravityConfiguration {
    /// Current gravity being applied to the ball
    pub current: Vec3,
    /// Level's starting gravity (used to reset on ball loss)
    pub level_default: Vec3,
}

impl Default for GravityConfiguration {
    fn default() -> Self {
        Self {
            current: Vec3::ZERO,
            level_default: Vec3::ZERO,
        }
    }
}

pub fn run() {
    let mut app = App::new();
    // Register BallWallHit as an event so the observer is active from the start
    // ...existing code...

    app.insert_resource(GravityConfig::default());
    app.insert_resource(GameProgress::default());
    // Physics config resources
    app.insert_resource(crate::physics_config::BallPhysicsConfig::default());
    app.insert_resource(crate::physics_config::PaddlePhysicsConfig::default());
    app.insert_resource(crate::physics_config::BrickPhysicsConfig::default());
    // Scoring system state
    app.init_resource::<systems::scoring::ScoreState>();
    app.add_message::<crate::signals::BrickDestroyed>();
    app.add_message::<crate::signals::SpawnMerkabaMessage>();
    app.add_message::<crate::signals::LifeAwardMessage>();
    app.add_message::<systems::scoring::MilestoneReached>();
    app.add_message::<bevy_rapier3d::prelude::CollisionEvent>();
    app.insert_resource(level_loader::LevelAdvanceState::default());
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "brkrs".to_string(),
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
    // Register BallWallHit as an event so the observer is active before AudioPlugin
    app.add_message::<crate::signals::BallWallHit>();
    app.add_plugins(AudioPlugin);
    app.add_plugins(MerkabaPlugin);
    app.add_plugins(PaddleSizePlugin);
    // Cheat mode plugin (feature: toggle, indicator, gated level controls)
    app.add_plugins(systems::CheatModePlugin);

    #[cfg(feature = "texture_manifest")]
    {
        app.add_plugins(TextureManifestPlugin);
    }

    // FontsPlugin wires platform-appropriate font loading systems
    app.add_plugins(crate::ui::fonts::FontsPlugin);
    // UI plugin (Constitution VIII: Plugin-Based Architecture)
    app.add_plugins(crate::ui::UiPlugin);

    app.add_systems(
        Startup,
        (
            setup,
            spawn_border,
            systems::grid_debug::spawn_grid_overlay,
            systems::spawning::spawn_camera,
            systems::spawning::spawn_ground_plane,
            systems::spawning::spawn_light,
        )
            .chain(),
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
            // Chain brick-hit handling, despawn, and life award application to guarantee ordering
            (
                mark_brick_on_ball_collision,
                despawn_marked_entities,
                crate::systems::respawn::apply_life_awards,
            )
                .chain(),
        ),
    );

    // Scoring: award points after brick despawn events have been emitted
    app.add_systems(
        Update,
        (
            systems::scoring::award_points_system,
            systems::scoring::detect_milestone_system,
            systems::respawn::award_milestone_ball_system,
        )
            .chain()
            .after(despawn_marked_entities),
    );

    // Texture manifest system (conditional on feature flag)
    #[cfg(feature = "texture_manifest")]
    app.add_systems(Update, systems::multi_hit::watch_brick_type_changes);

    app.add_observer(on_wall_hit);
    app.add_observer(on_paddle_ball_hit);
    app.add_observer(on_brick_hit);
    app.add_observer(start_camera_shake);
    // GravityChanged message type registration (gravity bricks feature)
    app.add_message::<systems::gravity::GravityChanged>();
    // Foundation system: load gravity configuration from level metadata
    app.add_systems(
        Update,
        systems::gravity::gravity_configuration_loader_system,
    );
    // Note: Multi-hit brick sound observer is now registered by AudioPlugin
    app.run();
}

fn setup(mut rapier_config: Query<&mut RapierConfiguration>, gravity_cfg: Res<GravityConfig>) {
    // Set gravity for normal gameplay (respawn will temporarily disable it)
    if let Ok(mut config) = rapier_config.single_mut() {
        config.gravity = gravity_cfg.normal;
    } else {
        warn!("RapierConfiguration not found; gravity not set");
    }
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
            damping.linear_damping = 0.000_1 + speed_ratio * 0.08;
        } else {
            // Near target: moderate damping
            damping.linear_damping = 0.000_1;
        }

        // Clamp damping to reasonable bounds
        damping.linear_damping = damping.linear_damping.clamp(0.000_1, 10.0);
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
        Mesh3d(meshes.add(Cuboid::new(5.0, 5.0, PLANE_W + 5.0).mesh())),
        MeshMaterial3d(border_material.clone()),
        Transform::from_xyz(-17.5, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(2.5, 2.5, PLANE_W / 2.0),
        ActiveEvents::COLLISION_EVENTS,
        Border,
    ));
    // side borders
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(PLANE_H, 5.0, 5.0).mesh())),
        MeshMaterial3d(border_material.clone()),
        Transform::from_xyz(-0.0, 0.0, -22.5),
        RigidBody::Fixed,
        Collider::cuboid(PLANE_H / 2.0, 2.5, 2.5),
        ActiveEvents::COLLISION_EVENTS,
        Border,
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(PLANE_H, 5.0, 5.0).mesh())),
        MeshMaterial3d(border_material.clone()),
        Transform::from_xyz(-0.0, 0.0, 22.5),
        RigidBody::Fixed,
        Collider::cuboid(PLANE_H / 2.0, 2.5, 2.5),
        ActiveEvents::COLLISION_EVENTS,
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
        RigidBody::Fixed,
        Collider::cuboid(0.0, 2.5, PLANE_W / 2.0),
        ActiveEvents::COLLISION_EVENTS,
        //Sensor::default(),
        LowerGoal,
    ));
}

/// Mark bricks for despawn when hit by the ball, or transition multi-hit bricks.
///
/// Multi-hit bricks (indices 10-13) transition to the next lower index instead of
/// being despawned immediately. When a multi-hit brick at index 10 is hit, it
/// transitions to index 20 (simple stone), which can then be destroyed on the next hit.
///
/// This allows the physics collision response to complete before removal.
pub fn mark_brick_on_ball_collision(
    mut collision_events: MessageReader<CollisionEvent>,
    balls: Query<Entity, With<Ball>>,
    // Use a ParamSet to avoid Bevy B0001 conflicting borrows across queries
    mut bricks: ParamSet<(
        Query<
            (
                Entity,
                &BrickTypeId,
                Option<&GlobalTransform>,
                Option<&Transform>,
            ),
            (
                With<Brick>,
                With<CountsTowardsCompletion>,
                Without<MarkedForDespawn>,
            ),
        >,
        Query<(Entity, &mut BrickTypeId), With<Brick>>,
    )>,
    transforms: Query<&Transform>,
    mut commands: Commands,
    mut processed_bricks: Local<std::collections::HashSet<Entity>>,
    mut spawn_msgs: Option<MessageWriter<crate::signals::SpawnMerkabaMessage>>,
    mut brick_destroyed_msgs: Option<MessageWriter<crate::signals::BrickDestroyed>>,
    mut life_award_msgs: Option<MessageWriter<crate::signals::LifeAwardMessage>>,
) {
    use crate::level_format::{is_multi_hit_brick, MULTI_HIT_BRICK_1, SIMPLE_BRICK};
    // Track bricks already processed this frame to avoid double-awards on multi-ball collisions
    processed_bricks.clear();

    for event in collision_events.read() {
        // collision event processed
        if let CollisionEvent::Started(e1, e2, _) = event {
            let e1_is_ball = balls.get(*e1).is_ok();
            let e2_is_ball = balls.get(*e2).is_ok();

            // Determine which entity is the brick (if any)
            let bricks_info = bricks.p0();
            let brick_info = if e1_is_ball {
                bricks_info.get(*e2).ok()
            } else if e2_is_ball {
                bricks_info.get(*e1).ok()
            } else {
                None
            };

            if let Some((entity, brick_type_ro, gt_opt, t_opt)) = brick_info {
                if processed_bricks.contains(&entity) {
                    continue;
                }
                let current_type = brick_type_ro.0;
                // Prefer Transform over GlobalTransform over direct query
                let brick_pos = if let Some(t) = t_opt {
                    t.translation
                } else if let Some(gt) = gt_opt {
                    gt.translation()
                } else {
                    transforms
                        .get(entity)
                        .map(|t| t.translation)
                        .unwrap_or(Vec3::ZERO)
                };

                if is_multi_hit_brick(current_type) {
                    // Multi-hit brick: transition to next state (requires mutable borrow)
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
                    if let Ok((_, mut brick_type)) = bricks.p1().get_mut(entity) {
                        brick_type.0 = new_type;
                    }

                    debug!(
                        "Multi-hit brick {:?} transitioned: {} -> {}",
                        entity, current_type, new_type
                    );
                } else {
                    // Regular brick: mark for despawn
                    // Brick 41 (Extra Life): award +1 life via Message before despawn
                    if current_type == crate::level_format::EXTRA_LIFE_BRICK {
                        if let Some(writer) = life_award_msgs.as_mut() {
                            writer.write(crate::signals::LifeAwardMessage { delta: 1 });
                        }
                    }
                    processed_bricks.insert(entity);
                    if current_type == 36 {
                        if let Some(writer) = spawn_msgs.as_mut() {
                            writer.write(crate::signals::SpawnMerkabaMessage {
                                position: brick_pos,
                                delay_seconds: 0.5,
                                angle_variance_deg: 20.0,
                                min_speed_y: 3.0,
                            });
                        }
                        // Record last rotor spawn position as fallback for spawn processing
                        // Emit BrickDestroyed for consistency and despawn immediately to satisfy tests
                        if let Some(writer) = brick_destroyed_msgs.as_mut() {
                            writer.write(crate::signals::BrickDestroyed {
                                brick_entity: entity,
                                brick_type: current_type,
                                destroyed_by: None,
                            });
                        }
                        commands.entity(entity).try_despawn();
                    } else {
                        commands.entity(entity).insert(MarkedForDespawn);
                    }
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
            let ball = if let Ok((entity, velocity)) = balls.get(*e1) {
                Some((entity, velocity, *e2))
            } else if let Ok((entity, velocity)) = balls.get(*e2) {
                Some((entity, velocity, *e1))
            } else {
                None
            };
            if let Some((ball_entity, _velocity, other_entity)) = ball {
                if borders.get(other_entity).is_ok() {
                    // Emit BallWallHit event for audio system (signals::BallWallHit)
                    println!(
                        "BallWallHit event emitted for ball {:?} and wall {:?}",
                        ball_entity, other_entity
                    );
                    commands.trigger(crate::signals::BallWallHit {
                        ball_entity,
                        wall_entity: other_entity,
                    });
                }
            }
        }
    }
}

/// Despawn entities marked for removal (runs after physics step).
/// Emits BrickDestroyed messages for audio/scoring integration.
fn despawn_marked_entities(
    marked: Query<(Entity, Option<&BrickTypeId>), With<MarkedForDespawn>>,
    mut commands: Commands,
    mut brick_events: Option<MessageWriter<crate::signals::BrickDestroyed>>,
) {
    for (entity, brick_type) in marked.iter() {
        // Emit BrickDestroyed message for audio/scoring systems
        if let Some(brick_type) = brick_type {
            if let Some(writer) = brick_events.as_mut() {
                writer.write(crate::signals::BrickDestroyed {
                    brick_entity: entity,
                    brick_type: brick_type.0,
                    destroyed_by: None,
                });
            }
        }
        commands.entity(entity).despawn();
    }
}

/// Public helper to register the brick collision + despawn systems on an arbitrary App.
/// Tests can call this to mimic the runtime configuration used by the main app.
pub fn register_brick_collision_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            mark_brick_on_ball_collision,
            despawn_marked_entities,
            crate::systems::respawn::apply_life_awards,
        )
            .chain()
            .before(crate::systems::merkaba::MerkabaSpawnFlowSystems::Queue),
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
    cheat_mode: Option<Res<systems::cheat_mode::CheatModeState>>,
) {
    let cheat_active = cheat_mode.map(|c| c.active).unwrap_or(false);
    if cheat_active && keyboard.just_pressed(KeyCode::Space) {
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
                // println!("collision {:?}", collision);
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
    mut balls: Query<&mut ExternalImpulse, With<Ball>>,
    mut controllers: Query<&mut KinematicCharacterController, With<Paddle>>,
    mut commands: Commands,
) {
    let event = trigger.event();

    // give the balls an impulse
    for mut impulse in balls.iter_mut() {
        impulse.impulse = event.impulse * BALL_BRICK_IMPULSE_FACTOR;
    }

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
