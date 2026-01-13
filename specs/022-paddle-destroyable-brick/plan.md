# Implementation Plan: Paddle-Destroyable Brick (Type 57)

**Branch**: `022-paddle-destroyable-brick` | **Date**: 2026-01-13 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/022-paddle-destroyable-brick/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add a new brick type (57) that is destroyed only by paddle contact (not ball), awards 250 points on destruction, counts toward level completion, and causes the ball to bounce off when hit.
This inverts the normal brick-ball destruction mechanic by making the paddle the only destructor while maintaining ball physics.

## Technical Context

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0 for collision detection, tracing 0.1 for logging **Storage**: RON files in `assets/levels/` directory for level definitions **Testing**: cargo test (integration tests in `tests/` directory) **Target Platform**: Native (Linux/Windows/macOS) and WASM (via wasm32-unknown-unknown) **Project Type**: Single project (game binary with library crate) **Performance Goals**: 60 FPS (16.67ms frame budget), collision detection within 1 frame (≤16.67ms) **Constraints**: Must maintain physics accuracy (ball bounce angles), score updates synchronous, no gameplay lag **Scale/Scope**: ~50 LOC for collision detection, ~30 LOC for scoring integration, ~20 LOC for level loader extension

## Constitution Check

*GATE: Must pass before Phase 0 research.*
                                                                                                                                                                                                       *Re-check after Phase 1 design.*

### Test-Driven Development (TDD) Compliance

- ✅ **Tests defined in spec**: User Stories 1-3 include comprehensive acceptance scenarios with Given/When/Then format
- ✅ **Failing test commits required**: Tests MUST be written first, committed, and verified to fail before implementation
- ✅ **Approval gate**: Tests MUST be reviewed and approved by feature owner before implementation begins
- ✅ **Multi-frame persistence testing**: Spec requires 10-frame persistence checks for score updates and brick destruction state (AS 1.4, AS 2.4)

### Bevy 0.17 ECS Compliance

**Message-Event Separation** (Constitution IX):
- ✅ **Decision**: Use **Messages** (`MessageWriter<BrickDestroyed>`) for paddle-brick collision/destruction
  - **Justification**: Matches existing brick destruction pattern (see `src/lib.rs:despawn_marked_entities`), enables frame-agnostic score integration, consistent with scoring system architecture
  - **NOT using Observers**: No immediate reactive logic needed; destruction flows through existing message-based brick destruction pipeline

**Hierarchy Safety** (Constitution IX):
- ✅ **Despawn pattern**: Use `commands.entity(brick).despawn_recursive()` for brick removal
  - **Rationale**: Handles potential nested entity structures (see spec AS 1.6, edge case 5)

**Fallible Systems** (Constitution IX):
- ✅ **Error handling**: Collision detection uses early returns for missing entities (`let Some(entity) = query.get() else { return; }`)
  - **No panicking queries**: Avoids `.unwrap()` on query results

**Multi-Frame Persistence** (Constitution IX):
- ✅ **No unconditional overwrites**: Brick destruction is one-way (despawn); scoring system uses saturating addition
  - **Guard**: Tests verify score persists across 10 frames (AS 1.4)

**Change Detection** (Constitution IX):
- N/A - No UI updates in this feature (score display handled by existing system)

**Logging** (Clarification: FR-014):
- ✅ **DEBUG-level logging**: Paddle-brick collisions logged via `debug!()` macro from tracing framework

### Coordinate System

**Not Applicable**: Brick type 57 is stationary and does not involve directional movement or physics velocity manipulation.
Ball bounce physics are handled entirely by bevy_rapier3d collision system.

### Gate Status

**✅ PASS** - All constitution requirements satisfied:
- TDD workflow defined
- Messages chosen over Observers (justified)
- Hierarchy safety via `despawn_recursive()`
- Fallible system patterns
- Multi-frame persistence testing mandated
- No coordinate system concerns

**Action**: Proceed to Phase 0 research.

## Project Structure

### Documentation (this feature)

