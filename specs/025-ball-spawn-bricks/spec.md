# Feature Specification: Ball Spawn Bricks

**Feature Branch**: `025-ball-spawn-bricks` **Created**: 2026-01-31 **Status**: Draft **Input**: User description: "add bricks index 37,38,39. 38 spawn one additional ball, 39 spawn two additional balls, 37 despawn all other balls. the bricks score 100. balls are spawned at the center of the destroyed brick"

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, or UI updates, the implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include at least one check that guards against prohibited patterns (e.g., panicking queries or per-frame UI updates without `Changed<T>`).
Acceptance criteria MUST explicitly state which event system is used (Messages vs Observers), justify the choice, and check for **Message-Event Separation** (correct use of `MessageWriter` vs observers/ `Trigger<T>`) and **Hierarchy Safety** (use of `commands.entity(parent).add_child(child)` or `EntityCommands::set_parent`).

**COORDINATE SYSTEM REQUIREMENT**: If the feature involves spatial movement, physics velocity, or directional behavior, the specification MUST include a coordinate system note clarifying:

- Which axes are used for movement (XZ plane for horizontal, Y for vertical, etc.)
- Whether directional terms (forward/backward/left/right) refer to Bevy's Transform API convention (forward = -Z), gameplay-relative directions (player perspective), or direct axis manipulation (±X, ±Y, ±Z)
- How the camera view orientation affects gameplay directions
- Any locked axes via `LockedAxes` constraints

**Coordinate System Note**: Ball spawning occurs at the XZ position (horizontal plane) of the destroyed brick, with Y-axis position inherited from the brick.
Spawned balls inherit the triggering ball's velocity vector, with modifications based on brick type (inverse direction for brick 38, Y-shaped spread for brick 39).

**MULTI-FRAME PERSISTENCE REQUIREMENT**: If the feature involves runtime state changes (gravity, scores, powerup effects, or any resource/component modified during gameplay), acceptance scenarios MUST include multi-frame persistence checks:

- Tests MUST verify state persists across multiple `app.update()` cycles (minimum 10 frames)
- Tests MUST include ALL systems that write to the affected resource/component to catch per-frame overwrite bugs
- This requirement exists because single-frame assertions miss bugs where initialization or cleanup systems unconditionally overwrite runtime state (see 020-gravity-bricks retrospective)

### User Story 1 - Red 2 Brick: Spawn Additional Ball (Priority: P1)

When a player hits the Red 2 brick (index 38), it spawns one additional ball at the brick's position, moving in the inverse direction of the triggering ball, creating dynamic multi-ball gameplay.

**Why this priority**: This is the foundation of the ball spawn mechanics.
It delivers immediate value by introducing controlled multi-ball gameplay with predictable behavior that players can strategize around.

**Independent Test**: Can be fully tested by placing a single Red 2 brick in a test level, hitting it with one ball, and verifying exactly two balls exist with inverse velocity vectors.
Delivers the core value of strategic ball multiplication.

**Acceptance Scenarios**:

1. **Given** one ball in play and a Red 2 brick (index 38) at position (5, 0, 3), **When** the ball hits the brick, **Then** the brick is destroyed, one new ball spawns at position (5, 0, 3), the new ball has the same speed as the triggering ball but inverse direction, 100 points are awarded, and exactly two balls are now in play
2. **Given** three balls in play and a Red 2 brick, **When** any ball hits the brick, **Then** one additional ball spawns (total of four balls), the new ball inherits inverse velocity from the triggering ball only
3. **Given** a Red 2 brick at the edge of the playfield, **When** hit by a ball, **Then** the spawned ball appears at the brick's center position and follows physics rules (may immediately bounce off walls)
4. **Given** a Red 2 brick just destroyed, **When** 10 update cycles pass, **Then** the spawned ball persists and continues moving according to physics (multi-frame persistence check)
5. **Given** the game uses Messages for brick destruction events, **When** a Red 2 brick is hit, **Then** a `BrickDestroyed` message is sent via `MessageWriter` (not an observer), the ball spawn system reads this message, and spawning occurs in a separate system that runs after physics collision detection

---

