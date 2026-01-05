# Feature Specification: Enhanced Brick Material Textures

**Feature Branch**: `017-brick-material-textures` **Created**: 2026-01-04 **Status**: Draft **Input**: User description: "In addition to the albedo and normal map the brick materials should also have the remaining StandardMaterial textures, perceptual_roughness, emissive, depth"

## Clarifications

### Session 2026-01-04

- Q: Should occlusion maps (ambient occlusion) be added to the feature scope? → A: Include occlusion maps as a fourth texture type with P1.5 priority (after roughness, before emissive) - adds `occlusion_path` field and maps to StandardMaterial's `occlusion_texture`
- Q: How should roughness and occlusion maps be stored and loaded? → A: Packed texture - single `orm_path` (Occlusion-Roughness-Metallic) texture where red=occlusion, green=roughness, blue=metallic (glTF 2.0 standard)
- Q: Should the ORM texture be assigned to both `metallic_roughness_texture` and `occlusion_texture` fields, or split into separate textures? → A: Use the same ORM texture for both fields - assign `orm_path` texture to both `metallic_roughness_texture` and `occlusion_texture` (shader extracts correct channels)
- Q: What color space settings should be used when loading the ORM and depth textures? → A: Linear (non-sRGB) for ORM and depth, sRGB for emissive (matches normal map pattern, correct PBR)
- Q: How should UV transforms (scale and offset) be applied to the new texture maps? → A: All new textures use the same UV transform as existing albedo/normal (single `uv_scale` and `uv_offset` per profile, applied to all maps)

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, UI updates, or hierarchy, the implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include at least one check that guards against prohibited patterns (e.g., panicking queries or per-frame UI updates without `Changed<T>`).
Acceptance criteria MUST explicitly state which event system is used (Messages vs Observers), justify the choice, and check for **Message-Event Separation** (correct use of `MessageWriter` vs observers/ `Trigger<T>`) and **Hierarchy Safety** (use of `commands.entity(parent).add_child(child)` or `EntityCommands::set_parent`).

### User Story 1 - Add Roughness and Occlusion Map Support (Priority: P1)

Level designers can specify a packed ORM texture (Occlusion-Roughness-Metallic) to create visually realistic brick surfaces with varying roughness patterns and ambient occlusion, following industry-standard glTF 2.0 PBR workflow.

**Why this priority**: Packed ORM textures are fundamental to physically-based rendering (PBR) and provide significant visual improvement while being memory-efficient (one texture instead of three separate ones).
Industry-standard format used across game engines and 3D tools.

**Independent Test**: Can be fully tested by creating a single brick profile with an ORM texture, spawning a brick with that profile, and visually verifying both roughness variation and darkened occlusion areas appear on the brick surface.

**Acceptance Scenarios**:

1. **Given** a VisualAssetProfile with `orm_path` set to a valid packed ORM texture file, **When** the manifest is loaded and a brick is spawned with that profile, **Then** the brick's StandardMaterial has both roughness (green channel) and occlusion (red channel) applied correctly
2. **Given** a VisualAssetProfile with both scalar `roughness` and `metallic` values plus `orm_path`, **When** a brick is spawned, **Then** the scalar values modulate the texture channels (multiplier behavior)
3. **Given** a VisualAssetProfile with `orm_path` pointing to a non-existent file, **When** the manifest loads, **Then** the system falls back to scalar roughness/metallic values and no occlusion, logging a warning (no crash)
4. **Given** a brick with an ORM texture, **When** viewed under lighting, **Then** both roughness variation (specular highlights) and occlusion darkening (crevices) are visible

---

### User Story 2 - Add Emissive Map Support (Priority: P2)

Level designers can specify emissive (glow) maps for bricks to create special visual effects like glowing patterns, neon signs, or energy-charged bricks without requiring separate lighting systems.

**Why this priority**: Emissive maps add visual polish and help distinguish special brick types (power-ups, multi-hit, etc.).
They work independently and don't affect gameplay logic, making them safe to implement after core PBR textures (ORM).

**Independent Test**: Can be fully tested by creating a brick profile with an emissive texture, spawning a brick, and verifying the brick has visible glow areas matching the emissive map pattern.

