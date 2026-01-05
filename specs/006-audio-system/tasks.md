# Tasks: Audio System

**Input**: Design documents from `/specs/006-audio-system/` **Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization, audio plugin structure, and asset directory

- [X] T001 Create audio assets directory structure at `assets/audio/`
- [X] T002 [P] Create placeholder audio manifest at `assets/audio/manifest.ron`
- [X] T003 [P] Create audio module file at `src/systems/audio.rs` with module doc and imports
- [X] T004 Export audio module from `src/systems/mod.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core audio infrastructure that MUST be complete before ANY user story

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T005 Define `SoundType` enum in `src/systems/audio.rs` with all 8 variants (BrickDestroy, MultiHitImpact, WallBounce, PaddleHit, PaddleWallHit, PaddleBrickHit, LevelStart, LevelComplete)
- [X] T006 [P] Define `AudioConfig` resource struct in `src/systems/audio.rs` with master_volume (f32) and muted (bool) fields, derive Serialize/Deserialize
- [X] T007 [P] Define `AudioAssets` resource struct in `src/systems/audio.rs` with `HashMap<SoundType, Handle<AudioSource>>`
- [X] T008 [P] Define `ActiveSounds` resource struct in `src/systems/audio.rs` with `HashMap<SoundType, u8>` for concurrent tracking
- [X] T009 Implement `AudioPlugin` struct in `src/systems/audio.rs` that registers resources and systems
- [X] T010 Implement `load_audio_assets` startup system in `src/systems/audio.rs` to load sounds from manifest
- [X] T011 Implement `play_sound` helper function in `src/systems/audio.rs` that checks muted, concurrent limits, spawns AudioPlayer
- [X] T012 Register `AudioPlugin` in `src/lib.rs` run() function
- [X] T013 Add unit tests for `AudioConfig` validation in `tests/audio_config.rs`

**Checkpoint**: Foundation ready - audio plugin registered, assets loading, play_sound helper works

---

## Phase 3: User Story 1 - Brick Hit Audio Feedback (Priority: P1) 🎯 MVP

**Goal**: Play audio when bricks are hit or destroyed (multi-hit impact + brick destruction sounds)

**Independent Test**: Launch ball at bricks, hear multi-hit impact sound for indices 10-13, hear destruction sound when brick despawns

### Implementation for User Story 1

- [X] T014 [US1] Define `BrickDestroyed` event struct in `src/lib.rs` with entity and brick_type fields
- [X] T015 [US1] Emit `BrickDestroyed` event in `despawn_marked_entities` system in `src/lib.rs` before despawning
- [X] T016 [US1] Implement `on_brick_destroyed_sound` observer in `src/systems/audio.rs` to play BrickDestroy sound
- [X] T017 [US1] Implement `on_multi_hit_brick_sound` observer in `src/systems/audio.rs` to play MultiHitImpact sound
- [X] T018 [US1] Register brick audio observers in `AudioPlugin` in `src/systems/audio.rs`
- [X] T019 [US1] Update existing `on_multi_hit_brick_sound` placeholder in `src/systems/multi_hit.rs` to remove placeholder code (observer now in audio.rs)

**Checkpoint**: Ball hitting multi-hit bricks plays impact sound; brick destruction plays destroy sound

---

## Phase 4: User Story 2 - Ball Bounce Audio (Priority: P1)

**Goal**: Play audio when ball bounces off walls or paddle

**Independent Test**: Launch ball, hear wall bounce when ball hits borders, hear paddle hit when ball hits paddle

### Implementation for User Story 2

- [X] T020 [US2] Define `BallWallHit` event struct in `src/lib.rs` with entity and impulse fields
- [X] T021 [US2] Add ball-wall collision detection in `src/lib.rs` (detect ball-border collisions via rapier events)
- [X] T022 [US2] Emit `BallWallHit` event when ball collides with border in `src/lib.rs`
- [X] T023 [US2] Implement `on_ball_wall_hit_sound` observer in `src/systems/audio.rs` to play WallBounce sound
- [X] T024 [US2] Implement `on_paddle_ball_hit_sound` observer in `src/systems/audio.rs` to play PaddleHit sound (observe existing BallHit event)
- [X] T025 [US2] Register ball bounce audio observers in `AudioPlugin` in `src/systems/audio.rs`

**Checkpoint**: Ball bouncing off walls and paddle produces distinct sounds

---

## Phase 5: User Story 3 - Paddle Collision Audio (Priority: P1)

**Goal**: Play audio when paddle bumps into walls or bricks

**Independent Test**: Move paddle into wall boundary, hear collision sound; move paddle into brick, hear different collision sound

### Implementation for User Story 3

- [X] T026 [US3] Implement `on_paddle_wall_hit_sound` observer in `src/systems/audio.rs` to play PaddleWallHit sound (observe existing WallHit event)
- [X] T027 [US3] Implement `on_paddle_brick_hit_sound` observer in `src/systems/audio.rs` to play PaddleBrickHit sound (observe existing BrickHit event)
- [X] T028 [US3] Register paddle collision audio observers in `AudioPlugin` in `src/systems/audio.rs`

**Checkpoint**: Paddle collisions with walls and bricks produce distinct sounds

---

## Phase 6: User Story 4 - Level Transition Audio (Priority: P2)

**Goal**: Play audio when levels start and complete

**Independent Test**: Complete a level (destroy all bricks), hear level complete sound; start new level, hear level start sound

### Implementation for User Story 4

- [X] T029 [US4] Define `LevelStarted` event struct in `src/level_loader.rs` with level_index field
- [X] T030 [US4] Emit `LevelStarted` event when level finishes loading in `src/level_loader.rs`
- [X] T031 [US4] Implement `on_level_started_sound` observer in `src/systems/audio.rs` to play LevelStart sound
- [X] T032 [US4] Implement `on_level_complete_sound` observer in `src/systems/audio.rs` to play LevelComplete sound (observe LevelSwitchRequested when source is completion)
- [X] T033 [US4] Register level transition audio observers in `AudioPlugin` in `src/systems/audio.rs`

**Checkpoint**: Level transitions have distinct audio cues

---

## Phase 7: User Story 5 - Audio Configuration (Priority: P2)

**Goal**: Allow players to adjust volume and mute/unmute audio with persistence

**Independent Test**: Adjust volume slider, sounds get louder/quieter; toggle mute, sounds stop/resume; restart game, settings persist

### Implementation for User Story 5

- [X] T034 [US5] Implement `load_audio_config` startup system in `src/systems/audio.rs` to load from `config/audio.ron` or use defaults
- [X] T035 [US5] Implement `save_audio_config` system in `src/systems/audio.rs` to persist config on change
- [X] T036 [US5] Add `set_volume` and `toggle_mute` methods to `AudioConfig` in `src/systems/audio.rs`
- [X] T037 [US5] Create config directory at `config/` if it doesn't exist during save
- [X] T038 [US5] Add WASM-specific localStorage persistence in `src/systems/audio.rs` using conditional compilation

**Checkpoint**: Volume and mute settings work and persist across sessions

---

## Phase 8: User Story 6 - Graceful Degradation (Priority: P3)

**Goal**: Game runs without errors if audio assets are missing

**Independent Test**: Remove audio assets, run game, verify no crashes and warning logs appear

### Implementation for User Story 6

- [X] T039 [US6] Update `load_audio_assets` in `src/systems/audio.rs` to handle missing manifest gracefully (log warning, use empty map)
- [X] T040 [US6] Update `play_sound` helper in `src/systems/audio.rs` to log warning and return early if asset handle missing
- [ ] T041 [US6] Add integration test for graceful degradation in `tests/audio_events.rs`

**Checkpoint**: Game runs without crashes when audio assets are missing

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

## Dependencies & Execution Order

## CI Note: Test Isolation Recommendation

Some integration tests create temporary files under `assets/levels/` which can collide when the test suite runs in parallel.
To avoid intermittent failures in CI we recommend one of the following:

- Run tests single-threaded in CI: set `RUST_TEST_THREADS=1` in the CI job environment.
- Prefer test-local temporary files/directories (e.g., `tempfile`/`tempdir`) for any tests that
    write to shared paths like `assets/levels/`.

Either option will make the test runs more deterministic; using both (temp files + single-threaded CI) is the most robust approach.

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup - BLOCKS all user stories
- **User Stories (Phase 3-8)**: All depend on Foundational phase completion
- **Polish (Phase 9)**: Depends on all user stories being complete

### User Story Dependencies

| Story | Priority | Dependencies | Can Parallel With |
|-------|----------|--------------|-------------------|
| US1 - Brick Hit Audio | P1 | Foundational only | US2, US3 |
| US2 - Ball Bounce Audio | P1 | Foundational only | US1, US3 |
| US3 - Paddle Collision Audio | P1 | Foundational only | US1, US2 |
| US4 - Level Transition Audio | P2 | Foundational only | US1, US2, US3, US5 |
| US5 - Audio Configuration | P2 | Foundational only | US1, US2, US3, US4 |
| US6 - Graceful Degradation | P3 | US1 (needs play_sound working) | None |

### Within Each User Story

- Define events before emitting them
- Implement observers before registering them
- Register observers in AudioPlugin last

### Parallel Opportunities

**Setup Phase (all [P] tasks):**

```text
T002 manifest.ron  ||  T003 audio.rs module  ||  T004 mod.rs export
```

**Foundational Phase (all [P] resources):**

```text
T006 AudioConfig  ||  T007 AudioAssets  ||  T008 ActiveSounds
```

**User Stories (P1 stories can run in parallel):**

```text
US1 (T014-T019)  ||  US2 (T020-T025)  ||  US3 (T026-T028)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T004)
2. Complete Phase 2: Foundational (T005-T013)
3. Complete Phase 3: User Story 1 - Brick Hit Audio (T014-T019)
4. **STOP and VALIDATE**: Hit bricks, verify sounds play
5. Demo/Deploy MVP