### User Story 2 - Red 3 Brick: Spawn Two Additional Balls (Priority: P2)

When a player hits the Red 3 brick (index 39), it spawns two additional balls at the brick's position in a Y-shaped spread pattern, creating chaotic multi-ball scenarios for advanced gameplay.

**Why this priority**: Builds on the Red 2 foundation to offer higher-risk/higher-reward gameplay.
Players must manage more balls simultaneously, adding complexity.

**Independent Test**: Can be fully tested by hitting a single Red 3 brick and verifying exactly three balls exist with Y-shaped velocity vectors.
Delivers escalated multi-ball chaos independently of other brick types.

**Acceptance Scenarios**:

1. **Given** one ball in play with velocity (5, 0, -3) and a Red 3 brick at position (7, 0, 5), **When** the ball hits the brick, **Then** the brick is destroyed, two new balls spawn at position (7, 0, 5) with velocities forming a Y-shaped pattern (one angled left, one angled right relative to the original trajectory), 100 points are awarded, and exactly three balls are now in play
2. **Given** two balls in play and a Red 3 brick, **When** any ball hits the brick, **Then** two additional balls spawn (total of four balls), maintaining the Y-shaped spread relative to the triggering ball's direction
3. **Given** multiple balls hitting a Red 3 brick simultaneously, **When** collision occurs, **Then** only one set of two balls spawns (brick can only be destroyed once)
4. **Given** a Red 3 brick just destroyed with two spawned balls, **When** 10 update cycles pass, **Then** both spawned balls persist and move independently according to their velocity vectors (multi-frame persistence check)

---

### User Story 3 - Red 1 Brick: Reset to Single Ball (Priority: P3)

When a player hits the Red 1 brick (index 37), all balls except one are immediately despawned, resetting to single-ball gameplay.
This provides strategic relief from overwhelming multi-ball situations.

**Why this priority**: This is a corrective/defensive mechanic that helps players recover from chaotic states.
Lower priority because it depends on multi-ball scenarios existing first (from P1/P2 stories).

**Independent Test**: Can be fully tested by spawning multiple balls manually, hitting a Red 1 brick, and verifying exactly one ball remains.
Delivers immediate simplification value independently.

**Acceptance Scenarios**:

1. **Given** five balls in play and a Red 1 brick at position (10, 0, 8), **When** any ball hits the brick, **Then** the brick is destroyed, all balls except the one that triggered the brick destruction are despawned, exactly one ball remains in play, 100 points are awarded
2. **Given** only one ball in play and a Red 1 brick, **When** the ball hits the brick, **Then** the brick is destroyed, the ball remains in play (no despawning occurs), 100 points are awarded
3. **Given** three balls in play where two are currently off-screen, **When** an on-screen ball hits a Red 1 brick, **Then** the two off-screen balls are despawned, only the triggering ball remains
4. **Given** a Red 1 brick just destroyed leaving one ball, **When** 10 update cycles pass, **Then** exactly one ball persists in play with no re-spawning of despawned balls (multi-frame persistence check)

---

### Edge Cases