```text
specs/022-paddle-destroyable-brick/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── events.md        # PaddleBrickCollision message contract
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── lib.rs                          # Core game logic, existing brick destruction system
├── level_loader.rs                 # Level loading, brick spawning (extend for type 57)
├── signals.rs                      # Message definitions (add PaddleBrickCollision if needed)
├── systems/
│   ├── scoring.rs                  # Scoring system (already handles BrickDestroyed)
│   ├── paddle_destroyable.rs       # NEW: Paddle-brick collision detection system
│   └── ...
└── ...

tests/
├── paddle_destroyable_brick.rs     # NEW: Integration tests for paddle-destroyable brick
└── ...

assets/
└── levels/
    └── *.ron                        # Level files (can include type 57 bricks)
```

**Structure Decision**: Single project structure maintained.
Feature adds one new system file (`systems/paddle_destroyable.rs`), extends level loader, and integrates with existing scoring/destruction systems.

## Complexity Tracking

**NO VIOLATIONS** - All constitution requirements satisfied without justification needed.
This section remains empty as no complexity violations were introduced by the design.

---

## Phase 0: Research & Decision Log

**Status**: ✅ COMPLETE

**Output**: [research.md](research.md)

**Key Decisions**:
1. **Paddle collision detection**: Extend existing `read_character_controller_collisions` system (reuses infrastructure)
2. **Scoring integration**: Leverage existing `BrickDestroyed` message flow (zero scoring system changes)
3. **Ball bounce prevention**: Add `is_paddle_destroyable_brick()` guard in ball collision handler
4. **Level file support**: Use `BrickTypeId(57)` component (no special loader logic)
5. **Simultaneous collision priority**: Paddle marks despawn; ball checks `MarkedForDespawn` guard
6. **Logging**: DEBUG level with target `"paddle_destroyable"` (standard tracing pattern)

**Unknowns Resolved**: All technical unknowns from spec clarified through code analysis.
No blocking questions remain.

---

## Phase 1: Design & Contracts

**Status**: ✅ COMPLETE

**Output**:
- [data-model.md](data-model.md) - Components, messages, systems, relationships
- [contracts/events.md](contracts/events.md) - `BrickDestroyed` message contract (reused)
- [quickstart.md](quickstart.md) - Build, test, and verification instructions

### Data Model Summary

**Components** (all existing - zero new types):
- `BrickTypeId(57)` - Type identifier
- `Brick` - Marker for brick entities
- `CountsTowardsCompletion` - Level completion flag
- `MarkedForDespawn` - Despawn trigger
- `Transform`, `Collider`, `Mesh3d`, `MeshMaterial3d` - Standard brick components

**Messages** (existing - zero new types):
- `BrickDestroyed` - Emitted for both ball-triggered AND paddle-triggered destruction
  - `brick_type: 57` identifies paddle-destroyable bricks
  - `destroyed_by: None` for paddle destruction

**Functions**:
- `is_paddle_destroyable_brick(u8) -> bool` - Type 57 identifier helper
- `brick_points(57, _) -> 250` - Already implemented in scoring system

**Systems Modified**:
- `read_character_controller_collisions` - Add type 57 check + `MarkedForDespawn` insertion
- `handle_collision_events` - Add `is_paddle_destroyable_brick()` guard to skip destruction

**Systems Unchanged**:
- `despawn_marked_entities` - Handles despawn + message emission (no changes)
- `award_points_system` - Reads `BrickDestroyed`, awards 250 points (no changes)
- Level loader - Spawns type 57 bricks with existing pattern (no changes)

### Constitution Re-Check (Post-Design)

**✅ ALL REQUIREMENTS MAINTAINED**:
- Messages used for destruction (not Observers) - [data-model.md systems section]
- `despawn_recursive()` for hierarchy safety - [data-model.md despawn system]
- Fallible queries with early returns - [contracts/events.md code examples]
- Multi-frame persistence testing mandated - [quickstart.md acceptance tests]
- DEBUG logging at correct level - [research.md Q6]
- No coordinate system concerns - [constitution check: N/A]
- No new components or messages - [data-model.md: zero new types]

**Design Validation**: ✅ PASS - Design aligns with constitution; no violations introduced.

---
