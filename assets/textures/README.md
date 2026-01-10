# Texture Asset Guide for Brkrs

This guide explains how to add, modify, and manage textures for the Brkrs breakout game.

<!-- INCLUSION-MARKER-DO-NOT-REMOVE -->

The game uses a **texture manifest system** (`manifest.ron`) to define all visual materials for gameplay objects.
Textures can be customized globally or overridden per-level for unique visual themes.

## Quick Start

### Adding a New Texture

1. **Place your texture file** in `assets/textures/` or a subdirectory
   - Supported formats: PNG, KTX2
   - Recommended naming: descriptive lowercase with underscores (e.g., `metal_rough.png`)

2. **Add a profile entry** to `manifest.ron`:

```rust
(
  profiles: [
    // ... existing profiles ...

    // Your new profile
    (
      id: "brick/metal",              // Unique identifier
      albedo_path: "metal_rough.png", // Base color texture
      normal_path: Some("metal_normal.png"), // Optional normal map
      roughness: 0.8,                 // 0.0 (smooth) to 1.0 (rough)
      metallic: 0.9,                  // 0.0 (non-metal) to 1.0 (metal)
      uv_scale: (1.0, 1.0),          // Texture tiling (x, y)
      uv_offset: (0.0, 0.0),         // Texture offset
      fallback_chain: [],             // Backup profiles if texture fails
    ),
  ],

  // ... rest of manifest ...
)
```

1. **Hot-reload**: Save `manifest.ron` and the game will reload automatically (no restart needed)

### Applying Textures to Objects

#### Default Object Textures

Edit the canonical profiles in `manifest.ron`:

- `ball/default` - Ball appearance
- `paddle/default` - Paddle appearance
- `brick/default` - Default brick appearance
- `sidewall/default` - Border walls
- `ground/default` - Ground plane
- `background/default` - Background plane

#### Type-Specific Textures (Brick/Ball Variants)

Use `type_variants` to map gameplay types to visual profiles:

```rust
(
  profiles: [
    // ... profiles ...
  ],

  type_variants: [
    // Map brick type 3 to metal texture
    (
      object_class: Brick,
      type_id: 3,
      profile_id: "brick/metal",
      emissive_color: None,
      animation: None,
    ),

    // Map ball type 1 to fire texture
    (
      object_class: Ball,
      type_id: 1,
      profile_id: "ball/fire",
      emissive_color: Some(Srgba(red: 1.0, green: 0.3, blue: 0.0, alpha: 1.0)),
      animation: None,
    ),
  ],

  // ... rest of manifest ...
)
```

### New type mappings for designers

Two new brick indices are introduced for designers:

- `20` — canonical "simple" brick type going forward (legacy index `3` is still recognized during a compatibility window)
- `90` — indestructible brick (designer-visible; will never count toward level completion)

Add profiles for `brick/type20` and `brick/indestructible` in `manifest.ron` and map them in `type_variants` to ensure the in-game editor and runtime render the correct visuals for these indices.

Example:

```rust
(
  // ... profiles ...
  type_variants: [
    ( object_class: Brick, type_id: 20, profile_id: "brick/type20", emissive_color: None, animation: None ),
    ( object_class: Brick, type_id: 90, profile_id: "brick/indestructible", emissive_color: None, animation: None ),
  ]
)
```

#### Per-Level Texture Overrides

Customize ground, background, and sidewall textures for specific levels:

##### **Method 1: In manifest.ron**

```rust
(
  profiles: [
    // ... profiles ...
  ],

  type_variants: [
    // ... variants ...
  ],

  level_overrides: [
    (
      level_number: 2,
      ground_profile: Some("ground/lava"),
      background_profile: Some("background/sunset"),
      sidewall_profile: Some("sidewall/marble"),
      tint: Some(Srgba(red: 1.0, green: 0.8, blue: 0.6, alpha: 1.0)),
      notes: Some("Lava-themed level with warm tint"),
    ),
  ],
)
```

##### **Method 2: Inline in level file** (`assets/levels/level_002.ron`)

