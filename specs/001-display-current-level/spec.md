# Feature Specification: Display Current Level

**Feature Branch**: `001-display-current-level` **Created**: 2025-12-18 **Status**: Draft **Input**: User description: "display the current level.
As a player I want to have feedback about my progress and see the number of the level I am currently playing"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - See level number while playing (Priority: P1)

As a player playing a level, I want to see the level number displayed clearly so I always know which level I'm on.

**Why this priority**: This is core orientation feedback that reduces confusion and improves confidence while playing.

**Independent Test**: Start any level and verify the HUD shows the correct level number within 1 second of level start.
This can be tested manually and with a UI integration test that loads a level and checks the HUD element.

**Acceptance Scenarios**:

1. **Given** the game has loaded level 3, **When** the level starts, **Then** the HUD displays "Level 3" in the configured location within 1 second.
2. **Given** the player continues playing, **When** gameplay proceeds, **Then** the level number stays visible and accurate until level end or transition.

---

### User Story 2 - No in-level progress indicator (Priority: P2)

As a player, I should not be shown a detailed progress indicator during active gameplay; the HUD will only show the level number so as to reduce HUD clutter and help focus on gameplay.

**Why this priority**: The product decision is to keep the HUD minimal during play to reduce distraction.

**Independent Test**: Start an objective-based level and verify that during active gameplay no numeric progress (e.g., X/Y or percentage) or progress bar is visible; only the level number is shown.

**Acceptance Scenarios**:

1. **Given** the game is in active play on level 7, **When** objectives are completed, **Then** no in-play progress metrics or bar appear on HUD; level number remains visible.
2. **Given** rapid completion of objectives, **When** many objectives are completed quickly, **Then** the HUD remains free of progress metrics until pause or summary screens.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST display the current level number on-screen when a level starts.
- **FR-002**: The level number display MUST appear within 1 second of level start.
- **FR-003**: The level number MUST remain visible and accurate during gameplay until level end or transition.
- **FR-004**: During active gameplay the HUD MUST only display the level number and MUST NOT display detailed progress metrics (numeric X/Y, percentage, or progress bar).
- **FR-006**: The level number and progress text (where shown) MUST be accessible to screen readers and meet legibility expectations across supported resolutions.
- **FR-007**: The display components MUST gracefully handle missing level metadata by showing a fallback label and logging the issue.

*Notes/Assumptions*:

- Default placement: top-center of the HUD.
  This choice is based on common HUD patterns and minimizes overlap with core gameplay.
- Display format default: "Level {N}"; no live progress metrics are shown during active gameplay.
- Pause and summary screens MAY include final progress summaries (e.g., "12/20 — 60%") where useful.

### Key Entities *(include if feature involves data)*

- **Level**: Represents level metadata.
  Key attributes: number (int), name (optional), total_objectives (optional), type (objective-based/time-based/endless)
- **HUD Element**: UI element displaying level number and, optionally, progress in pause/summary.
  Key attributes: placement, content, accessibility label

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: When a level starts, 100% of test runs show the correct level number displayed within 1 second.
- **SC-002**: During active gameplay, 100% of test runs show no detailed progress indicator on the HUD; only the level number is visible.
- **SC-003**: In UX validation with at least 20 participants, 95% can correctly state the current level when asked without guidance.
- **SC-004**: Accessibility validation: screen readers announce level number and progress (where shown in pause/summary) on request.
- **SC-005**: No visual overflow or overlap occurs on the 5 most common screen resolutions supported by the product.

## Assumptions

- Placement default: top-center HUD.
  This can be changed if design preferences require a different location.
- Progress indicators are intentionally omitted during active gameplay; pause and summary screens may show final progress where it aids player understanding.
- Language/localization: level number is language-agnostic (numeric).
  Any surrounding text should be localizable.

---

**Spec status**: Draft — no open clarifications.
