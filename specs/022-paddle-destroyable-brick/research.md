# Phase 0 Research: Paddle-Destroyable Brick (Type 57)

**Feature**: 022-paddle-destroyable-brick | **Date**: 2026-01-13

## Research Questions

### Q1: How does the existing paddle collision detection system work?

**Context**: Need to detect paddle-brick collisions to trigger brick destruction and point award.

**Findings**:

- Paddle uses kinematic character controller (`KinematicCharacterControllerOutput`) in `src/lib.rs:read_character_controller_collisions`
- System reads `output.collisions` to detect paddle-wall and paddle-brick contacts
- Currently emits `BrickHit` event for paddle-brick collisions (used for audio)
- Existing system at line 977-1009 in `src/lib.rs` already distinguishes paddle-wall vs paddle-brick vs paddle-ball collisions

**Decision**: Extend `read_character_controller_collisions` system to check brick type ID and trigger destruction for type 57 bricks.

**Rationale**: Reuses existing collision detection infrastructure; minimal code duplication; aligns with established pattern for paddle collision handling.

**Alternatives Considered**:

- Separate collision event listener using rapier `CollisionEvent`: Would duplicate paddle collision logic; kinematic controller output is the canonical source for paddle contacts
- Observer pattern on `BrickHit` event: Observers are for immediate reactive logic, but we need Message-based destruction for consistency with existing brick destruction system

---

### Q2: How should paddle-destroyable bricks integrate with the scoring system?

**Context**: Brick type 57 awards 250 points on destruction.
Existing scoring system expects `BrickDestroyed` messages.

**Findings**:

- Scoring system in `src/systems/scoring.rs` reads `BrickDestroyed` messages (see line 153-160)
- `brick_points()` function maps brick type to points (line 65-126)
- Already includes type 57 mapping: `57 => 250` (line 123)
- Existing `despawn_marked_entities` system emits `BrickDestroyed` messages before despawning bricks (see `src/lib.rs`)

**Decision**: Use existing `BrickDestroyed` message flow.
Mark paddle-destroyable brick with `MarkedForDespawn` component when paddle contact occurs; let existing despawn system emit message and handle scoring.