**Acceptance Scenarios**:

1. **Given** a VisualAssetProfile with `emissive_path` set to a valid texture file, **When** a brick is spawned with that profile, **Then** the brick's StandardMaterial has the emissive texture applied and glows in those areas
2. **Given** a VisualAssetProfile with both `emissive_color` (solid color) and `emissive_path` (texture), **When** a brick is spawned, **Then** the emissive texture is tinted by the emissive_color value (multiplicative combination)
3. **Given** a brick with an emissive map in a dark scene, **When** all directional lights are disabled, **Then** the emissive areas remain visible (self-illuminated)
4. **Given** a VisualAssetProfile with `emissive_path` pointing to a non-existent file, **When** the manifest loads, **Then** the system falls back to solid emissive_color (if set) or no emission and logs a warning

---

### User Story 3 - Add Depth/Parallax Map Support (Priority: P3)

Level designers can specify depth maps (parallax occlusion mapping) to create the illusion of surface depth on bricks without adding geometric complexity, such as deep mortar grooves or carved patterns.

**Why this priority**: Depth maps are the most advanced PBR feature and have the highest performance cost.
They provide visual polish but are not essential for basic PBR rendering.
Implementing after roughness and emissive ensures foundation is solid.

**Independent Test**: Can be fully tested by creating a brick profile with a depth map, spawning a brick, and verifying parallax offset is visible when viewing the brick at grazing angles.

**Acceptance Scenarios**:

1. **Given** a VisualAssetProfile with `depth_path` set to a valid texture file, **When** a brick is spawned with that profile, **Then** the brick's StandardMaterial has the depth texture applied for parallax occlusion mapping
2. **Given** a brick with a depth map, **When** the camera views the brick at a shallow angle, **Then** the parallax effect is visible (surface appears to have depth/grooves)
3. **Given** a VisualAssetProfile with `depth_path` and a `depth_scale` parameter, **When** a brick is spawned, **Then** the depth_scale controls the intensity of the parallax effect
4. **Given** a VisualAssetProfile with `depth_path` pointing to a non-existent file, **When** the manifest loads, **Then** the system falls back to no depth mapping and logs a warning

---

### Edge Cases

- What happens when an ORM/emissive/depth texture file is deleted or moved after the manifest loads?
  - System should handle missing assets gracefully via Bevy's asset system (weak handles or default fallback)

- What happens when orm_path and scalar roughness/metallic are both set?
  - The scalars act as multipliers on the texture channel values (standard PBR behavior)

- What happens when emissive_path is set but no emissive_color is specified?
  - The emissive texture is used with white (1,1,1) as the default tint color

- What happens when depth maps are used on low-end devices or with high brick counts?
  - Depth mapping should be optional and can be disabled via settings; fallback to normal maps only

- What happens when a texture has incorrect format (wrong dimensions, bit depth)?
  - Bevy's asset loader handles format conversion; if conversion fails, fallback to default values and log warning

- What happens when an ORM texture has occlusion in the wrong channel?
  - The visual result will be incorrect; designers must use proper glTF 2.0 ORM format (red=occlusion, green=roughness, blue=metallic)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support optional `orm_path` field in VisualAssetProfile to specify a packed ORM (Occlusion-Roughness-Metallic) texture following glTF 2.0 standard
