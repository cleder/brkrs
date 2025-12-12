# Tasks: Paddle Size Powerups

**Feature**: `001-paddle-size-powerups` **Branch**: `001-paddle-size-powerups` **Date**: 2025-12-12 **Tech Stack**: Rust 1.81, Bevy 0.17.3, bevy_rapier3d 0.32.0

**Feature Summary**: Implement paddle size modification mechanics triggered when the **ball** collides with special bricks.
Brick 30 shrinks paddle to 70% (14 units) for 10 seconds.
Brick 32 enlarges paddle to 150% (30 units) for 10 seconds.
Effects are temporary, replaced on new triggers, and cleared on level advance.
Includes visual feedback (color tint + glow) and audio cues.

---

## User Stories Overview

| Story | Priority | Title | MVP |
|-------|----------|-------|-----|
| US1 | P1 | Paddle Shrink on Brick 30 Hit | ✅ Yes |
| US2 | P1 | Paddle Enlarge on Brick 32 Hit | ✅ Yes |
| US3 | P2 | Visual Feedback for Size Changes | ⚠️ Polish |

**Recommended MVP Scope**: US1 + US2 (core mechanics).
US3 (visual/audio) can follow in polish phase.

---

## Phase 1: Setup & Foundational Infrastructure

> **Goal**: Prepare codebase for paddle size feature. This phase is prerequisite to all user stories.

### 1.1 Component Definition & Event Setup

- [ ] T001 Create component definition file `src/systems/paddle_size_components.rs` with `PaddleSizeEffect`, `SizeEffectType`, marker components (`BrickType30`, `BrickType32`), and `PaddleSizeEffectApplied` event struct

- [ ] T002 [P] Create type constants file `src/systems/paddle_size_constants.rs` defining `PADDLE_BASE_WIDTH: f32 = 20.0`, `SHRINK_MULTIPLIER: f32 = 0.7`, `ENLARGE_MULTIPLIER: f32 = 1.5`, `EFFECT_DURATION: f32 = 10.0`, `MIN_PADDLE_WIDTH: f32 = 10.0`, `MAX_PADDLE_WIDTH: f32 = 30.0`

- [ ] T003 Update `src/systems/mod.rs` to export new component and event types from `paddle_size_components` module

### 1.2 Size Calculation Utilities

- [ ] T004 Create utility module `src/systems/paddle_size_utils.rs` with function `calculate_clamped_width(base_width: f32, effect_type: SizeEffectType) -> f32` that applies multiplier and clamps to [10, 30] range

- [ ] T005 [P] Add function `size_to_color(effect_type: SizeEffectType) -> Color` returning `Color::srgb(1.0, 0.3, 0.3)` for Shrink, `Color::srgb(0.3, 1.0, 0.3)` for Enlarge

- [ ] T006 [P] Add function `size_to_glow(effect_type: SizeEffectType) -> LinearRgba` returning `LinearRgba::rgb(0.3, 0.0, 0.0)` for Shrink, `LinearRgba::rgb(0.0, 0.3, 0.0)` for Enlarge

### 1.3 Audio Asset Setup

- [ ] T007 Add audio files `assets/audio/paddle_shrink.ogg` and `assets/audio/paddle_enlarge.ogg` (or `.mp3`/`.wav` alternatives)

- [ ] T008 Update `assets/audio/manifest.ron` to include entries for new audio files (if using manifest loading pattern)

### 1.4 Level Loader Integration

- [ ] T009 Update `src/level_loader.rs` brick spawning logic to detect brick type 30 and add `BrickType30` marker component

- [ ] T010 [P] Update `src/level_loader.rs` brick spawning logic to detect brick type 32 and add `BrickType32` marker component

### 1.5 App Registration

- [ ] T011 Register `PaddleSizeEffectApplied` event in app setup (`src/lib.rs` or `src/main.rs`) using `.add_event::<PaddleSizeEffectApplied>()`

- [ ] T012 [P] Import all paddle size systems into app (will be defined in later phases), ready for scheduling

---

## Phase 2: Core Collision & Effect Creation (US1 + US2 Foundational)

> **Goal**: Implement brick collision detection and paddle size effect creation. This is the blocking prerequisite for both US1 and US2.

### 2.1 Collision Detection System

