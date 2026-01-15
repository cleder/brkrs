**Feature Specification: Audio Wall Delay Fix**

**Feature Branch**: `016-audio-wall-delay-fix` **Created**: 2025-12-28 **Status**: Draft **Input**: User description: "when the ball hits the wall, the audio feedback has a delay, it should be played immediately"

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, UI updates, or hierarchy, the implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include at least one check that guards against prohibited patterns (e.g., panicking queries or per-frame UI updates without `Changed<T>`).
Include checks for **Message-Event Separation** (correct use of `MessageWriter` vs observers/ `Trigger<T>`) and **Hierarchy Safety** (use of `commands.entity(parent).add_child(child)` or `EntityCommands::set_parent`).

### User Story 1 - Immediate Wall Hit Audio (Priority: P1)

As a player, when the ball hits the wall, I want to hear the audio feedback immediately so that the game feels responsive and satisfying.

**Why this priority**: Immediate audio feedback is critical for game feel and user satisfaction.
Delayed sounds reduce perceived responsiveness and can frustrate players.

**Independent Test**: Can be fully tested by simulating a ball-wall collision and measuring the time between collision and audio playback.
The test passes if the audio is played within an imperceptible delay (e.g., <50ms).

**Acceptance Scenarios**:

1. **Given** the ball is moving and collides with a wall, **When** the collision occurs, **Then** the wall hit audio is played immediately (within 50ms of collision event).
2. **Given** the ball collides with multiple walls in quick succession, **When** each collision occurs, **Then** the audio feedback is played for each collision without perceptible delay.

---

### Edge Cases

- Q: What should happen if multiple wall collisions occur in the same frame?
A: Play audio for each collision, subject to the concurrency limit (if reached, log and skip further sounds for that frame).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST play wall hit audio immediately upon ball-wall collision event.
- **FR-002**: System MUST ensure that audio feedback is not delayed by unrelated game logic or frame timing.
- **FR-003**: System MUST play audio for each wall collision, even if collisions occur in rapid succession.
- **FR-004**: System MUST not introduce audio artifacts or overlapping issues when multiple collisions occur close together.
- **FR-005**: Only wall collision audio is in scope; paddle/brick/other collision audio is out of scope for this feature.

## Clarifications

### Session 2025-12-28

- Q: Is only wall collision audio in scope, or should paddle/brick/other collision audio also be included?
    → A: Only wall collision audio is in scope; paddle/brick/other collision audio is out of scope for this feature.

### Key Entities

### Session 2025-12-28

    → A: Use a dedicated `BallWallHit` message/event with explicit fields for ball and wall entities.

## Success Criteria *(mandatory)*

### Measurable Outcomes

**SC-001**: Wall hit audio is played within 50ms of collision in 99% of cases (measured in test runs).
**SC-002**: No perceptible delay in audio feedback reported by playtesters.
**SC-003**: Audio feedback is played for every wall collision, even during rapid sequences.
**SC-004**: No new audio artifacts or bugs are introduced as a result of this change.

- **SC-001**: Wall hit audio is played within 50ms of collision in 99% of cases (measured in test runs).
