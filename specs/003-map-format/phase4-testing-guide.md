# Phase 4 Testing Guide: Grid Overlay Verification

## Objective

Verify that the grid debug overlay correctly displays a 20x20 grid after the dimension change from 22x22 to 20x20.

## Prerequisites

- ✅ Phase 3 complete (grid constants updated to 20x20)
- ✅ Grid debug comments updated
- ✅ Compilation verified (`cargo check`)

## Test Environment

- **Platform**: Linux (native build)
- **Input**: Space bar to toggle wireframe/grid overlay
- **Note**: Grid overlay only works on native builds (not WASM)

## Testing Procedure

### T028: Verify 20 Vertical Lines

**Steps:**

1. Run the game: `cargo run`
2. Wait for the game to load (level 1 should appear)
3. Press **Space** to enable wireframe mode
4. Observe the grid overlay appears
5. **Count vertical lines** (lines running top-to-bottom along the Y-axis)
   - Should see 21 lines total (0 to 20 inclusive creates 21 dividing lines)
   - These lines are spaced by CELL_WIDTH (40.0 / 20 = 2.0 units)

**Expected Result:**

- ✅ 21 vertical lines visible (creating 20 columns)
- ✅ Lines evenly spaced across the width of the play area
- ✅ Lines span the full height of the play area

**Actual Result:**

- [ ] PASS / [ ] FAIL
- Notes: **_****_****_**

---

### T029: Verify 20 Horizontal Lines

**Steps:**

1. With wireframe still enabled from T028
2. **Count horizontal lines** (lines running left-to-right along the Z-axis)
   - Should see 21 lines total (0 to 20 inclusive creates 21 dividing lines)
   - These lines are spaced by CELL_HEIGHT (30.0 / 20 = 1.5 units)

**Expected Result:**

- ✅ 21 horizontal lines visible (creating 20 rows)
- ✅ Lines evenly spaced across the height of the play area
- ✅ Lines span the full width of the play area

**Actual Result:**

- [ ] PASS / [ ] FAIL
- Notes: **_****_****_**

---

### T030: Verify Entity Alignment

**Steps:**

1. With wireframe still enabled
2. Observe the **paddle** entity (bottom of screen)
   - Should be centered in a grid cell
   - Should not overlap multiple cells
3. Observe the **ball** entity (center area)
   - Should be centered in a grid cell
   - Should not overlap multiple cells
4. Observe the **brick** entities (top rows)
   - Each brick should align with one grid cell
   - Bricks should not overlap cell boundaries
5. Move the paddle left/right using arrow keys
   - Paddle should smoothly move through cells
   - Grid alignment should remain consistent

**Expected Result:**

- ✅ Paddle centered in grid cell
- ✅ Ball centered in grid cell
- ✅ Bricks aligned with grid cells
- ✅ No entity overlap across cell boundaries
- ✅ Entities positioned at cell centers (not edges)

**Actual Result:**

- [ ] PASS / [ ] FAIL
- Notes: **_****_****_**

---

## Technical Details

### Grid Overlay Implementation

- **Source**: `src/systems/grid_debug.rs`
- **Vertical lines**: `for i in 0..=GRID_WIDTH` → 0 to 20 (21 lines)
- **Horizontal lines**: `for i in 0..=GRID_HEIGHT` → 0 to 20 (21 lines)
- **Grid dimensions**: PLANE_H = 30.0, PLANE_W = 40.0
- **Cell size**: CELL_HEIGHT = 1.5, CELL_WIDTH = 2.0

### Calculations

```text
GRID_WIDTH = 20
GRID_HEIGHT = 20
PLANE_H = 30.0
PLANE_W = 40.0

CELL_HEIGHT = PLANE_H / GRID_HEIGHT = 30.0 / 20 = 1.5
CELL_WIDTH = PLANE_W / GRID_WIDTH = 40.0 / 20 = 2.0

Vertical lines: 0..=20 → 21 lines (creating 20 columns)
Horizontal lines: 0..=20 → 21 lines (creating 20 rows)
```

### Wireframe Toggle

- **Key**: Space bar
- **Function**: Toggles `WireframeConfig.global`
- **Effect**: Shows/hides grid overlay and entity wireframes
- **Platform**: Native only (not available on WASM)

---

## Troubleshooting

### Grid Not Visible

- **Symptom**: Pressing Space doesn't show grid
- **Solution**: Verify running native build (not WASM)
- **Check**: `WireframePlugin` enabled in `src/lib.rs`

### Wrong Number of Lines

- **Symptom**: More or fewer than 21 lines per axis
- **Solution**: Verify GRID_WIDTH and GRID_HEIGHT are both 20
- **Check**: Constants in `src/lib.rs`

### Entity Misalignment

- **Symptom**: Entities not centered in cells
- **Solution**: Verify spawn position calculations use CELL_WIDTH/CELL_HEIGHT
- **Check**: `src/level_loader.rs` spawn logic

---

## Completion Criteria

Phase 4 is complete when:

- ✅ T028: 21 vertical lines counted (creating 20 columns)
- ✅ T029: 21 horizontal lines counted (creating 20 rows)
- ✅ T030: All entities properly aligned with grid cells

After completion, update `specs/003-map-format/tasks.md`:

- Mark T028, T029, T030 as `[x]`
- Add checkpoint note: "Phase 4 complete: Grid overlay verified as 20x20"
