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

- [ ] T001 Ensure feature docs are present in specs/010-refactor/ (plan.md, spec.md, compliance-audit.md, refactoring-plan.md)
- [ ] T002 [P] Add CI/local verification notes to specs/010-refactor/quickstart.md (create if missing) for `cargo test`, `cargo fmt --all`, `cargo clippy --all-targets --all-features`, `bevy lint`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared foundations required before any story work begins.

- [ ] T003 Define shared UI error type `UiSystemError` in src/ui/mod.rs (or src/ui/error.rs) for fallible systems (Constitution VIII: Fallible Systems)
- [ ] T004 Define a consistent “expected failure” policy for UI queries (0/1/many entities): document in docs/developer-guide.md or docs/ui-systems.md (Constitution VIII: Error Recovery Patterns)

**Checkpoint**: Foundation ready — user story work can begin.

---

## Phase 3: User Story 1 — Produce a Complete Compliance Audit (Priority: P1) 🎯 MVP

**Goal**: A complete, traceable compliance audit for `src/ui` against Constitution Section VIII + other applicable MUST/NEVER rules.

**Independent Test**: Audit artifact can be verified for coverage and traceability without any refactor changes.

### Tests for User Story 1 (REQUIRED) ⚠️

- [ ] T005 [P] [US1] Add audit-artifact test in tests/ui_compliance_audit.rs that fails until specs/010-refactor/compliance-audit.md (a) references every src/ui/*.rs file and (b) includes findings for Section VIII mandates “Plugin-Based Architecture” and “System Organization”; commit failing test and record hash in this task description (Constitution VII, Constitution VIII)
- [ ] T006 [US1] Approval gate: requestor explicitly approves the US1 failing tests (record approver + date in this task) before T007–T008 proceed (Constitution VII: Approval gate)

### Implementation for User Story 1

- [ ] T007 [US1] Update specs/010-refactor/compliance-audit.md to ensure every finding includes (file path + Constitution rule + explanation) and covers all src/ui files (Constitution VIII + VI)
- [ ] T008 [US1] Validate audit completeness by running `cargo test` and manually spot-checking 3 findings for confirmability (Spec US1 Acceptance Scenarios)

---

## Phase 4: User Story 2 — Refactor UI Code into Full Compliance (Priority: P2)

**Goal**: Refactor `src/ui` (plus minimal supporting edits) to comply with Constitution mandates/prohibitions while keeping behavior stable.

**Independent Test**: Code compiles, tests pass, and previously identified violations are resolved or explicitly scoped out.

### Tests for User Story 2 (REQUIRED) ⚠️

- [ ] T009 [P] [US2] Add/extend palette change-detection test in tests/editor_palette.rs to fail until selection feedback is change-driven (no per-frame mutation when selection unchanged); commit failing test and record hash in this task description (Constitution VII, Constitution VIII: Change Detection)
- [ ] T010 [P] [US2] Add cheat-indicator asset caching test in tests/ui_cheat_indicator.rs that fails until toggling does not re-load assets; commit failing test and record hash in this task description (Constitution VII, Constitution VIII: Asset Handle Reuse)
- [ ] T011 [P] [US2] Add fallible-system conformance test (compile-time or behavior test) ensuring UI systems return Result and use non-panicking query patterns; commit failing test and record hash in this task description (Constitution VII, Constitution VIII: Fallible Systems, NO Panicking Queries)
- [ ] T012 [US2] Approval gate: requestor explicitly approves the US2 failing tests (record approver + date in this task) before T013+ proceed (Constitution VII: Approval gate)

### Implementation for User Story 2

- [ ] T013 [US2] Convert all UI systems in src/ui to return `Result<(), UiSystemError>` and update call sites/registration accordingly (Constitution VIII: Fallible Systems)
- [ ] T014 [US2] Replace `single()` / `single_mut()` usage patterns with `?` / `let Ok(..) = .. else { return Ok(()); }` as appropriate (Constitution VIII: Error Recovery Patterns; Constitution VIII Prohibitions: NO Panicking Queries)
- [ ] T015 [US2] Refactor update_palette_selection_feedback in src/ui/palette.rs to be change-driven using `Changed<SelectedBrick>` and “palette spawned” handling via `Added<PalettePreview>` or equivalent (Constitution VIII: Change Detection)
- [ ] T016 [US2] Refactor ghost preview + placement flow in src/ui/palette.rs to avoid per-frame allocations and minimize per-frame work; cache fallback material handle in a Resource (Constitution IV: Performance-First; Constitution VIII: Change Detection)
- [ ] T017 [US2] Introduce cached cheat indicator asset handle Resource and update handle_cheat_indicator to use it (Constitution VIII: Asset Handle Reuse)
- [ ] T018 [US2] Add `#[require(Transform, Visibility)]` to relevant marker components (e.g., GhostPreview, PreviewViewport) and ensure spawns rely on required components (Constitution VIII: Required Components; Constitution VIII: Mesh3d Components)
- [ ] T019 [US2] Fill rustdoc gaps for public items in src/ui/{cheat_indicator,fonts,palette}.rs (Constitution VI)
- [ ] T020 [US2] Implement a self-contained UI plugin in src/ui/mod.rs that registers UI resources + systems; apply minimal supporting edits outside src/ui to use it (Constitution VIII: Plugin-Based Architecture; Spec FR-010)
- [ ] T021 [US2] Organize UI systems into system sets with `*Systems` suffix and use `.configure_sets()`; avoid chaining individual systems (Constitution VIII: System Organization; Constitution VIII Prohibitions: NO Over-Chaining Systems)

---

## Phase 5: User Story 3 — Preserve Player-Facing UI Behavior (Priority: P3)

**Goal**: Ensure the refactor does not change UI behavior visible to players.

**Independent Test**: Existing + added UI behavior tests pass; manual smoke test confirms no regressions.

### Tests for User Story 3 (REQUIRED) ⚠️

- [ ] T022 [P] [US3] Add overlay behavior test in tests/ui_overlays.rs covering pause overlay vs game-over overlay precedence; commit failing test and record hash in this task description (Constitution VII)
- [ ] T023 [P] [US3] Add level label update test in tests/ui_level_label.rs verifying both observer-driven and CurrentLevel sync paths update label safely (no crash on missing resources); commit failing test and record hash in this task description (Constitution VII, Constitution VIII: Error Recovery Patterns)
- [ ] T024 [US3] Approval gate: requestor explicitly approves the US3 failing tests (record approver + date in this task) before T025–T026 proceed (Constitution VII: Approval gate)

### Implementation for User Story 3

- [ ] T025 [US3] Adjust pause/game-over/level-label systems as needed to satisfy new behavior tests while remaining Constitution-compliant (Constitution VIII: Fallible Systems + Change Detection)
- [ ] T026 [US3] Run manual smoke test: start game, pause/unpause, trigger game over, verify lives/score/level label update; document results in specs/010-refactor/notes.md (create) (Spec US3 Acceptance Scenarios)

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
