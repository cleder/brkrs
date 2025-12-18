# Cheat Mode (developer/testing)

Cheat Mode is a testing/developer feature that allows quick exploration and debugging of levels and mechanics.

## How to toggle

- Press `G` during active gameplay to toggle Cheat Mode on or off.

## Behavior

- When Cheat Mode is toggled (either on or off), the player's current **score** is reset to `0`.
- When Cheat Mode is enabled, a persistent `CHEAT MODE` indicator appears in the lower-right corner of the screen so the player knows the session is in cheat mode.
- Level-control keys (R = respawn, N = next level, P = previous level) are gated to Cheat Mode: they only execute when Cheat Mode is active.
  If they are pressed while Cheat Mode is inactive, a short soft UI beep plays and the action is ignored.
- If Cheat Mode is toggled while a **Game Over** overlay is active (i.e., the player has 0 lives), Cheat Mode activation will set `LivesState.lives_remaining` to `3` and remove the Game Over overlay so the player can resume play.
  Note: toggling Cheat Mode does **not** reload or reset the current level — gameplay resumes in-place with the level state unchanged.

## Notes & Testing

- Use Cheat Mode for rapid iteration or to explore levels without the normal gating of level-control keys.
- The feature is intended for debugging and testing; enable it intentionally — the UI indicates when it's active.
- Unit and integration tests for Cheat Mode are in `tests/cheat_mode.rs` and `tests/restart_cheat.rs`.
