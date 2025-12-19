---

description: "Task list for Systems Constitution Refactor"

---

# Tasks: Systems Constitution Refactor

**Input**: Design documents from `specs/011-refactor-systems/`

**Prerequisites**: [specs/011-refactor-systems/plan.md](plan.md) (required), [specs/011-refactor-systems/spec.md](spec.md) (required)

**Tests**: Tests are MANDATORY for all user stories.
Each story MUST include tests written and committed first, verified to FAIL (red), and then approved before implementation begins.

**Bevy 0.17 compliance**: Include explicit checks for fallible systems (`Result`), no panicking queries, query specificity, `Changed<T>` for reactive systems, correct message vs event usage, asset handle reuse, required component markers, **system organization via system sets**, and **plugin-based architecture**.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare documentation, task tracking, and test scaffolding for a refactor-focused feature.

- [X] T001 Ensure feature docs are present in specs/011-refactor-systems/ (plan.md, spec.md)
  - ✓ Created spec.md, plan.md, compliance-audit.md, refactoring-plan.md
- [X] T002 [P] Add CI/local verification notes to specs/011-refactor-systems/quickstart.md (create if missing) for `cargo test`, `cargo fmt --all`, `cargo clippy --all-targets --all-features`, `bevy lint`
  - ✓ Created quickstart.md with full verification workflow

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared foundations required before any story work begins.

- [X] T003 Define shared systems error type `SystemsError` in src/systems/mod.rs (or src/systems/error.rs) for fallible systems (Constitution VIII: Fallible Systems)
  - ✓ Documented fallible systems pattern in src/systems/mod.rs
  - ✓ Using `Result<(), Box<dyn std::error::Error>>` as standard return type
- [X] T004 Define a consistent "expected failure" policy for systems queries (0/1/many entities): document in docs/systems.md or docs/developer-guide.md (Constitution VIII: Error Recovery Patterns)
  - ✓ Created docs/systems.md with comprehensive error handling patterns
  - ✓ Documented query error handling: 0 entities, 1 required, 1 optional, many
  - ✓ Documented resource error handling: required vs optional
  - ✓ Included common pitfalls and best practices

**Checkpoint**: Foundation ready — user story work can begin.

---

## Phase 3: User Story 1 — Produce a Complete Compliance Audit (Priority: P1) 🎯 MVP

**Goal**: A complete, traceable compliance audit for `src/systems` against Constitution Section VIII + other applicable MUST/NEVER rules.

**Independent Test**: Audit artifact can be verified for coverage and traceability without any refactor changes.

### Tests for User Story 1 (REQUIRED) ⚠️