- [ ] T013 Create system `src/systems/paddle_size_collision.rs` with function `detect_ball_powerup_brick_collisions(mut commands: Commands, mut collision_events: EventReader<CollisionEvent>, paddles: Query<(Entity, &Paddle)>, balls: Query<&Ball>, brick30: Query<&BrickType30>, brick32: Query<&BrickType32>, mut effect_events: EventWriter<PaddleSizeEffectApplied>)`

- [ ] T014 Implement collision event filtering logic: for each `CollisionEvent::Started`, check if one entity has `Ball` component and other has `BrickType30` or `BrickType32` marker component

- [ ] T015 On ball-brick match, determine which brick type (30 or 32) was hit and create corresponding `PaddleSizeEffect` component on the **paddle entity** (not the ball)

- [ ] T016 Query for the paddle entity in the world and insert/replace `PaddleSizeEffect` component on it using `commands.entity(paddle_entity).remove::<PaddleSizeEffect>().insert(new_effect)`

- [ ] T017 Emit `PaddleSizeEffectApplied` event with paddle entity, effect type, and calculated new width for audio/visual systems to listen to

### 2.2 Effect Timer System

- [ ] T018 Create system `src/systems/paddle_size_timer.rs` with function `update_paddle_size_timers(mut paddles: Query<&mut PaddleSizeEffect>, time: Res<Time>)`

- [ ] T019 Implement timer countdown: decrement `remaining_duration` by `time.delta_seconds()` for each active effect

- [ ] T020 Add condition to prevent negative durations: clamp `remaining_duration` to minimum of 0.0

### 2.3 Effect Cleanup System

- [ ] T021 Create system `src/systems/paddle_size_cleanup.rs` with function `remove_expired_size_effects(mut commands: Commands, paddles: Query<(Entity, &PaddleSizeEffect)>)`

- [ ] T022 Find all paddle entities where `remaining_duration <= 0.0` and remove `PaddleSizeEffect` component

- [ ] T023 Add lifecycle cleanup for `LevelChangeEvent`: on event, remove `PaddleSizeEffect` from all paddle entities in `src/systems/paddle_size_cleanup.rs`

- [ ] T024 [P] Add lifecycle cleanup for `PlayerLossEvent` (or equivalent): on event, remove `PaddleSizeEffect` from all paddle entities

### 2.4 System Scheduling

- [ ] T025 Add systems to app in correct order in `src/lib.rs` or `src/main.rs`:

  ```text
  detect_paddle_powerup_collisions
      ↓
  update_paddle_size_timers
      ↓
  remove_expired_size_effects
  ```

  Using `.chain()` to enforce execution order

---

## Phase 3: User Story 1 - Paddle Shrink on Brick 30 Hit

> **Goal**: Implement shrink mechanic. Player's paddle shrinks to 70% when hitting brick 30, persists for 10s, then returns to normal.

### 3.1 Shrink Effect Behavior Verification

- [ ] T026 [US1] Verify in integration tests that **ball collision with brick 30** creates `PaddleSizeEffect { effect_type: Shrink, remaining_duration: 10.0 }` on paddle

- [ ] T027 [US1] [P] Verify paddle width calculation: normal 20 units → shrunk to 14 units (0.7 × 20) after ball hits brick 30

- [ ] T028 [US1] [P] Verify timer countdown: ball hits brick 30, advance time 10 seconds in test, confirm effect removed and width returns to 20

- [ ] T029 [US1] [P] Verify clamping at minimum: paddle at 10 units, ball hits brick 30, confirm size stays 10 (not 7)

### 3.2 Effect Replacement Logic (Shrink)

- [ ] T030 [US1] [P] Verify timer reset: ball hits brick 30 twice within 10s, confirm timer resets to 10s both times

- [ ] T031 [US1] [P] Verify effect replacement: paddle shrunk, then ball hits brick 32, confirm shrink effect replaced with enlarge (width jumps to 30)

### 3.3 Lifecycle Integration (Shrink)

- [ ] T032 [US1] [P] Verify level transition: active shrink effect → new level starts → effect cleared, width returns to 20

- [ ] T033 [US1] [P] Verify loss event: active shrink effect → player loses life → effect cleared, width returns to 20

### 3.4 Acceptance Criteria Validation (Shrink)