- **FR-002**: System MUST support optional `emissive_path` field in VisualAssetProfile to specify an emissive (glow) map texture
- **FR-003**: System MUST support optional `depth_path` field in VisualAssetProfile to specify a depth/parallax map texture
- **FR-004**: System MUST apply ORM texture to StandardMaterial's `metallic_roughness_texture` field and extract red channel for `occlusion_texture` when `orm_path` is specified
- **FR-005**: System MUST apply emissive map textures to StandardMaterial's `emissive_texture` field when `emissive_path` is specified
- **FR-006**: System MUST apply depth map textures to StandardMaterial's `depth_map` field when `depth_path` is specified
- **FR-007**: System MUST use scalar `roughness` and `metallic` values as multipliers when both scalars and `orm_path` are specified
- **FR-008**: System MUST combine `emissive_color` and `emissive_path` multiplicatively when both are specified (texture tinted by color)
- **FR-009**: System MUST support a `depth_scale` parameter to control the intensity of parallax occlusion mapping (default: 0.1)
- **FR-010**: System MUST load ORM textures as linear (non-sRGB) textures, matching Bevy's PBR pipeline requirements
- **FR-011**: System MUST load emissive maps as sRGB textures (color data)
- **FR-012**: System MUST load depth maps as linear (non-sRGB) grayscale textures
- **FR-013**: System MUST handle missing texture files gracefully by falling back to scalar values or defaults without crashing
- **FR-014**: System MUST log warnings when texture files specified in manifest cannot be loaded
- **FR-015**: System MUST apply the same UV transform (scale, offset) to all texture maps (albedo, normal, ORM, emissive, depth) to maintain alignment
- **FR-016**: Existing bricks with only albedo and normal maps MUST continue to work without modification (backward compatibility)

### Key Entities

- **VisualAssetProfile**: Extended with three new optional fields:
  - `orm_path: Option<String>` - Path to packed ORM (Occlusion-Roughness-Metallic) texture following glTF 2.0 standard (red=occlusion, green=roughness, blue=metallic)
  - `emissive_path: Option<String>` - Path to emissive map texture relative to assets/textures/
  - `depth_path: Option<String>` - Path to depth/parallax map texture relative to assets/textures/
  - `depth_scale: f32` - Intensity multiplier for parallax effect (default: 0.1)

- **StandardMaterial (Bevy built-in)**: Target material type receiving the new texture maps:
  - `metallic_roughness_texture` - Receives ORM texture (red=occlusion in some pipelines, green=roughness, blue=metallic)
  - `occlusion_texture` - Receives the same ORM texture (red channel extracted for occlusion)
  - `emissive_texture` - Receives emissive map
  - `depth_map` - Receives depth/parallax map

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Bricks with ORM textures display both visible roughness variation (specular highlights) and occlusion darkening (crevices) under lighting conditions (verified by visual inspection)
- **SC-002**: Bricks with emissive maps glow in specified areas and remain visible in unlit scenes (verified by visual inspection with lights disabled)
- **SC-003**: Bricks with depth maps show parallax effect when viewed at angles less than 45 degrees from the surface (verified by camera movement)
- **SC-004**: Manifest files with new texture fields load successfully without errors (verified by test suite)
- **SC-005**: Existing manifest files without new fields continue to work without modification (backward compatibility verified by running existing tests)
- **SC-006**: Missing texture files result in logged warnings but do not cause crashes or missing visuals (verified by test with intentionally broken paths)
- **SC-007**: All three new texture types can be used independently (ORM-only, emissive-only, depth-only) or in combination without conflicts (verified by test matrix)

## Assumptions *(optional)*

- Bevy 0.17.3's StandardMaterial supports all required texture fields (metallic_roughness_texture, emissive_texture, depth_map)
- Level designers understand PBR texture authoring (what makes a valid roughness/emissive/depth map)
- Texture files will be provided in standard formats (PNG, KTX2) compatible with Bevy's asset loader
- Performance impact of additional textures is acceptable for target platforms (modern desktop/web)
- The existing `make_material` function in materials.rs is the appropriate place to add new texture loading logic

## Dependencies *(optional)*

- Bevy 0.17.3 rendering pipeline (already integrated)
- Existing texture manifest system (`VisualAssetProfile`, `ProfileMaterialBank`, etc.)
- Bevy's asset loading system for texture files
- Existing fallback/error handling infrastructure in the texture system

## Out of Scope *(optional)*

- Automatic conversion of bump maps to normal maps or depth maps (designers must provide proper texture formats)
- Dynamic generation or procedural textures (all textures must be pre-authored files)
- Per-brick runtime texture swapping or animation (static textures only)
- Advanced shader effects beyond standard PBR (custom shaders, screen-space effects, etc.)
- Metallic maps as separate textures (Bevy combines metallic and roughness in one texture; metallic remains a scalar value)
- Performance optimization or LOAD system for texture-heavy scenes (assumed acceptable performance)
- Texture compression or format conversion tooling (designers use existing tools)
