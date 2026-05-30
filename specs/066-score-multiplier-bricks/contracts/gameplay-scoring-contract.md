# Contract: Gameplay Scoring Multiplier Semantics

## Scope

Internal gameplay contract for score multiplier behavior tied to multiplier bricks (`26..=29`), score awards, and life decrement events.

## Inputs

- Brick destruction signal with `brick_index` and base score.
- Life state change signal indicating whether player lives decreased.
- Level transition signal/context change.

## Outputs

- Updated total score.
- Updated active multiplier factor.
- Updated multiplier indicator visibility/text beneath the score indicator.

## Behavioral Rules

1. Multiplier mapping:
   - Brick `26` => active factor `1`
   - Brick `27` => active factor `2`
   - Brick `28` => active factor `3`
   - Brick `29` => active factor `4`
2. The multiplier brick that triggers activation is scored at normal base value for that hit.
3. Active multiplier applies only to subsequent brick-destruction score awards.
4. Non-brick score sources are never multiplied.
5. Hitting a new multiplier brick replaces previous active multiplier immediately for following hits.
6. Reset to `1x` only when player life counter decreases.
7. Ball despawn without life decrement does not reset multiplier.
8. Level transition does not reset multiplier unless accompanied by life decrement.
9. When active multiplier is `2`, `3`, or `4`, the UI displays `x2`, `x3`, or `x4` beneath the score indicator.
10. When active multiplier is `1`, the multiplier indicator is hidden.
11. Multiplier indicator updates occur only when multiplier state changes.

## Test Assertions

- Deterministic scoring for activation/replace/reset sequences.
- Persistence across >=10 frames without reset trigger.
- Multi-ball non-life despawn does not reset multiplier.
- Level transition without life decrement preserves multiplier.
- UI indicator shows the correct value for `2x`, `3x`, and `4x` and is hidden at `1x`.
