# Quickstart: Ball Lives Counter

## Prerequisites

- Rust toolchain installed (project targets Rust 1.81)

## Run locally

From repo root:

- `cargo run`

## Verify manually

1. Start a new play session.
2. Confirm the on-screen lives/balls counter starts at **3**.
3. Lose a ball (e.g., let the ball pass the lower goal).
4. Confirm the counter decrements by **1** each time.
5. Lose balls until the counter reaches **0**.
6. Confirm a **Game over** message appears when the last ball is lost and remains visible while lives are 0.

## Verify with tests

From repo root:

- `cargo test`

## Dev checks

From repo root:

- `cargo fmt --all`
- `cargo clippy --all-targets --all-features`
- `bevy lint`
