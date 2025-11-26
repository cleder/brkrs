//! Level-specific texture override pipeline placeholder.

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct LevelPresentation;

impl LevelPresentation {
    pub fn reset(&mut self) {
        // Future work: track per-level ground/background/sidewall material overrides.
    }
}