- [ ] T034 [US1] **AC1**: Normal size (20) + ball hits brick 30 → width becomes 14 units ✓

- [ ] T035 [US1] **AC2**: Ball hits brick 30, shrunk paddle → 10s timeout → returns to previous size ✓

- [ ] T036 [US1] **AC3**: Shrunk paddle + ball hits brick 32 → becomes 30 units (not 70% of 30) ✓

- [ ] T037 [US1] **AC4**: Shrunk paddle + ball hits brick 30 again → timer resets, width stays 14 ✓

---

## Phase 4: User Story 2 - Paddle Enlarge on Brick 32 Hit

> **Goal**: Implement enlarge mechanic. Player's paddle enlarges to 150% when hitting brick 32, persists for 10s, then returns to normal.

### 4.1 Enlarge Effect Behavior Verification

- [ ] T038 [US2] Verify in integration tests that ball hits brick 32 creates `PaddleSizeEffect { effect_type: Enlarge, remaining_duration: 10.0 }`

- [ ] T039 [US2] [P] Verify paddle width calculation: normal 20 units → enlarged to 30 units (1.5 × 20) after ball hits brick 32

- [ ] T040 [US2] [P] Verify timer countdown: ball hits brick 32, advance time 10 seconds in test, confirm effect removed and width returns to 20

- [ ] T041 [US2] [P] Verify clamping at maximum: ball hits brick 32 when already at 30 units, confirm size stays 30 (not 45)

### 4.2 Effect Replacement Logic (Enlarge)

- [ ] T042 [US2] [P] Verify timer reset: ball hits brick 32 twice within 10s, confirm timer resets to 10s both times

- [ ] T043 [US2] [P] Verify effect replacement: paddle enlarged, then ball hits brick 30, confirm enlarge effect replaced with shrink (width drops to 14)

### 4.3 Lifecycle Integration (Enlarge)

- [ ] T044 [US2] [P] Verify level transition: active enlarge effect → new level starts → effect cleared, width returns to 20

- [ ] T045 [US2] [P] Verify loss event: active enlarge effect → player loses life → effect cleared, width returns to 20

### 4.4 Acceptance Criteria Validation (Enlarge)

- [ ] T046 [US2] **AC1**: Normal size (20) + ball hits brick 32 → width becomes 30 units ✓

- [ ] T047 [US2] **AC2**: Enlarged paddle → 10s timeout → returns to previous size ✓

- [ ] T048 [US2] **AC3**: Enlarged paddle + ball hits brick 30 → becomes 14 units (not 70% of 30) ✓

- [ ] T049 [US2] **AC4**: Enlarged paddle + ball hits brick 32 again → timer resets, width stays 30 ✓

---

## Phase 5: User Story 3 - Visual Feedback for Size Changes

> **Goal**: Implement visual and audio feedback. Red glow for shrink, green glow for enlarge. Distinct sound cues.

### 5.1 Visual Feedback System

- [ ] T050 [US3] Create system `src/systems/paddle_size_visual.rs` with function `update_paddle_size_visual(effects: Query<(&PaddleSizeEffect, &Handle<StandardMaterial>)>, mut materials: ResMut<Assets<StandardMaterial>>)`

- [ ] T051 [US3] For each paddle with active `PaddleSizeEffect`, query and update its material:
  - Shrink: set `base_color = Color::srgb(1.0, 0.3, 0.3)` (red) and `emissive = LinearRgba::rgb(0.3, 0.0, 0.0)` (red glow)
  - Enlarge: set `base_color = Color::srgb(0.3, 1.0, 0.3)` (green) and `emissive = LinearRgba::rgb(0.0, 0.3, 0.0)` (green glow)

- [ ] T052 [US3] Handle effect expiry: when `PaddleSizeEffect` removed, restore material to original (non-tinted, no emission)

- [ ] T053 [US3] [P] Add material reset system to `paddle_size_cleanup.rs`: on `PaddleSizeEffect` removal, restore paddle material to default appearance

### 5.2 Audio Feedback System

- [ ] T054 [US3] Create system `src/systems/paddle_size_audio.rs` with function `play_size_effect_audio(mut effect_events: EventReader<PaddleSizeEffectApplied>, audio: Res<Audio>, audio_assets: Res<AudioAssets>)`

