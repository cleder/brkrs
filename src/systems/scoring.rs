//! Scoring system module - tracks player score and milestone progression
//!
//! This module implements the core scoring mechanics:
//! - Score state tracking across game sessions
//! - Score multiplier state for brick-destruction scoring
//! - Point awards on brick destruction
//! - Milestone detection at 5000-point intervals
//! - Event communication with other game systems

use crate::signals::BrickDestroyed;
use bevy::ecs::message::{Message, MessageReader, MessageWriter};
use bevy::prelude::*;
use rand::{rng, Rng, RngExt};

const MILESTONE_STEP: u32 = 5_000;

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

/// Global score multiplier state used for future brick-destruction score awards.
///
/// The multiplier defaults to `1x`, changes only when multiplier bricks are destroyed,
/// and resets when an actual life-loss event occurs.
#[derive(Resource, Debug, Clone, Copy)]
pub struct ScoreMultiplierState {
    /// Active multiplier factor applied to eligible future brick-destruction scores.
    pub factor: u32,

    /// Latest multiplier brick that set the active factor, if any.
    pub source_brick_index: Option<u8>,

    /// Optional diagnostic marker for the last update source.
    pub updated_frame: Option<u64>,
}

impl Default for ScoreMultiplierState {
    fn default() -> Self {
        Self {
            factor: 1,
            source_brick_index: None,
            updated_frame: None,
        }
    }
}

impl ScoreMultiplierState {
    /// Activates a new multiplier factor from a multiplier brick.
    pub fn activate(&mut self, factor: u32, source_brick_index: u8) {
        self.factor = factor;
        self.source_brick_index = Some(source_brick_index);
        self.updated_frame = None;
    }

    /// Resets multiplier state to normal scoring.
    pub fn reset(&mut self) {
        self.factor = 1;
        self.source_brick_index = None;
        self.updated_frame = None;
    }
}

// BrickDestroyed message is defined in `crate::signals`.

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
/// # Purpose
///
/// Provides authoritative point values for all destructible brick types.
/// Values are derived from the canonical brick documentation and remain
/// constant except for the Question brick which uses randomness.
///
/// # Special cases
///
/// - Question brick (53): Random 25-300 inclusive (uniform distribution)
/// - Extra Ball brick (41): 0 points (grants ball via separate logic)
/// - Magnet bricks (55-56): 0 points (effect-only)
/// - Solid bricks (90-97): Not scored (indestructible)
///
/// # Arguments
///
/// * `brick_type` - The brick type index (10-57 for destructible bricks)
/// * `rng` - Random number generator for Question brick scoring
///
/// # Returns
///
/// Point value to award (0 for non-scoring bricks)
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
        42 => 90,  // Killer (Type 42: Destructible hazard, awards 90 points)

        // Extra ball brick (no points)
        41 => 0,

        // Direction bricks
        43..=48 => 125,
        52 => 40, // Phone

        // Special bricks
        49 => 150,                        // Teleport
        50 => 0,                          // Level Up (no score, utility brick like Extra Ball)
        51 => 30,                         // Slow / Hourglass
        53 => rng.random_range(25..=300), // Question brick random score
        54 => 0,                          // Level Down (no score, utility brick)
        55 => 0,                          // Magnet (enabled)
        56 => 0,                          // Magnet (disabled)
        57 => 250,                        // Bat (paddle destroyable)

        // Indestructible / unknown bricks
        _ => 0,
    }
}

/// Returns the score multiplier factor activated by a multiplier brick.
pub fn multiplier_factor_for_brick(brick_type: u8) -> Option<u32> {
    match brick_type {
        26 => Some(1),
        27 => Some(2),
        28 => Some(3),
        29 => Some(4),
        _ => None,
    }
}

