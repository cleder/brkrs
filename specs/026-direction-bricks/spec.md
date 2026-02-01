# Feature Specification: Direction Bricks

**Feature Branch**: `026-direction-bricks` **Created**: 2026-02-01 **Status**: Draft **Input**: Implement bricks 43-48 and 52 that accelerate ball in specific directions or randomize velocity

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, UI updates, or hierarchy, the implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include at least one check that guards against prohibited patterns (e.g., panicking queries or per-frame UI updates without `Changed<T>`).
Acceptance criteria MUST explicitly state which event system is used (Messages vs Observers), justify the choice, and check for **Message-Event Separation** (correct use of `MessageWriter` vs observers/ `Trigger<T>`) and **Hierarchy Safety** (use of `commands.entity(parent).add_child(child)` or `EntityCommands::set_parent`).

**COORDINATE SYSTEM REQUIREMENT**: Direction bricks apply velocity modifications along the XZ plane (horizontal movement) and Y-axis (vertical movement).
The system uses Bevy's standard coordinates:

- **X-axis**: Horizontal (left = -X, right = +X from player perspective)
- **Y-axis**: Vertical (down = -Y, up = +Y)
- **Z-axis**: Horizontal (backward = -Z, forward = +Z from player perspective)
- Direction bricks apply impulses/velocities to the ball's linear velocity
- All directional accelerations assume a fixed acceleration magnitude (5.0 units/sec in the specified direction) applied to existing velocity

**MULTI-FRAME PERSISTENCE REQUIREMENT**: If the feature involves runtime state changes (gravity, scores, powerup effects, or any resource/component modified during gameplay), acceptance scenarios MUST include multi-frame persistence checks:

- Tests MUST verify state persists across multiple `app.update()` cycles (minimum 10 frames)
- Tests MUST include ALL systems that write to the affected resource/component to catch per-frame overwrite bugs
- This requirement exists because single-frame assertions miss bugs where initialization or cleanup systems unconditionally overwrite runtime state (see 020-gravity-bricks retrospective)

### User Story 1 - Single-Direction Brick Modifies Ball Velocity (Priority: P1)

When the player destroys a directional brick (43, 44, 45, or 46), the ball's velocity is modified in the specified direction with a fixed acceleration magnitude.
This creates dynamic, predictable gameplay where hitting certain bricks reliably sends the ball in known directions.

**Why this priority**: This is the core mechanic for 4 of the 7 brick types.
Without directional velocity modification, the feature is non-functional.
Single-direction bricks are independent and testable.

**Independent Test**: Can be fully tested by destroying a single direction brick and verifying the ball's velocity changes in the correct cardinal direction.
Delivers immediate gameplay value independent of diagonal or randomization features.

**Acceptance Scenarios**:

1. **Given** the ball is moving at any velocity, **When** it destroys brick 43 (Down), **Then** the ball's Y-velocity decreases by 5.0 units/sec (accelerates downward)

2. **Given** the ball is moving at any velocity, **When** it destroys brick 44 (Left), **Then** the ball's X-velocity decreases by 5.0 units/sec (accelerates leftward)

3. **Given** the ball is moving at any velocity, **When** it destroys brick 45 (Right), **Then** the ball's X-velocity increases by 5.0 units/sec (accelerates rightward)

4. **Given** the ball is moving at any velocity, **When** it destroys brick 46 (Up), **Then** the ball's Y-velocity increases by 5.0 units/sec (accelerates upward)

5. **Given** a ball moving downward at -3.0 Y-velocity, **When** it destroys brick 43 (Down), **Then** the ball's Y-velocity becomes -8.0 (additive acceleration, not replacement)

6. **Given** multiple direction bricks in sequence, **When** the ball destroys consecutive bricks, **Then** velocity modifications stack correctly across frames without being overwritten

---

### User Story 2 - Diagonal-Direction Brick Modifies Ball Velocity in Two Axes (Priority: P1)

When the player destroys a diagonal direction brick (47 or 48), the ball's velocity is modified simultaneously along two axes (e.g., up AND right for brick 47).
This enables more complex ball trajectories and increases level design flexibility.

