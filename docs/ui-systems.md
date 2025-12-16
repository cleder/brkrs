# UI Systems Guide

This guide explains the three main UI systems in Brkrs and how they interact.

## Overview

The UI is split into four independent but coordinated systems:

1. **Score Display** — Shows cumulative score in the top-right corner.
2. **Lives Counter** — Tracks remaining lives below the score display.
3. **Game-Over Overlay** — Displays a "Game over" message when the player runs out of lives.
4. **Designer Palette** — Allows designers to select and place bricks on the grid during gameplay (developer feature).

## Score Display

**Module**: `src/ui/score_display.rs`

**Purpose**: Display the player's cumulative score as a HUD element.

**How it works**:

- `spawn_score_display_system()` runs every Update and creates the display entity once (idempotent) when:
  - No `ScoreDisplayUi` entity exists, and
  - The `UiFonts` resource is available (desktop loads at Startup; WASM provides it once assets are ready).
- `update_score_display_system()` updates the text whenever `ScoreState` changes, using Bevy's change detection to avoid unnecessary updates.

**Spawn location**: Top-right corner (`Node` with `right: Val::Px(12.0)`, `top: Val::Px(40.0)`), positioned below the lives counter to avoid overlap.

**Score mechanics**:

- Points awarded based on brick type (see `docs/bricks.md`)
- Score persists across level transitions
- Every 5000 points triggers a milestone bonus (extra life)
- Special cases: Question brick (53) awards random 25-300 points; Extra Ball (41) and Magnet bricks (55-56) award 0 points

**Dependency**: Requires `UiFonts` resource and `ScoreState` resource.
If missing, the system logs a warning and defers spawning until fonts become available.

## Lives Counter

**Module**: `src/ui/lives_counter.rs`

**Purpose**: Display the player's remaining lives as a small HUD element.

**How it works**:

- `spawn_lives_counter()` runs every Update and creates the counter entity once (idempotent) when:
  - No `LivesCounter` entity exists, and
  - The `UiFonts` resource is available (desktop loads at Startup; WASM provides it once assets are ready).
- `update_lives_counter()` updates the text whenever `LivesState` changes and is scheduled after `RespawnSystems::Schedule` to reflect the latest respawn logic.

**Spawn location**: Top-right corner (`Node` with `right: Val::Px(12.0)`, `top: Val::Px(12.0)`).

**Dependency**: Requires `UiFonts` resource.
If missing (WASM startup), the system logs a warning and defers spawning until fonts become available.

## Game-Over Overlay

**Module**: `src/ui/game_over_overlay.rs`

**Purpose**: Display a centered "Game over" message when the player exhausts all lives.

**How it works**:

- `spawn_game_over_overlay()` listens for the `GameOverRequested` event and spawns the overlay once when:
  - The event is received, and
  - `LivesState.lives_remaining == 0`, and
  - No existing `GameOverOverlay` entity is present (idempotent), and
  - `UiFonts` is available (logs a warning and defers otherwise).
- Scheduled in Update after `RespawnSystems::Schedule` to ensure lives logic has finalized.

**Display**: Centered full-screen text (80pt Orbitron, white) covering the entire viewport.

**Coexistence with Lives Counter**: Both can exist simultaneously.
The overlay becomes the primary focus, and the counter remains visible in the background.
Future logic can hide the counter if desired.

## Designer Palette

**Module**: `src/ui/palette.rs`

**Purpose**: Provide an in-game tool for designers to select and place bricks on the grid.
This is a developer feature and can be disabled or hidden in production builds.

**User flow**:

1. Press **P** to toggle the palette open/closed.
2. When open, the palette displays a list of available brick types (e.g., Simple Brick type 20, Indestructible type 90) with small color previews.
3. Click a preview to select that brick type.
4. A "ghost" preview follows the cursor over the grid when a type is selected.
5. Hold the left mouse button and drag over grid cells to place bricks at those locations.

**Systems**:

