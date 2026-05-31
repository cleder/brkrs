# Data Model: Score Multiplier Bricks

## Entity: ScoreMultiplierState

- Purpose: Holds the currently active score multiplier used for brick-destruction scoring.
- Fields:
  - `factor`: enum/int constrained to `{1,2,3,4}`
  - `source_brick_index`: optional `u32` for latest activating brick (`26..=29`) for diagnostics/testing
  - `updated_frame`: optional frame tick for deterministic testing
- Validation rules:
  - `factor` must always remain in `{1,2,3,4}`.
  - `source_brick_index` if present must be one of `26,27,28,29`.
- State transitions:
  - `1 -> 2/3/4` when multiplier brick 27/28/29 is destroyed
  - `2/3/4 -> 1` when multiplier brick 26 is destroyed
  - `2/3/4 -> 1` when life decrements
  - `2/3/4 -> 2/3/4` replacement when another multiplier brick is destroyed
  - Any `factor` persists across level transition without life decrement

## Entity: BrickScoreAward

- Purpose: Represents score award caused by a brick-destruction event.
- Fields:
  - `brick_index`: `u32`
  - `base_points`: `u32`
  - `multiplier_applied`: `u32`
  - `awarded_points`: `u32`
- Validation rules:
  - `awarded_points = base_points * multiplier_applied`
  - `multiplier_applied` for activating multiplier brick is previous factor (typically `1` unless prior multiplier existed)
  - Only brick-destruction events are eligible for multiplier scaling

## Entity: LifeCounter

- Purpose: Authoritative life count used to trigger multiplier reset semantics.
- Fields:
  - `current_lives`: bounded integer
  - `last_life_change_frame`: optional frame tick
- Validation rules:
  - Multiplier reset happens only when `current_lives` decreases.
  - Ball despawn without life decrement must not change multiplier state.

## Entity: MultiplierIndicator

- Purpose: Represents the UI state for the multiplier text shown beneath the score indicator.
- Fields:
  - `visible`: `bool`
  - `label`: optional string constrained to `"x2"`, `"x3"`, or `"x4"`
  - `last_synced_factor`: optional `u32`
- Validation rules:
  - `visible = false` when multiplier factor is `1`.
  - `visible = true` only when multiplier factor is `2`, `3`, or `4`.
  - `label` must match the active multiplier factor when visible.
  - UI state updates only when the active multiplier factor changes.

## Relationships

- `BrickScoreAward` reads `ScoreMultiplierState.factor` to compute awarded points.
- `LifeCounter` decrement writes reset transition for `ScoreMultiplierState`.
- Multiplier brick destruction writes both `BrickScoreAward` and `ScoreMultiplierState` transition.
- `MultiplierIndicator` mirrors `ScoreMultiplierState.factor` for presentation beneath the score indicator.
