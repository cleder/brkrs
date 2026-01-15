//! Material and fallback plumbing for the texture subsystem.
//!
//! # System Organization
//!
//! Systems are organized using the [`TextureOverrideSystems`] SystemSet enum:
//! - [`TextureOverrideSystems::Refresh`]: Refresh the registries/banks based on manifest changes
//! - [`TextureOverrideSystems::Apply`]: Apply canonical or override materials when presentation changes
//!
//! Ordering: Refresh -> Apply

use super::loader::ObjectClass;
use std::collections::{HashMap, HashSet};

use bevy::asset::AssetEvent;
use bevy::asset::RenderAssetUsages;
use bevy::ecs::message::Messages;
use bevy::image::ImageLoaderSettings;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use tracing::{debug, info, warn};

use super::loader::{TextureManifest, VisualAssetProfile};
use crate::{Ball, BallTypeId};

/// System set organization for texture overrides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum TextureOverrideSystems {
    /// Refresh the registries/banks based on manifest changes
    Refresh,
    /// Apply canonical or override materials when presentation changes
    Apply,
}

/// Registers the fallback materials used whenever a texture fails to load.
pub struct TextureMaterialsPlugin;

impl Plugin for TextureMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_fallback_registry);
        app.init_resource::<ProfileMaterialBank>();
        app.init_resource::<CanonicalMaterialHandles>();
        app.init_resource::<TypeVariantRegistry>();

        // Configure system set ordering: Refresh → Apply
        app.configure_sets(
            Update,
            TextureOverrideSystems::Refresh.before(TextureOverrideSystems::Apply),
        );

        // Refresh systems: rebuild material banks from manifest
        app.add_systems(
            Update,
            hydrate_texture_materials.in_set(TextureOverrideSystems::Refresh),
        );
        app.add_systems(
            Update,
            watch_ball_type_changes.in_set(TextureOverrideSystems::Refresh),
        );

        // Apply systems: apply materials to entities based on current banks
        app.add_systems(
            Update,
            set_texture_repeat_mode
                .in_set(TextureOverrideSystems::Apply)
                .run_if(resource_exists::<Messages<AssetEvent<Image>>>),
        );
        app.add_systems(
            Update,
            apply_canonical_materials_to_existing_entities.in_set(TextureOverrideSystems::Apply),
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
    pub merkaba: Handle<StandardMaterial>,
    warned: HashSet<String>,
}

