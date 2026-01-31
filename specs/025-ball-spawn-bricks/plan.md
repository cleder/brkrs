# Implementation Plan: Ball Spawn Bricks

**Branch**: `025-ball-spawn-bricks` | **Date**: 2026-01-31 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/025-ball-spawn-bricks/spec.md`

## Summary

Implement three new destructible brick types (Red 1, Red 2, Red 3 at indices 37, 38, 39) that manipulate ball quantity during gameplay:

- **Red 2 (index 38)**: Spawns one additional ball with inverse velocity when hit
- **Red 3 (index 39)**: Spawns two additional balls in Y-shaped spread when hit
- **Red 1 (index 37)**: Despawns all balls except the triggering ball when hit

All three bricks award 100 points and count toward level completion.
Ball spawning occurs at the brick's XZ position.
This feature extends the existing multi-ball gameplay mechanics and provides both offensive (spawn) and defensive (reset) options for players.

## Technical Context

**Language/Version**: Rust 1.81 (2021 edition), Bevy 0.17.3 **Primary Dependencies**: bevy_rapier3d 0.32.0 (physics), serde 1.0 + ron 0.8 (level loading), tracing 0.1 (logging) **Storage**: In-memory ECS state only; levels persisted as RON files in `assets/levels/` **Testing**: `cargo test` with unit/integration tests committed before implementation (TDD) **Target Platform**: Native (Linux/Windows/macOS) + WASM `wasm32-unknown-unknown` (cross-platform) **Project Type**: Single binary game built with Bevy + Rapier3D **Performance Goals**: 60 FPS consistent on target hardware (native: desktop, WASM: Chrome/Firefox on moderate hardware) **Constraints**: Physics-driven gameplay via Rapier3D; top-down XZ plane with Y-axis locked for balls; coordinate system: XZ for horizontal, Y for layering **Scale/Scope**: Three brick types; modular feature design; integrates with existing brick destruction, physics, and scoring systems; ~2-3 KB compiled WASM overhead

## Constitution Check

**GATE STATUS**: ✅ **PASS** - Feature complies with all constitutional mandates.

### TDD Requirements

- ✅ Tests will be defined and committed prior to implementation (acceptance scenarios in spec serve as test specifications)
- ✅ Red-phase commit required: failing tests committed before implementation begins
- ✅ Tests require approval from feature owner before implementation proceeds

### Bevy 0.17 Mandates Compliance

**Feature Scope**: This feature touches ECS systems, entity spawning/despawning, messaging, and physics.
Full Bevy 0.17 compliance required.

#### 1. Event System Choice ✅

**Decision**: Use **Messages** (`MessageWriter<BrickDestroyed>` / `MessageReader<BrickDestroyed>`) for ball spawn/despawn logic.

**Rationale**:

- Ball spawning is a bufferable, frame-agnostic operation (can occur whenever a brick is destroyed, not requiring immediate reaction)
- Existing codebase uses Messages for `BrickDestroyed` signal (see `crate::signals::BrickDestroyed`)
- Allows batching of multiple ball spawns if multiple bricks destroyed in same frame
- Scoring system already reads `BrickDestroyed` messages via `MessageReader<BrickDestroyed>` (see `src/systems/scoring.rs`)
- Spawned balls will immediately interact with physics on next frame, no need for same-frame reaction

**Observers NOT used because**: Ball spawning is not immediate reactive logic (UI/sound); it's predictable state mutation that can be safely buffered.

#### 2. Coordinate System ✅

**Axes for Movement**:

- **XZ horizontal plane**: Ball movement on game surface (top-down camera view)
- **Y-axis**: Height/layering (typically locked via `LockedAxes::TRANSLATION_LOCKED_Y` for balls)
- **Spawning position**: Brick's XZ position + Y position inherited from brick

**Directional Terminology**:

- "Inverse direction" for Red 2 brick: Negate the velocity vector (if `v = (vx, vy, vz)`, then `-v = (-vx, -vy, -vz)`)
- "Y-shaped spread" for Red 3 brick: Two spawned balls at ±37.5 degrees from original trajectory in the XZ plane
  - Calculation: For velocity `v = (vx, vy, vz)`, compute 2D angle `θ = atan2(vz, vx)`, then left velocity at `θ + 37.5°` and right velocity at `θ - 37.5°` with same magnitude `sqrt(vx² + vz²)`
- Uses **direct axis manipulation** (`linvel.x`, `linvel.z`) rather than semantic "forward/backward" to avoid Transform API confusion

**Camera Context**: Top-down view (camera at positive Y looking down) means:

- Gameplay "forward" (+Z direction) appears upward on screen
- Gameplay "backward" (-Z direction) appears downward on screen
- Paddle is at -Z end of playfield, bricks at +Z end

#### 3. Initialization System Idempotence ✅

**Applicability**: NOT applicable to this feature.

**Justification**: Ball spawning is not an initialization/loader system.
It occurs on-demand during gameplay (when bricks are destroyed), not in Update schedule initialization.
No guard fields or context-change detection needed.

#### 4. Multi-Frame Persistence Testing ✅

**Required**: Yes.
This feature modifies runtime state (spawns/despawns ball entities).

**Test Requirements**:

- Tests MUST verify spawned balls persist across multiple `app.update()` cycles (minimum 10 frames)
- Tests MUST include ALL systems that write to ball state:
  - Ball spawning system (adds ball entities with velocity)
  - Physics system (updates velocity each frame)
  - Collision system (may trigger additional brick hits)
  - Ball despawning system (removes ball entities on Red 1 trigger)
- Persistence check: Spawned ball's position/velocity must change frame-to-frame (physics applied), not be reset or lost
- Despawn check: Despawned balls must remain gone (not respawned) across 10+ frames

**Acceptance Scenarios** (from spec) already include multi-frame checks:

- Scenario 1.4: "When 10 update cycles pass, the spawned ball persists and continues moving according to physics"
- Scenario 2.4: "When 10 update cycles pass, both spawned balls persist and move independently"
- Scenario 3.4: "When 10 update cycles pass, exactly one ball persists in play with no re-spawning"

#### 5. System Design & Query Safety ✅

**Fallible Systems**:

- Systems will use early returns and `.log_if_error()` patterns, not `.unwrap()` on queries
- Queries will use `With<T>` filters to target only relevant entities

**Query Specificity**:

- Ball spawning system queries: `Query<(&Transform, &Velocity), With<Ball>>` (only balls, not all entities)
- Brick detection: Already handled by existing collision detection + `BrickDestroyed` message pattern

**Change Detection**: Not needed for this feature (ball spawning doesn't watch for changes, it reacts to message).

#### 6. Message-Event Separation ✅

**Verified**: Existing system already uses correct pattern.

- `BrickDestroyed` is a `#[derive(Message)]` type (buffered queue)
- Consumed via `MessageReader<BrickDestroyed>` in scoring system
- Produced via `MessageWriter<BrickDestroyed>` in collision/destruction system
- Ball spawning system will follow same pattern: `MessageReader<BrickDestroyed>`