- [ ] T055 [US3] On `PaddleSizeEffectApplied` event:
  - If `effect_type == Shrink`: play `audio_assets.paddle_shrink` using `audio.play()`
  - If `effect_type == Enlarge`: play `audio_assets.paddle_enlarge` using `audio.play()`

- [ ] T056 [US3] [P] Ensure audio assets are loaded in audio asset manifest or preload during app startup

### 5.3 Visual & Audio Integration

- [ ] T057 [US3] Add `update_paddle_size_visual` system to app scheduling (runs after collision detection)

- [ ] T058 [US3] [P] Add `play_size_effect_audio` system to app scheduling (runs parallel with visual, after collision detection)

### 5.4 Acceptance Criteria Validation (Visual/Audio)

- [ ] T059 [US3] **AC1**: Ball hits brick 30 → paddle displays red tint + subtle glow + plays shrink sound ✓

- [ ] T060 [US3] **AC2**: Ball hits brick 32 → paddle displays green tint + subtle glow + plays enlarge sound ✓

- [ ] T061 [US3] **AC3**: Effect expires → color and glow disappear, paddle returns to normal appearance ✓

- [ ] T062 [US3] [P] Verify visual feedback latency < 100ms (immediate on collision)

---

## Phase 6: Integration & Cross-Level Testing

> **Goal**: Validate feature across multiple levels and edge cases. Ensure no regressions.

### 6.1 Multi-Level Integration Tests

- [ ] T063 [P] Integration test: Load level with brick 30 and brick 32, ball hits both, verify correct behavior in each

- [ ] T064 [P] Integration test: Level change while effect active → confirm effect cleared and paddle resets

- [ ] T065 [P] Integration test: Player loss while effect active → confirm effect cleared and paddle resets

- [ ] T066 [P] Integration test: Ball alternately hits brick 30 and brick 32 → verify effect replacement works correctly

### 6.2 Edge Case Validation

- [ ] T067 [P] Edge case test: Paddle at min size (10) + brick 30 collision → effect activates, timer resets, size stays clamped

- [ ] T068 [P] Edge case test: Paddle at max size (30) + brick 32 collision → effect activates, timer resets, size stays clamped

- [ ] T069 [P] Edge case test: Hit brick 30 twice in rapid succession → verify timer resets correctly both times

- [ ] T070 [P] Edge case test: Hit brick 32, then brick 30, then brick 32 again → verify effect replaces correctly each time

### 6.3 Performance Validation

- [ ] T071 [P] Performance test: Run game loop with continuous collisions, verify 60 FPS maintained (< 16.7ms frame budget)

- [ ] T072 [P] Memory test: Play through 10 levels with multiple effect triggers per level, verify no memory leaks

### 6.4 Platform Testing

- [ ] T073 [P] Native platform test: Run on Linux/macOS/Windows, verify all systems work correctly

- [ ] T074 [P] WASM platform test: Build WASM target, run in browser, verify feature functions without platform-specific issues

### 6.5 Code Quality

- [ ] T075 [P] Run `cargo fmt --all` on new code

- [ ] T076 [P] Run `cargo clippy --all-targets --all-features` and fix any warnings

- [ ] T077 [P] Run `cargo test --all` and ensure all tests pass

- [ ] T078 [P] Run `bevy lint` and address any Bevy-specific issues

---

## Phase 7: Documentation & Polish

> **Goal**: Document implementation, add final touches, prepare for merge.

### 7.1 Code Documentation

- [ ] T079 Add rustdoc comments to all public functions in `paddle_size_components.rs`, `paddle_size_utils.rs`, and all system files

- [ ] T080 [P] Add module-level documentation to `src/systems/paddle_size_*.rs` files explaining purpose and integration

- [ ] T081 [P] Update `src/systems/mod.rs` with brief documentation of paddle size feature integration

### 7.2 Testing Documentation

- [ ] T082 [P] Add test module documentation explaining test organization and how to run specific test suites

- [ ] T083 [P] Document acceptance criteria test mappings (which test validates which AC)

### 7.3 Feature Validation

- [ ] T084 Manual acceptance test: Play through level with brick 30 and brick 32, verify:
  - Shrink works (14 units)
  - Enlarge works (30 units)
  - Visual feedback (color + glow) appears
  - Audio cues play
  - Effects expire after 10s
  - Effects clear on level change

