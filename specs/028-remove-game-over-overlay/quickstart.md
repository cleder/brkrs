# Quickstart: 028 Remove Game-Over Overlay

## Goal

Remove the legacy gameplay game-over overlay and preserve existing non-overlay game-over handling.

## Prerequisites

- Rust toolchain for this repo (`cargo` available)
- Branch: `028-remove-game-over-overlay`
- TDD workflow: add failing tests first, then implementation

## Red/Green Proof Workflow

1. Write feature tests first and run targeted test command to capture a red failure.
2. Commit failing tests with a message tagged `red` and record the commit hash in `tasks.md` placeholders.
3. Obtain requestor approval for red tests before implementation.
4. Implement minimal production changes until tests pass (`green`).
5. Commit green implementation and record the validation commands used.

## Implementation Steps

1. Add/adjust tests first (red phase).

- Create or update integration tests for restart regression path (`lose all lives -> start new game -> no legacy overlay`).
- Add multi-frame assertion (`>=10` `app.update()` calls) to ensure overlay never reappears.
- Update tests that currently expect legacy overlay behavior.

2. Remove legacy overlay wiring.

- Remove `src/ui/game_over_overlay.rs` usage from `src/ui/mod.rs`.
- Remove `game_over_overlay::spawn_game_over_overlay` from UI update system registration.
- Remove `pub mod game_over_overlay;` export.

3. Decouple dependent modules.

- Update `src/ui/pause_overlay.rs` to remove `GameOverOverlay` query dependency.
- Update `src/systems/cheat_mode.rs` to remove overlay-despawn coupling.
- Keep lives reset behavior if still required by cheat-mode spec.

4. Preserve canonical game-over flow.

- Do not modify `GameState::GameOver` state wiring unless required by compile/test updates.
- Do not introduce replacement gameplay overlay.

5. Run validation.

- `cargo test`
- `cargo fmt --all`
- `cargo clippy --all-targets --all-features`
- `bevy lint`
- `cargo check --target wasm32-unknown-unknown`

## Expected Outcomes

- Restart flow shows active gameplay without legacy overlay artifacts.
- Repeated game-over/restart cycles do not spawn legacy overlay entities.
- Existing non-overlay game-over behavior remains intact.

## Execution Notes (2026-03-10)

- US1 red phase captured with failing assertions in `tests/game_over_overlay_removal.rs` for legacy overlay absence, then turned green after implementation.
- US2 red phase captured in `tests/ui_overlays.rs` and `tests/cheat_mode.rs` with legacy-coupled expectations, then turned green after compatibility updates.
- Requestor approval checkpoint recorded via `continue` instruction before proceeding with iterative implementation.
- Failing-test commit hashes are not available in this uncommitted workspace session; maintainers should add hash references when creating red/green commits.
- Validation results captured in this session:
  - `cargo test`: pass (test suite completed successfully).
  - `cargo fmt --all -- --check`: pass (no formatting diffs).
  - `cargo clippy --all-targets --all-features -- -D warnings`: fail due to existing repository-wide lint findings outside this feature scope:
    - `tests/gravity_bricks.rs`: `clippy::empty_line_after_doc_comments`.
    - `tests/ball_spawn_bricks.rs`: `clippy::useless_conversion`.
  - `bevy lint`: pass (`BEVY_LINT_EXIT:0` in serialized validation run).
  - `cargo check --target wasm32-unknown-unknown`: pass (`WASM_CHECK_EXIT:0` in serialized validation run).

### Follow-up (Out of Scope)

- Create a separate cleanup task/PR to resolve the existing repository-wide clippy findings in `tests/gravity_bricks.rs` and `tests/ball_spawn_bricks.rs`.
