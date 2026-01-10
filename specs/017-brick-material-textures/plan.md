# Implementation Plan: Enhanced Brick Material Textures

**Branch**: `017-brick-material-textures` | **Date**: 2026-01-04 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/017-brick-material-textures/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Extend the existing texture manifest system to support packed ORM (Occlusion-Roughness-Metallic) textures, emissive maps, and depth/parallax maps for brick materials, following glTF 2.0 PBR standards.
The implementation adds optional `orm_path`, `emissive_path`, and `depth_path` fields to `VisualAssetProfile`, loads these textures with correct color space settings (linear for data, sRGB for color), and assigns them to the appropriate StandardMaterial fields.
The technical approach uses Bevy's existing asset loading infrastructure and the established material bank pattern, ensuring backward compatibility with existing profiles that only specify albedo and normal maps.

## Technical Context

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3 (rendering, assets, ECS), serde 1.0, ron 0.8 **Storage**: N/A (texture assets loaded from `assets/textures/` directory; manifest in `assets/textures/manifest.ron`) **Testing**: cargo test with Bevy's test infrastructure **Target Platform**: Native (Linux/Windows/macOS) + WASM32 **Project Type**: Game engine feature (texture/rendering subsystem extension) **Performance Goals**: No performance regression from additional texture loads; maintain 60 FPS with multiple textured bricks **Constraints**: Backward compatibility with existing manifest files; texture loading must work identically on WASM and native **Scale/Scope**: Extends 3 existing data structures (`VisualAssetProfile`, `TypeVariantDefinition` contract, `StandardMaterial` usage); adds ~150 lines of code; affects texture loading pipeline only

## Constitution Check

*GATE: Must pass before Phase 0 research.*
       *Re-check after Phase 1 design.*

### TDD Compliance

- ✅ **Tests First**: Feature spec includes acceptance scenarios for each user story; tests will be written before implementation
- ✅ **Red Phase**: Test commit showing failures required before implementation begins
- ✅ **Approval Gate**: Tests must be validated before proceeding with implementation code
- ✅ **Coverage**: Tests will cover VisualAssetProfile deserialization, ORM texture loading, material field assignment, fallback behavior, and color space correctness

### Bevy 0.17 Compliance

**Event System Usage**:

- ✅ **Not Applicable**: This feature does NOT use events, messages, or observers.
  It extends the existing asset loading system which operates on resource changes during the `Update` schedule.
- The existing `hydrate_texture_materials` system already handles manifest changes reactively via `manifest.is_changed()` detection.

**ECS Mandates**:

- ✅ **Fallible Systems**: Existing systems use `Option<Res<T>>` patterns with early returns; new code will follow the same pattern
- ✅ **Asset Handle Reuse**: Textures are loaded once in `ProfileMaterialBank::rebuild()` and handles are cloned for entities
- ✅ **No Query Issues**: This feature does not add new queries; it extends existing material resource management

**Rendering Mandates**:

- ✅ **StandardMaterial Correct Usage**: Code assigns to established StandardMaterial fields (`metallic_roughness_texture`, `occlusion_texture`, `emissive_texture`, `depth_map`)
- ✅ **Asset Loading**: Uses existing `asset_server.load()` and `asset_server.load_with_settings()` patterns for color space control
- ✅ **No Deprecated APIs**: Uses current Bevy 0.17 APIs only

**Performance & Architecture**:

- ✅ **No Archetype Thrashing**: Feature does not insert/remove components; only modifies material resources
- ✅ **Change Detection**: Existing systems already use `is_changed()` for manifest updates
- ✅ **Plugin Architecture**: Changes contained within `TextureMaterialsPlugin` and related texture subsystem modules

### Gates Summary

✅ **PASS** - No constitution violations.
Feature extends existing asset loading infrastructure following established patterns.
No new ECS systems, queries, or event handling required.

## Project Structure

### Documentation (this feature)

```text
specs/017-brick-material-textures/
├── spec.md              # Feature specification (completed)
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (next step)
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── visual-asset-profile-contract.md
└── checklists/          # Quality validation
    └── requirements.md
```

### Source Code (repository root)

```text
src/systems/textures/
├── mod.rs                    # Module exports
├── loader.rs                 # VisualAssetProfile struct - ADD 3 optional fields
├── contracts.rs              # VisualAssetProfileContract - ADD 3 optional fields
├── materials.rs              # make_material() function - ADD ORM/emissive/depth texture loading
└── README.md                 # Documentation (may need updates)

assets/textures/
├── manifest.ron              # Example profiles (update for testing)
└── [texture files]           # Example ORM/emissive/depth textures (add for testing)

tests/
├── texture_manifest.rs       # NEW - ORM texture loading tests
├── texture_materials.rs      # NEW - Material field assignment tests
└── texture_fallback.rs       # NEW - Missing texture fallback tests
```

**Structure Decision**: This feature extends the existing texture subsystem without requiring new modules.
The changes are localized to three files in `src/systems/textures/` (loader.rs, contracts.rs, materials.rs).
All modifications preserve backward compatibility by making new fields optional.
Test files will be added to the `tests/` directory following the existing pattern of feature-specific integration tests.

## Complexity Tracking

