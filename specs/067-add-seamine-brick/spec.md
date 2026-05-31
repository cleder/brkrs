# Feature Specification: Sea Mine Brick

**Feature Branch**: `067-add-seamine-brick` **Created**: 2026-05-31 **Status**: Draft **Input**: User description: "add the seamine brick.
When the brick is destroyed a spiky ball like a sea mine is spawned. see experiments/spiky_ball/src/main.rs when the mine hits a brick with index > 90, the paddle or a wall it explodes, the size/blast radius of the explosion is 30. balls or paddles in the blast radius of the explosion are destroyed.
When the paddle is destroyed a life is lost"

## Clarifications

### Session 2026-05-31

- Q: How should a newly spawned sea mine move when it appears?
  -> A: It starts with initial velocity in an arbitrary direction and an initial spin, similar to the merkaba hazard.
- Q: Which minimum motion thresholds should the sea mine maintain after spawn?
  -> A: Minimum linear speed 3.0 u/s and minimum spin 180 deg/s.
- Q: What is the sea mine brick index?
  -> A: 31 for both the sea mine brick and the spawned mine's source brick entry.

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: This feature touches ECS gameplay state, spawned entities, physics collisions, and life-loss handling.
Acceptance scenarios MUST verify message-based gameplay flow for explosion consequences and life loss, and MUST confirm hierarchy-safe parenting when the spawned sea mine visual is assembled from multiple child meshes.

**COORDINATE SYSTEM REQUIREMENT**: Gameplay movement remains on the existing horizontal playfield, with X used for lateral movement, Z used for forward/backward travel across the arena, and Y used for vertical placement.
A spawned sea mine begins with linear velocity in an arbitrary gameplay direction, must maintain at least 3.0 u/s of speed, and must maintain at least 180 deg/s of angular velocity for visible spin.
Explosion radius is measured in world-space distance from the mine detonation point.
Camera orientation does not redefine trigger directions; collision checks use world-space positions.

**MULTI-FRAME PERSISTENCE REQUIREMENT**: This feature introduces runtime projectile state and explosion-driven destruction.
Tests MUST verify spawned sea mines persist correctly across at least 10 consecutive `app.update()` frames until a trigger collision occurs, and MUST verify life-loss and destroyed-entity state are not overwritten by later frames.

### User Story 1 - Release a Sea Mine From a Brick (Priority: P1)

As a player, when I destroy a sea mine brick, a spiky sea mine hazard is released into the arena instead of the interaction ending at brick destruction.

**Why this priority**: The spawned sea mine is the defining behavior of the brick.
Without it, the feature does not exist.

**Independent Test**: Destroy a sea mine brick in isolation and verify the brick is removed while exactly one sea mine entity appears with the expected hazard identity and remains active over multiple frames.

**Acceptance Scenarios**:

1. **Given** a level containing one sea mine brick, **When** the brick is destroyed by the normal brick-destruction flow, **Then** the brick is removed and exactly one sea mine hazard is spawned from that brick instance.
2. **Given** a sea mine brick is destroyed, **When** the next frame is processed, **Then** the spawned sea mine appears at the destroyed brick's location and is visually identifiable as a spiky mine.
3. **Given** a sea mine brick is destroyed, **When** the spawned sea mine initializes, **Then** it starts with linear velocity in an arbitrary gameplay direction and angular velocity that spins like a merkaba.
4. **Given** a spawned sea mine is active, **When** later frames or collision responses would reduce its motion below the allowed floor, **Then** the sea mine maintains at least 3.0 u/s linear speed and at least 180 deg/s angular velocity until it detonates or is otherwise removed.
5. **Given** a spawned sea mine and no trigger collision has occurred, **When** the game advances 10 update frames, **Then** the same sea mine remains active and no explosion is triggered.
6. **Given** the spawned sea mine uses child meshes for spikes, **When** the entity hierarchy is created, **Then** the parent-child relationship is built with hierarchy-safe commands and does not orphan any visual children.

---

### User Story 2 - Detonate on Hazard Contact (Priority: P1)

As a player, when the released sea mine reaches a wall, the paddle, or a brick with an index greater than 90, it detonates and destroys nearby balls and the paddle within a blast radius of 30.

**Why this priority**: The explosion rules define the mine's gameplay threat and must be reliable for the feature to be understandable and fair.

**Independent Test**: Spawn a sea mine near each valid trigger type and verify detonation occurs once, centered on the collision point, with only balls and the paddle inside radius 30 being destroyed.

