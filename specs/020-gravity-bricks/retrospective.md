# Retrospective: Gravity Brick Debugging Session

**Date**: January 2026 **Feature**: 020-gravity-bricks **Duration**: Extended debugging session (~4 cycles)

---

## Summary

Gravity brick destruction was supposed to change the game's gravity vector.
Logs showed gravity changes being applied, but they immediately reverted—ball physics remained unchanged.
The root cause was **three interacting bugs**, not one.

---

## Symptoms Observed

```text
[INFO] gravity changed: Vec3(10.0, 0.0, 0.0)  ← brick destroyed
... 16ms later ...
[INFO] gravity changed: Vec3(2.0, 0.0, 0.0)   ← reverted!
```

The 16ms interval was the key diagnostic: it corresponded to one frame at 60fps, indicating a per-frame system was overwriting the change.

---

## Bugs Found

### Bug 1: Paddle Growth Systems Used Wrong Resource

**Location**: `src/lib.rs` — `update_paddle_growth`, `restore_gravity_post_growth`

**Problem**: These systems reset gravity to `GravityConfig.normal` (a static config value) instead of respecting `GravityConfiguration.current` (runtime state).

**Fix**: Changed both systems to read from `GravityConfiguration.current`.

---

### Bug 2: Loader Ran Every Frame (Non-Idempotent)

**Location**: `src/systems/gravity.rs` — `gravity_configuration_loader_system`

**Problem**: The loader ran in the `Update` schedule and unconditionally wrote:

```rust
gravity_cfg.current = base;  // Executed EVERY FRAME
```

Any runtime gravity change was immediately overwritten on the next frame.

**Fix**: Added `last_level_number` tracking to `GravityConfiguration`.
The loader now only sets `current` when the level actually changes:

```rust
if gravity_cfg.last_level_number != Some(level.number) {
    gravity_cfg.current = base;
    gravity_cfg.last_level_number = Some(level.number);
}
```

---

### Bug 3: Duplicate BrickDestroyed Emissions

**Location**: `src/lib.rs` — `mark_brick_on_ball_collision`, `despawn_marked_entities`

**Problem**: `BrickDestroyed` could be emitted twice for the same entity:

1. Immediate path (rotor bricks with instant destruction)
2. Deferred path (despawn system)

Multiple emissions could cause conflicting `GravityChanged` messages.

**Fix**: Added `EmittedBrickDestroyed(HashSet<Entity>)` resource for per-frame deduplication.
Both emission sites check/insert before emitting.

---

## Why Tests Didn't Catch These Bugs

### Unit Test Limitations

| Test Pattern | Why It Missed the Bug |
|--------------|----------------------|
| **Isolated system tests** | Tested `gravity_application_system` in isolation. It worked correctly—the bug was in *other systems* interfering. |
| **Single-frame assertions** | Tests applied gravity and checked immediately. The loader reversion happened on the *next* frame. |
| **Mock resources** | Tests used clean `GravityConfiguration` without the loader system running alongside. |

### Integration Test Limitations

| Test Pattern | Why It Missed the Bug |
|--------------|----------------------|
| **Missing system registration** | Integration tests didn't always include `gravity_configuration_loader_system` in their app setup. |
| **No multi-frame simulation** | Tests didn't run `app.update()` multiple times after a gravity change to observe reversion. |
| **No paddle interaction** | Tests didn't exercise paddle growth during gravity brick gameplay. |

### The Core Problem

**Tests verified components worked correctly in isolation, but didn't verify they worked correctly *together* over multiple frames.**

The gravity application system was correct.
The loader system was correct (for its original purpose).
The paddle systems were correct (for their original purpose).
But when composed:

```text
Frame N:   BrickDestroyed → GravityChanged → current = Vec3(10,0,0) ✓
Frame N+1: Loader runs    → current = Vec3(2,0,0)                   ✗ (overwrite!)
```

---

## Test Gaps Identified

### Gap 1: No Multi-Frame Persistence Test

**Missing test**: "After changing gravity at runtime, it should persist across multiple frames."

