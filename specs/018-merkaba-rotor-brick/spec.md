# Feature Specification: Merkaba Rotor Brick

**Feature Branch**: `018-merkaba-rotor-brick` **Created**: 2026-01-07 **Status**: Draft **Input**: User description: "add brick 36 Rotor. when the ball hits brick with index of 36 a message is emitted to spawn a merkaba.
The delay in spawning is deliberate and not a violation of the constitution.
`experiments/merkaba/src/main.rs` contains an example of how to create a merkaba.
It stays in the gaming plane (XZ plane, Y-axis locked).
It interacts with other entities (bounce off bricks and walls), despawns on contact with the goal.
It rotates continuously around the Y-axis (vertical).
Its initial velocity is on the XZ plane (horizontal movement) with primary forward direction (Z-axis) and ±20 degrees lateral variance (X-axis).
It must maintain a minimum speed in the Z direction (forward motion on the XZ plane).
When it comes into contact with the player paddle, the life is lost, all balls are despawned."

**Note on Coordinate System**: This game uses a top-down view with gameplay on the XZ plane (Y-axis locked).
The term "forward" refers to **gameplay direction** (+Z toward goal/bricks), not Bevy's `Transform::forward()` API which returns -Z.
The implementation directly manipulates physics velocity (`linvel.z`), making +Z the forward gameplay direction from the player's perspective.

## Clarifications

### Session 2026-01-07

- Q: What specific spawn delay should be used for merkaba spawning after rotor brick is hit? → A: 0.5 seconds
- Q: What minimum forward (z-direction) speed should the merkaba maintain? → A: 3.0 units/second
- Q: Where should the merkaba spawn? → A: At the brick's position when destroyed
- Q: How should the rotor brick be visually distinguished? → A: Unique texture pattern
- Q: How should the required audio assets be sourced? → A: Use placeholder/synthesized sounds

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, UI updates, or hierarchy, the implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include at least one check that guards against prohibited patterns (e.g., panicking queries or per-frame UI updates without `Changed<T>`).
Acceptance criteria MUST explicitly state which event system is used (Messages vs Observers), justify the choice, and check for **Message-Event Separation** (correct use of `MessageWriter` vs observers/ `Trigger<T>`) and **Hierarchy Safety** (use of `commands.entity(parent).add_child(child)` or `EntityCommands::set_parent`).

### User Story 1 - Rotor Brick Spawns Merkaba Hazard (Priority: P1)

When a player hits a special Rotor brick (brick index 36), a spinning merkaba hazard is spawned into the game field after a brief delay.
The merkaba moves horizontally across the field, bouncing off walls and bricks, creating an additional challenge the player must avoid while continuing normal gameplay.

**Why this priority**: This is the core feature - introducing a new brick type that spawns a dangerous moving hazard.
Without this, the feature has no value.

**Independent Test**: Can be fully tested by placing a brick with index 36 in a level, hitting it with the ball, and verifying a merkaba entity spawns with correct physics and visual properties.

**Acceptance Scenarios**:

1. **Given** a level contains a brick with index 36, **When** the ball collides with this brick, **Then** a message is emitted to spawn a merkaba
2. **Given** a spawn merkaba message is emitted, **When** the spawn delay completes, **Then** a merkaba entity is created at the brick's position with dual tetrahedron visual mesh
3. **Given** a merkaba has spawned, **When** time passes, **Then** the merkaba rotates continuously around the Y-axis (vertical rotation)
4. **Given** a merkaba has spawned, **When** it initializes, **Then** its initial velocity is primarily forward (Z-axis direction) on the XZ plane with lateral (X-axis) variance of ±20 degrees

**Event System Choice**: This feature uses asynchronous message-based communication for brick-hit-to-merkaba-spawn flow because the spawn has a deliberate delay (not immediate cause-and-effect).
The delayed spawning behavior requires timer-based handling between event emission and entity creation.

---

### User Story 2 - Merkaba Physics Interactions (Priority: P2)

The merkaba hazard behaves as a physical object in the game world - it bounces off walls and bricks, stays within the gaming plane, and despawns when it reaches the goal area.
This creates dynamic, unpredictable movement patterns.

**Why this priority**: Physics interactions make the merkaba a challenging hazard rather than a static threat.
This is secondary to basic spawning but essential for gameplay value.

**Independent Test**: Can be tested by manually spawning a merkaba and verifying it bounces off walls, bricks, and despawns at the goal boundary.

**Acceptance Scenarios**:

