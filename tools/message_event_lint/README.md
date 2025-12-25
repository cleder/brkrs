# message_event_lint

Rust-based linter to detect likely misuse of `MessageWriter<T>` with immediate side-effects (spawn, asset load, audio.play, etc.).
Uses `syn` to parse the Rust source and make syntactic checks.

Usage:

cargo run -- <file.rs> [<file2.rs> ...]

CI integration:

- `cargo test` runs unit tests (included)
- `cargo run -- <paths>` can be used to run against repository files
