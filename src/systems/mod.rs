/// Game systems module
///
/// Contains all game system implementations organized by functionality
pub mod audio;
pub mod brick_decals;
pub mod cheat_mode;
pub mod grid_debug;
pub mod level_switch;
pub mod multi_hit;
pub mod paddle_size;
pub mod respawn;
pub mod scoring;
pub mod sets;
pub mod spawning;
pub mod textures;

pub use audio::{AudioConfig, AudioPlugin, BallWallHit, LevelCompleted, LevelStarted, SoundType};
pub use brick_decals::{assign_brick_decals, assign_brick_decals_fallback};
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
