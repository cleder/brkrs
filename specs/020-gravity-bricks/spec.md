# Feature Specification: Gravity Switching Bricks

**Feature Branch**: `020-gravity-bricks` **Created**: 2026-01-10 **Status**: Draft **Input**: Implement gravity switching bricks 21-25 with immediate gravity application on destruction and gravity reset on life loss

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, UI updates, or hierarchy, the implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include at least one check that guards against prohibited patterns (e.g., panicking queries or per-frame UI updates without `Changed<T>`).
Acceptance criteria MUST explicitly state which event system is used (Messages vs Observers), justify the choice, and check for **Message-Event Separation** (correct use of `MessageWriter` vs observers/ `Trigger<T>`) and **Hierarchy Safety** (use of `commands.entity(parent).add_child(child)` or `EntityCommands::set_parent`).

**COORDINATE SYSTEM REQUIREMENT**: Gravity bricks apply force vectors along the Y-axis (vertical, up = +Y) and XZ plane (horizontal).
The gravity configuration uses Bevy's standard coordinates:

- **X-axis**: Horizontal (left/right from player perspective)
- **Y-axis**: Vertical (up/down)
- **Z-axis**: Horizontal (forward/backward from player perspective)
- Zero gravity fully disables downward force, allowing the ball to float
- Gravity values represent acceleration magnitude in the respective axis directions

**MESSAGE SYSTEM REQUIREMENT**: Gravity mechanics use the Messages event system via `GravityChanged` message to trigger gravity updates in the physics system.
This ensures proper separation of concerns and deterministic gravity application across the physics pipeline.

## Clarifications

### Session 2026-01-10

- Q: What is the documented fallback if a level doesn't specify default gravity? → A: Use zero gravity (0.0, 0.0, 0.0) as a neutral fallback
- Q: Which RNG system should be used for Queer Gravity randomization? → A: Use the `rand` crate already in project dependencies
- Q: What is the frame timing precision for gravity reset on ball loss? → A: Reset timing is not critical; any delay during the ball respawn sequence is acceptable
- Q: Should gravity apply to ball, paddle, and enemies or only the ball? → A: Apply gravity ONLY to the ball; paddle and enemies maintain standard physics
- Q: How should existing levels without gravity metadata be handled? → A: All existing levels automatically receive zero gravity (0.0, 0.0, 0.0) as default (no migration needed)

### User Story 1 - Player Experiences Gravity Change When Destroying Gravity Brick (Priority: P1)

When the player destroys a gravity brick (indices 21-25), the game world's gravity immediately transitions to the gravity value associated with that brick.
This creates dynamic gameplay where gravity effects are core to level design and player challenge.

**Why this priority**: This is the core mechanic - without gravity application on brick destruction, the feature is non-functional.
All other stories depend on this working correctly.

**Independent Test**: Can be fully tested by destroying a single gravity brick in a level and verifying the ball's physics behavior changes according to the brick's gravity settings.
Delivers immediate, observable gameplay value.

**Acceptance Scenarios**:

1. **Given** a level with gravity brick 21 (Zero Gravity) and default gravity enabled, **When** the ball destroys this brick, **Then** the ball's velocity no longer decreases due to gravity and floats horizontally with constant velocity

2. **Given** a level with gravity brick 22 (2G - Moon gravity), **When** the ball destroys this brick, **Then** the ball falls at Moon gravity acceleration (2.0 units/sec²) on the Y-axis, and X/Z gravity components are 0

3. **Given** a level with gravity brick 23 (10G - Earth gravity), **When** the ball destroys this brick, **Then** the ball falls at Earth gravity acceleration (10.0 units/sec²) on the Y-axis

4. **Given** a level with gravity brick 24 (20G - High gravity), **When** the ball destroys this brick, **Then** the ball falls at high gravity acceleration (20.0 units/sec² on Y-axis)

5. **Given** a level with gravity brick 25 (Queer Gravity), **When** the ball destroys this brick, **Then** the ball experiences random direction gravity with X component between -2.0 and +15.0, Y component always 0, and Z component between -5.0 and +5.0

6. **Given** multiple gravity bricks in sequence have been destroyed, **When** the third brick is destroyed, **Then** the gravity immediately switches to the third brick's gravity value (no interpolation or delay)

