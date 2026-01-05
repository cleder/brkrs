# API Contract: VisualAssetProfile

**Feature**: 017-brick-material-textures **Date**: 2026-01-04 **Version**: 2.0.0 (extended with ORM, emissive, depth support)

## Overview

The VisualAssetProfile contract defines the structure for material profiles in the texture manifest system.
This contract is used for:

- RON file deserialization (`assets/textures/manifest.ron`)
- External tooling and validation
- API compatibility between texture system versions

## Contract Definition

### Rust Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualAssetProfileContract {
    /// Unique identifier for this profile (e.g., "brick/stone", "ball/metal")
    pub id: String,

    /// Path to base color/albedo texture (relative to assets/textures/)
    pub albedo_path: String,

    /// Optional path to normal map texture (linear color space)
    pub normal_path: Option<String>,

    /// Optional path to packed ORM texture (Occlusion-Roughness-Metallic, glTF 2.0)
    /// Red channel = Ambient Occlusion
    /// Green channel = Roughness
    /// Blue channel = Metallic
    pub orm_path: Option<String>,

    /// Optional path to emissive/glow map texture (sRGB color space)
    pub emissive_path: Option<String>,

    /// Optional path to depth/parallax map texture (linear color space)
    pub depth_path: Option<String>,

    /// Base/multiplier roughness value (0.0 = smooth/mirror, 1.0 = rough/matte)
    /// When orm_path is set, this multiplies the green channel value
    pub roughness: f32,

    /// Base/multiplier metallic value (0.0 = non-metal, 1.0 = metal)
    /// When orm_path is set, this multiplies the blue channel value
    pub metallic: f32,

    /// UV scale factor [x, y] for texture tiling
    pub uv_scale: [f32; 2],

    /// UV offset [x, y] for texture positioning
    pub uv_offset: [f32; 2],

    /// Ordered list of fallback profile IDs if this profile fails to load
    pub fallback_chain: Vec<String>,
}
```

### JSON Schema (for external tools)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "VisualAssetProfile",
  "type": "object",
  "required": ["id", "albedo_path", "roughness", "metallic", "uv_scale", "uv_offset", "fallback_chain"],
  "properties": {
    "id": {
      "type": "string",
      "description": "Unique identifier for this profile",
      "pattern": "^[a-z0-9_/]+$",
      "examples": ["brick/stone", "ball/default", "paddle/wood"]
    },
    "albedo_path": {
      "type": "string",
      "description": "Path to base color texture (relative to assets/textures/)",
      "pattern": "^[a-zA-Z0-9_/.-]+\\.(png|ktx2)$"
    },
    "normal_path": {
      "type": ["string", "null"],
      "description": "Optional path to normal map texture",
      "pattern": "^[a-zA-Z0-9_/.-]+\\.(png|ktx2)$"
    },
    "orm_path": {
      "type": ["string", "null"],
      "description": "Optional path to packed ORM texture (glTF 2.0: R=AO, G=Roughness, B=Metallic)",
      "pattern": "^[a-zA-Z0-9_/.-]+\\.(png|ktx2)$"
    },
    "emissive_path": {
      "type": ["string", "null"],
      "description": "Optional path to emissive/glow map texture",
      "pattern": "^[a-zA-Z0-9_/.-]+\\.(png|ktx2)$"
    },
    "depth_path": {
      "type": ["string", "null"],
      "description": "Optional path to depth/parallax map texture",
      "pattern": "^[a-zA-Z0-9_/.-]+\\.(png|ktx2)$"
    },
    "roughness": {
      "type": "number",
      "description": "Base/multiplier roughness value",
      "minimum": 0.0,
      "maximum": 1.0,
      "default": 0.5
    },
    "metallic": {
      "type": "number",
      "description": "Base/multiplier metallic value",
      "minimum": 0.0,
      "maximum": 1.0,
      "default": 0.0
    },
    "uv_scale": {
      "type": "array",
      "description": "UV scale factor [x, y]",
      "items": {
        "type": "number"
      },
      "minItems": 2,
      "maxItems": 2,
      "default": [1.0, 1.0]
    },
    "uv_offset": {
      "type": "array",
      "description": "UV offset [x, y]",
      "items": {
        "type": "number"
      },
      "minItems": 2,
      "maxItems": 2,
      "default": [0.0, 0.0]
    },
    "fallback_chain": {
      "type": "array",
      "description": "Ordered list of fallback profile IDs",
      "items": {
        "type": "string",
        "pattern": "^[a-z0-9_/]+$"
      },
      "default": []
    }
  }
}
```

