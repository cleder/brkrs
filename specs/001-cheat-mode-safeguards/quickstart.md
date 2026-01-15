# Quickstart: Cheat Mode Safeguards

## Build & Test

- `cargo fmt --all`
- `cargo clippy --all-targets --all-features`
- `cargo test`
- `bevy lint`

## Manual Verification

1) Launch game (debug build). 2) During gameplay, press `g` → cheat mode activates, score resets to 0, indicator shows bottom-right. 3) Press `g` again → cheat mode deactivates, score resets to 0, indicator hides. 4) Press `R/N/P` while cheat mode **inactive** → no action, hear short soft beep, level unchanged. 5) Press `R/N/P` while cheat mode **active** → respawn/next/previous level works. 6) Press `g` during pause/transition → ignored, no toggle. 7) Verify indicator does not obscure gameplay and remains visible while active.

## Notes

- 'P' previously opened texture picker; that functionality must be removed so 'P' is free for previous-level control.
- Observability: expect tracing events for cheat toggles and blocked level-control attempts.
