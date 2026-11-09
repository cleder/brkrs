# Research: Level Navigation Bricks (Bricks 50 & 54)

**Date**: 2026-01-31 **Feature**: Level Navigation Bricks **Phase**: Phase 0 - Research & Pattern Analysis

## Overview

This document consolidates research findings for implementing level navigation bricks (50 and 54) by analyzing existing brick patterns, level transition systems, and audio integration.

## Research Questions

### 1. How are special brick types currently implemented?

**Research Scope**: Analyze existing special bricks (41 Extra Ball, 42/91 Hazard, 57 Paddle-Destroyable) to understand the implementation pattern.

**Findings**:

**Brick Type Identification**:

- Each brick type has a constant in `src/level_format/mod.rs`:
  - `EXTRA_LIFE_BRICK: u8 = 41`
  - `HAZARD_BRICK_42: u8 = 42`
  - `HAZARD_BRICK_91: u8 = 91`
  - `PADDLE_DESTROYABLE_BRICK: u8 = 57`
- Bricks spawned with `BrickTypeId(value)` component
- No special marker components needed; `BrickTypeId.0` is the single source of truth

**Collision Handling Pattern** (`src/lib.rs:mark_brick_on_ball_collision`):

1. Query brick type via `BrickTypeId` component
2. Check brick type ID against constants
3. Execute type-specific logic (emit messages, skip destruction, etc.)
4. Standard bricks: mark for despawn → emit `BrickDestroyed` message
5. Special bricks: custom logic (e.g., brick 91 skips destruction, brick 41 emits life award)

**Message Flow**:

- Ball-brick collision detected → `mark_brick_on_ball_collision` system
- Brick marked with `MarkedForDespawn` component
- `despawn_marked_entities` system emits `BrickDestroyed` message before despawning
- Downstream systems (audio, scoring) consume `BrickDestroyed` messages

**Rationale**: New bricks 50 and 54 will follow this exact pattern: add constants, check type ID in collision handler, emit appropriate messages.

---

### 2. How does the level transition system work?

**Research Scope**: Understand `LevelSwitchRequested` message flow, `LevelSwitchState` resource, and `process_level_switch_requests` system.

**Findings**:

**Level Switch Message** (`src/systems/level_switch.rs`):

```rust
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LevelSwitchRequested {
    pub source: LevelSwitchSource,  // Keyboard, Brick, etc.
    pub direction: LevelSwitchDirection,  // Next, Previous
}
```

**Level Switch State** (`src/systems/level_switch.rs`):

- `LevelSwitchState` resource tracks available levels (discovered from `assets/levels/*.ron`)
- `next_level_after(current: u32)` returns next level slot or `None` if at end
- `previous_level_before(current: u32)` returns previous level slot or `None` if at start
- `pending_transition` flag prevents concurrent transitions

**Transition Flow** (`src/level_loader.rs:process_level_switch_requests`):

1. Read `LevelSwitchRequested` messages
2. Check if transition already in progress (`LevelAdvanceState.active` or `LevelSwitchState.pending_transition`)
3. Query `LevelSwitchState` for target level based on direction
4. If target exists: despawn all bricks/paddle/ball, call `force_load_level_from_path`
5. If target missing: log warning and ignore request
6. Mark transition end

**Boundary Behavior (Current)**:

- `next_level_after()` returns `None` if current level is the last in sequence
- `previous_level_before()` returns `None` if current level is the first
- **Current behavior when `None`**: Log warning, ignore request, clear messages

**Rationale**: Brick 50 and 54 will emit `LevelSwitchRequested` messages.
Need to extend boundary handling to:

- Brick 50 on final level → show victory screen (new behavior)
- Brick 54 on level 1 → no-op (existing behavior already handles this via `None` check)

**Decision**:

- Emit `LevelSwitchRequested` message from brick collision handler
- Extend `process_level_switch_requests` to detect boundary conditions:
  - When `next_level_after()` returns `None`: trigger victory screen instead of transition
  - When `previous_level_before()` returns `None`: skip transition (already happens)

---

### 3. How is audio integrated for brick destruction?

**Research Scope**: Understand how `BrickDestroyed` messages trigger unique sounds for different brick types.

**Findings**:

**Audio System Pattern** (`src/systems/audio.rs` - inferred from test patterns):