- What happens when a ball hits a Red 2 or Red 3 brick in a corner position? (Spawned balls appear at brick center; may immediately collide with walls and bounce)
- What happens when many balls are already in play and a Red 2/Red 3 brick is hit? (No maximum ball limit exists; new balls are always spawned when these bricks are destroyed, regardless of current ball count)
- What happens when a Red 1 brick is hit by multiple balls in the same frame? (Only one ball survives - the first one processed by the collision system)
- What happens when spawned balls immediately collide with the paddle or other bricks? (Normal collision physics apply; they may trigger additional brick effects)
- What happens when a Red 2 or Red 3 brick is hit while the ball is very slow or near zero velocity? (Spawned balls inherit the slow velocity/inverse direction, may appear stationary or move very slowly)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST recognize brick indices 37, 38, and 39 as valid destructible brick types during level loading
- **FR-002**: System MUST destroy brick index 38 (Red 2) on ball collision and spawn exactly one additional ball at the brick's center position
- **FR-003**: System MUST destroy brick index 39 (Red 3) on ball collision and spawn exactly two additional balls at the brick's center position
- **FR-004**: System MUST destroy brick index 37 (Red 1) on ball collision and despawn all balls except the triggering ball
- **FR-005**: System MUST award 100 points when any of these three bricks (37, 38, 39) are destroyed
- **FR-006**: Spawned balls from brick 38 MUST inherit the same speed as the triggering ball but with inverse direction vector (negated velocity)
- **FR-007**: Spawned balls from brick 39 MUST inherit the same speed as the triggering ball but with velocity vectors forming a Y-shaped spread pattern (one angled approximately 30-45 degrees left, one angled 30-45 degrees right from the original trajectory)
- **FR-008**: Ball despawning from brick 37 MUST remove all ball entities except the one that collided with the brick
- **FR-009**: System MUST count all three brick types (37, 38, 39) toward level completion requirements (they are destructible bricks in the 10-57 range)
- **FR-010**: System MUST use the Messages event system (`MessageWriter`) for brick destruction notifications, ensuring ball spawn/despawn logic runs in systems that read `BrickDestroyed` messages after physics collision detection
- **FR-011**: Ball spawning/despawning MUST be safe with respect to Bevy's entity hierarchy requirements (spawned balls should use `commands.spawn()` without parent relationships, or use proper hierarchy APIs if parented)

### Key Entities

- **Ball Spawn Bricks (37, 38, 39)**: Destructible bricks that manipulate the number of balls in play.
  Each has:
  - Brick index (37, 38, or 39)
  - Position in 3D space (center point for ball spawning)
  - Score value (100 points)
  - Destruction behavior (spawns/despawns balls)
  - Visual representation (Red 1, Red 2, Red 3 textures)
- **Ball Entity**: Physics-enabled sphere that:
  - Has position, velocity, and collision properties
  - Can be spawned dynamically during gameplay
  - Can be despawned when Red 1 brick is triggered
  - Inherits velocity characteristics from triggering ball (for spawned balls)
- **Brick Destruction Event**: Message sent when any brick is destroyed, containing:
  - Brick entity reference
  - Brick type/index
  - Ball entity that triggered the destruction (for spawn/despawn logic)
  - Ball velocity at collision time (for calculating spawned ball velocities)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Players can trigger Red 2 brick and observe exactly one additional ball spawning within one frame of collision
- **SC-002**: Players can trigger Red 3 brick and observe exactly two additional balls spawning in a Y-shaped pattern within one frame of collision
- **SC-003**: Players can trigger Red 1 brick with 5 balls in play and observe reduction to exactly 1 ball within one frame of collision
- **SC-004**: All three brick types award exactly 100 points upon destruction, verifiable in score display
- **SC-005**: Spawned balls persist and behave identically to original balls (same physics, collision detection, and brick interaction) across 100+ frames of gameplay
- **SC-006**: Ball spawn/despawn mechanics work correctly regardless of triggering ball's speed, direction, or position on playfield
- **SC-007**: System handles rapid consecutive triggers (e.g., hitting Red 2, then Red 3, then Red 1 within seconds) without errors or unexpected ball counts

## Assumptions

- The game's existing brick destruction system supports sending a message with sufficient context (brick type, triggering ball entity, ball velocity) for the spawn/despawn logic
- The physics system supports dynamically spawning entities mid-game without disrupting existing ball trajectories
- The "inverse direction" for Red 2 means negating the velocity vector (if ball travels at velocity V, spawned ball travels at velocity -V)
- The Y-shaped pattern for Red 3 uses a spread angle of approximately 30-45 degrees from the original trajectory (specific angle can be tuned during implementation)
- There is no maximum limit on the number of balls in play - the system must handle any number of simultaneous balls without artificial caps
- Ball entities can be safely spawned and despawned at runtime without causing memory leaks or entity reference issues
- Visual assets (textures) for Red 1, Red 2, and Red 3 bricks already exist in the asset system
- The scoring system can handle dynamic point additions during gameplay

## Dependencies

- Existing brick destruction system and collision detection
- Existing ball physics and spawning infrastructure
- Scoring system integration
- Level loading system must support brick indices 37, 38, and 39
- Messages event system for brick destruction notifications
