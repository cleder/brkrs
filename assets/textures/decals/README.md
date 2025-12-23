# Brick Decals

This directory contains the decal assets for the brick-type-decals feature, which provides visual brick type identification and 3D embossed effects.

## Overview

Decals are visual indicators centered on the top side of bricks that help players:

1. **Identify brick types** at a glance (User Story 1)
2. **See embossed/engraved effects** using normal/bump mapping (User Story 2)

## How It Works

### System Architecture

The decal system consists of:

- **Decal Component**: Stores brick type and normal map handle
- **Assignment Systems**: `assign_brick_decals` and `assign_brick_decals_fallback`
- **Asset Loading**: Automatic loading of type-specific normal maps
- **Rendering**: Normal maps provide 3D depth effects under lighting

### Integration Points

- **Level Loading**: Decals are assigned during level loading in `src/level_loader.rs`
- **System Scheduling**: Decal systems run in the Update schedule
- **Asset Management**: Normal maps are loaded via Bevy's `AssetServer`

## Asset Requirements

### File Naming Convention

Normal map files must follow this pattern:

```text
{brick_type}_normal.png
```

Where `brick_type` matches the `BrickType` enum variants (lowercase):

- `standard_normal.png` - Standard destructible bricks
- `indestructible_normal.png` - Indestructible bricks
- `multihit_normal.png` - Multi-hit bricks

### Asset Specifications

- **Format**: PNG (recommended for normal maps)
- **Color Space**: Should contain normal vectors (typically blue-tinted)
- **Resolution**: Match the decal texture resolution
- **Compression**: Use lossless compression to preserve normal data

### Current Assets

- `standard_normal.png` - Normal map for standard bricks
- `indestructible_normal.png` - Normal map for indestructible bricks
- `multihit_normal.png` - Normal map for multi-hit bricks

## Adding New Brick Types

To add decals for a new brick type:

1. **Add to BrickType enum** in `src/level_format/brick_types.rs`:

   ```rust
   pub enum BrickType {
       Standard,
       Indestructible,
       MultiHit,
       NewType,  // Add new variant
   }
   ```

2. **Update from_id mapping** in the same file:

   ```rust
   pub fn from_id(id: u8) -> Option<Self> {
       match id {
           3 => Some(BrickType::Standard),
           4 => Some(BrickType::Indestructible),
           // Add mapping for new type
           5 => Some(BrickType::NewType),
           _ => None,
       }
   }
   ```

3. **Add normal map asset** following the naming convention:

   ```text
   assets/textures/decals/newtype_normal.png
   ```

4. **Update create_decal_for_type function** in `src/systems/brick_decals.rs`:

   ```rust
   fn create_decal_for_type(brick_type: &BrickType, asset_server: &AssetServer) -> Decal {
       let normal_map_path = match brick_type {
           BrickType::Standard => "textures/decals/standard_normal.png",
           BrickType::Indestructible => "textures/decals/indestructible_normal.png",
           BrickType::MultiHit => "textures/decals/multihit_normal.png",
           BrickType::NewType => "textures/decals/newtype_normal.png",  // Add new case
       };

       Decal {
           brick_type: *brick_type,
           normal_map_handle: Some(asset_server.load(normal_map_path)),
       }
   }
   ```

## Technical Details

### Normal Mapping

Normal maps encode surface normals as RGB values:

- **Red (X)**: Left/right surface orientation
- **Green (Y)**: Up/down surface orientation
- **Blue (Z)**: Depth/height information

The blue channel typically dominates for embossed effects, with:

- **High blue values** (near 255): Surface protrudes outward
- **Low blue values** (near 0): Surface recesses inward

### Rendering Integration

Decals are rendered as part of the brick materials using:

- **StandardMaterial** with normal map textures
- **Lighting calculations** that respect surface normals
- **Consistent appearance** from all viewing angles

### Performance Considerations

- **Asset Reuse**: Normal map handles are cached and reused
- **Lazy Loading**: Assets load asynchronously via AssetServer
- **Memory Management**: Assets are managed by Bevy's asset system

## Testing

The decal system includes comprehensive tests:

- **Contract Tests**: Verify decal assignment and normal mapping
- **Integration Tests**: Confirm visual positioning and 3D effects
- **Compliance Tests**: Ensure Bevy 0.17 best practices

Run tests with:

```bash
cargo test --test test_brick_decals
cargo test --test test_decal_rendering
cargo test --test test_decal_normals
cargo test --test test_decal_normals_integration
```

## Troubleshooting

### Common Issues

1. **Decals not appearing**: Check that normal map assets exist and paths are correct
2. **No 3D effect**: Ensure normal maps contain proper normal vector data
3. **Lighting issues**: Verify scene has appropriate lighting for normal mapping
4. **Asset loading errors**: Check console for AssetServer loading failures

### Debug Tips

- Enable asset loading logging to see load status
- Use Bevy's asset inspector to verify normal maps load correctly
- Check brick entities have both `BrickType` and `Decal` components

## Future Enhancements

Potential improvements:

- **Albedo textures**: Add diffuse color maps for more detailed decals
- **Roughness/Metallic**: PBR material properties for advanced rendering
- **Animation**: Animated decals for special effects
- **LOAD**: Level-of-detail normal maps for performance
- **Procedural generation**: Runtime decal creation for dynamic content</content>
<parameter name="filePath">/home/christian/devel/bevy/brkrs/assets/textures/decals/README.md