**NOT using Observers** because: Ball spawning is not reactive lifecycle logic (OnAdd/OnRemove), it's result-based logic (when message arrives, perform action).

#### 7. Hierarchy Safety ✅

**Decision**: Spawned balls will use simple `commands.spawn(ball_bundle)` without parent relationships.

**Rationale**:

- Balls are independent physics entities with no parent-child dependencies
- No need for `ChildOf`, `Parent`, `Children` components
- Matches existing ball spawning pattern in `src/systems/spawning.rs`

#### 8. Asset Management ✅

**Visual Assets**: Red 1, Red 2, Red 3 brick textures already exist in asset system (referenced in `docs/bricks.md`).

**Implementation**: Load brick textures once at startup (existing texture manifest system), reuse handles for all brick instances.

---

### Complexity Tracking

No constitutional violations.
Feature design is straightforward and aligns with existing patterns in codebase.

| Area | Status | Notes |
|------|--------|-------|
| ECS Architecture | ✅ Pass | Uses components, queries, systems as designed |
| Physics | ✅ Pass | Leverages Rapier3D for spawned ball behavior |
| Messaging | ✅ Pass | Uses existing `BrickDestroyed` message pattern |
| Coordinate System | ✅ Pass | Documented XZ plane + Y-axis convention |
| TDD | ✅ Pass | Tests-first requirement enforced |
| Multi-Frame Persistence | ✅ Pass | Tests verify persistence across 10+ frames |

## Project Structure

### Documentation (this feature)

```text
specs/025-ball-spawn-bricks/
├── spec.md              # Feature specification (complete, non-ambiguous)
├── plan.md              # This file (implementation plan)
├── research.md          # Phase 0 output (completed below)
├── data-model.md        # Phase 1 output (completed below)
├── quickstart.md        # Phase 1 output (completed below)
├── contracts/           # Phase 1 output (message specifications)
│   └── ball_spawn_bricks_messages.md
├── checklists/
│   └── requirements.md
└── tasks.md             # Phase 2 output (not created by /speckit.plan)
```

### Source Code (Brkrs repository)