**Acceptance Scenarios**:

1. **Given** an active sea mine, **When** it collides with any wall collider, **Then** the sea mine detonates within 1 frame.
2. **Given** an active sea mine, **When** it collides with the paddle, **Then** the sea mine detonates within 1 frame.
3. **Given** an active sea mine, **When** it collides with a brick whose index is greater than 90, **Then** the sea mine detonates within 1 frame.
4. **Given** an active sea mine, **When** it collides with a brick whose index is 90 or lower, **Then** the sea mine does not detonate from that collision alone.
5. **Given** a sea mine detonates, **When** balls are located at distances of 29 and 31 units from the detonation point, **Then** the ball at 29 units is destroyed and the ball at 31 units remains in play.
6. **Given** a sea mine detonates and the paddle center lies within 30 units of the detonation point, **When** explosion consequences are processed, **Then** the paddle is destroyed.
7. **Given** a sea mine detonates and multiple balls are within 30 units, **When** explosion consequences are processed, **Then** every ball within the blast radius is destroyed in the same resolution step.
8. **Given** a detonation has started, **When** explosion handling resolves, **Then** the triggering sea mine is removed exactly once and does not detonate a second time.
9. **Given** explosion outcomes are emitted, **When** gameplay state updates are applied, **Then** destruction and life-loss propagation uses the project's message-driven gameplay flow rather than observer-only side effects.

---

### User Story 3 - Preserve Existing Progression Rules (Priority: P2)

As a level designer and player, I need the sea mine brick to fit existing level progression and life-loss rules so the new hazard behaves predictably inside normal gameplay.

**Why this priority**: The feature must integrate cleanly with scoring, completion, and life management instead of introducing special-case regressions.

**Independent Test**: Place sea mine bricks in a level, destroy them during play, and verify level-completion bookkeeping and life-loss flow remain consistent with existing rules.

**Acceptance Scenarios**:

1. **Given** a sea mine brick is a completion-relevant destructible brick, **When** the brick is destroyed, **Then** the level-completion counter updates using the same progression rules as other destructible bricks.
2. **Given** a sea mine explosion destroys the paddle, **When** standard gameplay consequences resolve, **Then** exactly one life is lost and the normal respawn or game-over flow begins.
3. **Given** the paddle has already been destroyed by the same detonation, **When** later systems in the frame run, **Then** no second life loss is recorded from that single explosion.
4. **Given** a sea mine explosion occurs near ordinary bricks, **When** the blast radius overlaps those bricks, **Then** only balls and the paddle are removed by the explosion unless a separate existing brick-destruction rule also applies.
5. **Given** multiple sea mine bricks are present in a level file, **When** the level loads, **Then** each one spawns as the new sea mine brick type and can independently release its own mine when destroyed.

### Edge Cases

