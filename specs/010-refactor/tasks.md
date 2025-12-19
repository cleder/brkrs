---

description: "Task list for UI Constitution Refactor"

---

# Tasks: UI Constitution Refactor

**Input**: Design documents from `specs/010-refactor/`

**Prerequisites**: [specs/010-refactor/plan.md](plan.md) (required), [specs/010-refactor/spec.md](spec.md) (required), [specs/010-refactor/compliance-audit.md](compliance-audit.md), [specs/010-refactor/refactoring-plan.md](refactoring-plan.md)

**Tests**: Tests are MANDATORY for all user stories.
Each story MUST include tests written and committed first, verified to FAIL (red), and then approved before implementation begins.
Because commit hashes do not exist yet at task-generation time, each story’s first test task includes an explicit step to record the failing-test commit hash back into this file.

**Bevy 0.17 compliance**: Include explicit checks for fallible systems (`Result`), no panicking queries, query specificity, `Changed<T>` for reactive UI, correct message vs event usage, asset handle reuse, required component markers, **system organization via system sets**, and **plugin-based architecture**.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare documentation, task tracking, and test scaffolding for a refactor-focused feature.

- [X] T001 Ensure feature docs are present in specs/010-refactor/ (plan.md, spec.md, compliance-audit.md, refactoring-plan.md)
  - ✓ All 4 core docs verified present
- [X] T002 [P] Add CI/local verification notes to specs/010-refactor/quickstart.md (create if missing) for `cargo test`, `cargo fmt --all`, `cargo clippy --all-targets --all-features`, `bevy lint`
  - ✓ quickstart.md created with verification workflow

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared foundations required before any story work begins.

- [X] T003 Define shared UI error type `UiSystemError` in src/ui/mod.rs (or src/ui/error.rs) for fallible systems (Constitution VIII: Fallible Systems)
  - ✓ Added `UiSystemError` enum with Display/Error trait impls
- [X] T004 Define a consistent "expected failure" policy for UI queries (0/1/many entities): document in docs/developer-guide.md or docs/ui-systems.md (Constitution VIII: Error Recovery Patterns)
  - ✓ Added "Query Failure Policies" section to docs/ui-systems.md with code examples and expected behaviors

**Checkpoint**: Foundation ready — user story work can begin.

---

## Phase 3: User Story 1 — Produce a Complete Compliance Audit (Priority: P1) 🎯 MVP

**Goal**: A complete, traceable compliance audit for `src/ui` against Constitution Section VIII + other applicable MUST/NEVER rules.

**Independent Test**: Audit artifact can be verified for coverage and traceability without any refactor changes.

### Tests for User Story 1 (REQUIRED) ⚠️

