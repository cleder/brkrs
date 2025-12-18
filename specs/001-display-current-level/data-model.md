# Data Model: Display Current Level

**Feature**: Display Current Level (`001-display-current-level`) **Created**: 2025-12-18

## Entities

### Level

- **Description**: Represents level metadata used for display and logic.
- **Fields**:
  - `number: int` (required) — the ordinal level number
  - `name: string` (optional) — human-friendly level title
  - `type: enum("objective", "time", "endless")` (optional) — level type
  - `total_objectives: int?` (optional) — used for pause/summary progress

### PlayerProgress (optional)

- **Description**: Aggregated progress data for use in pause/summary screens.
- **Fields**:
  - `completed_objectives: int` (default 0)
  - `total_objectives: int?` (optional)
  - `percentage_complete: float?` (derived when total_objectives present)

### HUDConfig

- **Description**: Local configuration for HUD placement and accessibility.
- **Fields**:
  - `placement: enum("top-center","top-left","top-right")` (default `"top-center"`)
  - `font_scale: float` (UI scaling)
  - `accessible_label: string` (localizable label)

## Validation Rules

- `Level.number` MUST be a positive integer.
- If `PlayerProgress.total_objectives` is present and > 0, `percentage_complete` MUST be computed as `completed / total * 100` and rounded to integer percentage for display.

## State Transitions

- `LevelStarted(number)` triggers HUD update with `Level.number` and accessibility announcement.
- `LevelCompleted` or `PauseRequested` triggers summary view that may expose `PlayerProgress`.

## Notes

- `PlayerProgress` is optional and only required for pause/summary screens; absence should not break the HUD level number display.
