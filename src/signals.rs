//! Signal types for inter-system communication via Messages
//!
//! This module defines signal types that allow systems to communicate via Bevy's Message pattern,
//! providing a clean, single-path interface for event semantics without observer callbacks.
//!
//! # Architecture
//!
//! Signals follow the Constitution requirement for unified message boundaries:
//! - Each semantic event has exactly ONE producer path and ONE consumer path
//! - No dual Event/Message paths for the same signal type
//! - Systems consume via `MessageReader<T>` parameters
//! - Systems produce via `MessageWriter<T>` parameters
//!
//! # Observer Events
//!
//! Some signals use Bevy 0.17 [`Event`](bevy::ecs::event::Event) with observer pattern for
//! immediate, synchronous effects within the same frame:
//! - [`DirectionBrickEffect`]: Ball velocity impulses (triggered via `commands.trigger()`)
//! - [`BallWallHit`]: Immediate wall collision audio (triggered via `commands.trigger()`)
//! - [`MerkabaPaddleCollision`]: Paddle penalty effects (triggered via `commands.trigger()`)
//!
//! # Message Types (Buffered)
//!
//! - [`UiBeep`]: Short audio feedback cue from UI interactions
//! - [`BrickDestroyed`]: Brick destruction signal for scoring and audio
//! - [`LifeAwardMessage`]: Life increment signal (+1 or other delta) clamped to max
//! - [`SpawnMerkabaMessage`]: Delayed merkaba spawning
//! - [`MerkabaWallCollision`]: Merkaba-wall collisions (non-immediate)
//! - [`MerkabaBrickCollision`]: Merkaba-brick collisions (non-immediate)
//!
//! # Usage Example
//!
//! Producing a signal:
//! ```no_run
//! # use bevy::prelude::*;
//! # use bevy::ecs::message::MessageWriter;
//! # use brkrs::signals::UiBeep;
//! fn my_producer(mut writer: MessageWriter<UiBeep>) {
//!     writer.write(UiBeep); // Send signal
//! }
//! ```
//!
//! Consuming a signal:
//! ```no_run
//! # use bevy::prelude::*;
//! # use bevy::ecs::message::MessageReader;
//! # use brkrs::signals::UiBeep;
//! fn my_consumer(mut reader: MessageReader<UiBeep>) {
//!     for _ in reader.read() {
//!         println!("Beep!");
//!     }
//! }
//! ```

use bevy::ecs::message::Message;
use bevy::prelude::{Entity, Event, Vec3};

/// Short UI feedback cue (beep) — buffered message consumed by audio systems.
///
/// **Producers**: UI systems (pause menu, ball loss, game over)
/// **Consumers**: Audio system [`crate::systems::audio::consume_ui_beep_messages`]
/// **Contract**: One beep per user interaction (no duplicates within same frame)
#[derive(Message, Debug, Clone, Copy)]
pub struct UiBeep;

/// Ball-wall collision event for immediate wall hit audio feedback.
///
/// **Producers**: Ball-wall collision system (emits via `commands.trigger`)
/// **Consumers**: Audio system (observer: `On<BallWallHit>`, plays wall hit sound immediately)
/// **Contract**: Fired once per ball-wall collision, includes ball and wall entities
#[derive(Event, Message, Debug, Clone, Copy)]
pub struct BallWallHit {
    /// Entity of the ball that hit the wall
    pub ball_entity: Entity,
    /// Entity of the wall that was hit
    pub wall_entity: Entity,
}

/// Brick destruction signal unified for scoring and audio.
///
/// **Producers**: Brick collision system, entity despawn system
/// **Consumers**: Audio system, scoring system (when implemented)
/// **Contract**: Fired exactly once per brick destruction, includes destruction context
#[derive(Message, Debug, Clone, Copy)]
pub struct BrickDestroyed {
    /// Entity of the destroyed brick
    pub brick_entity: Entity,
    /// Brick type identifier (0=normal, 1=indestructible, etc.)
    pub brick_type: u8,
    /// World position of the destroyed brick center
    pub brick_position: Vec3,
    /// Entity that caused destruction (ball, paddle, etc.) or None for despawn
    pub destroyed_by: Option<Entity>,
}

/// Life award signal for granting extra lives to the player.
///
/// **Producers**: Brick collision systems (e.g., brick 41 extra life brick)
/// **Consumers**: Lives system (clamps to configured max, updates UI via Changed<T>)
/// **Contract**: Buffered message; consumer clamps `current + delta` to `[0, max]` defensively
/// to handle corrupted state and logs warnings for out-of-bounds values.
#[derive(Message, Debug, Clone, Copy)]
pub struct LifeAwardMessage {
    /// Life delta to apply (use +1 for brick 41; negative values allowed for penalties)
    pub delta: i32,
}

