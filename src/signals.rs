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
//! # Available Signals
//!
//! - [`UiBeep`]: Short audio feedback cue from UI interactions
//! - [`BrickDestroyed`]: Brick destruction event for scoring and audio
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
use bevy::prelude::Entity;

/// Short UI feedback cue (beep) — buffered message consumed by audio systems.
///
/// **Producers**: UI systems (pause menu, ball loss, game over)
/// **Consumers**: Audio system [`crate::systems::audio::consume_ui_beep_messages`]
/// **Contract**: One beep per user interaction (no duplicates within same frame)
#[derive(Message, Debug, Clone, Copy)]
pub struct UiBeep;

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
    /// Entity that caused destruction (ball, paddle, etc.) or None for despawn
    pub destroyed_by: Option<Entity>,
}
