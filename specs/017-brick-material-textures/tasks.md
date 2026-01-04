# Task Breakdown: Enhanced Brick Material Textures

**Feature**: 017-brick-material-textures **Date**: 2026-01-04 **Implementation Strategy**: Test-Driven Development (TDD) - Write tests first, verify failures (red phase), then implement minimum code to pass tests (green phase)

**Total Tasks**: 49 **Task Breakdown by Phase**:

- Phase 1 (Setup): 6 tasks
- Phase 2 (Foundational): 7 tasks
- Phase 3 (US1 - ORM): 9 tasks (T014-T022)
- Phase 4 (US2 - Emissive): 8 tasks (T023-T029, includes T024b for FR-008)
- Phase 5 (US3 - Depth): 9 tasks (T030-T038)
- Phase 6 (Integration): 4 tasks (T039-T042)
- Phase 7 (Polish): 6 tasks (T043-T048, includes WASM verification)

---

## Dependency Graph

```text
Phase 1: Setup (no dependencies)
  ↓
Phase 2: Foundational Infrastructure (blocks all user stories)
  ├→ P1: ORM Textures (independent)
  ├→ P2: Emissive Maps (depends on Phase 2)
  └→ P3: Depth Maps (depends on Phase 2)
Phase 3: Polish & Cross-Cutting
```

**Story Completion Order**: US1 (ORM) → US2 (Emissive) → US3 (Depth) **Parallel Opportunities**:

- US1 and US2 can be implemented in parallel once Phase 2 is complete (separate texture fields)
- Tests can be written in parallel with planning phase for all stories

---

## Phase 1: Setup & Project Initialization

**Goal**: Initialize project structure and verify build environment.

- [ ] T001 Verify Rust toolchain 1.81 and Bevy 0.17.3 dependencies in Cargo.toml
- [ ] T002 Create test asset fixtures directory at `tests/fixtures/textures/`
- [ ] T003 [P] Create placeholder ORM texture file at `tests/fixtures/textures/test_orm.png` (256x256, R=0.5, G=0.7, B=0.3)
- [ ] T004 [P] Create placeholder emissive texture at `tests/fixtures/textures/test_emissive.png` (256x256, red glow pattern)
- [ ] T005 [P] Create placeholder depth texture at `tests/fixtures/textures/test_depth.png` (256x256, grayscale depth pattern)
- [ ] T006 Run `cargo test --lib` to verify baseline test infrastructure works

---

## Phase 2: Foundational Infrastructure

**Goal**: Establish data structure extensions and texture loading patterns.

- [X] T007 [P] Extend `VisualAssetProfile` struct in [src/systems/textures/loader.rs](src/systems/textures/loader.rs) with three optional fields: `orm_path: Option<String>`, `emissive_path: Option<String>`, `depth_path: Option<String>` with `#[serde(default)]`
- [X] T008 [P] Extend `VisualAssetProfileContract` struct in [src/systems/textures/contracts.rs](src/systems/textures/contracts.rs) with same three optional fields
- [X] T009 [P] Add `From<VisualAssetProfileContract>` and `From<VisualAssetProfile>` implementations in [src/systems/textures/loader.rs](src/systems/textures/loader.rs) to handle conversion with new fields
- [X] T010 Update [assets/textures/manifest.ron](assets/textures/manifest.ron) with test profile containing all three new texture paths
- [X] T011 [P] Add `depth_scale: f32` parameter to `VisualAssetProfile` with default value of 0.1
- [X] T012 Verify backward compatibility: Run existing brick profile loads without errors (existing manifest entries work unchanged)
- [X] T013 Run `cargo fmt --all` to ensure code style compliance

---

## Phase 3: User Story 1 - ORM Texture Support (P1)

**Goal**: Implement packed ORM (Occlusion-Roughness-Metallic) texture loading following glTF 2.0 standard.

**Story Description**: Level designers can specify an ORM texture to create visually realistic brick surfaces with varying roughness and ambient occlusion using a single packed texture.

**Independent Test Criteria**:

