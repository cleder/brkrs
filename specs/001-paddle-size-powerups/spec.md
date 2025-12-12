# Feature Specification: Paddle Size Powerups

**Feature Branch**: `001-paddle-size-powerups` **Created**: 2025-12-12 **Status**: Draft **Input**: User description: "When the ball hits the brick, the paddle grows to 1.5 times of it's size (30) or shrinks to 0.7 times it's size: brick 30: Shrinks paddle (temporary), brick 32: Enlarges paddle" **Input**: User description: "$ARGUMENTS"

## Clarifications

### Session 2025-12-12

- Q: What type of visual feedback should be provided for paddle size changes? → A: Color change + subtle glow/outline effect
- Q: What happens to active size effects when advancing to the next level? → A: Clear all effects (paddle returns to normal size at level start)
- Q: How should overlapping size effects be handled when multiple brick types are hit? → A: Only one effect active at a time (new effect replaces old, timer resets)
- Q: What happens when hitting powerup bricks when already at min/max size limits? → A: Effect still applies, timer resets, but size clamped to limit
- Q: Should audio feedback be provided for paddle size changes? → A: Play distinct sound effect on brick hit (different for shrink vs enlarge)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Paddle Shrink on Brick 30 Hit (Priority: P1)

When a player's ball collides with a brick of type 30, the paddle immediately shrinks to 70% of its current size temporarily, increasing game difficulty and requiring more precise ball control.

**Why this priority**: This is the core negative powerup mechanic that adds strategic challenge to the game.
It's independently valuable as a difficulty modifier.

**Independent Test**: Can be fully tested by placing a brick 30 in a level, hitting it with the ball, and verifying the paddle shrinks to 70% of its size and then returns to normal size after the temporary effect expires.

**Acceptance Scenarios**:

1. **Given** the paddle is at normal size (20 units) and a brick 30 exists, **When** the ball hits brick 30, **Then** the paddle shrinks to 14 units (0.7 × 20)
2. **Given** the paddle has already shrunk from hitting brick 30, **When** the temporary shrink effect duration expires, **Then** the paddle returns to its previous size
3. **Given** the paddle is enlarged from brick 32 (30 units), **When** the ball hits brick 30, **Then** the enlarge effect is replaced and paddle immediately becomes 14 units (70% of normal 20 units), not 70% of 30
4. **Given** the paddle is shrunk from brick 30, **When** the ball hits brick 30 again before expiry, **Then** the effect timer resets to 10 seconds but paddle size remains at 14 units

---

### User Story 2 - Paddle Enlarge on Brick 32 Hit (Priority: P1)

When a player's ball collides with a brick of type 32, the paddle immediately enlarges to 150% of its current size, making it easier to control the ball and continue gameplay.

**Why this priority**: This is the core positive powerup mechanic that provides strategic advantage and player reward.
It's independently valuable as a help mechanic.

**Independent Test**: Can be fully tested by placing a brick 32 in a level, hitting it with the ball, and verifying the paddle enlarges to 150% of its size.

**Acceptance Scenarios**:

1. **Given** the paddle is at normal size (20 units) and a brick 32 exists, **When** the ball hits brick 32, **Then** the paddle enlarges to 30 units (1.5 × 20)
2. **Given** the paddle has enlarged from brick 32, **When** the enlarge effect duration expires, **Then** the paddle returns to its previous size
3. **Given** the paddle is shrunk from brick 30 (14 units), **When** the ball hits brick 32, **Then** the shrink effect is replaced and paddle immediately becomes 30 units (150% of normal 20 units), not 150% of 14
4. **Given** the paddle is enlarged from brick 32, **When** the ball hits brick 32 again before expiry, **Then** the effect timer resets to 10 seconds but paddle size remains at 30 units

---

### User Story 3 - Visual Feedback for Size Changes (Priority: P2)

Players receive clear visual feedback when the paddle size changes, making it obvious when a powerup or penalty is active.

**Why this priority**: Visual feedback ensures players understand the game state and can react appropriately.
This enhances user experience but the core mechanic works without it.

**Independent Test**: Can be fully tested by triggering paddle size changes and verifying that visual indicators (color change, particle effects, or animation) are displayed during the transition and while the effect is active.

