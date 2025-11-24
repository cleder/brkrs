---

description: "Task list for Brkrs Complete Game - User Story 1 breakdown"
---

# Tasks: Brkrs Complete Game

**Input**: Design documents from `/specs/001-complete-game/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), data-model.md, contracts/events.md, research.md, quickstart.md

**Tests**:Every function should be comprehensibly unit tested.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- [P]: Can run in parallel (different files, no dependencies)
- [Story]: Which user story this task belongs to (US1, US2, ...)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare minimal structure and constants to support User Story 1.

- [x] T001 [P] Create grid debug system file at src/systems/grid_debug.rs (module for 22x22 wireframe grid overlay)
- [x] T002 Add module wiring for systems in src/main.rs (declare `mod systems;` and `pub mod grid_debug;` as needed)
- [x] T003 [P] Define grid constants in src/main.rs: `GRID_WIDTH=22`, `GRID_HEIGHT=22`, `CELL_SIZE` (world units per cell)
- [x] T004 [P] Add GridOverlay marker component in src/main.rs to tag the grid entity
- [x] T005 [P] Configure window to launch in borderless fullscreen mode in src/main.rs: set `WindowPlugin` with `primary_window.mode = WindowMode::BorderlessFullscreen`; use conditional compilation for WASM (Windowed mode)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core components and collision hooks required for User Story 1.

- [x] T006 Ensure marker components exist in src/main.rs: `Paddle`, `Ball`, `Border` (already present) and add new `LowerGoal` marker
- [x] T007 Tag lower edge border with `LowerGoal` in `spawn_border()` in src/main.rs (the -Z or designated lower wall)
- [x] T008 Add CollisionEvent reader system stub in src/main.rs for ball/world interactions (if not already present)

**Checkpoint**: Foundation ready — US1 tasks can proceed in parallel where marked [P]

---

## Phase 3: User Story 1 - Basic Gameplay Loop (Priority: P1) 🎯 MVP

**Goal**: Player can control paddle with mouse (X/Z) and rotation (wheel); ball bounces from paddle/walls; bricks are destroyed on hit; lower wall destroys ball; grid wireframe overlay is visible only in wireframe mode to help debugging.

**Independent Test**:

- Launch the game; move mouse → paddle moves X and Z; scroll wheel → paddle rotates
- Ball collides with paddle/walls and bounces; ball collides with a brick and destroys it
- Ball collides with lower wall → ball is destroyed (despawned)
- Toggle wireframe (Space on native) → 22x22 grid overlay becomes visible; hidden when wireframe off

### Implementation Tasks

- [x] T009 [P] [US1] Spawn a sample Brick for MVP in src/main.rs within setup (position to align with grid cell at center)
- [x] T010 [US1] Handle ball-brick collision to despawn brick in src/main.rs (CollisionEvent handler)
- [x] T011 [US1] Calibrate paddle "english" impulse factor in on_paddle_ball_hit in src/main.rs (tune multiplier for noticeable but controlled steering)
- [x] T012 [P] [US1] Implement lower wall ball-destroy rule in src/main.rs: on CollisionEvent between `Ball` and entity tagged `LowerGoal` → despawn ball entity
- [x] T013 [P] [US1] Implement 22x22 grid wireframe overlay spawn in src/systems/grid_debug.rs; spawn entity with `GridOverlay` + `Visibility::Hidden`
- [x] T014 [P] [US1] Toggle grid overlay visibility in a new system (src/systems/grid_debug.rs): if wireframe enabled (WireframeConfig.global == true) → Visible; else Hidden; register system
- [x] T015 [P] [US1] Constrain paddle movement to play area: clamp transform X/Z inside bounds in move_paddle system in src/main.rs (in addition to colliders)
- [ ] T016 [P] [US1] Verify/tune mouse sensitivity and rotation responsiveness in src/main.rs for smooth control (<=100ms perceived latency)
- [x] T017 [US1] Document the grid overlay debug behavior in specs/001-complete-game/quickstart.md (wireframe toggle shows grid; hidden otherwise)

**Checkpoint**: MVP playable — brick destruction, lower-wall destroy, paddle control, wireframe grid debug works

---

## Phase N: Polish & Cross-Cutting Concerns

- [ ] T100 [P] Update README.md with quick controls, fullscreen behavior, and debug grid note
- [ ] T101 [P] Document fullscreen Alt+Enter toggle in specs/001-complete-game/quickstart.md (if not already documented)
- [ ] T102 Performance tuning pass for ball physics and input smoothing (optional)

---

## Dependencies & Execution Order

### Phase Dependencies

- Setup (Phase 1) → Foundational (Phase 2) → US1 (Phase 3)

### User Story Dependencies

- US1 depends on Foundational completion

### Within US1

- Parallelizable: T008, T011, T012, T013, T014, T015, T016
- Sequential: T009 after T008; T010 after baseline collisions confirm

---

## Parallel Example: User Story 1

```bash
# In parallel after foundational:
Task: "Spawn a sample Brick for MVP in src/main.rs within setup"
Task: "Implement lower wall ball-destroy rule in src/main.rs"
Task: "Implement 22x22 grid wireframe overlay spawn in src/systems/grid_debug.rs"
Task: "Toggle grid overlay visibility based on WireframeConfig in src/systems/grid_debug.rs"
Task: "Constrain paddle movement to play area in src/main.rs"
Task: "Tune mouse sensitivity and rotation in src/main.rs"

# Setup phase parallelizable:
Task: "Configure window fullscreen mode in src/main.rs (T005)"
Task: "Create grid debug system file (T001)"
Task: "Define grid constants (T003)"
Task: "Add GridOverlay marker component (T004)"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Setup + Foundational
2. Implement US1 tasks
3. Validate independently via manual test criteria

### Incremental Delivery

- After US1, proceed with US2 (Game State Management), then US3 (Brick Types), US4 (Level System), US5 (Visuals)
