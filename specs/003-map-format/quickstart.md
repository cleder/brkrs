# Quickstart: Map Format Change (Completed Migration to 20×20 Grid)

**Feature**: 003-map-format **Created**: 2025-11-27 **Purpose**: Developer guide for implementing and testing grid dimension changes

## Overview

This feature migrated the game grid from 22×22 to 20×20.
The change is complete and reflected across code, assets, and documentation.
Key updates included:

- Grid constants (GRID_WIDTH, GRID_HEIGHT)
- Level asset files (level_001.ron, level_002.ron)
- WASM embedded level strings
- Level transition timing (spawn bricks before ball physics)

**Implementation Status**: Complete (All user stories + polish)

---

## Prerequisites

- Rust 1.81+ with Bevy 0.16 project set up
- Working knowledge of Bevy ECS (entities, components, systems)
- Basic understanding of RON serialization format
- Familiarity with project structure (see specs/003-map-format/plan.md)

---

## Step 1: Grid Constants (FINAL STATE)

**File**: `src/lib.rs`

**Changes**:

```rust
// Historical (22×22 grid)
// const GRID_WIDTH: usize = 22;
// const GRID_HEIGHT: usize = 22;
// const CELL_WIDTH: f32 = PLANE_W / GRID_WIDTH as f32;
// const CELL_HEIGHT: f32 = PLANE_H / GRID_HEIGHT as f32;

// Current (20×20 grid)
const GRID_WIDTH: usize = 20;  // Columns (Z-axis)
const GRID_HEIGHT: usize = 20; // Rows (X-axis)
const CELL_WIDTH: f32 = PLANE_W / 20.0;   // 2.0
const CELL_HEIGHT: f32 = PLANE_H / 20.0;  // 1.5
```

**Verification**:

```bash
# Check constant usage across codebase
grep -r "GRID_WIDTH\|GRID_HEIGHT\|CELL_WIDTH\|CELL_HEIGHT" src/

# Expected: All code uses these constants (no hardcoded 22 or 1.818 values)
```

**Test**:

```bash
cargo build
# Should compile without errors (constants are derived, no other changes needed yet)
```

---

## Step 2: Level Validation (normalize_matrix)

**File**: `src/level_loader.rs`

**Find** the validation logic (likely in `load_level` or similar function):

```rust
// BEFORE
// expect 22 x 22

// AFTER
// expect 20 x 20
```

**Final Implementation** (simplified warnings + truncation; pads silently):

```rust
fn normalize_matrix(mut matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    const TARGET_ROWS: usize = 20;
    const TARGET_COLS: usize = 20;

    let original_rows = matrix.len();
    let original_cols = matrix.first().map_or(0, |r| r.len());
    if original_rows != TARGET_ROWS || original_cols != TARGET_COLS {
        warn!("Level matrix wrong dimensions; expected 20x20, got {}x{}", original_rows, original_cols);
    }
    while matrix.len() < TARGET_ROWS { matrix.push(vec![0; TARGET_COLS]); }
    if matrix.len() > TARGET_ROWS {
        warn!("Level matrix has {} rows; truncating to {}", matrix.len(), TARGET_ROWS);
        matrix.truncate(TARGET_ROWS);
    }
    for (i, row) in matrix.iter_mut().enumerate() {
        let len_before = row.len();
        while row.len() < TARGET_COLS { row.push(0); }
        if len_before > TARGET_COLS {
            warn!("Row {} has {} columns; truncating to {}", i, len_before, TARGET_COLS);
            row.truncate(TARGET_COLS);
        }
    }
    matrix
}
```

**Integrate** into level loading:

```rust
pub fn load_level(/* ... */) {
    // ... parse LevelDefinition ...
    let normalized_matrix = normalize_matrix(level.matrix);
    level.matrix = normalized_matrix;
    // ... continue with spawning ...
}
```

**Verification**:

```bash
cargo check
# Should compile without errors
```

---

## Step 3: Level Asset Files (Updated)

