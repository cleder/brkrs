//! Level-specific texture override pipeline.
//!
//! This module implements per-level texture overrides for ground plane,
//! background, and sidewall materials as defined in `LevelTextureSet`.

use bevy::prelude::*;
use tracing::debug;

use super::loader::TextureManifest;
use super::materials::{
    BaselineMaterialKind, CanonicalMaterialHandles, FallbackRegistry, ProfileMaterialBank,
};
use crate::{Border, GroundPlane};

/// Plugin that applies per-level texture overrides to ground/background/sidewall entities.
pub struct LevelOverridesPlugin;

impl Plugin for LevelOverridesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelPresentation>();
        app.add_systems(
            Update,
            (
                refresh_presentation_on_manifest_change,
                apply_level_overrides,
            )
                .chain(),
        );
    }
}

/// Resource tracking the current level's presentation overrides.
///
/// Updated when a new level loads, this resource caches the resolved
/// profile IDs and tint for the current level. Systems querying materials
/// for ground/background/sidewall entities use this to decide whether
/// to apply canonical defaults or level-specific overrides.
#[derive(Resource, Debug, Clone, Default)]
pub struct LevelPresentation {
    /// Current level number (0 = no level loaded)
    level_number: u32,
    /// Override profile ID for ground plane (None = use canonical)
    ground_profile: Option<String>,
    /// Override profile ID for background (None = use canonical)
    background_profile: Option<String>,
    /// Override profile ID for sidewalls (None = use canonical)
    sidewall_profile: Option<String>,
    /// Optional tint color to multiply with final material color
    tint: Option<Color>,
}

impl LevelPresentation {
    /// Construct a new LevelPresentation for the given level number.
    ///
    /// Looks up the level in the manifest's `level_overrides` map.
    /// If no override exists, all fields will be None (use canonical).
    pub fn for_level(level_number: u32, manifest: &TextureManifest) -> Self {
        let Some(level_set) = manifest.level_overrides.get(&level_number) else {
            return Self {
                level_number,
                ground_profile: None,
                background_profile: None,
                sidewall_profile: None,
                tint: None,
            };
        };

        Self {
            level_number,
            ground_profile: level_set.ground_profile.clone(),
            background_profile: level_set.background_profile.clone(),
            sidewall_profile: level_set.sidewall_profile.clone(),
            tint: level_set.tint,
        }
    }

    /// Current level number.
    pub fn level_number(&self) -> u32 {
        self.level_number
    }

    /// Override profile ID for ground plane (None = use canonical).
    pub fn ground_profile(&self) -> Option<&String> {
        self.ground_profile.as_ref()
    }

    /// Override profile ID for background (None = use canonical).
    pub fn background_profile(&self) -> Option<&String> {
        self.background_profile.as_ref()
    }

    /// Override profile ID for sidewalls (None = use canonical).
    pub fn sidewall_profile(&self) -> Option<&String> {
        self.sidewall_profile.as_ref()
    }

    /// Optional tint color modifier.
    pub fn tint(&self) -> Option<Color> {
        self.tint
    }

    /// Reset to defaults (no overrides).
    pub fn reset(&mut self) {
        self.level_number = 0;
        self.ground_profile = None;
        self.background_profile = None;
        self.sidewall_profile = None;
        self.tint = None;
    }

    /// Update this presentation from a new level's texture set.
    pub fn update_from(&mut self, level_number: u32, manifest: &TextureManifest) {
        let new = Self::for_level(level_number, manifest);
        *self = new;
    }

    /// Returns the profile ID to use for the given material kind.
    ///
    /// If an override is set for this level, returns that profile ID.
    /// Otherwise returns None (caller should use canonical profile).
    pub fn override_profile_for(&self, kind: BaselineMaterialKind) -> Option<&String> {
        match kind {
            BaselineMaterialKind::Ground => self.ground_profile.as_ref(),
            BaselineMaterialKind::Background => self.background_profile.as_ref(),
            BaselineMaterialKind::Sidewall => self.sidewall_profile.as_ref(),
            // Other kinds don't have per-level overrides
            BaselineMaterialKind::Ball
            | BaselineMaterialKind::Paddle
            | BaselineMaterialKind::Brick => None,
        }
    }

