# Implementation Plan: Gravity Indicator UI

**Branch**: `021-gravity-bricks` | **Date**: 2026-01-11 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/021-gravity-bricks/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Display a UI indicator in the lower-left corner showing the current gravity level.
Map gravity Vec3 values (X/Z axes only, Y ignored) to discrete levels (0, 2, 10, 20) using ±0.5 tolerance.
Use weight icon assets and update instantly when gravity changes.
Spawn when GravityConfiguration and textures are both loaded.

## Technical Context

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0 (physics), tracing 0.1 **Storage**: In-memory ECS state only (no persistent storage) **Testing**: cargo test (unit + integration tests) **Target Platform**: Native (Linux/Windows/macOS) + WASM **Project Type**: Single project (game) **Performance Goals**: Maintain 60 FPS; indicator updates within one frame of gravity change **Constraints**: WASM-compatible; asset loading must be deferred until textures ready **Scale/Scope**: Single UI element; 5 texture assets; minimal computational overhead

## Constitution Check

*GATE: Must pass before Phase 0 research.*
Re-check after Phase 1 design.

**Status**: ✅ PASSED (pre-design evaluation)

### TDD Gates

- ✅ Tests defined in spec acceptance scenarios (User Story 1-3)
- ⚠️ Tests must be committed and FAIL before implementation (pending verification)
- ⚠️ Tests must be reviewed by feature owner before implementation begins

**Re-evaluation (post Phase 1 design)**: ✅ PASSED

- Integration tests documented in [data-model.md](data-model.md) testing strategy section
- Multi-frame persistence test explicitly defined
- Spawn timing, idempotence, update correctness all covered

### Bevy 0.17 Compliance

**Event System Choice**: Not using Messages or Observers for this feature.
Indicator updates via resource change detection (`Changed<GravityConfiguration>`), which is the appropriate pattern for reactive UI that responds to resource state changes.

**Coordinate System**: Not applicable - feature is UI-only with no spatial movement or physics interaction.

**Initialization System Idempotence**: `spawn_gravity_indicator` system is idempotent:

- Guards with `if !existing.is_empty() { return; }` to prevent duplicate spawns
- Defers spawn until both `GravityConfiguration` and `GravityIndicatorTextures` are available
- Runs in Update schedule but only spawns once per indicator instance

**Multi-Frame Persistence Testing**: Required for gravity changes:

- Tests must verify indicator remains correct across ≥10 frames after gravity change
- Tests must include all systems that write to GravityConfiguration to catch overwrites
- Validates that `update_gravity_indicator` only reacts to `Changed<GravityConfiguration>`

**System Fallibility**: ✅

- Systems use `Option<Res<T>>` with early `return` for missing resources
- No `.unwrap()` on query results

**Query Specificity**: ✅

- `spawn_gravity_indicator`: `Query<Entity, With<GravityIndicator>>`
- `update_gravity_indicator`: `Query<&mut ImageNode, With<GravityIndicator>>`

**Change Detection**: ✅

- `update_gravity_indicator` uses `gravity_cfg.is_changed()` to avoid per-frame updates
- Only updates UI when GravityConfiguration changes

**Message-Event Separation**: ✅ N/A - feature uses resource change detection, not messages/observers

**Asset Handle Reuse**: ✅

- Assets loaded once in `setup_ui_assets` (Startup) and stored in `GravityIndicatorTextures` resource
- Handles cloned from resource in spawn/update systems

**Hierarchy Safety**: ✅ N/A - no parent/child relationships in this feature

**No Panicking Queries**: ✅ - Uses `Option<Res<T>>` and early returns **No Broad EntityRef Queries**: ✅ - Specific filters used **No Manual Component Bundles**: ✅ - Uses `ImageNode` and `Node` directly **No Archetype Thrashing**: ✅ - No component insertion/removal in update loop **No Resource-Based Entity Data**: ✅ - UI entity uses components **No Universal Query Updates**: ✅ - Updates gated by `is_changed()` **No Static Mutable State**: ✅ - All state in ECS **No Repeated Asset Loading**: ✅ - Assets loaded once at startup **No Unconditional State Overwrites**: ✅ - Spawn system is idempotent with guards

### Gates Summary

- All Bevy 0.17 mandates satisfied
- No constitution violations requiring justification
- TDD workflow pending verification (tests-first commits)

## Project Structure

### Documentation (this feature)

