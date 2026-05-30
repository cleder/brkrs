# Feature Specification: Score Multiplier Bricks

**Feature Branch**: `066-score-multiplier-bricks` **Created**: 2026-05-30 **Status**: Draft **Input**: User description: "Score Multiplier Bricks. type 26-29.
When a score multiplier brick gets hit all following brick hits get multiplied by the factor.
When a life is lost the score goes back to normal.
Score multipliers are active until a ball is lost or another score multiplier brick gets hit"

## Clarifications

### Session 2026-05-30

- Q: Does the multiplier apply to the same hit that activates it?
  -> A: No. The triggering multiplier brick is scored at normal value; multiplier applies only to following hits.
- Q: Should multiplier reset on any ball despawn or only on actual life decrement?
  -> A: Reset only when the player's life counter decreases.
- Q: Should multiplier apply only to brick-destruction points or all score sources?
  -> A: Apply only to brick-destruction score awards.
- Q: Should multiplier persist across level transitions?
  -> A: Yes.
  Multiplier persists across level transitions unless reset conditions occur.
- Q: How should active multiplier be shown in the UI?
  -> A: Display `x2`, `x3`, or `x4` beneath the score indicator when that multiplier is active; display no multiplier indicator at `1x`.

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, tests must be written first and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: This feature touches ECS state and gameplay messages through score and life-loss flow.
Acceptance scenarios MUST verify Message-Event Separation by using a message-based score/life signal flow (Messages) and avoiding observer-only coupling for score persistence logic.
Acceptance scenarios MUST also verify hierarchy safety is unchanged (no new parent/child mutations are introduced by multiplier updates).

**MULTI-FRAME PERSISTENCE REQUIREMENT**: This feature changes runtime scoring state.
Tests MUST verify multiplier state persists across at least 10 consecutive `app.update()` frames while no reset condition occurs, and MUST verify reset behavior after a life-loss event.

### User Story 1 - Activate Multiplier on Brick Hit (Priority: P1)

As a player, when I hit a score multiplier brick (26-29), future brick score awards are scaled by that brick's factor.

**Why this priority**: This is the core user-facing behavior of the feature.

**Independent Test**: Hit a multiplier brick, then destroy a known-value brick and verify the awarded points are scaled by the active multiplier.

**Acceptance Scenarios**:

1. **Given** score is 0 and no multiplier is active, **When** the player hits brick 27 (Times 2), **Then** multiplier state becomes 2x and remains active for subsequent scoring events.
2. **Given** multiplier 2x is active and a 25-point brick is destroyed, **When** score is awarded, **Then** score increases by 50 points.
3. **Given** multiplier 3x is active and a 125-point brick is destroyed, **When** score is awarded, **Then** score increases by 375 points.
4. **Given** multiplier 4x is active and no reset event occurs, **When** the game advances 10 update frames and a 50-point brick is destroyed, **Then** score increases by 200 points, proving multi-frame persistence.
5. **Given** no multiplier is active and the player destroys brick 28 (Times 3), **When** points for that destruction are awarded, **Then** that brick awards its normal base score and 3x applies only to later brick hits.

---

### User Story 2 - Replace Existing Multiplier with New One (Priority: P1)

As a player, when I hit a different multiplier brick, the newest multiplier replaces the previous one.

**Why this priority**: Levels can contain multiple multiplier bricks, and players need deterministic score behavior.

**Independent Test**: Activate one multiplier, activate a second multiplier, then destroy a known brick and verify only the newest multiplier applies.

**Acceptance Scenarios**:

1. **Given** multiplier 2x is active, **When** the player hits brick 28 (Times 3), **Then** active multiplier becomes 3x.
2. **Given** multiplier 4x is active, **When** the player hits brick 26 (Times 1), **Then** active multiplier becomes 1x (normal scoring).
3. **Given** multiplier 3x is active and another multiplier brick is hit in the same gameplay sequence, **When** the next brick is destroyed, **Then** exactly one multiplier is applied and it is the most recently activated value.

