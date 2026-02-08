# Implementation Plan: Direction Bricks

**Branch**: `026-direction-bricks` | **Date**: 2026-02-01 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/026-direction-bricks/spec.md`

## Summary

Implement seven new destructible brick types (43-48, 52) that manipulate ball velocity direction and magnitude during gameplay:

**Directional Impulse Bricks (43-46)**: Apply instantaneous 5.0 units/sec velocity impulse in cardinal directions:

- Brick 43 (Forward): Increase X-velocity by 5.0 units/sec (toward far wall)
- Brick 44 (Left): Increase Z-velocity by 5.0 units/sec
- Brick 45 (Right): Decrease Z-velocity by 5.0 units/sec
- Brick 46 (Backward): Decrease X-velocity by 5.0 units/sec (toward paddle)

**Diagonal Impulse Bricks (47-48)**: Apply simultaneous velocity impulses along two axes:

- Brick 47 (Backward-Right): Decrease X and Z velocity by 5.0 units/sec each
- Brick 48 (Backward-Left): Decrease X and increase Z velocity by 5.0 units/sec each

**Randomization Brick (52)**: Replace ball velocity with random direction and magnitude:

- Brick 52 (Randomizer): Generate random direction (0-360°) and magnitude (5.0-15.0 units/sec)

All bricks award points (75 for 43-46, 100 for 47-48, 125 for 52) and emit tracing spans for observability.
Velocity impulses are applied once per brick destruction event via Observers pattern.
This feature extends existing ball physics mechanics with directional control options for level designers.

## Technical Context

**Language/Version**: Rust 1.81 (2021 edition), Bevy 0.17.3

**Primary Dependencies**: bevy_rapier3d 0.32.0 (physics, LinearVelocity component), tracing 0.1 (structured logging), rand 0.8 (randomization for brick 52)

**Storage**: In-memory ECS state only; levels persisted as RON files in `assets/levels/`

**Testing**: `cargo test` with unit/integration tests committed before implementation (TDD); multi-frame persistence tests required per 020-gravity-bricks retrospective

**Target Platform**: Native (Linux/Windows/macOS) + WASM `wasm32-unknown-unknown` (cross-platform)

**Project Type**: Single binary game built with Bevy + Rapier3D

**Performance Goals**: 60 FPS consistent on target hardware (native: desktop, WASM: Chrome/Firefox on moderate hardware)

**Constraints**: Physics-driven gameplay via Rapier3D; XZ horizontal plane with Y-axis locked for balls; coordinate system: XZ for horizontal, Y for vertical; direction bricks modify velocity only (no continuous forces)

**Scale/Scope**: Seven brick types; modular feature design; integrates with existing brick destruction, physics, scoring, and observability systems; ~1-2 KB compiled WASM overhead

## Constitution Check

**GATE STATUS**: ✅ **PASS** - Feature complies with all constitutional mandates.

### TDD Requirements

- ✅ Tests will be defined and committed prior to implementation (acceptance scenarios in spec serve as test specifications)
- ✅ Red-phase commit required: failing tests committed before implementation begins
- ✅ Tests require approval from feature owner before implementation proceeds
- ✅ Multi-frame persistence testing required (minimum 10 frames per 020-gravity-bricks retrospective mandate)

### Bevy 0.17 Mandates Compliance

**Feature Scope**: This feature touches ECS systems, brick entity spawning, messaging/observers, physics (LinearVelocity modification), and tracing.
Full Bevy 0.17 compliance required.

#### 1. Event System Choice ✅

**Decision**: Use **Observers with `Trigger<DirectionBrickEffect>`** for applying direction brick velocity effects.

**Rationale**:

- Direction brick effects are immediate, reactive logic triggered by brick destruction event
- Observers provide per-entity reactivity: each ball entity receives velocity modification independently
- Aligns with Bevy 0.17 best practices for reactive, deterministic entity-level actions
- Existing codebase uses Observers for reactive mechanics (see `src/systems/observers.rs` for collision-based triggers)
- Scoring system already reads `BrickDestroyed` messages (buffered), direction effect is separate Observer trigger for immediate velocity modification
- Allows emission of tracing spans with per-entity context (ball ID, velocity delta)

**System Architecture**:

1. Brick destruction system emits `BrickDestroyed` message (existing) → read by scoring system (existing)
2. Same brick destruction system also triggers `Trigger<DirectionBrickDestroyed>` observer event (new)
3. Observer system listening to `Trigger<DirectionBrickDestroyed>` applies velocity impulse to ball entity (new)
4. Observer system emits tracing span with modification details (new)

**Observers NOT used for**: Ball spawning (not in this feature), scoring (buffered message pattern is appropriate)

#### 2. Coordinate System ✅

**Axes for Movement**:

- **XZ horizontal plane**: Ball movement on game surface (top-down camera view)
- **Y-axis**: Height/vertical movement (typically locked via `LockedAxes::TRANSLATION_LOCKED_Y` for balls, but Y-velocity can change)
- **Direction brick effects**: Modify X and Y components of `LinearVelocity` only; Z-axis untouched per edge case specification

**Directional Terminology**:

- "Velocity impulse" for bricks 43-48: Instantaneous change to `Velocity.linvel` components (addition/subtraction of 5.0 units/sec)
  - Brick 43 (Forward): `velocity.linvel.x += 5.0`
  - Brick 44 (Left): `velocity.linvel.z += 5.0`
  - Brick 45 (Right): `velocity.linvel.z -= 5.0`
  - Brick 46 (Backward): `velocity.linvel.x -= 5.0`
  - Brick 47 (Backward-Right): `velocity.linvel.x -= 5.0; velocity.linvel.z -= 5.0`
  - Brick 48 (Backward-Left): `velocity.linvel.x -= 5.0; velocity.linvel.z += 5.0`
- "Randomized velocity" for brick 52: Generate random 2D direction in XZ plane (horizontal gameplay), magnitude 5.0-15.0 units/sec
  - Direction angle generated in **radians** (0.0..TAU = 0.0..2π), NOT degrees (0.0..360.0)
  - Implementation: `angle = rng.random_range(0.0..std::f32::consts::TAU)`; convert to X/Z components via `Vec3::new(mag * cos(angle), 0.0, mag * sin(angle))`
- Uses **XZ plane only** (Y always 0) to match horizontal gameplay surface; vertical axis unused

**Camera Context**: Angled view (camera above and behind paddle looking forward) means:

- Gameplay "forward" (toward bricks, away from paddle) = **+Z direction**
- Gameplay "backward" (toward paddle) = **-Z direction**
- Gameplay "left" = **-X direction**
- Gameplay "right" = **+X direction**
- Y-axis vertical (up = +Y, down = -Y) in 3D space

**Important**: This gameplay convention (+Z = forward) differs from Bevy's `Transform::forward()` API, which returns -Z per OpenGL convention (per Constitution Principle VIII).
**Direction brick code uses direct axis manipulation** (`velocity.linvel.x`, `velocity.linvel.y`) rather than Transform methods, so this convention applies directly without confusion.

**Edge Case - Z-Axis Independence**: Direction bricks only modify X and Y velocity.
Z-velocity persists unchanged.
This is intentional per specification edge case: "Cardinal and diagonal bricks only modify X and Y; Z-velocity is unchanged."

#### 3. Initialization System Idempotence ✅

**Applicability**: NOT applicable to this feature.

**Justification**: Direction brick velocity impulses are not initialization/loader systems.
They occur on-demand during gameplay (when bricks are destroyed via collision), not in Update schedule initialization phase.
No guard fields or context-change detection needed.
Impulses are applied once per `Trigger<DirectionBrickDestroyed>` event, which fires exactly once per brick destruction.

#### 4. Multi-Frame Persistence Testing ✅

**Required**: Yes.
This feature modifies runtime state (ball `LinearVelocity`).

**Test Requirements**:

- Tests MUST verify velocity modifications persist across multiple `app.update()` cycles (minimum 10 frames)
- Tests MUST include ALL systems that write to ball velocity:
  - Direction brick observer system (applies impulse)
  - Physics system (applies forces, gravity; updates velocity each frame)
  - Collision system (may trigger additional brick hits)
- Persistence check: Ball velocity after impulse must persist; subsequent physics integration must apply on modified velocity (not reset it)
- Velocity accumulation check: Multiple direction bricks hit in sequence must have stacking effects (Acceptance Scenario 1.6)
- Randomization check: Brick 52 randomized velocity must persist and only be overwritten by next brick destruction (not reset per frame)

**Acceptance Scenarios** (from spec) explicitly include multi-frame checks:

- Scenario 1.6: "Given multiple direction bricks in sequence, When the ball destroys consecutive bricks, Then velocity modifications stack correctly across frames without being overwritten"
- Scenario 3.2: "Given brick 52 is destroyed multiple times, When observing the resulting ball velocities, Then each destruction produces a statistically different random velocity (not deterministic)"
- Success Criterion SC-003: "Velocity modifications persist correctly across multiple game frames (minimum 10 update cycles) without being overwritten or reset"

#### 5. System Design & Query Safety ✅

**Fallible Systems**:

- Observer system will use early returns and `.warn_if_error()` patterns, not `.unwrap()` on queries
- Queries will use `With<>` and `Without<>` filters to target only relevant entities

**Query Specificity**:

- Ball query: `Query<&mut LinearVelocity, With<Ball>>` (only balls, filtered to avoid non-physics entities)
- Brick query: Not needed in direction brick observer; brick entity is passed via trigger context

**Change Detection**: Not needed for this feature (direction observer doesn't watch for changes, it reacts to trigger event).

#### 6. Message-Event Separation ✅

**Verified**: Two-system pattern with correct separation.

- **System 1 (Existing)**: Brick destruction reads collision/trigger event, emits `BrickDestroyed` message via `MessageWriter<BrickDestroyed>`
- **System 2 (Existing)**: Scoring system reads `MessageReader<BrickDestroyed>`, updates score (buffered message pattern appropriate for cross-frame batching)
- **System 3 (New)**: Observer listening to `Trigger<DirectionBrickDestroyed>` applies velocity impulse (observer pattern for immediate, per-entity reaction)
- **System 4 (New)**: Same observer emits tracing span for observability

**NOT using Messages for velocity impulse** because: Velocity modification is immediate, per-entity logic tied to specific ball entity, not a batched, frame-agnostic data stream.

#### 7. Hierarchy Safety ✅

**Decision**: Direction brick impulses operate on ball `LinearVelocity` component directly; no parent-child relationships created.

**Rationale**:

- Balls are independent physics entities with no parent-child dependencies
- No need for `ChildOf`, `Parent`, `Children` components
- Direction bricks don't spawn new entities; they modify existing ball entity state

#### 8. Asset Management ✅

**Visual Assets**: Bricks 43-48, 52 textures already exist in asset system (referenced in existing brick documentation).

**Implementation**: Reuse existing texture handles; no new assets needed.

---

### Complexity Tracking

No constitutional violations.
Feature design is straightforward and aligns with existing patterns in codebase.

| Area | Status | Notes |
|------|--------|-------|
| ECS Architecture | ✅ | Observer-driven velocity modification per entity |
| Physics-Driven Gameplay | ✅ | Impulses applied to `LinearVelocity` via Rapier3D |
| TDD Compliance | ✅ | Tests committed before implementation; red-phase required |
| Bevy 0.17 Mandates | ✅ | Observers for immediate reaction; no initialization issues; multi-frame testing required |
| Coordinate System Clarity | ✅ | XZ horizontal, Y vertical; direct axis references |
| Query Safety | ✅ | Fallible queries with filters |
| Message-Event Separation | ✅ | Observers for immediate logic, Messages for buffered signals |
| No Undeferred Overwrites | ✅ | No per-frame unconditional velocity resets |

## Project Structure

### Documentation (this feature)

```text
specs/026-direction-bricks/
├── spec.md              # Feature specification (existing)
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 research (placeholder - may be empty if no unknowns)
├── data-model.md        # Phase 1 data model output
├── quickstart.md        # Phase 1 quickstart for developers
├── contracts/           # Phase 1 API contracts (empty if no external APIs)
├── checklists/
│   └── requirements.md   # Specification quality checklist (existing)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── level_format/        # Level file loader (uses existing brick type enum)
├── systems/
│   ├── brick_effects.rs # NEW: Observer system for direction brick impulses + tracing
│   ├── physics_config.rs # EXISTING: Physics system
│   ├── scoring.rs       # EXISTING: Scoring system (reads BrickDestroyed messages)
│   └── [other systems]
├── signals.rs           # EXISTING: Message types including BrickDestroyed
└── lib.rs               # EXISTING: Module organization

