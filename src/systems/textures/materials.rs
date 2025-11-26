//! Material and fallback plumbing for the texture subsystem.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use tracing::{info, warn};

use super::loader::{TextureManifest, VisualAssetProfile};

/// Registers the fallback materials used whenever a texture fails to load.
pub struct TextureMaterialsPlugin;

impl Plugin for TextureMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_fallback_registry);
        app.init_resource::<ProfileMaterialBank>();
        app.init_resource::<CanonicalMaterialHandles>();
        app.add_systems(Update, hydrate_texture_materials);
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

impl From<BaselineMaterialKind> for FallbackMaterial {
    fn from(value: BaselineMaterialKind) -> Self {
        match value {
            BaselineMaterialKind::Ball => FallbackMaterial::Ball,
            BaselineMaterialKind::Paddle => FallbackMaterial::Paddle,
            BaselineMaterialKind::Brick => FallbackMaterial::Brick,
            BaselineMaterialKind::Sidewall => FallbackMaterial::Sidewall,
            BaselineMaterialKind::Ground => FallbackMaterial::Ground,
            BaselineMaterialKind::Background => FallbackMaterial::Background,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaselineMaterialKind {
    Ball,
    Paddle,
    Brick,
    Sidewall,
    Ground,
    Background,
}

const BASELINE_PROFILE_IDS: &[(BaselineMaterialKind, &str)] = &[
    (BaselineMaterialKind::Ball, "ball/default"),
    (BaselineMaterialKind::Paddle, "paddle/default"),
    (BaselineMaterialKind::Brick, "brick/default"),
    (BaselineMaterialKind::Sidewall, "sidewall/default"),
    (BaselineMaterialKind::Ground, "ground/default"),
    (BaselineMaterialKind::Background, "background/default"),
];

#[derive(Resource, Default)]
pub struct ProfileMaterialBank {
    handles: HashMap<String, Handle<StandardMaterial>>,
}

impl ProfileMaterialBank {
    fn rebuild(
        &mut self,
        manifest: &TextureManifest,
        asset_server: &AssetServer,
        materials: &mut Assets<StandardMaterial>,
    ) {
        self.handles.clear();
        for profile in manifest.profiles.values() {
            let handle = bake_material(profile, asset_server, materials);
            self.handles.insert(profile.id.clone(), handle);
        }
    }

    pub fn handle(&self, profile_id: &str) -> Option<Handle<StandardMaterial>> {
        self.handles.get(profile_id).cloned()
    }
}

#[derive(Resource, Default)]
pub struct CanonicalMaterialHandles {
    handles: HashMap<BaselineMaterialKind, Handle<StandardMaterial>>,
    ready: bool,
}

impl CanonicalMaterialHandles {
    fn sync(&mut self, bank: &ProfileMaterialBank, fallback: &mut FallbackRegistry) {
        self.handles.clear();
        for (kind, profile_id) in BASELINE_PROFILE_IDS {
            let handle = bank
                .handle(profile_id)
                .unwrap_or_else(|| fallback_handle(fallback, *kind, profile_id));
            self.handles.insert(*kind, handle);
        }
        self.ready = true;
    }

    pub fn get(&self, kind: BaselineMaterialKind) -> Option<Handle<StandardMaterial>> {
        self.handles.get(&kind).cloned()
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }
}

fn initialize_fallback_registry(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(FallbackRegistry::new(materials.as_mut()));
}

fn hydrate_texture_materials(
    manifest: Option<Res<TextureManifest>>,
    asset_server: Option<Res<AssetServer>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bank: ResMut<ProfileMaterialBank>,
    mut canonical: ResMut<CanonicalMaterialHandles>,
    mut fallback: ResMut<FallbackRegistry>,
) {
    let Some(manifest) = manifest else {
        return;
    };
    let Some(asset_server) = asset_server else {
        return;
    };
    if !manifest.is_changed() {
        return;
    }
    bank.rebuild(&manifest, &asset_server, materials.as_mut());
    canonical.sync(&bank, fallback.as_mut());
    info!(
        target: "textures::materials",
        profiles = manifest.profiles.len(),
        "Canonical materials baked"
    );
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

fn bake_material(
    profile: &VisualAssetProfile,
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
) -> Handle<StandardMaterial> {
    let base_color_texture = asset_server.load(manifest_asset_path(&profile.albedo_path));
    let normal_map_texture = profile
        .normal_path
        .as_ref()
        .map(|path| asset_server.load(manifest_asset_path(path)));
    materials.add(StandardMaterial {
        base_color_texture: Some(base_color_texture),
        normal_map_texture,
        metallic: profile.metallic,
        perceptual_roughness: profile.roughness,
        ..default()
    })
}

fn manifest_asset_path(relative: &str) -> String {
    if relative.starts_with("textures/") {
        relative.to_string()
    } else {
        format!("textures/{relative}")
    }
}

fn fallback_handle(
    fallback: &mut FallbackRegistry,
    kind: BaselineMaterialKind,
    profile_id: &str,
) -> Handle<StandardMaterial> {
    fallback.log_once(format!("missing profile {profile_id}; using fallback"));
    fallback.handle(kind.into()).clone()
}