---

### User Story 3 - Reset Multiplier on Life Loss (Priority: P1)

As a player, when I lose a life, score multiplier effects end and scoring returns to normal.

**Why this priority**: This is explicitly required behavior and affects game balance and predictability.

**Independent Test**: Activate a multiplier, trigger a life-loss event, then destroy a known brick and verify unmultiplied score gain.

**Acceptance Scenarios**:

1. **Given** multiplier 4x is active, **When** any life-loss event occurs, **Then** active multiplier resets to 1x before the next scoring event.
2. **Given** multiplier has reset after life loss and a 125-point brick is destroyed, **When** score is awarded, **Then** score increases by 125 points.
3. **Given** multiple balls are in play and an active multiplier exists, **When** one ball loss triggers a life-loss event, **Then** multiplier resets to 1x for all subsequent scoring.
4. **Given** multiplier 2x is active and a life-loss message is emitted, **When** systems process one frame, **Then** reset is applied through message-driven flow (Messages) and no observer-only path is required.
5. **Given** multiple balls are in play and one ball despawns without decreasing the life counter, **When** the next brick is destroyed, **Then** the previously active multiplier is still applied.
6. **Given** multiplier 3x is active and the player advances to the next level without losing a life, **When** the first brick in the new level is destroyed, **Then** its brick-destruction score is multiplied by 3.

---

### User Story 4 - Show Active Multiplier Indicator (Priority: P2)

As a player, when a multiplier greater than 1x is active, I can see a small multiplier indicator beneath the score display so I know which multiplier is currently affecting future brick scores.

**Why this priority**: The scoring mechanic works without the indicator, but the UI feedback reduces confusion and lets players understand the active scoring state.

**Independent Test**: Activate each multiplier and verify the UI displays `x2`, `x3`, or `x4` beneath the score indicator; then reset to `1x` and verify the indicator disappears.

**Acceptance Scenarios**:

1. **Given** multiplier 2x is active, **When** the score UI updates, **Then** an `x2` indicator is displayed beneath the score indicator.
2. **Given** multiplier 3x is active, **When** the score UI updates, **Then** an `x3` indicator is displayed beneath the score indicator.
3. **Given** multiplier 4x is active, **When** the score UI updates, **Then** an `x4` indicator is displayed beneath the score indicator.
4. **Given** multiplier resets to 1x because of life loss or brick 26, **When** the score UI updates, **Then** no multiplier indicator is displayed.
5. **Given** multiplier state does not change between frames, **When** the app runs for 10 update frames, **Then** the indicator remains stable without unnecessary per-frame content changes.

### Edge Cases