- Audio system consumes `BrickDestroyed` messages via `MessageReader<BrickDestroyed>`
- Maps `BrickDestroyed.brick_type` (u8) to `SoundType` enum
- Existing unique sounds:
  - Brick 41 (Extra Ball): `SoundType::Brick41ExtraLife`
  - Standard bricks: `SoundType::BrickDestroy`
- Sound plays exactly once per message

**Audio Asset Loading**:

- Assets loaded once during startup and handles stored in resource
- No per-frame loading

**Rationale**: Need to add two new `SoundType` variants:

- `SoundType::Brick50LevelUp` for brick 50
- `SoundType::Brick54LevelDown` for brick 54

**Decision**:

- Add constants for audio file paths in audio system configuration
- Extend sound mapping logic to check `brick_type == 50` or `brick_type == 54`
- Fallback to generic `BrickDestroy` sound if dedicated assets missing (per FR-006)

---

### 4. How are 0-point bricks handled in the scoring system?

**Research Scope**: Verify that 0-point brick types are already supported without special logic.

**Findings**:

**Scoring System** (`src/systems/scoring.rs`):

- `brick_points(brick_type: u8, rng: &mut impl Rng) -> u32` function maps brick types to point values
- Brick 41 (Extra Ball) already returns 0 points:

  ```rust
  41 => 0,  // Extra Ball brick
  ```

- Pattern: Explicit match arms for each brick type, default case for unknown types

**Rationale**: Bricks 50 and 54 will return 0 points using same pattern as brick 41.

**Decision**:

- Add match arms in `brick_points()`:

  ```rust
  50 => 0,  // Level Up brick (utility)
  54 => 0,  // Level Down brick (utility)
  ```

---

### 5. What happens to game state during level transitions?

**Research Scope**: Understand how `force_load_level_from_path` clears/resets game state.

**Findings**:

**State Clearing** (`src/level_loader.rs:force_load_level_from_path`):

- Despawns all entities with `Brick`, `Paddle`, `Ball`, `Merkaba` components
- Loads new level definition from RON file
- Updates `CurrentLevel` resource with new level number
- Spawns new bricks, paddle, ball from new level definition
- Preserves `LivesState` and `ScoreState` resources (player progress persists)
- Resets `GravityConfig` to new level's gravity settings (if specified)

**Powerup State**:

- Powerup effects stored as components on paddle/ball entities
- When paddle/ball despawned during transition → powerup effects cleared automatically
- No explicit powerup cleanup needed

**Rationale**: Specification requires "clear all active balls, powerups, and temporary effects" → existing transition system already does this via entity despawning.

**Decision**: No additional state clearing logic needed.
Reuse existing `force_load_level_from_path` behavior.

---

### 6. How are victory screens triggered in the game?

**Research Scope**: Check if victory screen system already exists or needs to be implemented.

**Findings**:

**Current Game Completion Logic** (`src/level_loader.rs:advance_level_when_cleared`):

- When all destructible bricks cleared and no next level file exists:
  - Sets `GameProgress.finished = true`
  - Spawns "You Win!"
    UI overlay
  - Emits `LevelCompleted` event for audio

**Victory Screen Components**:

- Text node with "You Win!" message
- Positioned at screen center with style settings

**Rationale**: Victory screen logic already exists for natural game completion.
Brick 50 on final level should trigger the same UI.

**Decision**:

- Reuse existing victory screen spawning logic
- Trigger when `next_level_after()` returns `None` and `brick_type == 50`
- Set `GameProgress.finished = true` for consistency

**Alternatives Considered**:

- Create separate victory screen for brick 50 → Rejected: Unnecessary duplication; existing UI is appropriate

---

## Implementation Decisions Summary

| Research Question | Decision | Rationale |
|-------------------|----------|-----------|
| Brick type pattern | Add constants `BRICK_50 = 50`, `BRICK_54 = 54` in `level_format/mod.rs` | Follows existing pattern for special bricks |
| Collision handling | Check `brick_type == 50` or `54` in `mark_brick_on_ball_collision`, emit `LevelSwitchRequested` | Reuses existing collision detection flow |
| Level transitions | Emit `LevelSwitchRequested { source: Brick, direction: Next/Previous }` | Integrates with existing level switch system |
| Boundary conditions | Extend `process_level_switch_requests` to handle `None` from level queries | Victory screen on final level, no-op on first level |
| Audio feedback | Add `SoundType::Brick50LevelUp` and `SoundType::Brick54LevelDown` | Follows brick 41 unique sound pattern |
| Scoring | Add `50 => 0, 54 => 0` to `brick_points()` | Matches brick 41 0-point utility pattern |
| State clearing | No changes needed | Existing `force_load_level_from_path` already clears entities |
| Victory screen | Reuse existing "You Win!" UI spawn logic | Consistent with natural game completion |

