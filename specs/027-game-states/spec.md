# Feature Specification: Game States

**Feature Branch**: `027-game-states` **Created**: 2026-02-08 **Status**: Draft **Input**: User description: "provide a main menu state where players can start a new game, support a playing state where active gameplay occurs, support a paused state that freezes gameplay, support a game over state when all lives are lost, support a level transition between levels, fade out state after life loss, fade in state before starting new level, game over state after life loss when no life is left.
Game MUST transition between states based on game events, can be triggered by messages."

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: This feature touches ECS state management, events/messages, and state transitions.
The implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include checks that:

- Verify correct use of `MessageWriter` vs observers/`Trigger<T>` for state transition events
- Guard against prohibited patterns (panicking state queries)
- Ensure state transitions are properly sequenced across frames

### User Story 1 - Main Menu Navigation (Priority: P1)

Players launch the game and must see a main menu where they can start a new game.
This is the critical entry point for all gameplay.

**Why this priority**: This is the foundational entry state.
Without it, players cannot enter the game world.

**Independent Test**: Can be tested by launching the game, verifying main menu renders, and simulating a "start game" action that transitions to the Playing state.

**Acceptance Scenarios**:

1. **Given** the game is launched, **When** the application initializes, **Then** the Main Menu state is active
2. **Given** the Main Menu state is active, **When** player selects "New Game", **Then** a state transition message is sent and the Playing state becomes active
3. **Given** the Main Menu state is active, **When** player selects "New Game", **Then** gameplay systems are enabled and the first level is loaded

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

Players can pause the game mid-level, freezing all gameplay mechanics, then resume from the same point.
Physics, animations, and input must all freeze when paused.

**Why this priority**: Pause is essential for accessibility and player control.
Players need to step away from the game without losing progress.

**Independent Test**: Can be tested by transitioning to Paused state and verifying that physics, animations, and input no longer process, then resuming and confirming state restoration.

**Acceptance Scenarios**:

1. **Given** the Playing state is active, **When** pause message is sent, **Then** the Paused state becomes active
2. **Given** the Paused state is active, **When** a frame updates, **Then** physics simulation does NOT run
3. **Given** the Paused state is active, **When** player provides input, **Then** paddle does NOT respond
4. **Given** the Paused state is active, **When** resume message is sent, **Then** the Playing state becomes active with all previous state intact
5. **Given** the game was paused for multiple frames, **When** the game resumes, **Then** entity positions and velocities reflect the state before the pause message

---

### User Story 4 - Level Transition (Priority: P1)

When a player completes a level, the game transitions to a Level Transition state that includes fade-out of the completed level, level change, and fade-in of the new level.

**Why this priority**: Level progression is core to the game structure.
Players must be able to advance to subsequent levels with a smooth transition.

**Independent Test**: Can be tested by completing a level and verifying the fade-out → load-next-level → fade-in sequence occurs correctly.

**Acceptance Scenarios**:

1. **Given** a level is completed, **When** the level-complete message is sent, **Then** the Fade Out state becomes active
2. **Given** the Fade Out state is active, **When** fade-out animation completes, **Then** the next level is loaded and the Fade In state becomes active
3. **Given** the Fade In state is active, **When** fade-in animation completes, **Then** the Playing state becomes active with the new level
4. **Given** fade-out and fade-in states are triggered in sequence, **When** the transitions complete, **Then** the game state reflects the new level with correct entities and no orphaned state from the previous level

---

### User Story 5 - Life Loss Handling (Priority: P1)

When the player loses a life but has lives remaining, the game enters Fade Out state, respawns the ball, and fades back in.
If no lives remain, the game enters Game Over state.

**Why this priority**: Life loss is a core game mechanic that players encounter frequently.
Proper handling is essential for game feel and progression.

**Independent Test**: Can be tested by triggering a life loss with lives remaining, verifying fade-out → respawn → fade-in, then testing with zero lives to verify Game Over state.

**Acceptance Scenarios**:

1. **Given** the Playing state is active with 1+ lives remaining, **When** ball-lost message is sent, **Then** the Fade Out state becomes active
2. **Given** the Fade Out state is active after life loss, **When** fade-out completes, **Then** ball is respawned, the Fade In state becomes active, and lives count decreases by 1
3. **Given** the Fade In state is active after life loss, **When** fade-in completes, **Then** the Playing state becomes active with the same level
4. **Given** the Playing state is active with 0 lives remaining, **When** ball-lost message is sent, **Then** the Game Over state becomes active directly (no respawn)

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
2. **Given** the Game Over state is active, **When** "return to menu" message is sent, **Then** the Main Menu state becomes active
3. **Given** the Game Over state is active, **When** "new game" message is sent, **Then** the Main Menu state becomes active and a new game session begins

---

### Edge Cases

- What happens if a state transition message is received while a fade animation is in progress? (Should queue or ignore based on state hierarchy)
- How does the system handle rapid pause/resume messages in succession?
- What happens if the level-complete message is received while the game is paused? (Should transition to Fade Out or remain paused?)
- Does the game properly clean up all entities from the previous level before loading the next level to prevent memory leaks?
- What happens if a state transition message is invalid or targets a non-existent state? (Should log error and remain in current state)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support a Main Menu state that is active on game launch
- **FR-002**: System MUST support a Playing state where physics simulation, entity updates, input processing, and collision detection all run
- **FR-003**: System MUST support a Paused state that freezes physics simulation, entity movement, input processing, and animations
- **FR-004**: System MUST support a Fade Out state that plays a fade-out animation and blocks normal gameplay
- **FR-005**: System MUST support a Fade In state that plays a fade-in animation before returning to Playing
- **FR-006**: System MUST support a Level Transition state sequence: Fade Out → load next level → Fade In → Playing
- **FR-007**: System MUST support a Game Over state that is active when lives reach zero
- **FR-008**: System MUST define state transitions triggered by game events/messages, including: "start-game", "pause-game", "resume-game", "level-complete", "ball-lost", "return-to-menu", "new-game"
- **FR-009**: System MUST use message-based architecture to trigger state transitions (Messages or Observers as appropriate)
- **FR-010**: System MUST prevent state transitions that are invalid (e.g., cannot pause from Main Menu)
- **FR-011**: System MUST ensure state transitions are idempotent (sending the same transition message twice must not cause unexpected behavior)
- **FR-012**: System MUST properly disable/enable relevant systems when entering/exiting states (e.g., input processing only active in Playing/Paused, physics only active in Playing)
- **FR-013**: System MUST preserve game state when transitioning from Playing → Paused and back to Playing
- **FR-014**: System MUST preserve level state when transitioning from Playing → Level Transition → Playing with the next level (entities from previous level cleaned up)
- **FR-015**: System MUST support resuming from Paused state at the exact same game state without player-initiated movement or physics changes

### Key Entities

- **GameState** (enum): Represents the current game state - Main Menu, Playing, Paused, Fade Out, Fade In, Level Transition, Game Over
- **StateTransitionEvent** (message/event): Carries information about requested state transitions, including the target state and optional context (e.g., level index for Level Transition)
- **Game Session**: Tracks current level, remaining lives, score, and other runtime data that persists across state transitions within a single play session

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: State transitions occur within 1 frame of receiving the transition message (no delayed state changes)
- **SC-002**: Paused state reliably freezes all gameplay: physics does not advance, entities do not move, input does not affect game state across a minimum of 10 consecutive frames
- **SC-003**: Game resumes from Paused state with 100% state accuracy (all entity positions, velocities, and component states match the pre-pause state)
- **SC-004**: Fade animations (Fade Out and Fade In) complete in 0.5-1.0 seconds without interrupting state transition logic
- **SC-005**: Level transition sequence (Fade Out → Load → Fade In) completes without entity orphaning or memory leaks (verified by profiling entity count before and after transition)
- **SC-006**: State machine correctly rejects invalid transitions (e.g., pause from Main Menu), logging errors and maintaining current state
- **SC-007**: Game Over state properly disables all gameplay systems immediately upon activation
- **SC-008**: New game can be started from Main Menu or Game Over state with a clean slate (no residual state from previous session)

## Assumptions

- State transitions use Bevy's message system (MessageWriter/MessageReader) or Observers (TBD: clarified below)
- Fade Out/Fade In animations take approximately 0.5-1.0 seconds
- All state-related entity spawning/despawning is idempotent (spawning the same entity twice does not cause duplicates)
- Level loading includes automatic despawn of all level-specific entities from the previous level
- The pause state preserves physics bodies and queries such that resumption requires no re-initialization
