# Tasks: Indestructible bricks (LevelDefinition)

**Input**: Design documents from `/home/christian/devel/bevy/brkrs/specs/001-indestructible-bricks/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

---

## Summary

This tasks file breaks the work into phases: Setup, Foundational (blocking), then three user story phases (P1, P2, P3) followed by a polish phase. Each story-phase is independently testable. The MVP is User Story 1 (gameplay: levels complete despite indestructible bricks).

Total tasks: 21

---

## Phase 1: Setup (Shared Infrastructure)

Purpose: create project helpers, test scaffolding and add new constants/configs used by the feature.

- [x] T001 Create `tools/migrate-level-indices` helper scaffold (Rust cli) with README at `tools/migrate-level-indices/README.md`
- [ ] T002 [P] Add specification references and new index constants to `src/level_format/mod.rs` (document constant for `SIMPLE_BRICK=20`, `INDESTRUCTIBLE_BRICK=90`)
- [x] T003 [P] Add test harness scaffolding: create `tests/level_definition.rs` with placeholder test cases and include it in CI (update `Cargo.toml` test configs if necessary)

---

## Phase 2: Foundational (Blocking prerequisites)

Purpose: implement core parser + migration tooling used across user stories. All user stories depend on this phase.

- [x] T004 Implement LevelDefinition parser support for recognizing index `90` as indestructible in `src/level_loader.rs` and/or `src/level_format/mod.rs`
- [x] T005 [P] Update core Brick/BrickType runtime data structures to include `counts_towards_completion: bool` (files: `src/lib.rs`, `src/level_format/mod.rs`, `src/level_loader.rs`)
- [x] T006 [P] Implement migration CLI `tools/migrate-level-indices/src/main.rs` to update repository assets `assets/levels/*.ron` from `3` -> `20`, writing backups `*.ron.bak`
- [x] T007 Add unit tests for parser + BrickType semantics for indexes 3, 20, 90 in `tests/level_definition.rs` (assert parity/migration)

Checkpoint: Foundation complete — runtime understands index 90, canonical simple brick index is 20, and migration tooling exists for repo assets.

---

## Phase 3: User Story 1 - Player completes levels regardless of indestructible bricks (Priority: P1) 🎯 MVP

Goal: Ensure indestructible bricks (index `90`) do not block level completion and do not decrement the destructible counter.

Independent Test: Create a level with destructible + indestructible bricks and assert level completes when destructible bricks hit 0; test-level files under `assets/levels/test_mixed_indestructible.ron`.

- [x] T008 [US1] Add logic to LevelCompletionCounter to ignore bricks where `counts_towards_completion == false` in `src/level_loader.rs` and `src/systems/level_switch.rs`
- [x] T009 [US1] Add unit tests validating completion behaviour for mixed levels in `tests/level_definition.rs` (new tests focusing on runtime level completion)
- [x] T010 [US1] Add an integration test that loads a sample level from `assets/levels/test_mixed_indestructible.ron` and confirms completion (tests/integration/level_completion.rs)
- [x] T011 [US1] Add sample level `assets/levels/test_mixed_indestructible.ron` with comments describing expected behavior and add it to quickstart.md
- [x] T008 [US1] Add logic to LevelCompletionCounter to ignore bricks where `counts_towards_completion == false` in `src/level_loader.rs` and `src/systems/level_switch.rs`
- [x] T009 [US1] Add unit tests validating completion behaviour for mixed levels in `tests/level_definition.rs` (new tests focusing on runtime level completion)
- [x] T010 [US1] Add an integration test that loads a sample level from `assets/levels/test_mixed_indestructible.ron` and confirms completion (tests/integration/level_completion.rs)
- [x] T011 [US1] Add sample level `assets/levels/test_mixed_indestructible.ron` with comments describing expected behavior and add it to quickstart.md

Checkpoint: The gameplay behaviour is independently verifiable and passes tests.

---

## Phase 4: User Story 2 - Level designer can place indestructible bricks (Priority: P2)

Goal: Provide clear editor / level-format support and documentation so designers can author index `90` indestructible bricks safely.

Independent Test: Editor/loader renders tile index `90` as indestructible in a sample level; docs updated.

- [x] T012 [US2] Add rendering/behaviour hook to spawn indestructible bricks when parsing index `90` in `src/level_loader.rs` and any `src/systems/*` that spawn bricks (e.g., `src/systems/respawn.rs` or new `src/systems/indestructible.rs`)
- [x] T013 [US2] Update `assets/textures/manifest.ron` or texture mapping docs if a new sprite/visual is required (path: `assets/textures/`)
- [x] T014 [US2] Update documentation (docs/ or `specs/001-indestructible-bricks/quickstart.md`) showing how to author `90` and update `assets/levels/README.md` with examples
- [x] T015 [US2] Add rendering test / manual verification steps to `tests/visual_manual.md` (or integration test that asserts a brick entity exists with the right properties after load)

---

## Phase 5: User Story 3 - Update simple brick index for clear semantics (Priority: P3)

Goal: Move the canonical simple brick index from `3` → `20` for newly authored levels and ensure older files are migrated safely.

Independent Test: After running `tools/migrate-level-indices`, assets under `assets/levels/` no longer contain `3` for simple bricks; runtime behaves the same.

- [x] T016 [US3] Implement migration script execution in repository landing steps (add instructions to README and add `scripts/migrate-assets.sh` wrapper that calls `tools/migrate-level-indices`)
- [x] T017 [US3] Add regression tests confirming migration keeps visual/layout parity for example levels (tests/migration_parity.rs)
- [x] T018 [US3] Update any CI job to run migration tests on PRs that touch `assets/levels/` (CI config: `.github/workflows/*`)

---

## Final Phase: Polish & Cross-Cutting Concerns

Purpose: Docs, profiling, final validation and release prep.

- [ ] T019 [P] Clean up code comments and add in-code documentation for new constants (`src/level_format/mod.rs`)
- [ ] T020 [P] Update `specs/001-indestructible-bricks/quickstart.md` with exact run commands and CI instructions; run quickstart validation
- [ ] T021 [P] Profile gameplay (manual or automated) to ensure no FPS regressions caused by new behaviour (document in `specs/001-indestructible-bricks/plan.md`)

---

## Dependencies & Execution Order

- Setup (T001–T003) can run immediately. T002/T003 are parallelizable.
- Foundational (T004–T007) blocks user story work — must finish before T008–T018.
- User Story phases (T008–T015) can run in parallel after foundation completes but follow HUD/test dependencies inside each story (e.g., tests -> implementation -> integration tests).
- Polish (T019–T021) runs last and can be partially parallel.

### Parallel opportunities

- T002 & T003 are parallel [P].
- T005 & T006 (data structure changes and migration) are parallelizable to different files where safe [P].
- Each user story’s model/service tasks can be worked on by separate engineers in parallel.

---

## Counts & Quick Summary

- Total tasks: 21
- Tasks for US1 (P1): 4
- Tasks for US2 (P2): 4
- Tasks for US3 (P3): 3
- Setup/Foundation/Polish tasks: 10

## MVP Suggestion

Focus on User Story 1 first (T008–T011) after completing the Foundational phase (T004–T007). That yields a small, demonstrable feature (gameplay unaffected by indestructible bricks) and enables further updates.
