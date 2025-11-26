//! Textures module scaffolding
//!
//! This tree will eventually manage manifest loading, fallback materials,
//! and per-level overrides as described in specs/001-textured-visuals.

pub mod contracts;
pub mod loader;
pub mod materials;
pub mod overrides;

pub use contracts::TextureManifestContract;
pub use loader::{
    LevelSwitchState, LevelTextureSet, TextureManifest, TextureManifestPlugin,
    TypeVariantDefinition, VisualAssetProfile,
};
pub use materials::{
    baseline_material_handle, BaselineMaterialKind, CanonicalMaterialHandles, FallbackMaterial,
    FallbackRegistry, ProfileMaterialBank, TextureMaterialsPlugin, TypeVariantRegistry,
};
