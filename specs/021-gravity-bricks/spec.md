# Feature Specification: Gravity Indicator UI

**Feature Branch**: `021-gravity-bricks` **Created**: 2026-01-11 **Status**: Draft **Input**: "Display an indicator for the current gravity; round X/Z components to integers with ±0.5 tolerance (Y ignored); use weight icons; place bottom-left opposite developer indicator; follow-up of 020-gravity-bricks."

## User Scenarios & Testing *(mandatory)*

**TDD REQUIREMENT**: For every user story, tests must be written first and included as testable acceptance scenarios.
Tests MUST be committed before implementation and include a failing-test commit (red).

**BEVY 0.17 REQUIREMENT**: If the feature touches ECS, rendering, assets, UI updates, or hierarchy, acceptance scenarios should guard against prohibited patterns (e.g., per-frame UI updates without change detection).
Acceptance criteria must state which event system is used and verify Message-Event Separation and Hierarchy Safety.

**COORDINATE SYSTEM REQUIREMENT**: Gravity affects ball movement; X/Z are horizontal axes, Y is vertical.
Camera orientation follows Bevy defaults.

**MULTI-FRAME PERSISTENCE REQUIREMENT**: Verify gravity indicator correctness persists across ≥10 frames after changes and through systems that write gravity.

### User Story 1 - See Current Gravity (P1)

Players need a clear, at-a-glance indicator of current gravity level to anticipate ball behavior.

**Independent Test**: Change gravity via gravity brick or level default and verify indicator updates.

**Acceptance Scenarios**:

1. Given level default gravity, When the first frame with valid GravityConfiguration executes, Then indicator spawns at lower-left showing the correct level (0/2/10/20 or unknown).
2. Given gravity changes via brick, When gravity updates, Then indicator updates within one frame to corresponding icon.
3. Given sequential gravity changes in quick succession (including multiple changes in a single frame), When last change takes effect, Then indicator reflects final gravity from that frame.
4. Given minor float variance, When values are within ±0.5 of targets, Then indicator uses nearest target (0, 2, 10, 20).
5. Given values outside all targets, When no tolerance-match is found, Then indicator shows the question icon.

### User Story 2 - Non-Intrusive Placement (P2)

Indicator should be visible but not distracting, opposite the developer indicator.

**Independent Test**: Verify position anchors while changing window mode/resolution.

**Acceptance Scenarios**:

1. Given windowed/fullscreen modes, When UI renders, Then indicator anchors bottom-left (12px offsets) and remains visible.
2. Given developer indicator bottom-right, When both visible, Then they do not overlap and appear on opposite corners.
3. Given the game-over overlay is displayed, When UI renders, Then the gravity indicator remains visible above the overlay (z-order/layering is correct).

*Note: FR-007 requires visibility over overlays; this scenario verifies layering explicitly.*

### User Story 3 - Robust Through Pause & Life Loss (P3)

Indicator should remain consistent through pausing and gravity reset on life loss.

**Independent Test**: Toggle pause and trigger life loss, verify indicator stability.

**Acceptance Scenarios**:

1. Given game paused, When gravity does not change, Then indicator remains static and visible.
2. Given life loss resets gravity to level default, When gameplay resumes, Then indicator reflects reset gravity.

### Edge Cases

- Thresholds near targets (e.g., 1.6–2.4, 9.6–10.4) round to 2 or 10 (±0.5 tolerance).
- Mixed X/Z gravity: select highest recognized level among axes (20 > 10 > 2 > 0).
- Random gravity (brick 25): if no match within tolerance, show question icon.
- Multiple changes per frame: indicator shows the last applied gravity value (matches physics last-write-wins behavior).
- Level transitions: indicator updates automatically to new level's default gravity via GravityConfiguration change detection.
- Asset loading: spawn deferred until both GravityConfiguration and textures are loaded to prevent blank display.
- Asset availability: if textures missing, skip indicator and log non-fatal warning.

## Clarifications

### Session 2026-01-11

- Q: When should the gravity indicator first appear on screen? → A: At the start of the first frame with valid GravityConfiguration
- Q: Should the icon swap instantly or use a visual transition effect? → A: Instant swap to new icon (no transition animation)
- Q: How should the indicator handle multiple gravity changes within a single frame? → A: Display the last applied gravity value from that frame
- Q: What should happen to the indicator during level transitions? → A: Update to new level's default gravity automatically
- Q: What should happen if GravityConfiguration exists but indicator textures aren't loaded yet? → A: Defer spawn until both GravityConfiguration and textures are loaded

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Display a gravity indicator icon anchored bottom-left, opposite developer indicator.
  Spawn occurs at the first frame when both GravityConfiguration resource and indicator textures are available.
- **FR-002**: Map current gravity `Vec3` to indicator levels using integer rounding with ±0.5 tolerance on X/Z axes (Y ignored).
- **FR-003**: Recognized levels: 0, 2, 10, 20.
  If none match, display the question icon.
- **FR-004**: For mixed-axis matches, select the highest recognized level among X/Z for display.
- **FR-005**: Update the indicator within one frame when gravity changes (level load, brick destruction, life loss reset).
  Icon swap is instant with no transition animation.
- **FR-006**: Use assets: assets/textures/default/weight-0.png, weight-2.png, weight-10.png, weight-20.png, and weight-question.png; failure is non-fatal (log and skip).
- **FR-007**: Indicator remains visible over game-over overlay and consistent across window modes and pauses.
- **FR-008**: Indicator correctness persists across ≥10 frames after change unless gravity changes again.

### Key Entities

- **GravityConfiguration**: Resource with `current` and `level_default` gravity.
- **Gravity Indicator UI Entity**: Bottom-left image node updated on gravity changes.
- **Gravity Indicator Textures**: Handles to the five indicator images.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Indicator updates within one frame after gravity changes.
- **SC-002**: 95% of changes yield correct indicator selection under ±0.5 tolerance across 10 frames.
- **SC-003**: Indicator visible and correctly positioned bottom-left across supported window modes.
- **SC-004**: No blocking issues related to indicator visibility/placement or asset failures.