- [ ] T005 [P] [US1] Add audit-artifact test in tests/systems_compliance_audit.rs that fails until specs/011-refactor-systems/compliance-audit.md (a) references every src/systems/*.rs file and (b) includes findings for Section VIII mandates "Plugin-Based Architecture" and "System Organization"; commit failing test and record hash in this task description (Constitution VII, Constitution VIII)
  - **Red commit**: _pending_
  - **Failing tests**: _pending_
  - **Awaiting approval** ⏳
- [ ] T006 [US1] Approval gate: requestor explicitly approves the US1 failing tests (record approver + date in this task) before T007–T008 proceed (Constitution VII: Approval gate)
  - **Approver**: _pending_
  - **Approved**: _pending_

### Implementation for User Story 1

- [ ] T007 [US1] Update specs/011-refactor-systems/compliance-audit.md to ensure every finding includes (file path + Constitution rule + explanation) and covers all src/systems files (Constitution VIII + VI)
- [ ] T008 [US1] Validate audit completeness by running `cargo test` and manually spot-checking 3 findings for confirmability (Spec US1 Acceptance Scenarios)

---

## Phase 4: User Story 2 — Refactor Systems Code into Full Compliance (Priority: P2)

**Goal**: Refactor `src/systems` (plus minimal supporting edits) to comply with Constitution mandates/prohibitions while keeping behavior stable.

**Independent Test**: Code compiles, tests pass, and previously identified violations are resolved or explicitly scoped out.

### Tests for User Story 2 (REQUIRED) ⚠️

- [ ] T009 [P] [US2] Add fallible-system conformance test (compile-time or behavior test) ensuring systems return Result and use non-panicking query patterns; commit failing test and record hash in this task description (Constitution VII, Constitution VIII: Fallible Systems, NO Panicking Queries)
  - **Test file**: tests/systems_fallible.rs (create)
  - **Commit**: _pending_
  - **Tests**: _pending_
- [ ] T010 [P] [US2] Add asset caching test in tests/systems_assets.rs that fails until audio/texture assets are cached in Resources; commit failing test and record hash in this task description (Constitution VII, Constitution VIII: Asset Handle Reuse)
  - **Test file**: tests/systems_assets.rs (create)
  - **Commit**: _pending_
  - **Tests**: _pending_
- [ ] T011 [P] [US2] Add change-detection test for reactive systems (if applicable) that fails until systems use Changed<T> filters; commit failing test and record hash in this task description (Constitution VII, Constitution VIII: Change Detection)
  - **Test file**: tests/systems_change_detection.rs (create)
  - **Commit**: _pending_
  - **Tests**: _pending_
- [ ] T012 [US2] Approval gate: requestor explicitly approves the US2 failing tests (record approver + date in this task) before T013+ proceed (Constitution VII: Approval gate)
  - **Approver**: _pending_
  - **Approved**: _pending_

### Implementation for User Story 2

- [ ] T013 [US2] Establish Result-returning system wrapper pattern in src/systems/mod.rs; document pattern for future systems (Constitution VIII: Fallible Systems)
  - **Deliverable**: Wrapper pattern supports Result-returning systems without breaking Bevy 0.17 app build
  - **Files**: src/systems/mod.rs (documented pattern + example)
- [ ] T014 [US2] Refactor audio.rs systems to return Result and use fallible query patterns (Constitution VIII: Fallible Systems, Error Recovery Patterns)
  - **Files**: src/systems/audio.rs
  - **Pattern**: Replace `.unwrap()` with `?` or early `Ok(())` returns
- [ ] T015 [US2] Refactor cheat_mode.rs systems to return Result and use fallible patterns (Constitution VIII: Fallible Systems)
  - **Files**: src/systems/cheat_mode.rs
- [ ] T016 [US2] Refactor grid_debug.rs systems to return Result and use fallible patterns (Constitution VIII: Fallible Systems)
  - **Files**: src/systems/grid_debug.rs
- [ ] T017 [US2] Refactor level_switch.rs systems to return Result and use fallible patterns (Constitution VIII: Fallible Systems)
  - **Files**: src/systems/level_switch.rs
- [ ] T018 [US2] Refactor multi_hit.rs systems to return Result and use fallible patterns (Constitution VIII: Fallible Systems)
  - **Files**: src/systems/multi_hit.rs
- [ ] T019 [US2] Refactor paddle_size.rs systems to return Result and use fallible patterns (Constitution VIII: Fallible Systems)
  - **Files**: src/systems/paddle_size.rs
- [ ] T020 [US2] Refactor respawn.rs systems to return Result and use fallible patterns (Constitution VIII: Fallible Systems)
  - **Files**: src/systems/respawn.rs
- [ ] T021 [US2] Refactor scoring.rs systems to return Result and use fallible patterns (Constitution VIII: Fallible Systems)
  - **Files**: src/systems/scoring.rs
- [ ] T022 [US2] Refactor textures/ subsystem to return Result and use fallible patterns (Constitution VIII: Fallible Systems)
  - **Files**: src/systems/textures/
- [ ] T023 [US2] Cache audio asset handles in AudioAssets Resource at startup (Constitution VIII: Asset Handle Reuse)
  - **Files**: src/systems/audio.rs
- [ ] T024 [US2] Cache texture manifest handles at startup (Constitution VIII: Asset Handle Reuse)
  - **Files**: src/systems/textures/
- [ ] T025 [US2] Add `#[require(Transform, Visibility)]` to relevant marker components in systems (Constitution VIII: Required Components)
  - **Files**: src/systems/*.rs (as needed)
- [ ] T026 [US2] Fill rustdoc gaps for public items in src/systems/*.rs (Constitution VI)
  - **Files**: src/systems/*.rs
- [ ] T027 [US2] Organize systems into system sets with `*Systems` suffix and use `.configure_sets()` in each plugin (Constitution VIII: System Organization)
  - **Files**: src/systems/*.rs
- [ ] T028 [US2] Ensure each subsystem has a Plugin implementation (Constitution VIII: Plugin-Based Architecture)
  - **Files**: src/systems/*.rs, src/lib.rs

---

## Phase 5: User Story 3 — Preserve System Behavior (Priority: P3)

**Goal**: Ensure the refactor does not change system behavior visible to players.

**Independent Test**: Existing + added behavior tests pass; manual smoke test confirms no regressions.

### Tests for User Story 3 (REQUIRED) ⚠️

- [ ] T029 [P] [US3] Add audio behavior test in tests/systems_audio.rs verifying sound playback on events; commit failing test and record hash in this task description (Constitution VII)
  - **Commit**: _pending_
  - **Tests**: _pending_
- [ ] T030 [P] [US3] Add scoring behavior test in tests/systems_scoring.rs verifying point awards and milestones; commit failing test and record hash in this task description (Constitution VII)
  - **Commit**: _pending_
  - **Tests**: _pending_
- [ ] T031 [P] [US3] Add paddle size effect test in tests/systems_paddle_size.rs verifying shrink/enlarge behavior; commit failing test and record hash in this task description (Constitution VII)
  - **Commit**: _pending_
  - **Tests**: _pending_
- [ ] T032 [P] [US3] Add respawn behavior test in tests/systems_respawn.rs verifying ball respawn and lives tracking; commit failing test and record hash in this task description (Constitution VII)
  - **Commit**: _pending_
  - **Tests**: _pending_
- [ ] T033 [US3] Approval gate: requestor explicitly approves the US3 failing tests (record approver + date in this task) before T034–T035 proceed (Constitution VII: Approval gate)
  - **Approver**: _pending_
  - **Approved**: _pending_

### Implementation for User Story 3

- [ ] T034 [US3] Adjust systems as needed to satisfy new behavior tests while remaining Constitution-compliant (Constitution VIII: Fallible Systems + all mandates)
- [ ] T035 [US3] Manual smoke test — Native + WASM: start game, verify audio, scoring, paddle effects, respawn, level switching all work correctly. Document results in specs/011-refactor-systems/notes.md (Spec US3 Acceptance Scenarios)

---

## Phase 6: Polish & Cross-Cutting Concerns

- [ ] T036 [P] Run `cargo fmt --all` and commit formatting-only changes
- [ ] T037 Run `cargo clippy --all-targets --all-features` and fix clippy issues introduced by refactor (do not fix unrelated lint)
- [ ] T038 [P] Run `bevy lint` and fix Bevy-specific lint issues introduced by refactor
- [ ] T039 Update docs/systems.md (or create) with patterns adopted (fallible systems, system sets, plugin architecture)

---

## Dependencies & Execution Order

### User Story Dependencies

- **US1 (P1)** depends on Phase 2 only.
- **US2 (P2)** depends on US1 (audit findings define the refactor scope).
- **US3 (P3)** depends on US2 (behavior validation after refactor).

### Parallel Opportunities

- Phase 1 tasks marked [P] can run in parallel.
- In US2, tests T009–T011 can be developed in parallel (different files) before any implementation.
- In US3, tests T029–T032 can be developed in parallel.

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 + Phase 2
2. Complete US1 tests (red → hash recorded → approval → green)
3. Produce and validate compliance audit artifact
4. Stop and review audit before starting refactor work

### Incremental Delivery

- US1 (audit) → US2 (refactor compliance) → US3 (behavior preservation)
