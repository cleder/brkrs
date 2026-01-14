# Feature Specification: Brick Types 42 & 91 — Paddle Life Loss

**Feature Branch**: `023-brick-42-91-life-loss` **Created**: 2026-01-13 **Status**: Draft **Input**: User description: "brick types 42 and 91.
When the paddle hits the brick, the life is lost.
Brick type 42 is destroyed when the ball hits the brick, scores 90 points Brick type 91 is indestructible"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Ball destroys brick 42 and awards points (Priority: P1)

Destroying a type 42 brick with the ball should remove it from play and award 90 points.

**Why this priority**: Establishes core scoring behavior for brick 42 and validates destruction flow.

**Independent Test**: Spawn a type 42 brick and a ball; simulate ball-brick collision; verify brick is removed and score increases by 90.

**Acceptance Scenarios**:

- Given a level containing a type 42 brick, When the ball collides with that brick, Then the brick is removed from play and the player's score increases by 90 points.
- Given multiple type 42 bricks, When the ball destroys one brick, Then only 90 points are awarded for that specific brick instance.

---

### User Story 2 - Paddle collision with hazard/solid bricks reduces a life (Priority: P1)

When the player's paddle collides with certain bricks, the player loses one life.

**Why this priority**: Ensures the life-loss penalty is enforced for paddle collisions and interacts correctly with the existing lives and respawn flow.

**Independent Test**: Simulate paddle contact with a type 42 brick and with a type 91 brick; verify lives decrement by one in each case and standard life-loss handling initiates.

**Acceptance Scenarios**:

- Given the player has 3 lives, When the paddle collides with a type 42 brick, Then the player's remaining lives decrease to 2 and the standard life-loss flow begins.
- Given the player has 3 lives, When the paddle collides with a type 91 brick, Then the player's remaining lives decrease to 2 and the standard life-loss flow begins.
- Given multiple simultaneous paddle collisions with bricks in the same frame, When life loss is processed, Then [NEEDS CLARIFICATION: should only one life be lost per frame regardless of number of contacts, or one per unique hazardous contact?]

---

### User Story 3 - Brick 91 is indestructible and does not affect scoring (Priority: P2)

Type 91 bricks are indestructible: they remain in play when hit by the ball and do not award points on collision.

**Why this priority**: Clarifies interaction rules to avoid confusion and enforces correct completion criteria.

**Independent Test**: Spawn a type 91 brick and a ball; simulate ball-brick collision; verify the brick remains, score does not change, and the level can still complete once destructible bricks are cleared.

**Acceptance Scenarios**:

- Given a level with at least one type 91 brick and at least one destructible brick, When all destructible bricks are destroyed, Then the level completes even if type 91 bricks remain.
- Given a type 91 brick, When the ball collides with it, Then the brick remains and no points are awarded.
- Given a type 91 brick, When the paddle collides with it, Then the player loses one life (per User Story 2) and the brick remains.

---

### Edge Cases

- If the paddle starts a level overlapping a type 42 or type 91 brick, the life-loss rule applies immediately on first update.
- If multiple balls exist, destroying a single type 42 brick awards 90 points exactly once (no double-awards).
- If multiple paddle contacts occur in rapid succession, life loss should follow the clarified rule in User Story 2.
- Levels containing only indestructible bricks (type 91) should be considered complete or skippable once no destructible bricks remain present.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Ball collisions with type 42 bricks MUST destroy the brick and award exactly 90 points.
- **FR-002**: Paddle collisions with type 42 bricks MUST cause the player to lose one life.
- **FR-003**: Paddle collisions with type 91 bricks MUST cause the player to lose one life.
- **FR-004**: Type 91 bricks MUST be indestructible; ball collisions MUST NOT remove them from play.
- **FR-005**: Type 91 bricks MUST award 0 points on ball collision.
- **FR-006**: Type 42 bricks MUST contribute to level completion; removing all destructible bricks MUST mark the level complete.
- **FR-007**: Type 91 bricks MUST NOT count toward level completion; the presence of only type 91 bricks MUST NOT block level completion.
- **FR-008**: Life loss MUST integrate with the existing lives flow such that losing the last life triggers the standard game-over request.
- **FR-009**: When multiple paddle-brick contacts occur concurrently in the same frame, the player MUST lose at most one life (single life lost per frame).

### Key Entities

- **Brick (Type 42)**: Destructible object; ball collision removes it; awards 90 points; contributes to level completion.
- **Brick (Type 91)**: Indestructible object; ball collision does not remove it; awards 0 points; does not contribute to level completion; paddle collision causes life loss.
- **Lives Counter**: Tracks remaining lives; decremented on life-loss events; reaching 0 triggers game-over.
- **Score**: Tracks cumulative points awarded from destructible brick destruction; increases by 90 for each type 42 destroyed.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Destroying a type 42 brick increases the player's score by 90 within one interaction cycle and the brick is removed from play.
- **SC-002**: Paddle contact with type 42 or type 91 reduces remaining lives by 1 and initiates the standard loss/respawn flow; 95% of test runs reflect the correct decrement under concurrent contacts as per clarified policy.
- **SC-003**: Type 91 bricks remain after ball collisions and never change the score; levels complete when all destructible bricks are cleared even if type 91 bricks remain.
- **SC-004**: In a controlled test of 10 consecutive frames post-interaction, changes to score and lives persist accurately without regression or overwrite.

## Assumptions

- Paddle contact life loss applies to both type 42 and type 91 bricks.
- Type 42 bricks are part of the destructible set and count toward completion.
- Type 91 bricks are part of the indestructible set and do not count toward completion.
- Life-loss effects (respawn timing, animations, overlays) are handled by the existing game flow once a loss is triggered.
- Score milestones behave consistently when adding 90-point increments from type 42 destruction.

## Clarifications

### Session 2026-01-13

- Q: For multiple concurrent paddle-brick contacts, should the system lose only one life per frame, or one life per unique hazardous brick contact? → A: One life lost per frame
