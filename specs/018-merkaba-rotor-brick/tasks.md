---

description: "Task list for Merkaba Rotor Brick feature"
---

# Tasks: Merkaba Rotor Brick (018)

**Input**: Design documents from `specs/018-merkaba-rotor-brick/` **Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Tests are MANDATORY for all user stories.
Write tests first, commit failing tests (red) and record the commit hash in the task description, then implement to pass (green).
Tests MUST be approved before implementation.

**Bevy 0.17 compliance**: Tasks include acceptance checks to enforce Bevy mandates (no panicking queries, filtered queries, `Changed<T>` for reactive UI, correct Message vs Observer usage, asset handle reuse, hierarchy APIs).

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: US1, US2, US3
- Include exact file paths

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Ensure baseline infrastructure and assets paths are ready.

- [ ] T001 [P] Create rotor brick texture placeholder in `assets/textures/rotor_brick_placeholder.png`
- [ ] T002 [P] Add placeholder audio assets: `assets/audio/merkaba_wall.ogg`, `assets/audio/merkaba_brick.ogg`, `assets/audio/merkaba_paddle.ogg`, `assets/audio/merkaba_loop_helicopter.ogg`
- [ ] T003 [P] Register dev feature flags in `Cargo.toml` if needed for fast builds (align with constitution performance mandates)
- [ ] T004 Configure test level `assets/levels/test_rotor_36.ron` with at least one brick index 36

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core resources, messages, event registration.

- [ ] T005 Define `SpawnMerkabaMessage` in `src/signals.rs` (buffered message; position, angle variance, delay)
- [ ] T006 [P] Add `AudioLoopState` resource and audio handles in `src/audio.rs` (store handles once; loop state; respect global audio)
- [ ] T007 [P] Add `Merkaba` marker and mesh builder utilities in `src/systems/merkaba.rs` (dual tetrahedron children; hierarchy-safe)
- [ ] T008 Register plugins and system sets in `src/lib.rs` (Messages for spawn; Observers/Events for audio + life-loss)
- [ ] T009 Add Bevy 0.17 compliance checks in comments and tests: filtered queries, no unwraps, asset handle reuse (applies to all phases)

**Checkpoint**: Foundation ready — proceed to user stories.

---

## Phase 3: User Story 1 — Rotor Brick Spawns Merkaba Hazard (Priority: P1) 🎯 MVP

**Goal**: Hitting rotor brick (index 36) emits buffered message; after 0.5s spawn merkaba at brick position; rotates around z; initial y-direction ±20°.

**Independent Test**: Level with brick 36; hit it; merkaba spawns and rotates; initial direction within range.

### Tests for US1 (REQUIRED)

- [ ] T010 [P] [US1] Write failing integration test in `tests/merkaba_spawn.rs` to assert message emission on brick 36 hit (record failing commit hash)
- [ ] T011 [P] [US1] Write failing integration test in `tests/merkaba_spawn.rs` to assert 0.5s delayed spawn at brick position with dual tetrahedron children (record failing commit hash)
- [ ] T012 [P] [US1] Write failing unit test in `tests/unit/merkaba_direction.rs` for initial y-direction ±20° (record failing commit hash)
- [ ] T012b [P] [US1] Write failing integration test in `tests/merkaba_spawn.rs` to assert rotor brick (index 36) is destroyed on collision while spawn message is emitted (FR-016; record failing commit hash)
- [ ] T013 [US1] Add acceptance checks for Bevy mandates: message vs observer separation, hierarchy safety, no panicking queries (in tests)

### Implementation for US1

- [ ] T014 [P] [US1] Implement rotor brick collision → `SpawnMerkabaMessage` emit in `src/systems/rotor_brick.rs`
- [ ] T015 [US1] Implement delayed spawn system (0.5s) reading `SpawnMerkabaMessage` in `src/systems/merkaba.rs`
- [ ] T016 [US1] Implement merkaba rotation (around z-axis) and child mesh construction in `src/systems/merkaba.rs`
- [ ] T017 [US1] Implement initial velocity logic (horizontal y ±20°) in `src/systems/merkaba.rs`
- [ ] T018 [US1] Wire systems in `src/lib.rs` to correct schedules/sets; add filters (`With`, `Without`) for queries and ensure no unwraps

**Checkpoint**: US1 independently testable.

---

## Phase 4: User Story 2 — Merkaba Physics Interactions (Priority: P2)

**Goal**: Bounce off walls/bricks, stay in plane, maintain min y-speed (≥3.0 u/s), despawn on goal.

**Independent Test**: Manually spawn merkaba; verify bounce responses, min-speed enforcement, goal despawn.

