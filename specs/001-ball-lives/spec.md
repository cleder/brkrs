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

### User Story 1 - See Remaining Lives (Priority: P1)

As a player, I want to see how many lives I have left so I can understand my remaining chances and adjust how carefully I play.

**Why this priority**: This is the core feedback loop for a limited-lives mechanic; without it, players can’t make informed decisions.

**Independent Test**: Start a new play session and verify the on-screen display shows the starting count and updates immediately after a life is lost.

**Acceptance Scenarios**:

1. **Given** a new play session has started, **When** gameplay becomes visible, **Then** the screen shows the remaining lives count as exactly 3.
2. **Given** the remaining lives count is 3, **When** one LifeLostEvent occurs, **Then** the remaining lives count becomes exactly 2 and the screen updates to show 2 within the same frame.
3. **Given** the remaining lives count is N (>0), **When** exactly one LifeLostEvent occurs, **Then** the remaining lives count becomes N-1 (one decrement per event; event-driven, not time- or frame-based).
4. **Given** gameplay continues across a level transition within the same session, **When** the next level becomes playable, **Then** the remaining lives count persists (no reset) and the HUD reflects the same value as before the transition.

5. **Given** gameplay is active, **When** the HUD is visible, **Then** the remaining lives count is displayed on-screen in a readable font and placement (Orbitron font is defined in the plan/tasks; styling specifics are out of scope here) [covers FR-004].

---

### User Story 2 - Game Over on Last Life (Priority: P2)

As a player, I want a clear "Game over" message when I run out of lives so I immediately understand the session is over.

**Why this priority**: Without an explicit game-over signal, players may be confused about whether the game is still active.

**Independent Test**: Trigger LifeLostEvent repeatedly until no lives remain, and verify the "Game over" message appears when the last life is lost.

**Acceptance Scenarios**:

1. **Given** the remaining lives count is 1, **When** a LifeLostEvent occurs, **Then** the remaining lives count becomes 0 and a "Game over" message is displayed.
2. **Given** the "Game over" message is displayed, **When** additional LifeLostEvent events occur (if any), **Then** the remaining lives count stays at 0 and the "Game over" message remains displayed.

3. **Given** the "Game over" message is displayed, **When** the player attempts to pause or perform gameplay input, **Then** pause/input is disabled and the modal "Game over" message remains above overlays [covers FR-008, FR-009].

4. **Given** the "Game over" message is displayed, **When** the player toggles cheat mode (presses 'g'), **Then** the system sets remaining lives to 3, removes the "Game over" overlay, and gameplay may resume.
   **Note:** The current level is not reloaded or reset by this action; the player resumes within the same level state.

5. **Given** the remaining lives count transitions from 1 to 0 due to a LifeLostEvent, **When** the same frame is processed, **Then** the HUD shows 0 and the "Game over" message appears within that frame [covers FR-005, FR-006].

---

### User Story 3 - Lives Never Go Negative (Priority: P3)

As a player, I want the remaining lives count to be consistent and never become negative so the HUD and game-over behavior stay trustworthy.

**Why this priority**: Prevents confusing UI states and ensures "Game over" is stable under repeated/duplicate life-loss signals.

**Independent Test**: Force more LifeLostEvent occurrences than the starting lives count and verify the count clamps at 0 and the game-over message behavior is stable.

**Acceptance Scenarios**:

1. **Given** the remaining lives count is 0, **When** a LifeLostEvent occurs, **Then** the remaining lives count stays at 0.

---

[Add more user stories as needed, each with an assigned priority]

#### Primary Flow

- Start a new play session.
- Gameplay becomes visible; HUD shows exactly 3 remaining lives.
- A `LifeLostEvent` occurs; remaining lives decrements to 2.
- In the same frame, HUD updates to show 2.
- Gameplay continues with lives persisting across level transitions within the session.

### Edge Cases

