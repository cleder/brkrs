# Feature Specification: Extra Ball Brick (Brick 41)

**Feature Branch**: `019-extra-ball-brick` **Created**: 2026-01-10 **Status**: Draft **Input**: User description: "add brick 41, Extra ball.
This is a distructable brick, when it is hit by the ball it adds an additional life, it awards 0 points; it emits an unique sound when destroyed; the next available spec number is 019"

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, UI updates, or hierarchy, the implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include at least one check that guards against prohibited patterns (e.g., panicking queries or per-frame UI updates without `Changed<T>`).
Acceptance criteria MUST explicitly state which event system is used (Messages vs Observers), justify the choice, and check for **Message-Event Separation** (correct use of `MessageWriter` vs observers/ `Trigger<T>`) and **Hierarchy Safety** (use of `commands.entity(parent).add_child(child)` or `EntityCommands::set_parent`).

**COORDINATE SYSTEM REQUIREMENT**: If the feature involves spatial movement, physics velocity, or directional behavior, the specification MUST include a coordinate system note clarifying:

- Which axes are used for movement (XZ plane for horizontal, Y for vertical, etc.)
- Whether directional terms (forward/backward/left/right) refer to Bevy's Transform API convention (forward = -Z), gameplay-relative directions (player perspective), or direct axis manipulation (±X, ±Y, ±Z)
- How the camera view orientation affects gameplay directions
- Any locked axes via `LockedAxes` constraints

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

### User Story 1 - Gain Extra Life on Brick 41 (Priority: P1)

Players want a brick that rewards an extra life when they hit it so they can extend their run.

**Why this priority**: Directly affects player survivability and game feel; must work before balancing points or audio.

**Independent Test**: Load a level containing only brick 41 and a ball, hit the brick once, and verify life increase and brick removal without affecting score.

**Acceptance Scenarios**:

1. **Given** a level containing a single brick 41 and the player at $L$ lives with a configured max life cap $L_{max}$, **When** the ball collides with the brick and destroys it using the standard brick-hit path, **Then** one life is awarded via the game Message channel, the life total becomes $\min(L+1, L_{max})$, the brick despawns, and no score change occurs (0 points added).
2. **Given** multiple balls in play and a single brick 41, **When** exactly one ball collision triggers the destruction, **Then** the life award happens once (no duplicate awards), the Message-event separation is respected (life change emitted as a Message; no observer panics), and the brick cannot be hit again.

---

### User Story 2 - Unique Audio Feedback (Priority: P2)

Players want distinct audio feedback when brick 41 is destroyed so they can instantly recognize the extra-life reward.

**Why this priority**: Reinforces reward feedback; audible distinction avoids confusion with other bricks.

**Independent Test**: In a level with brick 41 and other brick types, destroy brick 41 and confirm only its unique destruction sound plays once, even if other bricks are destroyed nearby.

**Acceptance Scenarios**:

1. **Given** a level containing brick 41 and other bricks with their own sounds, **When** brick 41 is destroyed, **Then** exactly one unique audio cue for brick 41 plays via the chosen audio Message, no other brick sound is substituted, and replay is prevented on subsequent collisions because the brick is already despawned.

---

### Edge Cases

- Life cap reached: when current lives already at the configured maximum, the award is clamped and the UI/audio still fires once.
- Corrupted life state: if `current > max` or `current < 0` (defensive check), clamp to `[0, max]` and log warning; life award still processes normally after clamping.
- Multi-ball simultaneous hits: only the first collision grants the life; subsequent hits on the despawned brick do nothing.
- Sound fallback: if the unique audio asset is missing or fails to load, gameplay proceeds and a generic brick sound plays once (still 0 points and life award behavior validated).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Define brick type 41 "Extra Ball" as a destructible brick with 0 point value and standard single-hit durability; it must be available in level definitions/asset metadata alongside existing bricks.
- **FR-002**: On the first valid collision between a ball and brick 41, the system MUST award exactly +1 player life (clamped to the configured maximum lives) via the game Message channel, then destroy the brick so it cannot be hit again.
- **FR-003**: Destroying brick 41 MUST NOT change the score: no points are added, and score multipliers/combo chains are unaffected.
- **FR-004**: Destroying brick 41 MUST trigger a unique destruction sound once; if the dedicated sound asset is unavailable, fall back to a generic brick sound without blocking gameplay.
- **FR-005**: Message-event separation MUST be maintained: life-award and audio triggers are emitted as Messages; observers or systems must not panic on missing components, and hierarchy updates (if any) use safe parent-child APIs.
- **FR-006**: Acceptance tests MUST set up and assert behavior using Bevy 0.17-compliant patterns (no per-frame UI mutation without `Changed<T>`, no panicking queries), and must include a failing-first commit per TDD requirement.

### Key Entities *(include if feature involves data)*

- **Brick 41 (Extra Ball)**: Destructible brick definition with id 41, durability 1, score value 0, references a unique destruction sound, participates in standard brick collision handling.
- **Player Lives Counter**: Tracks current lives and maximum lives cap; receives life-award Messages and exposes current total for UI/testing.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of test runs, destroying brick 41 increases lives by exactly +1 within one game tick and never exceeds the configured max.
- **SC-002**: In 100% of test runs, destroying brick 41 adds 0 points and leaves any active score multipliers/combos unchanged.
- **SC-003**: In 100% of test runs, the brick 41 destruction sound plays once; no duplicate or wrong sounds play even with simultaneous nearby events.
- **SC-004**: Automated acceptance tests for User Stories 1 and 2 execute in CI and pass; no Bevy 0.17 mandate violations (message-event separation, safe hierarchy updates, no panicking queries) are reported.

### Assumptions

- The game already defines a maximum lives cap; awarding an extra life clamps to that cap.
- Existing brick placement/grid coordinates remain unchanged; brick 41 uses the standard brick transform conventions (XZ plane for layout, Y for vertical stacking if any) and no new movement logic is introduced.
- Audio system supports mapping a unique sound asset key to brick 41; a generic brick sound is available for fallback.