### Tests for US2 (REQUIRED)

- [ ] T019 [P] [US2] Write failing integration test `tests/merkaba_physics.rs` for wall bounce + distinct sound (record failing commit hash)
- [ ] T020 [P] [US2] Write failing integration test `tests/merkaba_physics.rs` for brick bounce (no destruction) + distinct sound (record failing commit hash)
- [ ] T021 [P] [US2] Write failing unit test `tests/unit/merkaba_min_speed.rs` for min y-speed clamp ≥ 3.0 u/s (record failing commit hash)
- [ ] T022 [US2] Write failing integration test `tests/merkaba_goal.rs` for goal area despawn (record failing commit hash)
- [ ] T022b [P] [US2] Write failing integration test `tests/merkaba_physics.rs` to assert multiple merkabas (≥2 from separate rotor hits) coexist without interference or performance degradation; validate 60 FPS baseline (FR-015; record failing commit hash)
- [ ] T022c [P] [US2] Write failing unit test `tests/unit/merkaba_z_plane.rs` to assert z-position remains within tolerance (0 ± 0.01 units) under collisions and rotation (FR-008; record failing commit hash)
- [ ] T023 [US2] Add Bevy compliance checks: filtered queries, `Changed<T>` where reactive, asset handle reuse

### Implementation for US2

- [ ] T024 [P] [US2] Implement physics interaction systems (wall/brick bounce) in `src/systems/merkaba.rs` using Rapier collisions
- [ ] T025 [US2] Implement min y-speed enforcement in `src/systems/merkaba.rs`
- [ ] T026 [US2] Constrain z-plane (fixed or narrow band) in `src/systems/merkaba.rs`
- [ ] T027 [US2] Implement goal boundary detection + despawn in `src/systems/merkaba.rs`
- [ ] T028 [US2] Implement audio observers for collisions (wall/brick) in `src/systems/audio_merkaba.rs` and loop management in `src/audio.rs`

**Checkpoint**: US1 + US2 independently functional.

---

## Phase 5: User Story 3 — Merkaba-Paddle Contact Penalty (Priority: P3)

**Goal**: Paddle contact → lose life, despawn balls and all merkabas; stop helicopter loop; distinct paddle collision sound.

**Independent Test**: Spawn merkaba; drive into paddle; verify life loss and despawns.

### Tests for US3 (REQUIRED)

- [ ] T020b [US3] Write failing integration test `tests/merkaba_audio.rs` to assert helicopter blade loop starts when first merkaba spawns and remains active with multiple merkabas; verify idempotency and no duplicate loops (FR-020 start condition; record failing commit hash)
- [ ] T039 [P] [US3] Write failing integration test `tests/merkaba_paddle.rs` for paddle contact → life -1 + distinct sound (record failing commit hash)
- [ ] T030 [P] [US3] Write failing integration test `tests/merkaba_paddle.rs` for ball despawn and merkaba despawn on life loss (record failing commit hash)
- [ ] T031 [US3] Add acceptance checks for loop stop when merkaba_count returns to 0

### Implementation for US3

- [ ] T032 [US3] Implement paddle contact detection and life loss trigger in `src/systems/merkaba.rs` or `src/systems/paddle.rs`
- [ ] T033 [US3] Implement ball despawn and all-merkaba despawn on life loss in `src/systems/merkaba.rs`
- [ ] T034 [US3] Implement paddle collision audio observer in `src/systems/audio_merkaba.rs`

**Checkpoint**: All stories independently functional.

---

## Phase N: Polish & Cross-Cutting Concerns

- [ ] T035 [P] Documentation updates in `docs/` and `specs/018-merkaba-rotor-brick/quickstart.md`
- [ ] T036 Performance tuning and profiling; verify 60 FPS target; optimize systems
- [ ] T037 [P] Add additional unit tests in `tests/unit/` to increase coverage
- [ ] T038 CI updates to enforce TDD gates and Bevy lint
- [ ] T039 [P] Update `assets/levels/` examples and add README notes

---

## Dependencies & Execution Order

- Setup → Foundational → US1 (MVP) → US2 → US3 → Polish
- User stories are independently testable and can be parallelized after Foundational.

### Parallel Execution Examples

- [ ] [US1] Run T010–T012 tests in parallel; implement T014–T017 in parallel (different files)
- [ ] [US2] Run T019–T022 tests in parallel; implement T024–T028 in parallel
- [ ] [US3] Run T029–T031 tests in parallel; implement T032–T034 in parallel

## Implementation Strategy

- MVP first: Complete US1 before proceeding; validate independently.
- Incremental delivery: Add US2 and US3 independently with tests.
