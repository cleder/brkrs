# Feature Specification: Remove Legacy Game Over Overlay

**Feature Branch**: `028-remove-game-over-overlay` **Created**: 2026-03-10 **Status**: Draft **Input**: User description: "Fix: when all lives were lost and I start a new game the game over overlay is displayed during gameplay.
The overlay is a legacy UI and can be removed entirely"

## Clarifications

### Session 2026-03-10

- Q: How far should overlay removal scope go? -> A: Remove the legacy game-over overlay behavior entirely in all flows; keep non-overlay game-over state handling unchanged.
- Q: Should this feature add a replacement game-over UI? -> A: Do not add any replacement game-over UI in this feature; only remove the legacy overlay.

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, **tests must be written first** and included in this spec as testable acceptance scenarios.
Tests MUST be committed before implementation and a failing-test commit (red) MUST exist in the branch history as proof.

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS systems, queries, events/messages, rendering, assets, UI updates, or hierarchy, the implementation MUST comply with the constitution's **Bevy 0.17 mandates & prohibitions**.
Acceptance scenarios SHOULD include at least one check that guards against prohibited patterns (e.g., panicking queries or per-frame UI updates without `Changed<T>`).
Acceptance criteria MUST explicitly state which event system is used (Messages vs Observers), justify the choice, and check for **Message-Event Separation** (correct use of `MessageWriter` vs observers/ `Trigger<T>`) and **Hierarchy Safety** (use of `commands.entity(parent).add_child(child)` or `EntityCommands::set_parent`).

**COORDINATE SYSTEM REQUIREMENT**: If the feature involves spatial movement, physics velocity, or directional behavior, the specification MUST include a coordinate system note clarifying:

- Which axes are used for movement (XZ plane for horizontal, Y for vertical, etc.)
- Whether directional terms (forward/backward/left/right) refer to Bevy's Transform API convention (forward = -Z), gameplay-relative directions (player perspective), or direct axis manipulation (±X, ±Y, ±Z)
- How the camera view orientation affects gameplay directions
- Any locked axes via `LockedAxes` constraints

**MULTI-FRAME PERSISTENCE REQUIREMENT**: If the feature involves runtime state changes (gravity, scores, powerup effects, or any resource/component modified during gameplay), acceptance scenarios MUST include multi-frame persistence checks:

- Tests MUST verify state persists across multiple `app.update()` cycles (minimum 10 frames)
- Tests MUST include ALL systems that write to the affected resource/component to catch per-frame overwrite bugs
- This requirement exists because single-frame assertions miss bugs where initialization or cleanup systems unconditionally overwrite runtime state (see 020-gravity-bricks retrospective)

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

### User Story 1 - Clean Restart After Life Loss (Priority: P1)

As a player, when I lose all lives and start a new game, gameplay should resume without any game-over overlay covering or interfering with the playfield.

**Why this priority**: This is the reported gameplay regression and directly affects core playability after restart.

**Independent Test**: Can be fully tested by forcing a game-over condition, starting a new game, and verifying no game-over overlay is visible while gameplay is active.

**Acceptance Scenarios**:

1. **Given** a player has reached zero lives and ended a run, **When** the player starts a new game, **Then** no game-over overlay is present during active gameplay.
2. **Given** gameplay has restarted after a prior game over, **When** the game runs for multiple frames and state transitions that normally occur during early play, **Then** no legacy game-over overlay appears at any point.
3. **Given** the game-over to restart flow uses existing game-over signaling, **When** this feature is implemented, **Then** buffered message routing remains in use for `GameOverRequested` and no observer-trigger replacement is introduced.
4. **Given** the player starts a new game and resumes active play after overlay removal, **When** normal control inputs are used, **Then** new-game and gameplay controls remain responsive and unchanged.

---

### User Story 2 - No Legacy Overlay Artifacts (Priority: P2)

As a player, I should not see legacy game-over UI artifacts in any flow, including first run, game-over transitions, and repeated restarts.

**Why this priority**: Removing the legacy overlay entirely prevents repeated UI regressions and simplifies user experience consistency.

**Independent Test**: Can be tested by launching the game, playing through multiple start/game-over/restart cycles, and verifying that no game-over overlay entity is ever rendered in gameplay.

**Acceptance Scenarios**:

1. **Given** a fresh launch or any subsequent restart cycle, **When** gameplay enters the active playing state, **Then** no legacy game-over overlay is displayed.
2. **Given** a run reaches game over, **When** the game transitions through end-of-run handling, **Then** no legacy game-over overlay is shown at any point.
3. **Given** touched UI lifecycle paths are updated as part of this feature, **When** UI parent-child relationships are modified, **Then** hierarchy-safe APIs are used and no manual `Parent`/`Children` mutation is introduced.

---

### Edge Cases

- Player starts a new game immediately after triggering game over; overlay must still never appear.
- Player goes through several consecutive game-over and restart cycles in one session; no legacy overlay entities should accumulate or reappear.
- Game loads directly into active gameplay on first run; no legacy overlay should be visible.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST remove the legacy game-over overlay behavior entirely across all game flows.
- **FR-002**: The system MUST ensure starting a new game after losing all lives does not display any game-over overlay during active gameplay.
- **FR-003**: The system MUST ensure repeated game-over and restart cycles do not reintroduce the removed overlay.
- **FR-004**: Existing new-game and gameplay controls MUST remain functional after overlay removal.
- **FR-005**: Automated acceptance tests MUST cover the regression path: lose all lives, start a new game, verify no overlay appears.
- **FR-006**: Because this feature changes UI behavior, acceptance tests MUST verify event/message routing and parent-child UI relationships remain correct for any touched UI lifecycle paths.
- **FR-007**: This feature MUST NOT introduce a new game-over overlay or replacement game-over UI.

## Assumptions

- The legacy game-over overlay is not required for current gameplay UX and is removed with no replacement UI in this feature.
- Players can still determine game progress through existing non-overlay cues and restart flow.
- Any non-overlay game-over handling (state transitions, life reset, level reset) remains in scope and should continue to work unchanged.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of tested runs where a player loses all lives and starts a new game, active gameplay is shown without a game-over overlay.
- **SC-002**: In at least 10 consecutive automated restart cycles within one session, no legacy game-over overlay is displayed during gameplay.
- **SC-003**: All existing tests related to life loss and restart behavior continue to pass with no regressions in starting a new game.
- **SC-004**: QA can complete the restart flow (game over -> new game -> active gameplay) without any visual obstruction from legacy overlay UI.
