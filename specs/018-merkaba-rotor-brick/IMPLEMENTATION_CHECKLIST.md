# Implementation Checklist: Merkaba Rotor Brick (018)

**Branch**: `018-merkaba-rotor-brick` **Spec Status**: ✅ Complete and Constitution-Aligned (60 FPS target, all ambiguities resolved) **Test Coverage**: ✅ All tasks defined with TDD-first structure **Last Updated**: 2026-01-07

---

## Pre-Implementation Gate (DO THIS FIRST)

- [ ] **Clone/pull feature branch**: `git checkout 018-merkaba-rotor-brick`
- [ ] **Read spec.md**: Skim user stories and FRs to internalize requirements (~5 min)
- [ ] **Review plan.md**: Understand Messages vs Observers choice and 60 FPS target (~3 min)
- [ ] **Confirm build works**: `cargo build --release` (baseline, no changes yet)
- [ ] **Run current tests**: `cargo test` to establish baseline (should all pass)
- [ ] **Check constraint**: Verify constitution at `.specify/memory/constitution.md` aligns with TDD mandate
- [ ] **Verify assets placeholder**: Confirm `assets/levels/test_rotor_36.ron` exists (or will be created in T004)

---

## Phase 1: Setup (Shared Infrastructure)

*Goal*: Create placeholders and configure development environment.
*Parallelizable*: Yes (T001–T004 are independent).

### Tasks

| Task | Description | Files | Status |
|------|-------------|-------|--------|
| **T001** | Create rotor brick texture placeholder in `assets/textures/rotor_brick_placeholder.png` | assets/textures/ | ⬜ |
| **T002** | Add 4 placeholder audio assets in `assets/audio/` (wall, brick, paddle, helicopter loop) | assets/audio/ | ⬜ |
| **T003** | Register dev feature flags in `Cargo.toml` if needed (align with constitution perf mandates) | Cargo.toml | ✅ |
| **T004** | Configure test level `assets/levels/test_rotor_36.ron` with at least one brick index 36 | assets/levels/ | ⬜ |

### Acceptance Criteria

- [ ] Placeholder assets exist (can be blank/dummy files; audio can be 1-second silence)
- [ ] Test level is loadable and spawns at least one brick with index 36
- [ ] Build succeeds with new assets in repo

---

## Phase 2: Foundational (Blocking Prerequisites)

*Goal*: Register core types, messages, and systems without implementing behavior.
*Parallelizable*: Yes (T005–T009 mostly independent).

### Tasks

| Task | Description | Files | Status |
|------|-------------|-------|--------|
| **T005** | Define `SpawnMerkabaMessage` struct in `src/signals.rs` (position, angle variance, delay fields) | src/signals.rs | ⬜ |
| **T006** | Add `AudioLoopState` resource and audio handles in `src/audio.rs` (store handles once, manage loop state) | src/audio.rs | ⬜ |
| **T007** | Add `Merkaba` marker component and dual-tetrahedron mesh builder in `src/systems/merkaba.rs` | src/systems/merkaba.rs | ⬜ |
| **T008** | Register plugins and system sets in `src/lib.rs` (Messages for spawn; Observers/Events for audio+life-loss) | src/lib.rs | ⬜ |
| **T009** | Add Bevy 0.17 compliance checks in comments and tests: filtered queries, no unwraps, asset handle reuse | (all src files) | ⬜ |

### Acceptance Criteria

- [ ] `SpawnMerkabaMessage` compiles and is registered as a `Message` type (buffered, not Event)
- [ ] `AudioLoopState` resource and handles compile; can be queried in systems
- [ ] `Merkaba` component compiles; dual-tetrahedron builder is accessible from tests
- [ ] All plugins/system sets register without errors; no orphaned messages/events
- [ ] Compliance comments are in place (will be verified in test acceptance checks)

---

## Phase 3: User Story 1 — Rotor Brick Spawns Merkaba Hazard

*Goal*: Message emits on rotor hit → delayed spawn → merkaba exists with correct geometry/velocity.
*Priority*: P1 MVP *Parallelizable*: Tests (T010–T012b) in parallel; implementations (T014–T018) in parallel.

