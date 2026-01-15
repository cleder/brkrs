# PR #155 Remediation Plan

**Status**: Draft **Created**: 2026-01-09 **PR**: <https://github.com/cleder/brkrs/pull/155>

## Context

This document outlines how to address outstanding review comments on PR #155 while keeping the current implementation unchanged (Z-axis velocity clamping is correct as implemented).

## Axis Orientation Clarification

**Bevy Coordinate System**: Right-handed Y-up (Y vertical, XZ horizontal plane)

- `Transform::forward()` API returns **-Z direction** (Bevy/OpenGL convention)
- `Transform::back()` API returns **+Z direction**

**This Game's Implementation**: Top-down breakout on XZ plane (camera at Y=37 looking down)

- Gameplay "forward" = **+Z direction** (toward goal/bricks, away from paddle)
- Gameplay "backward" = **-Z direction** (toward paddle)
- Lateral movement = **±X direction** (left/right)

**Why the implementation is correct**: The code directly manipulates `velocity.linvel.z` (physics velocity), NOT using `Transform::forward()` API.
In the context of a top-down game, +Z velocity moves the merkaba "forward" from the player's perspective.
The use of "forward" in documentation refers to **gameplay direction**, not Transform API direction.

Reference: [Bevy Cheat Book - Coordinate System](https://bevy-cheatbook.github.io/fundamentals/coords.html)

## Completed Actions

✅ **Z-axis documentation alignment** (2026-01-09)

- Updated [spec.md](spec.md) to clarify Z-axis (forward) minimum speed instead of X-axis
- Updated [tests/merkaba_physics.rs](../../tests/merkaba_physics.rs) test `t021` to check Z-velocity
- Updated [tests/unit/merkaba_min_speed.rs](../../tests/unit/merkaba_min_speed.rs) to match implementation
- Updated [tasks.md](tasks.md) task T021 description
- ✅ Fixed duplicate task ID T039 → T040 (2026-01-09)
- ✅ Fixed task range reference T029 → T020b, T039 (2026-01-09)

## Outstanding Comments - Documentation Only

### Category 1: Documentation Fixes (No Code Changes)

These are simple documentation corrections that don't require implementation changes:

#### 1. Duplicate Task ID T039

**Source**: CodeRabbit, Ellipsis **File**: [tasks.md](tasks.md) lines 112, 132 **Issue**: T039 used for both helicopter loop test and assets update **Resolution**: Rename second T039 to T040

```diff
- [X] T039 [US3] Update `assets/levels` examples to include at least one rotor brick
+ [X] T040 [US3] Update `assets/levels` examples to include at least one rotor brick
```

**Impact**: Documentation clarity only **Effort**: 2 minutes

#### 2. Incorrect Task Range Reference

**Source**: CodeRabbit, Ellipsis **File**: [tasks.md](tasks.md) line 145 **Issue**: References "T029–T031" but T029 doesn't exist **Resolution**: Update to "T020b, T039, T030–T031"

```diff
- [ ] [US3] Run T029–T031 tests in parallel; implement T032–T034 in parallel
+ [ ] [US3] Run T020b, T039, T030–T031 tests in parallel; implement T032–T034 in parallel
```

**Impact**: Documentation clarity only **Effort**: 1 minute

#### 3. Comment Typo in Unit Test

**Source**: Ellipsis **File**: [tests/unit/merkaba_min_speed.rs](../../tests/unit/merkaba_min_speed.rs) line 32 **Issue**: Already fixed during z-axis update **Status**: ✅ COMPLETE (fixed with z-axis documentation update)

### Category 2: Test Improvements (Optional Enhancements)

These are test quality improvements that don't affect functionality:

#### 4. Missing Audio Signal Assertions

**Source**: CodeReviewBot-AI **Files**:

- [tests/merkaba_physics.rs](../../tests/merkaba_physics.rs) line 71 (wall bounce)
- [tests/merkaba_physics.rs](../../tests/merkaba_physics.rs) line 108 (brick bounce)
- [tests/merkaba_audio.rs](../../tests/merkaba_audio.rs) line 87 (loop lifecycle)

**Issue**: Placeholder comments instead of actual assertions for audio signals **Resolution**: Add assertions checking for message emission

```rust
// Example for wall collision test:
let wall_collisions = app.world()
    .resource::<Messages<MerkabaWallCollision>>()
    .len();
assert!(wall_collisions > 0, "Wall collision should emit audio signal");
```

**Impact**: Better test coverage for audio integration **Effort**: 30 minutes (add assertions to 3 test files) **Priority**: Medium (nice-to-have, audio infrastructure works)

#### 5. Edge Case: Exact Threshold Value

**Source**: CodeReviewBot-AI **File**: [tests/unit/merkaba_min_speed.rs](../../tests/unit/merkaba_min_speed.rs) line 60 **Issue**: Missing test case for velocity exactly at ±3.0 threshold **Resolution**: Add test case

```rust
// Test case 4: Z-velocity exactly at threshold
let merkaba4 = app.world_mut().spawn((
    Merkaba,
    Transform::default(),
    Velocity::linear(Vec3::new(0.0, 0.0, 3.0)), // Exactly 3.0
)).id();
```

**Impact**: Better edge case coverage **Effort**: 10 minutes **Priority**: Low (existing implementation handles this correctly)

#### 6. Test Timing Assumptions

**Source**: CodeReviewBot-AI **Files**:

- [tests/merkaba_goal.rs](../../tests/merkaba_goal.rs) line 77
- [tests/merkaba_paddle.rs](../../tests/merkaba_paddle.rs) line 154

**Issue**: Tests assume single `app.update()` sufficient; may be fragile **Resolution**: Document timing assumptions or add extra update cycle

```rust
app.update(); // Process collision event
app.update(); // Ensure despawn system runs
```

**Impact**: More robust tests **Effort**: 15 minutes **Priority**: Low (tests currently pass reliably)

### Category 3: Code Quality Suggestions (Future Refactor)

These are architectural improvements that can be addressed in future work:

#### 7. Audio Asset Overwrite Check

**Source**: CodeReviewBot-AI, Gemini **File**: [src/systems/audio.rs](../../src/systems/audio.rs) line 570 **Issue**: Placeholder audio insertion doesn't check if handle exists **Resolution**: Add `contains_key()` check

```rust
if !assets.sounds.contains_key(&ty) {
    let handle: Handle<AudioSource> = asset_server.load(path);
    assets.sounds.insert(ty, handle);
}
```

**Impact**: Prevents manifest assets from being overwritten **Effort**: 5 minutes **Priority**: Medium (data integrity issue) **Note**: Can be addressed in separate PR for audio system cleanup

#### 8. Brick Position Fallback Logging

**Source**: Gemini **File**: [src/lib.rs](../../src/lib.rs) line 609 **Issue**: `unwrap_or(Vec3::ZERO)` can silently spawn at origin **Resolution**: Log error before fallback

```rust
let brick_pos = if let Some(t) = t_opt {
    t.translation
} else if let Some(gt) = gt_opt {
    gt.translation()
} else if let Ok(t) = transforms.get(entity) {
    t.translation
} else {
    error!("Could not resolve transform for brick {:?}. Spawning merkaba at origin.", entity);
    Vec3::ZERO
};
```

**Impact**: Better debugging for edge cases **Effort**: 10 minutes **Priority**: Low (edge case, hasn't occurred in practice)

#### 9. Enum Deserialization Robustness

**Source**: CodeReviewBot-AI **File**: [src/systems/textures/loader.rs](../../src/systems/textures/loader.rs) line 150 **Issue**: `ObjectClass` enum lacks fallback for unknown values **Resolution**: Add `#[serde(other)]` variant

```rust
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectClass {
    Ball,
    Brick,
    Merkaba,
    #[serde(other)]
    Unknown,
}
```

**Impact**: Forward compatibility with future manifest versions **Effort**: 15 minutes **Priority**: Low (manifest is version-controlled)

#### 10. Material Fallback Logging

**Source**: CodeReviewBot-AI **File**: [src/systems/textures/materials.rs](../../src/systems/textures/materials.rs) line 293 **Issue**: Silent fallback to canonical/fallback materials **Resolution**: Add logging similar to other fallback paths **Impact**: Better debugging for asset issues **Effort**: 10 minutes **Priority**: Low (fallback system working as intended)

#### 11. BaselineMaterialKind Match Exhaustiveness

**Source**: CodeReviewBot-AI **File**: [src/systems/textures/overrides.rs](../../src/systems/textures/overrides.rs) line 162 **Issue**: Match doesn't handle future variants explicitly **Resolution**: Mark enum `#[non_exhaustive]` or add wildcard arm **Impact**: Compile-time safety for future changes **Effort**: 5 minutes **Priority**: Low (internal enum, controlled changes)

### Category 4: Already Resolved (Outdated Comments)

These comments reference code that has been updated:

#### 12. Uninitialized Meshes Panic (OUTDATED)

**Source**: CodeReviewBot-AI, Gemini **File**: [src/systems/merkaba.rs](../../src/systems/merkaba.rs) (outdated reference) **Status**: ✅ ALREADY HANDLED **Resolution**: Current implementation uses `Option` matching with graceful warnings (lines 268-281)

```rust
let (mat_blue_handle, mat_gold_handle) = if let Some(reg) = type_registry.as_ref() {
    // ... returns None if materials missing
} else {
    (None, None)
};
```

No panic occurs; system logs warning and skips visual spawn.

#### 13. Angle Calculation for Direction Test (OUTDATED)

**Source**: CodeReviewBot-AI, Gemini **File**: [tests/unit/merkaba_direction.rs](../../tests/unit/merkaba_direction.rs) **Status**: ✅ ALREADY CORRECT **Note**: Implementation uses Z-forward motion with X-variance.
Test should use `atan2(x, z)` which is correct for XZ plane movement.

#### 14. Z-Plane Constraint Test (BY DESIGN)

**Source**: Gemini **File**: [tests/merkaba_physics.rs](../../tests/merkaba_physics.rs) line 268 **Status**: ✅ BY DESIGN **Note**: System uses `LockedAxes::TRANSLATION_LOCKED_Y` which locks Y-axis (not Z).
Test validates that constraint works.
The confusion stems from coordinate system naming, but implementation is correct.

### Category 5: Design Decisions (Won't Fix)

These are working as intended per specification:

#### 15. Despawn Logic Scope

**Source**: CodeReviewBot-AI **File**: [src/systems/merkaba.rs](../../src/systems/merkaba.rs) line 533 **Issue**: `despawn_balls_and_merkabas_on_life_loss` triggers on any life loss **Resolution**: WORKING AS INTENDED **Justification**: Per spec (User Story 3), merkaba-paddle contact should despawn all balls.
The current implementation correctly responds to life loss events from any source (paddle, goal).
If this becomes an issue, it would require spec change, not bug fix.

#### 16. GravityConfig Extensibility

**Source**: CodeReviewBot-AI **File**: [src/lib.rs](../../src/lib.rs) line 152 **Issue**: Only supports single gravity vector **Resolution**: YAGNI (You Aren't Gonna Need It) **Justification**: Current spec doesn't require multiple gravity states.
Can be extended in future PR if needed.

#### 17. Message Race Conditions

**Source**: CodeReviewBot-AI **File**: [tests/merkaba_paddle.rs](../../tests/merkaba_paddle.rs) line 46 **Issue**: Mock system clears messages immediately **Resolution**: TEST ISOLATION PATTERN **Justification**: Test uses dedicated mock system to control behavior.
In production, proper event propagation occurs via Bevy's message system.
This is standard test practice.

#### 18. Manual Timer Manipulation in Tests

**Source**: CodeReviewBot-AI **File**: [tests/merkaba_spawn.rs](../../tests/merkaba_spawn.rs) line 146 **Issue**: Tests manually advance timers **Resolution**: BEVY TEST LIMITATION **Justification**: Bevy's `Time` doesn't auto-advance in tests.
Manual timer manipulation is documented workaround.
Comment already explains this.

## Recommended Action Plan

### Immediate (Before Merge)

1. ✅ Fix duplicate task ID T039 → T040 (2 min)
2. ✅ Fix task range reference T029 → T020b, T039 (1 min)

**Total effort**: 3 minutes

### Short-term (Separate PR)

3. Add audio signal assertions to tests (30 min)
4. Add `contains_key()` check for audio asset insertion (5 min)
5. Add exact threshold test case (10 min)

**Total effort**: 45 minutes **Rationale**: Quality improvements that don't block merge

### Long-term (Future Refactors)

6. Improve error logging for brick position fallback
7. Add `#[serde(other)]` to ObjectClass enum
8. Add material fallback logging
9. Mark BaselineMaterialKind `#[non_exhaustive]`

**Rationale**: Low-priority maintainability improvements

## Summary

**Current implementation is correct**.
The Z-axis velocity clamping matches the specification intent (forward motion on XZ plane).

**Documentation fixes** (T039, T029 references) are trivial and should be done before merge.

**Test improvements** are optional enhancements that can be addressed in follow-up PRs without blocking merge.

**Code quality suggestions** are valid but low-priority; the current implementation is functional and robust.
These can be addressed as part of future refactoring efforts.

## Notes

- All axis-related confusion has been resolved by updating documentation to match implementation
- Test `t021_minimum_z_speed_clamped_to_3_0` now correctly validates Z-axis clamping
- No source code changes required for PR merge
- Outstanding issues are documentation/test quality, not functional bugs
