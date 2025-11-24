# Feature Specification: Brkrs Complete Game

**Feature Branch**: `001-complete-game`
**Created**: 2025-10-31
**Status**: Draft
**Input**: User description: "Brkrs is a classic Breakout/Arkanoid style game implemented in Rust with the Bevy game engine. It's a feature-rich clone with advanced gameplay mechanics beyond the basic Breakout formula. It features a paddle that can be controlled with the mouse, in all directions (left/right (x), up/down (y)). If the player is moving the paddle to the right when the ball makes contact, the game calculates a greater horizontal velocity component in the rightward direction, sending the ball off at a sharper horizontal angle. Conversely, moving the paddle to the left imparts a leftward 'english.' The mouse wheel controls the rotation of the paddle. It uses 3D rendering to display the bricks, the walls, and the ball. The game will be implemented in 3D but constrained to a 2D plane above the ground. The game area is divided into a 22x22 grid, the stones are placed into this grid and fill a grid cell."

## Clarifications

### Session 2025-11-24

- Q: What should happen when the ball hits a multi-hit brick? The spec mentions "durability" but doesn't specify the reflection behavior. → A: Ball always reflects normally; only durability changes
- Q: How many lives should the player start with, and where should the ball respawn after being lost? → A: 3 lives; ball respawns at position designated by "2" in level matrix; paddle respawns at position designated by "1" in level matrix
- Q: What brick types should count toward level completion? The spec mentions 37 brick types but doesn't clarify which must be destroyed to advance. → A: Only destructible bricks count; indestructible bricks can remain
- Q: How should mouse movement speed affect paddle movement? → A: Velocity-based (current implementation: mouse delta * 0.0004 / delta_time creates proportional velocity)
- Q: What maximum ball velocity should be enforced? → A: Ball-type dependent; smaller balls (golf ball) can reach higher speeds, larger balls (beach ball) have lower max speed

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Gameplay Loop (Priority: P1)

A player launches the game, starts playing with mouse-controlled paddle movement, and experiences the core ball-paddle-brick physics interaction with mouse-based ball steering.

**Why this priority**: This is the absolute core of the game - without basic paddle control, ball physics, and brick destruction, there is no game. This MVP provides the fundamental gameplay experience.

**Independent Test**: Can be fully tested by launching the game, moving the paddle with the mouse (X and Y directions), hitting the ball, and destroying at least one brick. Delivers a playable Breakout experience.

**Acceptance Scenarios**:

1. **Given** the game is launched, **When** the player moves the mouse left/right, **Then** the paddle moves horizontally (X-axis) in the corresponding direction
2. **Given** the game is launched, **When** the player moves the mouse up/down, **Then** the paddle moves vertically (Y-axis) in the corresponding direction
3. **Given** the game is launched, **When** the player scrolls the mouse wheel, **Then** the paddle rotates around its center point
4. **Given** the paddle is moving right, **When** the ball collides with the paddle, **Then** the ball receives additional rightward velocity (english effect)
5. **Given** the paddle is moving left, **When** the ball collides with the paddle, **Then** the ball receives additional leftward velocity (english effect)
6. **Given** the ball is in play, **When** the ball collides with a brick, **Then** the brick is destroyed and removed from the play area
7. **Given** the ball is in play, **When** the ball collides with a wall, **Then** the ball bounces according to physics reflection
8. **Given** the ball exits the play area at the bottom, **When** the ball passes the lower boundary, **Then** the player loses a life and the ball respawns at position "2" from the level matrix (paddle respawns at position "1")

---

### User Story 2 - Game State Management (Priority: P2)

A player navigates through different game states including starting, pausing, game over, and progressing through levels.

**Why this priority**: Essential for a complete game experience, but the core gameplay (P1) must work first. This adds game flow and progression.

**Independent Test**: Can be tested by starting a game, pausing/resuming, losing all lives to trigger game over, and completing a level to advance. Delivers a structured game experience with proper state transitions.

**Acceptance Scenarios**:

1. **Given** the game is launched, **When** the player is at the main menu, **Then** they can start a new game
2. **Given** a game is in progress, **When** the player presses the pause key, **Then** the game pauses and displays a pause menu
3. **Given** the game is paused, **When** the player resumes, **Then** gameplay continues from the paused state
4. **Given** the player has lost all lives, **When** the last ball is lost, **Then** the game displays a game over screen
5. **Given** all bricks in a level are destroyed, **When** the last brick is destroyed, **Then** the game transitions to the next level
6. **Given** the player completes the final level, **When** the last brick of level 77 is destroyed, **Then** the game displays a victory/completion screen

---

### User Story 3 - Multiple Brick Types (Priority: P3)

A player encounters different brick types with unique behaviors, adding variety and challenge to the gameplay.

**Why this priority**: Enhances gameplay depth and variety, but requires core gameplay (P1) and level progression (P2) to be functional first. Delivers the "feature-rich" aspect beyond basic Breakout.

**Independent Test**: Can be tested by playing a level containing different brick types and observing varied behaviors (different hit counts, special effects, etc.). Delivers enhanced gameplay variety.