**Constitution Violations**: 0 (no event system, observer, or query issues) **Bevy 0.17 Compliance**: PASS (uses StandardMaterial fields correctly, no deprecated APIs) **Files Modified**: 3 (loader.rs, contracts.rs, materials.rs) **Lines Added**: ~150 (50 per file on average for struct fields + loading logic) **Tests Required**: 9 scenarios (3 per user story: P1 ORM, P2 emissive, P3 depth)

**No violations identified.**
This feature follows all constitutional principles:

- Extends existing asset loading infrastructure using established patterns
- Maintains backward compatibility through optional fields
- Uses Bevy's built-in StandardMaterial fields (no custom shaders)
- Follows TDD with tests written before implementation
- No new ECS systems, events, or complex state management required

---

## Phase 0: Research & Clarification

**Goal**: Resolve all NEEDS CLARIFICATION items from Technical Context.

**Status**: ✅ COMPLETED

**Output**: [research.md](./research.md)

**Findings**:

1. **ORM Texture Format**: Use packed glTF 2.0 format (R=AO, G=Roughness, B=Metallic)
2. **Field Assignment**: Assign same ORM texture to both `metallic_roughness_texture` and `occlusion_texture`
3. **Color Space**: Linear for ORM/depth/normal, sRGB for albedo/emissive
4. **UV Transforms**: Unified transform from manifest applied to all textures
5. **Backward Compatibility**: Optional fields with `#[serde(default)]` ensure old manifests work

---

## Phase 1: Design & Contracts

**Goal**: Generate data model, API contracts, and quickstart guide.

**Status**: ✅ COMPLETED

**Outputs**:

- ✅ [data-model.md](./data-model.md) - Extended VisualAssetProfile structure with 3 new optional fields
- ✅ [contracts/visual-asset-profile.md](./contracts/visual-asset-profile.md) - API contract with validation rules and RON examples
- ✅ [quickstart.md](./quickstart.md) - Designer guide for creating and using advanced PBR textures
- ✅ Agent context updated via `.specify/scripts/bash/update-agent-context.sh copilot`

**Constitution Re-check**: ✅ PASS (no changes to previous assessment; design extends asset loading only)

---

## Phase 2: Task Decomposition

**Goal**: Break down implementation into testable tasks.

**Status**: ✅ COMPLETED

**Output**: [tasks.md](./tasks.md) with comprehensive TDD workflow

**Generated Artifacts**:

- 47 total tasks organized by phase
- 9 tasks per user story (red/green test phases + implementation + visual verification)
- Dependency graph showing story completion order (US1 → US2 → US3)
- Parallel execution examples for optimal team workflow
- MVP strategy with phased delivery (ORM first, then emissive, then depth)

**Task Organization**:

- Phase 1: Setup & fixtures (6 tasks)
- Phase 2: Foundational infrastructure (7 tasks) - blocks all user stories
- Phase 3: User Story 1 - ORM textures (9 tasks, P1)
- Phase 4: User Story 2 - Emissive maps (7 tasks, P2)
- Phase 5: User Story 3 - Depth maps (9 tasks, P3)
- Phase 6: Integration & cross-story testing (4 tasks)
- Phase 7: Polish & quality assurance (5 tasks)

**TDD Workflow**: Each user story follows the red/green/refactor cycle:

1. Write failing test (RED phase, document commit hash)
2. Implement minimum code to pass test (GREEN phase)
3. Refactor if needed (REFACTOR phase)
4. Visual verification for rendering features (VERIFIED phase)

---

## Next Steps

**Immediate Action**: Begin task execution from Phase 1 **Prerequisites**: All planning artifacts complete (✅ done) **Recommended Sequence**:

1. Execute Phase 1 (Setup) - initializes fixtures and verifies environment
2. Execute Phase 2 (Foundational) - extends data structures
3. Execute Phase 3 (US1) - core ORM texture implementation
4. Execute Phases 4-5 (US2-US3) - additional texture types (can parallelize)
5. Execute Phase 6 (Integration) - cross-story validation
6. Execute Phase 7 (Polish) - final quality gates

**Estimated Total Duration**: 4-5 days with disciplined TDD approach

---

## Report

**Branch**: `017-brick-material-textures` **Implementation Plan**: `specs/017-brick-material-textures/plan.md`

**Complete Artifact Set**:

- ✅ `spec.md` - Feature specification with 3 user stories, 16 functional requirements, 7 success criteria
- ✅ `research.md` - Phase 0: 5 technical decisions resolved (ORM format, field assignment, color spaces, UV transforms, backward compatibility)
- ✅ `data-model.md` - Phase 1: Extended VisualAssetProfile structure with 3 optional fields (orm_path, emissive_path, depth_path)
- ✅ `contracts/visual-asset-profile.md` - Phase 1: API contract v2.0.0 with JSON schema, validation rules, RON examples, backward compatibility guide
- ✅ `quickstart.md` - Phase 1: Designer guide with texture creation workflow, 4 common patterns, ORM creation options, troubleshooting
- ✅ `tasks.md` - Phase 2: Task decomposition with 47 tasks, TDD workflow, dependency graph, parallel opportunities, MVP strategy
- ✅ Agent context updated (`.github/agents/copilot-instructions.md` reflects Rust 1.81, Bevy 0.17.3, serde, ron)

**Constitution Check**: ✅ PASS (verified at planning stage; no changes in decomposition)

**Ready for**: Implementation phase (execute tasks.md starting with Phase 1)
