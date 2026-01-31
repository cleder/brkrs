# Quickstart Guide: Level Navigation Bricks (Bricks 50 & 54)

**Feature**: Level Navigation Bricks **Date**: 2026-01-31 **For**: Developers implementing or extending this feature

## Overview

This guide provides a quickstart for implementing level navigation bricks (50 and 54) that allow players to control level progression by hitting special bricks.

---

## Prerequisites

- **Bevy 0.17.3** installed
- **bevy_rapier3d 0.32.0** for physics
- Existing game systems: brick collision, level loading, scoring, audio
- Test environment with `cargo test` working

---

## Feature Summary

**Brick 50 (Level Up)**:

- Destructible brick that advances to the next level when hit by the ball
- On final level: shows victory screen instead of transitioning
- Awards 0 points (utility brick)
- Plays unique "level up" sound

**Brick 54 (Level Down)**:

- Destructible brick that returns to the previous level when hit by the ball
- On level 1: destroys normally but no level transition occurs
- Awards 0 points (utility brick)
- Plays unique "level down" sound

---

## Implementation Checklist

### Phase 0: Setup & Constants

**Manual Testing Reference**: Use existing `assets/levels/level_014.ron` for gameplay testing and verification.
No new level files are needed; level 014 contains brick types needed for testing navigation brick mechanics.

**Assertion**: Verify `assets/levels/level_014.ron` exists before integration testing begins.

- [ ] Confirm `assets/levels/level_014.ron` exists
- [ ] No new level files created during implementation

### Phase 1: Constants & Foundation

- [ ] Add brick type constants to `src/level_format/mod.rs`:

  ```rust
  pub const BRICK_50: u8 = 50;
  pub const BRICK_54: u8 = 54;
  ```

- [ ] Verify `LevelSwitchSource::Brick` variant exists in `src/systems/level_switch.rs`
  - If not, add it to the enum

---

### Phase 1: Write TDD Tests (RED phase)

**Critical**: Tests MUST be written and committed BEFORE implementation.

#### Test File 1: `tests/brick_50_level_up.rs`

- [ ] Test: Brick 50 collision emits `LevelSwitchRequested { direction: Next }`
- [ ] Test: Brick 50 collision emits `BrickDestroyed { brick_type: 50 }`
- [ ] Test: Brick 50 advances to next level (verify `CurrentLevel` resource updated)
- [ ] Test: Brick 50 on final level shows victory screen (no transition)
- [ ] Test: Multi-frame persistence (level number persists across 10+ frames)
- [ ] Test: Scoring awards 0 points for brick 50

#### Test File 2: `tests/brick_54_level_down.rs`

- [ ] Test: Brick 54 collision emits `LevelSwitchRequested { direction: Previous }`
- [ ] Test: Brick 54 collision emits `BrickDestroyed { brick_type: 54 }`
- [ ] Test: Brick 54 returns to previous level (verify `CurrentLevel` resource updated)
- [ ] Test: Brick 54 on level 1 has no effect (no transition, brick still destroyed)
- [ ] Test: Multi-frame persistence (level number persists across 10+ frames)
- [ ] Test: Scoring awards 0 points for brick 54

#### Test File 3: `tests/level_navigation_audio.rs`

- [ ] Test: Brick 50 destruction plays unique sound (or fallback)
- [ ] Test: Brick 54 destruction plays unique sound (or fallback)
- [ ] Test: Sounds play exactly once per brick destruction

**Commit Point**: All tests written, all tests FAIL (red) → Request approval

---

### Phase 2: Brick Collision Logic (GREEN phase)

File: `src/lib.rs:mark_brick_on_ball_collision`

- [ ] Add `MessageWriter<LevelSwitchRequested>` parameter to system
- [ ] Check for brick types 50 and 54:

  ```rust
  use crate::level_format::{BRICK_50, BRICK_54};
  use crate::systems::level_switch::{LevelSwitchRequested, LevelSwitchSource, LevelSwitchDirection};

  // In mark_brick_on_ball_collision system:
  if current_type == BRICK_50 {
      level_switch_writer.write(LevelSwitchRequested {
          source: LevelSwitchSource::Brick,
          direction: LevelSwitchDirection::Next,
      });
      commands.entity(entity).insert(MarkedForDespawn);
  } else if current_type == BRICK_54 {
      level_switch_writer.write(LevelSwitchRequested {
          source: LevelSwitchSource::Brick,
          direction: LevelSwitchDirection::Previous,
      });
      commands.entity(entity).insert(MarkedForDespawn);
  }
  ```

**Verification**: Run tests → brick collision tests should pass

---

### Phase 3: Level Transition Boundary Logic

File: `src/level_loader.rs:process_level_switch_requests`

- [ ] Extend boundary condition handling:

  ```rust
  // When next_level_after() returns None:
  if request.source == LevelSwitchSource::Brick && request.direction == LevelSwitchDirection::Next {
      // Brick 50 on final level: show victory screen
      spawn_victory_screen(&mut commands);
      game_progress.finished = true;
      info!("Brick 50 hit on final level; game complete");
  } else {
      warn!("No next level available for switching");
  }
  ```

- [ ] Reuse existing victory screen logic from `advance_level_when_cleared` function:

  ```rust
  fn spawn_victory_screen(commands: &mut Commands) {
      commands.spawn((
          Text::new("You Win!"),
          Node {
              position_type: PositionType::Absolute,
              top: Val::Px(150.0),
              left: Val::Px(60.0),
              ..default()
          },
      ));
  }
  ```

