# Implementation Plan: Refactor Systems for Constitution Compliance

**Branch**: `011-refactor-systems` | **Date**: 2025-12-19 | **Spec**: specs/011-refactor-systems/spec.md
**Input**: Feature specification from `specs/011-refactor-systems/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Refactor `src/systems` to conform to Constitution 1.3.0 Bevy 0.17 mandates & prohibitions.
Primary goals:

- Fallible systems using `anyhow::Result<()>` with `?` and early returns
- Clear Message vs Event boundaries: shared `crate::signals` with Messages (`UiBeep`, `BrickDestroyed`); engine events via observers
- Replace tuple `.chain()` with System Sets and set ordering
- Strict change-driven updates (`Changed<T>`, `RemovedComponents<T>`, `OnAdd`) — no periodic fallbacks
- Enforce `#[require(Transform, Visibility)]` on marker components and remove redundant bundles
- Improve query specificity and parallelism, and ensure asset handle reuse

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, serde/ron, tracing **Storage**: N/A (in-memory ECS state only) **Testing**: cargo test (unit + integration), WASM build checks when applicable **Target Platform**: Native (Linux/macOS/Windows) + WASM **Project Type**: Single project (game) **Performance Goals**: 60 FPS target; minimize per-frame work; maintain parallelism **Constraints**: Avoid panics; adhere to Bevy 0.17 scheduling and APIs; asset handle reuse **Scale/Scope**: Refactor confined to `src/systems/**` plus shared `crate::signals` and marker attributes

## Constitution Check

*GATE: Must pass before Phase 0 research.*
      *Re-check after Phase 1 design.*

This check MUST verify compliance with the constitution, including **Test-Driven Development (TDD)** gates:

- Tests are defined and committed prior to implementation efforts for each story/feature.
- A proof-of-failure commit (tests that FAIL) MUST exist in the branch history prior to implementation commits.
- Tests MUST be reviewed and approved by the feature owner or requestor before implementation begins.

This check MUST also verify compliance with **Bevy 0.17 mandates & prohibitions** (if the feature touches ECS, rendering, assets, or scheduling):

- Systems are fallible (`Result`) and do not panic on query outcomes (`?`, no `.unwrap()` on queries).
- Queries use `With<T>`/`Without<T>` filters and `Changed<T>` where appropriate (especially UI).
- Message vs Event usage follows Bevy 0.17 APIs (`MessageWriter/Reader` vs observers).
- Assets are loaded once and handles are stored in Resources (no repeated `asset_server.load()` in loops).
- Hierarchies use `ChildOf::parent()` and `add_children()`/`remove::<Children>()` patterns.

[Gates determined based on constitution file]

Gate status pre-design:

- TDD: New compliance tests will be authored first per FRs; failing commit required before implementation.
- Bevy 0.17: Violations identified (non-fallible systems, .chain() on tuples, mixed Message/Event usage, missing change filters, manual bundles).
  These will be addressed in the tasks below.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
src/
├── systems/
│   ├── audio.rs
│   ├── cheat_mode.rs
│   ├── grid_debug.rs
│   ├── level_switch.rs
│   ├── multi_hit.rs
│   ├── paddle_size.rs
│   ├── respawn.rs
│   ├── scoring.rs
│   └── textures/
├── level_loader.rs
├── lib.rs
└── main.rs

tests/
└── [existing integration/unit tests; add new compliance tests]
```

**Structure Decision**: Single project.
Add `src/signals.rs` (or `mod signals;`) to host shared Messages; update `src/lib.rs` to expose.

## Phases

### Phase 0: Research (Complete)

- Decision: `anyhow::Result<()>` across systems; Rationale: ergonomic `?`, minimal boilerplate.
- Decision: Centralize `BrickDestroyed` and `UiBeep` as Messages in `crate::signals`; Rationale: single source of truth, no dual-derive.
- Decision: Engine events (e.g., `AssetEvent<Image>`) via observers; Rationale: matches engine semantics and Constitution.
- Decision: Strict change-driven updates; Rationale: avoid per-frame work, align with mandates.
- Decision: Apply `#[require(Transform, Visibility)]` to `Paddle`, `Ball`, `GridOverlay`, `Border`, `GroundPlane` now; Rationale: remove redundant bundles.

### Phase 1: Design & Contracts

- Data Model: See data-model.md for Messages, System Sets, and Required Components inventory.
- Contracts: Internal messaging contracts for `UiBeep` and `BrickDestroyed` documented; no external API.
- Quickstart: Developer steps for tests-first workflow and branch usage.

### Phase 2: Plan tasks (implementation sequencing)

1) Tests-first (Compliance)
   - Add tests to assert: fallible systems, single-path messaging, no tuple `.chain()` (observable order), change-driven visuals, asset handle reuse, required components present on marker-only spawns.
2) Signals module
   - Introduce `src/signals.rs` with `#[derive(Message)]` types; replace duplicates in `audio` and `scoring` and update imports.
3) Fallible systems conversion
   - Update all systems to `anyhow::Result<()>`; replace `unwrap()` and nested matches with `?` and early-return patterns.
4) Event vs Message alignment
   - Convert `UiBeep` consumption to Message readers; convert `AssetEvent<Image>` handling to observers; remove `MessageReader<AssetEvent<Image>>` and guards.
5) System Set refactor
   - Create `AudioSystems`, `PaddleSizeSystems`, `TextureOverrideSystems`; remove tuple `.chain()`; express ordering via `.configure_sets()` and `.after()`.
6) Change detection gates
   - Add `Changed<T>` / `RemovedComponents<T>` / `OnAdd` to paddle visuals and textures application; gate grid visibility on `Changed<WireframeConfig>`.
7) Required components
   - Add `#[require(Transform, Visibility)]` to listed markers; adjust spawns to rely on requirements and remove redundant bundles.
8) Asset handle reuse audit
   - Verify and adjust any repeated `asset_server.load()` usage in loops (esp. spawns) to use resource-held handles.
9) Docs & cleanup
   - Update module docs where public APIs changed; ensure no dual signal types remain; ensure tests pass native and WASM.

## Constitution Check (Post-Design)

All planned changes satisfy the Bevy 0.17 mandates and TDD gates.
Implementation MUST preserve tests-first history (failing commit before green).
No exceptions or violations are required.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| — | — | — |
