---
description: "Task list for Textured Visuals Overhaul implementation"
---

# Tasks: Textured Visuals Overhaul

**Input**: Design documents from `/specs/001-textured-visuals/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: Added where explicitly called out by research + quickstart (manifest parsing, fallback registry, type variants, level switcher).

**Organization**: Tasks are grouped by user story priority so each slice remains independently testable.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Task can run in parallel (separate files, no blocking deps)
- **[Story]**: Label per user story (US1..US4). Setup/Foundational/Polish omit the label.
- Descriptions always include concrete file paths.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish texture asset scaffolding and module skeletons required by all stories.

- [x] T001 Create `assets/textures/` scaffolding with `manifest.ron` stub and `fallback/` placeholders per plan.md.
- [x] T002 Add `src/systems/textures/` module tree (`mod.rs`, `loader.rs`, `materials.rs`, `overrides.rs`) referenced in plan.md structure.
- [x] T003 Wire a feature-gated `TextureManifestPlugin` registration hook in `src/main.rs` (and expose flag in `Cargo.toml` if needed) to confirm compile coverage early.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core ECS resources and instrumentation required before any story work.

- [x] T004 Define Serde structs/enums for `VisualAssetProfile`, `TypeVariantDefinition`, `LevelTextureSet`, and `LevelSwitchState` in `src/systems/textures/loader.rs` + associated module exports.
- [x] T005 Implement the RON manifest asset loader + resource insertion in `src/systems/textures/loader.rs`, reading `assets/textures/manifest.ron` on startup and hot-reload.
- [x] T006 Build the `FallbackRegistry` resource + plugin in `src/systems/textures/materials.rs`, pre-baking `StandardMaterial` handles for each object class.
- [x] T007 Add structured tracing + error logging for manifest/fallback flows (missing files, invalid refs) across `src/systems/textures/loader.rs` and `src/systems/textures/materials.rs`.

**Checkpoint**: ECS resources load correctly; warnings/logging verified so user story work can begin.

---

## Phase 3: User Story 1 - Textured Baseline Objects (Priority: P1) 🎯 MVP

**Goal**: Ensure every major gameplay object renders with the canonical textured material on the first gameplay frame, with resilient fallbacks.

**Independent Test**: Start any level, delete one optional texture, and confirm all tracked objects render textured while a single fallback warning logs (per spec).

### Tests for User Story 1

- [x] T008 [P] [US1] Add manifest parsing regression tests validating required profiles + fallback chains in `tests/texture_manifest.rs`.
- [x] T009 [P] [US1] Add `FallbackRegistry` single-warning behavior tests in `tests/fallback_registry.rs`.

### Implementation for User Story 1

- [x] T010 [US1] Implement canonical material baking system for ball/paddle/bricks/sidewalls/background in `src/systems/textures/materials.rs` using manifest data.
- [x] T011 [US1] Apply baseline textures during level spawn/resets by extending `src/level_loader.rs` to request materials via the new resources.
- [x] T012 [US1] Wire fallback application + `log_once` helper inside `src/systems/textures/materials.rs` so missing assets swap immediately without spam.
- [x] T013 [US1] Keep `/visual-assets/manifest` contract aligned with runtime schema by documenting exported fields in `specs/001-textured-visuals/contracts/visual-assets.openapi.yaml` and adding any required adapters.

**Parallel Example (US1)**: Implementers can build T008 and T009 simultaneously, while T010/T011 proceed in parallel once manifest loader is stable (different files: `tests/*` vs `src/systems/textures/*`).

**Checkpoint**: Baseline textured experience complete; MVP ready for stakeholder review.

---

## Phase 4: User Story 2 - Type-Driven Materials (Priority: P2)

**Goal**: Ball and brick variants automatically display type-specific textures within 0.1 seconds of a gameplay state change.

**Independent Test**: Spawn multiple ball/brick types via debug tools and verify each reflects the matching texture instantly without level reloads.

### Tests for User Story 2

- [x] T014 [P] [US2] Add type-variant swap tests in `tests/type_variants.rs` covering both immediate spawns and runtime mutations.

### Implementation for User Story 2

- [ ] T015 [US2] Extend `src/systems/textures/loader.rs` to populate a `TypeVariantRegistry` resource from manifest `type_variants` entries.
- [ ] T016 [US2] Implement a ball-type watcher system in `src/systems/textures/materials.rs` that swaps materials within 0.1 seconds when the ball component changes.
- [ ] T017 [US2] Apply brick-type textures on spawn/update by integrating registry lookups into `src/level_loader.rs` (brick matrix parsing).
- [ ] T018 [US2] Surface type-variant metadata through the `/visual-assets/manifest` contract by updating `specs/001-textured-visuals/contracts/visual-assets.openapi.yaml` and any associated tooling/export commands.

**Parallel Example (US2)**: While T014 establishes regression tests, T015 and T016 can run concurrently (loader vs materials). T017 waits on registry definitions but can start before contract doc (T018).

**Checkpoint**: Gameplay types are visually distinguishable and independently testable.

---

## Phase 5: User Story 3 - Per-Level Presentation Pack (Priority: P3)

**Goal**: Allow designers to override ground plane, background, and sidewall textures per level, with hot-swappable assets for rapid iteration.

**Independent Test**: Configure three distinct levels with unique overrides, swap one texture file, reload only that level, and confirm the change is isolated.

### Tests for User Story 3

- [ ] T019 [P] [US3] Add level override coverage in `tests/level_overrides.rs` verifying defaults vs custom `LevelTextureSet` entries.

### Implementation for User Story 3

- [ ] T020 [US3] Extend `assets/levels/*.ron` schema + `src/level_loader.rs` parsing to ingest `LevelTextureSet` references defined in data-model.md.
- [ ] T021 [US3] Implement per-level override application pipeline in `src/systems/textures/overrides.rs`, layering ground/background/sidewall materials.
- [ ] T022 [US3] Add manifest hot-reload + asset swap detection in `src/systems/textures/overrides.rs` so artists can replace textures without rebuilding.
- [ ] T023 [US3] Honor the `/visual-assets/preview` contract by wiring a tooling hook or asset ingestion script referenced in `specs/001-textured-visuals/contracts/visual-assets.openapi.yaml`.

**Parallel Example (US3)**: T019 can run alongside T020 since it targets `tests/`, while T021 and T022 proceed sequentially within `src/systems/textures/overrides.rs`. T023 can begin once overrides expose an API endpoint or script integration point.

**Checkpoint**: Each level presents unique art, and asset-swapping fits into the documented pipeline.

---

## Phase 6: User Story 4 - Level Switch Preview (Priority: P4)

**Goal**: Provide an **L** key shortcut (and matching contract endpoint) that cycles levels in order, wrapping safely for rapid visual QA.

**Independent Test**: From a running session, press **L** repeatedly to cycle through all levels in <2 seconds each, verifying textures reset correctly and logging remains clean.

### Tests for User Story 4

- [ ] T024 [P] [US4] Add `KeyCode::L` integration test in `tests/level_switcher.rs` covering wrap-around and texture reinitialization.

### Implementation for User Story 4

- [ ] T025 [US4] Implement `LevelSwitchRequested` event + `LevelSwitchState` resource and scheduler wiring in `src/systems/level_switch.rs` per research.md Decision 4.
- [ ] T026 [US4] Connect keyboard input to the new event in `src/main.rs` (or existing input system) and ensure `src/level_loader.rs` handles queued switches.
- [ ] T027 [US4] Mirror the `/levels/next` contract by exposing a debug command or tooling shim as defined in `specs/001-textured-visuals/contracts/visual-assets.openapi.yaml`.

**Parallel Example (US4)**: T024 can be authored while T025 builds the event/resource plumbing. T026 and T027 can proceed concurrently once the event exists (different files: `src/main.rs` vs contracts/tooling script).

**Checkpoint**: Artists loop through levels instantly via keyboard or tooling endpoint.

---

## Phase 7: Polish & Cross-Cutting Concerns

- [ ] T028 [P] Update `specs/001-textured-visuals/quickstart.md` with any new commands or flags discovered during implementation.
- [ ] T029 Document manifest + override authoring steps in `README.md` (or a new section) to guide artists.
- [ ] T030 Run the full validation matrix (`cargo test`, `cargo clippy --all-targets --all-features`, `bevy lint`, `cargo build --target wasm32-unknown-unknown --release`) and capture results inside `specs/001-textured-visuals/quickstart.md`.

---

## Dependencies & Execution Order

1. **Phase 1 → Phase 2**: Setup must complete before foundational ECS resources compile.
2. **Phase 2 → User Stories**: Foundational resources block all story phases; once complete, US1..US4 may proceed (priority order recommended: P1 → P4).
3. **User Stories**: Each story remains independently testable but may start in parallel after Phase 2 if staffing allows. Ensure US1 (baseline) lands before dependent visual polish (US2/US3).
4. **Polish**: Runs last after desired stories ship.

## Parallel Opportunities Summary

- Setup: T001–T003 share no files; execute concurrently.
- Foundational: T004/T005 can run in parallel (structs vs loader), while T006/T007 proceed once resources exist.
- US1–US4: Tests (T008, T009, T014, T019, T024) are parallel-friendly; contract/documentation tasks (T013, T018, T023, T027) can run alongside gameplay code.

## Implementation Strategy

1. **MVP**: Complete Phases 1–3 (through US1) to deliver textured baseline objects with fallbacks.
2. **Incremental Delivery**: Layer US2 (type-driven) → US3 (per-level) → US4 (level switch) based on priority, validating each slice via its independent test before merging.
3. **Parallel Teaming**: After Phase 2, separate contributors can own different user stories, coordinating via the shared manifest resources defined earlier.

## Parallel Execution Examples per User Story

- **US1**: Run `tests/texture_manifest.rs` (T008) while another dev completes `src/systems/textures/materials.rs` updates (T010).
- **US2**: Build `TypeVariantRegistry` (T015) simultaneously with integration tests (T014) to validate behavior early.
- **US3**: One contributor updates level schemas (T020) as another implements hot-reload logic (T022).
- **US4**: Develop the `LevelSwitchRequested` event (T025) while a teammate authors the `/levels/next` tooling shim (T027).

## MVP Scope Recommendation

Deliver up through **User Story 1** (Phases 1–3). This slice guarantees textured gameplay with resilient fallbacks and unlocks stakeholder demos while later stories iterate in parallel.
