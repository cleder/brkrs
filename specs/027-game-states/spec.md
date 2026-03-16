# Feature Specification: Game States

**Feature Branch**: `027-game-states` **Created**: 2026-02-08 **Status**: Draft **Input**: User description: "provide a main menu state where players can start a new game, support a playing state where active gameplay occurs, support a paused state that freezes gameplay, support a game over state when all lives are lost, support a level transition between levels, fade out state after life loss, fade in state before starting new level, game over state after life loss when no life is left.
Game MUST transition between states based on game events."

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: This feature touches ECS state management and state transitions.
The implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include checks that:

- Verify correct use of States derive with `NextState<GameState>` for state transitions
- Guard against prohibited patterns (panicking state queries)
- Ensure state transitions are properly sequenced across frames

### User Story 1 - Main Menu Navigation (Priority: P1)

Players launch the game and must see a main menu where they can start a new game.
This is the critical entry point for all gameplay.
The main menu provides two options: "New Game" (transitions to Playing) and "Quit" (exits the application).

**Why this priority**: This is the foundational entry state.
Without it, players cannot enter the game world.

**Independent Test**: Can be tested by launching the game, verifying main menu renders with New Game and Quit buttons, and simulating a "New Game" action that transitions to the Playing state.

**Acceptance Scenarios**:

1. **Given** the game is launched, **When** the application initializes, **Then** the Main Menu state is active
2. **Given** the Main Menu state is active, **When** player selects "New Game", **Then** NextState(GameState::Playing) is set and the Playing state becomes active
3. **Given** the Main Menu state is active, **When** player selects "New Game", **Then** gameplay systems are enabled and the first level is loaded
4. **Given** the Main Menu state is active, **When** player selects "Quit", **Then** the application cleanly shuts down

---

### User Story 2 - Active Gameplay (Priority: P1)

Players engage in active gameplay where the ball moves, bricks respond to collisions, and the paddle can be controlled.
The Playing state must allow normal game mechanics.

**Why this priority**: This is the core game loop.
Without active gameplay working, the feature has no value.

**Independent Test**: Can be tested by transitioning to Playing state, verifying physics and input systems are active, and confirming game entities respond to events.

**Acceptance Scenarios**:

1. **Given** the Playing state is active, **When** a frame updates, **Then** physics simulation runs and entities move
2. **Given** the Playing state is active, **When** player provides input, **Then** paddle responds to control
3. **Given** the Playing state is active, **When** ball collides with brick, **Then** collision callbacks execute and brick responds
4. **Given** the Playing state is active across multiple frames, **Then** all game mechanics (movement, collisions, scoring) persist across 10+ consecutive frames

---

### User Story 3 - Pause/Resume Gameplay (Priority: P1)

Players can pause the game mid-level from the Playing state, freezing all gameplay mechanics, then resume from the same point.
Physics, animations, and input must all freeze when paused.
Pause is only valid from the Playing state; pause requests from other states (e.g., Main Menu, Fade Out) are invalid and logged as warnings.

**Why this priority**: Pause is essential for accessibility and player control.
Players need to step away from the game without losing progress.

**Independent Test**: Can be tested by transitioning to Playing state, pausing, and verifying that physics, animations, and input no longer process, then resuming and confirming state restoration.
Also test that pause from other states is rejected.

**Acceptance Scenarios**:

1. **Given** the Playing state is active, **When** NextState(GameState::Paused) is set, **Then** the Paused state becomes active
2. **Given** the Paused state is active, **When** a frame updates, **Then** physics simulation does NOT run
3. **Given** the Paused state is active, **When** player provides input, **Then** paddle does NOT respond
4. **Given** the Paused state is active, **When** NextState(GameState::Playing) is set, **Then** the Playing state becomes active with all previous state intact
5. **Given** the game was paused for multiple frames, **When** the game resumes, **Then** entity positions and velocities reflect the state before pausing
6. **Given** the Main Menu state is active, **When** pause is requested, **Then** the state remains Main Menu and a warning is logged

---