```text
specs/021-gravity-bricks/
├── spec.md              # Feature specification (user requirements)
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (design decisions)
├── data-model.md        # Phase 1 output (component/resource design)
├── quickstart.md        # Phase 1 output (developer guide)
├── contracts/           # Phase 1 output (API contracts - N/A for UI)
├── checklists/
│   └── requirements.md  # Quality checklist (clarifications resolved)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── ui/
│   ├── mod.rs                  # UI plugin registration
│   ├── gravity_indicator.rs   # Gravity indicator UI module (new)
│   ├── cheat_indicator.rs      # Reference for placement pattern
│   └── ...
├── systems/
│   ├── gravity.rs              # Gravity physics system (existing - read only)
│   └── ...
├── lib.rs                      # GravityConfiguration resource definition
└── ...

assets/
└── textures/
    └── default/
        ├── weight-question.png
        ├── weight-0.png
        ├── weight-2.png
        ├── weight-10.png
        └── weight-20.png

tests/
├── gravity_indicator_ui.rs    # Integration tests (new)

## Phase 1: Design Artifacts

**Status**: ✅ COMPLETED

### Generated Artifacts

1. **[data-model.md](data-model.md)** - ECS component/resource design
    - Documents `GravityIndicator` component (marker)
    - Documents `GravityIndicatorTextures` resource (texture cache)
    - Documents `GravityLevel` enum (mapping logic)
    - Defines helper functions (`map_gravity_to_level`, `select_texture`)
    - UI entity structure with `ImageNode` + `Node`
    - System data flow diagram
    - Testing strategy (unit + integration)

2. **[quickstart.md](quickstart.md)** - Developer guide
    - Prerequisites and architecture overview
    - Quick verification steps
    - System flow diagram
    - Testing instructions (unit + integration + manual)
    - Debugging guide with common issues
    - Extension points (new levels, position, Y-axis)
    - Performance notes and troubleshooting checklist

3. **contracts/** - API contracts
    - **Status**: N/A (not applicable for UI feature)
    - **Rationale**: This is an internal UI component with no external API surface. No public contracts needed. Component/resource definitions in data-model.md serve as internal interface documentation.

### Constitution Re-Check

**Post-design evaluation**: ✅ ALL GATES PASSING

- **TDD**: Integration tests defined in data-model.md (spawn timing, idempotence, update correctness, multi-frame persistence, level transitions)
- **Bevy 0.17 Compliance**: Uses resource change detection (not Messages/Observers), idempotent spawn, multi-frame testing planned
- **No violations**: All mandates satisfied, no prohibited patterns used

### Agent Context Update

**Status**: ✅ COMPLETED

**Action taken**: Ran `.specify/scripts/bash/update-agent-context.sh copilot`

**Technologies added** to `.github/agents/copilot-instructions.md`:
- Rust 1.81 (edition 2021)
- Bevy 0.17.3
- bevy_rapier3d 0.32.0 (physics)
- tracing 0.1
- In-memory ECS state only (no persistent storage)

**Additional patterns documented** (implicit in design artifacts):
- bevy::ui::ImageNode (0.17 component for UI images)
- Change detection patterns (Changed<T> query filter)
- Idempotent spawn patterns (guard with existing entity query)

## Phase 2: Task Breakdown

**Status**: PENDING (requires `/speckit.tasks` command)

This phase will generate [tasks.md](tasks.md) with implementation task breakdown per design artifacts.

---

## Plan Command Completion Summary

**Status**: ✅ PLAN PHASE COMPLETE

### Artifacts Generated

1. **Phase 0: Research** ([research.md](research.md))
    - 6 design decisions documented
    - All NEEDS CLARIFICATION items resolved
    - Technology choices validated

2. **Phase 1: Design**
    - [data-model.md](data-model.md) - ECS architecture fully documented
    - [quickstart.md](quickstart.md) - Developer guide complete
    - contracts/ - N/A (internal UI feature)

3. **Plan Document Updates**
    - Technical Context filled
    - Constitution Check passed (pre and post-design)
    - Project Structure mapped
    - Complexity Tracking documented
    - Phase 1 completion section added

4. **Agent Context**
    - `.github/agents/copilot-instructions.md` updated
    - New technologies added to Active Technologies section

### Next Steps

To continue with implementation:

1. **Generate task breakdown**: Run `/speckit.tasks` command
    - Will create [tasks.md](tasks.md) with step-by-step implementation tasks
    - Will map tasks to design artifacts

2. **Write tests first**: Follow TDD mandate
    - Write integration tests from data-model.md testing strategy
    - Write unit tests for `map_gravity_to_level` function
    - Commit failing tests as proof (red commit)

3. **Implement feature**: Follow task breakdown
    - Create `src/ui/gravity_indicator.rs` module
    - Add textures to `assets/textures/`
    - Register systems in `src/ui/mod.rs`

4. **Verify**: Tests pass, linters happy
    - `cargo test`
    - `cargo clippy`
    - `cargo fmt --all`

### Files Created/Modified

**Created**:
- specs/021-gravity-bricks/research.md
- specs/021-gravity-bricks/data-model.md
- specs/021-gravity-bricks/quickstart.md

**Modified**:
- specs/021-gravity-bricks/plan.md (this file)
- .github/agents/copilot-instructions.md

### Branch Status

**Current branch**: `021-gravity-bricks`
**Spec file**: `specs/021-gravity-bricks/spec.md`
**Plan file**: `specs/021-gravity-bricks/plan.md`

---

**Plan generated**: 2026-01-11
**Command**: `/speckit.plan`
**Next command**: `/speckit.tasks`
└── ...
```

**Structure Decision**: Single project structure (Bevy game).
New UI module at `src/ui/gravity_indicator.rs` follows existing pattern from `src/ui/cheat_indicator.rs`.
Assets already exist in `assets/textures/default/`.
Tests added to `tests/` directory per convention.

## Complexity Tracking

No constitution violations.
All Bevy 0.17 mandates satisfied.
No exceptional complexity justification needed.
