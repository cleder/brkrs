# Research: Enhanced Brick Material Textures

**Feature**: 017-brick-material-textures **Date**: 2026-01-04 **Phase**: 0 (Research & Clarification)

## Research Questions Resolved

### 1. ORM Texture Format and Channel Assignment

**Question**: How should occlusion, roughness, and metallic data be packed into a single texture, and which channels should be used?

**Decision**: Follow glTF 2.0 standard ORM texture format

- **Red channel**: Ambient Occlusion
- **Green channel**: Roughness
- **Blue channel**: Metallic

**Rationale**:

- Industry standard used by Blender, Substance Painter, and all major 3D tools
- Bevy's StandardMaterial is designed for glTF 2.0 compliance
- Memory efficient (3 data channels in one texture instead of 3 separate files)
- Automatic channel extraction by Bevy's PBR shader

**Alternatives Considered**:

- Separate texture files (rejected: wastes memory, requires extra asset loads, alignment issues)
- Different channel order (rejected: breaks interoperability with standard authoring tools)

### 2. StandardMaterial Field Assignment Strategy

**Question**: Should the ORM texture be assigned to both `metallic_roughness_texture` and `occlusion_texture` fields, or split?

**Decision**: Assign the same ORM texture handle to both fields

**Rationale**:

- Bevy's shader automatically extracts the correct channel from each field
- Standard practice in Bevy and glTF 2.0 pipelines
- Simpler implementation (one texture load, two field assignments)
- No duplicate texture data in memory

**Alternatives Considered**:

