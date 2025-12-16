//! Scoring system module - tracks player score and milestone progression
//!
//! This module implements the core scoring mechanics:
//! - Score state tracking across game sessions
//! - Point awards on brick destruction
//! - Milestone detection at 5000-point intervals
//! - Event communication with other game systems

use bevy::ecs::message::Message;
use bevy::prelude::*;

/// Global game state tracking cumulative player score and milestone progress.
///
/// The score persists throughout a game session (across level transitions) and only
/// resets when the player starts a new game.
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct ScoreState {
    /// Total points accumulated in current game session (range: 0 to u32::MAX)
    pub current_score: u32,

    /// Highest milestone tier achieved (e.g., 0, 1, 2 for 0, 5000, 10000 points)
    pub last_milestone_reached: u32,
}

/// Domain signal that a brick was destroyed by ball collision, triggering point award.
#[derive(Message, Debug, Clone, Copy)]
pub struct BrickDestroyed {
    /// The brick entity that was destroyed
    pub brick_entity: Entity,

    /// Type/index of brick (determines point value)
    pub brick_type: u8,

    /// The ball entity that destroyed the brick (optional, for future features)
    pub destroyed_by: Option<Entity>,
}

/// Domain signal that score crossed a 5000-point threshold, triggering an extra ball/life award.
#[derive(Message, Debug, Clone, Copy)]
pub struct MilestoneReached {
    /// Which milestone was reached (1 for 5000, 2 for 10000, etc.)
    pub milestone_tier: u32,

    /// Current score when milestone triggered
    pub total_score: u32,
}