**Acceptance Scenarios**:

1. **Given** the ball hits a standard brick, **When** collision occurs, **Then** the brick is destroyed in one hit
2. **Given** the ball hits a multi-hit brick, **When** collision occurs, **Then** the ball reflects normally (standard physics), the brick's durability decreases by 1, and visual state changes to indicate remaining durability
3. **Given** the ball hits a multi-hit brick with remaining durability, **When** collision occurs again, **Then** the brick is eventually destroyed after sufficient hits (each hit reduces durability by 1)
4. **Given** the ball hits a special brick type, **When** collision occurs, **Then** unique behavior triggers (velocity modification, special effects, etc.)
5. **Given** bricks are arranged in a 22x22 grid, **When** viewing the play area, **Then** bricks properly fill grid cells without overlap

---

### User Story 4 - Level System (Priority: P4)

A player progresses through 77 unique levels with different brick layouts and challenges.

**Why this priority**: Provides long-term engagement and replayability, but depends on core gameplay (P1), state management (P2), and brick variety (P3) being complete.

**Independent Test**: Can be tested by loading different level definitions and verifying correct brick placement and progression. Delivers extensive content and replay value.

**Acceptance Scenarios**:

1. **Given** a level is loaded, **When** the level starts, **Then** bricks are spawned according to the level's layout definition
2. **Given** multiple levels exist, **When** progressing through the game, **Then** each level presents a unique brick arrangement
3. **Given** the player is on level N, **When** completing that level, **Then** the game loads level N+1
4. **Given** level progress, **When** viewing the game, **Then** the current level number is displayed to the player

---

### User Story 5 - Visual Presentation (Priority: P5)

A player experiences a visually appealing 3D-rendered game with proper lighting, shadows, and camera perspective.

**Why this priority**: Enhances player experience and game polish, but all core gameplay mechanics must be functional first. Delivers professional visual quality.

**Independent Test**: Can be tested by launching the game and observing 3D models, lighting effects, shadows, and overhead camera view. Delivers polished visual presentation.

**Acceptance Scenarios**:

1. **Given** the game is running, **When** viewing the play area, **Then** all game objects (paddle, ball, bricks, walls) are rendered as 3D models
2. **Given** the game is running, **When** viewing the scene, **Then** lighting creates visible shadows and depth
3. **Given** all gameplay occurs on a 2D plane, **When** viewing the game, **Then** the camera is positioned above looking down at the play area
4. **Given** 3D objects exist, **When** they are rendered, **Then** they maintain 3D visual aesthetics while gameplay remains constrained to Y=2.0 plane
5. **Given** the game is running on native platform, **When** toggling wireframe mode, **Then** the visualization changes to wireframe rendering
6. **Given** the game is running on WASM, **When** playing in a web browser, **Then** the game renders correctly without wireframe support

---

### Edge Cases

- What happens when the paddle reaches the boundary of the play area (collision with walls)? Paddle bounces back proportionally to collision impulse. Balls get an impulse from the wall collision. Screen shake effect (could be implemented by moving the camera position).
- How does the system handle the ball getting stuck in a corner or between bricks?
- What happens when the ball velocity becomes too high (speed limiting)? A speed limit should be enforced depending on the ball type: small balls (golf ball) can reach higher maximum speeds, while large balls (beach ball) have lower maximum speeds to maintain physics stability and gameplay balance.
- How does the game handle window resize or focus loss during gameplay? Pause the game.
- What happens when mouse input is lost or disconnected during gameplay? pause the game.
- How does the paddle rotation affect ball collision angles at extreme rotation values? Rotation angles should be limited to 45 degrees. when the paddle is rotated an angular force should be applied that nudges the paddle back into a horizontal position.
- What happens when multiple bricks are destroyed simultaneously?
- How does the game handle loading a corrupted or missing level definition?

## Requirements *(mandatory)*

### Functional Requirements

#### Core Gameplay

- **FR-001**: Game MUST display a paddle that moves in X and Z directions based on mouse movement
- **FR-002**: Game MUST rotate the paddle based on mouse wheel scrolling
- **FR-003**: Game MUST constrain paddle movement within the play area boundaries
- **FR-004**: Game MUST spawn a ball that moves continuously using physics simulation
- **FR-005**: Game MUST apply physics-based bouncing when the ball collides with walls, paddle, or bricks
- **FR-006**: Game MUST destroy bricks when the ball collides with them
- **FR-007**: Game MUST calculate and apply "english" (steering impulse) to the ball based on paddle movement direction at the moment of collision
- **FR-008**: Game MUST detect when the ball exits the lower boundary of the play area
- **FR-009**: Game MUST track player lives (starting at 3) and decrease life count when ball is lost
- **FR-009a**: Game MUST respawn the ball at the position designated by "2" in the level matrix after ball is lost (if lives remain)
- **FR-009b**: Game MUST respawn the paddle at the position designated by "1" in the level matrix after ball is lost (if lives remain)

#### Game States

