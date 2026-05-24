# Feature Specification: Collision Particle Feedback

**Feature Branch**: `029-collision-particle-feedback` **Created**: 2026-05-24 **Status**: Draft **Input**: User description: "when a ball hits a wall, the paddle or a brick i want to have some visual feedback, particles flying away, a sparkly effect"

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, tests must be written first and included in this spec as testable acceptance scenarios.
Tests must be committed before implementation and a failing-test commit (red) must exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, UI updates, or hierarchy, the implementation must comply with the constitution's Bevy 0.17 mandates and prohibitions.
Acceptance scenarios include checks for no panicking queries, no per-frame broad updates without change triggers, proper Message-Event Separation, and Hierarchy Safety.

**COORDINATE SYSTEM REQUIREMENT**: This feature uses collision positions in gameplay space.
Particle emission direction and spread must be described with direct axis references (plus/minus X, plus/minus Z) or collision normal vectors.
Directional wording like "forward" must be avoided unless explicitly mapped to gameplay-relative direction.

## Clarifications

### Session 2026-05-24

- Q: How should simultaneous or burst collisions be handled for particle spawning limits?
  → A: Spawn an effect for every collision with no cap.
- Q: What lifetime should each particle effect use?
  → A: Short effect, 0.20-0.35 seconds.
- Q: How many particles should each collision spawn?
  → A: Moderate burst, 8-16 particles per collision.
- Q: How should collision effects behave while paused?
  → A: Suppress all new collision effects while paused; do not replay missed effects on resume.
- Q: Where should each collision effect spawn?
  → A: Spawn at exact collision contact point.
- Q: What happens when a brick is destroyed on impact?
  → A: The visual effect still appears for that collision.

### User Story 1 - Immediate Hit Feedback (Priority: P1)

As a player, I want to instantly see a sparkly particle burst whenever the ball collides with a wall, paddle, or brick so that impacts feel responsive and satisfying.

**Why this priority**: This is the core value of the request and directly improves gameplay feel for every collision.

**Independent Test**: This can be tested by running a level with one of each collision type and verifying that each collision triggers one visible, short-lived particle effect at the impact location.

**Acceptance Scenarios**:

1. **Given** gameplay is active and the ball collides with a wall, **When** the collision event is processed, **Then** a visible sparkly particle effect appears near the collision point within the same frame.
2. **Given** gameplay is active and the ball collides with the paddle, **When** the collision event is processed, **Then** a visible sparkly particle effect appears near the collision point within the same frame.
3. **Given** gameplay is active and the ball collides with a brick, **When** the collision event is processed, **Then** a visible sparkly particle effect appears near the collision point within the same frame.
4. **Given** the feature uses immediate visual reaction, **When** event architecture is reviewed, **Then** collision feedback triggering uses observer-style immediate reaction (not buffered messages) and documents the reason for this choice.

---

### User Story 2 - Clear and Non-Disruptive Effects (Priority: P2)

As a player, I want particles to be noticeable but not visually overwhelming so that I can still track the ball and gameplay state clearly.

**Why this priority**: Visual quality matters, but preserving gameplay readability is more important than decorative intensity.

**Independent Test**: This can be tested by triggering repeated collisions and verifying effects stay brief, do not obscure core entities, and expire automatically.

**Acceptance Scenarios**:

1. **Given** repeated collisions occur in quick succession, **When** particle effects are emitted, **Then** each effect completes and despawns within 0.20-0.35 seconds and does not permanently persist on screen.
2. **Given** particle effects are active, **When** a player tracks the ball and paddle, **Then** core gameplay entities remain visible and readable.
3. **Given** parent/child relationships are used for any temporary VFX entities, **When** hierarchy behavior is reviewed, **Then** parent-child links are created only through safe hierarchy commands and not through manual parent/children component mutation.

---

### User Story 3 - Consistent Feedback Across Surfaces (Priority: P3)

As a player, I want hit feedback to feel consistent across walls, paddle, and bricks while still allowing light variation so that impacts look intentional rather than random noise.

**Why this priority**: Consistency reinforces game polish and player trust in collision outcomes.

**Independent Test**: This can be tested by comparing sample collisions across all supported target types and confirming a shared visual language (sparkly burst family) with controlled variation.

**Acceptance Scenarios**:

