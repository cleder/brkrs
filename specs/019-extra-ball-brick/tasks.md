# Tasks: Extra Ball Brick (Brick 41)

**Input**: Design documents from `/specs/019-extra-ball-brick/` **Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are MANDATORY for all user stories.
Write tests first, commit failing tests (red) and record the failing-test commit hash in task notes before implementation.
Obtain approval before moving to green.

**Bevy 0.17 compliance**: All tasks must honor Bevy mandates (no panicking queries, filtered queries, `Changed<T>` for reactive UI if touched, message vs observer correctness, asset handle reuse, safe hierarchy APIs).
Use Messages (buffered) for life and audio per spec; no observers for this feature.

## Phase 1: Setup (Shared Infrastructure)

- [X] T001 [P] Validate toolchain and lint setup for this branch (Rust 1.81, Bevy 0.17.3, bevy_rapier3d 0.32.0); ensure `cargo fmt`, `cargo clippy --all-targets --all-features`, and `bevy lint` run clean locally.
- [X] T002 [P] Add placeholder entry for brick 41 destruction sound at assets/audio/brick_41_extra_life.ogg (and fallback brick_generic_destroy.ogg if not present) so references in config/audio.ron can resolve.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data/config needed by all user stories.
No story work until complete.

- [X] T003 Define brick id 41 metadata (durability 1, score 0) in level format code paths (e.g., map id→brick kind) in src/level_format/ and expose through level loading.
- [X] T004 [P] Add brick 41 registry/metadata entry with sound handle key to assets (e.g., assets/levels/brick_41_extra_ball.ron or existing registry file) so levels can place this brick.
- [X] T005 [P] Add audio config mapping for brick 41 unique destruction sound and fallback to config/audio.ron; keep handle names consistent with assets/audio entry.
- [X] T006 [P] Ensure LifeAwardMessage and AudioMessage types exist and are documented for reuse in src/signals.rs; include Message derive, message-event separation note, and avoid duplicate definitions.- [X] T006b [P] Add regression test in tests/brick_regression.rs ensuring existing brick types (ids 1-40 or current range) retain score/sound/durability behavior after brick 41 code lands; prevents accidental breakage of brick hit pipeline.

---

## Phase 3: User Story 1 - Gain Extra Life on Brick 41 (Priority: P1) 🎯 MVP

**Goal**: Hitting brick 41 grants +1 life (clamped to max), despawns the brick, and awards 0 points.
**Independent Test**: Level with only brick 41 and a ball; hit once → life increments (clamped), brick despawns, score unchanged.

### Tests for User Story 1 (REQUIRED) ⚠️

- [X] T007 [P] [US1] Add failing integration test in tests/extra_ball_brick.rs covering single hit: +1 life (clamped), brick despawned, score/multiplier unchanged; record failing-test commit hash in task notes.
  **Failing-test commit:** `9533efc466ea50458f8ba6783450ee98c30b312f`
- [X] T008 [P] [US1] Add failing multi-ball collision test in tests/extra_ball_brick.rs ensuring only one life award and message-event separation (life via Message, no panics, brick not rehittable); record failing-test commit hash in task notes.
  **Failing-test commit:** `9533efc466ea50458f8ba6783450ee98c30b312f`

### Implementation for User Story 1

- [X] T009 [P] [US1] Implement brick 41 hit handling system in existing brick-hit module (`mark_brick_on_ball_collision`): on first valid hit, write `LifeAwardMessage { delta: +1 }`, mark brick for despawn; prevent double-awards on multi-ball via per-frame processed set; no unwraps.
- [X] T010 [US1] Register life award consumer in schedules (`register_brick_collision_systems`) so it runs with the brick hit pipeline and respects ordering with destruction/score systems.
- [X] T011 [US1] Ensure score pipeline ignores brick 41 (0 points, no combo/multiplier effects) via `brick_points()`; verified by tests that score remains unchanged.
- [X] T012 [US1] Wire level loading/spawning to instantiate brick 41 using existing metadata in `level_format` (EXTRA_LIFE_BRICK constant); loader already assigns `BrickTypeId` from matrix values; no repeated `asset_server.load`.
- [X] T013 [US1] Implement life consumer `apply_life_awards` that clamps to max and logs gracefully (no panics) when processing `LifeAwardMessage`; optional reader to avoid panics when message not initialized in certain tests.

**Checkpoint**: User Story 1 independently testable (life gain, brick despawn, score unchanged).

---

## Phase 4: User Story 2 - Unique Audio Feedback (Priority: P2)

**Goal**: Destroying brick 41 plays its unique destruction sound once (with fallback if asset missing), distinct from other bricks.
**Independent Test**: Mixed-brick level; destroying brick 41 plays only its unique sound once; other bricks retain their sounds.

### Tests for User Story 2 (REQUIRED) ⚠️

- [ ] T014 [P] [US2] Add failing audio integration test in tests/extra_ball_brick_audio.rs verifying brick 41 plays the unique sound once and other bricks do not reuse it; record failing-test commit hash in task notes.
- [ ] T015 [P] [US2] Add failing test covering multi-ball simultaneous hits: unique sound fires once, fallback sound used if dedicated asset missing; record failing-test commit hash in task notes.

### Implementation for User Story 2

- [ ] T016 [P] [US2] Load and store brick 41 destruction sound handle once in an audio resource/startup system (src/systems/audio.rs); include fallback handle reference to generic brick sound.
- [ ] T017 [US2] Hook brick 41 destruction to enqueue AudioMessage with the unique handle (fallback on missing) in the hit handling path; ensure message-event separation and no double-send after despawn.
- [ ] T018 [US2] Validate audio config and runtime wiring (config/audio.ron, assets/audio) and add instrumentation/logging for missing-handle fallback without panics.

**Checkpoint**: User Story 2 independently testable (unique audio once, fallback safe).

---

## Phase 5: Polish & Cross-Cutting Concerns

- [ ] T019 [P] Update documentation (specs/019-extra-ball-brick/quickstart.md and any user-facing docs) to include brick 41 behavior, sound, and testing steps.
- [ ] T020 Code cleanup and perf pass: ensure no per-frame work added; confirm asset handles reused; re-run `cargo fmt`, `cargo clippy --all-targets --all-features`, `bevy lint`, `cargo test`.
- [ ] T021 [P] WASM sanity check: build/run wasm target to ensure audio/life handling works and no platform-specific panics.

---

## Dependencies & Execution Order

- Phase 1 → Phase 2 → User Stories (P1 before P2 for MVP value); Polish after stories.
- User Story order: US1 (P1) first, US2 (P2) next.
  US2 can start after Phase 2 but should not block US1 delivery.
- Within each story: tests (fail) → implementation → story checkpoint.
- Bevy mandates: message-event separation, filtered queries, no unwraps, handle reuse, safe hierarchy.

## Parallel Execution Examples

- After Phase 2 completes, run T007 and T008 in parallel (US1 tests).
  Separately, T014 and T015 in parallel (US2 tests).
- Implementation parallelism: T009 and T011 can proceed in parallel (different files) once US1 tests are red; T016 can proceed in parallel with T009 after Phase 2.

## Implementation Strategy

- MVP: Deliver US1 first (life gain, 0 points, despawn) with passing tests; demo before US2.
- Incremental: After US1 passes, add US2 audio handling and tests, then polish.
- Always keep failing-test commit hashes documented for TDD proof before implementation tasks proceed.