**Verification**: Run tests → boundary condition tests should pass

---

### Phase 4: Scoring Integration

File: `src/systems/scoring.rs:brick_points`

- [ ] Add match arms for bricks 50 and 54:

  ```rust
  fn brick_points(brick_type: u8, rng: &mut impl Rng) -> u32 {
      match brick_type {
          // ... existing arms ...
          41 => 0,  // Extra Ball
          50 => 0,  // Level Up
          54 => 0,  // Level Down
          // ... remaining arms ...
      }
  }
  ```

**Verification**: Run tests → scoring tests should pass

---

### Phase 5: Audio Integration

File: `src/systems/audio.rs` (or equivalent)

- [ ] Add `SoundType` enum variants:

  ```rust
  pub enum SoundType {
      // ... existing variants ...
      Brick41ExtraLife,
      Brick50LevelUp,      // NEW
      Brick54LevelDown,    // NEW
      BrickDestroy,
      // ... remaining variants ...
  }
  ```

- [ ] Extend brick sound mapping:

  ```rust
  fn brick_sound(brick_type: u8) -> SoundType {
      match brick_type {
          41 => SoundType::Brick41ExtraLife,
          50 => SoundType::Brick50LevelUp,
          54 => SoundType::Brick54LevelDown,
          _ => SoundType::BrickDestroy,  // Fallback
      }
  }
  ```

- [ ] Add audio asset paths (if not already present):

  ```rust
  const BRICK_50_SOUND: &str = "audio/brick_50_level_up.ogg";
  const BRICK_54_SOUND: &str = "audio/brick_54_level_down.ogg";
  ```

- [ ] Load assets in startup system (if not already done)

**Verification**: Run tests → audio tests should pass (or fallback correctly if assets missing)

---

## Running the Feature

### Add Bricks to Level Files

Edit `assets/levels/level_XXX.ron`:

```ron
LevelDefinition(
    number: 1,
    matrix: [
        [0, 0, 50, 0, 0, ...],  // Brick 50 (Level Up) in position 3
        [0, 54, 0, 0, 0, ...],  // Brick 54 (Level Down) in position 2
        // ... remaining rows ...
    ],
    gravity: None,
)
```

### Test Manually

1. Run the game: `cargo run`
2. Hit brick 50 → should advance to next level
3. Hit brick 54 → should return to previous level
4. Hit brick 50 on final level → should show victory screen
5. Hit brick 54 on level 1 → should destroy brick, no transition

---

## Validation Commands

```bash
# Run all tests
cargo test

# Run only navigation brick tests
cargo test brick_50
cargo test brick_54
cargo test level_navigation

# Check formatting
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features

# Run bevy lint
bevy lint

# Build and run
cargo run
```

---

## Troubleshooting

### Issue: Tests fail with "message not emitted"

**Solution**: Ensure `MessageWriter<LevelSwitchRequested>` parameter is added to collision system and messages are written before `app.update()` in tests.

### Issue: Level doesn't transition

**Solution**:

1. Check `LevelSwitchState` resource is initialized with available levels
2. Verify `process_level_switch_requests` system is registered
3. Confirm `force_load_level_from_path` is called when target level exists

### Issue: Victory screen doesn't appear on final level

**Solution**: Ensure boundary condition logic checks `source == LevelSwitchSource::Brick` and `direction == Next` when `next_level_after()` returns `None`.

### Issue: Audio doesn't play

**Solution**:

1. Verify `SoundType` enum variants are added
2. Check audio assets are loaded in startup
3. Confirm fallback to `BrickDestroy` works if dedicated assets missing (per FR-006)

### Issue: Multi-frame persistence tests fail

**Solution**: Verify no systems in `Update` schedule unconditionally overwrite `CurrentLevel` resource.
Use guard fields (e.g., `last_level_number: Option<u32>`) to ensure idempotent initialization.

---

## Key Integration Points

| System | File | Modification |
|--------|------|--------------|
| Brick collision | `src/lib.rs:mark_brick_on_ball_collision` | Add brick type checks, emit messages |
| Level transitions | `src/level_loader.rs:process_level_switch_requests` | Extend boundary condition handling |
| Scoring | `src/systems/scoring.rs:brick_points` | Add 0-point mapping for types 50, 54 |
| Audio | `src/systems/audio.rs` | Add sound type variants, extend mapping |
| Constants | `src/level_format/mod.rs` | Add `BRICK_50`, `BRICK_54` constants |

---

## Next Steps

1. **After tests pass (GREEN)**: Commit implementation with tests
2. **Refactor (REFACTOR phase)**: Clean up code, remove duplication if any
3. **Integration testing**: Test with real level files and audio assets
4. **Manual QA**: Verify UX flow (sounds, transitions, victory screen)
5. **Documentation**: Update any user-facing documentation (if applicable)

---

## References

- **Specification**: [spec.md](spec.md)
- **Data Model**: [data-model.md](data-model.md)
- **Event Contracts**: [contracts/events.md](contracts/events.md)
- **Research**: [research.md](research.md)
- **Constitution**: `.specify/memory/constitution.md` (v1.6.0)
- **Existing Brick Patterns**:
  - Brick 41: `specs/019-extra-ball-brick/`
  - Bricks 42/91: `specs/023-brick-42-91-life-loss/`
  - Brick 57: `specs/022-paddle-destroyable-brick/`
