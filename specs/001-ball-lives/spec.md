# Feature Specification: Ball Lives Counter

**Feature Branch**: `001-ball-lives` **Created**: 2025-12-14 **Status**: Draft **Input**: User description: "Count and limit the balls a player has.
A player starts with three balls.
When a LifeLostEvent occurs the number of balls gets decremented by one.
When a LifeLostEvent occurs, and no balls are left, display a Game over message.
The number of balls gets displayed on the screen."

## User Scenarios & Testing *(mandatory)*

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

### User Story 1 - See Remaining Balls (Priority: P1)

As a player, I want to see how many balls I have left so I can understand my remaining chances and adjust how carefully I play.

**Why this priority**: This is the core feedback loop for a limited-lives mechanic; without it, players can’t make informed decisions.

**Independent Test**: Start a new play session and verify the on-screen display shows the starting count and updates immediately after a life is lost.

**Acceptance Scenarios**:

1. **Given** a new play session has started, **When** gameplay becomes visible, **Then** the screen shows the remaining balls count as 3.
2. **Given** the remaining balls count is 3, **When** one LifeLostEvent occurs, **Then** the remaining balls count becomes 2 and the screen updates to show 2.

---

### User Story 2 - Game Over on Last Ball (Priority: P2)

As a player, I want a clear “Game over” message when I run out of balls so I immediately understand the session is over.

**Why this priority**: Without an explicit game-over signal, players may be confused about whether the game is still active.

**Independent Test**: Trigger LifeLostEvent repeatedly until no balls remain, and verify the “Game over” message appears when the last ball is lost.

**Acceptance Scenarios**:

1. **Given** the remaining balls count is 1, **When** a LifeLostEvent occurs, **Then** the remaining balls count becomes 0 and a “Game over” message is displayed.
2. **Given** the “Game over” message is displayed, **When** additional LifeLostEvent events occur (if any), **Then** the remaining balls count stays at 0 and the “Game over” message remains displayed.

---

### User Story 3 - Lives Never Go Negative (Priority: P3)

As a player, I want the remaining balls count to be consistent and never become negative so the HUD and game-over behavior stay trustworthy.

**Why this priority**: Prevents confusing UI states and ensures “Game over” is stable under repeated/duplicate life-loss signals.

**Independent Test**: Force more LifeLostEvent occurrences than the starting ball count and verify the count clamps at 0 and the game-over message behavior is stable.

**Acceptance Scenarios**:

1. **Given** the remaining balls count is 0, **When** a LifeLostEvent occurs, **Then** the remaining balls count stays at 0.

---

[Add more user stories as needed, each with an assigned priority]

### Edge Cases

- Multiple LifeLostEvent occurrences arrive in the same moment (e.g., rapid duplicate triggers): the remaining balls count decrements at most once per event and never goes below 0.
- LifeLostEvent occurs after the “Game over” message is already displayed: count remains at 0 and “Game over” remains visible.
- Remaining balls display is visible during normal gameplay and accurately reflects the current remaining count.

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: System MUST initialize the player’s remaining balls count to 3 at the start of a new play session.
- **FR-002**: When a LifeLostEvent occurs, the system MUST decrement the remaining balls count by exactly 1 if the current count is greater than 0.
- **FR-003**: The system MUST NOT allow the remaining balls count to go below 0.
- **FR-004**: The system MUST display the remaining balls count on-screen during gameplay.
- **FR-005**: The on-screen remaining balls count MUST update to reflect the new value immediately after it changes.
- **FR-006**: When a LifeLostEvent occurs while the remaining balls count is 1 (resulting in 0), the system MUST display a “Game over” message.
- **FR-007**: Once the “Game over” message is displayed, it MUST remain displayed while the remaining balls count is 0.

### Assumptions

- The remaining balls count represents how many additional life losses are allowed in the current play session.
- The remaining balls count persists across normal gameplay progression within a play session (e.g., across levels) and resets only when a new play session starts.
- “LifeLostEvent” refers to the game’s domain signal for “the player has just lost one ball/life.”

### Key Entities *(include if feature involves data)*

- **Remaining Balls**: A non-negative integer representing how many balls/lives the player has left in the current play session.
- **LifeLostEvent**: A domain event indicating one ball/life was lost and the remaining balls count should be updated.
- **Game Over Message**: A visible on-screen message indicating the player has no balls remaining.

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: In 100% of tested sessions, players start with exactly 3 remaining balls.
- **SC-002**: In 100% of tested life-loss events, the remaining balls count decreases by exactly 1 until it reaches 0, and never becomes negative.
- **SC-003**: In 100% of tested cases where the last ball is lost (transition from 1 to 0), a “Game over” message becomes visible within 1 second.
- **SC-004**: In a usability check with at least 5 participants, at least 80% can correctly report how many balls remain within 5 seconds of being asked during gameplay.