**Files**: `assets/levels/level_001.ron`, `assets/levels/level_002.ron`

**Process**:

1. **Open** level_001.ron in text editor
2. **Find** the matrix array (currently 22 rows × 22 cols)
3. **Remove** last 2 rows
4. **For each remaining row**: remove last 2 elements
5. **Verify** final matrix is exactly 20 rows × 20 cols
6. **Repeat** for level_002.ron

**Example**:

```ron
# BEFORE (22x22)
matrix: [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // Row 0 (22 cols)
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // Row 1
    // ... 20 more rows
]

# AFTER (20x20)
matrix: [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // Row 0 (20 cols)
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // Row 1
    // ... 18 more rows (total 20 rows)
]
```

**Verification Script**:

```bash
# Check level_001.ron dimensions
python3 << 'EOF'
import re

with open('assets/levels/level_001.ron', 'r') as f:
    content = f.read()
    # Extract matrix section
    matrix_match = re.search(r'matrix:\s*\[(.*?)\]', content, re.DOTALL)
    if matrix_match:
        matrix_str = matrix_match.group(1)
        rows = re.findall(r'\[.*?\]', matrix_str)
        print(f"level_001.ron: {len(rows)} rows")
        for i, row in enumerate(rows):
            cols = len(re.findall(r'\d+', row))
            print(f"  Row {i}: {cols} cols")
EOF
```

**Expected Output**:

```text
level_001.ron: 20 rows
  Row 0: 20 cols
  Row 1: 20 cols
  ...
  Row 19: 20 cols
```

---

## Step 4: WASM Embedded Strings (Auto-Embedded)

**File**: `src/level_loader.rs`

**Find** the `embedded_level_str()` function:

```rust
#[cfg(target_arch = "wasm32")]
fn embedded_level_str(name: &str) -> Option<&'static str> {
    match name {
        "level_001" => Some(include_str!("../assets/levels/level_001.ron")),
        "level_002" => Some(include_str!("../assets/levels/level_002.ron")),
        _ => None,
    }
}
```

**No code changes needed** - `include_str!` will automatically embed the updated RON files at compile time.

**Verification**:

```bash
# Build WASM to verify embedded strings compile
cargo build --target wasm32-unknown-unknown --release
# Should succeed without errors
```

**Test in Browser**:

```bash
# Build and deploy
.github/workflows/deploy-wasm.yml  # or manual wasm-bindgen
# Open wasm/index.html in browser
# Play game and verify levels load correctly
```

---

## Step 5: Level Transition Timing (Final Sequence)

**File**: `src/level_loader.rs`

**Goal**: Ensure bricks spawn before ball physics activate (FR-012 to FR-019)

**Find** the `advance_level_when_cleared` system (or equivalent):

```rust
// Current behavior (WRONG): Ball spawns immediately, bricks spawn later
// Target behavior (CORRECT): Bricks spawn first, then frozen ball, then physics activate
```

**Final Sequence (Implemented)**:

Implementation refactored to guarantee bricks spawn first; ball freezes via `BallFrozen` + `GravityScale(0.0)`; paddle growth (1s) then staged unfreeze and impulse wake-up to ensure gravity engages (see `finalize_level_advance` two-stage logic).

**Key Changes**:

- Bricks spawn immediately when level clears (before timer)
- Ball spawns with `BallFrozen` marker component
- Ball physics disabled until paddle growth completes
- System execution order: `advance_level_when_cleared` → `handle_level_advance_delay` → `finalize_level_advance`

**Verification**:

```bash
cargo check
# Should compile

# Look for system scheduling in Plugin impl
grep -A 20 "impl Plugin for LevelLoaderPlugin" src/level_loader.rs
# Verify systems are registered in correct order with .chain() or explicit ordering
```

---

## Step 6: Testing Summary

### Unit Tests (Suggested – not all included in repo)