- Multiple LifeLostEvent occurrences arrive in the same moment (e.g., rapid duplicate triggers): the remaining lives count decrements once per distinct event (no coalescing by frame timing) and never goes below 0.
- LifeLostEvent occurs after the "Game over" message is already displayed: count remains at 0 and "Game over" remains visible.
- LifeLostEvent occurs while remaining lives is already 0: the count stays at 0; the "Game over" message remains unchanged (no re-trigger/re-animation required).
- Remaining lives display is visible during normal gameplay and accurately reflects the current remaining count.

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: System MUST initialize the player's remaining lives count to 3 at the start of a new play session.
- **FR-002**: When a LifeLostEvent occurs, the system MUST decrement the remaining lives count by exactly 1 (if current count > 0).
  Decrement is strictly event-driven: one decrement per LifeLostEvent, independent of frame rate or elapsed time.
- **FR-003**: The system MUST NOT allow the remaining lives count to go below 0.
- **FR-004**: The system MUST display the remaining lives count on-screen during gameplay.
- **FR-005**: The on-screen remaining lives count MUST update to reflect the new value synchronously within one frame (at 60 FPS target, ~16ms) after the count changes.
  When the remaining lives transitions from 1 to 0 due to a LifeLostEvent, the decrement to 0, the HUD update, and the appearance of the "Game over" message MUST occur within the same frame.
**FR-006**: When a LifeLostEvent occurs while the remaining lives count is 1 (resulting in 0), the system MUST display the exact message "Game over" (lowercase, centered on-screen).
"No lives left" is explicitly defined as `remaining lives count == 0` (game state), not a world-state query for physical ball entities.

The lives display remains visible during gameplay and while the game-over message is active.
**FR-007**: Once the "Game over" message is displayed, it MUST remain displayed while the remaining lives count is 0.
**FR-008**: When the "Game over" message is active, pause input MUST be disabled; the game-over message is modal and appears above any pause overlay.

- **FR-009**: Once the "Game over" message is displayed, gameplay is considered ended; no respawn shall occur, and player input (movement, actions) MUST be disabled until the player explicitly restarts or returns to the main menu.

### Assumptions

- A "new play session" is initiated when the player launches the game or clicks a "New Game" button; it persists across all level transitions within that session and resets only when the player explicitly exits or restarts.
- The remaining lives count represents how many additional life losses are allowed in the current play session.
- The remaining lives count persists across normal gameplay progression within a play session (e.g., across levels) and resets only when a new play session starts.
- “LifeLostEvent” refers to the game’s domain signal for “the player has just lost one ball/life.”
- Assumptions validated: persistence across levels, session reset conditions, and event semantics are acceptable product behavior for this feature.

### Non-Functional Constraints

- Font choice and styling (e.g., Orbitron) are defined in the implementation plan/tasks and treated as non-functional constraints.
  Functional acceptance does not depend on a specific font family, only on message text, placement, and visibility.

### Dependencies

- Depends on domain events: `LifeLostEvent` for decrement; game-over signaling/UI systems for message display.
  Requirements reference these at the behavioral level without prescribing implementation details.

### Key Entities *(include if feature involves data)*

- **Remaining Lives**: A non-negative integer representing how many lives the player has left in the current play session.
- **LifeLostEvent**: A domain event indicating one life was lost and the remaining lives count should be updated.
- **Game Over Message**: A visible on-screen message indicating the player has no lives remaining.

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: In 100% of tested sessions, players start with exactly 3 remaining lives.
- **SC-002**: In 100% of tested life-loss events, the remaining lives count decreases by exactly 1 until it reaches 0, and never becomes negative.
- **SC-003**: In 100% of tested cases where the last life is lost (transition from 1 to 0), the remaining lives count updates on-screen within one frame (~16ms), and the "Game over" message becomes visible within 1 second (allowing for animation/transition effects).
- **SC-004**: Usability test protocol: Recruit at least 5 participants familiar with action games.
  Test on native desktop build (Linux) with standard resolution (1920x1080).
  Prompt wording: "Without pausing, how many lives remain right now?"
  Measurement: Success if the participant verbally reports the correct number within 5 seconds of the prompt while gameplay is visible.
  Threshold: At least 80% of participants succeed.