```rust
#[test]
fn gravity_persists_across_frames() {
    // Setup app with ALL gravity-related systems
    // Change gravity via brick destruction
    // Run app.update() 10 times
    // Assert gravity is still the changed value
}
```

### Gap 2: No System Interaction Test

**Missing test**: "Gravity changes should persist even when paddle growth systems run."

```rust
#[test]
fn gravity_survives_paddle_growth_cycle() {
    // Change gravity
    // Trigger paddle growth (adds PaddleGrowth component)
    // Let growth complete (removes component, runs restore)
    // Assert gravity is still the changed value
}
```

### Gap 3: No Full Schedule Test

**Missing test**: "With all Update systems running, gravity changes persist."

The integration tests cherry-picked which systems to include.
A test with the *complete* Update schedule would have caught the loader overwrite.

---

## Fixes Applied

| File | Change |
|------|--------|
| `src/lib.rs` | Added `last_level_number` to `GravityConfiguration` |
| `src/lib.rs` | Changed paddle systems to use `GravityConfiguration` |
| `src/lib.rs` | Added `EmittedBrickDestroyed` dedupe resource |
| `src/systems/gravity.rs` | Made loader idempotent |

---

## Tests Added Post-Fix

| Test File | Purpose |
|-----------|---------|
| `tests/gravity_loader_idempotence.rs` | Verifies loader doesn't overwrite runtime changes |
| `tests/gravity_effect_integration.rs` | Verifies gravity affects ball velocity |
| `tests/gravity_playtest_sim.rs` | Simulates observed gameplay sequence |
| `tests/gravity_message_sequence.rs` | Verifies message ordering |
| `tests/brick_destroy_dedupe.rs` | Verifies single emission per entity |

---

## Lessons Learned

### 1. Test Multi-Frame Behavior

Single-frame tests miss timing bugs.
Any test for runtime state changes should:

- Apply the change
- Run multiple `app.update()` cycles
- Assert the change persists

### 2. Test with Full System Schedules

Cherry-picking systems for integration tests creates false confidence.
At least one integration test should run the *complete* schedule the production app uses.

### 3. Initialization Systems Need Guards

Systems that initialize state must either:

- Run in `Startup` (once), or
- Guard with "already initialized" checks

The loader pattern should have been:

```rust
fn loader_system(mut config: ResMut<Config>, level: Res<Level>) {
    if config.initialized_for_level == Some(level.number) {
        return;  // Already initialized, don't overwrite
    }
    // ... initialization logic ...
    config.initialized_for_level = Some(level.number);
}
```

### 4. Trace All Writers of Shared State

When debugging "value gets overwritten", the first question should be: **"What systems write to this resource, and when do they run?"**

A quick `grep` for `ResMut<GravityConfiguration>` would have revealed the loader and paddle systems immediately.

### 5. Add Logging Before Second Fix Attempt

First fix fails → add INFO logging → second attempt with visibility.

We delayed logging instrumentation, which cost investigation cycles.

---

## Process Improvements

| Before | After |
|--------|-------|
| Assume single root cause | Map all writers of affected state |
| Single-frame unit tests | Multi-frame persistence tests |
| Cherry-picked system tests | Full-schedule integration tests |
| Debug without logs | Add tracing on first failure |

---

## Conclusion

The bugs were individually simple but collectively obscured.
The tests didn't catch them because:

1. **Unit tests** verified isolated correctness, not composed behavior
2. **Integration tests** didn't include all interacting systems
3. **No tests** verified state persistence across multiple frames

The fix required understanding the *runtime system composition*, not just individual system logic.
Future gravity-related features should include multi-frame integration tests with the complete Update schedule.

---

## Application to Upcoming Feature: Modular Force Field System

The following lessons from this debugging session directly apply to the planned **Magnet Brick / Attractor System** feature.

### Feature Overview

A modular 3D force field system using Rapier3D with:

- **Strategy Pattern**: `ForceStrategy` trait for calculation logic
- **Generic struct**: `Attractor<S>` handling common properties and radius cutoff
- **6 force types**: Inverse-Distance, Rubber Band, Whirling, Linear Field, Directional Field, Angular Field

