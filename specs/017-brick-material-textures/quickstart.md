# Quick Start: Advanced PBR Textures for Bricks

**Feature**: 017-brick-material-textures **Date**: 2026-01-04 **Audience**: Level designers, texture artists, content creators

## What's New

Brick materials now support the full Physically-Based Rendering (PBR) pipeline:

- **ORM Textures**: Occlusion-Roughness-Metallic packed into one texture (glTF 2.0 standard)
- **Emissive Maps**: Make bricks glow with custom patterns
- **Depth Maps**: Add parallax/depth effects for enhanced realism

All new texture types are **optional** — existing profiles continue to work without changes.

---

## Basic Usage

### Step 1: Prepare Your Textures

Create textures in your image editor (GIMP, Photoshop, Substance Painter):

1. **ORM Texture** (optional):
   - **Red Channel**: Ambient Occlusion (0=fully occluded, 255=no occlusion)
   - **Green Channel**: Roughness (0=smooth/mirror, 255=rough/matte)
   - **Blue Channel**: Metallic (0=non-metal, 255=full metal)
   - **Format**: PNG or KTX2, **linear color space**
   - **Resolution**: Match albedo resolution (e.g., 512×512, 1024×1024)

2. **Emissive Texture** (optional):
   - RGB color for glow/emission
   - **Format**: PNG or KTX2, **sRGB color space**
   - Black areas don't emit light

3. **Depth Texture** (optional):
   - Grayscale (white=high, black=low)
   - **Format**: PNG or KTX2, **linear color space**
   - Used for parallax mapping

### Step 2: Save Textures to Assets

Place textures in `assets/textures/`:

```text
assets/
  textures/
    brick_stone_albedo.png
    brick_stone_normal.png
    brick_stone_orm.png       ← NEW
    brick_stone_emissive.png  ← NEW
    brick_stone_depth.png     ← NEW
```

### Step 3: Update manifest.ron

Edit `assets/textures/manifest.ron` to add new texture paths:

```ron
[
  // Full PBR profile with all textures
  (
    id: "brick/stone",
    albedo_path: "brick_stone_albedo.png",
    normal_path: Some("brick_stone_normal.png"),
    orm_path: Some("brick_stone_orm.png"),           // ← NEW
    emissive_path: Some("brick_stone_emissive.png"), // ← NEW
    depth_path: Some("brick_stone_depth.png"),       // ← NEW
    roughness: 1.0,   // Multiplies green channel of ORM
    metallic: 0.0,    // Multiplies blue channel of ORM
    uv_scale: (2.0, 2.0),
    uv_offset: (0.0, 0.0),
    fallback_chain: ["brick/default"],
  ),
]
```

### Step 4: Reference Profile in Level

In your level definition (e.g., `assets/levels/level_01.ron`), reference the profile:

```ron
(
  bricks: [
    (
      position: (0.0, 1.0, 0.0),
      profile: "brick/stone",  // Uses the full PBR profile
      // ...
    ),
  ],
)
```

---

## Common Patterns

### Pattern 1: Minimal Upgrade (Add ORM Only)

If you already have albedo and normal maps, add just the ORM texture:

```ron
(
  id: "brick/stone_upgraded",
  albedo_path: "brick_stone_albedo.png",
  normal_path: Some("brick_stone_normal.png"),
  orm_path: Some("brick_stone_orm.png"),  // ← Add this
  roughness: 1.0,
  metallic: 0.0,
  uv_scale: (1.0, 1.0),
  uv_offset: (0.0, 0.0),
  fallback_chain: [],
)
```

**Result**: Better surface variation from occlusion map, per-pixel roughness/metallic control.

### Pattern 2: Emissive-Only (Glowing Bricks)

For special effects like neon signs or power-ups:

```ron
(
  id: "brick/neon",
  albedo_path: "neon_base.png",
  emissive_path: Some("neon_glow.png"),  // ← Only add emissive
  roughness: 0.3,  // Smooth surface for neon look
  metallic: 0.0,
  uv_scale: (1.0, 1.0),
  uv_offset: (0.0, 0.0),
  fallback_chain: ["brick/default"],
)
```

**Result**: Brick emits light with custom pattern from emissive map.

### Pattern 3: Metallic Bricks

For metallic surfaces (steel, chrome, gold bricks):

```ron
(
  id: "brick/steel",
  albedo_path: "steel_albedo.png",
  normal_path: Some("steel_normal.png"),
  orm_path: Some("steel_orm.png"),  // Blue channel = high metallic
  roughness: 0.2,   // Multiplier (makes it shinier)
  metallic: 1.0,    // Multiplier (ensures full metallic)
  uv_scale: (1.0, 1.0),
  uv_offset: (0.0, 0.0),
  fallback_chain: ["brick/default"],
)
```