**Acceptance Scenarios**:

1. **Given** the paddle is at normal size, **When** brick 30 is hit causing shrinkage, **Then** the paddle displays a red color tint with subtle glow/outline and plays a shrink sound effect
2. **Given** the paddle is at normal size, **When** brick 32 is hit causing enlargement, **Then** the paddle displays a green color tint with subtle glow/outline and plays an enlarge sound effect
3. **Given** a size effect is active, **When** the effect expires, **Then** the color tint and glow/outline disappear and the paddle transitions smoothly to normal appearance

---

### Edge Cases

- What happens when the paddle hits brick 30 while already at minimum playable size? (Resolved: Effect activates, timer resets, size stays at 10 units)
- What happens when the paddle hits brick 32 while already at maximum playable size? (Resolved: Effect activates, timer resets, size stays at 30 units)
- How does the system handle rapid alternating hits between brick 30 and brick 32? (Resolved: Each new hit replaces previous effect)
- What happens to active size effects when the player loses a life? (Resolved: All effects cleared)
- What happens to active size effects when advancing to the next level? (Resolved: All effects cleared, paddle returns to normal)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST detect collision between ball and brick type 30
- **FR-002**: System MUST detect collision between ball and brick type 32
- **FR-003**: System MUST change paddle width to 70% of its current size when brick 30 is hit
- **FR-004**: System MUST change paddle width to 150% of its current size when brick 32 is hit
- **FR-005**: System MUST apply size changes immediately upon brick collision
- **FR-006**: System MUST restore paddle to its previous size after a temporary effect duration of 10 seconds
- **FR-007**: System MUST replace any active size effect with the new effect when a different brick type (30 or 32) is hit, resetting the timer to 10 seconds
- **FR-008**: System MUST reset the effect timer to 10 seconds when the same brick type is hit again before the current effect expires, maintaining the same size
- **FR-009**: System MUST enforce minimum paddle width of 10 units (50% of normal paddle size); when shrink effect applied at minimum, size remains clamped but effect and timer activate
- **FR-010**: System MUST enforce maximum paddle width of 30 units (150% of normal paddle size); when enlarge effect applied at maximum, size remains clamped but effect and timer activate
- **FR-011**: System MUST clear all active size effects when player loses a life
- **FR-011b**: System MUST clear all active size effects and restore paddle to normal size when advancing to next level
- **FR-012**: System MUST provide visual feedback during paddle size transitions using color change and subtle glow/outline effect
- **FR-013**: System MUST display color change (red tint for shrink, green tint for enlarge) and subtle glow/outline while size effect is active
- **FR-014**: System MUST play a distinct audio sound effect when brick 30 is hit (shrink sound)
- **FR-015**: System MUST play a distinct audio sound effect when brick 32 is hit (enlarge sound, different from shrink)
- **FR-016**: System MUST remove brick 30 or brick 32 from the level after being hit (consistent with existing brick behavior)

### Key Entities

- **Paddle**: The player-controlled horizontal bar.
  Key attributes include current width, base width, active size effects, and visual state
- **Brick Type 30**: A special brick that triggers paddle shrinkage when destroyed.
  Attributes include position, collision detection, and effect trigger
- **Brick Type 32**: A special brick that triggers paddle enlargement when destroyed.
  Attributes include position, collision detection, and effect trigger
- **Size Effect**: A temporary modifier on paddle size.
  Attributes include effect type (shrink/enlarge), duration remaining, multiplier value (0.7 or 1.5), and visual state

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Paddle size changes are visually apparent within 100 milliseconds of brick collision
- **SC-002**: Players can identify whether they hit brick 30 or brick 32 based on visual and audio feedback alone
- **SC-003**: Paddle remains playable and responsive at both minimum (70% shrunk) and maximum (150% enlarged) sizes
- **SC-004**: Size effects work consistently across all game levels without performance degradation
- **SC-005**: Players successfully complete levels that include brick 30 and brick 32 without confusion about paddle behavior
- **SC-006**: Zero crashes or game-breaking bugs related to paddle size changes during 100 consecutive level plays
