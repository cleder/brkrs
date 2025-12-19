# Refactoring Plan: src/ui Constitution Compliance

This plan is designed to be executed as a sequence of small, reviewable changes.
Tasks are grouped so that small mechanical fixes are batched, while behavior changes are isolated.

## Task 1 — Add “tests-first” harness for UI compliance

**Fixes rules**:

- Constitution VII.
  Test-Driven Development (TDD-First)

**Work**:

- Add one new test per high-risk refactor area (palette, cheat indicator, overlays) that initially fails on current behavior (red commit).
- Ensure each subsequent task has a corresponding passing test update or new test.

**Notes**:

- Prefer integration tests in `tests/` that run minimal apps and validate behavior with ECS state.

## Task 2 — Convert UI systems to Fallible Systems (`Result`)

**Fixes rules**:

- Constitution VIII.
  Bevy 0.17 ECS Architecture Mandates → Fallible Systems
- Constitution VIII.
  Bevy 0.17 ECS Architecture Mandates → Error Recovery Patterns

**Work**:

- Update every Bevy system function under `src/ui` to return `Result<(), UiSystemError>` (or a shared error type), including systems currently returning `()`.
- Replace early `return;` paths for expected failures with `return Ok(());`.
- Ensure every system uses `?` where it currently uses `if let Ok(...)` / `let Ok(...) else { return; }` patterns.

**Targets**:

- [src/ui/cheat_indicator.rs](../../src/ui/cheat_indicator.rs)
- [src/ui/fonts.rs](../../src/ui/fonts.rs)
- [src/ui/game_over_overlay.rs](../../src/ui/game_over_overlay.rs)
- [src/ui/level_label.rs](../../src/ui/level_label.rs)
- [src/ui/lives_counter.rs](../../src/ui/lives_counter.rs)
- [src/ui/palette.rs](../../src/ui/palette.rs)
- [src/ui/pause_overlay.rs](../../src/ui/pause_overlay.rs)
- [src/ui/score_display.rs](../../src/ui/score_display.rs)

## Task 3 — Standardize “single/single_mut” query usage

**Fixes rules**:

- Constitution VIII.
  Bevy 0.17 ECS Architecture Mandates → Fallible Systems
- Constitution VIII.
  Bevy 0.17 ECS Prohibitions → NO Panicking Queries

**Work**:

- Replace `query.single()` / `query.single_mut()` call sites with `?` propagation (or with `let Ok(..) = .. else { return Ok(()); }` when an empty/multiple match is expected and not exceptional).
- Ensure the error paths are consistent and do not panic.

**Targets**:

- [src/ui/lives_counter.rs](../../src/ui/lives_counter.rs)
- [src/ui/level_label.rs](../../src/ui/level_label.rs)
- [src/ui/palette.rs](../../src/ui/palette.rs)

## Task 4 — Make palette selection feedback change-driven (no per-frame UI updates)

**Fixes rules**:

- Constitution VIII.
  Bevy 0.17 ECS Architecture Mandates → Change Detection

**Work**:

- Refactor `update_palette_selection_feedback` to only run when its inputs change.
- Preferred approach:
  - Add `Changed<SelectedBrick>` to trigger updates, and
  - Also handle the “palette just spawned” case via `Added<PalettePreview>` or a one-shot system when opening the palette.
- Update or extend [tests/editor_palette.rs](../../tests/editor_palette.rs) to prove that the system does not perform work when neither selection nor previews changed.

**Targets**:

- [src/ui/palette.rs](../../src/ui/palette.rs)
- [tests/editor_palette.rs](../../tests/editor_palette.rs)

## Task 5 — Make ghost preview + drag placement event-driven (or explicitly change-detected)

**Fixes rules**:

- Constitution VIII.
  Bevy 0.17 ECS Architecture Mandates → Change Detection
- Constitution IV.
  Performance-First Implementation (supporting: reduce per-frame work)

**Work**:

- Replace cursor polling in `update_ghost_preview` with an event-driven flow using cursor/mouse events (e.g., `CursorMoved` + button state) and selection changes.
- Ensure the fallback material path is cached (no per-frame `materials.add(...)`).

**Targets**:

- [src/ui/palette.rs](../../src/ui/palette.rs)

## Task 6 — Fix asset handle reuse for cheat indicator

**Fixes rules**:

- Constitution VIII.
  Bevy 0.17 ECS Architecture Mandates → Asset Handle Reuse

**Work**:

- Introduce a small resource (e.g., `CheatIndicatorAssets`) that loads `textures/default/cheat-mode-128.png` once (startup) and stores the handle.
- Update `handle_cheat_indicator` to use the cached handle instead of calling `asset_server.load` during toggles.
- Add a new test ensuring multiple toggles do not cause repeated loads/spawns.

**Targets**:

- [src/ui/cheat_indicator.rs](../../src/ui/cheat_indicator.rs)

## Task 7 — Add required components on 3D marker components

**Fixes rules**:

- Constitution VIII.
  Bevy 0.17 ECS Architecture Mandates → Required Components
- Constitution VIII.
  Bevy 0.17 3D Graphics Mandates → Mesh3d Components

**Work**:

- Add `#[require(Transform, Visibility)]` to marker components that represent 3D entities (e.g., `GhostPreview`, `PreviewViewport`).
- Adjust spawns to rely on required components (or at minimum ensure `Transform` is always present).

**Targets**:

- [src/ui/palette.rs](../../src/ui/palette.rs)

## Task 8 — Fill rustdoc gaps for public UI APIs

**Fixes rules**:

- Constitution VI.
  Comprehensive Rustdoc Documentation

**Work**:

- Add module-level `//!` docs where missing.
- Add rustdoc `///` to all public types and system functions in `src/ui`, focusing on purpose/when-to-use.

**Targets**:

- [src/ui/cheat_indicator.rs](../../src/ui/cheat_indicator.rs)
- [src/ui/fonts.rs](../../src/ui/fonts.rs)
- [src/ui/palette.rs](../../src/ui/palette.rs)

## Task 9 — Optional: consolidate UI registration into a single plugin

**Fixes rules**:

- Constitution VIII.
  Bevy 0.17 ECS Architecture Mandates → Plugin-Based Architecture

**Work**:

- Create a `UiPlugin` in [src/ui/mod.rs](../../src/ui/mod.rs) that registers all UI systems/resources in one place.
- Keep minimal supporting edits outside `src/ui` to switch the app to use `UiPlugin` (as allowed by spec clarifications).

**Notes**:

- Do this after Task 2 so the plugin registers already-compliant systems.