impl FallbackRegistry {
    fn new(materials: &mut Assets<StandardMaterial>, images: &mut Assets<Image>) -> Self {
        let debug_texture = images.add(uv_debug_texture());
        Self {
            ball: add_unlit_debug(
                materials,
                debug_texture.clone(),
                Color::srgb(0.95, 0.95, 0.95),
            ),
            paddle: add_unlit_debug(
                materials,
                debug_texture.clone(),
                Color::srgb(0.82, 0.35, 0.14),
            ),
            brick: add_unlit_debug(
                materials,
                debug_texture.clone(),
                Color::srgb(0.75, 0.18, 0.18),
            ),
            sidewall: add_unlit_debug(
                materials,
                debug_texture.clone(),
                Color::srgb(0.25, 0.25, 0.35),
            ),
            ground: add_unlit_debug(
                materials,
                debug_texture.clone(),
                Color::srgb(0.18, 0.18, 0.18),
            ),
            background: add_unlit_debug(
                materials,
                debug_texture.clone(),
                Color::srgb(0.05, 0.08, 0.15),
            ),
            merkaba: add_unlit_debug(
                materials,
                debug_texture.clone(),
                Color::srgb(0.8, 0.8, 0.2), // Gold/yellow color for merkaba
            ),
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
            FallbackMaterial::Merkaba => &self.merkaba,
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
    Merkaba,
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
            BaselineMaterialKind::Merkaba => FallbackMaterial::Merkaba,
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
    Merkaba,
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
    (BaselineMaterialKind::Merkaba, "merkaba/default"),
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

        for profile in manifest.profiles.values() {
            if let Some(existing_handle) = self.handles.get(&profile.id).cloned() {
                if let Some(material) = materials.get_mut(&existing_handle) {
                    *material = make_material(profile, asset_server, None);
                    continue;
                }
            }

            let handle = bake_material(profile, asset_server, materials);
            self.handles.insert(profile.id.clone(), handle);
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
        asset_server: Option<&AssetServer>,
        mut materials: Option<&mut Assets<StandardMaterial>>,
    ) {
        self.map.clear();
        for variant in manifest.type_variants.iter() {
            let profile_id = variant.profile_id.as_str();

            // Look up the profile from the manifest to create a variant-specific material
            // with emissive color tinting applied (FR-008: emissive color × texture combination)
            let handle = if let (Some(profile), Some(asset_server), Some(materials)) = (
                manifest.profiles.get(profile_id),
                asset_server,
                materials.as_deref_mut(),
            ) {
                // Create a variant-specific material with emissive color tinting applied
                materials.add(make_material(profile, asset_server, variant.emissive_color))
            } else {
                // Fall back to canonical profile material or fallback if profile or resources not available
                bank.handle(profile_id).unwrap_or_else(|| {
                    fallback
                        .handle(match variant.object_class {
                            ObjectClass::Ball => FallbackMaterial::Ball,
                            ObjectClass::Brick => FallbackMaterial::Brick,
                            ObjectClass::Merkaba => FallbackMaterial::Merkaba,
                        })
                        .clone()
                })
            };
            self.map
                .insert((variant.object_class, variant.type_id), handle);
        }
    }

    pub fn get(&self, class: ObjectClass, type_id: u8) -> Option<Handle<StandardMaterial>> {
        self.map.get(&(class, type_id)).cloned()
    }

    pub fn insert_for_tests(
        &mut self,
        class: ObjectClass,
        type_id: u8,
        handle: Handle<StandardMaterial>,
    ) {
        self.map.insert((class, type_id), handle);
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
        debug!(
            target: "textures::materials",
            paddle = self.handles.contains_key(&BaselineMaterialKind::Paddle),
            ball = self.handles.contains_key(&BaselineMaterialKind::Ball),
            brick = self.handles.contains_key(&BaselineMaterialKind::Brick),
            "CanonicalMaterialHandles marked ready"
        );
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
    materials: Option<ResMut<Assets<StandardMaterial>>>,
    images: Option<ResMut<Assets<Image>>>,
) {
    if let (Some(materials), Some(images)) = (materials, images) {
        commands.insert_resource(FallbackRegistry::new(
            materials.into_inner(),
            images.into_inner(),
        ));
    }
}

fn set_texture_repeat_mode(
    images: Option<ResMut<Assets<Image>>>,
    mut events: bevy::ecs::message::MessageReader<AssetEvent<Image>>,
) {
    use bevy::image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor};

    // Skip if Assets<Image> resource doesn't exist (e.g., in tests)
    let Some(mut images) = images else {
        return;
    };

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
    materials: Option<ResMut<Assets<StandardMaterial>>>,
    mut bank: ResMut<ProfileMaterialBank>,
    mut canonical: ResMut<CanonicalMaterialHandles>,
    fallback: Option<ResMut<FallbackRegistry>>,
    type_variants: Option<ResMut<TypeVariantRegistry>>,
    mut hydrated: Local<bool>,
) {
    let Some(manifest) = manifest else {
        return;
    };
    let Some(asset_server) = asset_server else {
        return;
    };
    let Some(materials) = materials else {
        return;
    };
    let Some(mut fallback) = fallback else {
        return;
    };

    // Run once when manifest becomes available, even if not marked as changed
    // This handles async asset loading across all platforms consistently
    if !manifest.is_changed() && *hydrated {
        return;
    }
    *hydrated = true;

    let mut materials_mut = materials.into_inner();
    bank.rebuild(&manifest, &asset_server, materials_mut);
    if let Some(mut registry) = type_variants {
        // Rebuild variant registry with variant-specific materials (emissive color tinting applied)
        // Pass asset_server and materials so variants can have unique material instances with emissive colors
        registry.rebuild(
            &manifest,
            &bank,
            fallback.as_mut(),
            Some(&asset_server),
            Some(&mut materials_mut),
        );
        info!(
            target: "textures::materials",
            type_variants = manifest.type_variants.len(),
            registry_size = registry.map.len(),
            "TypeVariantRegistry rebuilt"
        );
    } else {
        warn!(
            target: "textures::materials",
            "TypeVariantRegistry resource missing during hydrate"
        );
    }
    canonical.sync(&bank, fallback.as_mut());
    info!(
        target: "textures::materials",
        profiles = manifest.profiles.len(),
        "Canonical materials baked"
    );
}

/// Helper function to load optional textures with specified color space settings.
/// Reduces code duplication for normal map, ORM, emissive, and depth texture loading patterns.
fn load_optional_texture(
    asset_server: &AssetServer,
    path: Option<&String>,
    is_srgb: bool,
) -> Option<Handle<Image>> {
    path.map(move |p| {
        asset_server.load_with_settings(
            manifest_asset_path(p),
            move |settings: &mut ImageLoaderSettings| settings.is_srgb = is_srgb,
        )
    })
}

fn make_material(
    profile: &VisualAssetProfile,
    asset_server: &AssetServer,
    emissive_color: Option<Color>,
) -> StandardMaterial {
    let base_color_texture = asset_server.load(manifest_asset_path(&profile.albedo_path));

    // Load normal map texture (linear color space for proper surface normals)
    let normal_map_texture = load_optional_texture(
        asset_server,
        profile.normal_path.as_ref(),
        false, // is_srgb = false (linear for normals)
    );

    // Load ORM (Occlusion-Roughness-Metallic) packed texture
    // ORM textures use linear color space (not sRGB) following glTF 2.0 standard
    let orm_texture = load_optional_texture(
        asset_server,
        profile.orm_path.as_ref(),
        false, // is_srgb = false (linear for data)
    );

    // Load emissive (glow/self-illumination) texture
    // Emissive textures use sRGB color space for proper light emission
    let emissive_texture = load_optional_texture(
        asset_server,
        profile.emissive_path.as_ref(),
        true, // is_srgb = true (sRGB for color)
    );

    // Load depth/parallax texture for surface detail
    // Depth textures use linear color space (grayscale displacement)
    // Bevy's StandardMaterial supports parallax mapping via depth_map and parallax_depth_scale
    let depth_texture = load_optional_texture(
        asset_server,
        profile.depth_path.as_ref(),
        false, // is_srgb = false (linear for depth data)
    );

    use bevy::math::Affine2;
    let uv_transform =
        Affine2::from_scale_angle_translation(profile.uv_scale, 0.0, profile.uv_offset);

    // Scalar metallic and roughness values act as multipliers for ORM texture channels
    // When ORM texture present: blue channel × metallic, green channel × roughness
    // When no ORM texture: use profile scalar values directly for solid appearance
    let metallic = profile.metallic;
    let roughness = profile.roughness;

    // Compute parallax depth scale: only apply parallax if depth texture is actually present
    // Otherwise, set to 0.0 to avoid parallax effect on materials without depth maps
    let parallax_depth_scale = if depth_texture.is_some() {
        profile.depth_scale
    } else {
        0.0
    };

    StandardMaterial {
        base_color_texture: Some(base_color_texture),
        normal_map_texture,
        // Assign ORM texture to both metallic_roughness and occlusion for dual-channel functionality
        metallic_roughness_texture: orm_texture.clone(),
        occlusion_texture: orm_texture,
        emissive_texture,
        // Emissive color acts as a multiplier/tint when combined with emissive texture
        // If variant-specific emissive_color is provided, use it; otherwise default to WHITE (1.0 multiplier)
        emissive: emissive_color
            .unwrap_or(if profile.emissive_path.is_some() {
                Color::WHITE
            } else {
                Color::BLACK
            })
            .into(),
        metallic,
        perceptual_roughness: roughness,
        depth_map: depth_texture,
        parallax_depth_scale,
        uv_transform,
        ..default()
    }
}

fn bake_material(
    profile: &VisualAssetProfile,
    asset_server: &AssetServer,
    materials: &mut Assets<StandardMaterial>,
) -> Handle<StandardMaterial> {
    materials.add(make_material(profile, asset_server, None))
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
/// This ensures paddle, balls, and bricks spawned during Startup get proper textures
/// once the manifest finishes loading.
///
/// Runs every frame but no-ops when materials not ready (internal is_ready() guard).
/// Updated count optimization prevents redundant re-applications.
fn apply_canonical_materials_to_existing_entities(
    canonical: Option<Res<CanonicalMaterialHandles>>,
    type_registry: Option<Res<TypeVariantRegistry>>,
    mut logged_missing_type_registry: Local<bool>,
    mut logged_missing_type_variant: Local<bool>,
    mut paddle_query: Query<
        &mut MeshMaterial3d<StandardMaterial>,
        (With<crate::Paddle>, Without<crate::Brick>, Without<Ball>),
    >,
    added_paddles: Query<Entity, Added<crate::Paddle>>,
    mut ball_query: Query<
        (&mut MeshMaterial3d<StandardMaterial>, &BallTypeId),
        (With<Ball>, Without<crate::Paddle>, Without<crate::Brick>),
    >,
    added_balls: Query<Entity, Added<Ball>>,
    mut brick_query: Query<
        (&mut MeshMaterial3d<StandardMaterial>, &crate::BrickTypeId),
        (With<crate::Brick>, Without<crate::Paddle>, Without<Ball>),
    >,
    added_bricks: Query<Entity, Added<crate::Brick>>,
) {
    let Some(canonical) = canonical else {
        return;
    };

    // Skip if materials not ready
    if !canonical.is_ready() {
        debug!(
            target: "textures::materials",
            "Canonical materials not ready yet"
        );
        return;
    }

    let force_update = canonical.is_changed()
        || (type_registry
            .as_ref()
            .map(|r| r.is_changed())
            .unwrap_or(false));

    if !force_update
        && added_paddles.is_empty()
        && added_balls.is_empty()
        && added_bricks.is_empty()
    {
        return;
    }

    let mut updated_count = 0;

    // Update paddle materials
    if let Some(paddle_handle) = canonical.get(BaselineMaterialKind::Paddle) {
        if force_update {
            for mut material in paddle_query.iter_mut() {
                if material.0 != paddle_handle {
                    material.0 = paddle_handle.clone();
                    updated_count += 1;
                }
            }
        } else {
            for entity in added_paddles.iter() {
                if let Ok(mut material) = paddle_query.get_mut(entity) {
                    if material.0 != paddle_handle {
                        material.0 = paddle_handle.clone();
                        updated_count += 1;
                    }
                }
            }
        }
    }

    // Update ball materials based on type
    let mut update_ball = |material: &mut MeshMaterial3d<StandardMaterial>,
                           ball_type: &BallTypeId| {
        if let Some(registry) = type_registry.as_ref() {
            if let Some(handle) = registry.get(ObjectClass::Ball, ball_type.0) {
                if material.0 != handle {
                    material.0 = handle;
                    updated_count += 1;
                }
                return;
            }
        }
        // Fall back to canonical ball material
        if let Some(ball_handle) = canonical.get(BaselineMaterialKind::Ball) {
            if material.0 != ball_handle {
                material.0 = ball_handle.clone();
                updated_count += 1;
            }
        }
    };

    if force_update {
        for (mut material, ball_type) in ball_query.iter_mut() {
            update_ball(&mut material, ball_type);
        }
    } else {
        for entity in added_balls.iter() {
            if let Ok((mut material, ball_type)) = ball_query.get_mut(entity) {
                update_ball(&mut material, ball_type);
            }
        }
    }

    // Update brick materials based on type
    let mut process_brick = |material: &mut MeshMaterial3d<StandardMaterial>,
                             brick_type: &crate::BrickTypeId| {
        if let Some(registry) = type_registry.as_ref() {
            *logged_missing_type_registry = false;
            if let Some(handle) = registry.get(ObjectClass::Brick, brick_type.0) {
                if material.0 != handle {
                    material.0 = handle;
                    updated_count += 1;
                }
                *logged_missing_type_variant = false;
                return;
            }

            if !*logged_missing_type_variant {
                debug!(
                    target: "textures::materials",
                    brick_type = brick_type.0,
                    "Type variant not found in registry for brick"
                );
                *logged_missing_type_variant = true;
            }

            if let Some(brick_handle) = canonical.get(BaselineMaterialKind::Brick) {
                if material.0 != brick_handle {
                    material.0 = brick_handle.clone();
                    updated_count += 1;
                }
            }
        } else {
            if !*logged_missing_type_registry {
                debug!(
                    target: "textures::materials",
                    "TypeVariantRegistry not available for brick material application"
                );
                *logged_missing_type_registry = true;
            }

            if let Some(brick_handle) = canonical.get(BaselineMaterialKind::Brick) {
                if material.0 != brick_handle {
                    material.0 = brick_handle.clone();
                    updated_count += 1;
                }
            } else if !*logged_missing_type_variant {
                debug!(
                    target: "textures::materials",
                    brick_type = brick_type.0,
                    "No canonical brick material available; using fallback"
                );
                *logged_missing_type_variant = true;
            }
        }
    };

    if force_update {
        for (mut material, brick_type) in brick_query.iter_mut() {
            process_brick(&mut material, brick_type);
        }
    } else {
        for entity in added_bricks.iter() {
            if let Ok((mut material, brick_type)) = brick_query.get_mut(entity) {
                process_brick(&mut material, brick_type);
            }
        }
    }

    // Log material application for debugging
    if updated_count > 0 {
        debug!(
            target: "textures::materials",
            count = updated_count,
            "Applied canonical materials to existing entities"
        );
    }
}

fn add_unlit_debug(
    materials: &mut Assets<StandardMaterial>,
    texture: Handle<Image>,
    color: Color,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: color,
        base_color_texture: Some(texture),
        unlit: true,
        ..default()
    })
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