```rust
LevelDefinition(
  number: 2,
  gravity: Some((-1.5, 0.0, 0.0)),
  matrix: [
    // ... matrix data ...
  ],
  presentation: Some((
    level_number: 2,
    ground_profile: Some("ground/ice"),
    background_profile: None,  // Use default
    sidewall_profile: None,     // Use default
    tint: None,
    notes: Some("Ice level"),
  )),
)
```

## File Structure

```text
assets/textures/
├── README.md           # This guide
├── manifest.ron        # Master texture configuration
├── fallback/          # Placeholder textures (auto-generated)
└── [your textures]    # PNG/KTX2 texture files
```

## Texture Manifest

The `manifest.ron` file is the central configuration for all textures in the game.
It defines visual asset profiles, maps gameplay types to textures, and configures per-level overrides.

### Manifest Structure

```rust
// assets/textures/manifest.ron
(
  profiles: [
    // Visual asset profiles (materials with textures)
    (
      id: "brick/default",
      albedo_path: "debug/four_squares_64.png",
      normal_path: Some("debug/cube_normal.png"),
      orm_path: None,
      emissive_path: Some("test/test_emissive.png"),
      depth_path: Some("debug/cube_depth.png"),
      roughness: 0.7,
      metallic: 0.0,
      uv_scale: (1.0, 1.0),
      uv_offset: (0.0, 0.0),
      depth_scale: 0.2,
      fallback_chain: [],
    ),
    // ... more profiles ...
  ],

  type_variants: [
    // Map gameplay types to visual profiles
    (
      object_class: Brick,
      type_id: 3,
      profile_id: "brick/default",
      emissive_color: None,
      animation: None,
    ),
    // ... more type mappings ...
  ],

  level_overrides: [
    // Per-level texture customizations
    (
      level_number: 2,
      ground_profile: Some("ground/lava"),
      background_profile: Some("background/sunset"),
      sidewall_profile: Some("sidewall/marble"),
      tint: Some(Srgba(red: 1.0, green: 0.8, blue: 0.6, alpha: 1.0)),
      notes: Some("Lava-themed level"),
    ),
    // ... more level overrides ...
  ],
)
```

### Manifest Loading

- **Startup**: Manifest is loaded once when the game starts
- **Hot-Reload**: Changes to `manifest.ron` are detected and applied automatically
- **Validation**: Parse errors are logged with line numbers for debugging
- **Fallback Behavior**: If manifest is missing or invalid, game uses hardcoded default materials

### Path Resolution

All texture paths in the manifest are **relative to `assets/textures/`**:

```rust
// These are equivalent:
albedo_path: "brick_stone.png"              // → assets/textures/brick_stone.png
albedo_path: "materials/brick_stone.png"    // → assets/textures/materials/brick_stone.png
```

**Do NOT** use absolute paths or `../` navigation - they will fail in WASM builds.

## Fallback Textures

The `fallback/` directory contains default textures used when:

- Custom textures are missing or fail to load
- A profile's `fallback_chain` is triggered
- The manifest is invalid or absent

### Default Fallback Files

Generated automatically if missing:

| File | Purpose | Dimensions | Format |
|------|---------|------------|--------|
| `brick_base.png` | Default brick texture | 64×64 | PNG, sRGB |
| `paddle_base.png` | Default paddle texture | 64×64 | PNG, sRGB |
| `ball_base.png` | Default ball texture | 64×64 | PNG, sRGB |
| `ground_base.png` | Floor texture | 512×512 | PNG, sRGB |
| `sidewall_base.png` | Wall textures | 512×512 | PNG, sRGB |
| `background_base.png` | Background texture | 1024×1024 | PNG, sRGB |

### Fallback Chain Mechanism

Profiles can specify a `fallback_chain` to gracefully degrade when textures fail:

```rust
(
  id: "brick/exotic",
  albedo_path: "exotic_unavailable.png",  // Might not exist
  // ... other fields ...
  fallback_chain: ["brick/metal", "brick/default"],
)
```

**Resolution order**:

1. Try loading `exotic_unavailable.png`
2. If that fails, try `brick/metal` profile
3. If that fails, try `brick/default` profile
4. If all fail, use hardcoded fallback material

### Customizing Fallback Textures

You can replace the auto-generated fallbacks:

1. Create custom textures matching the filenames above
2. Place them in `assets/textures/fallback/`
3. Restart the game (fallbacks are loaded at startup, not hot-reloaded)

