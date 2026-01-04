# Texture System Documentation

## Overview

The texture system manages PBR (Physically Based Rendering) materials for bricks and other game objects.
It supports a comprehensive set of texture types including base color, normal maps, and advanced PBR textures (ORM, emissive, and depth).

## Texture Types

### Core Textures

#### Albedo (Base Color) - Required

- **Path Field**: `albedo_path: String`
- **Format**: sRGB color image (24-bit RGB or 32-bit RGBA)
- **Resolution**: Recommended 256×256 or 512×512
- **Purpose**: Base color/diffuse color of the material
- **Note**: This is the only required texture type; all others are optional

#### Normal Map - Optional

- **Path Field**: `normal_path: Option<String>`
- **Format**: Linear RGB (not sRGB)
- **Channels**: X (red), Y (green), Z (blue) - DirectX tangent space
- **Resolution**: Should match albedo resolution
- **Purpose**: Surface detail without additional geometry
- **Effect**: Creates surface bumps and crevices through normal perturbation

### Advanced PBR Textures

#### ORM Texture (Occlusion-Roughness-Metallic) - Optional (Phase 1)

- **Path Field**: `orm_path: Option<String>`
- **Format**: Linear RGB - three separate channels
- **Channel Mapping** (glTF 2.0 standard):
  - **Red (R)**: Ambient Occlusion (AO) - darkens crevices and corners
    - Value range: 0.0 (fully occluded) to 1.0 (fully lit)
  - **Green (G)**: Roughness - controls specular reflection
    - Value range: 0.0 (mirror-like) to 1.0 (completely rough)
  - **Blue (B)**: Metallic - controls metallic behavior
    - Value range: 0.0 (non-metal) to 1.0 (fully metallic)
- **Resolution**: Recommended 256×256 or 512×512
- **Note**: Bevy loads as linear color space (`is_srgb=false`)
- **Material Fields**:
  - Assigned to `StandardMaterial::metallic_roughness_texture` (G and B channels)
  - Assigned to `StandardMaterial::occlusion_texture` (R channel)

**Example RON Configuration:**

```ron
(
    id: "brick_with_orm",
    albedo_path: "textures/brick_albedo.png",
    normal_path: Some("textures/brick_normal.png"),
    orm_path: Some("textures/brick_orm.png"),
    roughness: 0.5,      // Scalar multiplier for roughness channel
    metallic: 0.3,       // Scalar multiplier for metallic channel
    uv_scale: (1.0, 1.0),
    uv_offset: (0.0, 0.0),
)
```

**Visual Effect**:

- Specular highlights vary across surface (from G channel roughness)
- Dark areas in surface crevices and corners (from R channel occlusion)
- Metallic reflection properties vary by location (from B channel)

#### Emissive Texture - Optional (Phase 2)

- **Path Field**: `emissive_path: Option<String>`
- **Format**: sRGB color image (24-bit RGB or 32-bit RGBA)
- **Channel Meaning**: RGB = emitted light color
- **Resolution**: Recommended 256×256 or 512×512
- **Note**: Bevy loads as sRGB color space (`is_srgb=true`)
- **Material Field**: `StandardMaterial::emissive_texture`
- **Interaction**: Can combine with `emissive_color` on TypeVariantDefinition for tinting

**Example RON Configuration:**

```ron
(
    id: "glowing_brick",
    albedo_path: "textures/brick_albedo.png",
    normal_path: Some("textures/brick_normal.png"),
    emissive_path: Some("textures/brick_glow.png"),
)
```

**Visual Effect**:

- Pattern of self-illumination based on texture
- Visible in dim/dark environments
- Creates appearance of light source without affecting lighting
- Example: neon signs, glowing runes, power-up indicators

#### Depth Texture (Parallax Maps) - Optional (Phase 3)

- **Path Field**: `depth_path: Option<String>`
- **Format**: Linear grayscale (8-bit or 16-bit)
- **Value Interpretation**:
  - 0.0 (black) = surface recessed deepest
  - 1.0 (white) = surface raised highest
- **Resolution**: Should match albedo resolution
- **Scale Parameter**: `depth_scale: f32` (default: 0.1)
- **Current Status**: Infrastructure prepared for future parallax mapping
- **Note**: Bevy's StandardMaterial doesn't natively support parallax mapping
  - Depth texture and scale parameters are reserved for custom shader implementation
  - Framework loads and validates depth maps; rendering awaits custom shader

**Example RON Configuration:**

```ron
(
    id: "detailed_brick",
    albedo_path: "textures/brick_albedo.png",
    normal_path: Some("textures/brick_normal.png"),
    depth_path: Some("textures/brick_depth.png"),
    depth_scale: 0.15,  // Controls parallax intensity
)
```

