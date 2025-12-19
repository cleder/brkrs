//! Game systems module
//!
//! Contains all game system implementations organized by functionality.
//!
//! # Fallible Systems Pattern
//!
//! All systems in this module follow the Constitution mandate for fallible systems
//! (Section VIII: Bevy 0.17 Mandates & Prohibitions). Every system function returns
//! `Result<(), Box<dyn std::error::Error>>` to enable safe error propagation using
//! the `?` operator.
//!
//! ## Expected Failure Pattern
//!
//! When a system encounters an expected failure (e.g., missing optional resource,
//! empty query for optional entities), it should return early with `Ok(())`:
//!
//! ```ignore
//! fn my_system(query: Query<&Component>) -> Result<(), Box<dyn std::error::Error>> {
//!     // Early return on empty query (expected when no entities exist)
//!     if query.is_empty() {
//!         return Ok(());
//!     }
//!     
//!     // Use ? for unexpected failures (component should exist)
//!     let component = query.get_single()?;
//!     
//!     // ... system logic
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Query Error Handling
//!
//! - **0 entities**: Use `query.is_empty()` check + early `Ok(())` return
//! - **1 entity (required)**: Use `query.get_single()` + `?` operator
//! - **1 entity (optional)**: Use `if let Ok(entity) = query.get_single() { ... }`
//! - **Many entities**: Use `query.iter()` (never fails)
//!
//! ## Resource Error Handling
//!
//! - **Required resource**: System parameter `Res<T>` (panics if missing - use sparingly)
//! - **Optional resource**: System parameter `Option<Res<T>>` + early return if None
//! - **Conditionally available**: Use `Option<Res<T>>` and handle None gracefully
//!
//! ## Error Type Strategy
//!
//! Systems use `Result<(), Box<dyn std::error::Error>>` as the standard return type.
//! This provides:
//! - Compatibility with `?` operator for any error type
//! - Flexibility for different error sources
//! - Clear error propagation
//! - Minimal boilerplate
//!
//! Custom error types can be added later if needed for more specific error handling.

pub mod audio;
pub mod cheat_mode;
pub mod grid_debug;
pub mod level_switch;
pub mod multi_hit;
pub mod paddle_size;
pub mod respawn;
pub mod scoring;
pub mod textures;

pub use audio::{
    AudioConfig, AudioPlugin, BallWallHit, BrickDestroyed, LevelCompleted, LevelStarted, SoundType,
};
pub use cheat_mode::{CheatModePlugin, CheatModeState, CheatModeToggled};
pub use level_switch::{
    LevelSwitchPlugin, LevelSwitchRequested, LevelSwitchSource, LevelSwitchState,
};
pub use multi_hit::MultiHitBrickHit;
pub use paddle_size::{
    PaddleSizeEffect, PaddleSizeEffectApplied, PaddleSizePlugin, SizeEffectType,
};
pub use respawn::{InputLocked, RespawnPlugin, RespawnSystems};
pub use textures::TextureManifestPlugin;