**Recommended**: Keep fallbacks simple and low-resolution for fast loading and WASM bundle size.

## Texture Formats & Color Channels

### Supported File Formats

#### PNG (Recommended for Development)

- **Pros**: Lossless compression, wide tool support, easy iteration
- **Cons**: Larger file sizes, slower loading than KTX2
- **Color Depth**: 8-bit or 16-bit per channel supported
- **Alpha Channel**: Fully supported (RGBA)
- **Use Case**: Quick prototyping, asset creation, debugging

#### KTX2 (Recommended for Production)

- **Pros**: GPU-native compression, 50-90% smaller than PNG, faster loading
- **Cons**: Requires conversion tools, lossy compression options
- **Compression**: Supports BC7 (high quality), BC3 (legacy), or Basis Universal
- **Alpha Channel**: Fully supported
- **Use Case**: Final builds, WASM deployments, performance-critical scenarios
- **Conversion**: Use `toktx` or `PVRTexTool` to convert PNG → KTX2

### Color Channel Meanings by Texture Type

#### Albedo/Base Color Texture (`albedo_path`)

- **Format**: RGB or RGBA
- **Color Space**: **sRGB** (gamma-corrected)
- **Channels**:
  - **R, G, B**: Diffuse surface color (what you see under white light)
  - **Alpha**: Transparency (1.0 = opaque, 0.0 = fully transparent)
    - For breakout game objects, typically use fully opaque (alpha = 1.0)
    - Alpha < 1.0 enables transparency but may affect rendering order
- **Best Practices**:
  - Avoid pure white (255,255,255) or pure black (0,0,0) as they can look unrealistic
  - Use mid-range values (50-200) for most materials
  - Paint shadows/lighting in normal maps, not albedo

#### Normal Map Texture (`normal_path`)

- **Format**: RGB (alpha channel ignored)
- **Color Space**: **Linear** (do NOT use sRGB for normals)
- **Channels**:
  - **R (Red)**: X-axis normal component (left ↔ right surface angle)
  - **G (Green)**: Y-axis normal component (down ↔ up surface angle)
  - **B (Blue)**: Z-axis normal component (into ↔ out of surface)
- **Color Interpretation**:
  - RGB (128, 128, 255) = flat surface pointing toward camera (normal)
  - RGB (255, 128, 128) = surface angled right
  - RGB (0, 128, 128) = surface angled left
  - RGB (128, 255, 128) = surface angled up
  - RGB (128, 0, 128) = surface angled down
- **Common Mistakes**:
  - ❌ Using a bump/height map instead of a normal map
  - ❌ Saving as sRGB instead of linear color space
  - ❌ Inverting Y-axis (use OpenGL format, not DirectX)
- **Conversion**: GIMP → Filters → Generic → Normal Map (from height map)

#### ORM Texture (`orm_path`)

- **Format**: RGB (Occlusion/Roughness/Metallic packed)
- **Color Space**: **Linear**
- **Channels**:
  - **R (Red)**: Ambient Occlusion
    - 255 (white) = fully lit, no occlusion
    - 0 (black) = fully shadowed (crevices, cracks)
    - Controls subtle ambient shadows in recessed areas
  - **G (Green)**: Roughness
    - 255 (white) = completely rough/matte (1.0)
    - 0 (black) = perfectly smooth/mirror (0.0)
    - Overrides profile's `roughness` scalar value
  - **B (Blue)**: Metallic
    - 255 (white) = fully metallic (1.0)
    - 0 (black) = non-metallic/dielectric (0.0)
    - Overrides profile's `metallic` scalar value
  - **Alpha**: Ignored
- **Optimization**: Packing 3 grayscale maps into one RGB texture saves memory and texture slots

#### Emissive Texture (`emissive_path`)

- **Format**: RGB or RGBA
- **Color Space**: **sRGB**
- **Channels**:
  - **R, G, B**: Emitted light color and intensity
    - Values > 128 (0.5 linear) emit noticeable light
    - Values > 200 (0.8 linear) emit strong light
    - Pure black (0,0,0) = no emission
  - **Alpha**: Can mask emission regions (0 = no emission, 255 = full emission)