**Result**: Realistic metallic reflections.

### Pattern 4: Backward Compatible (No Changes)

Existing profiles continue to work without modification:

```ron
(
  id: "brick/old_style",
  albedo_path: "old_brick.png",
  normal_path: Some("old_normal.png"),
  // No ORM, emissive, or depth → defaults to None
  roughness: 0.7,
  metallic: 0.0,
  uv_scale: (1.0, 1.0),
  uv_offset: (0.0, 0.0),
  fallback_chain: [],
)
```

**Result**: Works exactly as before.

---

## Creating ORM Textures

### Option A: Photoshop/GIMP

1. Create three grayscale layers:
   - **Occlusion**: Bake AO in Blender/Substance, or paint manually
   - **Roughness**: Paint rough areas white, smooth areas black
   - **Metallic**: Paint metal areas white, non-metal black

2. Export as PNG using **Channels to Layers** plugin:
   - Red channel = Occlusion layer
   - Green channel = Roughness layer
   - Blue channel = Metallic layer

3. Save as `brick_orm.png` (linear color space)

### Option B: Substance Painter

1. Export textures using **glTF 2.0 Metallic Roughness** preset
2. Substance automatically creates `<name>_occlusionRoughnessMetallic.png`
3. Rename to `brick_orm.png` and copy to `assets/textures/`

### Option C: Blender (Bake from 3D Model)

1. Unwrap your brick model (UV mapping)
2. Bake **Ambient Occlusion** to Red channel
3. Bake **Roughness** to Green channel (or use shader node value)
4. Bake **Metallic** to Blue channel (or use shader node value)
5. Combine in Image Editor, export as PNG (linear color space)

---

## Troubleshooting

### Problem: Textures Look Too Dark/Bright

**Cause**: Wrong color space (sRGB vs linear)

**Solution**:

- **Albedo & Emissive**: sRGB color space (automatically handled)
- **ORM, Normal, Depth**: Linear color space (automatically handled)
- Verify export settings in image editor match requirements

### Problem: Roughness/Metallic Not Visible

**Cause**: Multiplier values too low

**Solution**: Increase `roughness` or `metallic` in manifest:

```ron
roughness: 1.0,  // Full intensity from texture
metallic: 1.0,   // Full intensity from texture
```

**Cause**: ORM texture channels incorrect

**Solution**: Verify channel order (Red=AO, Green=Roughness, Blue=Metallic)

### Problem: Emissive Map Not Glowing

**Cause**: Emissive color too dark

**Solution**: Use brighter colors in emissive texture (RGB values > 128)

**Cause**: Missing emissive_path in manifest

**Solution**: Add `emissive_path: Some("brick_emissive.png")` to profile

### Problem: Texture Load Error in Logs

**Cause**: File path incorrect or file missing

**Solution**:

- Check file exists in `assets/textures/`
- Verify path in manifest matches filename (case-sensitive)
- Check file extension (`.png` or `.ktx2` only)

**Example Error**:

```text
WARN texture_loader: Failed to load ORM texture: brick_orm.png (file not found)
```

**Fix**: Ensure `assets/textures/brick_orm.png` exists

---

## Best Practices

### Performance

1. **Use KTX2 for Production**:
   - Faster loading than PNG
   - GPU-compressed formats reduce VRAM usage
   - Convert with `basisu` or Substance Painter export

2. **Match Resolutions**:
   - Keep ORM, normal, depth same resolution as albedo
   - Avoid mixing 512×512 albedo with 2048×2048 ORM

3. **Optimize Emissive**:
   - Use lower resolution for simple glow patterns
   - Black areas are free (no emission)

### Quality

1. **Bake Occlusion from 3D**:
   - Hand-painted AO often looks fake
   - Bake in Blender/Substance for realistic contact shadows

2. **Test in Game**:
   - Lighting environment affects appearance
   - Adjust roughness/metallic multipliers in manifest

3. **Use Depth Sparingly**:
   - Only for close-up bricks where parallax is visible
   - Expensive shader operation

### Workflow

1. **Start Simple**: Add ORM first, then emissive, then depth
2. **Reuse Textures**: Share ORM across similar brick types
3. **Fallback Chain**: Always provide fallback to basic profile:

   ```ron
   fallback_chain: ["brick/default", "brick/fallback"],
   ```

---