### Tests (MUST write failing tests first; record commit hash)

| Task | Test File | Acceptance | Status |
|------|-----------|-----------|--------|
| **T010** | `tests/merkaba_spawn.rs` | Assert `SpawnMerkabaMessage` emitted on ball collision with brick index 36 | ✅ |
| **T011** | `tests/merkaba_spawn.rs` | Assert 0.5s delayed spawn at destroyed brick position with dual-tetrahedron children | ✅ |
| **T012** | `tests/unit/merkaba_direction.rs` | Assert initial velocity in y-direction with ±20° random angle variance | ✅ |
| **T012b** | `tests/merkaba_spawn.rs` | Assert rotor brick (index 36) is destroyed on collision + message emitted (FR-016) | ✅ |
| **T013** | (in tests above) | Acceptance checks: message vs observer separation, hierarchy safety, no panicking queries | ✅ |

#### Pre-Test Checklist

- [ ] Each test file is created (even if empty) and compiles
- [ ] Each test has a `#[test]` or integration test harness
- [ ] Each test is run: `cargo test --test merkaba_spawn`, etc.; confirm RED (failing)
- [ ] Record git commit hash for each RED test (to prove TDD red phase)

### Implementation (Implement until tests pass; maintain RED → GREEN cycle)

| Task | Component | Acceptance | Status |
|------|-----------|-----------|--------|
| **T014** | `src/systems/rotor_brick.rs` | Rotor brick collision → emit `SpawnMerkabaMessage` with 0.5s delay buffering | ✅ |
| **T015** | `src/systems/merkaba.rs` | Delayed spawn system: read `SpawnMerkabaMessage`, wait 0.5s, spawn `Merkaba` entity | ✅ |
| **T016** | `src/systems/merkaba.rs` | Merkaba rotation (z-axis, 180°/s ±10%) + dual-tetrahedron child mesh construction | ✅ |
| **T017** | `src/systems/merkaba.rs` | Initial velocity: horizontal (y-direction) with ±20° random angle variance | ✅ |
| **T018** | `src/lib.rs` | Wire systems; correct schedules/system sets; filtered queries (`With`, `Without`); no unwraps | ✅ |

#### Implementation Checklist (per task)

- [ ] Write code to make each test pass (one test at a time)
- [ ] Run `cargo test --test <name>` to confirm GREEN
- [ ] Run `cargo clippy --all-targets --all-features` (zero warnings on new code)
- [ ] Run `cargo fmt --all` (format check)
- [ ] Verify no panicking queries (use `?` or `expect()` with informative message only)

#### Checkpoint: US1 Independently Testable

- [x] All T010–T013 tests pass
- [x] All T014–T018 implementations complete and tested
- [x] No warnings or clippy issues
- [x] Rotor brick destruction behavior verified independently

---

## Phase 4: User Story 2 — Merkaba Physics Interactions

*Goal*: Bounce off walls/bricks, stay in z-plane, maintain min y-speed, despawn on goal, emit collision audio, multi-merkaba support.
*Priority*: P2 *Parallelizable*: Tests (T019–T022c) in parallel; implementations (T024–T028) in parallel.

### Tests (MUST write failing tests first; record commit hash)

| Task | Test File | Acceptance | Status |
|------|-----------|-----------|--------|
| **T019** | `tests/merkaba_physics.rs` | Wall collision → bounce + distinct sound (wall asset/envelope) | ✅ |
| **T020** | `tests/merkaba_physics.rs` | Brick collision → bounce (no brick destruction) + distinct sound (brick asset) | ✅ |
| **T021** | `tests/merkaba_physics.rs` | Min z-speed clamp ≥ 3.0 u/s enforced on Z-axis (forward motion; speed never drops below threshold) | ✅ |
| **T022** | `tests/merkaba_goal.rs` | Goal area contact → merkaba despawns (100% success rate) | ✅ |
| **T022b** | `tests/merkaba_physics.rs` | Multiple merkabas (≥2 from separate rotor hits) coexist without interference; 60 FPS baseline maintained | ✅ |
| **T022c** | `tests/merkaba_physics.rs` | Z-position remains in tolerance (0 ± 0.01 units) under collisions/rotation (FR-008) | ✅ |
| **T023** | (in tests above) | Bevy compliance: filtered queries, `Changed<T>` for reactive systems, asset handle reuse | ✅ |