### User Story 4 - Level Transition (Priority: P1)

When a player completes a level, the game transitions to a Level Transition state that includes fade-out of the completed level, level change, and fade-in of the new level.

**Why this priority**: Level progression is core to the game structure.
Players must be able to advance to subsequent levels with a smooth transition.

**Independent Test**: Can be tested by completing a level and verifying the fade-out → load-next-level → fade-in sequence occurs correctly.

**Acceptance Scenarios**:

1. **Given** a level is completed, **When** NextState(GameState::FadeOut) is set with LevelChange context, **Then** the Fade Out state becomes active
2. **Given** the Fade Out state is active, **When** fade-out animation completes, **Then** the next level is loaded and the Fade In state becomes active
3. **Given** the Fade In state is active, **When** fade-in animation completes, **Then** the Playing state becomes active with the new level
4. **Given** fade-out and fade-in states are triggered in sequence, **When** the transitions complete, **Then** the game state reflects the new level with correct entities and no orphaned state from the previous level

---

### User Story 5 - Life Loss Handling (Priority: P1)

When the player loses a life, the game **always** enters Fade Out state.
During the Fade Out animation, no decision is made.
After the fade-out animation completes, a lives check determines the next transition: if lives remain, the ball respawns and Fade In plays; if no lives remain, the Game Over state activates.

**Why this priority**: Life loss is a core game mechanic that players encounter frequently.
Proper handling is essential for game feel and progression.

**Independent Test**: Can be tested by triggering a life loss with lives remaining, verifying fade-out → respawn → fade-in, then testing with zero lives to verify Game Over state after fade completes.

**Acceptance Scenarios**:

1. **Given** the Playing state is active with 1+ lives remaining, **When** NextState(GameState::FadeOut) is set with LifeLoss context (triggered by ball lost, paddle-brick 42/91 collision, or paddle-merkaba collision), **Then** the Fade Out state becomes active immediately and all merkabas and remaining balls are despawned
2. **Given** the Fade Out state is active after life loss, **When** fade-out animation completes, **Then** a lives check occurs
3. **Given** the Fade Out state's animation completes with 1+ lives remaining, **When** the lives check concludes, **Then** the ball is respawned and the Fade In state becomes active
4. **Given** the Fade In state is active after life loss, **When** fade-in completes, **Then** the Playing state becomes active with the same level and lives count decreased by 1
5. **Given** the Fade Out state's animation completes with 0 lives remaining, **When** the lives check concludes, **Then** the Game Over state becomes active (no respawn, no Fade In)

---

### User Story 6 - Game Over State (Priority: P2)

When all lives are lost, the game enters Game Over state.
From here, the player can either return to the main menu or start a new game.
The Game Over state communicates the loss clearly.

**Why this priority**: Game Over is important for closure and flow, but secondary to core gameplay.
The player has already lost, so this state is reached less frequently than Playing.

**Independent Test**: Can be tested by reaching zero lives and verifying Game Over state renders appropriately, then testing transitions back to Main Menu.

**Acceptance Scenarios**:

1. **Given** the Game Over state is active, **When** a frame updates, **Then** gameplay systems remain disabled (physics, input, collision response do not run)
2. **Given** the Game Over state is active, **When** NextState(GameState::MainMenu) is set, **Then** the Main Menu state becomes active
3. **Given** the Game Over state is active, **When** "new game" is requested, **Then** the Main Menu state becomes active and a new game session begins

---

### Edge Cases

#### EC-001: State Transition During Fade Animation

**Behavior**: Transition requests during FadeOut or FadeIn are ignored and logged as warnings.
The fade animation must complete before the next state becomes active.

**Rationale**: Fade states represent atomic transitions; interrupting them would create undefined intermediate states.

**Acceptance Scenarios**:

1. **Given** FadeOut is active with 0.3s remaining, **When** NextState(GameState::Paused) is set, **Then** the request is ignored, a warning is logged, and FadeOut continues
2. **Given** FadeIn is active, **When** any NextState is set, **Then** the request is ignored, a warning is logged, and FadeIn continues to completion

