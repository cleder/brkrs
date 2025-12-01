---
title: "Retrospective — 006-audio-system"
date: 2025-12-01
authors:
  - copilot
  - christian
---

# Retrospective — 006-audio-system

Summary

- Implementation: Added the audio subsystem that maps game events to sound playback, `AudioConfig` persistence for native and WASM (via `localStorage`), and defensive asset loading so tests can run without the Bevy AssetServer present.
- Tests: Converted flaky tests that wrote into `assets/` to tempfile-backed tests and added a graceful-degradation integration test for missing audio assets.

Timeline

- Investigated feature spec and requirements under `specs/006-audio-system/`.
- Implemented `src/systems/audio.rs` (audio plugin, event observers, `AudioConfig`, `AudioAssets`, `ActiveSounds`).
- Made `load_audio_assets` defensive to avoid panics in minimal test harnesses.
- Converted tests (`tests/level_definition.rs`, etc.) to use `tempfile` and add `BK_LEVEL_PATH` overrides in test runs.
- Implemented WASM persistence for `AudioConfig` (key `brkrs_audio`), gated to the wasm target.
- Added integration test `tests/audio_events.rs` to ensure safe startup without audio assets.
- Ran the full test suite until green and documented CI/test isolation guidance.

Key files changed

- `src/systems/audio.rs` — AudioPlugin, observers, asset loading, WASM persistence.
- `src/level_loader.rs` — Accept `BK_LEVEL_PATH` override for tests.
- `tests/level_definition.rs` — Converted to use `tempfile::NamedTempFile`.
- `tests/audio_events.rs` — Added graceful-degradation integration test.
- `Cargo.toml` — Added `tempfile` dev-dependency, added wasm-gated `web-sys` for `localStorage` access.
- `specs/006-audio-system/*` — Updated `tasks.md`, `plan.md`, and `quickstart.md` with status and notes.

What went well

- Spec-driven work: The speckit-style flow made the implementation scoped and traceable to tasks in `specs/006-audio-system/tasks.md`.
- Tests: Converting tests to tempfile-backed artifacts eliminated flakiness caused by shared `assets/` writes and made the suite reliable when run in full.
- Defensive coding: Making asset loading resilient when `AssetServer` is absent made test harnesses simpler and reduced test setup complexity.

What didn't go well / problems encountered

- Flaky test caused by tests writing into shared `assets/` caused intermittent failures when running the full suite in parallel.
- PR creation automation in the agent environment required authentication; the CLI was installed but interactive `gh auth login` or a token was needed.

Resolutions and decisions

- Use `tempfile` for tests that need writable files and add `BK_LEVEL_PATH` for path injection — results: reproducible tests and no repository pollution.
- Gate browser persistence to wasm builds and use `localStorage` key `brkrs_audio` for AudioConfig — keeps parity between native file persistence and wasm.
- Keep `load_audio_assets` defensive to avoid adding test-only setup scaffolding.

Learnings

- Tests that depend on repo-local writable files must either use temp files or the CI must enforce single-threaded tests; the former is more robust for parallel CI.
- Implementing a small defensive contract (skip loading if missing resources) reduces test-maintenance overhead.

Follow-ups / Next steps

- (Optional) Add a CI workflow change to set `RUST_TEST_THREADS=1` for test jobs that require env-var isolation, or ensure all tests use tempfiles.
- (Optional) Add small placeholder OGG assets for manual runtime testing (task T043) or add a smoke test that runs audio playback in CI (if CI supports audio output or headless emulation).
- Confirm the PR created by the user and merge once reviewed; link PR number here for traceability.

PR / Merge status

- PR: <https://github.com/cleder/brkrs/pull/57>

Note

- On 2025-12-01 the repository CI was updated to run tests single-threaded by setting the `RUST_TEST_THREADS=1` environment variable in `.github/workflows/ci.yaml`. This change was made to avoid intermittent test failures caused by parallel tests touching shared files or relying on environment variables.

Acknowledgements

- Thanks to the team for the spec and review feedback. If you'd like, I can open the PR from this environment given an authenticated `gh` session or `GH_TOKEN`.

---

File created by Copilot assistant on 2025-12-01.