- [ ] T085 [P] Manual edge case test: Force paddle to min/max size, hit bricks, verify clamping behavior

- [ ] T086 [P] Regression test: Ensure existing game mechanics still work (ball physics, other bricks, paddle movement)

### 7.4 Documentation Files

- [ ] T087 Update project README if feature is significant enough for user documentation

- [ ] T088 [P] Add entry to CHANGELOG.md documenting new feature

---

## Dependencies & Execution Strategy

### Critical Path (Must Complete in Order)

```text
Phase 1 (Setup)
    ↓
Phase 2 (Core Collision & Effect Creation)
    ├→ Phase 3 (US1 - Shrink)
    ├→ Phase 4 (US2 - Enlarge)
    └→ Phase 5 (US3 - Visual/Audio) [Optional for MVP]
         ↓
Phase 6 (Integration & Testing)
    ↓
Phase 7 (Documentation & Polish)
```

### Parallelization Opportunities

**Within Phase 1**:

- T002, T003, T005, T006, T010 can run in parallel (no interdependencies)

**Within Phase 2**:

- T013-017 (collision system) must complete before T018+ (timer/cleanup)
- T018-020 can run parallel with T013-017 (different systems)
- T023-024 (cleanup) can run parallel with T018-020

**Within Phase 3 & 4**:

- Shrink tests (Phase 3) and Enlarge tests (Phase 4) can run **in parallel** after Phase 2 completes
- Both stories use same collision detection and effect system, only differ in effect type

**Within Phase 5**:

- Visual system (T050-053) and Audio system (T054-056) can run in parallel

**Within Phase 6**:

- All edge case and integration tests can run in parallel

### Recommended MVP Execution (Parallel When Possible)

```text
Day 1: Phase 1 (All setup tasks)
       Phase 2 (All collision & effect creation tasks)

Day 2: Phase 3 & Phase 4 in parallel
       (Shrink and Enlarge mechanics both implemented concurrently)

Day 3: Phase 6 (Integration testing, validation)

Day 4: Phase 5 (Visual/Audio polish)
       Phase 7 (Documentation)
```

**Estimated Effort**:

- Phase 1: 2-3 hours
- Phase 2: 4-5 hours
- Phase 3: 3-4 hours
- Phase 4: 3-4 hours (parallel with Phase 3)
- Phase 5: 2-3 hours
- Phase 6: 3-4 hours
- Phase 7: 1-2 hours

**Total**: ~20-25 hours (assuming developer familiar with Bevy ECS)

---

## Task Checklist Format

Each task follows this structure:

```text
- [ ] [TaskID] [P?] [Story?] Description with file path
```

Where:

- `[ ]` = Checkbox (mark ✓ when complete)
- `TaskID` = T001, T002, etc. (sequential)
- `[P]` = Parallelizable (optional marker)
- `[Story]` = [US1], [US2], [US3] (story identifier, only in Phase 3+)
- `Description` = Clear action with **exact file path**

---

## Validation & Success Criteria

### All User Stories Complete When

✅ **US1 (Shrink)**:

- [ ] Collision with brick 30 triggers shrink (14 units)
- [ ] Effect persists for 10 seconds
- [ ] Paddle returns to previous size after expiry
- [ ] All 4 acceptance scenarios pass

✅ **US2 (Enlarge)**:

- [ ] Collision with brick 32 triggers enlarge (30 units)
- [ ] Effect persists for 10 seconds
- [ ] Paddle returns to previous size after expiry
- [ ] All 4 acceptance scenarios pass

✅ **US3 (Visual/Audio)** [Optional for MVP]:

- [ ] Visual feedback (red/green tint + glow) displays
- [ ] Audio cues play on brick hit
- [ ] All 3 acceptance scenarios pass

### Code Quality Gate

- [ ] All tests pass: `cargo test`
- [ ] No clippy warnings: `cargo clippy --all-targets --all-features`
- [ ] Formatted: `cargo fmt --all`
- [ ] No Bevy lint issues: `bevy lint`

### Feature Validation Gate

- [ ] Manual play-through: All mechanics work as designed
- [ ] No regressions: Existing game mechanics unaffected
- [ ] Cross-platform: Native and WASM builds successful
- [ ] Performance: 60 FPS maintained during normal play