**Why this priority**: Diagonal bricks expand tactical depth and level design possibilities.
Essential for balanced, varied gameplay.

**Independent Test**: Can be tested independently by destroying a single diagonal brick and verifying the ball's velocity changes correctly in both axes.

**Acceptance Scenarios**:

1. **Given** the ball is moving at any velocity, **When** it destroys brick 47 (Up-Right), **Then** the ball's Y-velocity increases by 5.0 units/sec AND the ball's X-velocity increases by 5.0 units/sec

2. **Given** the ball is moving at any velocity, **When** it destroys brick 48 (Up-Left), **Then** the ball's Y-velocity increases by 5.0 units/sec AND the ball's X-velocity decreases by 5.0 units/sec

3. **Given** a ball moving at (2.0, 2.0, 0) velocity, **When** it destroys brick 47 (Up-Right), **Then** the ball's velocity becomes (7.0, 7.0, 0) (both modifications applied additively)

---

### User Story 3 - Randomization Brick Unpredictably Modifies Ball Velocity (Priority: P1)

When the player destroys brick 52 (Randomizer), the ball's velocity is set to a random direction with a random magnitude.
This brick adds chaotic gameplay elements and forces players to adapt to unpredictable situations.

**Why this priority**: The randomization brick is its own distinct mechanic and complete feature.
Essential for level design variety and unpredictability.

**Independent Test**: Can be tested by destroying brick 52 and verifying that:

1. The ball's velocity changes to a random value within documented bounds
2. Multiple destruction events produce different random velocities
3. The randomization is not repeatable in the same game session

**Acceptance Scenarios**:

1. **Given** the ball is moving at any velocity, **When** it destroys brick 52 (Randomizer), **Then** the ball's linear velocity is replaced with a random velocity vector

2. **Given** brick 52 is destroyed multiple times, **When** observing the resulting ball velocities, **Then** each destruction produces a statistically different random velocity (not deterministic)

3. **Given** a randomizer brick is destroyed, **When** the random velocity is applied, **Then** the magnitude is between 5.0 and 15.0 units/sec (velocity length) and direction is uniformly distributed across all 360 degrees

4. **Given** the ball has velocity (10.0, 0.0, 0.0), **When** it destroys brick 52, **Then** the previous velocity is completely replaced (not modified additively) with the random value

---

### User Story 4 - Direction Bricks Award Points When Destroyed (Priority: P1)

Each direction brick type (43-48, 52) awards the player a specific point value when destroyed.
This maintains scoring consistency with other brick types in the game.

**Why this priority**: Scoring is fundamental to the game's progression system.
Direction bricks must integrate seamlessly into existing scoring.

**Independent Test**: Can be tested by destroying individual direction bricks and verifying the score increases by the correct amount, independent of velocity mechanics.

**Acceptance Scenarios**:

1. **Given** the player destroys brick 43 (Down), **Then** the player's score increases by 75 points

2. **Given** the player destroys brick 44 (Left), **Then** the player's score increases by 75 points

3. **Given** the player destroys brick 45 (Right), **Then** the player's score increases by 75 points

4. **Given** the player destroys brick 46 (Up), **Then** the player's score increases by 75 points

5. **Given** the player destroys brick 47 (Up-Right), **Then** the player's score increases by 100 points (diagonal bricks worth more)

6. **Given** the player destroys brick 48 (Up-Left), **Then** the player's score increases by 100 points

7. **Given** the player destroys brick 52 (Randomizer), **Then** the player's score increases by 125 points (special brick worth most)

---

### Edge Cases

- What happens when a direction brick is destroyed while the ball is stationary (velocity ≈ 0)?
  The direction acceleration should still apply, moving the ball in that direction.
- What happens when multiple direction bricks are destroyed in rapid succession (same frame)?
  Velocity modifications should stack; each brick's modification applies to the ball's current velocity.
- What happens when brick 52 (Randomizer) generates a zero-magnitude velocity?
  This should be prevented; randomized velocity should always have a minimum magnitude of 5.0 units/sec.
