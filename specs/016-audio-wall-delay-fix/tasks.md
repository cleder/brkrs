# Tasks: Audio Wall Delay Fix

**Feature**: Audio Wall Delay Fix **Branch**: 016-audio-wall-delay-fix **Spec**: specs/016-audio-wall-delay-fix/spec.md

---

## Phase 1: Setup

- [ ] T001 Create feature branch 016-audio-wall-delay-fix
- [ ] T002 [P] Create initial test harness for wall collision audio in tests/integration/wall_audio.rs
- [ ] T003 [P] Add BallWallHit event struct to src/signals.rs

## Phase 2: Foundational

- [ ] T004 [P] Register BallWallHit event in Bevy app in src/lib.rs
- [ ] T005 [P] Add BallWallHit event emission to collision system in src/systems/physics.rs
- [ ] T006 [P] Add BallWallHit event observer system to src/systems/audio.rs

## Phase 3: User Story 1 - Immediate Wall Hit Audio

- [ ] T007 [US1] Write failing test: wall hit audio is played within 50ms of collision in tests/integration/wall_audio.rs
- [ ] T008 [US1] Implement BallWallHit event emission for every ball-wall collision in src/systems/physics.rs
- [ ] T009 [US1] Implement audio system to play wall hit sound immediately on BallWallHit event in src/systems/audio.rs
- [ ] T010 [US1] Enforce concurrency limit for wall hit sounds in src/systems/audio.rs
- [ ] T011 [US1] Log and skip audio if concurrency limit is reached in src/systems/audio.rs
- [ ] T012 [US1] Write test: multiple wall collisions in same frame trigger multiple audio events in tests/integration/wall_audio.rs
- [ ] T013 [US1] Write test: audio system logs/skips if concurrency limit is reached in tests/integration/wall_audio.rs
- [ ] T014 [US1] Write test: zero wall collisions in a frame does not play audio in tests/integration/wall_audio.rs
- [ ] T015 [US1] Write test: simultaneous ball and wall destruction does not panic in tests/integration/wall_audio.rs

## Phase 3b: Non-Functional & Documentation

- [ ] T021 [NF] Write test: wall hit audio latency is <50ms in all supported platforms (native, WASM)
- [ ] T022 [NF] Write test: no audio artifacts or overlapping issues in rapid collision scenarios
- [ ] T023 [NF] Manual test: concurrency limit is enforced and logged
- [ ] T024 [NF] Review and update rustdoc documentation for all public APIs affected by BallWallHit event and audio system

- [ ] T020 [P] Manual playtest: verify concurrency limit and logging in overload scenarios

## Dependencies

- Phase 1 → Phase 2 → Phase 3 → Phase 4
- All [P] tasks can be run in parallel within their phase
- User Story 1 tasks are independent and testable in isolation

## Parallel Execution Examples

- T002, T003 can be done in parallel
- T004, T005, T006 can be done in parallel
- T017, T018, T019, T020 can be done in parallel

## Implementation Strategy

- MVP: Complete all Phase 1–3 tasks for User Story 1
- Incremental: Polish and cross-cutting tasks in Phase 4

---

All tasks follow the strict checklist format and are independently testable.
