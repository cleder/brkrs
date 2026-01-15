# Quickstart: Audio Wall Delay Fix

## Prerequisites

- Rust 1.81 (edition 2021)
- Bevy 0.17.3
- bevy_rapier3d 0.32.0
- tracing 0.1

## How to Run

1. Clone the repository and checkout the `016-audio-wall-delay-fix` branch.
2. Run `cargo test` to verify all tests (including wall collision audio timing) pass.
3. Run `cargo run` to start the game.
4. Trigger a ball-wall collision and verify immediate audio feedback (<50ms delay).

## How to Test

- Integration tests for wall collision and audio timing are in `tests/`.
- To measure latency, run tests with `--nocapture` and check logs for event and audio timestamps.
- To simulate overload, trigger multiple wall collisions in rapid succession and verify concurrency limit behavior.

## Troubleshooting

- If audio is delayed, check system scheduling and ensure BallWallHit events are emitted and consumed in the same frame.
- If audio artifacts occur, verify concurrency limit and asset loading logic.

## Reference

- See `specs/016-audio-wall-delay-fix/spec.md` for requirements and acceptance criteria.
- See `specs/016-audio-wall-delay-fix/data-model.md` for entity/event structure.
- See `specs/016-audio-wall-delay-fix/contracts/ball_wall_hit_event.md` for event contract.

---

This quickstart provides all steps to build, test, and validate the Audio Wall Delay Fix feature.
