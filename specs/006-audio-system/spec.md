# Feature Specification: Audio System

**Feature Branch**: `006-audio-system` **Created**: 2025-11-29 **Status**: Draft **Input**: User description: "Audio event hooks for brick interactions, level transitions, and multi-hit brick feedback.
Addresses GitHub issues #10 and #23."
**Related Issues**: [#10](https://github.com/cleder/brkrs/issues/10), [#23](https://github.com/cleder/brkrs/issues/23)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Brick Hit Audio Feedback (Priority: P1)

As a player, I want to hear sounds when bricks are hit or destroyed so that I receive immediate audio feedback confirming my actions.

**Why this priority**: Audio feedback for brick collisions is the most fundamental audio interaction in a brick-breaker game.
It provides essential gameplay feedback and is triggered most frequently during play.

**Independent Test**: Can be fully tested by launching a ball at any brick and listening for the appropriate sound.
Delivers immediate confirmation that audio is working.

**Acceptance Scenarios**:

1. **Given** a simple brick (index 20) is on screen, **When** the ball collides with and destroys it, **Then** a brick destruction sound plays
2. **Given** a multi-hit brick (index 10-13) is on screen, **When** the ball collides with it, **Then** the multi-hit impact sound plays
3. **Given** a multi-hit brick that has transitioned to simple stone (index 20), **When** the ball destroys it, **Then** the standard brick destruction sound plays (not multi-hit impact sound)

---

### User Story 2 - Ball Bounce Audio (Priority: P1)

As a player, I want to hear sounds when the ball bounces off walls or the paddle so that I can track ball movement even when not looking directly at it.

**Why this priority**: Wall and paddle bounces are frequent events that provide crucial spatial audio cues, helping players anticipate ball position.

**Independent Test**: Can be tested by launching the ball and listening for wall bounce sounds when it hits the side, top, or paddle boundaries.

**Acceptance Scenarios**:

1. **Given** the ball is in play, **When** it collides with any wall boundary, **Then** a wall bounce sound plays
2. **Given** the ball is in play, **When** it bounces off the paddle, **Then** a paddle hit sound plays (distinct from wall bounce)

---

### User Story 3 - Paddle Collision Audio (Priority: P1)

As a player, I want to hear sounds when I bump the paddle into walls or bricks so that I receive feedback about my paddle movements and collisions.

**Why this priority**: Paddle collisions provide important tactile feedback that helps players understand the play area boundaries and adds to the arcade feel.

**Independent Test**: Can be tested by moving the paddle into a wall boundary or brick and listening for collision sounds.

**Acceptance Scenarios**:

1. **Given** the paddle is in play, **When** I move it into a wall boundary, **Then** a paddle-wall collision sound plays
2. **Given** the paddle is in play and bricks are nearby, **When** I move it into a brick, **Then** a paddle-brick collision sound plays

---

### User Story 4 - Level Transition Audio (Priority: P2)

As a player, I want to hear audio cues when levels start and complete so that transitions feel polished and clearly signal game state changes.

**Why this priority**: Level transitions are less frequent than gameplay sounds but provide important milestone feedback that enhances game feel.

**Independent Test**: Can be tested by completing a level and listening for transition sounds, or starting a new game and hearing level start audio.

**Acceptance Scenarios**:

1. **Given** a level has just been loaded, **When** gameplay begins, **Then** a level start sound plays
2. **Given** the last destructible brick is destroyed, **When** the level is completed, **Then** a level completion sound plays
3. **Given** paddle growth animation completes (after level transition), **When** the animation finishes, **Then** an optional paddle ready sound may play

---

### User Story 5 - Audio Configuration (Priority: P2)

As a player, I want to be able to adjust audio settings so that I can customize my gameplay experience or play silently.

**Why this priority**: Accessibility and user preference control are important for a quality experience, though not essential for core gameplay.

**Independent Test**: Can be tested by adjusting volume settings and verifying sounds respond to the new levels.

**Acceptance Scenarios**:

1. **Given** the audio settings menu, **When** I adjust the master volume, **Then** all game sounds reflect the new volume level
2. **Given** the audio settings menu, **When** I mute audio, **Then** no sounds play during gameplay
3. **Given** audio is muted, **When** I unmute, **Then** sounds resume at the previously set volume

---

### User Story 6 - Graceful Degradation Without Assets (Priority: P3)

As a developer or player on a minimal setup, I want the game to run without errors if audio assets are missing so that development and testing are not blocked.

**Why this priority**: Important for development workflow and headless testing, but not a user-facing feature.

**Independent Test**: Can be tested by removing audio assets and verifying the game runs without crashes or console errors.

**Acceptance Scenarios**:

1. **Given** audio assets are missing or not loaded, **When** an audio-triggering event occurs, **Then** the system logs a warning but does not crash
2. **Given** the game is running in headless mode (no audio device), **When** audio events are triggered, **Then** the system operates as a no-op without errors

---

### Edge Cases

- What happens when multiple bricks are destroyed in rapid succession (e.g., chain reaction)?
  System limits concurrent sounds to 3-4 of the same type; excess sounds are dropped.
- What happens when the ball hits a corner between two bricks simultaneously?
  A single sound should play (not double).
- How does the system handle audio on web builds where audio context requires user interaction?
  The system should gracefully handle browser audio restrictions.
- What happens if volume is set to 0%?
  Audio events should still fire internally (for future analytics) but produce no audible output.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST play the multi-hit impact sound when any multi-hit brick (index 10-13) is hit by the ball
- **FR-002**: System MUST play a brick destruction sound when any destructible brick is destroyed
- **FR-003**: System MUST play distinct sounds for different collision types (wall bounce, paddle hit, brick hit)
- **FR-012**: System MUST play a paddle-wall collision sound when the paddle collides with a wall boundary
- **FR-013**: System MUST play a paddle-brick collision sound when the paddle collides with a brick
- **FR-004**: System MUST trigger audio playback within 50ms of the collision event
- **FR-005**: System MUST map specific game events to specific audio assets (event-to-sound mapping)
- **FR-006**: System MUST provide volume control (0-100%) as a configurable resource
- **FR-007**: System MUST support mute/unmute toggle functionality
- **FR-008**: System MUST gracefully degrade (log warning, no crash) when audio assets are unavailable
- **FR-009**: System MUST support level transition sounds (level start, level complete)
- **FR-010**: System MUST limit concurrent playback to a maximum of 4 sounds of the same type; excess sounds are dropped
- **FR-011**: Volume and mute settings MUST persist across game sessions

### Key Entities

- **Audio Event**: A game event (collision, transition, action) that should trigger audio playback.
  Examples: BrickHit, WallHit, PaddleWallHit, PaddleBrickHit, LevelComplete.
- **Sound Asset**: An audio file mapped to one or more audio events.
  Identified by a sound ID or name.
- **Audio Configuration**: User-adjustable settings including master volume (0-100%) and mute state.
- **Sound Mapping**: The relationship between game events and their corresponding sound assets.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of brick collisions produce audio feedback within 50ms of impact
- **SC-002**: Multi-hit brick hits (index 10-13) consistently play the multi-hit impact sound, distinguishable from other brick sounds
- **SC-003**: Players can identify ball position by audio alone when not visually tracking the ball
- **SC-004**: Game runs without errors or crashes in headless tests (audio graceful degradation)
- **SC-005**: Volume changes take effect immediately without requiring game restart
- **SC-006**: Level transitions are audibly distinct from regular gameplay, signaling state changes clearly

## Clarifications

### Session 2025-11-29

- Q: How should concurrent sounds of the same type be handled? → A: Limit concurrent sounds (max 3-4 of same type, drop excess)
- Q: Are gravity bricks (index 21-25) in scope? → A: No, gravity brick audio is out of scope

## Out of Scope

- Special audio for gravity bricks (index 21-25) - these use standard brick destruction sound if implemented
- Brick-type-specific sounds beyond multi-hit bricks (all other bricks use generic destruction sound)

## Assumptions

- A sound asset library exists or will be provided with sounds for: brick destruction, wall bounce, paddle hit, paddle-wall collision, paddle-brick collision, level start, level complete, and multi-hit impact
- The existing `MultiHitBrickHit` event infrastructure in `src/systems/multi_hit.rs` will be used to trigger the multi-hit impact sound
- Standard audio formats (OGG, WAV, MP3) are supported by the game engine
- Web builds may require user interaction before audio plays (browser autoplay restrictions)
- Volume and mute settings will be stored in a persistent configuration file or local storage