- What happens to the ball's Z-velocity (forward/backward movement) when destroying direction bricks?
  Cardinal and diagonal bricks only modify X and Y; Z-velocity is unchanged (only horizontal XZ plane and vertical Y are affected per coordinate system definition).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Direction bricks (43-48, 52) MUST be distinguishable from all other brick types in level files and in-game display
- **FR-002**: System MUST apply directional velocity modification (5.0 units/sec acceleration) when bricks 43, 44, 45, or 46 are destroyed
- **FR-003**: System MUST apply simultaneous velocity modification in two axes (both axes at 5.0 units/sec) when bricks 47 or 48 are destroyed
- **FR-004**: System MUST replace ball velocity with random direction and magnitude (5.0-15.0 units/sec) when brick 52 is destroyed
- **FR-005**: System MUST prevent zero-magnitude velocity when generating random velocities for brick 52
- **FR-006**: Direction brick destruction events MUST trigger existing scoring system with correct point values (75 points for 43-46, 100 points for 47-48, 125 points for 52)
- **FR-007**: Velocity modifications MUST be applied as additive changes to existing velocity (not replacement, except for brick 52 randomization)
- **FR-008**: Direction bricks MUST integrate with existing brick destruction event system and not bypass any established brick lifecycle behaviors
- **FR-009**: Randomization (brick 52) MUST use a uniform distribution for direction and use the project's existing RNG mechanism (rand crate)

### Key Entities

- **Direction Brick (Bricks 43-48, 52)**: Game object with assigned brick type ID that, when destroyed by ball collision, applies velocity modification to the ball
  - **Brick 43 (Down)**: Applies -5.0 Y-velocity acceleration
  - **Brick 44 (Left)**: Applies -5.0 X-velocity acceleration
  - **Brick 45 (Right)**: Applies +5.0 X-velocity acceleration
  - **Brick 46 (Up)**: Applies +5.0 Y-velocity acceleration
  - **Brick 47 (Up-Right)**: Applies +5.0 X and +5.0 Y velocity accelerations
  - **Brick 48 (Up-Left)**: Applies -5.0 X and +5.0 Y velocity accelerations
  - **Brick 52 (Randomizer)**: Replaces velocity with random direction (0-360°) and magnitude (5.0-15.0 units/sec)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Players can destroy and observe behavior changes for all 7 direction brick types (43-48, 52) in test levels without errors or unexpected physics behavior
- **SC-002**: 100% of destruction events for direction bricks award correct point values (75, 100, or 125 points as specified)
- **SC-003**: Velocity modifications persist correctly across multiple game frames (minimum 10 update cycles) without being overwritten or reset
- **SC-004**: Randomized velocities from brick 52 span the full range of 5.0-15.0 units/sec magnitude and cover at least 180 degrees of directional variation in a typical play session (not clustered)
- **SC-005**: Direction bricks integrate seamlessly with existing brick destruction systems, triggering all established lifecycle behaviors (scoring, particle effects, audio, etc.)
- **SC-006**: All existing tests continue to pass; no regression in ball physics, collision detection, or other brick types

## Assumptions

- Direction brick behavior (velocity acceleration magnitude of 5.0 units/sec per direction) follows the same physics system as existing ball mechanics and gravity bricks
- The `rand` crate is already available as a project dependency for randomization
- Brick type IDs 43-48 and 52 are reserved and not used for any other brick types
- Scoring system already has infrastructure to support arbitrary point values per brick type
- Ball physics use Bevy's `LinearVelocity` component from bevy_rapier3d, which can be directly modified
- Level files (RON format) already support arbitrary brick type IDs and custom metadata per brick

## Dependencies

- **Existing Systems**: Direction bricks depend on the existing brick destruction event system (`BrickDestroyed` or equivalent Message/Observer)
- **Physics Engine**: Requires bevy_rapier3d to provide and modify ball `LinearVelocity`
- **Scoring System**: Requires integration with the existing points/score tracking system
- **Level Format**: Requires the level loader to recognize and spawn bricks with IDs 43-48 and 52
- **RNG**: Uses the project's existing `rand` crate for brick 52 randomization
