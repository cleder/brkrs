use std::collections::BTreeMap;

use bevy::prelude::{Color, Event, Vec2};
use ron::Value as RonValue;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use super::loader::{
    AnimationDescriptor, LevelSwitchState, LevelTextureSet, ObjectClass, TextureManifest,
    TypeVariantDefinition, VisualAssetProfile,
};

/// Serializable view of the runtime texture manifest that matches the
/// `/visual-assets/manifest` contract.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextureManifestContract {
    pub profiles: Vec<VisualAssetProfileContract>,
    pub type_variants: Vec<TypeVariantContract>,
    pub level_overrides: Vec<LevelTextureSetContract>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_switch: Option<LevelSwitchStateContract>,
}

impl From<&TextureManifest> for TextureManifestContract {
    fn from(manifest: &TextureManifest) -> Self {
        let mut profiles: Vec<_> = manifest
            .profiles
            .values()
            .map(VisualAssetProfileContract::from)
            .collect();
        profiles.sort_by(|a, b| a.id.cmp(&b.id));

        let mut level_overrides: Vec<_> = manifest
            .level_overrides
            .values()
            .map(LevelTextureSetContract::from)
            .collect();
        level_overrides.sort_by(|a, b| a.level_number.cmp(&b.level_number));

        let mut type_variants: Vec<_> = manifest
            .type_variants
            .iter()
            .map(TypeVariantContract::from)
            .collect();
        type_variants.sort_by(|a, b| {
            a.object_class
                .cmp(&b.object_class)
                .then(a.type_id.cmp(&b.type_id))
        });

        Self {
            profiles,
            type_variants,
            level_overrides,
            level_switch: manifest
                .level_switch
                .as_ref()
                .map(LevelSwitchStateContract::from),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VisualAssetProfileContract {
    pub id: String,
    pub albedo_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_path: Option<String>,
    pub roughness: f32,
    pub metallic: f32,
    pub uv_scale: [f32; 2],
    pub uv_offset: [f32; 2],
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fallback_chain: Vec<String>,
}

impl From<&VisualAssetProfile> for VisualAssetProfileContract {
    fn from(profile: &VisualAssetProfile) -> Self {
        Self {
            id: profile.id.clone(),
            albedo_path: profile.albedo_path.clone(),
            normal_path: profile.normal_path.clone(),
            roughness: profile.roughness,
            metallic: profile.metallic,
            uv_scale: vec2_to_array(profile.uv_scale),
            uv_offset: vec2_to_array(profile.uv_offset),
            fallback_chain: profile.fallback_chain.clone(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TypeVariantContract {
    pub object_class: String,
    pub type_id: u8,
    pub profile_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissive_color: Option<[f32; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<AnimationDescriptorContract>,
}

impl From<&TypeVariantDefinition> for TypeVariantContract {
    fn from(variant: &TypeVariantDefinition) -> Self {
        Self {
            object_class: object_class_label(variant.object_class),
            type_id: variant.type_id,
            profile_id: variant.profile_id.clone(),
            emissive_color: variant.emissive_color.map(color_to_rgba),
            animation: variant
                .animation
                .as_ref()
                .map(AnimationDescriptorContract::from),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnimationDescriptorContract {
    pub kind: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub params: BTreeMap<String, JsonValue>,
}

impl From<&AnimationDescriptor> for AnimationDescriptorContract {
    fn from(descriptor: &AnimationDescriptor) -> Self {
        Self {
            kind: descriptor.kind.clone(),
            params: descriptor
                .params
                .iter()
                .map(|(key, value)| (key.clone(), ron_to_json(value)))
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LevelTextureSetContract {
    pub level_number: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sidewall_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint: Option<[f32; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl From<&LevelTextureSet> for LevelTextureSetContract {
    fn from(set: &LevelTextureSet) -> Self {
        Self {
            level_number: set.level_number,
            ground_profile: set.ground_profile.clone(),
            background_profile: set.background_profile.clone(),
            sidewall_profile: set.sidewall_profile.clone(),
            tint: set.tint.map(color_to_rgba),
            notes: set.notes.clone(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LevelSwitchStateContract {
    pub ordered_levels: Vec<u32>,
    pub current_index: usize,
    pub pending_switch: bool,
}

impl From<&LevelSwitchState> for LevelSwitchStateContract {
    fn from(state: &LevelSwitchState) -> Self {
        Self {
            ordered_levels: state.ordered_levels.clone(),
            current_index: state.current_index,
            pending_switch: state.pending_switch,
        }
    }
}

fn vec2_to_array(value: Vec2) -> [f32; 2] {
    value.to_array()
}

fn color_to_rgba(color: Color) -> [f32; 4] {
    let linear = color.to_linear();
    [linear.red, linear.green, linear.blue, linear.alpha]
}

fn object_class_label(class: ObjectClass) -> String {
    match class {
        ObjectClass::Ball => "Ball",
        ObjectClass::Brick => "Brick",
    }
    .to_string()
}

fn ron_to_json(value: &RonValue) -> JsonValue {
    serde_json::to_value(value).unwrap_or(JsonValue::Null)
}

// ============================================================================
// Preview Asset Tooling Hook
// ============================================================================

/// Event to request a temporary texture profile preview.
///
/// Implements the `/visual-assets/preview` contract for tooling.
/// When received, the system temporarily injects the profile into the manifest
/// so artists can preview new art without rebuilding the game.
#[derive(Event, Debug, Clone)]
pub struct PreviewVisualAsset {
    /// The profile to preview. Must include a valid `id` field.
    pub profile: VisualAssetProfileContract,
    /// If true, the preview persists until cleared; otherwise, it's cleared on next level load.
    pub persist: bool,
}

/// Contract-friendly input for creating a preview profile from JSON.
///
/// This mirrors `VisualAssetProfile` but uses contract-compatible field names
/// and can be deserialized from JSON payloads.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PreviewProfileInput {
    pub id: String,
    pub albedo_path: String,
    #[serde(default)]
    pub normal_path: Option<String>,
    #[serde(default = "default_roughness")]
    pub roughness: f32,
    #[serde(default)]
    pub metallic: f32,
    #[serde(default = "default_uv_scale")]
    pub uv_scale: [f32; 2],
    #[serde(default)]
    pub uv_offset: [f32; 2],
    #[serde(default)]
    pub fallback_chain: Vec<String>,
}

fn default_roughness() -> f32 {
    0.5
}

fn default_uv_scale() -> [f32; 2] {
    [1.0, 1.0]
}

impl From<PreviewProfileInput> for VisualAssetProfileContract {
    fn from(input: PreviewProfileInput) -> Self {
        Self {
            id: input.id,
            albedo_path: input.albedo_path,
            normal_path: input.normal_path,
            roughness: input.roughness,
            metallic: input.metallic,
            uv_scale: input.uv_scale,
            uv_offset: input.uv_offset,
            fallback_chain: input.fallback_chain,
        }
    }
}

impl From<PreviewProfileInput> for VisualAssetProfile {
    fn from(input: PreviewProfileInput) -> Self {
        Self {
            id: input.id,
            albedo_path: input.albedo_path,
            normal_path: input.normal_path,
            roughness: input.roughness,
            metallic: input.metallic,
            uv_scale: Vec2::from_array(input.uv_scale),
            uv_offset: Vec2::from_array(input.uv_offset),
            fallback_chain: input.fallback_chain,
        }
    }
}
