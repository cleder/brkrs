# Feature Specification: Multi-Hit Bricks

**Feature Branch**: `005-multi-hit-bricks`
**Created**: 2025-11-29
**Status**: Draft
**Input**: User description: "Create bricks that take several hits to destroy - Multi-Hit Bricks (Index 10-13) that need multiple hits before being destroyed"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Hitting a Multi-Hit Brick (Priority: P1)

As a player, when I hit a multi-hit brick with the ball, the brick should visually change to show it has been damaged and requires fewer hits to destroy.

**Why this priority**: This is the core mechanic of multi-hit bricks. Without visual feedback on damage, players cannot strategize or understand the brick's remaining durability.

**Independent Test**: Can be fully tested by launching a ball at a multi-hit brick and observing the visual transition. Delivers immediate gameplay feedback.

**Acceptance Scenarios**:

1. **Given** a brick with index 13 (needs 4 more hits) is on screen, **When** the ball collides with it, **Then** the brick transforms to index 12 (needs 3 more hits) with a distinct visual appearance
2. **Given** a brick with index 12 (needs 3 more hits) is on screen, **When** the ball collides with it, **Then** the brick transforms to index 11 (needs 2 more hits) with a distinct visual appearance
3. **Given** a brick with index 11 (needs 2 more hits) is on screen, **When** the ball collides with it, **Then** the brick transforms to index 10 (needs 1 more hit) with a distinct visual appearance
4. **Given** a brick with index 10 (needs 1 more hit) is on screen, **When** the ball collides with it, **Then** the brick transforms to index 20 (simple stone)

---

### User Story 2 - Destroying the Final Stage (Priority: P1)

As a player, when I hit a multi-hit brick that has been reduced to a simple stone (index 20), it should be destroyed and award me points.

**Why this priority**: This completes the multi-hit brick lifecycle and provides the satisfying payoff after multiple hits.

**Independent Test**: Can be tested by hitting a brick at index 10 to transform it to index 20, then hitting it again to destroy it.

**Acceptance Scenarios**:

1. **Given** a brick that was originally multi-hit and is now a simple stone (index 20), **When** the ball collides with it, **Then** the brick is destroyed
2. **Given** a multi-hit brick is destroyed (after becoming simple stone), **When** the destruction occurs, **Then** 50 points (base score) are awarded to the player per hit during the multi-hit phase, plus 25 points for the final simple stone destruction

---

### User Story 3 - Audio Feedback for Multi-Hit Bricks (Priority: P2)

As a player, I want to hear a distinct sound when hitting multi-hit bricks so I can distinguish them from regular bricks by audio cue alone.

**Why this priority**: Audio feedback enhances the gameplay experience but is not essential for core functionality.

**Independent Test**: Can be tested by hitting a multi-hit brick and verifying Sound 29 plays.

**Acceptance Scenarios**:

1. **Given** any multi-hit brick (index 10-13) is hit, **When** the collision occurs, **Then** Sound 29 plays
2. **Given** the multi-hit brick has transitioned to a simple stone, **When** it is hit, **Then** the standard simple brick sound plays

---

### User Story 4 - Level Completion with Multi-Hit Bricks (Priority: P2)

As a player, I want multi-hit bricks to count toward level completion only when fully destroyed, so the level doesn't end prematurely.

**Why this priority**: Ensures game progression logic works correctly with the new brick type.

**Independent Test**: Can be tested by creating a level with only multi-hit bricks and verifying the level completes only after all have been fully destroyed.

**Acceptance Scenarios**:

1. **Given** a level containing only multi-hit bricks, **When** some bricks are hit but not fully destroyed, **Then** the level remains incomplete
2. **Given** a level containing multi-hit bricks, **When** all multi-hit bricks have been reduced to simple stones and those stones are destroyed, **Then** the level is completed

---

### Edge Cases

- What happens when a multi-hit brick is hit in rapid succession before the transition animation completes? The hit should still register and transition should occur immediately.
- How does the system handle a multi-hit brick placed at index 10 that is hit once? It transitions to simple stone (index 20).
- What happens if a level only contains multi-hit bricks at various stages? All must be fully destroyed (reduced to simple stone and then destroyed) for level completion.
- How are multi-hit bricks saved/loaded in level files? Each brick's current state (index 10-13) is stored, not its original state.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support brick indices 10-13 representing multi-hit bricks with 1-4 remaining hits respectively
- **FR-002**: System MUST transform a multi-hit brick to the next lower index when hit (13→12→11→10→20)
- **FR-003**: System MUST display a distinct visual appearance for each multi-hit state (indices 10, 11, 12, 13)
- **FR-004**: System MUST play Sound 29 when any multi-hit brick (index 10-13) is hit
- **FR-005**: System MUST award 50 points for each hit on a multi-hit brick (before it becomes simple stone)
- **FR-006**: System MUST treat index 20 (simple stone) as the final destructible state for former multi-hit bricks
- **FR-007**: Multi-hit bricks MUST count as destructible bricks for level completion purposes
- **FR-008**: System MUST support loading multi-hit bricks from level files at any stage (10-13)
- **FR-009**: The ball MUST bounce off multi-hit bricks normally during collision

### Key Entities

- **Multi-Hit Brick**: A brick type that requires multiple hits to destroy. Characterized by a hit counter (1-4) that decrements on each collision. Transforms through visual states before becoming a simple stone.
- **Brick Index**: The numeric identifier (10-13) that determines the brick's appearance and remaining durability. Lower index = fewer hits needed.
- **Simple Stone (Index 20)**: The terminal state of a multi-hit brick before destruction. Behaves identically to a standalone simple stone.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Players can visually distinguish between all 4 multi-hit brick states at a glance
- **SC-002**: 100% of multi-hit brick collisions result in correct state transitions (13→12→11→10→20→destroyed)
- **SC-003**: Players hear the correct audio feedback (Sound 29) for multi-hit collisions within 50ms of impact
- **SC-004**: Levels containing multi-hit bricks complete only when all bricks are fully destroyed
- **SC-005**: Multi-hit bricks can be placed in level files and load correctly at any initial state (10-13)
- **SC-006**: Score awarded for multi-hit brick destruction matches the documented values (50 per hit phase + 25 for final destruction)

## Assumptions

- The existing brick collision system can be extended to support state transitions rather than immediate destruction
- Visual assets for indices 10-13 (Stonehit1.gif through Stonehit4.gif) are available or will be provided
- Audio file exists in the game's sound system
- The scoring system can handle incremental point awards during multi-hit phases