## Validation Rules

### Required Fields

- `id`: Non-empty string following pattern `[a-z0-9_/]+`
- `albedo_path`: Valid texture file path (PNG or KTX2)
- `roughness`: Float in range [0.0, 1.0]
- `metallic`: Float in range [0.0, 1.0]
- `uv_scale`: Array of two floats
- `uv_offset`: Array of two floats
- `fallback_chain`: Array (can be empty)

### Optional Fields

- `normal_path`: If provided, must be valid texture path
- `orm_path`: If provided, must be valid texture path following glTF 2.0 channel convention
- `emissive_path`: If provided, must be valid texture path
- `depth_path`: If provided, must be valid texture path

### Semantic Rules

1. **Path Format**: Texture paths are relative to `assets/textures/` directory
2. **Path Normalization**: Paths MAY start with `textures/` prefix (will be normalized)
3. **File Extensions**: Only `.png` and `.ktx2` formats are supported
4. **UV Scale**: Values > 1.0 tile the texture, < 1.0 shrink it
5. **Fallback Chain**: Profile IDs in fallback chain must exist in the manifest
6. **ORM Channel Convention**: If `orm_path` is provided, texture MUST follow glTF 2.0 standard (red=AO, green=roughness, blue=metallic)

## RON Format Examples

### Minimal Profile (Backward Compatible)

```ron
(
  id: "brick/simple",
  albedo_path: "simple_brick.png",
  roughness: 0.5,
  metallic: 0.0,
  uv_scale: (1.0, 1.0),
  uv_offset: (0.0, 0.0),
  fallback_chain: [],
)
```

### Full PBR Profile with All Textures

```ron
(
  id: "brick/deluxe",
  albedo_path: "brick_albedo.png",
  normal_path: Some("brick_normal.png"),
  orm_path: Some("brick_orm.png"),
  emissive_path: Some("brick_emissive.png"),
  depth_path: Some("brick_depth.png"),
  roughness: 1.0,
  metallic: 0.0,
  uv_scale: (2.0, 2.0),
  uv_offset: (0.0, 0.0),
  fallback_chain: ["brick/default"],
)
```

### Profile with Emissive Only (Special Effect)

```ron
(
  id: "brick/neon",
  albedo_path: "neon_base.png",
  emissive_path: Some("neon_glow.png"),
  roughness: 0.3,
  metallic: 0.0,
  uv_scale: (1.0, 1.0),
  uv_offset: (0.0, 0.0),
  fallback_chain: [],
)
```

## Conversion Between Contract and Runtime

### Contract → Runtime (Deserialization)

```rust
impl From<VisualAssetProfileContract> for VisualAssetProfile {
    fn from(contract: VisualAssetProfileContract) -> Self {
        Self {
            id: contract.id,
            albedo_path: contract.albedo_path,
            normal_path: contract.normal_path,
            orm_path: contract.orm_path,           // NEW
            emissive_path: contract.emissive_path, // NEW
            depth_path: contract.depth_path,       // NEW
            roughness: contract.roughness,
            metallic: contract.metallic,
            uv_scale: Vec2::from_array(contract.uv_scale),
            uv_offset: Vec2::from_array(contract.uv_offset),
            fallback_chain: contract.fallback_chain,
        }
    }
}
```

### Runtime → Contract (Serialization)