    /// Resolve the material handle for the given kind, respecting level overrides.
    ///
    /// 1. If this level has an override profile for the kind, look it up in the bank.
    /// 2. If no override or the override profile is missing, return None (caller uses canonical/fallback).
    pub fn resolve_material(
        &self,
        kind: BaselineMaterialKind,
        bank: &ProfileMaterialBank,
        mut fallback: Option<&mut FallbackRegistry>,
    ) -> Option<Handle<StandardMaterial>> {
        let profile_id = self.override_profile_for(kind)?;

        if let Some(handle) = bank.handle(profile_id) {
            return Some(handle);
        }

        // Override profile specified but not found in bank - log and use fallback
        if let Some(fb) = fallback.as_mut() {
            fb.log_once(format!(
                "level {}: override profile '{}' not found for {:?}",
                self.level_number, profile_id, kind
            ));
        }

        None
    }
}

/// System that applies per-level texture overrides when `LevelPresentation` changes.
///
/// This system watches for changes to the `LevelPresentation` resource and applies
/// override materials to ground plane, sidewall (Border), and background entities.
/// If no override is specified for a material kind, the canonical material is used.
fn apply_level_overrides(
    presentation: Res<LevelPresentation>,
    bank: Res<ProfileMaterialBank>,
    canonical: Option<Res<CanonicalMaterialHandles>>,
    mut fallback: Option<ResMut<FallbackRegistry>>,
    mut ground_query: Query<&mut MeshMaterial3d<StandardMaterial>, With<GroundPlane>>,
    mut border_query: Query<
        &mut MeshMaterial3d<StandardMaterial>,
        (With<Border>, Without<GroundPlane>),
    >,
) {
    // Only run when presentation changes
    if !presentation.is_changed() {
        return;
    }

    // Apply ground plane override
    if let Some(handle) = resolve_override_or_canonical(
        &presentation,
        BaselineMaterialKind::Ground,
        &bank,
        canonical.as_deref(),
        fallback.as_deref_mut(),
    ) {
        for mut material in ground_query.iter_mut() {
            debug!(
                target: "textures::overrides",
                level = presentation.level_number(),
                profile = ?presentation.ground_profile(),
                "Applying ground material override"
            );
            material.0 = handle.clone();
        }
    }

    // Apply sidewall (border) override
    if let Some(handle) = resolve_override_or_canonical(
        &presentation,
        BaselineMaterialKind::Sidewall,
        &bank,
        canonical.as_deref(),
        fallback.as_deref_mut(),
    ) {
        for mut material in border_query.iter_mut() {
            debug!(
                target: "textures::overrides",
                level = presentation.level_number(),
                profile = ?presentation.sidewall_profile(),
                "Applying sidewall material override"
            );
            material.0 = handle.clone();
        }
    }
}

/// Refresh LevelPresentation when TextureManifest or ProfileMaterialBank changes.
///
/// This system enables hot-reload: when an artist edits the manifest file or texture
/// assets are rebuilt, the presentation is refreshed and materials are reapplied.
fn refresh_presentation_on_manifest_change(
    manifest: Option<Res<TextureManifest>>,
    bank: Option<Res<ProfileMaterialBank>>,
    mut presentation: ResMut<LevelPresentation>,
) {
    // Check if manifest or bank changed (indicating hot-reload)
    let manifest_changed = manifest.as_ref().is_some_and(|m| m.is_changed());
    let bank_changed = bank.as_ref().is_some_and(|b| b.is_changed());

    if !manifest_changed && !bank_changed {
        return;
    }

    let level_number = presentation.level_number();
    if level_number == 0 {
        // No level loaded yet
        return;
    }

    if let Some(manifest) = manifest {
        debug!(
            target: "textures::overrides::hot_reload",
            level = level_number,
            manifest_changed,
            bank_changed,
            "Refreshing level presentation after manifest/asset change"
        );
        presentation.update_from(level_number, &manifest);
    }
}

/// Resolve the material handle for a given kind, preferring level override then canonical.
fn resolve_override_or_canonical(
    presentation: &LevelPresentation,
    kind: BaselineMaterialKind,
    bank: &ProfileMaterialBank,
    canonical: Option<&CanonicalMaterialHandles>,
    mut fallback: Option<&mut FallbackRegistry>,
) -> Option<Handle<StandardMaterial>> {
    // First try level-specific override
    if let Some(handle) = presentation.resolve_material(kind, bank, fallback.as_deref_mut()) {
        return Some(handle);
    }

    // Fall back to canonical material
    if let Some(handles) = canonical {
        return handles.get(kind);
    }

    // Last resort: use fallback registry
    fallback.map(|fb| fb.handle(kind.into()).clone())
}