---

### User Story 2 - Gravity Resets on Ball Loss (Priority: P1)

When the player loses a ball (life), the gravity automatically resets to the gravity value defined for the current level's default settings.
This prevents gravity changes from persisting across ball respawns and maintains consistent level rules.

**Why this priority**: Critical for proper game mechanics - without gravity reset, level progression becomes unpredictable and unfair.
Players expect gravity to be consistent at the start of each ball.

**Independent Test**: Can be tested by destroying a gravity brick (changing gravity), then intentionally losing the ball (letting it fall off screen or hit a deadly brick), and verifying gravity returns to the level's default configuration on the next ball spawn.

**Acceptance Scenarios**:

1. **Given** a level with default gravity of 10.0 on Y-axis, **When** the player destroys a zero gravity brick and then loses a ball, **Then** the gravity resets to the level's default (10.0) before the next ball spawns

2. **Given** gravity has been set to 20G (high gravity), **When** the ball is lost, **Then** subsequent balls spawn with the level's original gravity configuration

3. **Given** a level with multiple gravity bricks and dynamic gravity changes, **When** the player loses a life, **Then** the game world gravity immediately reverts to the level's starting gravity configuration (no animation or delay)

---

### User Story 3 - Gravity Bricks Award Points When Destroyed (Priority: P1)

Each gravity brick type awards the player a specific point value when destroyed, as defined in the brick documentation.
This maintains scoring consistency with other brick types in the game.

**Why this priority**: Scoring is fundamental to the game's progression and player motivation.
Gravity bricks must integrate into the existing scoring system seamlessly.

**Independent Test**: Can be tested by destroying individual gravity bricks and verifying the score increases by the correct amount.
Works independently of gravity mechanic.

**Acceptance Scenarios**:

1. **Given** the player destroys gravity brick 21 (Zero Gravity), **Then** the player's score increases by 125 points

2. **Given** the player destroys gravity brick 22 (2G - Light gravity), **When** the score updates, **Then** the score increases by 75 points

3. **Given** the player destroys gravity brick 23 (10G - Normal gravity), **Then** the score increases by 125 points

4. **Given** the player destroys gravity brick 24 (20G - High gravity), **Then** the score increases by 150 points

5. **Given** the player destroys gravity brick 25 (Queer Gravity), **Then** the score increases by 250 points

---

### User Story 4 - Multiple Gravity Changes in Sequence (Priority: P2)

The player can destroy multiple gravity bricks in sequence, each one immediately overriding the previous gravity setting.
The game smoothly transitions between gravity states without glitches or missed updates.

**Why this priority**: Enhances gameplay depth by allowing level designers to create sequences of gravity challenges.
Lower priority because single gravity brick destruction works first.

**Independent Test**: Can be tested by destroying 3+ gravity bricks of different types in sequence and verifying each gravity change applies correctly without state corruption.

**Acceptance Scenarios**:

1. **Given** a level with 3 gravity bricks (21, 24, 22), **When** destroyed in sequence, **Then** gravity transitions: zero → high → light without any conflicting states

2. **Given** two gravity bricks destroyed in rapid succession, **When** both messages arrive, **Then** the most recently triggered gravity is applied (no message queueing issues)

---

### Edge Cases

- What happens if a gravity brick is destroyed while a ball is mid-flight? (Gravity applies immediately, ball trajectory updates smoothly)
- What happens if zero gravity is active and the player loses a ball? (Gravity resets to default before ball respawns)
- Can a gravity brick be destroyed by a paddle hit (brick 57 can only be destroyed by paddle)? (No - gravity bricks are only destroyed by ball)
- What happens if the level's default gravity is never explicitly set? (System uses Bevy's standard gravity configuration or documented fallback)
- What if Queer Gravity generates contradictory values on random calculation? (RNG is independently seeded and pure; edge cases like exact zero are permissible per specification)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST apply gravity based on brick type immediately when brick index 21 (Zero Gravity) is destroyed, setting gravity to (0.0, 0.0, 0.0)

- **FR-002**: System MUST apply gravity based on brick type immediately when brick index 22 (2G) is destroyed, setting gravity to (2.0, 0.0, 0.0) representing Moon-like gravity

