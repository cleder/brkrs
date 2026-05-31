/// Game systems module
///
/// Contains all game system implementations organized by functionality
pub mod audio;
pub mod ball_spawn_bricks;
pub mod brick_effects;
pub mod cheat_mode;
pub mod collision_feedback;
pub mod game_state_transitions;
pub mod gravity;
pub mod grid_debug;
pub mod level_switch;
pub mod merkaba;
pub mod multi_hit;
pub mod paddle_size;
pub mod respawn;
pub mod scoring;
pub mod sets;
pub mod spawning;
pub mod textures;
pub mod ui {
    pub mod game_over;
    pub mod main_menu;
}

pub use crate::signals::BallWallHit;
pub use audio::{AudioConfig, AudioPlugin, LevelCompleted, LevelStarted, SoundType};
pub use ball_spawn_bricks::BallSpawnBricksPlugin;
pub use brick_effects::{
    BRICK_TYPE_DIRECTION_BACKWARD, BRICK_TYPE_DIRECTION_BACKWARD_LEFT,
    BRICK_TYPE_DIRECTION_BACKWARD_RIGHT, BRICK_TYPE_DIRECTION_FORWARD, BRICK_TYPE_DIRECTION_LEFT,
    BRICK_TYPE_DIRECTION_RANDOM, BRICK_TYPE_DIRECTION_RIGHT, IMPULSE_MAGNITUDE_CARDINAL,
    IMPULSE_MAGNITUDE_RANDOM_MAX, IMPULSE_MAGNITUDE_RANDOM_MIN,
};
pub use cheat_mode::{CheatModePlugin, CheatModeState, CheatModeToggled};
pub use collision_feedback::{CollisionFeedbackParticle, FeedbackEffectInstance, FeedbackProfile};
pub use gravity::GravityChanged;
pub use level_switch::{
    LevelSwitchPlugin, LevelSwitchRequested, LevelSwitchSource, LevelSwitchState,
};
pub use merkaba::{MerkabaPlugin, PendingMerkabaSpawn, PendingMerkabaSpawns};
pub use multi_hit::MultiHitBrickHit;
pub use paddle_size::{
    PaddleSizeEffect, PaddleSizeEffectApplied, PaddleSizePlugin, SizeEffectType,
};
pub use respawn::{InputLocked, RespawnPlugin, RespawnSystems};
pub use textures::TextureManifestPlugin;
