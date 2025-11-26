//! Textures module scaffolding
//!
//! This tree will eventually manage manifest loading, fallback materials,
//! and per-level overrides as described in specs/001-textured-visuals.

pub mod contracts;
pub mod loader;
pub mod materials;
pub mod overrides;

pub use contracts::{PreviewProfileInput, PreviewVisualAsset, TextureManifestContract};
pub use loader::{
    LevelSwitchState, LevelTextureSet, ObjectClass, TextureManifest, TextureManifestPlugin,
    TypeVariantDefinition, VisualAssetProfile,
};
pub use materials::{
    baseline_material_handle, brick_type_material_handle, BaselineMaterialKind,
    CanonicalMaterialHandles, FallbackMaterial, FallbackRegistry, ProfileMaterialBank,
    TextureMaterialsPlugin, TypeVariantRegistry,
};
pub use overrides::{LevelOverridesPlugin, LevelPresentation};