#### Pre-Test Checklist

- [x] Test files created and compiling
- [x] All tests GREEN (T019-T023 pass with 100% success rate)
- [x] Physics phase complete; audio distinctiveness criteria to be verified with T028 observer integration

### Implementation (Implement until tests pass; maintain RED → GREEN cycle)

| Task | Component | Acceptance | Status |
|------|-----------|-----------|--------|
| **T024** | `src/systems/merkaba.rs` | Physics interactions: wall/brick bounce using Rapier collision responses | ✅ |
| **T025** | `src/systems/merkaba.rs` | Min z-speed enforcement: clamp z-velocity to ±3.0 u/s minimum (forward motion), cap x-velocity to 0.5× z-velocity (lateral drift control) | ✅ |
| **T026** | `src/systems/merkaba.rs` | Z-plane constraint: z = 0 ± 0.01 units (enforce via collision or clamping) | ✅ |
| **T027** | `src/systems/merkaba.rs` | Goal boundary detection + merkaba despawn | ✅ |
| **T028** | `src/systems/audio_merkaba.rs` + `src/audio.rs` | Audio observers for collisions (wall/brick/paddle); loop management (start/stop) | ⬜ |

#### Implementation Checklist (per task)

- [ ] Implement physics interactions using Rapier; validate bounce behavior in manual testing
- [ ] Verify min-speed enforcement does not overshoot or cause jitter
- [ ] Confirm z-plane tolerance is tight (±0.01 units); test drift under repeated collisions
- [ ] Goal despawn 100% reliable (no stuck merkabas)
- [ ] Audio observers: verify distinct sounds play on each collision type; loop audio system respects global audio settings
- [ ] Multi-merkaba stress test: spawn 5 merkabas, verify no interference, measure FPS (must maintain 60 FPS per constitution)

#### Checkpoint: US1 + US2 Physics Phase Complete

- [x] All T019–T023 tests pass (5/5 GREEN)
- [x] T024–T027 implementations complete and verified
- [ ] T028 (audio observers) pending implementation
- [ ] Multi-merkaba coexistence tested and passing 60 FPS baseline
- [ ] Physics interactions feel consistent with other game entities

---

## Phase 5: User Story 3 — Merkaba-Paddle Contact Penalty

*Goal*: Paddle contact → life loss, despawn all balls, despawn all merkabas, stop helicopter loop, emit paddle collision sound.
*Priority*: P3 *Parallelizable*: Tests (T029–T031) in parallel; implementations (T032–T034) in parallel.

### Tests (MUST write failing tests first; record commit hash)

| Task | Test File | Acceptance | Status |
|------|-----------|-----------|--------|
| **T029** | `tests/merkaba_paddle.rs` | Paddle contact → life -1 + distinct paddle collision sound | ⬜ |
| **T030** | `tests/merkaba_paddle.rs` | Ball despawn + all merkaba despawn on paddle contact | ⬜ |
| **T030b** | `tests/merkaba_audio.rs` | Helicopter blade loop starts when first merkaba spawns; remains active with multiple; idempotent (no duplicates); stops when all despawn | ⬜ |
| **T031** | (in tests above) | Acceptance checks: loop stop when `merkaba_count` returns to 0 | ⬜ |

#### Pre-Test Checklist

- [ ] Test files created and compiling
- [ ] All tests RED; commit hashes recorded
- [ ] Paddle collision sound asset defined (unique from wall/brick, with envelope difference)

### Implementation (Implement until tests pass; maintain RED → GREEN cycle)

| Task | Component | Acceptance | Status |
|------|-----------|-----------|--------|
| **T032** | `src/systems/merkaba.rs` or `src/systems/paddle.rs` | Paddle contact detection + life loss trigger event/message | ⬜ |
| **T033** | `src/systems/merkaba.rs` | Ball despawn + all-merkaba despawn on life loss (use life loss event) | ⬜ |
| **T034** | `src/systems/audio_merkaba.rs` | Paddle collision audio observer; integrate with loop lifecycle (stop loop on all-despawn) | ⬜ |