**Create** test file `src/level_loader_tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20x20_level_loads() {
        let level_str = r#"
            (
                number: 1,
                gravity: Some((0.0, -15.0, 0.0)),
                matrix: [
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    // ... 18 more rows (total 20)
                ],
            )
        "#;
        let level: LevelDefinition = ron::de::from_str(level_str).unwrap();
        assert_eq!(level.matrix.len(), 20, "Expected 20 rows");
        assert_eq!(level.matrix[0].len(), 20, "Expected 20 cols");
    }

    #[test]
    fn test_padding_undersized_level() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        let normalized = normalize_matrix(matrix);
        assert_eq!(normalized.len(), 20, "Should pad to 20 rows");
        assert_eq!(normalized[0].len(), 20, "Should pad to 20 cols");
        assert_eq!(normalized[0][0], 1, "Should preserve existing data");
        assert_eq!(normalized[0][19], 0, "Should pad with 0");
    }

    #[test]
    fn test_truncation_oversized_level() {
        let mut matrix = vec![vec![1; 25]; 25];  // 25x25 matrix
        let normalized = normalize_matrix(matrix);
        assert_eq!(normalized.len(), 20, "Should truncate to 20 rows");
        assert_eq!(normalized[0].len(), 20, "Should truncate to 20 cols");
    }

    #[test]
    fn test_cell_dimensions() {
        assert_eq!(CELL_WIDTH, 2.0, "CELL_WIDTH should be 2.0 for 20x20 grid");
        assert_eq!(CELL_HEIGHT, 1.5, "CELL_HEIGHT should be 1.5 for 20x20 grid");
    }
}
```

**Run**:

```bash
cargo test
# All tests should pass
```

### Integration Test (Manual)

**Manual Playtest**:

1. **Build** game:

   ```bash
   cargo build --release
   ```

2. **Run** game:

   ```bash
   cargo run --release
   ```

3. **Verify** grid overlay:
   - Press Space to enable wireframe
   - Count grid lines: should be 20 horizontal × 20 vertical
   - Verify entities align with grid cells

4. **Test** level transition:
   - Clear all bricks in level 1
   - Observe transition sequence:
     - [ ] Bricks appear immediately (during fade)
     - [ ] Ball remains stationary (not falling)
     - [ ] Paddle grows from tiny to full size
     - [ ] Ball starts moving AFTER paddle growth
   - Verify no "ball moving on empty field" behavior

5. **Test** both levels:
   - Complete level 1, verify level 2 loads with 20x20 grid
   - Verify entities spawn correctly in both levels

### WASM Test (Completed)

**Build and Deploy**:

```bash
# Build WASM
cargo build --target wasm32-unknown-unknown --release

# Run wasm-bindgen
wasm-bindgen --out-dir wasm --target web \
  target/wasm32-unknown-unknown/release/brkrs.wasm

# Copy assets
cp -r assets wasm/

# Serve locally
cd wasm
python3 -m http.server 8000
```

**Test in Browser**:

1. Open <http://localhost:8000>
2. Verify game loads without errors
3. Play through both levels
4. Verify level transitions work correctly
5. Check browser console for any warnings/errors

---

## Step 7: Success Criteria Verification (All Met)

| Criteria | Status | Notes |
|----------|--------|-------|
| SC-001 | Met | Load times unchanged; fewer entities (400 vs 484). |
| SC-002 | Met | Overlay uses GRID_WIDTH/HEIGHT=20. |
| SC-003 | Met | Positions derive from CELL_WIDTH/HEIGHT; no overlaps. |
| SC-004 | Met | normalize_matrix warns on any mismatch; truncation logged. |
| SC-005 | Met | Grep shows no residual "22x22" in src/. |
| SC-006 | Met | WASM build loads levels; user confirmed. |
| SC-007 | Met | Visual feel preserved (aspect ratio & spacing). |
| SC-008 | Met | Transition sequence ensures bricks before physics. |
| SC-009 | Met | Ball frozen (GravityScale 0, BallFrozen) until growth done. |
| SC-010 | Met | 1s delay + 1s growth = 2s total. |

### Original Checklist (Historical Reference)

