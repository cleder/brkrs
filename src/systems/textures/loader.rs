use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};

use bevy::asset::io::Reader;
use bevy::asset::{AssetEvent, AssetLoader, LoadContext};
use bevy::prelude::*;
use bevy::tasks::ConditionalSendFuture;
use ron::Value as RonValue;
use serde::Deserialize;
use tracing::{info, warn};

use super::materials::TextureMaterialsPlugin;

const TEXTURE_MANIFEST_PATH: &str = "textures/manifest.ron";

/// Plugin responsible for loading the texture manifest and related resources.
pub struct TextureManifestPlugin;

impl Plugin for TextureManifestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextureMaterialsPlugin);
        app.init_asset::<RawTextureManifest>();
        app.register_asset_loader(TextureManifestLoader::default());
        app.init_resource::<TextureManifest>();
        app.add_systems(Startup, load_texture_manifest);
        app.add_systems(Update, (hydrate_manifest_resource, log_manifest_removal));
    }
}

/// Runtime-friendly representation of the texture manifest.
#[derive(Resource, Debug, Clone, Default)]
pub struct TextureManifest {
    pub profiles: HashMap<String, VisualAssetProfile>,
    pub type_variants: Vec<TypeVariantDefinition>,
    pub level_overrides: HashMap<u32, LevelTextureSet>,
    pub level_switch: Option<LevelSwitchState>,
}

impl TextureManifest {
    fn replace_with(&mut self, raw: RawTextureManifest) {
        let RawTextureManifest {
            profiles,
            type_variants,
            level_overrides,
            level_switch,
        } = raw;
        self.profiles = profiles
            .into_iter()
            .map(|profile| (profile.id.clone(), profile))
            .collect();
        self.type_variants = type_variants;
        self.level_overrides = level_overrides
            .into_iter()
            .map(|set| (set.level_number, set))
            .collect();
        self.level_switch = level_switch;
    }
}

#[derive(Resource, Clone)]
struct TextureManifestHandle(pub Handle<RawTextureManifest>);

#[derive(Asset, TypePath, Debug, Clone, Deserialize)]
pub struct RawTextureManifest {
    pub profiles: Vec<VisualAssetProfile>,
    #[serde(default)]
    pub type_variants: Vec<TypeVariantDefinition>,
    #[serde(default)]
    pub level_overrides: Vec<LevelTextureSet>,
    #[serde(default)]
    pub level_switch: Option<LevelSwitchState>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VisualAssetProfile {
    pub id: String,
    pub albedo_path: String,
    #[serde(default)]
    pub normal_path: Option<String>,
    #[serde(default = "default_roughness")]
    pub roughness: f32,
    #[serde(default = "default_metallic")]
    pub metallic: f32,
    #[serde(default = "default_uv_scale")]
    pub uv_scale: Vec2,
    #[serde(default)]
    pub uv_offset: Vec2,
    #[serde(default)]
    pub fallback_chain: Vec<String>,
}

fn default_uv_scale() -> Vec2 {
    Vec2::splat(1.0)
}

fn default_roughness() -> f32 {
    0.5
}

fn default_metallic() -> f32 {
    0.0
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum ObjectClass {
    Ball,
    Brick,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TypeVariantDefinition {
    pub object_class: ObjectClass,
    pub type_id: u8,
    pub profile_id: String,
    #[serde(default)]
    pub emissive_color: Option<Color>,
    #[serde(default)]
    pub animation: Option<AnimationDescriptor>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnimationDescriptor {
    pub kind: String,
    #[serde(default)]
    pub params: BTreeMap<String, RonValue>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LevelTextureSet {
    pub level_number: u32,
    #[serde(default)]
    pub ground_profile: Option<String>,
    #[serde(default)]
    pub background_profile: Option<String>,
    #[serde(default)]
    pub sidewall_profile: Option<String>,
    #[serde(default)]
    pub tint: Option<Color>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LevelSwitchState {
    #[serde(default)]
    pub ordered_levels: Vec<u32>,
    #[serde(default)]
    pub current_index: usize,
    #[serde(default)]
    pub pending_switch: bool,
}

#[derive(Default)]
struct TextureManifestLoader;

impl AssetLoader for TextureManifestLoader {
    type Asset = RawTextureManifest;
    type Settings = ();
    type Error = TextureManifestLoaderError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let manifest = ron::de::from_bytes::<RawTextureManifest>(&bytes)?;
            Ok(manifest)
        }
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

#[derive(Debug)]
enum TextureManifestLoaderError {
    Io(std::io::Error),
    Ron(ron::error::SpannedError),
}

impl Display for TextureManifestLoaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "failed to read manifest: {err}"),
            Self::Ron(err) => write!(f, "failed to parse manifest: {err}"),
        }
    }
}

impl std::error::Error for TextureManifestLoaderError {}

impl From<std::io::Error> for TextureManifestLoaderError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ron::error::SpannedError> for TextureManifestLoaderError {
    fn from(value: ron::error::SpannedError) -> Self {
        Self::Ron(value)
    }
}

fn load_texture_manifest(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle = asset_server.load(TEXTURE_MANIFEST_PATH);
    info!(
        target: "textures::manifest",
        path = TEXTURE_MANIFEST_PATH,
        "Loading texture manifest"
    );
    commands.insert_resource(TextureManifestHandle(handle));
}

fn hydrate_manifest_resource(
    handle: Option<Res<TextureManifestHandle>>,
    assets: Res<Assets<RawTextureManifest>>,
    mut manifest: ResMut<TextureManifest>,
    mut events: EventReader<AssetEvent<RawTextureManifest>>,
    mut ready_once: Local<bool>,
) {
    let Some(handle) = handle else {
        return;
    };

    let asset_id = handle.0.id();
    let mut dirty = !*ready_once && assets.get(&handle.0).is_some();

    for event in events.read() {
        if event.is_added(asset_id)
            || event.is_modified(asset_id)
            || event.is_loaded_with_dependencies(asset_id)
        {
            dirty = true;
        }
    }

    if dirty {
        if let Some(raw) = assets.get(&handle.0) {
            manifest.replace_with(raw.clone());
            *ready_once = true;
            info!(
                target: "textures::manifest",
                profiles = manifest.profiles.len(),
                type_variants = manifest.type_variants.len(),
                overrides = manifest.level_overrides.len(),
                "Texture manifest hydrated"
            );
        }
    }
}

fn log_manifest_removal(
    handle: Option<Res<TextureManifestHandle>>,
    mut events: EventReader<AssetEvent<RawTextureManifest>>,
) {
    let Some(handle) = handle else {
        return;
    };
    let asset_id = handle.0.id();
    for event in events.read() {
        if event.is_removed(asset_id) {
            warn!(
                target: "textures::manifest",
                path = TEXTURE_MANIFEST_PATH,
                "Texture manifest asset removed; retaining previous data"
            );
        }
    }
}
