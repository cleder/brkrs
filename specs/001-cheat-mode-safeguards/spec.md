# Feature Specification: Cheat Mode Safeguards

**Feature Branch**: `001-cheat-mode-safeguards` **Created**: 2025-12-17 **Status**: Draft **Input**: User description: "Prevent that the Keys to respawn a level, or cycle through levels, are accidentally pressed.
Add an indicator that cheat mode is active.
It should be displayed in the lower right or left corner of the screen.
When entering cheat mode, the score #34 gets reduced to 0.
The cheatmode is activated by pressing 'g'"

## Clarifications

### Session 2025-12-17

- Q: Which specific keyboard keys are the "level control keys" that should be disabled during normal gameplay? → A: Letter keys - R=respawn, N=next level, P=previous level (Note: P currently invokes texture picker UI which will be removed)
- Q: What visual style should the cheat mode indicator use? → A: Simple white text "CHEAT MODE" on semi-transparent dark background
- Q: Should the cheat mode indicator position be fixed or user-configurable? → A: Fixed to lower right corner
- Q: What audio feedback should play when level control keys are pressed without cheat mode active? → A: Short soft beep
- Q: When pressing 'g' during pause/transition, how should toggle behave? → A: Ignore 'g' during pause/transition (no toggle)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Cheat Mode Activation with Score Reset (Priority: P1)

A player wants to enable cheat mode to test or explore game levels freely.
They press the 'g' key during gameplay, which activates cheat mode.
Upon activation, their current score resets to 0, and a clear visual indicator appears on screen to confirm cheat mode is active.

**Why this priority**: This is the core functionality that enables the feature and provides immediate feedback to the player.
Without this, the feature doesn't exist.

**Independent Test**: Can be fully tested by pressing 'g' during gameplay, verifying the score resets to 0, and confirming a visual indicator appears on screen.

**Acceptance Scenarios**:

1. **Given** player is in an active game level with a non-zero score, **When** player presses 'g' key, **Then** cheat mode activates, score resets to 0, and indicator appears in corner of screen
2. **Given** player has already activated cheat mode, **When** player presses 'g' again, **Then** cheat mode deactivates, score resets to 0, and indicator disappears from screen
3. **Given** player activates cheat mode, **When** gameplay continues, **Then** cheat mode indicator remains visible throughout the session
4. **Given** the player has no remaining lives and a "Game over" overlay is displayed, **When** the player presses 'g' to toggle cheat mode, **Then** cheat mode activates, the remaining lives are set to 3, any active game-over overlay is removed, and the player may resume gameplay. **Note:** toggling cheat mode does NOT reload or reset the current level; gameplay resumes in-place with the level state unchanged.

---

### User Story 2 - Accidental Key Press Prevention (Priority: P2)

A player is playing normally and accidentally presses level control keys (respawn level or cycle through levels).
Since cheat mode is not active, the system ignores the key press and no action is executed.
Level control keys only become functional after the player deliberately activates cheat mode by pressing 'g'.

**Why this priority**: Prevents frustration from accidental inputs during intense gameplay by completely disabling these keys during normal play, but the game is still playable without it.

**Independent Test**: Can be tested by pressing level control keys during normal gameplay and verifying no action occurs, then activating cheat mode and verifying the keys work as expected.

**Acceptance Scenarios**:

1. **Given** player is in normal gameplay without cheat mode active, **When** player presses any level control key, **Then** no action is executed and gameplay continues uninterrupted
2. **Given** player has activated cheat mode, **When** player presses a level control key, **Then** the corresponding level control action executes as expected (respawn or cycle levels)
3. **Given** player is in intense gameplay without cheat mode, **When** player accidentally presses a level control key multiple times, **Then** the game ignores all inputs and does not change level state

---

### User Story 3 - Cheat Mode Visual Indicator (Priority: P1)

A player who has activated cheat mode needs to clearly see that cheat mode is currently active throughout their gameplay session.

**Why this priority**: Critical for player awareness and transparency - players must know when their actions won't count toward legitimate scores.

**Independent Test**: Can be tested by activating cheat mode and verifying the indicator is visible, positioned correctly (lower right or left corner), and persists during gameplay.

**Acceptance Scenarios**:

1. **Given** cheat mode is activated, **When** player looks at the game screen, **Then** a clear indicator displaying "CHEAT MODE" or similar text appears in the lower right or left corner
2. **Given** cheat mode indicator is displayed, **When** player performs game actions, **Then** indicator remains visible and does not obscure critical gameplay elements
3. **Given** player deactivates cheat mode, **When** cheat mode is disabled, **Then** the indicator disappears from the screen

---

### Edge Cases

- What happens when player presses 'g' during a pause menu or transition screen?
  Input is ignored; no toggle occurs
- How does system handle rapid repeated presses of the 'g' key? cheatmode toggles
- What happens to the cheat mode status when player completes a level or dies? cheatmode stays active
- How does the cheat mode indicator behave on different screen resolutions or aspect ratios? scaled to screen height/width
- What visual or audio feedback should players receive when pressing level control keys without cheat mode active? short soft beep
- Can players earn achievements or progress while in cheat mode even though score resets to 0? yes

## Requirements *(mandatory)*

### Functional Requirements

#### Cheat Mode Activation

- **FR-001**: System MUST activate cheat mode when player presses the 'g' key during active gameplay
- **FR-002**: System MUST reset the player's score to 0 immediately upon cheat mode activation
- **FR-003**: System MUST ignore 'g' presses during non-gameplay states (menus, loading screens, pause/transition screens) and not toggle cheat mode

#### Cheat Mode Visual Indicator

- **FR-004**: System MUST display a persistent visual indicator when cheat mode is active
- **FR-005**: Visual indicator MUST be positioned in the lower right corner of the screen
- **FR-006**: Visual indicator MUST display white text reading "CHEAT MODE" on a semi-transparent dark background
- **FR-007**: Visual indicator MUST remain visible throughout the entire duration that cheat mode is active
- **FR-008**: Visual indicator MUST NOT obscure critical gameplay elements or UI components

#### Accidental Key Press Prevention

- **FR-009**: System MUST disable the 'R' key (respawn) during normal gameplay (when cheat mode is not active)
- **FR-010**: System MUST disable the 'N' and 'P' keys (cycle next/previous level) during normal gameplay (when cheat mode is not active)
- **FR-011**: System MUST only enable level control keys (R, N, P) after cheat mode has been activated
- **FR-012**: System MUST allow level control keys (R, N, P) to function normally when cheat mode is active
- **FR-013-NOTE**: The 'P' key currently invokes a texture picker UI; this functionality must be removed to enable previous level cycling

#### Cheat Mode Behavior

- **FR-014**: System MUST persist cheat mode across all level transitions (remains active when player moves between levels)
- **FR-015**: System MUST allow players to toggle cheat mode off by pressing 'g' again
- **FR-016**: System MUST reset the player's score to 0 when exiting cheat mode (toggling off)
- **FR-017**: System MUST allow score tracking and achievement earning while in cheat mode (score starts at 0 but can increase)
- **FR-018**: System MUST set the player's `LivesState.lives_remaining` to 3 when cheat mode is toggled on, and any active game-over overlay MUST be removed so the player may resume play (UI removal and lives reset should occur within 100ms of the toggle)

### Key Entities

- **Cheat Mode State**: Represents whether cheat mode is currently active for the player's session.
  Attributes include: activation status (on/off), activation timestamp
- **Level Control Action**: Represents player-initiated level control commands (respawn, cycle forward/backward).
  Attributes include: action type, confirmation status, timestamp
- **Score**: Represents the player's current score value.
  Must be resettable to 0 when cheat mode activates

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Players can activate cheat mode within 1 second of pressing 'g' key
- **SC-002**: Score resets to exactly 0 within 100 milliseconds of cheat mode activation
- **SC-003**: Cheat mode indicator is visible on screen within 100 milliseconds of activation
- **SC-004**: Accidental level control activations are reduced by 95% compared to current behavior
- **SC-005**: Level control keys execute actions 100% of the time when cheat mode is active, and 0% of the time when cheat mode is not active
- **SC-006**: Cheat mode indicator remains visible for 100% of the duration that cheat mode is active
- **SC-007**: Zero instances of cheat mode indicator obscuring critical gameplay UI elements during playtesting
