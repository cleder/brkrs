# Feature Specification: Add Scoring System

**Feature Branch**: `009-add-scoring` **Created**: 16 December 2025 **Status**: Draft **Input**: User description: "add scoring.
Add new functionality that adds points to the players score. (score not implemented yet), the player starts with a score of 0 for a game.
When a brick gets destroyed, add the points to the current score.
Scores are documented in docs/bricks.md.
Every 5000 points the player gets an extra ball. scores get displayed"

## Clarifications

### Session 2025-12-16

- Q: Should the scoring system implement score multiplier bricks (26-29) in this feature? → A: Defer multiplier bricks to a future feature (Option B)
- Q: Does score persist between levels or reset at each level? → A: Score persists between levels - cumulative throughout the game session (Option A)
- Q: What should the range of random points be for the Question brick (index 53)? → A: Uniform random range 25-300 points (Option B)
- Q: When bricks have both points and special effects, do both always occur? → A: Points and effects always happen together when a brick is destroyed (Option A)
- Q: Should score changes be logged or tracked for observability? → A: No logging for MVP - display only (Option B)

## User Scenarios & Testing

### User Story 1 - Player Starts Game with Zero Score (Priority: P1)

A player starts a new game and immediately sees their score displayed on screen, beginning at 0 points.

**Why this priority**: This is the foundational requirement - the score display and initialization must work before any scoring can occur.
Without this, players have no feedback on their performance.

**Independent Test**: Can be fully tested by starting a new game and verifying the score UI element displays "0" and persists throughout the game session.

**Acceptance Scenarios**:

1. **Given** a player starts a new game, **When** the game initializes, **Then** the score is set to 0
2. **Given** the game is running, **When** the score UI is rendered, **Then** it displays "0" when no bricks have been destroyed yet
3. **Given** a player is mid-game, **When** they check the score display, **Then** it shows the current accumulated score

---

### User Story 2 - Points Awarded on Brick Destruction (Priority: P1)

When a player destroys a brick with the ball, points corresponding to that brick type are immediately added to their total score.

**Why this priority**: This is the core mechanic of the scoring system.
Without this, there's no way for players to earn points.

**Independent Test**: Can be fully tested by destroying a single brick of a known type (e.g., a Simple Stone brick worth 25 points) and verifying the score increases from 0 to 25.

**Acceptance Scenarios**:

1. **Given** a game with score at 0 and a Simple Stone brick (25 points) on the field, **When** the player destroys that brick, **Then** the score increases to 25
2. **Given** a game with multiple brick types on the field, **When** the player destroys a multi-hit brick (50 points), **Then** the score increases by exactly 50
3. **Given** a game already in progress with non-zero score, **When** the player destroys another brick, **Then** the new score equals previous score plus brick point value
4. **Given** a brick that triggers an effect (e.g., Apple/paddle shrink at 300 points), **When** destroyed, **Then** points are awarded and the effect occurs independently (both always happen)

---

### User Story 3 - Extra Ball Every 5000 Points (Priority: P1)

As the player accumulates 5000 or more points, they are awarded an additional ball in play.

**Why this priority**: This is a core progression mechanic that directly affects gameplay difficulty and player survival.
It's essential to the core loop.

**Independent Test**: Can be fully tested by accumulating exactly 5000 points (through brick destruction) and verifying a new ball spawns and enters play.

**Acceptance Scenarios**:

1. **Given** a player has earned 4999 points, **When** they destroy bricks worth at least 1 more point, **Then** the score reaches 5000 and a new ball appears in play
2. **Given** a player has 5000 points and continues destroying bricks, **When** they accumulate 10,000 points total, **Then** they receive a second additional ball (total of 3 balls: original + 2 bonus)
3. **Given** a player receives a new ball from the 5000-point milestone, **When** the ball spawns, **Then** it appears at a valid starting position and is immediately in play
4. **Given** the player loses all balls except one earned from a 5000-point milestone, **When** that ball is lost, **Then** the game follows normal ball loss logic