### Recommended Order (Solo Developer)

1. Setup → Foundational → US1 (MVP) ✅
2. Add US2 (Ball Bounce) → Test
3. Add US3 (Paddle Collision) → Test
4. Add US4 (Level Transition) → Test
5. Add US5 (Configuration) → Test
6. Add US6 (Graceful Degradation) → Test
7. Polish phase → Final validation

### Parallel Team Strategy

With 3 developers after Foundational complete:

- Developer A: US1 (Brick Hit)
- Developer B: US2 (Ball Bounce)
- Developer C: US3 (Paddle Collision)

Then sequentially: US4 → US5 → US6 → Polish

---

## Notes

- All audio observers use Bevy's observer pattern (`On<Event>`)
- Max 4 concurrent sounds per type - enforced in play_sound helper
- Placeholder audio files needed for testing (silent OGG files work)
- WASM requires user interaction before audio plays - handled by existing restart-audio-context.js
- Config persistence: RON file on native, localStorage on WASM

- **T038 implemented**: WASM persistence now stores `AudioConfig` (RON) in browser `localStorage` under the key `brkrs_audio` (implemented via `web-sys` and gate-compiled for `wasm32`).
    `specs/006-audio-system/quickstart.md` documents how to reset the key.
    Native behavior (file `config/audio.ron`) remains unchanged.
