# Feature Specification: Paddle-Destroyable Brick (Type 57)

**Feature Branch**: `022-paddle-destroyable-brick` **Created**: 2026-01-13 **Status**: Draft **Input**: User description: "implement brick type 57, paddle destroyable brick.
This brick will be destroyed on contact with the paddle, the ball bounces off the brick.
On destruction 250 points are awarded, the ball cannot destroy the brick, no points are awarded when the ball hits the brick.
The brick (countsTowardsCompletion), this brick contributes to level completion - all bricks of this type must be destroyed"

## Clarifications

### Session 2026-01-13

- Q: What is the maximum number of paddle-destroyable bricks allowed in a single level? → A: No hard limit - as many as level designer places
- Q: Should paddle-brick collision events be logged for debugging purposes? → A: Include collision event logging at DEBUG level
- Q: What should happen if a paddle-destroyable brick spawns overlapping the paddle at level start? → A: Brick immediately destroyed on first frame, 250 points awarded

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, UI updates, or hierarchy, the implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include at least one check that guards against prohibited patterns (e.g., panicking queries or per-frame UI updates without `Changed<T>`).
Acceptance criteria MUST explicitly state which event system is used (Messages vs Observers), justify the choice, and check for **Message-Event Separation** (correct use of `MessageWriter` vs observers/ `Trigger<T>`) and **Hierarchy Safety** (use of `commands.entity(parent).add_child(child)` or `EntityCommands::set_parent`).

**COORDINATE SYSTEM REQUIREMENT**: Not applicable - brick type is stationary and does not involve movement or directional behavior.

**MULTI-FRAME PERSISTENCE REQUIREMENT**: If the feature involves runtime state changes (gravity, scores, powerup effects, or any resource/component modified during gameplay), acceptance scenarios MUST include multi-frame persistence checks:

- Tests MUST verify state persists across multiple `app.update()` cycles (minimum 10 frames)
- Tests MUST include ALL systems that write to the affected resource/component to catch per-frame overwrite bugs
- This requirement exists because single-frame assertions miss bugs where initialization or cleanup systems unconditionally overwrite runtime state (see 020-gravity-bricks retrospective)

### User Story 1 - Paddle Destroys Brick on Contact (Priority: P1)

When the player moves the paddle to touch a paddle-destroyable brick, the brick is immediately destroyed, awards 250 points, and the brick is removed from the level.
The level can be completed when all paddle-destroyable bricks (and other completion-required bricks) are destroyed.

**Why this priority**: Core mechanic - this is the primary interaction that defines this brick type.
Without this, the brick serves no purpose.

**Independent Test**: Can be fully tested by spawning a paddle-destroyable brick, moving the paddle to collide with it, and verifying the brick is destroyed, 250 points are awarded, and the brick counts toward level completion.

**Acceptance Scenarios**:

1. **Given** a level with one paddle-destroyable brick (type 57), **When** the paddle collides with the brick, **Then** the brick entity is despawned within 1 frame
2. **Given** a paddle-destroyable brick exists, **When** paddle contact occurs, **Then** exactly 250 points are added to the player's score
3. **Given** a level with only paddle-destroyable bricks, **When** all paddle-destroyable bricks are destroyed by paddle contact, **Then** the level completion condition is met
4. **Given** score is tracked across multiple frames, **When** a paddle-destroyable brick is destroyed, **Then** the 250-point award persists for at least 10 frames (multi-frame persistence check)
5. **Given** a paddle-destroyable brick, **When** paddle collision is detected, **Then** the brick destruction uses Messages (via `MessageWriter`) for consistency with existing brick destruction patterns, NOT observers
6. **Given** the brick entity has a parent in the hierarchy, **When** the brick is destroyed, **Then** the destruction uses `commands.entity(brick).despawn_recursive()` to ensure hierarchy safety

---

### User Story 2 - Ball Bounces Off Brick Without Destruction (Priority: P1)

When the ball collides with a paddle-destroyable brick, the ball bounces off (reflects) as if hitting a normal wall, the brick remains intact, and no points are awarded.