/// Awards points for destroyed bricks and updates ScoreState.
///
/// # Purpose
///
/// This system is the primary entry point for score accumulation. It listens
/// for brick destruction events and awards points based on brick type.
///
/// # When it runs
///
/// Runs in the Update schedule after `despawn_marked_entities` emits
/// `BrickDestroyed` messages. Must run before `detect_milestone_system`
/// to ensure milestone detection sees updated scores.
///
/// # Behavior
///
/// - Reads all `BrickDestroyed` messages from this frame
/// - Calls `brick_points()` to determine point value
/// - Updates `ScoreState.current_score` using saturating addition
/// - Score updates are synchronous (immediate)
///
/// # Performance
///
/// O(n) where n is the number of bricks destroyed this frame.
/// Typically 0-3 bricks per frame during normal gameplay.
pub fn award_points_system(
    mut brick_destroyed_events: MessageReader<BrickDestroyed>,
    mut score_state: ResMut<ScoreState>,
    mut multiplier_state: Option<ResMut<ScoreMultiplierState>>,
) {
    let mut rng = rng();

    for event in brick_destroyed_events.read() {
        let base_points = brick_points(event.brick_type, &mut rng);
        let points = match multiplier_state.as_mut() {
            Some(multiplier_state) => {
                if let Some(factor) = multiplier_factor_for_brick(event.brick_type) {
                    multiplier_state.activate(factor, event.brick_type);
                    base_points
                } else {
                    base_points.saturating_mul(multiplier_state.factor)
                }
            }
            None => base_points,
        };

        score_state.current_score = score_state.current_score.saturating_add(points);
    }
}

/// Resets the score multiplier after a real life-loss event occurs.
pub fn reset_multiplier_on_life_loss_system(
    life_lost_events: Option<MessageReader<crate::systems::respawn::LifeLostEvent>>,
    multiplier_state: Option<ResMut<ScoreMultiplierState>>,
) {
    let Some(mut life_lost_events) = life_lost_events else {
        return;
    };
    let Some(mut multiplier_state) = multiplier_state else {
        return;
    };

    let mut saw_life_loss = false;
    for _event in life_lost_events.read() {
        saw_life_loss = true;
    }

    if saw_life_loss {
        multiplier_state.reset();
    }
}

/// Emits milestone events when score crosses 5000-point boundaries.
///
/// # Purpose
///
/// Detects when the player's cumulative score reaches milestone thresholds
/// (5000, 10000, 15000, etc.) and emits events to trigger bonus life awards.
///
/// # When it runs
///
/// Runs in the Update schedule after `award_points_system` updates the score
/// and before `award_milestone_ball_system` processes milestone events.
///
/// # Behavior
///
/// - Calculates current milestone tier: `current_score / 5000`
/// - Compares to `last_milestone_reached` to detect new milestones
/// - Emits one `MilestoneReached` event per newly crossed threshold
/// - Updates `last_milestone_reached` to prevent duplicate events
///
/// # Example
///
/// Score increases from 4980 to 5020:
/// - Tier changes from 0 to 1
/// - Emits `MilestoneReached { milestone_tier: 1, total_score: 5020 }`
/// - Sets `last_milestone_reached = 1`
///
/// # Edge cases
///
/// Multiple milestones in one frame (e.g., 4990 → 10010) will emit
/// separate events for each crossed threshold (tier 1 and tier 2).
pub fn detect_milestone_system(
    mut score_state: ResMut<ScoreState>,
    mut milestone_events: MessageWriter<MilestoneReached>,
) {
    let reached_tier = score_state.current_score / MILESTONE_STEP;

    if reached_tier > score_state.last_milestone_reached {
        for tier in (score_state.last_milestone_reached + 1)..=reached_tier {
            milestone_events.write(MilestoneReached {
                milestone_tier: tier,
                total_score: score_state.current_score,
            });
        }
        score_state.last_milestone_reached = reached_tier;
    }
}

/// Helper to reset score state to initial values
pub fn reset_score(score_state: &mut ResMut<ScoreState>) {
    score_state.current_score = 0;
    score_state.last_milestone_reached = 0;
}

/// Helper to reset multiplier state to normal scoring.
pub fn reset_multiplier(multiplier_state: &mut ResMut<ScoreMultiplierState>) {
    multiplier_state.reset();
}