---

#### EC-002: Rapid Pause/Resume Requests

**Behavior**: Pause/resume requests are idempotent.
Multiple pause requests while already Paused (or multiple resume requests while Playing) are no-ops without state changes.

**Rationale**: Prevents state thrashing from input spam or rapid user actions.

**Acceptance Scenarios**:

1. **Given** Playing state is active, **When** NextState(GameState::Paused) is set 3 times in succession, **Then** only the first request triggers a transition; subsequent requests are no-ops
2. **Given** Paused state is active, **When** NextState(GameState::Playing) is set 3 times in succession, **Then** only the first request triggers a transition; subsequent requests are no-ops
3. **Given** transitions occur Paused→Playing→Paused→Playing over 4 frames, **Then** entity state after final Playing matches the original pre-pause state

---

#### EC-003: Level Complete While Paused

**Behavior**: Level completion is blocked while Paused.
The level-complete trigger is deferred until Playing resumes, then FadeOut with LevelChange context activates.

**Rationale**: Level transitions should only occur during active gameplay to maintain consistent state semantics.

**Acceptance Scenarios**:

1. **Given** Paused state is active with 0 bricks remaining, **When** level-complete is triggered, **Then** the transition is deferred and a warning is logged
2. **Given** level-complete was deferred while Paused, **When** NextState(GameState::Playing) is set, **Then** FadeOut with LevelChange context activates immediately

---

#### EC-004: Entity Cleanup on Level Transition

**Behavior**: All level-specific entities (bricks, power-ups, merkabas, balls) MUST be despawned during OnExit(GameState::LevelTransition) before the next level loads.
The system verifies entity count matches expected baseline.

**Rationale**: Prevents memory leaks and state corruption from orphaned entities.

**Acceptance Scenarios**:

1. **Given** LevelTransition completes with next level loaded, **When** FadeIn starts, **Then** entity count matches baseline (no bricks/powerups/enemies from previous level)
2. **Given** level 1 had 50 bricks and level 2 has 30 bricks, **When** transition completes, **Then** exactly 30 brick entities exist (50 were despawned)

---

#### EC-005: Invalid State Transition Requests

**Behavior**: Invalid transitions (e.g., MainMenu→FadeOut, Paused→GameOver, Playing→MainMenu without intermediate states) are rejected.
The system logs an error with details and maintains the current state.

**Rationale**: Enforces state machine contract and prevents undefined behavior from invalid paths.

**Acceptance Scenarios**:

1. **Given** MainMenu is active, **When** NextState(GameState::FadeOut) is set, **Then** the transition is rejected, an error is logged, and MainMenu remains active
2. **Given** Paused is active, **When** NextState(GameState::GameOver) is set, **Then** the transition is rejected, an error is logged, and Paused remains active
3. **Given** any state is active, **When** an invalid transition is attempted, **Then** the error log includes source state, target state, and valid transitions from source

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support a Main Menu state that is active on game launch
- **FR-002**: System MUST support a Playing state where physics simulation, entity updates, input processing, and collision detection all run
- **FR-003**: System MUST support a Paused state that freezes physics simulation, entity movement, input processing, and animations
- **FR-004**: System MUST support a Fade Out state that plays a fade-out animation and blocks normal gameplay
- **FR-005**: System MUST support a Fade In state that plays a fade-in animation before returning to Playing
- **FR-006**: System MUST support a Level Transition state sequence: Fade Out → load next level → Fade In → Playing
- **FR-007**: System MUST support a Game Over state that is active when lives reach zero
- **FR-008**: System MUST define state transitions triggered by game events, including: "start-game", "pause-game", "resume-game", "level-complete", "ball-lost", "return-to-menu", "new-game"
- **FR-009**: System MUST use Bevy's States derive with `NextState<GameState>` for state transitions (idiomatic Bevy state management for critical state changes)
- **FR-010**: System MUST prevent state transitions that are invalid (e.g., cannot pause from Main Menu), log them as warnings, and maintain the current state
- **FR-011**: System MUST ensure state transitions are idempotent (requesting the same transition twice must not cause unexpected behavior)
- **FR-012**: System MUST properly disable/enable relevant systems when entering/exiting states (e.g., input processing only active in Playing/Paused, physics only active in Playing)
- **FR-013**: System MUST preserve game state when transitioning from Playing → Paused and back to Playing
- **FR-014**: System MUST preserve level state when transitioning from Playing → Level Transition → Playing with the next level (entities from previous level cleaned up)
- **FR-015**: System MUST support resuming from Paused state at the exact same game state without player-initiated movement or physics changes
- **FR-016**: System MUST perform lives check **after** the Fade Out animation completes and branch based on remaining lives: if lives remain → respawn ball and transition to Fade In; if no lives remain → transition to Game Over
- **FR-017**: System MUST only allow pause requests from the Playing state; pause requests from other states (Main Menu, Paused, Fade Out, Fade In, Level Transition, Game Over) are invalid and logged as warnings
- **FR-018**: System MUST ignore state transition requests during FadeOut or FadeIn states, log warnings, and allow fade animations to complete (EC-001)
- **FR-019**: System MUST handle rapid pause/resume requests idempotently without state thrashing (EC-002)
- **FR-020**: System MUST defer level-complete triggers while Paused and activate FadeOut with LevelChange context upon resuming to Playing (EC-003)
- **FR-021**: System MUST despawn all level-specific entities (bricks, power-ups, merkabas, balls) during OnExit(LevelTransition) and verify entity count matches baseline (EC-004)
- **FR-022**: System MUST reject invalid state transitions with error logs containing source state, target state, and valid transitions (EC-005)

### Key Entities

- **GameState** (enum with States derive): Represents the current game state - Main Menu, Playing, Paused, Fade Out, Fade In, Level Transition, Game Over
- **StateTransitionContext** (Resource): Optional context for state transitions, carrying information like LifeLoss or LevelChange { target_level }
- **GameSession** (Resource): Tracks current level, remaining lives, score, and other runtime data that persists across state transitions within a single play session

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: State transitions occur within 1 frame of setting NextState (no delayed state changes)
- **SC-002**: Paused state reliably freezes all gameplay: physics does not advance, entities do not move, input does not affect game state across a minimum of 10 consecutive frames
- **SC-003**: Game resumes from Paused state with 100% state accuracy (all entity positions, velocities, and component states match the pre-pause state)
- **SC-004**: Fade animations (Fade Out and Fade In) complete in 0.5-1.0 seconds without interrupting state transition logic
- **SC-005**: Level transition sequence (Fade Out → Load → Fade In) completes without entity orphaning or memory leaks (verified by profiling entity count before and after transition)
- **SC-006**: State machine correctly rejects invalid transitions (e.g., pause from Main Menu), logging errors and maintaining current state
- **SC-007**: Game Over state properly disables all gameplay systems immediately upon activation
- **SC-008**: New game can be started from Main Menu or Game Over state with a clean slate (no residual state from previous session)

## Assumptions

- State transitions use Bevy's States system with NextState<GameState> for explicit state changes
- Fade Out/Fade In animations take approximately 0.5-1.0 seconds
- All state-related entity spawning/despawning is idempotent (spawning the same entity twice does not cause duplicates)
- Level loading includes automatic despawn of all level-specific entities from the previous level
- The pause state preserves physics bodies and queries such that resumption requires no re-initialization

## Clarifications

### Session 2026-02-08

- Q: Which system for state transitions?
  → A: Bevy's States derive with NextState<GameState> (idiomatic Bevy 0.17)
- Q: When does lives check occur during life loss flow?
  → A: After fade-out animation completes (during Fade Out state exit)
- Q: What should the Main Menu support?
  → A: Minimal (New Game, Quit buttons only)
- Q: How to handle invalid state transitions?
  → A: Log as warning and ignore (no state change)
- Q: Should pause be accessible from states other than Playing?
  → A: No, pause is only valid from Playing state
