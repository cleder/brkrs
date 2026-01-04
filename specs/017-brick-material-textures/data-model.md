# Data Model: Enhanced Brick Material Textures

**Feature**: 017-brick-material-textures **Date**: 2026-01-04 **Phase**: 1 (Design)

## Overview

This feature extends the existing `VisualAssetProfile` data structure to support three additional PBR texture types: packed ORM (Occlusion-Roughness-Metallic), emissive, and depth maps.
All extensions maintain backward compatibility through optional fields.

## Core Entities

### VisualAssetProfile (Extended)

**Purpose**: Defines a complete material profile including all PBR texture maps

**Location**: `src/systems/textures/loader.rs`

**Current Structure**:

```rust
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
```

**Extended Structure** (new fields marked with `// NEW`):

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct VisualAssetProfile {
    pub id: String,
    pub albedo_path: String,
    #[serde(default)]
    pub normal_path: Option<String>,

    // NEW: Packed ORM texture (Occlusion-Roughness-Metallic, glTF 2.0 format)
    #[serde(default)]
    pub orm_path: Option<String>,

    // NEW: Emissive/glow map texture
    #[serde(default)]
    pub emissive_path: Option<String>,

    // NEW: Depth/parallax map texture
    #[serde(default)]
    pub depth_path: Option<String>,

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
```

**Field Semantics**:

- `orm_path: Option<String>` - Path to packed ORM texture (red=occlusion, green=roughness, blue=metallic) following glTF 2.0 standard.
  If `None`, uses scalar `roughness` and `metallic` values with no occlusion.
- `emissive_path: Option<String>` - Path to emissive/glow map texture.
  If `None`, uses solid `emissive_color` from TypeVariantDefinition (if set) or no emission.
- `depth_path: Option<String>` - Path to depth/parallax map texture for parallax occlusion mapping.
  If `None`, no parallax effect.

**Validation Rules**:

- All paths are relative to `assets/textures/` directory
- Paths can optionally start with `textures/` prefix (normalized in code)
- Texture files must be in PNG or KTX2 format (Bevy compatible)
- All new fields are optional (`#[serde(default)]`) for backward compatibility
- Scalar `roughness` and `metallic` act as multipliers/base values when ORM texture is present

**State Transitions**: None (immutable after deserialization from manifest)

---

### VisualAssetProfileContract (Extended)

**Purpose**: API contract definition for tooling and external validation

**Location**: `src/systems/textures/contracts.rs`

**Extended Structure**:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualAssetProfileContract {
    pub id: String,
    pub albedo_path: String,
    pub normal_path: Option<String>,

    // NEW: Packed ORM texture
    pub orm_path: Option<String>,

    // NEW: Emissive map texture
    pub emissive_path: Option<String>,

    // NEW: Depth map texture
    pub depth_path: Option<String>,

    pub roughness: f32,
    pub metallic: f32,
    pub uv_scale: [f32; 2],
    pub uv_offset: [f32; 2],
    pub fallback_chain: Vec<String>,
}
```

**Relationship to VisualAssetProfile**: Direct mapping for API compatibility; converts Vec2 to/from `[f32; 2]` arrays for serialization.

---

### StandardMaterial (Bevy Built-in, Extended Usage)

**Purpose**: Bevy's built-in PBR material type that receives the loaded textures

**Location**: `bevy::pbr::StandardMaterial` (external dependency)

**Relevant Fields**:

```rust
pub struct StandardMaterial {
    pub base_color_texture: Option<Handle<Image>>,      // Existing: albedo
    pub normal_map_texture: Option<Handle<Image>>,      // Existing: normal

    // NEWLY USED: ORM texture assigned to both fields
    pub metallic_roughness_texture: Option<Handle<Image>>,
    pub occlusion_texture: Option<Handle<Image>>,

    // NEWLY USED: Emissive map
    pub emissive_texture: Option<Handle<Image>>,

    // NEWLY USED: Depth/parallax map
    pub depth_map: Option<Handle<Image>>,

    pub metallic: f32,                   // Scalar multiplier for blue channel of ORM
    pub perceptual_roughness: f32,       // Scalar multiplier for green channel of ORM
    pub uv_transform: Affine2,           // Applied to ALL texture maps
    // ... other fields not modified
}
```

**Field Assignment Strategy**:

- `metallic_roughness_texture` and `occlusion_texture` both receive the same ORM texture handle
- Bevy's shader automatically extracts the correct channel from each field
- `uv_transform` applied to all texture maps for alignment

---

## Data Flow

### Manifest Loading Pipeline

```text
1. Asset Loading
   assets/textures/manifest.ron
   ↓
   [RawTextureManifest] (deserialized via RON)
   ↓
   [TextureManifest resource] (hydrated in Update schedule)

2. Material Building (when manifest changes)
   TextureManifest.profiles: HashMap<String, VisualAssetProfile>
   ↓
   ProfileMaterialBank::rebuild()
   ↓
   For each profile:
     - make_material(profile, asset_server) → StandardMaterial
     - materials.add(material) → Handle<StandardMaterial>
     - Store handle in ProfileMaterialBank

3. Material Assignment
   TypeVariantDefinition.profile_id
   ↓
   ProfileMaterialBank.handle(profile_id)
   ↓
   MeshMaterial3d<StandardMaterial> component on entity
```

### Texture Loading Within make_material()

```text
VisualAssetProfile
↓
Load albedo_path → base_color_texture
Load normal_path (if Some) → normal_map_texture with is_srgb=false
Load orm_path (if Some) → metallic_roughness_texture AND occlusion_texture (both linear)
Load emissive_path (if Some) → emissive_texture (sRGB)
Load depth_path (if Some) → depth_map (linear)
↓
Create StandardMaterial with all texture handles
Apply uv_transform from profile (scale + offset)
↓
Return StandardMaterial
```

### Color Space Settings

| Texture Type | Field | Color Space | Rationale |
|--------------|-------|-------------|-----------|
| Albedo | `base_color_texture` | sRGB (default) | Color data |
| Normal | `normal_map_texture` | Linear | Normal vector data |
| ORM | `metallic_roughness_texture` | Linear | PBR parameter data |
| ORM | `occlusion_texture` | Linear | Same texture, different field |
| Emissive | `emissive_texture` | sRGB (default) | Color data |
| Depth | `depth_map` | Linear | Height/displacement data |

---

## Relationships

### VisualAssetProfile → StandardMaterial

**Cardinality**: 1:1 (one profile produces one material)

**Mapping**:

```text
VisualAssetProfile                   StandardMaterial
--------------------------------------------------
albedo_path                    →     base_color_texture
normal_path                    →     normal_map_texture
orm_path                       →     metallic_roughness_texture
orm_path (same texture)        →     occlusion_texture
emissive_path                  →     emissive_texture
depth_path                     →     depth_map
roughness                      →     perceptual_roughness
metallic                       →     metallic
uv_scale + uv_offset           →     uv_transform
```

### TypeVariantDefinition → VisualAssetProfile

**Cardinality**: N:1 (many type variants can reference one profile)

**Purpose**: Maps brick type IDs to material profiles

**Example**:

```ron
// In manifest.ron
(
  profiles: [
    (
      id: "brick/stone",
      albedo_path: "stone_albedo.png",
      normal_path: Some("stone_normal.png"),
      orm_path: Some("stone_orm.png"),
      emissive_path: None,
      depth_path: Some("stone_depth.png"),
      roughness: 0.8,
      metallic: 0.0,
      uv_scale: (2.0, 2.0),
      uv_offset: (0.0, 0.0),
      fallback_chain: ["brick/default"],
    ),
  ],

  type_variants: [
    (
      object_class: Brick,
      type_id: 3,
      profile_id: "brick/stone",  // References profile above
      emissive_color: None,
      animation: None,
    ),
  ],
)
```

---

## File Format Examples

### Minimal Profile (Backward Compatible)

```ron
(
  id: "brick/simple",
  albedo_path: "simple_brick.png",
  // No normal, ORM, emissive, or depth textures
  roughness: 0.5,
  metallic: 0.0,
  uv_scale: (1.0, 1.0),
  uv_offset: (0.0, 0.0),
  fallback_chain: [],
)
```

### Full PBR Profile (All Textures)

```ron
(
  id: "brick/deluxe",
  albedo_path: "brick_albedo.png",
  normal_path: Some("brick_normal.png"),
  orm_path: Some("brick_orm.png"),           // NEW
  emissive_path: Some("brick_emissive.png"), // NEW
  depth_path: Some("brick_depth.png"),       // NEW
  roughness: 1.0,    // Multiplier for ORM green channel
  metallic: 0.0,     // Multiplier for ORM blue channel
  uv_scale: (2.0, 2.0),
  uv_offset: (0.0, 0.0),
  fallback_chain: ["brick/default"],
)
```

### Emissive-Only Special Effect

```ron
(
  id: "brick/neon",
  albedo_path: "neon_base.png",
  emissive_path: Some("neon_glow.png"),  // NEW: adds glow
  roughness: 0.3,
  metallic: 0.0,
  uv_scale: (1.0, 1.0),
  uv_offset: (0.0, 0.0),
  fallback_chain: [],
)
```

---

## Invariants

1. **Backward Compatibility**: Existing profiles without new fields MUST deserialize successfully
2. **UV Alignment**: `uv_transform` MUST be applied identically to all texture maps
3. **Color Space Consistency**: Data textures (ORM, depth, normal) MUST use linear; color textures (albedo, emissive) MUST use sRGB
4. **Dual Assignment**: ORM texture MUST be assigned to both `metallic_roughness_texture` and `occlusion_texture` fields
5. **Fallback Integrity**: Missing textures MUST NOT cause crashes; fallback to scalar values or default textures
6. **Path Normalization**: Texture paths MUST be normalized to `textures/` prefix if not already present

---

## Migration Path

**From**: Existing profiles with only `albedo_path` and optional `normal_path` **To**: Profiles with optional `orm_path`, `emissive_path`, `depth_path`

**Migration Steps**: None required (backward compatible)

- Existing RON files parse successfully with new field definitions
- `#[serde(default)]` provides `None` values for missing fields
- Existing materials continue to work with scalar roughness/metallic

**Example Migration** (optional, for enhancing existing content):

```diff
(
  id: "brick/default",
  albedo_path: "brick_albedo.png",
  normal_path: Some("brick_normal.png"),
+ orm_path: Some("brick_orm.png"),
  roughness: 0.8,
  metallic: 0.0,
  uv_scale: (1.0, 1.0),
  uv_offset: (0.0, 0.0),
  fallback_chain: [],
)
```

---

## Summary

The data model extends `VisualAssetProfile` with three optional texture fields following industry-standard glTF 2.0 conventions.
All changes maintain backward compatibility through optional fields and established fallback patterns.
The implementation leverages Bevy's existing StandardMaterial capabilities without requiring custom shaders or new rendering systems.