/// Spawn a merkaba hazard after a rotor brick (index 36) is hit.
///
/// **Producers**: Rotor brick collision system (US1)
/// **Consumers**: Merkaba spawn system (US1 delayed spawn)
/// **Contract**: Message is buffered; spawns occur after `delay_seconds` at the
/// destroyed brick position with randomized y-direction angle variance.
#[derive(Message, Debug, Clone, Copy)]
pub struct SpawnMerkabaMessage {
    /// World position where the merkaba should spawn (destroyed brick position)
    pub position: Vec3,
    /// Delay before spawn occurs (seconds); expected 0.5s per spec
    pub delay_seconds: f32,
    /// Angle variance in degrees from pure horizontal (y-direction); expected ±20°
    pub angle_variance_deg: f32,
    /// Minimum y-speed (units/second) to enforce after spawn; expected ≥3.0
    pub min_speed_y: f32,
}

/// Merkaba collision with a wall boundary.
///
/// **Producers**: Merkaba collision detection system (US2)
/// **Consumers**: Audio system for wall collision sound
/// **Contract**: Fired once per merkaba-wall collision, triggers distinct audio feedback
#[derive(Message, Debug, Clone, Copy)]
pub struct MerkabaWallCollision {
    /// Entity of the merkaba that collided
    pub merkaba_entity: Entity,
    /// Entity of the wall that was hit
    pub wall_entity: Entity,
}

/// Merkaba collision with a brick (non-destructive bounce).
///
/// **Producers**: Merkaba collision detection system (US2)
/// **Consumers**: Audio system for brick collision sound
/// **Contract**: Fired once per merkaba-brick collision; brick is NOT destroyed
#[derive(Message, Debug, Clone, Copy)]
pub struct MerkabaBrickCollision {
    /// Entity of the merkaba that collided
    pub merkaba_entity: Entity,
    /// Entity of the brick that was hit
    pub brick_entity: Entity,
}

/// Merkaba collision with paddle (penalty interaction).
///
/// **Producers**: Merkaba collision detection system (US3)
/// **Consumers**: Audio system and paddle penalty system
/// **Contract**: Fired once per merkaba-paddle collision; triggers paddle shrink or life loss
#[derive(Event, Debug, Clone, Copy)]
pub struct MerkabaPaddleCollision {
    /// Entity of the merkaba that collided
    pub merkaba_entity: Entity,
    /// Entity of the paddle that was hit
    pub paddle_entity: Entity,
}

/// Direction brick impulse effect for immediate velocity impulses.
///
/// Direction bricks (types 43-48, 52) apply instantaneous impulses to the ball
/// via Bevy 0.17 observer pattern and rapier3d's `ExternalImpulse` component,
/// triggering immediate (within same frame) physics effects.
///
/// **Producers**: Brick collision system (emits via `commands.trigger()` when ball hits direction brick)
/// **Consumers**: Observer function `apply_direction_brick_effects` (reads `On<DirectionBrickEffect>`,
/// applies impulse via `ExternalImpulse` component to ball entity)
/// **Contract**: Fired once per ball-direction-brick collision; impulses applied instantaneously
/// within the same `app.update()` cycle as collision detection; velocity changes from impulses
/// persist and compound with other physics effects (gravity, wall bounces).
///
/// # Impulse Semantics
///
/// - **Coordinate System**: XZ plane is horizontal (gameplay surface); Y is vertical (unused, always 0)
/// - **Direction Bricks 43-48**: Apply 5.0 units/sec impulse in XZ plane (additive, compounds with existing forces)
/// - **Direction Brick 52**: Apply random magnitude (5.0-15.0) and direction (0.0..2π radians) impulse in XZ plane
/// - **Y-Axis Preservation**: No direction brick applies impulse to Y-component (vertical axis unused)
///
/// # Per-Brick Behavior
///
/// - **Brick 43 (Forward)**: Apply impulse `(+5.0, 0, 0)` units/sec (toward far wall)
/// - **Brick 44 (Left)**: Apply impulse `(0, 0, +5.0)` units/sec
/// - **Brick 45 (Right)**: Apply impulse `(0, 0, -5.0)` units/sec
/// - **Brick 46 (Backward)**: Apply impulse `(-5.0, 0, 0)` units/sec (toward paddle)
/// - **Brick 47 (Backward-Right)**: Apply impulse `(-5.0, 0, -5.0)` units/sec
/// - **Brick 48 (Backward-Left)**: Apply impulse `(-5.0, 0, +5.0)` units/sec
/// - **Brick 52 (Random)**: Apply impulse with magnitude ∈ [5.0, 15.0], direction ∈ [0, 2π) in XZ plane only (Y=0)
#[derive(Event, Debug, Clone, Copy)]
pub struct DirectionBrickEffect {
    /// Entity of the ball being affected
    pub ball_entity: Entity,
    /// Brick type identifier (43-48 for cardinal directions, 52 for random)
    pub brick_type: u8,
    /// World position of the direction brick's center
    pub brick_position: Vec3,
    /// Ball velocity before impulse application (for test validation and physics debugging)
    pub velocity_before: Vec3,
    /// Impulse vector to apply (units/sec); observer will transfer to ExternalImpulse component
    pub impulse: Vec3,
}