assets/
├── levels/              # EXISTING: Level RON files (can now include bricks 43-48, 52)
└── textures/            # EXISTING: Brick textures (already include 43-48, 52)

tests/
├── direction_bricks.rs  # NEW: Unit/integration tests for direction brick effects
├── multi_frame_persistence_direction_bricks.rs # NEW: Multi-frame persistence tests
└── [other tests]
```

## Implementation Phases

### Phase 1: Core Direction Brick System

**Deliverables**:

1. Direction brick observer system (`src/systems/brick_effects.rs`)
   - Observer listening to `Trigger<DirectionBrickDestroyed>`
   - Applies velocity impulses (bricks 43-48)
   - Emits tracing spans with brick ID, velocity before/after, points awarded

2. Randomization brick system (`src/systems/brick_effects.rs` or separate module)
   - Generates random velocity for brick 52 using `rand` crate
   - Magnitude directly in 5.0-15.0 range (no clamping)
   - Direction uniformly distributed 0-360°
   - Emits tracing spans

3. Integration with existing brick destruction system
   - Trigger `Trigger<DirectionBrickDestroyed>` when direction brick is destroyed
   - Ensure correct brick type detection (IDs 43-48, 52)
   - Verify scoring system still receives `BrickDestroyed` messages

4. Tracing instrumentation (FR-010)
   - Use `tracing::info_span!()` for structured logging
   - Log: brick ID, ball entity ID, velocity before/after, points awarded
   - Visible in test output and production gameplay logs

5. Multi-frame persistence tests
   - Test velocity modifications persist 10+ frames
   - Test velocity stacking (multiple bricks in sequence)
   - Test randomization non-determinism
   - Verify physics system applies correctly to modified velocities

### Phase 2: Level Design & Testing

**Deliverables**:

1. Test level files
   - Populate `assets/levels/` with test levels containing bricks 43-48, 52
   - Test single bricks, combinations, and edge cases

2. Acceptance test suite (`tests/direction_bricks.rs`)
   - All user story acceptance scenarios from spec
   - Edge case tests (stationary ball, rapid succession, Z-axis independence)
   - Scoring verification

3. Integration testing
   - Verify direction bricks work with existing game systems (gravity, multi-ball, etc.)
   - Verify no regression in existing brick types

### Phase 3: Documentation & Review

**Deliverables**:

1. Inline code documentation
   - Module-level docs explaining observer pattern and impulse mechanics
   - Function docs for each brick type's impulse calculation

2. Developer guide update (`docs/developer-guide.md`)
   - How to add new direction brick types
   - Impulse calculation formulas
   - Tracing instrumentation guide

## Key Design Decisions

### 1. Observer Pattern for Impulse Application

**Why Observers instead of Direct Brick System Call?**

- Observers provide reactive, immediate logic - impulse is applied as soon as brick is destroyed
- Allows per-entity context (ball ID, velocity before/after)
- Enables tracing spans with full context
- Decouples direction brick logic from collision/brick destruction system
- Aligns with Bevy 0.17 best practices for immediate reactions

### 2. Instantaneous Impulses vs. Continuous Acceleration

**Why Not Per-Frame Acceleration?**

- Specification clarification Q1 established impulses are instantaneous (once per destruction)
- Avoids per-frame accumulation bugs (physics system already applies forces per frame)
- Simpler semantics: "Hit this brick, get this velocity boost" vs.
  "Hit this brick, experience 5.0 units/sec² for how long?"
- Matches game design intent: direction bricks provide immediate, discrete velocity modifications

### 3. RNG Generation in 5.0-15.0 Range

**Why Generate Directly Instead of Clamping?**

- Specification clarification Q5 established direct generation in range
- Eliminates edge case of zero or low-magnitude velocity
- More efficient: one RNG call instead of retry loop
- Clearer intent in code

### 4. Tracing Over Debug Logging

**Why Structured Tracing?**

- Specification clarification Q2 established tracing spans as observability approach
- Provides structured data (brick ID, velocities) vs. unstructured log lines
- Integrates with project's existing `tracing` crate infrastructure
- Enables filtering and inspection in test output

### 5. Observers for Direction Effects, Messages for Destruction Signal

**Two-System Pattern Rationale**:

- `BrickDestroyed` message is buffered signal: batches multiple destructions, read by scoring system
- `DirectionBrickDestroyed` observer is reactive signal: immediate per-entity velocity modification
- Separation of concerns: scoring logic (buffered) vs. physics logic (immediate)
- Constitution mandates correct separation: Messages for frame-agnostic work, Observers for immediate reactions

## Risk Analysis & Mitigation

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Velocity modifications overwritten by physics system | High | Multi-frame persistence tests verify no overwriting; physics system is read-only in Update (no unconditional resets) |
| RNG determinism affects gameplay consistency | Medium | Tests verify different velocities across multiple brick 52 destructions; RNG is deterministic per seed (no issue in deterministic tests) |
| Tracing overhead in release builds | Low | Tracing crate compile-time filtering; no-op spans in release if tracing disabled |
| Z-axis velocity unmodified causes confusion | Low | Specification and edge case documentation clarify Z-axis independence |
| Observer trigger order vs. scoring message order | Low | Observers run before or after Messages in same schedule step; order doesn't matter since scoring reads messages independently |

## Dependencies & Integration Points

### New Crates

- `rand` (already in `Cargo.toml` per project dependencies)

### Modified Modules

- `src/systems/brick_effects.rs` (NEW): Observer system for direction brick impulses
- `src/signals.rs` (EXTEND): Add `DirectionBrickEffect` trigger type
- `src/systems/mod.rs` (EXTEND): Register new observer system
- `Cargo.toml` (NO CHANGE): `rand` already present

### No Changes Required

- `src/level_format/` (loads bricks by ID; handles new IDs automatically)
- `src/systems/scoring.rs` (reads existing `BrickDestroyed` messages)
- Physics system (applies impulse-modified velocity; no changes)
- Asset system (brick textures already exist for 43-48, 52)

## Testing Strategy

### Unit Tests (in `tests/direction_bricks.rs`)

```text
✓ test_brick_43_down_impulse: Verify Y-velocity decreases by 5.0
✓ test_brick_44_left_impulse: Verify X-velocity decreases by 5.0
✓ test_brick_45_right_impulse: Verify X-velocity increases by 5.0
✓ test_brick_46_up_impulse: Verify Y-velocity increases by 5.0
✓ test_brick_47_up_right_impulse: Verify both X and Y increase by 5.0
✓ test_brick_48_up_left_impulse: Verify X decreases, Y increases by 5.0
✓ test_brick_52_randomizer_magnitude: Verify random magnitude in 5.0-15.0 range
✓ test_brick_52_randomizer_direction: Verify uniform distribution across 360°
✓ test_stationary_ball_receives_impulse: Ball at rest gets velocity from impulse
✓ test_z_velocity_unchanged: Impulses don't affect Z-axis
✓ test_scoring: Each brick awards correct points (75, 100, or 125)
✓ test_tracing_spans: Verify tracing spans emitted with correct context
```

### Multi-Frame Persistence Tests (in `tests/multi_frame_persistence_direction_bricks.rs`)

```text
✓ test_velocity_persists_10_frames: Impulse-modified velocity persists 10+ frames
✓ test_velocity_accumulation: Multiple brick hits stack correctly across frames
✓ test_physics_applied_after_impulse: Physics system modifies impulse-modified velocity correctly
✓ test_randomizer_persists: Brick 52 random velocity persists and isn't reset per frame
```

### Integration Tests (in `tests/direction_bricks.rs`)

```text
✓ test_with_gravity_bricks: Direction bricks work with gravity brick effects
✓ test_with_multi_ball: Direction bricks affect all balls in multi-ball mode
✓ test_level_loading: Levels with bricks 43-48, 52 load and parse correctly
✓ test_no_regression: All existing brick types still function correctly
```

## Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| All acceptance scenarios pass | 100% | Test run passes all scenarios from spec |
| Multi-frame persistence verified | 10+ frames | Tests run 10+ app.update() cycles; velocity persists |
| Scoring accuracy | 100% | All 7 brick types award correct points in tests |
| Tracing coverage | 100% | All direction brick destructions emit tracing spans |
| No regression | 100% | Existing tests pass; no broken brick types |
| Performance (FPS) | 60 FPS | Game maintains 60 FPS with direction bricks in level |

## Next Steps

1. **Red Phase (TDD)**: Write failing tests covering all acceptance scenarios and edge cases
2. **Green Phase**: Implement observer system, impulse calculation, randomization, and scoring integration
3. **Refactor Phase**: Optimize tracing spans, add inline documentation, clean up code
4. **Integration Testing**: Verify with gravity, multi-ball, and level loading
5. **Acceptance Review**: Validate against all acceptance scenarios and success criteria