- **Combination**: Multiplied with `emissive_color` from TypeVariantDefinition if both set
- **Examples**:
  - RGB (255, 200, 0) = bright yellow-orange glow
  - RGB (0, 255, 255) = cyan neon glow
  - RGB (128, 0, 0) = subtle red glow

#### Depth/Parallax Map Texture (`depth_path`)

- **Format**: Grayscale (R channel used, GB ignored) or single-channel
- **Color Space**: **Linear**
- **Channel Interpretation**:
  - **White (255)**: Raised/protruding surface areas
  - **Black (0)**: Recessed/indented surface areas
  - **Mid-gray (128)**: Neutral height (no displacement)
- **Usage**: Creates illusion of depth on flat surfaces via parallax occlusion mapping
- **Scaling**: Actual depth effect controlled by `depth_scale` parameter (0.0-1.0 typical)
- **Performance**: Depth mapping is more expensive than normal mapping; use sparingly

### Alpha/Transparency Handling

#### Opaque Materials (Default)

- Set alpha = 1.0 (255) in albedo texture
- Faster rendering (no transparency sorting needed)
- Recommended for all breakout game objects (bricks, paddle, ball)

#### Transparent Materials

- Set alpha < 1.0 in albedo texture
- Enables alpha blending but may cause:
  - Rendering order issues (objects drawn back-to-front)
  - Performance overhead
  - Depth sorting artifacts
- **Use only when necessary** (e.g., glass bricks, particle effects)

#### Alpha Masking (Binary Transparency)

- Use alpha = 0.0 or 1.0 (no intermediate values)
- Enables cutout/clip effects (e.g., chain-link fence pattern)
- Better performance than alpha blending
- Set alpha threshold in material if needed

### Color Space Summary

| Texture Type | Color Space | Why |
|--------------|-------------|-----|
| Albedo | **sRGB** | Matches how artists paint and how displays show color |
| Normal Map | **Linear** | Mathematical vectors; sRGB gamma breaks calculations |
| ORM | **Linear** | Physical properties; gamma correction distorts values |
| Emissive | **sRGB** | Artist-friendly color specification |
| Depth | **Linear** | Height values; gamma correction distorts displacement |

**Important**: Most image editors save as sRGB by default.
For normal/ORM/depth maps, disable sRGB or use "Save as Linear" if available.

### File Naming Conventions

Recommended naming scheme for texture sets:

```text
material_name_albedo.png
material_name_normal.png
material_name_orm.png
material_name_emissive.png
material_name_depth.png
```

Example:

```text
brick_stone_albedo.png
brick_stone_normal.png
brick_stone_orm.png
```

## Manifest Schema Reference

### VisualAssetProfile

Defines a complete material profile.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | String | *required* | Unique identifier (e.g., "brick/metal") |
| `albedo_path` | String | *required* | Path to base color texture |
| `normal_path` | `Option<String>`|`None`| Optional normal map for surface detail. **Important**: Must be a proper normal map (RGB values representing XYZ normals in tangent space), not a bump map. If you have a grayscale bump map, convert it to a normal map using image editing software (e.g., GIMP: Filters → Generic → Normal Map). Bevy does not automatically convert bump maps to normal maps. |
| `orm_path` | `Option<String>`|`None`| Optional ORM (Occlusion/Roughness/Metallic) packed texture. Red channel = occlusion, Green channel = roughness, Blue channel = metallic. When provided, overrides separate `roughness` and `metallic` scalar values. Useful for optimizing texture memory by packing multiple material properties into one texture. |
| `emissive_path` | `Option<String>`|`None`| Optional emissive texture for self-illuminating surfaces. RGB values define the emitted light color and intensity. Combined with `emissive_color` from TypeVariantDefinition if both are specified. Use for glowing effects, neon signs, or light-emitting game objects. |
| `depth_path` | `Option<String>`|`None`| Optional depth/parallax mapping texture (height map). Grayscale values define surface height for parallax occlusion mapping, creating an illusion of depth on flat surfaces. White = raised areas, Black = recessed areas. Requires `depth_scale` to be set for visible effect. |
|`roughness`| f32 |`0.5`| Surface roughness (0.0 = mirror, 1.0 = matte). Overridden by `orm_path` green channel if ORM texture is provided. |
|`metallic`| f32 |`0.0`| Metallic property (0.0 = non-metal/dielectric, 1.0 = fully metal). Controls how reflective and mirror-like a surface appears. Combined with `roughness`: low metallic + low roughness = shiny plastic; high metallic + low roughness = polished mirror-like metal; high metallic + high roughness = brushed/worn metal. Overridden by `orm_path` blue channel if ORM texture is provided. |
|`uv_scale`| (f32, f32) |`(1.0, 1.0)`| Texture tiling factors (x, y) |
|`uv_offset`| (f32, f32) |`(0.0, 0.0)`| Texture offset (x, y) |
| `depth_scale` | f32 |`0.1`| Depth intensity multiplier for parallax mapping when `depth_path` is set. Higher values = more pronounced depth effect. Range typically 0.0-1.0, though values beyond can create extreme parallax. Only used if `depth_path` is provided. |
|`fallback_chain`|`Vec<String>`|`[]` | Backup profile IDs if texture fails to load |

