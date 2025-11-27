//! Material and fallback plumbing for the texture subsystem.

use super::loader::ObjectClass;
use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use tracing::{debug, info, warn};

use super::loader::{TextureManifest, VisualAssetProfile};
use crate::{Ball, BallTypeId};

/// Registers the fallback materials used whenever a texture fails to load.
pub struct TextureMaterialsPlugin;

impl Plugin for TextureMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_fallback_registry);
        app.init_resource::<ProfileMaterialBank>();
        app.init_resource::<CanonicalMaterialHandles>();
        app.init_resource::<TypeVariantRegistry>();
        app.add_systems(
            Update,
            (
                hydrate_texture_materials,
                watch_ball_type_changes,
                set_texture_repeat_mode,
                apply_canonical_materials_to_existing_entities,
            ),
        );
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

impl BaselineMaterialKind {
    fn profile_id(self) -> &'static str {
        BASELINE_PROFILE_IDS
            .iter()
            .find_map(|(kind, id)| (*kind == self).then_some(*id))
            .unwrap_or("unknown")
    }
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
        // Remove profiles that no longer exist
        self.handles
            .retain(|id, _| manifest.profiles.contains_key(id));

        // Update or create materials for each profile
        for profile in manifest.profiles.values() {
            if let Some(handle) = self.handles.get(&profile.id) {
                // Reuse existing handle and update the material in-place
                if let Some(material) = materials.get_mut(handle) {
                    update_material(material, profile, asset_server);
                }
            } else {
                // Create new material for new profile
                let handle = bake_material(profile, asset_server, materials);
                self.handles.insert(profile.id.clone(), handle);
            }
        }
    }

    pub fn handle(&self, profile_id: &str) -> Option<Handle<StandardMaterial>> {
        self.handles.get(profile_id).cloned()
    }

    /// Test helper: insert a mapping for a profile id -> material handle.
    /// This exists to make unit tests straightforward without requiring the AssetServer.
    pub fn insert_for_tests(&mut self, profile_id: &str, handle: Handle<StandardMaterial>) {
        self.handles.insert(profile_id.to_string(), handle);
    }
}

/// Registry mapping (object class + type id) -> baked material handle.
#[derive(Resource, Default, Debug)]
pub struct TypeVariantRegistry {
    map: HashMap<(ObjectClass, u8), Handle<StandardMaterial>>,
}

impl TypeVariantRegistry {
    pub fn rebuild(
        &mut self,
        manifest: &TextureManifest,
        bank: &ProfileMaterialBank,
        fallback: &mut FallbackRegistry,
    ) {
        self.map.clear();
        for variant in manifest.type_variants.iter() {
            let profile = variant.profile_id.as_str();
            let handle = bank.handle(profile).unwrap_or_else(|| {
                fallback
                    .handle(match variant.object_class {
                        ObjectClass::Ball => FallbackMaterial::Ball,
                        ObjectClass::Brick => FallbackMaterial::Brick,
                    })
                    .clone()
            });
            self.map
                .insert((variant.object_class, variant.type_id), handle);
        }
    }

    pub fn get(&self, class: ObjectClass, type_id: u8) -> Option<Handle<StandardMaterial>> {
        self.map.get(&(class, type_id)).cloned()
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

fn set_texture_repeat_mode(
    mut images: ResMut<Assets<Image>>,
    mut events: EventReader<AssetEvent<Image>>,
) {
    use bevy::image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor};

    for event in events.read() {
        if let AssetEvent::Added { id } | AssetEvent::LoadedWithDependencies { id } = event {
            if let Some(image) = images.get_mut(*id) {
                // Set the sampler to repeat mode for tiling
                image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    address_mode_w: ImageAddressMode::Repeat,
                    ..Default::default()
                });
            }
        }
    }
}

