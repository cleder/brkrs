# Research: Display Current Level

**Feature**: Display Current Level (`001-display-current-level`) **Created**: 2025-12-18

## Decisions

### Decision 1: HUD placement and sizing

- **Decision**: Place the level indicator at the top-center of the HUD with a compact font and safe-area margins; allow scaling for smaller viewports and maintain legibility.
  Use a single-line label format: `"Level {N}"`.
- **Rationale**: Top-center is a common, non-intrusive location that minimizes overlap with active gameplay in this project’s UI layout and matches the initial spec assumption.
- **Alternatives considered**:
  - Top-left: may conflict with lives/score HUD elements.
  - Bottom-center: conflicts with player controls on mobile.

### Decision 2: Accessibility behavior

- **Decision**: On level start, announce `"Level {N}"` via an accessibility live region with `polite` priority (non-blocking) and ensure the HUD element has an accessible label.
  Pause/summary screens should be accessible as dialogs announcing the level and summary on open.
- **Rationale**: Provides screen-reader users the same orientation information without interfering with gameplay.
- **Alternatives considered**:
  - No announcement and rely only on persistent HUD label (insufficient for screen-reader users).

### Decision 3: Progress visibility policy

- **Decision**: No detailed progress indicator will be shown during active gameplay; pause and summary screens will display final progress when objective metadata exists (format `X/Y — NN%`).
  If no objective metadata exists, show `"Progress: N/A"`.
- **Rationale**: Matches product decision to reduce HUD clutter and focuses attention during play, while preserving the ability to report progress in review contexts.
- **Alternatives considered**:
  - Show minimal progress bar during gameplay — rejected to keep HUD minimal.

## Implementation Implications

- The HUD element must support localization and dynamic updates of the label text.
- Accessibility hooks must be integrated into the UI layer; for web/WASM targets use an ARIA live region or equivalent.
- Pause and summary screens must query `PlayerProgress` (if present) to render final progress.

## Unresolved / Out-of-Scope

- Detailed visual styling and exact typeface choices are delegated to design and theming; this research focuses on behavior and accessibility.

---

**Next step**: Create data model and local contracts for the HUD events and pause/summary data used by this feature.