**Visual Effect** (when custom shader implemented):

- Parallax occlusion mapping effect at grazing camera angles
- Surface appears to have height variation beyond normal mapping
- Example: deep crevices, etched details, weathering patterns

## UV Transforms

All texture types share the same UV coordinate transformation parameters:

- **`uv_scale: Vec2`** (default: `(1.0, 1.0)`)
  - Scales UV coordinates before sampling
  - Values > 1.0 repeat the texture (tiling)
  - Values < 1.0 shrink the texture
  - Applied uniformly to all textures

- **`uv_offset: Vec2`** (default: `(0.0, 0.0)`)
  - Offsets UV coordinates
  - Useful for pattern shifting or animation
  - Applied uniformly to all textures

**Example: Tiled Pattern**

```ron
(
    id: "tiled_brick",
    albedo_path: "textures/brick_base.png",
    normal_path: Some("textures/brick_normal.png"),
    orm_path: Some("textures/brick_orm.png"),
    uv_scale: (2.0, 2.0),  // 2×2 tiling
    uv_offset: (0.0, 0.0),
)
```

## Fallback Chain

Profiles can reference fallback profiles when textures are missing:

```ron
(
    id: "primary_variant",
    albedo_path: "textures/variant_albedo.png",
    orm_path: None,  // Missing ORM
    fallback_chain: ["standard_brick"],  // Fall back to this profile
)
```

**Resolution Order**:

1. Try to load all textures from primary profile
2. If any required texture is missing, look up first fallback profile
3. Continue down fallback chain until complete profile found
4. If all fallbacks exhausted, fall back to default material

## Color Spaces

Bevy requires correct color space specification during asset loading:

- **Linear Color Space** (`is_srgb=false`):
  - ORM textures (raw channel data)
  - Normal maps (preserves direction vectors)
  - Depth maps (preserves height values)

- **sRGB Color Space** (`is_srgb=true`):
  - Albedo/base color (visual colors for display)
  - Emissive textures (visible light colors)

⚠️ **Important**: Using incorrect color space degrades visual quality significantly!

## Backward Compatibility

The texture system maintains backward compatibility with older profiles:

- **Old profiles** with only `albedo_path` and `normal_path` work unchanged
- **New texture fields** (orm_path, emissive_path, depth_path) are optional with defaults
- **UV transforms** default to identity (no scaling or offset)
- **Depth scale** defaults to 0.1

Example old profile (still supported):

```ron
(
    id: "legacy_brick",
    albedo_path: "brick.png",
    normal_path: Some("brick_normal.png"),
)
```

This profile will load and render correctly with default values for all new fields.

## Implementation Details

### Asset Loading

The texture system uses Bevy's `AssetServer` to load images with specific color space settings:

```rust
// ORM texture loading (linear)
asset_server.load_with_settings(
    orm_path,
    ImageLoaderSettings {
        is_srgb: false,
        ..default()
    }
)

// Emissive texture loading (sRGB)
asset_server.load_with_settings(
    emissive_path,
    ImageLoaderSettings {
        is_srgb: true,
        ..default()
    }
)
```

### Error Handling

Missing textures are handled gracefully:

- **Missing files**: Asset server returns a handle to missing asset (doesn't panic)
- **Shader rendering**: Missing textures fall back to default values
- **Logging**: Warnings are logged when textures can't be loaded

### Material Generation

The `make_material()` function:

1. Loads all specified textures with appropriate color spaces
2. Assigns textures to correct StandardMaterial fields
3. Sets scalar multipliers (roughness, metallic)
4. Returns complete StandardMaterial ready for rendering

## Testing

Comprehensive test coverage validates:

- ✅ Texture deserialization from RON
- ✅ Correct color space application
- ✅ All 5 texture types loading together
- ✅ UV transforms applying uniformly
- ✅ Fallback chain resolution
- ✅ Backward compatibility with old profiles
- ✅ Graceful handling of missing files

## Future Enhancements

### Custom Parallax Mapping Shader

Once a custom shader supporting parallax occlusion mapping is implemented:

- Depth texture will be actively used in rendering
- `depth_scale` will control parallax intensity
- Visual effect will be visible at grazing camera angles

### Animation Support

Potential for animated textures via:

- Frame-based UV offset changes
- Tiled spritesheet layouts
- Smooth state transitions

## Related Files

- [src/systems/textures/loader.rs](loader.rs) - Manifest loading and VisualAssetProfile definition
- [src/systems/textures/materials.rs](materials.rs) - Material creation from profiles
- [src/systems/textures/contracts.rs](contracts.rs) - API contracts and deserialization helpers
- [assets/textures/manifest.ron](../../../assets/textures/manifest.ron) - Profile definitions for all brick types