Use this checklist to verify all requirements are met:

#### Grid Dimension Requirements

- [ ] SC-001: Levels load with same speed as before (no performance regression)
- [ ] SC-002: Grid overlay displays exactly 20 lines per direction
- [ ] SC-003: Entities maintain proper spacing (no overlaps or out-of-bounds)
- [ ] SC-004: Dimension validation detects all mismatches (test with various wrong sizes)
- [ ] SC-005: No "22" or "22x22" references in code/docs (grep verification)

#### WASM Requirements

- [ ] SC-006: WASM builds load embedded 20x20 levels successfully
- [ ] FR-008: Embedded level strings updated (verified by WASM build test)

#### Visual/Gameplay Requirements

- [ ] SC-007: Game maintains same visual feel (playtest comparison)
- [ ] FR-020: Bricks use exact mathematical cell size (1.35 × 1.8 units)

#### Level Transition Requirements

- [ ] SC-008: Bricks visible before ball physics (0% empty-field motion instances)
- [ ] SC-009: Ball stationary during paddle growth (zero velocity)
- [ ] SC-010: Transition completes within 2 seconds (stopwatch test)
- [ ] FR-012: Bricks spawn before ball physics activate
- [ ] FR-013: Ball frozen during paddle growth
- [ ] FR-014: Physics activate only after growth completes
- [ ] FR-015: Transition sequence correct (bricks → frozen ball+tiny paddle → growth → physics)
- [ ] FR-016: Bricks visible before/simultaneously with paddle/ball

#### Error Handling Requirements

