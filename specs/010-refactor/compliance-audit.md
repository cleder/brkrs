# Compliance Audit: src/ui (Constitution)

**Scope**: `src/ui` (primary), with minimal supporting edits outside allowed when required.

**Rules in scope**: Constitution Section VIII (Bevy 0.17 Mandates & Prohibitions) + other applicable Constitution MUST/NEVER rules (e.g., Section VI rustdoc requirements).

## Findings

### VIII. Bevy 0.17 ECS Architecture Mandates — Fallible Systems

**Rule**: All systems MUST return `Result` and use `?` for error propagation.

**Violations (systems in `src/ui` do not return `Result`)**:

- [src/ui/cheat_indicator.rs](../../src/ui/cheat_indicator.rs): `handle_cheat_indicator`
- [src/ui/fonts.rs](../../src/ui/fonts.rs): `load_ui_fonts` (native + wasm variants), `ensure_ui_fonts_loaded` (wasm + native variants)
- [src/ui/game_over_overlay.rs](../../src/ui/game_over_overlay.rs): `spawn_game_over_overlay`
- [src/ui/level_label.rs](../../src/ui/level_label.rs): `spawn_level_label`, `on_level_started`, `sync_with_current_level`
- [src/ui/lives_counter.rs](../../src/ui/lives_counter.rs): `spawn_lives_counter`, `update_lives_counter`
- [src/ui/palette.rs](../../src/ui/palette.rs): `toggle_palette`, `ensure_palette_ui`, `handle_palette_selection`, `update_palette_selection_feedback`, `update_ghost_preview`, `place_bricks_on_drag`
- [src/ui/pause_overlay.rs](../../src/ui/pause_overlay.rs): `spawn_pause_overlay`, `despawn_pause_overlay`
- [src/ui/score_display.rs](../../src/ui/score_display.rs): `spawn_score_display_system`, `update_score_display_system`

### VIII. Bevy 0.17 ECS Architecture Mandates — Error Recovery Patterns

**Rule**: Use `let Ok(value) = result else { return Ok(()); }` and `let Some(value) = option else { return Ok(()); }` for expected failures.

**Violations (expected query failures handled with early `return;` because systems are not fallible)**:

- [src/ui/lives_counter.rs](../../src/ui/lives_counter.rs): `counter_query.single_mut()` handled via `if let Ok(..)` and falls through without `Result`.
- [src/ui/level_label.rs](../../src/ui/level_label.rs): `query.single_mut()` handled via `if let Ok(..)` and plain `return` paths.
- [src/ui/palette.rs](../../src/ui/palette.rs): `window.single()` / `camera_query.single()` handled via `let Ok(..) else { return; }`.

### VIII. Bevy 0.17 ECS Architecture Mandates — Change Detection

**Rule**: UI update systems MUST ONLY execute when source data changes, not every frame.

**Violations**:

- [src/ui/palette.rs](../../src/ui/palette.rs): `update_palette_selection_feedback` explicitly runs every frame and mutates `BackgroundColor` regardless of `SelectedBrick` changes.

**Potential violations (polling-based UI behavior)**:

- [src/ui/palette.rs](../../src/ui/palette.rs): `update_ghost_preview` polls cursor/window state each frame; consider driving from cursor/mouse events + `Changed<SelectedBrick>` to comply.

### VIII. Bevy 0.17 ECS Architecture Mandates — Asset Handle Reuse

**Rule**: Load assets once in startup systems and store handles in Resources.
NEVER call `asset_server.load()` repeatedly for the same path in spawn systems.

**Violations**:

- [src/ui/cheat_indicator.rs](../../src/ui/cheat_indicator.rs): `handle_cheat_indicator` calls `asset_server.load("textures/default/cheat-mode-128.png")` on activation.

### VIII. Bevy 0.17 3D Graphics Mandates — Mesh3d Components / Required Components

**Rules**:

- Spawn 3D entities with `Mesh3d(handle)` and `MeshMaterial3d(handle)` components.
- Include `Transform` as a required component on entity markers.
- Component marker structs MUST use `#[require(Transform, Visibility)]` when appropriate.

**Violations / risks**:

- [src/ui/palette.rs](../../src/ui/palette.rs): `PreviewViewport` and `GhostPreview` are marker components for 3D entities but do not declare required components; some spawned preview entities also omit `Transform`.

### VI. Comprehensive Rustdoc Documentation

**Rule**: All public modules, functions, and types MUST have rustdoc docs.

**Violations (representative, non-exhaustive list within file)**:

- [src/ui/cheat_indicator.rs](../../src/ui/cheat_indicator.rs): missing module-level `//!` docs; `CheatModeIndicator` and `handle_cheat_indicator` lack rustdoc.
- [src/ui/fonts.rs](../../src/ui/fonts.rs): `UiFonts` and `FontsPlugin` lack rustdoc.
- [src/ui/palette.rs](../../src/ui/palette.rs): multiple public types and systems lack rustdoc (e.g., `PaletteState`, `SelectedBrick`, `PaletteRoot`, `PalettePreview`, `GhostPreview`, `PreviewViewport`, and several `pub fn` systems).

### IV. Performance-First Implementation (supporting)

**Rule**: Game code MUST meet 60 FPS; minimize allocations in hot loops.

**Risks**:

- [src/ui/palette.rs](../../src/ui/palette.rs): `update_ghost_preview` can allocate new `StandardMaterial` handles in the fallback path; if registry is absent, this becomes a per-frame allocation.
