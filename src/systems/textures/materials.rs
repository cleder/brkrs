//! Material and fallback plumbing for the texture subsystem.

use std::collections::HashSet;

use bevy::prelude::*;
use tracing::warn;

/// Registers the fallback materials used whenever a texture fails to load.
pub struct TextureMaterialsPlugin;

impl Plugin for TextureMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_fallback_registry);
    }
}

/// Canonical handles that always exist, ensuring meshes never render untextured.
#[derive(Resource, Debug)]
pub struct FallbackRegistry {
    pub ball: Handle<StandardMaterial>,
    pub paddle: Handle<StandardMaterial>,
    pub brick: Handle<StandardMaterial>,
    pub sidewall: Handle<StandardMaterial>,
    pub ground: Handle<StandardMaterial>,
    pub background: Handle<StandardMaterial>,
    warned: HashSet<String>,
}

impl FallbackRegistry {
    fn new(materials: &mut Assets<StandardMaterial>) -> Self {
        Self {
            ball: add_unlit_color(materials, Color::srgb(0.95, 0.95, 0.95)),
            paddle: add_unlit_color(materials, Color::srgb(0.82, 0.35, 0.14)),
            brick: add_unlit_color(materials, Color::srgb(0.75, 0.18, 0.18)),
            sidewall: add_unlit_color(materials, Color::srgb(0.25, 0.25, 0.35)),
            ground: add_unlit_color(materials, Color::srgb(0.18, 0.18, 0.18)),
            background: add_unlit_color(materials, Color::srgb(0.05, 0.08, 0.15)),
            warned: HashSet::new(),
        }
    }

    /// Returns the handle for a fallback material.
    pub fn handle(&self, kind: FallbackMaterial) -> &Handle<StandardMaterial> {
        match kind {
            FallbackMaterial::Ball => &self.ball,
            FallbackMaterial::Paddle => &self.paddle,
            FallbackMaterial::Brick => &self.brick,
            FallbackMaterial::Sidewall => &self.sidewall,
            FallbackMaterial::Ground => &self.ground,
            FallbackMaterial::Background => &self.background,
        }
    }

    /// Emits a warning the first time a fallback is used for a given identifier.
    pub fn log_once(&mut self, id: impl Into<String>) -> bool {
        let key = id.into();
        if self.warned.insert(key.clone()) {
            warn!(
                target: "textures::fallback",
                missing = %key,
                "Missing texture reference; using fallback material"
            );
            true
        } else {
            false
        }
    }
}

/// Enumeration of available fallback material buckets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FallbackMaterial {
    Ball,
    Paddle,
    Brick,
    Sidewall,
    Ground,
    Background,
}

fn initialize_fallback_registry(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(FallbackRegistry::new(materials.as_mut()));
}

fn add_unlit_color(
    materials: &mut Assets<StandardMaterial>,
    color: Color,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: color,
        unlit: true,
        ..default()
    })
}