- **FR-010**: Game MUST provide a main menu state where players can start a new game
- **FR-011**: Game MUST support a playing state where active gameplay occurs
- **FR-012**: Game MUST support a paused state that freezes gameplay
- **FR-013**: Game MUST support a game over state when all lives are lost
- **FR-014**: Game MUST support a level transition state between levels
- **FR-015**: Game MUST transition between states based on game events

#### Level System

- **FR-016**: Game MUST support loading and displaying 77 unique levels
- **FR-017**: Game MUST arrange bricks in a 22x22 grid layout
- **FR-018**: Game MUST read level definitions that specify brick placement and types
- **FR-019**: Game MUST progress to the next level when all destructible bricks are cleared (indestructible bricks do not block level completion)
- **FR-020**: Game MUST display the current level number to the player

#### Brick System

- **FR-021**: Game MUST support at least 37 different brick types with unique behaviors (including both destructible and indestructible types)
- **FR-022**: Game MUST support standard bricks that are destroyed in one hit
- **FR-023**: Game MUST support multi-hit bricks with durability tracking
- **FR-023a**: Game MUST support indestructible brick types that cannot be destroyed and do not count toward level completion
- **FR-024**: Game MUST provide visual feedback for brick damage state
- **FR-025**: Game MUST support special brick types with unique collision behaviors
- **FR-026**: Game MUST handle brick destruction and removal from the play area

#### Visual Rendering

- **FR-027**: Game MUST render all game objects (paddle, ball, bricks, walls) as 3D models
- **FR-028**: Game MUST constrain all gameplay to the Y=2.0 horizontal plane
- **FR-029**: Game MUST position the camera above the play area looking downward
- **FR-030**: Game MUST provide lighting with shadows for depth perception
- **FR-031**: Game MUST support wireframe toggle on native platforms
- **FR-032**: Game MUST run on both native (Linux/Windows/macOS) and WASM platforms

#### Input Controls

- **FR-033**: Game MUST map mouse X movement to paddle X movement using velocity-based control (mouse delta proportional to paddle velocity)
- **FR-034**: Game MUST map mouse Y movement to paddle Z movement using velocity-based control (mouse delta proportional to paddle velocity)
- **FR-035**: Game MUST map mouse wheel scrolling to paddle rotation
- **FR-036**: Game MUST support mouse cursor locking during gameplay
- **FR-037**: Game MUST support keyboard input for game state controls (pause, resume, menu navigation)

#### Physics Constraints

- **FR-038**: Game MUST lock all rigid body Y-axis translation to maintain 2D plane gameplay
- **FR-039**: Game MUST use physics restitution for ball bouncing behavior
- **FR-040**: Game MUST use physics friction for paddle-ball interaction tuning
- **FR-041**: Game MUST apply damping to prevent unlimited acceleration
- **FR-041a**: Game MUST enforce ball-type dependent maximum velocity limits (smaller balls = higher max speed, larger balls = lower max speed)
- **FR-042**: Game MUST use Continuous Collision Detection (CCD) to prevent tunneling

### Key Entities

- **Paddle**: The player-controlled rectangular object that bounces the ball; has position (X, Z on Y=2.0 plane), rotation, and velocity derived from mouse delta (velocity-based control where faster mouse movement = faster paddle movement)
- **Ball**: A spherical object that moves continuously through physics; has position, velocity, collision properties (restitution, friction), type (which determines size and maximum velocity - smaller balls like golf balls have higher max speed, larger balls like beach balls have lower max speed), and radius
- **Brick**: Grid-aligned objects that are destroyed by ball collision; has type (which determines if destructible or indestructible), position, durability, and collision behavior properties; only destructible bricks count toward level completion
- **Border/Wall**: Boundary objects that contain the play area and reflect the ball; has position and collision properties
- **Level**: A configuration defining brick layout for a specific stage; has level number, brick placement data, and brick type assignments
- **Game State**: The current mode of the game; includes menu, playing, paused, game over, and level transition states
- **Lives**: Player resource tracking remaining attempts (starts at 3); decreases when ball is lost; triggers ball respawn at matrix position "2" and paddle respawn at matrix position "1" when lives remain

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Players can control the paddle smoothly in X and Z directions with mouse movement within 100ms of input
- **SC-002**: Players can influence ball trajectory through paddle movement, creating noticeably different bounce angles when moving versus stationary
- **SC-003**: Game maintains 60 frames per second on target hardware (modern desktop for native, moderate hardware for WASM)
- **SC-004**: Players can complete a single level from start (brick layout loaded) to finish (all bricks destroyed) within 5 minutes
- **SC-005**: Game successfully loads and renders all 77 unique levels without crashes or errors
- **SC-006**: Game supports at least 37 distinct brick types, each with observable unique behavior
- **SC-007**: Players can navigate through all game states (menu → playing → paused → resume → game over) without errors
- **SC-008**: Game runs successfully on both native platforms and in web browsers via WASM
- **SC-009**: Ball physics provides consistent and predictable bouncing behavior across all surface collisions
- **SC-010**: Players can observe 3D visual effects (lighting, shadows, depth) while experiencing 2D gameplay constraints