### Implementation Hints Based on Retrospective

#### 1. Force Application Must Be Additive, Not Overwriting

**Risk**: Multiple attractors applying forces to the same ball could overwrite each other if implemented naively.

**Pattern to use**:

```rust
// ❌ WRONG: Overwrites previous forces
ball_velocity.linvel = calculated_force;

// ✅ CORRECT: Accumulates forces
ball_velocity.linvel += calculated_force * time.delta_secs();
// Or use Rapier's ExternalForce component for proper accumulation
```

**Test requirement**: Multi-attractor test where two attractors affect the same ball simultaneously.

#### 2. Attractor State Must Not Be Clobbered by Loaders

**Risk**: If `Attractor<S>` has runtime-modifiable state (e.g., strength multiplier from powerups), a loader system could reset it every frame.

**Pattern to use** (from gravity fix):

```rust
pub struct AttractorConfiguration {
    pub base_strength: f32,      // From level definition
    pub current_strength: f32,   // Runtime value (may differ)
    pub last_level_number: Option<u32>,  // Idempotence guard
}

fn attractor_loader_system(mut config: ResMut<AttractorConfiguration>, level: Res<CurrentLevel>) {
    if config.last_level_number == Some(level.0.number) {
        return;  // Don't overwrite runtime changes
    }
    config.current_strength = config.base_strength;
    config.last_level_number = Some(level.0.number);
}
```

#### 3. Strategy Trait Must Be Pure (No Side Effects)

**Risk**: If `ForceStrategy::calculate()` modifies state, testing becomes non-deterministic and bugs become harder to trace.

**Pattern to use**:

```rust
pub trait ForceStrategy: Send + Sync {
    /// Calculate force based on relative position. MUST be pure.
    ///
    /// # Arguments
    /// * `displacement` - Vector from attractor center to ball
    /// * `distance` - Magnitude of displacement (pre-computed for efficiency)
    /// * `params` - Read-only strategy parameters
    fn calculate(&self, displacement: Vec3, distance: f32) -> Vec3;
}
```

**Test requirement**: Same inputs → same outputs, verified across multiple calls.

#### 4. Multi-Frame Persistence Tests Are Mandatory

**Risk**: Force accumulation bugs won't appear in single-frame tests.

**Required test pattern**:

```rust
#[test]
fn attractor_force_accumulates_over_frames() {
    let mut app = setup_app_with_all_physics_systems();
    spawn_ball_and_attractor(&mut app);

    let initial_velocity = get_ball_velocity(&app);

    // Run 60 frames (1 second at 60fps)
    for frame in 0..60 {
        app.update();
        let velocity = get_ball_velocity(&app);
        // Velocity should change progressively, not reset
        assert_ne!(velocity, initial_velocity, "Frame {}: force not applied", frame);
    }

    // Ball should have moved significantly toward attractor
    let final_position = get_ball_position(&app);
    assert!(final_position.distance(attractor_position) < initial_distance);
}
```

#### 5. Test Full System Composition, Not Just Strategies

**Lesson**: Testing `InverseDistanceStrategy::calculate()` in isolation won't catch bugs where other systems interfere.

**Required test structure**:

```rust
#[test]
fn attractor_works_with_full_update_schedule() {
    let mut app = App::new();

    // Register ALL production systems, not cherry-picked ones
    app.add_plugins(MinimalPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_plugins(AttractorPlugin);  // Must include loader, applier, cleanup
    app.add_plugins(PaddleSizePlugin); // Other systems that might interfere
    app.add_plugins(GravityPlugin);    // Gravity also affects ball

    // ... test attractor behavior ...
}
```

### Pitfalls to Avoid