**Why this priority**: Critical inverse behavior - defines what does NOT destroy the brick.
Without this, the brick would be identical to a normal destructible brick.

**Independent Test**: Can be fully tested by spawning a paddle-destroyable brick, launching a ball to collide with it, and verifying the ball bounces, the brick remains, and no points are awarded.

**Acceptance Scenarios**:

1. **Given** a paddle-destroyable brick and a moving ball, **When** the ball collides with the brick, **Then** the ball reflects according to standard physics (angle of incidence equals angle of reflection)
2. **Given** a paddle-destroyable brick and a ball collision, **When** the collision occurs, **Then** the brick entity is NOT despawned
3. **Given** a paddle-destroyable brick, **When** the ball hits the brick, **Then** zero points are added to the player's score
4. **Given** a ball has collided with a paddle-destroyable brick, **When** 10 frames have passed, **Then** the brick entity still exists (multi-frame persistence check)
5. **Given** a paddle-destroyable brick, **When** ball collision is detected, **Then** the collision event handling uses the existing physics system (bevy_rapier3d) contact events, NOT custom observers

---

### User Story 3 - Brick Type Configuration in Level Files (Priority: P2)

Level designers can place paddle-destroyable bricks in level files using brick type 57, specifying the brick material, position, and rotation.
The brick correctly loads with all required components.

**Why this priority**: Enables content creation - without level file support, the brick can only be tested programmatically.

**Independent Test**: Can be fully tested by creating a level file with a type 57 brick, loading the level, and verifying the brick spawns with correct properties.

**Acceptance Scenarios**:

1. **Given** a level RON file with a brick entry `{ brick_type: PaddleDestroyable }`, **When** the level is loaded, **Then** a paddle-destroyable brick entity is spawned
2. **Given** a paddle-destroyable brick in a level file, **When** the level loads, **Then** the brick has all required components: Transform, BrickType(PaddleDestroyable), Collider, countsTowardsCompletion=true
3. **Given** a level file with 3 paddle-destroyable bricks, **When** the level loads, **Then** exactly 3 paddle-destroyable brick entities exist in the world
4. **Given** a level with paddle-destroyable bricks is loaded, **When** 10 frames have passed, **Then** all paddle-destroyable bricks still exist and maintain their properties (multi-frame persistence check)

---

### Edge Cases

- What happens when the paddle and ball simultaneously contact the brick?
  The paddle contact takes precedence - brick is destroyed, 250 points awarded
- What happens if multiple paddle-destroyable bricks are touched by the paddle in one frame?
  Each brick is independently destroyed and awards 250 points (resulting in 500 points for 2 bricks, etc.)
- What happens if the brick is destroyed while the ball is inside the brick's collider?
  The ball continues its trajectory unaffected since the brick no longer exists to generate collision events
- What happens if all remaining bricks in the level are paddle-destroyable bricks?
  Level completion requires the player to move the paddle to touch all bricks (ball alone cannot complete the level)
- What happens when a paddle-destroyable brick is part of a compound/nested entity structure?
  The entire brick entity (and any children) must be despawned using `despawn_recursive()`