- A sea mine spawns or travels within 30 units of the paddle without touching a trigger surface: no explosion occurs until a valid trigger collision happens.
- A sea mine spawns with an arbitrary initial travel direction that points away from nearby trigger surfaces: it remains active and does not explode until a later valid collision occurs.
- A bounce or physics response would otherwise reduce sea mine motion below its minimum thresholds: speed and spin are restored to the minimum floor instead of letting the hazard stall.
- A detonation destroys multiple balls in one frame: all balls inside radius are removed together, while balls outside radius remain unaffected.
- A sea mine detonates while the paddle is already absent because of an earlier loss: no additional life is removed.
- A detonation overlaps bricks with indices greater than 90 that were not the trigger target: those bricks are not removed by blast radius alone.
- A second sea mine caught inside another mine's blast radius is destroyed as a ball target but does not create a chained secondary explosion unless it separately collides with its own trigger.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST define sea mine brick type 31 that can be placed in levels and identified distinctly from existing brick types.
- **FR-002**: System MUST spawn exactly one sea mine hazard when a sea mine brick is destroyed through the normal brick-destruction flow.
- **FR-003**: System MUST spawn the sea mine at the destroyed brick's playfield position.
- **FR-004**: System MUST represent the spawned hazard as a visually spiky mine so players can distinguish it from a normal ball.
- **FR-005**: System MUST assign each spawned sea mine initial linear velocity in an arbitrary gameplay direction.
- **FR-006**: System MUST assign each spawned sea mine initial angular velocity so it visibly spins after spawning.
- **FR-007**: System MUST maintain at least 3.0 u/s linear speed for each active sea mine until it detonates or is otherwise removed.
- **FR-008**: System MUST maintain at least 180 deg/s angular velocity for each active sea mine until it detonates or is otherwise removed.
- **FR-009**: System MUST keep a spawned sea mine active until it detonates or another existing cleanup rule removes it.
- **FR-010**: System MUST detonate a sea mine when it collides with a wall, the paddle, or a brick whose index is greater than 90.
- **FR-011**: System MUST NOT detonate a sea mine solely because it collided with a brick whose index is 90 or lower.
- **FR-012**: System MUST apply explosion effects using a blast radius of 30 world units measured from the detonation point.
- **FR-013**: System MUST destroy every ball entity whose position lies within the blast radius when a sea mine detonates.
- **FR-014**: System MUST destroy the paddle when the paddle lies within the blast radius when a sea mine detonates.
- **FR-015**: System MUST trigger exactly one life loss when a sea mine explosion destroys the paddle.
- **FR-016**: System MUST remove the detonating sea mine after its explosion is resolved so it cannot explode twice.
- **FR-017**: System MUST preserve existing brick behavior for non-trigger collisions and MUST NOT use explosion radius alone to destroy bricks.
- **FR-018**: System MUST allow multiple sea mines and multiple balls to be resolved correctly in the same level without duplicating life-loss or detonation handling for a single mine.
- **FR-019**: System MUST integrate sea mine destruction and paddle life-loss consequences through the project's existing gameplay messaging flow.
- **FR-020**: System MUST keep sea mine brick destruction compatible with normal completion tracking for destructible bricks.
- **FR-021**: System MUST support loading sea mine bricks from level data using the same authoring path as other brick types.

### Key Entities *(include if feature involves data)*

- **Sea Mine Brick**: A destructible brick (index 31) that releases one sea mine hazard when destroyed.
- **Sea Mine Hazard**: A spawned spiky projectile that begins with arbitrary initial travel direction and spin, maintains at least 3.0 u/s linear speed and 180 deg/s angular velocity, then travels through the arena until it detonates on a valid trigger collision.
- **Sea Mine Explosion**: The area-of-effect result centered on the mine's detonation point with a radius of 30 world units.
- **Explosion Target**: A paddle or ball entity that can be removed when positioned within an active sea mine explosion radius.
- **Hazard Trigger Surface**: A wall, the paddle, or any brick whose index is greater than 90 and that causes a sea mine to detonate on contact.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of destruction tests, destroying one sea mine brick spawns exactly one sea mine hazard within 1 frame.
- **SC-002**: In 100% of spawn-motion tests, a newly spawned sea mine has arbitrary travel direction with initial speed of at least 3.0 u/s and angular velocity of at least 180 deg/s on its first active frame.
- **SC-003**: In 100% of motion-maintenance tests, active sea mine speed never falls below 3.0 u/s and active spin never falls below 180 deg/s before detonation or cleanup.
- **SC-004**: In 100% of trigger-collision tests, sea mines detonate when contacting a wall, the paddle, or a brick with index greater than 90.
- **SC-005**: In 100% of non-trigger tests, collisions with bricks of index 90 or lower do not detonate the mine unless another valid trigger is also present.
- **SC-006**: In radius-boundary tests, entities at distances less than or equal to 30 units are destroyed and entities beyond 30 units survive in 100% of cases.
- **SC-007**: In 100% of paddle-destruction tests, a mine explosion that removes the paddle records exactly one life loss and starts the existing follow-up flow.
- **SC-008**: In multi-frame persistence tests spanning at least 10 updates, spawned sea mines remain active until a valid trigger collision and explosion consequences are not overwritten afterward.
- **SC-009**: In level-loading tests, 100% of configured sea mine bricks load as playable instances of the new brick type.

## Assumptions

- Sea mine bricks follow the existing destruction, scoring, and completion rules for ordinary destructible bricks unless this feature explicitly overrides them.
- A collision with a brick whose index is greater than 90 is only an explosion trigger; that brick's own durability rules remain unchanged unless another rule destroys it.
- Explosion effects apply only to balls and the paddle; they do not automatically destroy bricks, walls, or unrelated gameplay entities.
- If another sea mine is inside the blast radius, it is removed as a ball-like explosion target without creating a chained secondary explosion by default.
- Standard respawn, life display, and game-over handling continue to be owned by the existing life-loss flow once the paddle is destroyed.
