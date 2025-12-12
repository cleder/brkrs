# Texture Asset Guide for Brkrs

This guide explains how to add, modify, and manage textures for the Brkrs breakout game.

## Overview

The game uses a **texture manifest system** (`manifest.ron`) to define all visual materials for gameplay objects.
Textures can be customized globally or overridden per-level for unique visual themes.

## Quick Start

### Adding a New Texture

1. **Place your texture file** in `assets/textures/` or a subdirectory
   - Supported formats: PNG, KTX2
   - Recommended naming: descriptive lowercase with underscores (e.g., `metal_rough.png`)

2. **Add a profile entry** to `manifest.ron`:

```ron
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

```ron
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

```ron
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

```ron
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

```ron
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

## Manifest Schema Reference

### VisualAssetProfile

Defines a complete material profile.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | String | *required* | Unique identifier (e.g., "brick/metal") |
| `albedo_path` | String | *required* | Path to base color texture |
| `normal_path` | `Option<String>`|`None`| Optional normal map for surface detail. **Important**: Must be a proper normal map (RGB values representing XYZ normals in tangent space), not a bump map. If you have a grayscale bump map, convert it to a normal map using image editing software (e.g., GIMP: Filters → Generic → Normal Map). Bevy does not automatically convert bump maps to normal maps. |
|`roughness`| f32 |`0.5`| Surface roughness (0.0 = mirror, 1.0 = matte) |
|`metallic`| f32 |`0.0`| Metallic property (0.0 = non-metal/dielectric, 1.0 = fully metal). Controls how reflective and mirror-like a surface appears. Combined with `roughness`: low metallic + low roughness = shiny plastic; high metallic + low roughness = polished mirror-like metal; high metallic + high roughness = brushed/worn metal. |
|`uv_scale`| (f32, f32) |`(1.0, 1.0)`| Texture tiling factors (x, y) |
|`uv_offset`| (f32, f32) |`(0.0, 0.0)`| Texture offset (x, y) |
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
- **Format**: PNG for quick iteration, KTX2 for production (smaller, faster)
- **Compression**: Keep PNG files under 1MB for quick loading
- **UV mapping**: Design textures to tile seamlessly for `uv_scale > 1.0`

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

```ron
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

```ron
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

```ron
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

```ron
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

## Version History

- **2025-11-27**: Initial texture system guide
  - Basic profile configuration
  - Type-variant mapping
  - Per-level overrides
  - Hot-reload workflow