- What happens if a paddle-destroyable brick spawns overlapping the paddle at level start?
  The brick is immediately destroyed on the first frame after level load, and 250 points are awarded

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST define a new brick type identifier "PaddleDestroyable" (type 57) that can be stored in level files and loaded at runtime
- **FR-002**: System MUST detect collision events between the paddle entity and paddle-destroyable brick entities
- **FR-003**: System MUST immediately despawn paddle-destroyable brick entities upon paddle collision
- **FR-004**: System MUST award exactly 250 points when a paddle-destroyable brick is destroyed by paddle contact
- **FR-005**: System MUST reflect the ball's velocity according to physics when the ball collides with a paddle-destroyable brick (ball bounces off)
- **FR-006**: System MUST NOT despawn paddle-destroyable brick entities when the ball collides with them
- **FR-007**: System MUST NOT award any points when the ball collides with a paddle-destroyable brick
- **FR-008**: System MUST mark paddle-destroyable bricks with countsTowardsCompletion=true, requiring their destruction for level completion
- **FR-009**: System MUST count paddle-destroyable bricks in the total completion requirement calculation
- **FR-010**: System MUST decrement the remaining completion count when a paddle-destroyable brick is destroyed
- **FR-011**: System MUST persist the brick's destruction state (once destroyed, it cannot respawn) across all frames until level completion or restart
- **FR-012**: System MUST use the Message system (via `MessageWriter`) for brick destruction events to maintain consistency with existing brick destruction patterns
- **FR-013**: System MUST ensure paddle-destroyable bricks can be configured in level RON files with the same structure as other brick types (position, rotation, material)
- **FR-014**: System MUST log paddle-brick collision events at DEBUG level using the tracing framework for troubleshooting and development purposes

### Key Entities

- **Paddle-Destroyable Brick (Type 57)**: A brick entity that is destroyed only by paddle contact (not ball contact), awards 250 points on destruction, counts toward level completion, and causes the ball to bounce off when hit.
  Key attributes include:
  - Brick type identifier (PaddleDestroyable/type 57)
  - Collision detection enabled for both paddle and ball
  - Points value: 250 (awarded on paddle destruction only)
  - Completion flag: countsTowardsCompletion = true
  - Physics collider for ball bounce behavior
  - Spatial position and rotation (from level file)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Players can destroy paddle-destroyable bricks by moving the paddle to touch them, with destruction occurring within 1 frame (60ms at 60fps)
- **SC-002**: When a paddle destroys a paddle-destroyable brick, exactly 250 points are added to the score with no exceptions
- **SC-003**: Balls bounce off paddle-destroyable bricks at physically correct angles 100% of the time without destroying the brick
- **SC-004**: Levels containing only paddle-destroyable bricks can be completed by destroying all bricks via paddle contact
- **SC-005**: Level designers can create levels with paddle-destroyable bricks using the same RON file format as other brick types
- **SC-006**: Paddle-destroyable bricks contribute to the completion percentage accurately (e.g., 1 of 5 paddle-destroyable bricks = 20% completion for that brick type)
- **SC-007**: All tests pass including multi-frame persistence checks (minimum 10 frames) for score updates and brick destruction state

## Assumptions

- The existing scoring system can accept point awards from paddle-brick collision events
- The paddle entity has a collision component that can trigger contact events with brick entities
- The ball reflection physics are handled by the existing bevy_rapier3d collision system
- Brick type enumeration can be extended to include type 57 without breaking existing brick types
- Level loader supports extending the brick type parsing to include PaddleDestroyable variant
- The completion tracking system monitors brick destruction events via Messages and updates countsTowardsCompletion totals
- Hierarchy safety is required (use of `despawn_recursive()`) based on existing brick destruction patterns
- Message system (not observers) is used for brick destruction to maintain consistency with existing architecture
- No hard limit on number of paddle-destroyable bricks per level - level designers can place as many as needed without validation constraints

## Technical Context

- Bevy 0.17.3 ECS with bevy_rapier3d 0.32.0 for collision detection
- RON file format for level definitions in `assets/levels/` directory
- Existing brick type system that can be extended with new variants
- Existing scoring system that tracks player points
- Existing completion tracking system that monitors brick destruction
- Message-based event system for game state changes (brick destruction, score updates)

## Out of Scope

- Visual effects or animations for brick destruction (covered in separate feature)
- Sound effects for paddle-brick collision (covered in audio feature)
- Particle effects when brick is destroyed (separate visual enhancement)
- Paddle movement mechanics (already implemented)
- Ball physics beyond standard bounce behavior (already handled by bevy_rapier3d)
- UI display of score changes (existing score display system)
- Difficulty balancing or level design recommendations (game design decision)
- Multi-ball scenarios (assumes single ball; multi-ball would be a separate feature)
- Powerup integration (e.g., paddle size affecting collision area - separate feature)