fn hydrate_texture_materials(
    manifest: Option<Res<TextureManifest>>,
    asset_server: Option<Res<AssetServer>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bank: ResMut<ProfileMaterialBank>,
    mut canonical: ResMut<CanonicalMaterialHandles>,
    mut fallback: ResMut<FallbackRegistry>,
    type_variants: Option<ResMut<TypeVariantRegistry>>,
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
    if let Some(mut registry) = type_variants {
        // Need to call rebuild with mutable fallback which consumes &mut FallbackRegistry
        registry.rebuild(&manifest, &bank, fallback.as_mut());
    }
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

fn update_material(
    material: &mut StandardMaterial,
    profile: &VisualAssetProfile,
    asset_server: &AssetServer,
) {
    material.base_color_texture =
        Some(asset_server.load(manifest_asset_path(&profile.albedo_path)));
    material.normal_map_texture = profile
        .normal_path
        .as_ref()
        .map(|path| asset_server.load(manifest_asset_path(path)));
    material.metallic = profile.metallic;
    material.perceptual_roughness = profile.roughness;

    use bevy::math::Affine2;
    material.uv_transform =
        Affine2::from_scale_angle_translation(profile.uv_scale, 0.0, profile.uv_offset);
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

    use bevy::math::Affine2;
    let uv_transform =
        Affine2::from_scale_angle_translation(profile.uv_scale, 0.0, profile.uv_offset);

    materials.add(StandardMaterial {
        base_color_texture: Some(base_color_texture),
        normal_map_texture,
        metallic: profile.metallic,
        perceptual_roughness: profile.roughness,
        uv_transform,
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

fn fallback_material_handle(
    fallback: &mut FallbackRegistry,
    kind: BaselineMaterialKind,
    warn_key: Option<String>,
) -> Handle<StandardMaterial> {
    if let Some(key) = warn_key {
        fallback.log_once(key);
    }
    fallback.handle(kind.into()).clone()
}

/// Resolve a canonical baseline material, falling back (and logging) when missing.
pub fn baseline_material_handle(
    canonical: Option<&CanonicalMaterialHandles>,
    fallback: Option<&mut FallbackRegistry>,
    kind: BaselineMaterialKind,
    warn_context: &str,
) -> Option<Handle<StandardMaterial>> {
    if let Some(handles) = canonical {
        if let Some(handle) = handles.get(kind) {
            return Some(handle);
        }
        if !handles.is_ready() {
            return fallback.map(|registry| fallback_material_handle(registry, kind, None));
        }
        return fallback.map(|registry| {
            fallback_material_handle(
                registry,
                kind,
                Some(format!(
                    "{warn_context}: missing canonical profile '{}'",
                    kind.profile_id()
                )),
            )
        });
    }
    fallback.map(|registry| {
        fallback_material_handle(
            registry,
            kind,
            Some(format!(
                "{warn_context}: canonical handles unavailable; defaulting to '{}'",
                kind.profile_id()
            )),
        )
    })
}

/// System that watches for ball type changes and swaps materials accordingly.
/// Runs every frame and checks for balls whose `BallTypeId` changed since last frame.
fn watch_ball_type_changes(
    mut balls: Query<
        (&BallTypeId, &mut MeshMaterial3d<StandardMaterial>),
        (With<Ball>, Changed<BallTypeId>),
    >,
    type_registry: Res<TypeVariantRegistry>,
    fallback: Option<Res<FallbackRegistry>>,
) {
    for (ball_type, mut material) in balls.iter_mut() {
        if let Some(handle) = type_registry.get(ObjectClass::Ball, ball_type.0) {
            debug!(
                target: "textures::ball_type",
                type_id = ball_type.0,
                "Swapping ball material for type variant"
            );
            material.0 = handle;
        } else if let Some(fb) = &fallback {
            debug!(
                target: "textures::ball_type",
                type_id = ball_type.0,
                "No type variant for ball; using fallback"
            );
            material.0 = fb.ball.clone();
        }
    }
}

/// Resolve a type-variant material for a brick, falling back if not found.
pub fn brick_type_material_handle(
    type_registry: Option<&TypeVariantRegistry>,
    fallback: Option<&mut FallbackRegistry>,
    brick_type: u8,
    warn_context: &str,
) -> Option<Handle<StandardMaterial>> {
    if let Some(registry) = type_registry {
        if let Some(handle) = registry.get(ObjectClass::Brick, brick_type) {
            return Some(handle);
        }
    }
    // Type variant not found, use canonical brick or fallback
    fallback.map(|fb| {
        fb.log_once(format!(
            "{warn_context}: no type variant for brick type {brick_type}"
        ));
        fb.brick.clone()
    })
}

/// Apply canonical materials to existing entities when they become available.
/// This ensures paddle and bricks spawned during Startup get proper textures
/// once the manifest finishes loading.
fn apply_canonical_materials_to_existing_entities(
    canonical: Option<Res<CanonicalMaterialHandles>>,
    type_registry: Option<Res<TypeVariantRegistry>>,
    mut paddle_query: Query<
        &mut MeshMaterial3d<StandardMaterial>,
        (With<crate::Paddle>, Without<crate::Brick>),
    >,
    mut brick_query: Query<
        (&mut MeshMaterial3d<StandardMaterial>, &crate::BrickTypeId),
        (With<crate::Brick>, Without<crate::Paddle>),
    >,
) {
    let Some(canonical) = canonical else {
        return;
    };

    // Only run once when canonical materials first become ready
    if !canonical.is_changed() || !canonical.is_ready() {
        return;
    }

    // Update paddle materials
    if let Some(paddle_handle) = canonical.get(BaselineMaterialKind::Paddle) {
        for mut material in paddle_query.iter_mut() {
            material.0 = paddle_handle.clone();
        }
    }

    // Update brick materials based on type
    for (mut material, brick_type) in brick_query.iter_mut() {
        if let Some(registry) = type_registry.as_ref() {
            if let Some(handle) = registry.get(ObjectClass::Brick, brick_type.0) {
                material.0 = handle;
                continue;
            }
        }
        // Fall back to canonical brick material
        if let Some(brick_handle) = canonical.get(BaselineMaterialKind::Brick) {
            material.0 = brick_handle.clone();
        }
    }
}