- A multiplier brick is hit and a life-loss event occurs in the same frame: life-loss reset wins, so subsequent scoring is normal (1x).
- A non-terminal ball despawn in multi-ball play that does not decrement lives must not reset multiplier state.
- A Times 1 brick (26) is hit while another multiplier is active: this acts as an explicit reset to normal scoring without requiring life loss.
- Multiplier activation does not retroactively change points already awarded earlier in the same game.
- Non-scoring events (pause, level transition UI, audio triggers) do not alter active multiplier state.
- Level transition without life decrement does not reset multiplier state.
- The UI must not show a multiplier indicator when the active multiplier is 1x.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support multiplier bricks 26-29 as score multiplier activators with factors 1x, 2x, 3x, and 4x respectively.
- **FR-002**: System MUST apply the active multiplier only to score awards from brick hits that occur after multiplier activation.
- **FR-003**: System MUST not retroactively recalculate or alter score already awarded before a multiplier change.
- **FR-004**: System MUST replace the currently active multiplier when another multiplier brick is hit.
- **FR-005**: System MUST reset the active multiplier to normal scoring (1x) whenever a life-loss event occurs.
- **FR-006**: System MUST keep multiplier state active across frames until either a life-loss event or another multiplier brick hit changes it.
- **FR-007**: System MUST ensure exactly one multiplier value is active at any time.
- **FR-008**: System MUST process multiplier reset and scoring via message-driven flow consistent with existing score/life gameplay messaging, preserving Message-Event Separation.
- **FR-009**: System MUST preserve existing scoring behavior for all non-multiplier bricks when active multiplier is 1x.
- **FR-010**: System MUST keep multiplier behavior consistent in single-ball and multi-ball states.
- **FR-011**: System MUST award the triggering multiplier brick (26-29) at normal base score for that hit; the newly activated multiplier MUST apply starting with subsequent brick score awards.
- **FR-012**: System MUST NOT reset multiplier state on ball despawn events that do not decrease the player's life counter.
- **FR-013**: System MUST apply multiplier scaling only to score awards produced by brick-destruction events; non-brick score sources MUST remain unscaled.
- **FR-014**: System MUST preserve active multiplier state across level transitions unless a defined reset trigger occurs (life decrement or explicit replacement by another multiplier brick).
- **FR-015**: System MUST display an active multiplier indicator beneath the score indicator when the active multiplier is 2x, 3x, or 4x.
- **FR-016**: System MUST hide the multiplier indicator when the active multiplier is 1x.
- **FR-017**: System MUST update multiplier indicator text only when multiplier state changes, consistent with Bevy change-detection guidance for UI updates.

### Key Entities *(include if feature involves data)*

- **Score Multiplier State**: Current score factor (1x, 2x, 3x, 4x) used to scale future brick score awards.
- **Multiplier Brick Type**: Brick index 26-29 mapping to factor activation behavior.
- **Life Loss Event**: Gameplay event that resets multiplier state to 1x.
- **Brick Score Award**: Scoring action generated by a valid brick hit and scaled by current multiplier state.
- **Multiplier Indicator**: UI text element positioned beneath the score indicator that displays `x2`, `x3`, or `x4` when an active multiplier above `1x` exists and is hidden at `1x`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In test scenarios, 100% of brick score awards after hitting brick 27, 28, or 29 are scaled by 2x, 3x, or 4x respectively until reset conditions occur.
- **SC-002**: In test scenarios, 100% of scoring events after a life-loss event use normal scoring (1x) until another multiplier brick is hit.
- **SC-003**: Multiplier state persists correctly across at least 10 consecutive update frames with no unintended reset in 100% of persistence tests.
- **SC-004**: When two different multiplier bricks are hit sequentially, the second one becomes authoritative for 100% of subsequent score awards.
- **SC-005**: Existing non-multiplier scoring remains unchanged when active multiplier is 1x, with no regressions in baseline scoring tests.
- **SC-006**: In 100% of activation tests, the multiplier brick that activates 2x/3x/4x is scored at normal base value, and only later brick hits are multiplied.
- **SC-007**: In multi-ball tests where a ball despawns without life decrement, multiplier state remains unchanged in 100% of cases.
- **SC-008**: In 100% of scope tests, only brick-destruction score awards are multiplied; non-brick score sources are unchanged.
- **SC-009**: In 100% of level-transition tests without life decrement, multiplier state remains unchanged into the next level.
- **SC-010**: In 100% of UI tests, active multipliers `2x`, `3x`, and `4x` display the matching indicator beneath the score indicator.
- **SC-011**: In 100% of reset-state UI tests, no multiplier indicator is visible when active multiplier is `1x`.

## Assumptions

- Brick base scores continue to follow the values documented in `docs/bricks.md`.
- Multiplier bricks continue to award their own base points on destruction according to existing scoring rules; multiplier effect applies to following brick hits, not prior ones.
- A life-loss event is the authoritative reset trigger for multiplier state, including during multi-ball gameplay.
- A dedicated UI indicator beneath the score display is required for active multipliers above `1x`.
- Level transitions do not implicitly modify score multiplier state.