```text
src/
├── systems/
│   ├── mod.rs                          # Export new BallSpawnBricksPlugin
│   ├── ball_spawn_bricks.rs            # NEW: Ball spawn/despawn logic
│   ├── spawning.rs                     # Existing: Ball entity spawning (update for Red 1/2/3)
│   ├── scoring.rs                      # Existing: Scoring system (awards points)
│   └── [other systems]
├── lib.rs                              # Game library (register new plugin)
├── main.rs                             # Application entry
├── level_format/                       # Existing: Level loading from RON
└── [other modules]

tests/
├── ball_spawn_bricks.rs                # NEW: Unit + integration tests for feature
└── [other tests]
```

**Structure Rationale**: Single-binary game architecture.
All ball spawn mechanics implemented as:

1. New system module `src/systems/ball_spawn_bricks.rs` (spawning/despawning logic)
2. Plugin `BallSpawnBricksPlugin` registering systems and message types
3. Tests in dedicated `tests/ball_spawn_bricks.rs` file
4. Integration with existing `BrickDestroyed` message stream and scoring system

## Phase 0: Outline & Research

**Status**: No unknowns identified in Technical Context section.
All technical decisions are either:

- Specified in existing project architecture (Rust 1.81, Bevy 0.17.3, cargo test, WASM support)
- Clear from specification requirements (100 points, brick indices, spawning behavior)
- Already implemented in project (Messages system, level loading, physics)

**Consequence**: Research phase skipped.
Proceed directly to Phase 1.

## Phase 1: Design & Contracts

### 1. Data Model

See [data-model.md](data-model.md) for complete entity and message specifications.

**Key Entities**:

- **BrickSpawnConfig (new Resource)**: Holds brick index mappings for Red 1/2/3 and their associated spawn behaviors
- **Ball (existing Component)**: Updated to track "was spawned by brick" for proper lifecycle
- **BrickDestroyed (existing Message)**: Already contains enough context (brick entity, triggering ball entity)

### 2. API Contracts

See [contracts/ball_spawn_bricks_messages.md](contracts/ball_spawn_bricks_messages.md) for message specifications.

**Key Message Flows**:

1. Ball hits brick 37/38/39 → Physics collision system sends `BrickDestroyed` message
2. Ball spawn/despawn system reads `BrickDestroyed` message
3. System spawns/despawns ball entities based on brick index
4. Scoring system reads same `BrickDestroyed` message, awards 100 points
5. Spawned balls immediately subject to physics on next frame

### 3. Quick Start Guide

See [quickstart.md](quickstart.md) for:

- Project setup and compilation
- Running tests with TDD verification
- Manual testing in-game
- Debugging spawned ball behavior

## Phase 2: Task Breakdown

**Status**: NOT created by `/speckit.plan` command.
Generated via `/speckit.tasks` command.

## Execution Notes

### Development Workflow

1. **Setup**: Create `tests/ball_spawn_bricks.rs` with failing tests for all 3 user stories
2. **Red Phase**: Commit failing tests (proof of TDD)
3. **Request Approval**: Get feature owner/requestor to approve test specifications
4. **Green Phase**: Implement `src/systems/ball_spawn_bricks.rs` until all tests pass
5. **Integration**: Update `src/systems/mod.rs` and `src/lib.rs` to register plugin
6. **Verification**: Test native + WASM builds (`cargo test --target wasm32-unknown-unknown`)

### Key Testing Points

- **Multi-frame persistence**: Verify spawned balls move for 10+ frames without being reset
- **Velocity inheritance**: Red 2 (inverse), Red 3 (Y-shaped spread)
- **Red 1 behavior**: Only triggering ball survives (test with 3-5 balls in play)
- **Scoring**: All three bricks award exactly 100 points
- **Physics**: Spawned balls interact with paddle, bricks, walls correctly

### Code Review Checklist

- [ ] Tests committed before implementation (red commit visible)
- [ ] Systems use `MessageReader<BrickDestroyed>`, not panicking queries
- [ ] Spawned balls use `commands.spawn()` without parent complications
- [ ] Brick indices 37/38/39 handled in level loading (RON files)
- [ ] No unconditional state overwrites in Update schedule
- [ ] Asset loading happens once (reuse texture handles)
- [ ] WASM build passes (no platform-specific code)
- [ ] Documentation updated (rustdoc for public functions)

## Timeline & Effort

- **Specification**: ✅ Complete
- **Planning**: ✅ Complete (this document)
- **Research**: ⏭️ Skipped (no unknowns)
- **Task Breakdown**: Next (`/speckit.tasks`)
- **Estimated Implementation**: 4-6 hours (3 brick behaviors + tests + integration)
- **Testing**: Embedded in development (TDD)
- **Review & Merge**: 1-2 hours
