//! UI module for the game
//!
//! Contains UI components including the pause overlay.
//!
//! # Architecture
//!
//! UI systems in this module follow Bevy 0.17 best practices:
//! - All systems return `Result<(), UiSystemError>` for fallible execution.
//! - Queries use `?` operator for early returns on expected failures.
//! - UI updates are change-driven (`Changed<T>`) rather than per-frame.
//! - Assets are loaded once in startup systems and stored in resources.
//! - Systems are organized into system sets for reusability and parallelism.

use std::fmt;

/// Error type for UI systems.
///
/// Encapsulates failures in UI setup, entity lookup, or state updates.
/// All UI systems return `Result<(), UiSystemError>` and propagate errors with `?`.
#[derive(Debug, Clone)]
pub enum UiSystemError {
    /// A required UI entity was not found (e.g., `single()` or `single_mut()` failed).
    EntityNotFound(String),
    /// A required resource was not available.
    ResourceNotFound(String),
    /// A UI asset failed to load or was unavailable.
    AssetNotAvailable(String),
    /// A configuration or setup error in UI initialization.
    SetupError(String),
    /// Other errors.
    Other(String),
}

impl fmt::Display for UiSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EntityNotFound(msg) => write!(f, "UI entity not found: {}", msg),
            Self::ResourceNotFound(msg) => write!(f, "UI resource not found: {}", msg),
            Self::AssetNotAvailable(msg) => write!(f, "UI asset not available: {}", msg),
            Self::SetupError(msg) => write!(f, "UI setup error: {}", msg),
            Self::Other(msg) => write!(f, "UI error: {}", msg),
        }
    }
}

impl std::error::Error for UiSystemError {}

pub mod cheat_indicator;
pub mod fonts;
pub mod game_over_overlay;
pub mod level_label;
pub mod lives_counter;
pub mod palette;
pub mod pause_overlay;
pub mod score_display;
