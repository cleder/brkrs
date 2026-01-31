# Feature Specification: Level Navigation Bricks (Bricks 50 & 54)

**Feature Branch**: `024-level-navigation-bricks` **Created**: 2026-01-31 **Status**: Draft **Input**: User description: "implement the bricks: 50 (Level Up): Advances to next level, 54 (Level Down): Returns to previous level"

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, UI updates, or hierarchy, the implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include at least one check that guards against prohibited patterns (e.g., panicking queries or per-frame UI updates without `Changed<T>`).
Acceptance criteria MUST explicitly state which event system is used (Messages vs Observers), justify the choice, and check for **Message-Event Separation** (correct use of `MessageWriter` vs observers/ `Trigger<T>`) and **Hierarchy Safety** (use of `commands.entity(parent).add_child(child)` or `EntityCommands::set_parent`).

**COORDINATE SYSTEM REQUIREMENT**: Not applicable - these bricks involve level state transitions rather than spatial movement or physics velocity.

**MULTI-FRAME PERSISTENCE REQUIREMENT**: Acceptance scenarios MUST include multi-frame persistence checks for level state transitions:

- Tests MUST verify the new level state persists across multiple `app.update()` cycles (minimum 10 frames)
- Tests MUST include ALL systems that write to level state to catch per-frame overwrite bugs
- This requirement exists because single-frame assertions miss bugs where initialization or cleanup systems unconditionally overwrite runtime state (see 020-gravity-bricks retrospective)

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.

  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Level Up Brick (Brick 50) (Priority: P1)

Players want a brick that advances them to the next level when hit so they can create strategic progression paths.

**Why this priority**: Core level navigation mechanic; provides game variety and player agency in progression.

**Independent Test**: Load a level containing only brick 50, verify hitting it triggers level transition to the next sequential level, and confirm all game state (lives, score) persists across the transition.

**Acceptance Scenarios**:

1. **Given** the player is on level N with brick 50 present, **When** the ball collides with brick 50 and destroys it, **Then** a level transition Message is emitted to advance to level N+1, the brick despawns, current lives and score are preserved, and the new level loads with its own brick layout.

2. **Given** multiple balls in play and a single brick 50, **When** exactly one ball collision triggers the destruction, **Then** the level transition happens once (no duplicate transitions), Message-event separation is respected (level change emitted as a Message; no observer panics), and the brick cannot be hit again.

3. **Given** the player is on the final level in the level sequence, **When** brick 50 is hit, **Then** the brick is destroyed, 0 points are awarded, and a victory screen is displayed to signal game completion (no level transition occurs).

4. **Given** brick 50 destroyed and level transition in progress, **When** the new level loads, **Then** the level state persists across multiple frame updates (minimum 10 frames) without being overwritten by initialization systems.

---

### User Story 2 - Level Down Brick (Brick 54) (Priority: P2)

Players want a brick that takes them back to the previous level when hit so they can replay easier levels for score farming or practice.

**Why this priority**: Complements level-up functionality; less critical than forward progression but adds gameplay depth.

**Independent Test**: Load a level containing only brick 54, verify hitting it triggers level transition to the previous sequential level, and confirm all game state (lives, score) persists across the transition.

**Acceptance Scenarios**:

1. **Given** the player is on level N (where N > 1) with brick 54 present, **When** the ball collides with brick 54 and destroys it, **Then** a level transition Message is emitted to return to level N-1, the brick despawns, current lives and score are preserved, and the previous level loads with its original brick layout.

2. **Given** the player is on level 1 (the first level), **When** brick 54 is hit, **Then** the brick is destroyed, 0 points are awarded, but no level transition occurs (player remains on level 1).

3. **Given** brick 54 destroyed and level transition in progress, **When** the previous level loads, **Then** the level state persists across multiple frame updates (minimum 10 frames) without being overwritten by initialization systems.

---

### User Story 3 - Unique Audio Feedback for Navigation Bricks (Priority: P3)

Players want distinct audio feedback when navigation bricks (50 and 54) are destroyed so they can recognize level transitions are occurring.

**Why this priority**: Enhances user experience with audio cues; less critical than core functionality.

**Independent Test**: In a level with bricks 50 and 54, destroy each and confirm their unique destruction sounds play once per brick type.

**Acceptance Scenarios**:

1. **Given** a level containing brick 50 and other brick types, **When** brick 50 is destroyed, **Then** exactly one unique audio cue for "level up" plays via the audio Message system, no other brick sound is substituted, and replay is prevented on subsequent collisions because the brick is already despawned.

2. **Given** a level containing brick 54 and other brick types, **When** brick 54 is destroyed, **Then** exactly one unique audio cue for "level down" plays via the audio Message system, distinct from the brick 50 sound.

---

## Clarifications

### Session 2026-01-31

- Q: What should the score values be for navigation bricks 50 and 54? → A: Award 0 points (utility bricks similar to Extra Ball brick 41)

### Edge Cases

