# Feature Specification: Paddle Shrink Visual Feedback

**Feature Branch**: `008-paddle-shrink-feedback` **Created**: 2025-12-12 **Status**: Draft **Input**: User description: "When the ball is lost, the paddle should shrink to indicate that"

## Clarifications

### Session 2025-12-12

- Q: What is the primary reason for choosing 0.5 seconds as the shrink duration? → A: Match the existing fadeout timing
- Q: Should the paddle shrink animation run concurrently with the respawn delay or sequentially before it? → A: Concurrent - Shrink happens during the 1-second respawn delay (like fadeout)
- Q: Should the paddle remain visible throughout the shrink animation, or should it use a different visual treatment? → A: Remain visible

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Paddle Shrinks on Ball Loss (Priority: P1)

When a player loses the ball, the paddle provides immediate visual feedback by shrinking before the respawn sequence begins.
This gives players clear confirmation that their life was lost and helps them mentally prepare for the next attempt.

**Why this priority**: This is the core MVP feature that delivers the primary value - visual feedback for ball loss.
Without this, players rely only on life counter changes to understand they lost a ball.

**Independent Test**: Can be fully tested by playing the game, intentionally losing the ball, and observing the paddle shrink animation before the standard 1-second respawn delay and subsequent paddle regrowth.

**Acceptance Scenarios**:

1. **Given** the ball is in play and the paddle is at full size, **When** the ball collides with the lower goal boundary, **Then** the paddle immediately begins shrinking smoothly
2. **Given** the paddle has started shrinking due to ball loss, **When** the shrink animation completes during the respawn delay, **Then** the paddle reaches a small size (matching the respawn regrowth starting size)
3. **Given** the paddle is shrinking concurrently with the respawn delay, **When** the respawn delay completes, **Then** the paddle regrowth animation begins
4. **Given** the paddle is shrinking, **When** checking player controls, **Then** paddle input remains locked throughout the shrink animation

---

### User Story 2 - Shrink Animation Timing Integration (Priority: P2)

The shrink animation must integrate seamlessly with the existing respawn system timing, ensuring players experience smooth visual transitions without awkward pauses or jarring state changes.

**Why this priority**: Ensures the feature feels polished and doesn't disrupt the existing game flow that players are accustomed to.

**Independent Test**: Can be tested by measuring the total time from ball loss to full paddle regrowth completion, verifying the shrink duration is added appropriately and transitions are smooth.

**Acceptance Scenarios**:

1. **Given** a ball loss occurs, **When** measuring the time from ball collision to paddle reaching minimum size, **Then** the shrink animation completes within a defined duration
2. **Given** the paddle has shrunk to minimum size, **When** the respawn delay timer starts, **Then** the paddle remains at minimum size during the 1-second wait period
3. **Given** the respawn delay completes, **When** the paddle and ball respawn, **Then** the paddle grows from the same minimum size it shrunk to
4. **Given** multiple consecutive ball losses occur, **When** each loss triggers, **Then** each shrink animation plays fully regardless of rapid succession

---

### User Story 3 - Multiple Ball Scenarios (Priority: P3)

In multi-ball gameplay scenarios, only the paddle that loses its tracked ball should shrink, while other paddles (if any exist in future features) remain unaffected.

**Why this priority**: Ensures correctness for edge cases and future multi-ball power-ups, though current game has single paddle/ball.

**Independent Test**: Can be tested in multi-ball scenarios by losing one ball and verifying only the appropriate paddle responds.

**Acceptance Scenarios**:

1. **Given** a single paddle exists and the ball is lost, **When** the shrink animation triggers, **Then** that paddle shrinks
2. **Given** the game is in a state where ball loss is detected, **When** the paddle shrink begins, **Then** the shrink animation does not interfere with other game entities
3. **Given** a ball loss occurs during level transition, **When** the transition is in progress, **Then** the shrink animation is skipped or handled gracefully

---

### Edge Cases

- What happens when a ball is lost while the paddle is already in a growth animation (during level transition)? → Animation is interrupted, paddle immediately begins shrinking from current scale
- How does the system handle rapid consecutive ball losses (queued respawns)? → Each loss triggers a full shrink animation cycle before the queued respawn processes
- What happens to the paddle shrink if the player runs out of lives (game over)? → Shrink animation plays normally, but game over sequence takes over after shrink completes
- How does paddle shrink interact with the paddle growing component already used in respawn? → Reuse the existing `PaddleGrowing` component architecture but with reverse direction (shrink to minimum scale)
- What if collision detection sends multiple ball-loss events in quick succession? → The respawn queue system already handles this; shrink animation plays for the first event

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST trigger paddle shrink animation immediately when ball collides with lower goal boundary
- **FR-002**: Paddle MUST shrink smoothly from its current scale to a minimum scale over a defined duration
- **FR-003**: Minimum paddle scale MUST match the starting scale used in the respawn regrowth animation (0.01 per existing code)
- **FR-004**: Paddle shrink animation MUST run concurrently with the respawn delay timer (matching fadeout overlay behavior)
- **FR-005**: Paddle MUST reach and remain at minimum scale by the time the respawn delay completes
- **FR-006**: Paddle input MUST remain locked throughout the shrink animation (using existing `InputLocked` component)
- **FR-007**: Paddle MUST remain fully visible throughout the shrink animation (no fade, transparency, or despawn)
- **FR-008**: System MUST integrate with existing respawn system without breaking current respawn timing or behavior
- **FR-009**: Shrink animation MUST use smooth easing (matching the cubic easing used in paddle growth)
- **FR-010**: Shrink animation duration MUST match the existing respawn fadeout overlay timing (RespawnFadeOverlay duration)
- **FR-011**: System MUST handle interruption of existing paddle growth animation if ball loss occurs during level transition
- **FR-012**: System MUST work correctly with the existing respawn queue system for consecutive ball losses
- **FR-013**: Paddle shrink MUST only affect the paddle entity associated with the lost ball (future-proofing for multi-paddle scenarios)

### Key Entities

- **Paddle**: Player-controlled entity that deflects the ball; has position, scale, and growth/shrink state; affected by shrink animation on ball loss
- **Ball**: Game entity that triggers paddle shrink when lost; collision with lower goal initiates the visual feedback sequence
- **PaddleGrowing**: Existing component used for paddle growth animation; will be reused/adapted for shrink animation (inverse direction)
- **RespawnSchedule**: Existing resource tracking respawn timing; shrink animation must complete before respawn delay timer starts
- **LifeLostEvent**: Existing event emitted on ball loss; triggers the paddle shrink animation sequence
- **Lower Goal**: Boundary entity that detects ball loss; collision triggers the entire ball loss sequence including paddle shrink

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Paddle shrinks from full size to minimum size (0.01 scale) within the fadeout overlay duration (matches respawn delay timing) 100% of the time when ball is lost
- **SC-002**: Visual feedback is perceived by players as immediate (shrink begins within 1 frame of ball collision)
- **SC-003**: Total time from ball loss to gameplay resumption remains unchanged (shrink runs concurrently with respawn delay)
- **SC-004**: Shrink animation integrates seamlessly with existing respawn system in 100 consecutive ball losses without errors
- **SC-005**: Paddle scale transitions smoothly with no visual popping or jarring jumps during shrink-to-respawn-to-growth sequence
- **SC-006**: System handles 10 rapid consecutive ball losses (queued respawns) without animation glitches or timing errors
- **SC-007**: Paddle input remains locked from ball loss through shrink, respawn delay, and regrowth until ball is ready to launch