- Split channels into separate textures (rejected: requires custom image processing, increases complexity)
- Only assign to metallic_roughness_texture (rejected: occlusion wouldn't be applied)

### 3. Color Space Settings for PBR Textures

**Question**: What color space (sRGB vs linear) should be used when loading different texture types?

**Decision**: Linear (non-sRGB) for ORM and depth; sRGB for emissive

**Rationale**:

- **ORM textures contain data, not color**: Linear space preserves numeric values for calculations
- **Depth maps contain data**: Height values must be linear for parallax math
- **Emissive textures contain color**: sRGB space matches how artists author glowing colors
- Matches the existing pattern for normal maps (`is_srgb = false`)
- Follows Bevy's PBR pipeline requirements and glTF 2.0 specification

**Alternatives Considered**:

- sRGB for all textures (rejected: incorrect for data textures, causes visual artifacts)
- Auto-detect from file metadata (rejected: unreliable, depends on artist workflow)

### 4. UV Transform Application

**Question**: Should UV transforms (scale, offset) apply to all textures or be independent per texture type?

**Decision**: Apply the same UV transform to all texture maps (albedo, normal, ORM, emissive, depth)

**Rationale**:

- PBR materials require pixel-perfect alignment between all maps
- Different UV transforms cause visual artifacts (roughness in wrong places, misaligned occlusion)
- Standard practice in all game engines and 3D authoring tools
- Simpler for artists (one UV transform setting per material profile)

**Alternatives Considered**:

- Independent UV transforms per texture (rejected: complex, high risk of artist error)
- No UV transform on new textures (rejected: would break alignment with albedo/normal)

### 5. Backward Compatibility Strategy

**Question**: How should existing manifest files (with only albedo and normal) continue to work?

**Decision**: Make all new fields (`orm_path`, `emissive_path`, `depth_path`) optional with `#[serde(default)]`

**Rationale**:

- Existing profiles without new fields will deserialize successfully
- Default `None` values mean no additional textures are loaded
- Scalar roughness/metallic values continue to work as fallbacks
- No migration needed for existing content

**Alternatives Considered**:

- Required fields with migration (rejected: breaks existing levels, requires content update)
- Separate profile types (rejected: code duplication, complexity)

## Technology Decisions

### Bevy 0.17.3 StandardMaterial Fields

**Verified Capabilities**:

- `metallic_roughness_texture: Option<Handle<Image>>` - Supports ORM texture
- `occlusion_texture: Option<Handle<Image>>` - Supports occlusion map
- `emissive_texture: Option<Handle<Image>>` - Supports emissive/glow map
- `depth_map: Option<Handle<Image>>` - Supports parallax occlusion mapping
- `uv_transform: Affine2` - Already applied to base_color and normal; will apply to new textures

**Source**: Bevy 0.17.3 documentation and StandardMaterial struct definition

### Asset Loading API

**Color Space Control**:

```rust
// Linear (non-sRGB) for data textures
asset_server.load_with_settings(
    path,
    |settings: &mut ImageLoaderSettings| settings.is_srgb = false,
)

// sRGB for color textures (default)
asset_server.load(path)  // is_srgb = true by default
```

**UV Transform Application**:

```rust
use bevy::math::Affine2;
let uv_transform = Affine2::from_scale_angle_translation(
    profile.uv_scale,
    0.0,  // rotation (not used)
    profile.uv_offset
);
```

### Fallback Behavior

**Strategy**: Use existing `FallbackRegistry` pattern

- Missing ORM texture → use scalar roughness/metallic, no occlusion
- Missing emissive texture → no emission (or solid color if emissive_color is set)
- Missing depth texture → no parallax effect
- Log warnings for missing files (already implemented)

## Best Practices

### Texture Authoring Guidelines

For level designers creating ORM textures:

1. Use Blender, Substance Painter, or similar tool to export ORM textures
2. Ensure channel assignment matches glTF 2.0 standard (R=AO, G=Roughness, B=Metallic)
3. Save as PNG or KTX2 format (Bevy compatible)
4. Test under different lighting conditions to verify all channels work

### Performance Considerations

- **Memory**: ORM texture uses 3 channels efficiently; no performance penalty vs separate files
- **Loading**: Three texture loads per profile (ORM, emissive, depth) instead of one (albedo)
- **Rendering**: StandardMaterial shader already handles all these texture types; no performance impact
- **WASM**: Texture formats (PNG/KTX2) work identically on web and native platforms

### Testing Strategy

1. **Unit Tests**: VisualAssetProfile deserialization with new optional fields
2. **Integration Tests**:
   - Load manifest with ORM texture, verify material has correct handles
   - Test fallback behavior when texture files are missing
   - Verify color space settings (linear vs sRGB)
   - Test backward compatibility (old manifest files still work)
3. **Visual Tests**: Manual verification of roughness variation, occlusion darkening, emissive glow, parallax effect

## Integration Points

### Existing Code to Modify

1. **`src/systems/textures/loader.rs`**:
   - Add `orm_path`, `emissive_path`, `depth_path` to `VisualAssetProfile` struct
   - All fields use `#[serde(default)]` for backward compatibility

2. **`src/systems/textures/contracts.rs`**:
   - Add same fields to `VisualAssetProfileContract` (API contract definition)

3. **`src/systems/textures/materials.rs`**:
   - Modify `make_material()` function to load ORM/emissive/depth textures
   - Apply color space settings correctly
   - Assign ORM texture to both `metallic_roughness_texture` and `occlusion_texture`
   - Apply UV transform to all texture maps

### No Changes Required

- `ProfileMaterialBank` - Already handles any StandardMaterial configuration
- `TypeVariantRegistry` - Already maps type IDs to material handles
- Asset loading systems - Already reactive to manifest changes
- Level loading - Automatically uses new materials when available

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Artists create ORM textures with wrong channel order | Visual artifacts | Document glTF 2.0 standard clearly; provide example textures |
| Texture file size increases | Longer load times, more memory | Use compressed formats (KTX2); document texture size best practices |
| Color space errors (sRGB vs linear) | Incorrect rendering | Explicit settings in code; automated tests verify color space |
| UV transform not applied to new textures | Misaligned textures | Apply same transform to all; integration test verifies alignment |

## Summary

All research questions have been resolved.
The implementation approach is straightforward:

1. Add three optional fields to `VisualAssetProfile` and its contract
2. Modify `make_material()` to load additional textures with correct color space settings
3. Assign textures to appropriate StandardMaterial fields
4. Apply existing UV transform to all texture maps
5. Maintain backward compatibility through optional fields

No architectural changes or new systems required.
Feature extends existing asset loading infrastructure using established patterns.