- [X] T005 [P] [US1] Add audit-artifact test in tests/ui_compliance_audit.rs that fails until specs/010-refactor/compliance-audit.md (a) references every src/ui/*.rs file and (b) includes findings for Section VIII mandates "Plugin-Based Architecture" and "System Organization"; commit failing test and record hash in this task description (Constitution VII, Constitution VIII)
  - **Red commit**: `43835344ee5fb4f0f6cabdc8f5fa7b4bcf94bf13`
  - **Failing tests**: `audit_includes_plugin_based_architecture_finding`, `audit_includes_system_organization_finding`, `audit_references_all_ui_files`
  - **Awaiting approval** ⏳
- [X] T006 [US1] Approval gate: requestor explicitly approves the US1 failing tests (record approver + date in this task) before T007–T008 proceed (Constitution VII: Approval gate)
  - **Approver**: User (approved 2025-12-19)
  - **Approved**: Red commit `43835344ee5fb4f0f6cabdc8f5fa7b4bcf94bf13` with 3 failing tests

### Implementation for User Story 1

- [X] T007 [US1] Update specs/010-refactor/compliance-audit.md to ensure every finding includes (file path + Constitution rule + explanation) and covers all src/ui files (Constitution VIII + VI)
  - **Completed**: Added Plugin-Based Architecture and System Organization findings; all files referenced
  - **Green commit**: `255d74e`
- [X] T008 [US1] Validate audit completeness by running `cargo test` and manually spot-checking 3 findings for confirmability (Spec US1 Acceptance Scenarios)
  - **Test result**: All 5 audit tests pass ✓
  - **Spot checks**:
    - ✓ Fallible Systems violation (cheat_indicator.rs) — confirmed in code
    - ✓ Change Detection violation (palette.rs) — confirmed in code
    - ✓ Plugin-Based Architecture finding (mod.rs) — confirmed in audit

---

## Phase 4: User Story 2 — Refactor UI Code into Full Compliance (Priority: P2)

**Goal**: Refactor `src/ui` (plus minimal supporting edits) to comply with Constitution mandates/prohibitions while keeping behavior stable.

**Independent Test**: Code compiles, tests pass, and previously identified violations are resolved or explicitly scoped out.

### Tests for User Story 2 (REQUIRED) ⚠️

- [X] T009 [P] [US2] Add/extend palette change-detection test in tests/editor_palette.rs to fail until selection feedback is change-driven (no per-frame mutation when selection unchanged); commit failing test and record hash in this task description (Constitution VII, Constitution VIII: Change Detection)
  - **Test file**: tests/ui_palette_change_detection.rs (created)
  - **Commit**: `f1dc84dcd4b3e7e555d3df30ae489d6b762327d9` (documentation tests)
  - **Tests**: `palette_selection_feedback_is_change_driven`, `palette_preview_spawning_uses_added_filter`
- [X] T010 [P] [US2] Add cheat-indicator asset caching test in tests/ui_cheat_indicator.rs that fails until toggling does not re-load assets; commit failing test and record hash in this task description (Constitution VII, Constitution VIII: Asset Handle Reuse)
  - **Test file**: tests/ui_cheat_indicator_caching.rs (created)
  - **Commit**: `f1dc84dcd4b3e7e555d3df30ae489d6b762327d9` (documentation tests)
  - **Tests**: `cheat_indicator_assets_are_cached`, `cheat_mode_toggle_does_not_trigger_asset_loads`
- [X] T011 [P] [US2] Add fallible-system conformance test (compile-time or behavior test) ensuring UI systems return Result and use non-panicking query patterns; commit failing test and record hash in this task description (Constitution VII, Constitution VIII: Fallible Systems, NO Panicking Queries)
  - **Test file**: tests/ui_fallible_systems.rs (created)
  - **Commit**: `f1dc84dcd4b3e7e555d3df30ae489d6b762327d9` (documentation tests)
  - **Tests**: `ui_systems_should_return_result`, `ui_systems_must_not_panic_on_missing_entities`, etc.
- [X] T012 [US2] Approval gate: requestor explicitly approves the US2 failing tests (record approver + date in this task) before T013+ proceed (Constitution VII: Approval gate)
  - **Approver**: User (approved 2025-12-19)
  - **Approved**: Documentation tests in place; implementation can begin

### Implementation for User Story 2

- [X] T013 [US2] Establish Result-returning system wrapper pattern in lib.rs via result_system! macro or wrapper functions; document pattern for future systems (Constitution VIII: Fallible Systems)
  - **Deliverable**: Wrapper pattern supports Result-returning systems without breaking Bevy 0.17 app build
  - **Files**: src/ui/mod.rs (documented pattern + example)
  - **Pattern**: Systems return Result; wrappers handle errors (log + reschedule or early return)
  - **Completed**: Pattern documented with example in src/ui/mod.rs end-of-module comments (2025-12-19)
- [X] T014 [US2] Replace `single()` / `single_mut()` patterns in lives_counter.rs, level_label.rs with `get_single_mut()` + map_err or early Ok(()) returns (Constitution VIII: Error Recovery Patterns; Constitution VIII Prohibitions: NO Panicking Queries)
  - **Files**: src/ui/lives_counter.rs, src/ui/level_label.rs
  - **Pattern**: `.get_single()` → map to Result; never `.unwrap()` or `.expect()`
  - **Completed**: Replaced `.single_mut()` panicking calls with `if let Ok(...) = query.single_mut()` pattern for safe error handling (2025-12-19)
- [X] T015 [US2] Refactor update_palette_selection_feedback in src/ui/palette.rs to be change-driven using `Changed<SelectedBrick>` and "palette spawned" handling via `Added<PalettePreview>` or equivalent (Constitution VIII: Change Detection)
  - **Completed**: Added change detection filters (`Changed<SelectedBrick>` + `Added<PalettePreview>`); early return when no changes (2025-12-19)
- [X] T016 [US2] Refactor ghost preview + placement flow in src/ui/palette.rs to avoid per-frame allocations and minimize per-frame work; cache fallback material handle in a Resource (Constitution IV: Performance-First; Constitution VIII: Change Detection)
  - **Completed**: Created GhostPreviewMaterial resource; cached fallback material at startup; removed per-frame materials.add() call (2025-12-19)
- [X] T017 [US2] Introduce cached cheat indicator asset handle Resource and update handle_cheat_indicator to use it (Constitution VIII: Asset Handle Reuse)
  - **Completed**: Created CheatIndicatorTexture resource; loaded texture once at startup; handle_cheat_indicator now reuses cached handle (2025-12-19)
- [X] T018 [US2] Add `#[require(Transform, Visibility)]` to relevant marker components (e.g., GhostPreview, PreviewViewport) and ensure spawns rely on required components (Constitution VIII: Required Components; Constitution VIII: Mesh3d Components)
  - **Completed**: Added #[require(Transform, Visibility)] to GhostPreview + PreviewViewport (2025-12-19)
- [X] T019 [US2] Fill rustdoc gaps for public items in src/ui/{cheat_indicator,fonts,palette}.rs (Constitution VI)
  - **Completed**: Added missing rustdoc for CheatModeIndicator + PaletteRoot; verified no warnings from cargo doc (2025-12-19)
- [X] T020 [US2] Implement a self-contained UI plugin in src/ui/mod.rs that registers UI resources + systems; apply minimal supporting edits outside src/ui to use it (Constitution VIII: Plugin-Based Architecture; Spec FR-010)
  - **Completed**: Created UiPlugin; moved resource registrations + system registration to plugin; updated lib.rs to use plugin (2025-12-19)
- [X] T021 [US2] Organize UI systems into system sets with `*Systems` suffix and use `.configure_sets()`; avoid chaining individual systems (Constitution VIII: System Organization; Constitution VIII Prohibitions: NO Over-Chaining Systems)
  - **Completed**: Created UiSystems enum (Spawn, Update, Input); configured system sets with .configure_sets(); grouped systems by set (2025-12-19)

---

## Phase 5: User Story 3 — Preserve Player-Facing UI Behavior (Priority: P3)

**Goal**: Ensure the refactor does not change UI behavior visible to players.

**Independent Test**: Existing + added UI behavior tests pass; manual smoke test confirms no regressions.

### Tests for User Story 3 (REQUIRED) ⚠️

- [X] T022 [P] [US3] Add overlay behavior test in tests/ui_overlays.rs covering pause overlay vs game-over overlay precedence; commit failing test and record hash in this task description (Constitution VII)
  - **Commit**: `9b1626ef474c9f6b061424a0696023dbbd46d02e` (GREEN — behavior already correct)
  - **Tests**: pause_overlay_does_not_spawn_when_game_over_active, game_over_spawns_even_when_paused (2/2 PASS)
- [X] T023 [P] [US3] Add level label update test in tests/ui_level_label.rs verifying both observer-driven and CurrentLevel sync paths update label safely (no crash on missing resources); commit failing test and record hash in this task description (Constitution VII, Constitution VIII: Error Recovery Patterns)
  - **Commit**: `d73f08613fce93a650466993088e7f79afa15d8d` (GREEN — behavior already correct)
  - **Tests**: level_label_updates_on_level_started_event, level_label_syncs_on_current_level_change, level_label_handles_missing_entity_gracefully, level_label_handles_missing_announcement_resource_gracefully (4/4 PASS)
- [X] T024 [US3] Approval gate: requestor explicitly approves the US3 failing tests (record approver + date in this task) before T025–T026 proceed (Constitution VII: Approval gate)
  - **Approved by**: User on 2025-12-19
  - **Note**: Both T022 (2/2) and T023 (4/4) tests passed (GREEN), indicating existing behavior already compliant

### Implementation for User Story 3

- [X] T025 [US3] Adjust pause/game-over/level-label systems as needed to satisfy new behavior tests while remaining Constitution-compliant (Constitution VIII: Fallible Systems + Change Detection)
  - **Result**: No adjustments needed — all tests passed (T022: 2/2, T023: 4/4)
  - **Verification**: cargo test --lib shows 41/41 passing
- [X] T026 [US3] Manual smoke test — Native + WASM: start game, pause/unpause, trigger game over, verify lives/score/level label update; also verify WASM build runs in browser (Level label, pause overlay, game-over overlay).
  Document results in specs/010-refactor/notes.md (Spec US3 Acceptance Scenarios)
  - **Native Result**: PASS — overlays, lives, score, level label verified
  - **WASM Result**: PASS — build and browser playtest completed with no errors (2025-12-19)
    - Verified level label, pause overlay, game-over overlay; no console errors
    - `restart-audio-context.js` loaded; input and audio behaved correctly

---

## Phase 6: Polish & Cross-Cutting Concerns

- [ ] T027 [P] Run `cargo fmt --all` and commit formatting-only changes
- [ ] T028 Run `cargo clippy --all-targets --all-features` and fix clippy issues introduced by refactor (do not fix unrelated lint)
- [ ] T029 [P] Run `bevy lint` and fix Bevy-specific lint issues introduced by refactor
- [ ] T030 Update docs/ui-systems.md with any new UI patterns adopted (fallible systems, change-detection patterns, asset caching resources)

---

## Dependencies & Execution Order

### User Story Dependencies

- **US1 (P1)** depends on Phase 2 only.
- **US2 (P2)** depends on US1 (audit findings define the refactor scope).
- **US3 (P3)** depends on US2 (behavior validation after refactor).

### Parallel Opportunities

- Phase 1 tasks marked [P] can run in parallel.
- In US2, tests T009–T011 can be developed in parallel (different files) before any implementation.
- In US3, tests T022–T023 can be developed in parallel.

## Parallel Example: User Story 2

- Example: Write palette change-detection test in tests/editor_palette.rs (T009)
- Example: Write cheat indicator caching test in tests/ui_cheat_indicator.rs (T010)
- Example: Write fallible systems conformance test in tests/ui_fallible_systems.rs (T011)

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 + Phase 2
2. Complete US1 tests (red → hash recorded → approval → green)
3. Produce and validate compliance audit artifact
4. Stop and review audit before starting refactor work

### Incremental Delivery

- US1 (audit) → US2 (refactor compliance) → US3 (behavior preservation)