- **Last level boundary (brick 50)**: When hitting brick 50 on the final level, the brick is destroyed but no points are awarded, and a victory screen is displayed (no level transition occurs).
- **First level boundary (brick 54)**: When hitting brick 54 on level 1, the brick is destroyed but no points are awarded, and no level transition occurs; player remains on level 1.
- **Corrupted level state**: If current level index is invalid (negative, exceeds level count), clamp to valid range [1, max_level] and log warning; level transition still processes normally after clamping.
- **Missing level data**: If target level file is missing or fails to load, remain on current level and log error; audio feedback still plays but no transition occurs.
- **Multi-ball simultaneous hits**: Only the first collision on a navigation brick triggers the transition; subsequent hits on the despawned brick do nothing.
- **Sound fallback**: If unique audio assets are missing or fail to load, gameplay proceeds with generic brick sound; level transition behavior remains unaffected.
- **Level transition mid-game**: When transitioning, all active balls, powerups, and temporary effects are cleared and reset to the new level's default starting state.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Define brick type 50 "Level Up" as a destructible brick with standard single-hit durability; it must be available in level definitions alongside existing bricks.

- **FR-002**: On the first valid collision between a ball and brick 50, the system MUST emit a level transition Message to advance to the next sequential level, destroy the brick, and preserve player lives and score across the transition.

- **FR-003**: Define brick type 54 "Level Down" as a destructible brick with standard single-hit durability; it must be available in level definitions alongside existing bricks.

- **FR-004**: On the first valid collision between a ball and brick 54, the system MUST emit a level transition Message to return to the previous sequential level, destroy the brick, and preserve player lives and score across the transition.

- **FR-005**: Destroying brick 50 or 54 MUST award 0 points (utility bricks focused on level navigation, not scoring).

- **FR-006**: Destroying brick 50 MUST trigger a unique "level up" destruction sound once; destroying brick 54 MUST trigger a unique "level down" destruction sound once; if dedicated sound assets are unavailable, fall back to generic brick sounds without blocking gameplay.

- **FR-007**: Message-event separation MUST be maintained: level transition and audio triggers are emitted as Messages; systems must not panic on missing components, and hierarchy updates (if any) use safe parent-child APIs.

- **FR-008**: Level transitions MUST handle boundary conditions: on the final level, brick 50 displays a victory screen with no transition; on level 1, brick 54 has no effect and no transition occurs.
  Both bricks award 0 points and despawn regardless of boundary state.

- **FR-009**: When transitioning between levels, the new level state MUST persist across multiple frames (minimum 10 frames) and not be overwritten by initialization or cleanup systems.

- **FR-010**: When a level transition occurs (via brick 50 or 54), the system MUST clear all active balls, powerup effects, and temporary game state, then initialize the target level with its default starting state.

- **FR-011**: Acceptance tests MUST set up and assert behavior using Bevy 0.17-compliant patterns (no per-frame UI mutation without `Changed<T>`, no panicking queries), and must include a failing-first commit per TDD requirement.

### Key Entities *(include if feature involves data)*

- **Brick 50 (Level Up)**: Destructible brick definition with id 50, durability 1, score value 0, references a unique "level up" destruction sound, triggers level advancement on destruction.

- **Brick 54 (Level Down)**: Destructible brick definition with id 54, durability 1, score value 0, references a unique "level down" destruction sound, triggers level regression on destruction.

- **Level State**: Tracks current level index, total available levels, handles level transitions, and ensures state persistence across level changes.

- **Level Transition Message**: Message type emitted when navigation bricks are destroyed, carries target level index and transition direction (up/down).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of test runs, destroying brick 50 advances to the next level within one game tick and preserves all player state (lives, score).

- **SC-002**: In 100% of test runs, destroying brick 54 returns to the previous level within one game tick and preserves all player state (lives, score).

- **SC-003**: In 100% of test runs, level state persists across minimum 10 frames after transition without being overwritten by initialization systems.

- **SC-004**: In 100% of test runs, brick 50 and 54 destruction sounds play once and are distinct from each other and from standard brick sounds.

- **SC-005**: Automated acceptance tests for User Stories 1, 2, and 3 execute in CI and pass; no Bevy 0.17 mandate violations (message-event separation, safe hierarchy updates, no panicking queries) are reported.

- **SC-006**: Boundary condition tests (first level with brick 54, last level with brick 50) execute without crashes and follow defined behavior rules.

### Assumptions

- The game maintains a sequential level progression system with a defined first and last level.
- Level files are stored in a standard location (`assets/levels/`) and follow the existing level format.
- The level loading system supports dynamic level transitions during active gameplay.
- Player state (lives, score) is managed separately from level state and can persist across transitions.
- Audio system supports mapping unique sound asset keys to bricks 50 and 54; generic brick sounds are available for fallback.
- Navigation bricks (50 and 54) are utility bricks that award 0 points, similar to Extra Ball brick 41; this maintains focus on their strategic level-control purpose.
- Level transitions clear all active game elements (balls, powerups, temporary effects) and reset to the new level's default starting state.