- ✅ Bricks with ORM textures deserialize without errors
- ✅ ORM texture loads with linear color space (not sRGB)
- ✅ ORM texture assigns to both `metallic_roughness_texture` and `occlusion_texture` StandardMaterial fields
- ✅ Scalar roughness/metallic values multiply texture channels when both are specified
- ✅ Missing ORM texture falls back gracefully with warning (no crash)
- ✅ Visual verification: roughness variation (specular) and occlusion darkening (crevices) both visible

---

### T014: Test ORM Texture Deserialization (RED phase)

- [X] T014 [US1] Create test file [tests/orm_textures.rs](tests/orm_textures.rs) with test `test_orm_path_deserialization_minimal`
  - Parse RON with `orm_path: Some("brick_orm.png")` field
  - Verify `VisualAssetProfile::orm_path` is `Some("brick_orm.png")`
  - **Expected Result**: TEST FAILS (orm_path field doesn't exist yet)
  - **Commit**: "RED: Failing test for ORM texture deserialization"

---

### T015: Implement ORM Texture Deserialization (GREEN phase)

- [X] T015 [US1] Extend `VisualAssetProfile` in [src/systems/textures/loader.rs](src/systems/textures/loader.rs) with `orm_path: Option<String>` field and `#[serde(default)]`
  - Run test T014 to verify it passes
  - **Commit**: "GREEN: ORM texture deserialization support"

---

### T016: Test ORM Texture Loading with Correct Color Space (RED phase)

- [X] T016 [US1] Create test `test_orm_texture_loading_linear_color_space` in [tests/orm_textures.rs](tests/orm_textures.rs)
  - Create profile with `orm_path: Some("tests/fixtures/textures/test_orm.png")`
  - Call `make_material()` function to load texture
  - Verify texture is loaded with `is_srgb=false` (linear color space) by checking asset loader settings
  - **Expected Result**: TEST FAILS (make_material doesn't load ORM texture yet)
  - **Commit**: "RED: Failing test for ORM texture loading"

---

### T017: Implement ORM Texture Loading (GREEN phase)

- [X] T017 [US1] Extend `make_material()` function in [src/systems/textures/materials.rs](src/systems/textures/materials.rs) to load ORM texture
  - Call `asset_server.load_with_settings(orm_path, ImageLoaderSettings { is_srgb: false, ... })` for ORM texture
  - Assign result to `StandardMaterial::metallic_roughness_texture`
  - Assign same handle to `StandardMaterial::occlusion_texture`
  - Handle `None` case gracefully (skip if orm_path is None)
  - Run test T016 to verify it passes
  - **Commit**: "GREEN: ORM texture loading with linear color space"

---

### T018: Test ORM Scalar Multiplier Behavior (RED phase)

- [X] T018 [US1] Create test `test_orm_scalar_multiplier` in [tests/orm_textures.rs](tests/orm_textures.rs)
  - Create profile with `orm_path: Some("test_orm.png")`, `roughness: 0.5`, `metallic: 0.7`
  - Call `make_material()` to create StandardMaterial
  - Verify `metallic` field is set to 0.7 and will multiply texture's blue channel
  - Verify `roughness` field is set to 0.5 and will multiply texture's green channel
  - **Expected Result**: TEST FAILS (metallic/roughness assignment may not be correct)
  - **Commit**: "RED: Failing test for ORM scalar multiplier"

---

### T019: Implement ORM Scalar Multiplier (GREEN phase)

- [X] T019 [US1] Extend `make_material()` in [src/systems/textures/materials.rs](src/systems/textures/materials.rs) to set `metallic` and `roughness` scalars
  - Set `StandardMaterial::metallic` to profile's `metallic` value
  - Set `StandardMaterial::perceptual_roughness` to profile's `roughness` value (or `roughness_factor` depending on Bevy version)
  - Verify these scalars multiply the texture channels in PBR shader
  - Run test T018 to verify it passes
  - **Commit**: "GREEN: ORM scalar multiplier implementation"

---

### T020: Test ORM Fallback Behavior (RED phase)

- [X] T020 [US1] Create test `test_orm_fallback_missing_file` in [tests/orm_textures.rs](tests/orm_textures.rs)
  - Create profile with `orm_path: Some("nonexistent.png")`
  - Call `make_material()` to load texture
  - Verify no panic occurs (graceful fallback)
  - Verify warning is logged (check logs or return value)
  - Verify StandardMaterial falls back to scalar values (metallic/roughness still applied)
  - **Expected Result**: TEST FAILS (no error handling yet)
  - **Commit**: "RED: Failing test for ORM fallback behavior"

---

### T021: Implement ORM Fallback with Error Handling (GREEN phase)

- [X] T021 [US1] Extend error handling in `make_material()` in [src/systems/textures/materials.rs](src/systems/textures/materials.rs)
  - Wrap `asset_server.load_with_settings()` in match/error handling
  - Log warning if texture load fails
  - Continue with scalar roughness/metallic values (skip texture assignment)
  - Run test T020 to verify it passes
  - **Commit**: "GREEN: ORM fallback with error handling"
  - **NOTE**: Bevy's asset server handles missing assets gracefully - returns a handle to missing asset without panic.
    Material still gets scalar values.
    No explicit error handling needed in make_material().

---

### T022: Visual Verification - ORM Roughness and Occlusion (Manual Test)

- [ ] T022 [US1] Spawn brick with ORM profile and verify visual appearance
  - Create brick with profile containing ORM texture
  - Verify specular highlights vary across surface (roughness variation from green channel)
  - Verify dark areas in crevices (occlusion darkening from red channel)
  - Verify effect is visible under directional lighting
  - Document with screenshot or manual inspection checklist
  - **Commit**: "VERIFIED: ORM visual appearance (roughness + occlusion)"

---

## Phase 4: User Story 2 - Emissive Map Support (P2)

**Goal**: Implement emissive texture loading for glowing brick surfaces.

**Story Description**: Level designers can specify an emissive map to create glowing effects like neon signs or power-up bricks without additional lighting.

**Independent Test Criteria**:

- ✅ Bricks with emissive textures deserialize without errors
- ✅ Emissive texture loads with sRGB color space
- ✅ Emissive texture assigns to `emissive_texture` StandardMaterial field
- ✅ Missing emissive texture falls back gracefully
- ✅ Visual verification: glowing areas visible in dim lighting, remain visible with lights disabled

---

### T023: Test Emissive Texture Deserialization (RED phase)

- [X] T023 [US2] Create test file [tests/emissive_textures.rs](tests/emissive_textures.rs) with test `test_emissive_path_deserialization_minimal`
  - Parse RON with `emissive_path: Some("brick_emissive.png")` field
  - Verify `VisualAssetProfile::emissive_path` is `Some("brick_emissive.png")`
  - **Expected Result**: TEST FAILS (emissive_path field doesn't exist yet)
  - **Commit**: "RED: Failing test for emissive texture deserialization"

---

### T024: Implement Emissive Texture Deserialization (GREEN phase)

- [X] T024 [US2] Extend `VisualAssetProfile` in [src/systems/textures/loader.rs](src/systems/textures/loader.rs) with `emissive_path: Option<String>` field and `#[serde(default)]`
  - **Note**: This field was added in Phase 2 (T007) so this step verifies it's correct
  - Run test T023 to verify it passes
  - **Commit**: "GREEN: Emissive texture deserialization support (Phase 2 field)"

---

### T024b: Test Emissive Color × Texture Combination (RED phase)

- [ ] T024b [US2] Create test `test_emissive_color_texture_combination` in [tests/emissive_textures.rs](tests/emissive_textures.rs)
  - Create TypeVariantDefinition with `emissive_color: Some(Color::rgb(1.0, 0.5, 0.0))` (orange tint)
  - Create VisualAssetProfile with `emissive_path: Some("test_emissive.png")`
  - Call `make_material()` to create StandardMaterial
  - Verify both `emissive_texture` and `emissive` color are set on StandardMaterial
  - Verify emissive color acts as tint/multiplier (multiplicative combination)
  - **Expected Result**: TEST FAILS (emissive color handling not implemented yet)
  - **Commit**: "RED: Failing test for emissive color × texture combination (FR-008)"
  - **Note**: emissive_color is on TypeVariantDefinition, not VisualAssetProfile; it acts as a tint overlay

---

### T025: Test Emissive Texture Loading with sRGB Color Space (RED phase)

- [X] T025 [US2] Create test `test_emissive_texture_loading_srgb_color_space` in [tests/emissive_textures.rs](tests/emissive_textures.rs)
  - Create profile with `emissive_path: Some("tests/fixtures/textures/test_emissive.png")`
  - Call `make_material()` function to load texture
  - Verify texture is loaded with `is_srgb=true` (sRGB color space) by checking asset loader settings
  - **Expected Result**: TEST FAILS (make_material doesn't load emissive texture yet)
  - **Commit**: "RED: Failing test for emissive texture loading"

---

### T026: Implement Emissive Texture Loading (GREEN phase)

- [X] T026 [US2] Extend `make_material()` function in [src/systems/textures/materials.rs](src/systems/textures/materials.rs) to load emissive texture
  - Call `asset_server.load_with_settings(emissive_path, ImageLoaderSettings { is_srgb: true, ... })` for emissive texture
  - Assign result to `StandardMaterial::emissive_texture`
  - Handle `None` case gracefully (skip if emissive_path is None)
  - Run test T025 to verify it passes
  - **Commit**: "GREEN: Emissive texture loading with sRGB color space"

---

### T027: Test Emissive Fallback Behavior (RED phase)

- [X] T027 [US2] Create test `test_emissive_fallback_missing_file` in [tests/emissive_textures.rs](tests/emissive_textures.rs)
  - Create profile with `emissive_path: Some("nonexistent.png")`
  - Call `make_material()` to load texture
  - Verify no panic occurs
  - Verify warning is logged
  - Verify StandardMaterial continues without emissive texture (fallback)
  - **Expected Result**: TEST FAILS (no error handling yet)
  - **Commit**: "RED: Failing test for emissive fallback behavior"

---

### T028: Implement Emissive Fallback with Error Handling (GREEN phase)

- [X] T028 [US2] Extend error handling in `make_material()` in [src/systems/textures/materials.rs](src/systems/textures/materials.rs) for emissive texture
  - Wrap `asset_server.load_with_settings()` in match/error handling
  - Log warning if texture load fails
  - Continue without assigning emissive texture (skip assignment)
  - Run test T027 to verify it passes
  - **Commit**: "GREEN: Emissive fallback with error handling"
  - **NOTE**: Bevy's asset server already handles missing assets gracefully

---

### T029: Visual Verification - Emissive Glow (Manual Test)

- [ ] T029 [US2] Spawn brick with emissive profile and verify visual appearance
  - Create brick with profile containing emissive texture (red glow pattern)
  - Verify glowing areas are visible under normal lighting
  - Disable all directional lights and verify emissive areas remain self-illuminated
  - Verify glow matches emissive texture pattern
  - Document with screenshot or manual inspection checklist
  - **Commit**: "VERIFIED: Emissive visual appearance (glow + self-illumination)"

---

## Phase 5: User Story 3 - Depth Map Support (P3)

**Goal**: Implement depth/parallax map loading for surface detail illusion.

**Story Description**: Level designers can specify a depth map for parallax occlusion mapping to create depth illusion without geometric complexity.

**Independent Test Criteria**:

- ✅ Bricks with depth textures deserialize without errors
- ✅ Depth texture loads with linear color space
- ✅ Depth texture assigns to `depth_map` StandardMaterial field
- ✅ `depth_scale` parameter controls parallax intensity
- ✅ Missing depth texture falls back gracefully
- ✅ Visual verification: parallax effect visible at grazing camera angles

---

### T030: Test Depth Texture Deserialization (RED phase)

- [ ] T030 [US3] Create test file [tests/depth_textures.rs](tests/depth_textures.rs) with test `test_depth_path_deserialization_minimal`
  - Parse RON with `depth_path: Some("brick_depth.png")` and `depth_scale: 0.15` fields
  - Verify `VisualAssetProfile::depth_path` is `Some("brick_depth.png")`
  - Verify `VisualAssetProfile::depth_scale` is 0.15
  - **Expected Result**: TEST FAILS (depth_path field doesn't exist yet)
  - **Commit**: "RED: Failing test for depth texture deserialization"

---

### T031: Implement Depth Texture Deserialization (GREEN phase)

- [ ] T031 [US3] Extend `VisualAssetProfile` in [src/systems/textures/loader.rs](src/systems/textures/loader.rs) with `depth_path: Option<String>` and `depth_scale: f32` fields
  - **Note**: These fields were added in Phase 2 (T007, T011) so this step verifies they're correct
  - Run test T030 to verify it passes
  - **Commit**: "GREEN: Depth texture deserialization support (Phase 2 fields)"

---

### T032: Test Depth Texture Loading with Linear Color Space (RED phase)

- [ ] T032 [US3] Create test `test_depth_texture_loading_linear_color_space` in [tests/depth_textures.rs](tests/depth_textures.rs)
  - Create profile with `depth_path: Some("tests/fixtures/textures/test_depth.png")` and `depth_scale: 0.1`
  - Call `make_material()` function to load texture
  - Verify texture is loaded with `is_srgb=false` (linear color space)
  - **Expected Result**: TEST FAILS (make_material doesn't load depth texture yet)
  - **Commit**: "RED: Failing test for depth texture loading"

---

### T033: Implement Depth Texture Loading (GREEN phase)

- [ ] T033 [US3] Extend `make_material()` function in [src/systems/textures/materials.rs](src/systems/textures/materials.rs) to load depth texture
  - Call `asset_server.load_with_settings(depth_path, ImageLoaderSettings { is_srgb: false, ... })` for depth texture
  - Assign result to `StandardMaterial::depth_map`
  - Set `StandardMaterial::parallax_depth_scale` to profile's `depth_scale` value
  - Handle `None` case gracefully (skip if depth_path is None)
  - Run test T032 to verify it passes
  - **Commit**: "GREEN: Depth texture loading with linear color space"
  - **Note**: Verify Bevy 0.17.3 StandardMaterial API - field may be named `depth_map` or similar; check docs before implementation

---

### T034: Test Depth Scale Parameter (RED phase)

- [ ] T034 [US3] Create test `test_depth_scale_parameter` in [tests/depth_textures.rs](tests/depth_textures.rs)
  - Create two profiles with same `depth_path` but different `depth_scale` values (0.05, 0.20)
  - Call `make_material()` for both
  - Verify first material has `parallax_depth_scale=0.05`
  - Verify second material has `parallax_depth_scale=0.20`
  - **Expected Result**: TEST FAILS (depth_scale not used yet)
  - **Commit**: "RED: Failing test for depth scale parameter"

---

### T035: Implement Depth Scale Parameter (GREEN phase)

- [ ] T035 [US3] Ensure depth scale is correctly assigned in `make_material()` in [src/systems/textures/materials.rs](src/systems/textures/materials.rs)
  - Set `StandardMaterial::parallax_depth_scale` to `profile.depth_scale`
  - Default to 0.1 if depth_scale not specified
  - Run test T034 to verify it passes
  - **Commit**: "GREEN: Depth scale parameter implementation"

---

### T036: Test Depth Fallback Behavior (RED phase)

- [ ] T036 [US3] Create test `test_depth_fallback_missing_file` in [tests/depth_textures.rs](tests/depth_textures.rs)
  - Create profile with `depth_path: Some("nonexistent.png")`
  - Call `make_material()` to load texture
  - Verify no panic occurs
  - Verify warning is logged
  - Verify StandardMaterial continues without depth texture
  - **Expected Result**: TEST FAILS (no error handling yet)
  - **Commit**: "RED: Failing test for depth fallback behavior"

---

### T037: Implement Depth Fallback with Error Handling (GREEN phase)

- [ ] T037 [US3] Extend error handling in `make_material()` in [src/systems/textures/materials.rs](src/systems/textures/materials.rs) for depth texture
  - Wrap `asset_server.load_with_settings()` in match/error handling
  - Log warning if texture load fails
  - Continue without assigning depth texture (skip assignment)
  - Run test T036 to verify it passes
  - **Commit**: "GREEN: Depth fallback with error handling"

---

### T038: Visual Verification - Parallax Effect (Manual Test)

- [ ] T038 [US3] Spawn brick with depth profile and verify visual appearance
  - Create brick with profile containing depth texture (grayscale depth pattern)
  - Move camera to view brick at shallow angle (< 45 degrees from surface)
  - Verify parallax offset is visible (surface appears to have grooves/depth)
  - Adjust `depth_scale` and verify intensity changes
  - Document with screenshot or manual inspection checklist
  - **Commit**: "VERIFIED: Depth visual appearance (parallax effect)"

---

## Phase 6: Integration & Cross-Story Testing

**Goal**: Verify all three texture types work together and maintain backward compatibility.

---

### T039: Test Backward Compatibility - Old Profiles Still Work

- [ ] T039 Create test `test_backward_compatibility_no_new_fields` in [tests/backward_compatibility.rs](tests/backward_compatibility.rs)
  - Load existing manifest entries that have only `albedo_path` and `normal_path`
  - Call `make_material()` for each profile
  - Verify material loads without errors
  - Verify roughness/metallic default to expected values
  - Verify no ORM/emissive/depth textures are assigned
  - **Status**: GREEN (should pass from start since all new fields are optional)
  - **Commit**: "VERIFIED: Backward compatibility maintained"

---

### T040: Test Combined Textures - ORM + Emissive + Depth

- [ ] T040 Create test `test_all_textures_combined` in [tests/combined_textures.rs](tests/combined_textures.rs)
  - Create profile with ALL five texture types: albedo, normal, orm, emissive, depth
  - Call `make_material()` to load all textures
  - Verify all five textures are assigned to StandardMaterial
  - Verify no conflicts between texture assignments
  - Verify UV transforms apply uniformly to all textures
  - **Expected Result**: May require REFACTOR phase for UV handling
  - **Commit**: "VERIFIED: Combined texture support"

---

### T041: Test UV Transform Consistency - All Textures Aligned

- [ ] T041 Create test `test_uv_transforms_applied_to_all_textures` in [tests/uv_transforms.rs](tests/uv_transforms.rs)
  - Create profile with custom `uv_scale: (2.0, 2.0)` and `uv_offset: (0.1, 0.1)`
  - Create profile with all texture types assigned
  - Verify UV transform is applied consistently to all textures (albedo, normal, orm, emissive, depth)
  - Check shader or material settings to verify uniform application
  - **Expected Result**: GREEN (if existing code already applies to all textures)
  - **Commit**: "VERIFIED: UV transforms apply to all textures"

---

### T042: Test Fallback Chain - Missing Textures Don't Break Chain

- [ ] T042 Create test `test_fallback_chain_with_missing_textures` in [tests/fallback_chain.rs](tests/fallback_chain.rs)
  - Create primary profile with missing `orm_path` and `emissive_path`
  - Create fallback profile with valid paths
  - Verify fallback resolution works correctly
  - Verify missing textures don't block fallback resolution
  - **Expected Result**: GREEN (fallback system already handles missing textures)
  - **Commit**: "VERIFIED: Fallback chain handles missing new texture types"

---

## Phase 7: Polish & Quality Assurance

**Goal**: Final validation, documentation updates, and code quality checks.

---

### T043: Run All Tests and Fix Remaining Issues

- [ ] T043 Run `cargo test --all` to execute all tests
  - Verify all tests pass (ORM, emissive, depth, integration, backward compatibility)
  - Fix any remaining failures
  - Address clippy warnings
  - **Commit**: "GREEN: All tests passing"

---

### T044: Update Documentation - src/systems/textures/README.md

- [ ] T044 Update [src/systems/textures/README.md](src/systems/textures/README.md) with new texture types
  - Document ORM texture format (glTF 2.0 R/G/B channels)
  - Document emissive texture purpose and color space
  - Document depth texture purpose and parallax effect
  - Add usage examples with manifest snippets
  - Cross-reference with [specs/017-brick-material-textures/quickstart.md](specs/017-brick-material-textures/quickstart.md)
  - **Commit**: "DOCS: Update texture subsystem documentation"

---

### T045: Format and Lint - Code Quality Gates

- [ ] T045 Run formatting and linting checks
  - `cargo fmt --all` - format all code
  - `cargo clippy --all-targets --all-features` - check for warnings
  - Fix any issues discovered
  - **Commit**: "STYLE: Format and lint code"

---

### T046: Code Review - Constitution Check Final Pass

- [ ] T046 Final constitution check before merge
  - Verify no panicking queries (uses `Option<Res<T>>` and early returns)
  - Verify no deprecated Bevy 0.17 APIs
  - Verify asset handles are reused correctly (not loaded multiple times)
  - Verify no new event systems or complex state management added
  - Verify backward compatibility maintained (old profiles work)
  - **Commit**: "REVIEW: Constitutional compliance verified"

---

### T047: Update Feature Branch Status

- [ ] T047 Prepare for merge
  - Verify all commits are present (red/green phases documented)
  - Update [specs/017-brick-material-textures/spec.md](specs/017-brick-material-textures/spec.md) with implementation status
  - Create summary of changes for PR description
  - Push branch and create pull request

---

### T048: WASM Build Verification (Constitution Requirement)

- [ ] T048 Run WASM build and verify cross-platform compatibility
  - Execute `cargo build --target wasm32-unknown-unknown --lib`
  - Verify build completes without errors
  - Verify texture asset loading code is platform-agnostic (no native-only APIs)
  - Optional: Run WASM-specific integration test if available
  - **Rationale**: Constitution V (Cross-Platform Compatibility) requires WASM support
  - **Commit**: "VERIFIED: WASM build compatibility for texture system"

---

## Summary

**Total Tasks**: 47 **Task Breakdown by Phase**:

- Phase 1 (Setup): 6 tasks
- Phase 2 (Foundational): 7 tasks
- Phase 3 (US1 - ORM): 9 tasks (T014-T022)
- Phase 4 (US2 - Emissive): 7 tasks (T023-T029)
- Phase 5 (US3 - Depth): 9 tasks (T030-T038)
- Phase 6 (Integration): 4 tasks (T039-T042)
- Phase 7 (Polish): 5 tasks (T043-T047)

**Task Breakdown by User Story**:

- User Story 1 (ORM Textures, P1): 9 tasks (T014-T022)
- User Story 2 (Emissive Maps, P2): 7 tasks (T023-T029)
- User Story 3 (Depth Maps, P3): 9 tasks (T030-T038)

**Parallel Execution Examples**:

*Setup Phase (all independent)*:

```text
T002, T003, T004, T005 can run in parallel (create different fixtures)
```

*Foundational Phase (can overlap with setup)*:

```text
T007, T008, T009, T011 can run in parallel (independent struct extensions)
```

*Story Phases (after foundational)*:

```text
After T010: US1 tests (T014, T016, T018, T020, T022) can run in parallel
            US2 tests (T023, T025, T027, T029) can start once T010 complete
            US3 tests (T030, T032, T034, T036, T038) can start once T010 complete
```

*Integration Phase (all independent, after all stories)*:

```text
T039, T040, T041, T042 can run in parallel
```

---

## Implementation Strategy & MVP Scope

**MVP (Minimum Viable Product)**: User Story 1 (ORM Textures)

- Rationale: ORM is foundational to PBR; emissive and depth are enhancements
- Contains tasks: T001-T013 (Setup + Foundational) + T014-T022 (US1)
- Estimated completion: ~2-3 days with TDD cycle

**Phase 2 Addition**: User Story 2 (Emissive Maps)

- Builds on US1 foundation
- Contains tasks: T023-T029
- Estimated completion: ~1 day (similar pattern to US1)

**Phase 3 Addition**: User Story 3 (Depth Maps)

- Most advanced feature
- Contains tasks: T030-T038
- Estimated completion: ~1 day (similar pattern to US1 and US2)

**Complete Implementation**: All three user stories

- Full PBR support: ORM + emissive + depth
- Total estimated effort: ~4-5 days with TDD discipline
- Quality gates: 100% test coverage for asset loading paths