```rust
impl From<VisualAssetProfile> for VisualAssetProfileContract {
    fn from(profile: VisualAssetProfile) -> Self {
        Self {
            id: profile.id,
            albedo_path: profile.albedo_path,
            normal_path: profile.normal_path,
            orm_path: profile.orm_path,           // NEW
            emissive_path: profile.emissive_path, // NEW
            depth_path: profile.depth_path,       // NEW
            roughness: profile.roughness,
            metallic: profile.metallic,
            uv_scale: profile.uv_scale.to_array(),
            uv_offset: profile.uv_offset.to_array(),
            fallback_chain: profile.fallback_chain,
        }
    }
}
```

## Backward Compatibility

### Version 1.0.0 → 2.0.0 Migration

**Changes**:

- Added `orm_path: Option<String>` (optional, defaults to `None`)
- Added `emissive_path: Option<String>` (optional, defaults to `None`)
- Added `depth_path: Option<String>` (optional, defaults to `None`)

**Compatibility Guarantee**:

- Version 1.0.0 RON files (without new fields) parse correctly in 2.0.0
- All new fields use `#[serde(default)]` to provide `None` when absent
- Existing profiles continue to work without modification
- No migration tool required

**Example Version 1.0.0 File** (still valid in 2.0.0):

```ron
(
  id: "brick/old",
  albedo_path: "old_brick.png",
  normal_path: Some("old_normal.png"),
  // orm_path, emissive_path, depth_path omitted (defaults to None)
  roughness: 0.7,
  metallic: 0.0,
  uv_scale: (1.0, 1.0),
  uv_offset: (0.0, 0.0),
  fallback_chain: [],
)
```

## Error Handling

### Deserialization Errors

| Error | Cause | Mitigation |
|-------|-------|------------|
| Missing required field | `id`, `albedo_path`, etc. absent | RON parser error with field name |
| Invalid type | String where number expected | RON parser error with type mismatch |
| Out-of-range value | `roughness: 1.5` (>1.0) | Post-validation clamps to [0.0, 1.0] |
| Invalid path | File doesn't exist | Logged warning, fallback to default texture |

### Runtime Errors

| Error | Cause | Mitigation |
|-------|-------|------------|
| Texture load failure | File missing or corrupt | Log warning, use fallback or skip texture |
| Circular fallback chain | A→B→A | Detect cycle, break chain, log error |
| Invalid fallback reference | Fallback ID doesn't exist | Log warning, skip to next in chain |

## Testing Contract

### Required Tests

1. **Deserialization**: Parse RON with all fields present
2. **Backward Compatibility**: Parse RON without optional fields (v1.0.0 format)
3. **Validation**: Reject invalid roughness/metallic values
4. **Path Normalization**: Handle paths with/without `textures/` prefix
5. **Fallback Chain**: Verify fallback resolution logic
6. **Type Safety**: Verify array lengths for uv_scale/uv_offset

### Test Data Examples

```ron
// Valid: All fields
(id: "test/full", albedo_path: "a.png", normal_path: Some("n.png"),
 orm_path: Some("orm.png"), emissive_path: Some("e.png"), depth_path: Some("d.png"),
 roughness: 0.5, metallic: 0.0, uv_scale: (1.0, 1.0), uv_offset: (0.0, 0.0), fallback_chain: [])

// Valid: Minimal (backward compatible)
(id: "test/min", albedo_path: "a.png", roughness: 0.5, metallic: 0.0,
 uv_scale: (1.0, 1.0), uv_offset: (0.0, 0.0), fallback_chain: [])

// Invalid: Missing required field
(albedo_path: "a.png", roughness: 0.5, metallic: 0.0,
 uv_scale: (1.0, 1.0), uv_offset: (0.0, 0.0), fallback_chain: [])

// Invalid: Wrong type
(id: "test/bad", albedo_path: 123, roughness: 0.5, metallic: 0.0,
 uv_scale: (1.0, 1.0), uv_offset: (0.0, 0.0), fallback_chain: [])
```

## Versioning Policy

- **MAJOR**: Breaking changes (remove required fields, change field types)
- **MINOR**: Add optional fields (backward compatible)
- **PATCH**: Documentation updates, validation improvements

**Current Version**: 2.0.0 (added `orm_path`, `emissive_path`, `depth_path`)

**Previous Versions**:

- 1.0.0: Initial contract with albedo, normal, roughness, metallic