---

## Integration Points

### Code Locations to Modify

1. **`src/level_format/mod.rs`**:
   - Add `pub const BRICK_50: u8 = 50;`
   - Add `pub const BRICK_54: u8 = 54;`

2. **`src/lib.rs:mark_brick_on_ball_collision`**:
   - Add check for `brick_type == BRICK_50 or BRICK_54`
   - Emit `LevelSwitchRequested` message with appropriate direction
   - Continue with standard despawn flow (emit `BrickDestroyed`, mark for despawn)

3. **`src/level_loader.rs:process_level_switch_requests`**:
   - Detect when `next_level_after()` returns `None`
   - If source is brick 50: spawn victory screen, set `GameProgress.finished = true`
   - If source is other: existing warning behavior

4. **`src/systems/scoring.rs:brick_points`**:
   - Add `50 => 0,` and `54 => 0,` match arms

5. **`src/systems/audio.rs`** (or equivalent audio mapping):
   - Add `SoundType::Brick50LevelUp` and `SoundType::Brick54LevelDown` enum variants
   - Map `brick_type 50 → Brick50LevelUp`, `brick_type 54 → Brick54LevelDown`
   - Fallback to `BrickDestroy` if assets missing

### New Components/Resources

**None required**.
Existing components/resources are sufficient:

- `BrickTypeId(u8)` - identifies brick type
- `LevelSwitchRequested` - triggers level transitions
- `LevelSwitchState` - manages level sequence
- `GameProgress` - tracks game completion
- `CurrentLevel` - tracks active level number

### Testing Strategy

**Multi-Frame Persistence**:

- After brick 50/54 collision, verify `CurrentLevel` resource persists across 10+ frames
- Include all systems that write to `CurrentLevel` in test setup

**Boundary Conditions**:

- Test brick 50 on final level → victory screen spawned, no transition
- Test brick 54 on level 1 → no transition, brick destroyed normally

**Audio Integration**:

- Verify `BrickDestroyed` message emitted with correct `brick_type` (50 or 54)
- Verify unique sound mapping (when audio system extended)

---

## Alternatives Considered and Rejected

| Alternative | Rejected Reason |
|-------------|-----------------|
| Custom `NavigationBrick` component | Unnecessary complexity; `BrickTypeId` pattern already proven with 5+ brick types |
| Observer pattern for level transitions | Messages are more appropriate for cross-system, frame-agnostic transitions (per constitution) |
| Immediate level transition (no delay) | Inconsistent with existing fade-out/fade-in UX; would require removing `LevelAdvanceState` logic |
| Preserve powerups across transitions | Violates specification requirement (FR-010: "clear all active... powerup effects") |
| Separate victory trigger for brick 50 | Reusing existing `GameProgress.finished` logic is simpler and consistent |

---

## Dependencies

### External Crates

- No new dependencies required
- Uses existing: `bevy 0.17.3`, `bevy_rapier3d 0.32.0`, `serde`, `ron`

### Internal Systems

- **Depends on**:
  - Brick collision detection (`mark_brick_on_ball_collision`)
  - Level switch system (`process_level_switch_requests`)
  - Scoring system (`award_points_system`)
  - Audio system (brick destruction sounds)
- **Depended on by**: None (terminal feature; doesn't expose new APIs)

---

## Risk Assessment

| Risk | Mitigation |
|------|------------|
| Victory screen logic differs from expectation | Reuse existing "You Win!" UI; test boundary condition explicitly |
| Audio assets missing | Fallback to generic `BrickDestroy` sound (per FR-006) |
| Level transition mid-powerup causes state bugs | Existing despawn flow already clears components; multi-frame persistence tests will catch regressions |
| Boundary condition edge cases | TDD tests cover all 4 boundary scenarios (brick 50 on final, brick 54 on first, plus normal transitions) |

---

## References

- **Constitution**: `.specify/memory/constitution.md` (v1.6.0)
- **Existing Bricks**:
  - Brick 41: `specs/019-extra-ball-brick/`
  - Bricks 42/91: `specs/023-brick-42-91-life-loss/`
  - Brick 57: `specs/022-paddle-destroyable-brick/`
- **Level System**: `specs/007-level-metadata/`
- **Scoring**: `specs/009-add-scoring/`