## Reference Table

| Texture Type | Channels | Color Space | Required | Usage |
|--------------|----------|-------------|----------|-------|
| Albedo       | RGB(A)   | sRGB        | Yes      | Base color |
| Normal       | RGB      | Linear      | No       | Surface bumps |
| ORM          | R=AO, G=Rough, B=Metal | Linear | No | Surface properties |
| Emissive     | RGB      | sRGB        | No       | Glow/emission |
| Depth        | Grayscale | Linear     | No       | Parallax mapping |

## Field Reference

| Field | Type | Range | Default | Description |
|-------|------|-------|---------|-------------|
| `id` | String | - | (required) | Unique profile identifier |
| `albedo_path` | String | - | (required) | Base color texture path |
| `normal_path` | Option<String> | - | None | Normal map path |
| `orm_path` | Option<String> | - | None | ORM texture path (glTF 2.0) |
| `emissive_path` | Option<String> | - | None | Emissive map path |
| `depth_path` | Option<String> | - | None | Depth map path |
| `roughness` | f32 | 0.0–1.0 | 0.5 | Roughness multiplier |
| `metallic` | f32 | 0.0–1.0 | 0.0 | Metallic multiplier |
| `uv_scale` | (f32, f32) | - | (1.0, 1.0) | Texture tiling |
| `uv_offset` | (f32, f32) | - | (0.0, 0.0) | Texture offset |
| `fallback_chain` | Vec<String> | - | [] | Fallback profile IDs |

---

## Examples Gallery

### Example 1: Classic Brick (Albedo + Normal + ORM)

```ron
(
  id: "brick/classic",
  albedo_path: "classic_brick_albedo.png",
  normal_path: Some("classic_brick_normal.png"),
  orm_path: Some("classic_brick_orm.png"),
  roughness: 0.9,   // Rough surface
  metallic: 0.0,    // Non-metallic
  uv_scale: (2.0, 2.0),  // Tile 2x
  uv_offset: (0.0, 0.0),
  fallback_chain: ["brick/default"],
)
```

**Appearance**: Realistic brick with surface variation from AO and roughness.

### Example 2: Neon Sign Brick (Albedo + Emissive)

```ron
(
  id: "brick/neon_sign",
  albedo_path: "neon_sign_albedo.png",
  emissive_path: Some("neon_sign_glow.png"),
  roughness: 0.2,   // Smooth for neon look
  metallic: 0.0,
  uv_scale: (1.0, 1.0),
  uv_offset: (0.0, 0.0),
  fallback_chain: ["brick/default"],
)
```

**Appearance**: Brick with glowing letters/patterns from emissive map.

### Example 3: Polished Metal Brick (Full PBR)

```ron
(
  id: "brick/chrome",
  albedo_path: "chrome_albedo.png",
  normal_path: Some("chrome_normal.png"),
  orm_path: Some("chrome_orm.png"),
  emissive_path: Some("chrome_emissive.png"),  // Optional LED accents
  roughness: 0.1,   // Very smooth
  metallic: 1.0,    // Full metallic
  uv_scale: (1.0, 1.0),
  uv_offset: (0.0, 0.0),
  fallback_chain: ["brick/metal", "brick/default"],
)
```

**Appearance**: Mirror-like reflections with LED accents.

### Example 4: Stone Wall with Depth (Albedo + Normal + ORM + Depth)

```ron
(
  id: "brick/stone_wall",
  albedo_path: "stone_wall_albedo.png",
  normal_path: Some("stone_wall_normal.png"),
  orm_path: Some("stone_wall_orm.png"),
  depth_path: Some("stone_wall_depth.png"),
  roughness: 0.95,  // Very rough
  metallic: 0.0,
  uv_scale: (1.5, 1.5),
  uv_offset: (0.0, 0.0),
  fallback_chain: ["brick/default"],
)
```

**Appearance**: Deep surface detail from depth map, realistic stone lighting from ORM.

---

## Next Steps

1. **Read the Full Spec**: See `specs/017-brick-material-textures/spec.md` for complete requirements
2. **Explore Data Model**: Review `specs/017-brick-material-textures/data-model.md` for internal structure
3. **Check API Contract**: See `contracts/visual-asset-profile.md` for validation rules
4. **Run Tests**: Execute `cargo test` to verify texture loading works correctly

## Support

- **Documentation**: `docs/asset-format.md` for detailed texture guidelines
- **Issues**: Report bugs with texture loading on GitHub issue tracker
- **Community**: Ask questions in the project Discord/forum

---

**Happy texturing!** 🎨