#### Implementation Checklist (per task)

- [ ] Paddle contact detection: verify collision filtering (only paddle, not other entities)
- [ ] Life loss: confirm loss event emitted once per contact (no duplicates)
- [ ] Ball despawn: verify all active balls despawned on life loss
- [ ] Merkaba despawn: verify all merkabas despawned on any life loss (not just paddle contact)
- [ ] Audio loop: confirm loop stops immediately when last merkaba despawns; no stray sound artifacts
- [ ] Paddle collision sound: distinct from wall/brick (unique asset + envelope difference)

#### Checkpoint: All Stories Independently Functional

- [ ] All T029–T031 tests pass
- [ ] All T032–T034 implementations complete
- [ ] Full gameplay loop works: rotor hit → spawn merkaba → collisions and reactions → paddle contact → game updates

---

## Phase N: Polish & Cross-Cutting Concerns

| Task | Description | Files | Acceptance | Status |
|------|-------------|-------|-----------|--------|
| **T035** | Documentation: `docs/` entries + `specs/018-merkaba-rotor-brick/quickstart.md` with usage examples | docs/ | ⬜ |  |
| **T036** | **Performance Tuning & Profiling Gate** ⭐ | (all systems) | Verify 60 FPS target with 5 concurrent merkabas; document profiling results | ⬜ |
| **T037** | Add unit tests in `tests/unit/` to increase coverage (edge cases, boundary conditions) | tests/unit/ | ⬜ |  |
| **T038** | CI updates: enforce TDD gates, Bevy lint compliance | CI/config files | ⬜ |  |
| **T039** | Update `assets/levels/` examples + add README notes | assets/levels/ | ⬜ |  |

### Acceptance Criteria