- `toggle_palette()` — Listens for **P** keypress and toggles `PaletteState::open`.
- `ensure_palette_ui()` — Spawns/despawns the UI panel when `PaletteState` changes.
  Previews show material colors resolved from `TypeVariantRegistry` when available.
- `handle_palette_selection()` — Updates `SelectedBrick::type_id` when a preview button is clicked.
- `update_palette_selection_feedback()` — Highlights the selected preview with a bright yellow background.
- `update_ghost_preview()` — Spawns/positions a semi-transparent preview cube that follows the cursor over valid grid cells.
- `place_bricks_on_drag()` — Spawns actual brick entities on the grid when the mouse is held and dragged.
  Prevents duplicate placement at the same cell.

**Grid integration**: Uses camera raycasting to convert cursor positions to world coordinates and then to grid indices (0..GRID_HEIGHT × 0..GRID_WIDTH).

**Material integration**: When `TypeVariantRegistry` is available (loaded by `TextureManifestPlugin`), previews show the actual brick material colors.
Falls back to gray if unavailable.

## Resource Dependencies

All three systems depend on platform-specific font availability:

- **Desktop**: `UiFonts` is inserted at Startup by `load_ui_fonts()` in `FontsPlugin`, so all systems can spawn immediately.
- **WASM**: `UiFonts` is inserted asynchronously in Update by `ensure_ui_fonts_loaded()` in `FontsPlugin`.
  UI systems check for the resource and log warnings if it's missing during early frames; they will successfully spawn once fonts are ready.

## Scheduling Summary

All UI systems run in the **Update** schedule:

| System | Condition | Order |
|--------|-----------|-------|
| `spawn_lives_counter` | Every frame, idempotent | Before `update_lives_counter` |
| `update_lives_counter` | Only if `LivesState` changed | After `RespawnSystems::Schedule` |
| `spawn_game_over_overlay` | Only if `GameOverRequested` event received | After `RespawnSystems::Schedule` |
| `toggle_palette` | Every frame | Early (no explicit ordering) |
| `ensure_palette_ui` | Only if `PaletteState` changed | After `toggle_palette` |
| `handle_palette_selection` | Only if button interaction changed | During standard interaction phase |
| `update_palette_selection_feedback` | Every frame (previews may spawn dynamically) | After `handle_palette_selection` |
| `update_ghost_preview` | Every frame | Late (cursor tracking) |
| `place_bricks_on_drag` | Every frame if mouse held | Late (after ghost preview) |

## Adding New UI Systems

When adding a new UI system:

1. **Create a module** under `src/ui/` (e.g., `new_feature.rs`).
2. **Add module docs** explaining:
   - Purpose of the UI.
   - When/how it spawns (idempotent or event-driven?).
   - Dependency on `UiFonts` or other resources.
   - Scheduling relative to other systems.
3. **Use `Option<Res<UiFonts>>`** in function signatures to gracefully handle missing fonts on WASM.
4. **Register systems** in `src/lib.rs` under `pub fn run()`, respecting scheduling dependencies.
5. **Add unit tests** in the same module if applicable (see `tests/` directory).

## Common Patterns

### Idempotent Spawn

Query for an existing entity before spawning:

```rust
let existing: Query<Entity, With<MyUiMarker>> = ...;
if !existing.is_empty() {
    return; // Already spawned
}
```

### Handle Missing Fonts

Use `Option<Res<UiFonts>>` and provide a fallback:

```rust
let font = ui_fonts
    .as_ref()
    .map(|f| f.orbitron.clone())
    .unwrap_or_default();
```

### Cursor-to-Grid Conversion

See `palette.rs::cursor_to_grid()` for the full implementation using camera raycasting.

## Testing UI Systems

Unit tests for UI systems are in `tests/` and focus on:

- Spawning behavior (idempotent, resource-dependent).
- State transitions and event handling.
- Material/preview resolution when registries are available.

Example:

```bash
cargo test --lib ui::lives_counter
```

See `tests/` for full test suites and helper utilities.