**Rationale**: Zero changes needed to scoring system; `brick_points()` already configured; maintains consistency with ball-brick destruction flow; DRY principle (don't duplicate message emission).

**Alternatives Considered**:

- Custom `PaddleBrickDestroyed` message: Unnecessary complexity; scoring system would need dual message readers
- Emit `BrickDestroyed` directly in collision handler: Would duplicate message emission logic from `despawn_marked_entities`

---

### Q3: How should ball-brick collisions be prevented from destroying paddle-destroyable bricks?

**Context**: Ball must bounce off type 57 bricks without destroying them.
Normal bricks are destroyed by ball contact.

**Findings**:

- Ball-brick destruction logic in `src/lib.rs:handle_collision_events` (line 686-850)
- System marks bricks with `MarkedForDespawn` when ball hits them
- Multi-hit bricks use special handling (transition type instead of despawn)
- System checks brick type ID to determine behavior: `is_multi_hit_brick(current_type)` branches to special logic

**Decision**: Add `is_paddle_destroyable_brick(brick_type: u8) -> bool` helper function returning `true` for type 57.
In ball-brick collision handler, skip destruction (early continue) if `is_paddle_destroyable_brick()` returns true.
Ball physics naturally handles bounce via bevy_rapier3d.

**Rationale**: Minimal code change; follows established pattern for special brick types; no physics system changes needed (ball bounce is automatic from collider).

**Alternatives Considered**:

- Modify brick collider properties: Would affect ball bounce behavior unpredictably
- Custom collision filter: Over-engineered for single brick type check
- Remove `CountsTowardsCompletion` from type 57: Would break level completion requirement (spec FR-008)

---

### Q4: How should paddle-destroyable bricks be added to level files?

**Context**: Level designers must be able to place type 57 bricks in RON level files.

**Findings**:

- Level loader in `src/level_loader.rs` spawns bricks from `LevelDefinition` (line 579-650)
- Brick type ID stored as `BrickTypeId` component (u8 value)
- Special brick types get additional components:
  - Gravity bricks (21-25): `GravityBrick` component via `create_gravity_brick_component()` (line 38-74)
  - Multi-hit bricks (10-13): Implicitly handled by `is_multi_hit_brick()` check
- Loader uses `brick_type @ 3..=255` pattern match for all typed bricks

**Decision**: No special component needed for type 57.
`BrickTypeId(57)` component is sufficient.
Add `CountsTowardsCompletion` marker component during spawn (same as other destructible bricks).
Collision handler will check `BrickTypeId.0 == 57` to identify paddle-destroyable bricks.

**Rationale**: Simplest approach; no new loader logic required; follows same pattern as simple bricks (types 3-9, 20); `BrickTypeId` is the single source of truth.

**Alternatives Considered**:

- Add `PaddleDestroyable` marker component: Extra component overhead; `BrickTypeId` already provides type identity
- Create dedicated variant in brick type enum: Would require refactoring existing u8-based system; out of scope for single brick type

---

### Q5: How to handle simultaneous paddle and ball contact with the same brick?

**Context**: Edge case from spec - if paddle and ball touch brick in same frame, paddle takes precedence.

**Findings**:

- Paddle collision handler runs via `read_character_controller_collisions` in `FixedUpdate` schedule
- Ball collision handler runs via `handle_collision_events` (reads rapier `CollisionEvent`) in `FixedUpdate` schedule
- No explicit ordering between these systems
- `MarkedForDespawn` is checked in `despawn_marked_entities` system

**Decision**: Paddle collision handler marks brick with `MarkedForDespawn`.
Ball collision handler checks `if marked_despawn.contains(entity) { continue; }` before processing brick.
This ensures paddle-triggered destruction takes precedence even if both systems run in same frame.

**Rationale**: Explicit guard prevents double-processing; already used in multi-hit brick logic (`processed_bricks` HashSet pattern); minimal performance impact (hash lookup).

**Alternatives Considered**:

- System ordering (`.before()` / `.after()`): Fragile; doesn't guarantee single-frame consistency if physics produces events in both systems
- Frame-delay despawn: Would violate 1-frame destruction requirement (spec SC-001)

---

### Q6: How to implement DEBUG-level logging for paddle-brick collisions?

**Context**: Clarification requirement - log paddle-brick collision events at DEBUG level for troubleshooting.

**Findings**:

- Project uses `tracing` crate for logging (imported in multiple systems)
- Existing patterns: `debug!()` macro with optional target (e.g., `debug!(target: "textures::materials", ...)`)
- Paddle-brick collisions will be detected in `read_character_controller_collisions` extension

**Decision**: Use `debug!(target: "paddle_destroyable", "Paddle-brick type 57 collision detected: paddle={:?}, brick={:?}", paddle_entity, brick_entity)` in collision handler.

**Rationale**: Standard tracing pattern; target scoping allows filtering; DEBUG level appropriate for high-frequency game events; no performance impact in release builds.

**Alternatives Considered**:

- INFO level: Too verbose for production gameplay
- Custom event emission: Overkill for debugging-only feature
- Telemetry system: Out of scope; logging sufficient for troubleshooting

---

## Summary of Decisions

| Decision | Implementation Approach |
|----------|------------------------|
| Paddle collision detection | Extend `read_character_controller_collisions` to check `BrickTypeId == 57` and mark for despawn |
| Scoring integration | Reuse existing `BrickDestroyed` message flow via `MarkedForDespawn` component |
| Ball bounce behavior | Add `is_paddle_destroyable_brick()` guard in ball-brick collision handler; physics handles bounce automatically |
| Level file support | Use `BrickTypeId(57)` component; no special loader logic needed |
| Simultaneous collision priority | Paddle handler marks despawn; ball handler checks `MarkedForDespawn` and skips |
| Logging | `debug!()` macro with target `"paddle_destroyable"` in collision handler |

---

## Integration Points

1. **Paddle collision system** (`src/lib.rs:read_character_controller_collisions`): Add type 57 check and despawn marking
2. **Ball collision system** (`src/lib.rs:handle_collision_events`): Add paddle-destroyable brick guard to prevent ball-triggered destruction
3. **Scoring system** (`src/systems/scoring.rs`): No changes needed; `brick_points()` already maps 57 → 250
4. **Level loader** (`src/level_loader.rs`): No changes needed; type 57 bricks spawn with `BrickTypeId(57)` component
5. **Completion tracking**: Type 57 bricks spawn with `CountsTowardsCompletion` component (same as other destructible bricks)

---

## Risk Mitigation

| Risk | Mitigation Strategy |
|------|---------------------|
| Ball-brick collision destroys type 57 brick | Add explicit `is_paddle_destroyable_brick()` guard with early return in collision handler |
| Simultaneous paddle+ball contact ambiguity | Ball handler checks `MarkedForDespawn` before processing |
| Score not awarded on paddle contact | Leverage existing `BrickDestroyed` message flow; verify in integration test |
| Ball doesn't bounce off brick | No action needed; bevy_rapier3d handles physics bounce automatically from collider |
| Level completion doesn't count type 57 | Spawn with `CountsTowardsCompletion` component; verify in integration test |
| Multi-frame persistence failure | Add 10-frame persistence tests for score and brick destruction state |

---

## Dependencies

- **Existing systems**: Paddle collision handler, ball-brick collision handler, despawn system, scoring system
- **External crates**: bevy_rapier3d (collision detection), tracing (logging)
- **Constitution compliance**: Messages for destruction (not Observers), `despawn_recursive()` for hierarchy safety, fallible query patterns

---

## Open Questions

**None** - All unknowns from Technical Context section have been resolved through code analysis.
