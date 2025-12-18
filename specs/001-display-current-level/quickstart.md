# Quickstart: Display Current Level

**Purpose**: Quick steps to run and validate the Display Current Level feature locally.

## Run locally (native)

1. Checkout branch: `git checkout 001-display-current-level`
2. Build and run: `cargo run --example demo --features ui` (replace with the project run command if different)
3. Start a level and observe the top-center HUD shows "Level {N}" within 1 second.

## Run tests

- Unit tests: `cargo test --lib` (add or focus tests under `src/ui` or `src/systems` for HUD)
- Integration tests: Add an integration test that simulates `LevelStarted` and asserts HUD state

## Manual validation checklist

- [ ] Level label appears within 1 second of level start
- [ ] No in-play progress metrics are visible during active gameplay
- [ ] Pause/summary can render final progress where applicable
- [ ] Accessibility: screen reader announces level label on level start

## Notes

- Replace example run commands above with the project's standard run steps if they differ.