1. **Given** a merkaba is moving, **When** it collides with a wall, **Then** it bounces off with appropriate physics response and a distinct collision sound is emitted
2. **Given** a merkaba is moving, **When** it collides with a brick, **Then** it bounces off the brick (brick is not destroyed) and a distinct collision sound is emitted
3. **Given** a merkaba is moving, **When** it reaches the goal area boundary, **Then** the merkaba entity despawns
4. **Given** a merkaba is moving forward (Z-axis direction), **When** its speed would fall below minimum threshold, **Then** a minimum speed of 3.0 u/s is maintained on the Z-axis to prevent stalling, and lateral drift (X-axis) is capped to half the forward speed
5. **Given** a merkaba exists, **When** rendering, **Then** it remains constrained to the gaming plane (Y-axis position locked via `LockedAxes::TRANSLATION_LOCKED_Y`)
6. **Given** at least one merkaba exists in the game, **When** gameplay continues, **Then** a helicopter blade-like looping background sound plays continuously
7. **Given** the helicopter blade sound is playing and all merkabas despawn, **When** the last merkaba is removed, **Then** the helicopter blade sound stops

---

### User Story 3 - Merkaba-Paddle Contact Penalty (Priority: P3)

When a merkaba hazard contacts the player's paddle, it represents a critical failure state - the player loses a life and all balls are immediately despawned.
This creates high-stakes gameplay where players must avoid the merkaba while continuing to play.

**Why this priority**: This defines the consequence of failure to avoid the hazard.
While important for game balance, the feature is still valuable without this if merkabas simply bounce off paddles.

**Independent Test**: Can be tested by spawning a merkaba, directing it toward the paddle, and verifying life loss and ball despawn on contact.

**Acceptance Scenarios**:

1. **Given** a merkaba and paddle exist in the game, **When** the merkaba collides with the paddle, **Then** the player loses one life and a distinct collision sound is emitted
2. **Given** a merkaba contacts the paddle, **When** the collision occurs, **Then** all active ball entities are despawned
3. **Given** a merkaba contacts the paddle and balls are despawned, **When** the player has remaining lives, **Then** standard ball respawn mechanics apply
4. **Given** a merkaba contacts the paddle, **When** the collision occurs, **Then** the merkaba itself despawns
5. **Given** the player loses a life from any cause, **When** the life loss occurs, **Then** all active merkaba entities are despawned
6. **Given** multiple merkabas exist and the player loses a life, **When** all merkabas are despawned, **Then** the helicopter blade background sound stops

---

### Edge Cases

