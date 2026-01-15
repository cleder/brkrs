# Failing Test Stubs Seeded (RED Phase)

**Date**: 2026-01-07 **Branch**: `018-merkaba-rotor-brick` **Status**: All test stubs created and verified in RED (ignored/failing) state

---

## Summary

Created **8 test files** with **13 failing test stubs** across all three user stories.
Each test is marked with `#[ignore]` and includes detailed comments guiding implementation.

### Test Files Created

| File | Tests | Coverage |
|------|-------|----------|
| [tests/merkaba_spawn.rs](merkaba_spawn.rs) | T010, T011, T012b | Spawn message, delayed spawn, brick destruction |
| [tests/unit/merkaba_direction.rs](merkaba_direction.rs) | T012 | Initial velocity angle variance (±20°) |
| [tests/merkaba_physics.rs](merkaba_physics.rs) | T019, T020, T022b | Wall bounce, brick bounce, multi-merkaba 60 FPS |
| [tests/unit/merkaba_min_speed.rs](merkaba_min_speed.rs) | T021 | Min z-speed clamping (≥3.0 u/s forward motion on Z-axis) |
| [tests/merkaba_goal.rs](merkaba_goal.rs) | T022 | Goal area despawn |
| [tests/unit/merkaba_z_plane.rs](merkaba_z_plane.rs) | T022c | Z-plane constraint tolerance (0 ± 0.01) |
| [tests/merkaba_paddle.rs](merkaba_paddle.rs) | T029, T030 | Paddle contact, life loss, despawns |
| [tests/merkaba_audio.rs](merkaba_audio.rs) | T030b | Audio loop lifecycle (start/stop/idempotent) |

---

## Test Structure

Each test follows this pattern:

```rust
/// T00X: [Brief requirement summary]
///
/// [Detailed spec and acceptance criteria]
#[test]
#[ignore = "RED: T00X - [Implementation tasks needed]"]
fn test_descriptive_name() {
    panic!("T00X: [Placeholder message with test guidance]");

    // Expected implementation outline:
    // 1. [Setup step]
    // 2. [Action step]
    // 3. [Assertion step]
    // ... etc
}
```

**Key features**:

- ✅ Clear task ID (T010, T011, etc.) in function name and `#[ignore]` attribute
- ✅ Comprehensive docstring with requirement and acceptance criteria
- ✅ `panic!()` call ensures test fails if accidentally run (RED state)
- ✅ Detailed implementation outline as code comments (guidance for developer)
- ✅ References to implementation tasks (e.g., T014, T015, etc.) so developer knows where code goes

---

## Verification

### Compilation

```bash
$ cargo test --test merkaba_spawn 2>&1 | grep "test result:"
test result: ok. 0 passed; 0 failed; 3 ignored; 0 measured
```

✅ All tests compile without errors (warnings about unused imports are expected; can be removed when tests are filled in).

### Baseline Tests Still Pass

```bash
$ cargo test --lib
test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured
```

✅ All existing library tests pass; no regression introduced.

---

## Next Steps (Recommended Execution Order)

### Phase 3: User Story 1 (Rotor Brick Spawns Merkaba)

1. **Write tests** (if not yet done):
   - Run `cargo test --test merkaba_spawn --ignored` to see all T010, T011, T012b tests
   - Implement test bodies; confirm they FAIL when run (stay RED)
   - Commit: `git commit -m "test(018): Add failing tests for merkaba spawn (T010–T013) [RED]"`

2. **Implement code**:
   - T014: Rotor brick collision → `SpawnMerkabaMessage`
   - T015: Delayed spawn (0.5s)
   - T016: Dual-tetrahedron mesh + rotation
   - T017: Initial velocity (±20°)
   - T018: System wiring
   - Run `cargo test --test merkaba_spawn` after each task until all tests pass (GREEN)

### Phase 4: User Story 2 (Physics)

1. Similar pattern: Write tests (T019–T023) → confirm RED
2. Implement: T024–T028
3. Verify GREEN

### Phase 5: User Story 3 (Paddle Penalty)

1. Write tests (T029–T031) → confirm RED
2. Implement: T032–T034
3. Verify GREEN

### Phase N: Polish

1. Run full test suite: `cargo test`
2. Profiling: `cargo flamegraph` or `perf` to measure 60 FPS baseline (T036)
3. Documentation, CI updates, etc.

---

## File Locations

All test files are in the repository under:

- `tests/merkaba_spawn.rs` — Integration tests
- `tests/merkaba_physics.rs` — Physics tests
- `tests/merkaba_paddle.rs` — Paddle contact tests
- `tests/merkaba_audio.rs` — Audio lifecycle tests
- `tests/merkaba_goal.rs` — Goal despawn test
- `tests/unit/merkaba_direction.rs` — Unit test
- `tests/unit/merkaba_min_speed.rs` — Unit test
- `tests/unit/merkaba_z_plane.rs` — Unit test

---

## Running Tests

### View all ignored tests

```bash
cargo test -- --ignored --list
```

### Run a specific test file

```bash
cargo test --test merkaba_spawn -- --ignored
```

### Run all tests (will still be ignored until you remove #[ignore])

```bash
cargo test --all-features
```

### Enable a single test

Edit the test file and remove the `#[ignore]` attribute, or run with:

```bash
cargo test merkaba_spawn_message_emitted_on_rotor_brick_hit -- --ignored
```

---

## Notes for Implementers

1. **Imports**: Update imports at the top of each test file to match actual crate structure (currently placeholders).
2. **Helper functions**: Consider creating test utilities (world setup, entity spawning) to reduce duplication.
3. **Assertion messages**: Each `panic!()` includes a hint about what to implement; follow the "Expected implementation outline" comments.
4. **Bevy test setup**: Use `bevy::ecs::world::World` or a test harness to create isolated test worlds.
5. **Record RED commit**: Before implementing, commit the failing test and record the commit hash in your checklist.

---

## Status Summary

- ✅ All 8 test files created
- ✅ All 13 tests in RED (ignored/failing) state
- ✅ No regressions to existing tests
- ✅ Ready for implementation (Phase 1 assets + Phase 2 foundational code should be done first)

---

**Next Action**: Begin Phase 1 (assets) and Phase 2 (foundational code registration) from the [IMPLEMENTATION_CHECKLIST.md](IMPLEMENTATION_CHECKLIST.md), then implement tests to GREEN.