- [ ] FR-001: Validation checks for 20x20 dimensions
- [ ] FR-002: Warning logged for wrong dimensions
- [ ] FR-003: Actual dimensions included in error messages
- [ ] FR-024: Undersized levels padded with 0
- [ ] FR-025: Oversized levels truncated with warning
- [ ] FR-026: Malformed levels load (don't reject entirely)

---

## Troubleshooting (Retained for Reference)

### Issue: Tests Fail After Constant Change

**Symptom**: Unit tests fail with "expected 22, got 20" errors

**Solution**: Update test expectations to use 20x20 dimensions

```bash
# Find all test files referencing grid dimensions
grep -r "22" tests/
# Update test assertions to expect 20
```

### Issue: Entities Spawn Outside Play Area

**Symptom**: Bricks or paddle appear off-screen after grid change

**Solution**: Verify position calculation uses CELL_WIDTH and CELL_HEIGHT (not hardcoded values)

```rust
// WRONG
let x = -15.0 + (row as f32 * 1.364);  // Hardcoded old cell height

// CORRECT
let x = -PLANE_H / 2.0 + (row as f32 * CELL_HEIGHT) + CELL_HEIGHT / 2.0;
```

### Issue: Ball Moves Before Bricks Appear

**Symptom**: Level transition shows ball falling before bricks spawn

**Solution**: Check system execution order - bricks must spawn before ball

```rust
// Verify in Plugin impl
app.add_systems(Update, (
    advance_level_when_cleared,  // Spawns bricks immediately
    handle_level_advance_delay,  // Spawns frozen ball after timer
    finalize_level_advance,      // Activates physics after growth
).chain());  // <-- .chain() ensures sequential execution
```

### Issue: Grid Overlay Shows Wrong Line Count

**Symptom**: Wireframe shows more/fewer than 20 lines

**Solution**: Verify grid_debug.rs uses GRID_WIDTH and GRID_HEIGHT constants

```rust
// Should spawn GRID_WIDTH vertical lines and GRID_HEIGHT horizontal lines
for i in 0..GRID_WIDTH {
    // spawn vertical line
}
for j in 0..GRID_HEIGHT {
    // spawn horizontal line
}
```

### Issue: WASM Build Fails

**Symptom**: `cargo build --target wasm32-unknown-unknown` errors

**Solution**: Check for platform-specific code not gated by `#[cfg]`

```bash
# Look for filesystem operations in level_loader.rs
grep -n "std::fs" src/level_loader.rs
# Should only appear in #[cfg(not(target_arch = "wasm32"))] blocks
```

### Issue: Level Dimensions Warning Floods Logs

**Symptom**: Console spam with "Expected 20x20, got ..." messages

**Solution**: Update level files to correct format (Step 3)

```bash
# Verify RON files have correct dimensions
python3 << 'EOF'
import glob
for filepath in glob.glob('assets/levels/*.ron'):
    with open(filepath) as f:
        content = f.read()
        rows = content.count('[', content.find('matrix:'), content.find(']', content.find('matrix:')+50))
        print(f"{filepath}: ~{rows} rows (should be 20)")
EOF
```

---

## Performance Validation (Optional)

### Benchmark Level Loading

```bash
# Add timing to load_level function (temporary)
let start = std::time::Instant::now();
// ... load level ...
println!("Level loaded in {:?}", start.elapsed());

# Compare before/after
cargo run --release > load_times_before.txt  # With 22x22
# (make changes)
cargo run --release > load_times_after.txt   # With 20x20

# Should be similar or faster (fewer entities)
```

### Frame Rate Test

```bash
# Enable FPS counter (if available) or add custom logging
# Play game and monitor FPS
# Should maintain 60 FPS with 20x20 grid (fewer entities = better performance)
```

---

## Cleanup (Completed)

### Remove Old References

```bash
# Search for "22" in code (excluding version numbers, dates, etc.)
grep -rn "22" src/ --include="*.rs" | grep -v "2022\|0.22"

# Common places to check:
# - Comments mentioning grid dimensions
# - Documentation strings
# - Error messages
# - Test fixtures
```

### Update Documentation

**Files to update**:

- README.md (if it mentions grid size)
- specs/003-map-format/spec.md (mark as implemented)
- Any design documents or architecture notes

---

## Next Steps (Post-Migration)

After completing this feature:

1. **Git Commit**:

   ```bash
   git add .
   git commit -m "feat: change grid from 22x22 to 20x20

   - Update GRID_WIDTH and GRID_HEIGHT constants to 20
   - Modify level_001.ron and level_002.ron to 20x20 format
   - Add padding/truncation for malformed levels
   - Fix level transition to spawn bricks before ball physics
   - Update WASM embedded strings
   - Add BallFrozen marker for level transitions
   - Verify grid overlay shows 20x20 lines"
   ```

2. **Create Pull Request** (if using GitHub):

   ```bash
   git push origin 003-map-format
   # Create PR with specs/003-map-format/spec.md as description
   ```

3. **Request Code Review**:
   - Share PR with team
   - Highlight level transition timing changes (critical behavioral change)
   - Request playtest from QA

4. **Deploy**:
   - Merge to main after approval
   - CI/CD will automatically deploy WASM to GitHub Pages
   - Verify production deployment works

---

## Rollback Plan (Low Risk)

If issues arise in production:

1. **Revert Constants**:

   ```bash
   git revert <commit-hash>
   # Or manually change constants back to 22
   ```

2. **Restore Level Files**:

   ```bash
   git checkout HEAD~1 assets/levels/*.ron
   ```

3. **Rebuild and Deploy**:

   ```bash
   cargo build --release
   # Re-deploy WASM
   ```

**Note**: Grid dimension changes are backward-incompatible for level files.
Keep backups of 22x22 level files if needed.

---

## References

- **Feature Spec**: `specs/003-map-format/spec.md` (requirements and user stories)
- **Data Model**: `specs/003-map-format/data-model.md` (entity structures)
- **Research**: `specs/003-map-format/research.md` (technical decisions)
- **Implementation Plan**: `specs/003-map-format/plan.md` (architecture overview)

---

## Support

If you encounter issues not covered here:

1. Check `specs/003-map-format/research.md` for technical decisions
2. Review `specs/003-map-format/data-model.md` for entity relationships
3. Search project issues on GitHub
4. Ask in team chat/forum

**Happy coding! 🎮**