---

### User Story 4 - Score Display Visibility (Priority: P2)

The player can clearly see their current score at all times during gameplay, positioned prominently on the UI.

**Why this priority**: While essential for feedback, score visibility is secondary to actual scoring mechanics.
Players need to see the score, but the underlying scoring must work first.

**Independent Test**: Can be fully tested by starting a game, destroying any brick, and verifying the score is visible and updates properly on screen.

**Acceptance Scenarios**:

1. **Given** the game is running, **When** the score increases, **Then** the display updates immediately to show the new total
2. **Given** the score is 0, **When** displayed, **Then** it shows as "0" (or equivalent zero representation)
3. **Given** the score is 12345, **When** displayed, **Then** it shows the full number without truncation or overflow

---

### Edge Cases

- What happens when a player destroys the last brick with exact points to trigger a 5000-point milestone (both level completion and ball spawn)?
- What happens if the player destroys bricks rapidly in succession - is score accumulation instantaneous or queued?
- How does score behave when special brick effects spawn additional entities (e.g., Red 2 spawning balls, Rotor spawning enemies)?
  Points only accrue from destroyed bricks, not from spawned entities.

## Requirements

### Functional Requirements

- **FR-001**: System MUST initialize player score to 0 at the start of each game
- **FR-002**: System MUST track the current player score throughout the game session
- **FR-003**: When a brick is destroyed, the system MUST add the brick's point value (as defined in docs/bricks.md) to the player's score, independent of any special effects that brick triggers
- **FR-004**: Score multiplier bricks (indices 26-29) are explicitly out of scope for this feature and will be addressed in a future feature
- **FR-005**: System MUST detect when score reaches multiples of 5000 points and spawn an additional ball in play
- **FR-006**: System MUST display the current score on the game UI in a clearly visible location
- **FR-007**: System MUST update the score display in real-time when points are earned
- **FR-008**: Score values for each brick type MUST match the values documented in docs/bricks.md
- **FR-009**: System MUST support bricks with special score behavior: Question brick (index 53) awards a random score between 25-300 points when destroyed; Extra Ball brick (index 41) grants an additional ball instead of points

### Key Entities

- **Score**: Numeric value tracking total points accumulated in current game.
  Starts at 0, increases as bricks are destroyed.
  Can reach any positive integer value.
- **Brick**: Each brick type has an associated point value.
  When destroyed by ball impact, its points are added to the score.
- **Ball Lives**: Linked to score milestones.
  Every 5000 points grants one additional ball in play.
- **Score Multiplier**: A temporary game state that affects how many points are awarded (e.g., Times 2 multiplier doubles point values).
  Can be active or inactive.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Players can complete a level and see their total score increase from 0 to at least 1000 points through normal brick destruction
- **SC-002**: When a player accumulates exactly 5000 points, a new ball spawns and enters play within 1 second
- **SC-003**: The score display updates instantaneously (within 16ms, one frame at 60fps) when points are awarded
- **SC-004**: For bricks documented in docs/bricks.md, the awarded points match the "Score" column exactly
- **SC-005**: Players can see the score throughout an entire game session without visual obstruction or truncation
- **SC-006**: A player earning 10,000 points receives 2 additional balls (at 5000 and 10,000 point thresholds)
- **SC-007**: 100% of destructible bricks (indices 10-57) award their documented point values when destroyed

## Assumptions

- Score only resets at the start of a new game; it persists between levels in the same game session
- Score multiplier bricks (26-29) are out of scope for this initial implementation (can be addressed in future feature)
- Extra Ball brick (index 41) grants a ball directly, not through the 5000-point milestone system
- Question brick (index 53) awards a random score uniformly distributed between 25-300 points when destroyed
- Magnet bricks (55-56) have no score value (shown as "-" in docs)
- Level transition bricks (50, 54) award their points before advancing to the next level
- Score accumulation is synchronous (immediate) when bricks are destroyed
- Score change logging/observability is out of scope for MVP; can be added in future feature
