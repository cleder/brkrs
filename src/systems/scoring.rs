//! Scoring system module - tracks player score and milestone progression
//!
//! This module implements the core scoring mechanics:
//! - Score state tracking across game sessions
//! - Point awards on brick destruction
//! - Milestone detection at 5000-point intervals
//! - Event communication with other game systems

use bevy::ecs::message::{Message, MessageReader};
use bevy::prelude::*;
use rand::{rng, Rng};

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

/// Map brick type/index to point value per docs/bricks.md.
///
/// Special cases:
/// - Question brick (53): Random 25-300 inclusive
/// - Extra Ball brick (41): 0 points (grants ball via separate logic)
/// - Magnet bricks (55-56): 0 points
/// - Solid bricks (90-97): Not scored (0)
pub fn brick_points(brick_type: u8, rng: &mut impl Rng) -> u32 {
    match brick_type {
        // Multi-hit bricks
        10..=13 => 50,

        // Simple stone
        20 => 25,

        // Gravity bricks
        21 => 125,
        22 => 75,
        23 => 125,
        24 => 150,
        25 => 250,

        // Score multipliers (effect out-of-scope; award base points)
        26..=29 => 25,

        // Paddle effect bricks
        30 => 300, // Apple
        31 => 200, // Sun
        32 => 225, // Yin Yang

        // Ball size bricks
        33 => 25,
        34 => 25,
        35 => 25,

        // Enemy spawn bricks
        36 => 75, // Donut / Rotor

        // Ball spawn bricks
        37 => 100, // Red 1
        38 => 100, // Red 2
        39 => 100, // Red 3

        // Hazard bricks
        40 => 100, // Bomb
        42 => 90,  // Killer

        // Extra ball brick (no points)
        41 => 0,

        // Direction bricks
        43..=48 => 125,
        52 => 40, // Phone

        // Special bricks
        49 => 150,                        // Teleport
        50 => 300,                        // Level Up
        51 => 30,                         // Slow / Hourglass
        53 => rng.random_range(25..=300), // Question brick random score
        54 => 0,                          // Level Down (no score)
        55 => 0,                          // Magnet (enabled)
        56 => 0,                          // Magnet (disabled)
        57 => 250,                        // Bat (paddle destroyable)

        // Indestructible / unknown bricks
        _ => 0,
    }
}

/// Awards points for destroyed bricks and updates ScoreState.
pub fn award_points_system(
    mut brick_destroyed_events: MessageReader<BrickDestroyed>,
    mut score_state: ResMut<ScoreState>,
) {
    let mut rng = rng();

    for event in brick_destroyed_events.read() {
        let points = brick_points(event.brick_type, &mut rng);
        score_state.current_score = score_state.current_score.saturating_add(points);
    }
}
