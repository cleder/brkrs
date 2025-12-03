# Feature Specification: Ball Respawn System

**Feature Branch**: `002-ball-respawn` **Created**: 2025-11-24 **Status**: Draft **Input**: User description: "implement a ball respawn when a ball is despawned. respawn the ball at its initial position as specified in the level matrix, move the paddle to its initial position as specified in the matrix"

## Clarifications

### Session 2025-11-25

- Q: After the 1 second respawn delay, does the ball auto-launch or stay inert until the player acts? → A: Ball is stationary at spawn
- Q: Should the respawn feature decrement lives internally or emit an event for the lives system? → A: Emit LifeLost event
- Q: During the 1 second respawn delay, should paddle controls stay active? → A: Disable controls during delay
- Q: Can levels override respawn positions independently from the level matrix? → A: Always use matrix positions
- Q: What timing mechanism enforces the 1 second respawn delay? → A: Use global Time resource

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Ball Respawn After Loss (Priority: P1)

When a player loses the ball (it falls past the lower boundary), the game automatically respawns both the ball and paddle at their initial positions defined in the level matrix, allowing the player to continue playing without manual intervention.

**Why this priority**: This is core gameplay functionality - without respawn, the game ends after a single ball loss, making it unplayable.
This is the minimum viable feature.

**Independent Test**: Can be fully tested by launching the game, letting the ball fall past the lower boundary, and verifying that both ball and paddle return to their starting positions.
Delivers continuous gameplay.

**Acceptance Scenarios**:

1. **Given** the ball is in play, **When** the ball collides with the lower goal boundary and is despawned, **Then** a new ball spawns at position "2" from the level matrix
2. **Given** the ball is in play, **When** the ball collides with the lower goal boundary and is despawned, **Then** the paddle moves to position "1" from the level matrix and controls remain disabled until respawn completes
3. **Given** the ball and paddle have respawned, **When** respawn completes, **Then** the ball remains stationary (zero velocity) until the player relaunches it
4. **Given** a ball respawn occurs, **When** checking player lives, **Then** the life count decreases by one (assuming lives system exists)

---

### User Story 2 - Multiple Respawn Handling (Priority: P2)

When a player loses multiple balls in succession, each respawn correctly resets positions and maintains game state, ensuring consistent behavior across repeated ball losses.

**Why this priority**: Ensures the respawn system works reliably across the entire game session, not just the first occurrence.
Critical for gameplay quality but dependent on P1 working first.

**Independent Test**: Can be tested by intentionally losing the ball multiple times and verifying consistent respawn behavior each time.
Delivers reliable repeated gameplay.

**Acceptance Scenarios**:

1. **Given** the player has lost the ball once and it has respawned, **When** the player loses the ball again, **Then** the respawn process executes correctly a second time
2. **Given** the player is on their last life, **When** the ball is lost, **Then** game over state triggers instead of respawn
3. **Given** multiple balls exist in play (if multi-ball power-up exists), **When** one ball is lost, **Then** only that ball respawns while others continue

---

### User Story 3 - Respawn Visual Feedback (Priority: P3)

When a respawn occurs, the player receives clear visual feedback indicating the reset, such as a brief pause, fade effect, or repositioning animation.

**Why this priority**: Improves user experience and clarity but is not essential for core functionality.
Can be added after basic respawn mechanics work.

**Independent Test**: Can be tested by observing the respawn sequence and confirming visual indicators are present.
Delivers polished gameplay experience.

**Acceptance Scenarios**:

1. **Given** a ball respawn is triggered, **When** entities are repositioned, **Then** a brief visual indicator (flash, fade, or pause) signals the reset
2. **Given** respawn visual feedback is playing, **When** the animation completes, **Then** gameplay resumes with normal controls enabled

---

### Edge Cases

- What happens when the ball is despawned due to reasons other than lower goal collision (e.g., manual despawn, game state change)?
- How does the system handle respawn if the level matrix positions "1" or "2" are not defined or invalid?
- What happens if multiple balls are despawned simultaneously?
- How does respawn interact with the lives system when the player is on their final life?
- What happens if the paddle or ball positions in the level matrix conflict with existing entities (e.g., bricks)?
- How does respawn behave during level transitions or when the game is paused?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST detect when a ball entity is despawned due to lower goal collision
- **FR-002**: System MUST retrieve the ball respawn position directly from the "2" cell in the level matrix
- **FR-003**: System MUST retrieve the paddle respawn position directly from the "1" cell in the level matrix
- **FR-004**: System MUST spawn a new ball entity at the retrieved initial position with default physics properties
- **FR-005**: System MUST move the existing paddle entity to the retrieved initial position
- **FR-006**: System MUST reset paddle velocity and rotation to initial state during respawn
- **FR-007**: System MUST reset ball velocity to zero and keep the ball stationary until the player relaunches it
- **FR-008**: System MUST handle cases where level matrix positions "1" or "2" are missing by using fallback default positions (center of play area); levels cannot override respawn coordinates elsewhere
- **FR-009**: System MUST emit a `LifeLost` event before triggering respawn so the lives system can decrement counts
- **FR-010**: System MUST respect a `GameOver` signal from the lives system and skip respawn when zero lives remain
- **FR-011**: System MUST maintain game state (score, brick destruction, level progress) across respawns
- **FR-012**: Respawn process MUST include a 1 second delay, tracked via Bevy's global `Time` resource, between ball despawn and repositioning to give players time to register the loss; paddle and launch controls remain disabled for this duration

### Key Entities

- **Ball**: Game entity representing the ball; has position, velocity, physics properties; spawned at matrix position "2"
- **Paddle**: Player-controlled entity; has position, velocity, rotation; repositioned to matrix position "1" on respawn
- **Level Matrix**: 22x22 grid definition containing entity spawn positions; "1" designates paddle start, "2" designates ball start
- **Lives**: Player resource tracking remaining attempts; decremented on ball loss, triggers game over at zero
- **Respawn Event**: System event triggered when ball despawn occurs; coordinates position reset and entity state restoration

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Ball and paddle successfully return to initial positions within 1 second of ball despawn event
- **SC-002**: Respawn system handles 100 consecutive ball losses without errors or performance degradation
- **SC-003**: Player can continue gameplay immediately after respawn with full control restored
- **SC-004**: Respawn correctly decrements lives counter 100% of the time when integrated with lives system
- **SC-005**: Game correctly transitions to game over state when final life is lost (0% incorrect respawns on last life)
- **SC-006**: Initial positions from level matrix are accurately retrieved and applied 100% of the time
- **SC-007**: Fallback positions are used successfully when matrix positions are invalid or missing

## Known Limitations & Follow-Ups *(Phase 6)*

1. **Grid overlay visibility warning** – `bevy lint` / `cargo clippy` warn that `GridOverlay` is `pub(crate)` while `toggle_grid_visibility` is `pub`.
   Nothing fails at runtime, but we should either widen the struct visibility or reduce the helper function to crate scope before feature freeze.
2. **Unused monitor import in WASM** – `cargo build --target wasm32-unknown-unknown --release` currently warns about an unused `MonitorSelection` import (only required for desktop-exclusive window handling).
   Feature-gate or remove the import to keep WASM builds clean.
3. **Structured logging scope** – Respawn events emit structured `tracing` spans/logs (`life_lost`, `respawn_scheduled`, `game_over_requested`), but the rest of the gameplay stack still relies on ad-hoc `log` macros.
   Future polish should migrate other critical systems or install a shared `tracing-subscriber` configuration so downstream tooling can consume the richer fields.