- **FR-003**: System MUST apply gravity based on brick type immediately when brick index 23 (10G) is destroyed, setting gravity to (10.0, 0.0, 0.0) representing Earth gravity

- **FR-004**: System MUST apply gravity based on brick type immediately when brick index 24 (20G) is destroyed, setting gravity to (20.0, 0.0, 0.0) representing high gravity

- **FR-005**: System MUST apply gravity based on brick type immediately when brick index 25 (Queer Gravity) is destroyed, applying random gravity with X ∈ [-2.0, +15.0], Y = 0.0, Z ∈ [-5.0, +5.0] using the `rand` crate for random number generation

- **FR-006**: System MUST use the `GravityChanged` message event to communicate gravity updates from the brick destruction system to the physics system

- **FR-007**: System MUST detect life loss events and trigger gravity reset to the level's configured default gravity before the next ball spawns

- **FR-008**: System MUST award 125 points when gravity brick 21 (Zero Gravity) is destroyed

- **FR-009**: System MUST award 75 points when gravity brick 22 (2G) is destroyed

- **FR-010**: System MUST award 125 points when gravity brick 23 (10G) is destroyed

- **FR-011**: System MUST award 150 points when gravity brick 24 (20G) is destroyed

- **FR-012**: System MUST award 250 points when gravity brick 25 (Queer Gravity) is destroyed

- **FR-013**: System MUST store the level's default gravity configuration in level metadata (RON format) and restore it when a life is lost; if a level does not specify default gravity, the system MUST use zero gravity (0.0, 0.0, 0.0) as the fallback

- **FR-014**: System MUST apply gravity changes to the ball's physics body only; paddle and enemies maintain their standard physics behavior independent of gravity brick activation

- **FR-015**: System MUST apply gravity changes to the ball without causing physics state corruption or unexpected entity behavior in other game systems

### Key Entities

- **GravityConfiguration**: Stores gravity vector (x, y, z) for the current game world state; updated by `GravityChanged` message

- **BrickDestructionEvent**: Trigger that identifies which brick was destroyed; system filters for gravity brick indices (21-25) and sends corresponding `GravityChanged` message

- **GravityChanged**: Message containing the new gravity vector and triggered when a gravity brick is destroyed

- **LifeSystem**: Manages ball loss events and triggers gravity reset to level default

- **LevelMetadata**: RON data structure containing the level's default gravity configuration

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All 5 gravity brick types (21-25) correctly apply their configured gravity values immediately upon destruction, verified by physics test assertions

- **SC-002**: Gravity resets to level default before the next ball spawns after a life is lost, with no gravity state persisting into the next ball spawn (timing delay during respawn sequence is acceptable)

- **SC-003**: Score updates correctly for all gravity brick types (75, 125, 150, 250 points), verified by gameplay and unit tests

- **SC-004**: Message-based gravity updates trigger with no perceptible lag (gravity applies in same frame as brick destruction detection)

- **SC-005**: All gravity bricks can be destroyed in sequence without state corruption, physics conflicts, or unexpected behavior

- **SC-006**: Gravity changes to the ball work consistently without interfering with paddle collision, enemy physics, or ball trajectory calculations in other systems

- **SC-007**: Zero gravity brick (21) allows ball to float horizontally without downward acceleration, maintaining constant vertical velocity

- **SC-008**: Queer Gravity brick (25) generates random gravity within the specified ranges on each destruction with no bias or correlation to previous values

### Assumptions

- Level default gravity is explicitly configured in each level's RON metadata file; if not specified, defaults to zero gravity (0.0, 0.0, 0.0); existing levels without gravity metadata require no migration and automatically use the zero gravity fallback
- Gravity uses standard Bevy Y-axis convention (positive = up, negative = down)
- RNG for Queer Gravity uses the `rand` crate for deterministic, reproducible randomization
- Physics system uses standard gravity application via Rapier 3D integration in Bevy
- Brick indices 21-25 are reserved exclusively for gravity bricks (no other brick type uses these indices)
- Life loss is detected via existing ball/paddle collision or score system events
- Gravity changes apply only to the ball's physics body; paddle and enemies maintain standard physics