| Pitfall | Why It's Dangerous | Prevention |
|---------|-------------------|------------|
| **Loader in Update without guard** | Resets attractor config every frame, nullifying runtime changes | Use `last_level_number` pattern or move to `OnEnter(GameState)` |
| **Direct velocity assignment** | Multiple attractors overwrite each other | Use `ExternalForce` component or additive velocity changes |
| **Testing strategies in isolation only** | Misses system composition bugs | Include at least one "full schedule" integration test |
| **Single-frame force tests** | Misses accumulation/integration bugs | Run 10+ frames after force application |
| **Mutable state in ForceStrategy** | Non-deterministic behavior, hard to debug | Make `calculate()` pure; state lives in `Attractor<S>` |
| **Per-entity force without cleanup** | Forces persist after attractor despawned | Clear `ExternalForce` when attractor despawns |

### Recommended Architecture

```text
┌─────────────────────────────────────────────────────────────┐
│                    AttractorPlugin                          │
├─────────────────────────────────────────────────────────────┤
│  Systems (in Update schedule):                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ 1. attractor_loader_system (idempotent)             │   │
│  │    - Only initializes on level change               │   │
│  ├─────────────────────────────────────────────────────┤   │
│  │ 2. attractor_force_application_system               │   │
│  │    - Queries all Attractor<S> components            │   │
│  │    - For each ball in range, accumulates force      │   │
│  │    - Writes to ExternalForce component (additive)   │   │
│  ├─────────────────────────────────────────────────────┤   │
│  │ 3. attractor_cleanup_system                         │   │
│  │    - On attractor despawn, clear residual forces    │   │
│  └─────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│  Components:                                                │
│  - Attractor<S: ForceStrategy>  (generic over strategy)    │
│  - AttractorActive              (marker for enabled state)  │
│  - AttractorAffected            (on ball, tracks which      │
│                                  attractors are affecting)  │
├─────────────────────────────────────────────────────────────┤
│  Resources:                                                 │
│  - AttractorConfiguration       (global settings, tunable) │
│  - AttractorConstants           (strength values, radii)   │
└─────────────────────────────────────────────────────────────┘
```

### Force Strategy Constants (Following Gravity Pattern)

```rust
// In src/systems/attractor.rs

// ============= ATTRACTOR CONSTANTS =============
// Adjust these for gameplay tuning without modifying logic.

/// Default radius for inverse-distance attractors (magnet brick)
pub const ATTRACTOR_RADIUS_DEFAULT: f32 = 5.0;

/// Strength multiplier for inverse-distance force (F = strength / distance)
pub const INVERSE_DISTANCE_STRENGTH: f32 = 10.0;

/// Strength multiplier for rubber band force (F = strength * distance)
pub const RUBBER_BAND_STRENGTH: f32 = 2.0;

/// Maximum rubber band force (prevents infinite force at edge)
pub const RUBBER_BAND_MAX_FORCE: f32 = 50.0;

/// Whirling vortex strength (perpendicular force magnitude)
pub const WHIRLING_STRENGTH: f32 = 8.0;

/// Linear field constant force magnitude
pub const LINEAR_FIELD_FORCE: f32 = 5.0;

/// Angular cutoff for directional field (cone half-angle in radians)
pub const DIRECTIONAL_CONE_ANGLE: f32 = 0.5236; // 30 degrees

/// Angular field parallel component strength
pub const ANGULAR_PARALLEL_STRENGTH: f32 = 6.0;
/// Angular field perpendicular component strength
pub const ANGULAR_PERPENDICULAR_STRENGTH: f32 = 4.0;
```

### Test Checklist for Force Field Feature

Based on retrospective lessons, these tests are **mandatory** before merge:

- [ ] **Multi-frame persistence**: Force applied persists across 10+ frames
- [ ] **Multi-attractor composition**: Two attractors affect ball simultaneously
- [ ] **Attractor + gravity interaction**: Both systems affect ball without conflict
- [ ] **Loader idempotence**: Runtime attractor changes survive level reload
- [ ] **Full schedule integration**: Test with all production systems registered
- [ ] **Attractor despawn cleanup**: Forces clear when attractor brick destroyed
- [ ] **Strategy purity**: Same inputs produce same outputs (fuzz test)
- [ ] **Radius cutoff**: Ball outside radius receives zero force