- What happens when multiple rotor bricks (index 36) are hit in rapid succession? (Multiple merkabas should spawn with independent delays)
- How does the system handle merkaba spawning when the game is paused? (Spawn delay should pause with game time)
- What happens if a merkaba spawns at a brick location already occupied by another entity? (Should spawn at the brick's last position with collision resolution handled by physics)
- What happens when a merkaba bounces off a multi-hit brick? (Brick should not take damage from merkaba collision)
- What happens if the goal area is blocked when a merkaba tries to despawn there? (Despawn should occur regardless of physical obstacles)
- How does minimum z-speed maintenance work when bouncing off angled surfaces? (Speed boost should be applied in the z-direction component specifically)
- What happens to queued merkaba spawn timers when the level changes? (All pending spawns should be cancelled)
- What happens to the helicopter blade sound when the game is paused? (Sound should pause with game state)
- What happens if multiple merkabas collide with surfaces simultaneously? (Each collision produces its own sound, potentially overlapping)
- What happens to merkaba collision sounds when audio is muted? (Sounds should respect global audio settings)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST recognize brick index 36 as a special "Rotor" brick type
- **FR-002**: System MUST emit a spawn merkaba message when a ball collides with a brick with index 36
- **FR-003**: System MUST implement a 0.5 second delay between message emission and actual merkaba spawning
- **FR-004**: System MUST spawn a merkaba entity with dual tetrahedron visual geometry (one upright, one inverted)
- **FR-005**: System MUST apply continuous rotation to merkaba around the Y-axis (vertical rotation) at a measurable rate of 180 degrees per second (±10% tolerance)
- **FR-006**: System MUST initialize merkaba velocity primarily forward (z direction) on the XZ plane with lateral (x direction) variance of ±20 degrees
- **FR-007**: System MUST maintain a minimum speed threshold of 3.0 units/second for merkaba forward movement (z direction) and cap lateral drift (x direction) to half the forward speed
- **FR-008**: System MUST constrain merkaba movement to the gaming plane (Y-axis translation locked via `LockedAxes::TRANSLATION_LOCKED_Y`; XZ plane movement only)
- **FR-009**: System MUST make merkaba bounce off walls with appropriate physics response
- **FR-010**: System MUST make merkaba bounce off bricks without destroying them
- **FR-011**: System MUST despawn merkaba when it contacts the goal area
- **FR-012**: System MUST trigger life loss when merkaba contacts player paddle
- **FR-013**: System MUST despawn all active balls when merkaba contacts player paddle
- **FR-014**: System MUST despawn merkaba after it contacts player paddle
- **FR-015**: System MUST support multiple merkabas existing simultaneously (from multiple rotor brick hits)
- **FR-016**: Rotor brick MUST be destroyed when hit by ball (standard brick destruction behavior)
- **FR-017**: System MUST emit a distinct collision sound when merkaba contacts a wall
- **FR-018**: System MUST emit a distinct collision sound when merkaba contacts a brick
- **FR-019**: System MUST emit a distinct collision sound when merkaba contacts the paddle
  - *Distinctiveness criteria*: Each collision type (wall/brick/paddle) MUST use a unique audio asset AND differ in envelope (e.g., duration ≥100ms apart, spectral centroid >2 kHz difference, or explicit naming convention) to ensure player differentiation.
- **FR-020**: System MUST play a helicopter blade-like background sound loop when at least one merkaba exists
- **FR-021**: System MUST stop the helicopter blade background sound when all merkabas are despawned
- **FR-022**: System MUST despawn all active merkabas when the player loses a life from any cause

### Key Entities

- **Rotor Brick (Index 36)**: A special brick type that triggers merkaba spawning when struck by the ball.
  It is destroyed on impact like normal bricks but emits a spawn message.
- **Merkaba**: A spinning hazard entity composed of dual tetrahedrons (star tetrahedron shape).
  It has physics properties (collision, velocity, rotation), visual rendering, and special interaction rules.
  Spawns at the location of the destroyed rotor brick.
- **Spawn Merkaba Message**: An event/message that carries information needed to spawn a merkaba (spawn location from brick position, initial velocity direction).
- **Merkaba Spawn Timer**: A delayed spawning mechanism that waits before creating the merkaba entity after the message is received.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Players can visually identify rotor brick (index 36) by its unique texture pattern that distinguishes it from all other brick types
- **SC-002**: Merkaba spawns exactly 0.5 seconds after rotor brick is hit (quick response that heightens urgency)
- **SC-003**: Merkaba rotation animation is smooth and continuous at standard game frame rates (60 FPS per constitution)
- **SC-004**: Merkaba hazard creates observable challenge - players adjust strategy to avoid it while playing
- **SC-005**: Merkaba collision with paddle results in immediate and unambiguous life loss feedback (visual and/or audio cue within 0.1 seconds)
- **SC-006**: Multiple merkabas (from multiple rotor bricks) can coexist without performance degradation (up to 5 simultaneous merkabas maintain 60 FPS per constitution)
- **SC-007**: Merkaba physics interactions (bouncing) feel consistent with other game entities
- **SC-008**: 100% of merkaba-goal contacts result in merkaba despawn
- **SC-009**: 100% of merkaba-paddle contacts result in life loss and ball despawn
- **SC-010**: Merkaba maintains minimum horizontal movement speed - never appears stuck or motionless

## Scope & Assumptions

### In Scope

- Brick index 36 recognition and special behavior
- Unique texture pattern for rotor brick visual identification
- Merkaba spawning with delayed timing
- Merkaba visual representation (dual tetrahedron geometry)
- Merkaba physics (collision, bouncing, velocity constraints)
- Merkaba-paddle collision consequences (life loss, ball despawn)
- Merkaba-goal area despawn
- Support for multiple simultaneous merkabas
- Audio feedback for merkaba collisions (wall, brick, paddle)
- Helicopter blade-like background sound loop when merkaba(s) exist
- Merkaba despawn on player life loss

### Out of Scope

- Merkaba powerup collection (merkaba only acts as hazard)
- Merkaba visual effects beyond basic geometry (particles, trails, glows)
- Configurable merkaba parameters (speed, size, spawn delay) - uses fixed values
- Level editor integration for placing rotor bricks - brick index 36 specified in level data manually
- Production-quality custom audio file creation (feature uses placeholder/synthesized sounds)
- Custom texture asset creation for rotor brick (uses existing texture assets or placeholders)

### Assumptions

- Brick index 36 is not currently used by any existing brick type in the game
- The game already has a message/event system available for inter-system communication
- The game has a physics system supporting collision detection and velocity modification
- Paddle collision detection distinguishes between ball and other entities
- Goal area has defined boundaries for collision detection
- Level data format supports arbitrary brick indices (not limited to sequential numbers)
- Game pause functionality affects all timer-based delays including merkaba spawning
- Level transitions clear all active entities including pending spawn timers
- The dual tetrahedron (star tetrahedron) shape can be composed from standard geometric primitives
- Minimum speed enforcement is acceptable gameplay behavior (not exploitable by players)
- The game has an audio system that supports collision-triggered sounds and looping background sounds
- Placeholder or synthesized audio assets will be used for merkaba collision sounds and helicopter blade loop
- Audio playback respects global game audio settings (volume, mute)
- Life loss mechanism can trigger entity despawn for non-ball entities (merkabas)
- A suitable texture asset exists or can be sourced to visually distinguish the rotor brick from other brick types