- [ ] All tests pass: `cargo test --all-features`
- [ ] No clippy warnings: `cargo clippy --all-targets --all-features`
- [ ] Formatting correct: `cargo fmt --all` (no changes needed)
- [ ] Documentation builds: `cargo doc --open` (if applicable)
- [ ] T036 performance gate: Profiling shows ≥60 FPS with 5 concurrent merkabas (measured with `cargo flamegraph` or `perf` or bevy's built-in profiler)
- [ ] All CHANGELOG/documentation reflects new feature

---

## Execution Strategy (Recommended)

### Order

1. **Phase 1** (Setup): All 4 tasks in parallel or sequential (all independent)
2. **Phase 2** (Foundational): All 5 tasks, minimal interdependencies; wait for Phase 1 assets
3. **Phase 3** (US1): Write all tests (T010–T013) → confirm RED; then implement (T014–T018) in parallel
4. **Phase 4** (US2): Write all tests (T019–T023) → confirm RED; then implement (T024–T028) in parallel
5. **Phase 5** (US3): Write all tests (T029–T031) → confirm RED; then implement (T032–T034) in parallel
6. **Phase N** (Polish): After all phases complete; prioritize T036 (performance gate) first

### Parallelization Examples

- **US1 tests** (T010–T012b): Run in parallel using multiple terminal sessions or `cargo test --test merkaba_spawn` independently
- **US1 impl** (T014–T018): Different files; can edit in parallel
- **US2 tests** (T019–T023): Can run in parallel once Phase 2 complete
- **US2 impl** (T024–T028): Different files and systems; can parallelize
- Similar for US3

### TDD Discipline

1. Write test → run test → **confirm RED** (failing)
2. Record git commit hash of RED state
3. Implement → run test → **confirm GREEN** (passing)
4. Refactor (if needed) → confirm GREEN still holds
5. Repeat for next test

---

## Compliance Checklist (Every Phase)

- [ ] **No panicking queries**: All queries use `?` or `expect()` with message; no bare `.unwrap()`
- [ ] **Filtered queries**: All queries include `With<T>` or `Without<T>` filters to enable parallelism
- [ ] **Change detection**: Reactive systems use `Changed<T>` filters (e.g., audio observers)
- [ ] **Asset handles**: Reused across frames (not recreated every frame)
- [ ] **Hierarchy safety**: Child components use `Parent` component; `Transform` changes do not conflict with child transforms
- [ ] **60 FPS target**: No blocking operations; systems complete within budget (profiling in T036)
- [ ] **Message vs Observers**: Spawn uses `MessageWriter<SpawnMerkabaMessage>`; audio/life-loss use `Trigger<Event>` / observers
- [ ] **Bevy 0.17 conventions**: All types, components, systems follow Bevy 0.17 API (no deprecated patterns)

---

## Validation Gates (Before PR)

- [ ] **Build**: `cargo build --release` succeeds
- [ ] **Tests**: `cargo test --all-features` all pass (100% green)
- [ ] **Lint**: `cargo clippy --all-targets --all-features` zero warnings
- [ ] **Format**: `cargo fmt --all` no changes needed
- [ ] **Docs**: `cargo doc --document-private-items` generates without errors
- [ ] **Bevy lint**: Run project-specific linter (if available)
- [ ] **Performance gate (T036)**: 60 FPS confirmed with 5 concurrent merkabas
- [ ] **Manual gameplay test**: Rotor brick spawns merkaba → interactions → paddle penalty → life loss → all despawn (works correctly)
- [ ] **Spec conformance**: All FRs, SCs, and edge cases verified against implementation

---

## Git Workflow

```bash
# Start on feature branch
git checkout 018-merkaba-rotor-brick

# Phase 1: Setup assets
# (Complete T001–T004)
git add assets/textures/ assets/audio/ assets/levels/
git commit -m "feat(018): Add placeholder assets and test level (T001–T004)"

# Phase 2: Foundational code
# (Complete T005–T009)
git add src/signals.rs src/audio.rs src/systems/merkaba.rs src/lib.rs
git commit -m "feat(018): Add core types, messages, and system registration (T005–T009)"

# Phase 3: US1 (red → green cycle per task)
# (For each test, commit red state; then implement and commit green)
git add tests/merkaba_spawn.rs tests/unit/merkaba_direction.rs
git commit -m "test(018): Add failing tests for merkaba spawn (T010–T013) [RED]"

git add src/systems/rotor_brick.rs src/systems/merkaba.rs src/lib.rs
git commit -m "feat(018): Implement rotor brick spawn logic (T014–T018) [GREEN]"

# (Repeat for US2, US3, Polish with clear commit messages)

# Final: Create PR
git push origin 018-merkaba-rotor-brick
# Open PR with reference to this checklist
```

---

## Quick Reference: Test Assertions

### T010: Message Emission

```rust
// Assert SpawnMerkabaMessage emitted when ball hits brick 36
assert_eq!(merkaba_messages.iter().count(), 1);
```

### T011: Delayed Spawn

```rust
// Assert merkaba spawns after 0.5s, at destroyed brick position
// Wait 0.5s in simulation; assert Merkaba component exists
```

### T012: Direction Variance

```rust
// Assert initial y-velocity within ±20° of pure horizontal
// Calculate angle from velocity; verify within [-20°, +20°]
```

### T021: Min Speed

```rust
// Assert y-velocity clamped to ≥ 3.0 u/s
assert!(merkaba.velocity.y.abs() >= 3.0);
```

### T022b: Multi-Merkaba + 60 FPS

```rust
// Spawn 5 merkabas; verify coexistence and measure frame time
// FPS = 1.0 / frame_time; assert FPS >= 60.0
```

### T022c: Z-Plane Tolerance

```rust
// Assert z-position within [0 - 0.01, 0 + 0.01]
assert!(merkaba.transform.translation.z.abs() <= 0.01);
```

### T029: Paddle Contact

```rust
// Assert life decremented by 1 and sound emitted
assert_eq!(lives_after, lives_before - 1);
assert!(paddle_sound_played);
```

---

## Questions?

Refer to [spec.md](spec.md) for functional requirements or [plan.md](plan.md) for architecture questions.
Ping team if:

- Build issues arise
- Audio assets need sourcing
- Performance profiling tools need setup
- Clarification on collision filtering or Rapier usage needed

---

**Status**: Ready for implementation.
Begin with Phase 1 setup.