1. **Given** collisions with wall, paddle, and brick are observed, **When** visual feedback is compared, **Then** all use the same feedback family with small controlled variation in count, size, or spread.
2. **Given** the feature is enabled in a full level playthrough, **When** many collisions occur, **Then** there are no missing effects for supported collision targets.

### Edge Cases

- When multiple collisions are detected in the same frame for the same ball (for example, corner ricochet touching wall and brick), spawn one effect per qualifying collision with no merge, queue, or per-frame cap.
- For collisions near screen boundaries, effects still spawn at the exact collision contact point so visual origin remains physically consistent.
- When a brick is destroyed immediately on impact, spawn the visual effect for that collision at the recorded contact point before brick removal cleanup completes.
- If collision events are emitted while paused or in non-gameplay states, suppress new effects and do not queue or replay missed effects after resume.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system must generate a visible sparkly particle feedback effect whenever the ball collides with a wall.
- **FR-002**: The system must generate a visible sparkly particle feedback effect whenever the ball collides with the paddle.
- **FR-003**: The system must generate a visible sparkly particle feedback effect whenever the ball collides with a brick.
- **FR-004**: The system must place the feedback effect at the exact collision contact point so the visual origin matches player expectations.
- **FR-005**: The system must render feedback effects quickly enough that players perceive them as immediate impact response.
- **FR-006**: The system must keep each feedback effect temporary and automatically remove it after its short display window.
- **FR-007**: The system must maintain visual consistency of feedback across wall, paddle, and brick collisions while allowing controlled variation.
- **FR-008**: The system must avoid generating feedback effects when gameplay is inactive (for example, menus, pause overlays, or non-playing states).
- **FR-009**: The specification and implementation notes must explicitly state that immediate collision feedback uses observer-style event handling, while buffered message queues remain reserved for frame-agnostic streams.
- **FR-010**: The implementation must preserve hierarchy safety rules for any effect entities that use parent-child relationships.
- **FR-011**: For simultaneous or burst collisions, the system must spawn one feedback effect per qualifying collision event with no per-frame cap and no overflow queue.
- **FR-012**: Each feedback effect instance must fully complete (including fade-out) within 0.20-0.35 seconds from spawn.
- **FR-013**: Each qualifying collision must spawn a moderate burst of 8-16 particles.
- **FR-014**: While paused or in non-gameplay states, the system must suppress new collision feedback effects and must not replay missed effects after gameplay resumes.
- **FR-015**: Each feedback effect must spawn at the exact collision contact point.
- **FR-016**: If a brick is destroyed by the collision, the corresponding visual effect must still spawn for that collision at the recorded contact point.

### Key Entities *(include if feature involves data)*

- **Collision Feedback Trigger**: Represents a qualified ball-impact occurrence against one supported collision target (wall, paddle, or brick), including collision location.
- **Feedback Effect Instance**: Represents one temporary visual burst spawned from a trigger, including lifecycle state (active, fading, expired).
- **Feedback Profile**: Represents tuning values that define the shared sparkly style and per-target variation limits.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In test play sessions, 100% of valid ball collisions with walls, paddle, and bricks produce a visible feedback effect.
- **SC-002**: At least 95% of surveyed test players report that collisions feel more responsive and satisfying with the feature enabled.
- **SC-003**: At least 90% of observed effects appear visually anchored to the perceived impact point.
- **SC-004**: During a 5-minute stress play session with frequent collisions, no persistent orphan effects remain visible after their intended display duration.
- **SC-005**: At least 90% of test players report that effects are noticeable without reducing their ability to track the ball.
- **SC-006**: In automated timing checks, 100% of sampled feedback effects despawn within 0.35 seconds and no earlier than 0.20 seconds after spawn.
- **SC-007**: In automated spawn-count checks, 100% of sampled qualifying collisions emit between 8 and 16 particles.
- **SC-008**: In pause-state tests, 0 collision feedback effects are spawned during pause, and 0 deferred effects are replayed on resume.
- **SC-009**: In collision-position checks, 100% of sampled effects spawn at the recorded collision contact point.
- **SC-010**: In brick-destruction tests, 100% of collisions that destroy a brick still produce one visual effect at the recorded contact point.

## Assumptions

- A short-lived sparkly burst is the default visual style for all supported collision targets.
- Existing collision detection already provides enough context to identify wall, paddle, and brick hits.
- This feature focuses only on visual impact feedback and does not change collision physics, scoring, damage rules, or audio behavior.