### TypeVariantDefinition

Maps gameplay type IDs to visual profiles.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `object_class` | Enum | *required* | `Ball` or `Brick` |
| `type_id` | u8 | *required* | Gameplay type ID (3+ for bricks) |
| `profile_id` | String | *required* | Reference to `VisualAssetProfile.id` |
| `emissive_color` | `Option<Color>` |`None`| Self-illumination color |
|`animation`| `Option<AnimationDescriptor>` |`None` | Future: animation effects |

### LevelTextureSet

Per-level material overrides for environmental objects.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level_number` | u32 | *required* | Level number to override |
| `ground_profile` | `Option<String>` |`None`| Ground plane texture profile |
|`background_profile`| `Option<String>` |`None`| Background plane texture profile |

|`sidewall_profile`| `Option<String>` |`None`| Border wall texture profile |
|`tint`| `Option<Color>` |`None`| RGBA color multiplier for level mood |
|`notes`| `Option<String>` |`None` | Designer notes/description |

## Common Workflows

### Creating a Themed Level

1. Create custom texture profiles for ground, background, sidewalls
2. Add profiles to `manifest.ron`
3. Reference them in level_overrides or inline in the level file
4. Optionally add a tint for color mood adjustment
5. Test in-game with hot-reload

### Adding Brick Variety

1. Create texture files for each brick type (type 3, 4, 5, etc.)
2. Add VisualAssetProfile for each texture
3. Add TypeVariantDefinition entries mapping type IDs to profiles
4. Place brick types in level matrix (values 3+)
5. Test spawn behavior

### Testing Textures

1. **Hot-reload**: Edit `manifest.ron` while game is running
   - Changes apply automatically to current level
   - No restart required

2. **Level preview**: Press **L** to cycle through levels quickly

3. **Visual verification**:
   - Check all object types render correctly
   - Verify tiling/offset adjustments
   - Confirm fallbacks work when textures missing

## Troubleshooting

### Texture Not Appearing

1. **Check file path**: Paths in manifest are relative to `assets/textures/`
2. **Check format**: Only PNG and KTX2 supported
3. **Check logs**: Look for warnings about missing files
4. **Verify profile ID**: Ensure no typos in references

### Fallback Material Showing

If you see default gray/colored materials instead of textures:

1. Check `manifest.ron` for parse errors (logs show line numbers)
2. Verify profile ID matches between definition and usage
3. Ensure texture file exists at specified path
4. Check fallback_chain for backup options

### Hot-Reload Not Working

1. Save `manifest.ron` explicitly
2. Check for syntax errors in RON format
3. Restart game if asset server is stuck

## Best Practices

### Texture Creation

- **Resolution**: 512x512 or 1024x1024 for most objects
  - Bricks/Balls: 512x512 sufficient
  - Ground/Background: 1024x1024 or 2048x2048 for quality
  - Normal/ORM maps: Can be half resolution of albedo (256x256 for 512x512 albedo)
- **Format**:
  - PNG for development/iteration (lossless, easy editing)
  - KTX2 for production builds (50-90% size reduction, faster loading)
  - See "Texture Formats & Color Channels" section for detailed format guidance
- **Color Space**:
  - Albedo/Emissive: Save as **sRGB**
  - Normal/ORM/Depth: Save as **Linear** (disable gamma correction)
  - Check your image editor's export settings
- **Compression**:
  - PNG: Keep under 1MB per file for quick loading
  - KTX2: Use BC7 compression for high quality, BC3 for legacy support
- **Alpha Channel**:
  - Use fully opaque (alpha=1.0) for game objects unless transparency needed
  - Binary alpha masking (0.0 or 1.0 only) is faster than gradual transparency
- **UV Mapping**:
  - Design textures to tile seamlessly for `uv_scale > 1.0`
  - Test tiling by setting uv_scale to (2.0, 2.0) and checking for seams
  - Power-of-2 resolutions (512, 1024, 2048) optimize GPU memory
- **Channel Usage**:
  - Don't paint lighting/shadows in albedo - use normal maps instead
  - ORM packing saves memory: combine occlusion, roughness, metallic into one RGB texture
  - Depth maps are expensive - use only when parallax effect is essential

### Manifest Organization

- Group related profiles together with comments
- Use consistent naming conventions: `object_class/variant`
- Document complex fallback chains
- Add notes to level overrides explaining theme

### Performance

- Reuse profiles across levels when possible
- Use texture atlases for small repeated patterns
- Prefer lower-resolution textures for background elements
- Monitor WASM builds for asset size (use KTX2 compression)

## Advanced Features

### Fallback Chains

Create resilient material loading with fallback chains:

```rust
(
  id: "brick/exotic",
  albedo_path: "exotic_unavailable.png",
  // ... other fields ...
  fallback_chain: ["brick/metal", "brick/default"],
)
```

If `exotic_unavailable.png` fails, tries `brick/metal`, then `brick/default`.

### Emissive Materials

Add glow effects to bricks or balls:

```rust
(
  object_class: Brick,
  type_id: 5,
  profile_id: "brick/glowing",
  emissive_color: Some(Srgba(red: 0.0, green: 1.0, blue: 0.8, alpha: 1.0)),
  animation: None,
)
```

### Tint Modifiers

Create color variations without new textures:

```rust
(
  level_number: 3,
  ground_profile: Some("ground/default"),
  background_profile: None,
  sidewall_profile: None,
  tint: Some(Srgba(red: 0.8, green: 0.8, blue: 1.2, alpha: 1.0)), // Blueish tint
  notes: Some("Moonlight theme"),
)
```

## Contract API (For Tooling)

External tools can interact with the texture system via events:

### Preview Asset (Temporary Override)

Send `PreviewVisualAsset` event with a profile to test textures without editing manifest:

```rust
PreviewVisualAsset {
    profile: VisualAssetProfile {
        id: "test/preview".to_string(),
        albedo_path: "preview_texture.png".to_string(),
        // ... other fields ...
    },
    persist: false, // Don't save to manifest
}
```

This enables external editors to preview textures in real-time.

## Examples

### Complete Manifest Example

```rust
(
  profiles: [
    // Canonical defaults
    (
      id: "ball/default",
      albedo_path: "ball_default.png",
      normal_path: None,
      roughness: 0.3,
      metallic: 0.0,
      uv_scale: (1.0, 1.0),
      uv_offset: (0.0, 0.0),
      fallback_chain: [],
    ),

    // Custom variants
    (
      id: "brick/wood",
      albedo_path: "wood_planks.png",
      normal_path: Some("wood_normal.png"),
      roughness: 0.9,
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
      profile_id: "brick/wood",
      emissive_color: None,
      animation: None,
    ),
  ],

  level_overrides: [
    (
      level_number: 2,
      ground_profile: Some("ground/grass"),
      background_profile: Some("background/sky"),
      sidewall_profile: Some("sidewall/wood"),
      tint: Some(Srgba(red: 1.0, green: 1.0, blue: 0.9, alpha: 1.0)),
      notes: Some("Outdoor garden theme"),
    ),
  ],
)
```

## Getting Help

- Check game logs for detailed error messages
- Verify RON syntax with online validators
- Review existing profiles in manifest for examples
- Test changes incrementally with hot-reload

<!-- INCLUSION-MARKER-END-DO-NOT-REMOVE -->
## Version History

- **2025-11-27**: Initial texture system guide
  - Basic profile configuration
  - Type-variant mapping
  - Per-level overrides
  - Hot-reload workflow
