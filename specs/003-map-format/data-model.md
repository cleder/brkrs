# Data Model: Map Format Change (22x22 to 20x20)

**Feature**: 003-map-format **Created**: 2025-11-27 **Purpose**: Document entities and data structures affected by grid dimension changes

## Core Entities

### LevelDefinition

**Purpose**: Represents a complete level configuration loaded from RON asset files

**Location**: `src/level_loader.rs`

**Structure**:

```rust
#[derive(Deserialize, Debug, Clone)]
pub struct LevelDefinition {
    pub number: u32,
    pub gravity: Option<(f32, f32, f32)>,  // Optional gravity override (x,y,z)
    pub matrix: Vec<Vec<u8>>,              // 2D grid: CURRENT 22x22 → TARGET 20x20
    #[cfg(feature = "texture_manifest")]
    #[serde(default)]
    pub presentation: Option<LevelTextureSet>,
}
```

**Key Properties**:

- `matrix`: 2D array where `matrix[row][col]` encodes entity type at that grid position
- `matrix` dimensions: **CHANGING from 22 rows × 22 cols → 20 rows × 20 cols**
- Each cell value is `u8`: 0 = empty, 1 = paddle, 2 = ball, 3+ = brick types

**Validation**:

- **Current**: Expects exactly 22 rows, each with exactly 22 columns
- **Target**: Must expect exactly 20 rows, each with exactly 20 columns
- **Error Handling** (per FR-024, FR-025, FR-026):
  - If fewer than 20 rows/cols: pad with 0 (empty)
  - If more than 20 rows/cols: truncate and log warning
  - Attempt to load malformed levels rather than rejecting

**Asset Files**:

- `assets/levels/level_001.ron`
- `assets/levels/level_002.ron`
- Both files currently have 22x22 matrices, must be updated to 20x20

**WASM Embedding**:

- Embedded in `src/level_loader.rs` via `include_str!` macro
- `embedded_level_str()` function maps level names to compile-time strings
- Must update embedded strings when RON files change

---

### Grid Constants

**Purpose**: Define the play area dimensions and cell sizing

**Location**: `src/lib.rs`

**Current Values**:

```rust
const PLANE_H: f32 = 30.0;           // Play area height (X-axis)
const PLANE_W: f32 = 40.0;           // Play area width (Z-axis)
const GRID_WIDTH: usize = 22;        // Columns (Z-axis) → CHANGE TO 20
const GRID_HEIGHT: usize = 22;       // Rows (X-axis) → CHANGE TO 20
const CELL_WIDTH: f32 = PLANE_W / GRID_WIDTH as f32;   // ~1.818 → 2.0
const CELL_HEIGHT: f32 = PLANE_H / GRID_HEIGHT as f32; // ~1.364 → 1.5
```

**Target Values**:

```rust
const GRID_WIDTH: usize = 20;        // Columns (Z-axis)
const GRID_HEIGHT: usize = 20;       // Rows (X-axis)
const CELL_WIDTH: f32 = PLANE_W / 20.0;   // 2.0
const CELL_HEIGHT: f32 = PLANE_H / 20.0;  // 1.5
```

**Derivations**:

- Cell dimensions are **derived** from PLANE dimensions and GRID dimensions
- All code should use `CELL_WIDTH` and `CELL_HEIGHT` rather than hardcoding values
- Cell aspect ratio: CELL_HEIGHT / CELL_WIDTH = 30/40 × 20/20 = 0.75 (unchanged)

**Impact**:

- Larger cells: 1.5 × 2.0 vs previous 1.364 × 1.818
- Fewer total cells: 400 (20×20) vs 484 (22×22)
- Same play area coverage (30 × 40 units)

---

### Grid Cell (Conceptual)

**Purpose**: Represents a single cell position in the 20x20 grid

**Properties**:

- **Position** (row, col): Integers from 0 to 19 (inclusive)
- **World Coordinates**: Calculated from grid position
  - X = `-PLANE_H/2 + row * CELL_HEIGHT + CELL_HEIGHT/2`
  - Z = `-PLANE_W/2 + col * CELL_WIDTH + CELL_WIDTH/2`
- **Dimensions**: CELL_HEIGHT (1.5) × CELL_WIDTH (2.0)

**Usage**:

- `src/level_loader.rs`: Converts grid coordinates to world positions for entity spawning
- `src/systems/grid_debug.rs`: Renders grid lines at cell boundaries

**Coordinate System**:

- Origin (0,0,0) at center of play area
- X-axis: -15.0 to +15.0 (PLANE_H = 30)
- Z-axis: -20.0 to +20.0 (PLANE_W = 40)
- Row 0, Col 0 → world position (-13.5, 0.0, -19.0) [top-left corner]
- Row 19, Col 19 → world position (+13.5, 0.0, +19.0) [bottom-right corner]

---

### Paddle

**Purpose**: Player-controlled entity that deflects the ball

**Location**: `src/lib.rs` (marker component)

**Structure**:

```rust
#[derive(Component)]
pub struct Paddle;
```

**Spawning**:

- Spawned from level matrix (cell value = 1)
- Position calculated using grid coordinates
- Dimensions: PADDLE_RADIUS × PADDLE_HEIGHT (independent of grid size)

**Growth Animation**:

```rust
#[derive(Component)]
pub struct PaddleGrowing {
    pub timer: Timer,           // Duration: 1.0 second (unchanged)
    pub target_scale: Vec3,     // Target scale after growth
}
```

**Level Transition Behavior** (NEW - per FR-013, FR-015):

- Spawns as tiny paddle during level transition
- Grows to full size over 1 second
- Ball remains frozen until growth completes

---

### Ball

**Purpose**: Physics-driven entity that collides with bricks and paddle

**Location**: `src/lib.rs` (marker component)

**Structure**:

```rust
#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct BallFrozen;  // Marker: ball should not move

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BallTypeId(pub u8);  // Type variant for texture system
```

**Spawning**:

- Spawned from level matrix (cell value = 2)
- Position calculated using grid coordinates
- Dimensions: BALL_RADIUS (independent of grid size)

**Level Transition Behavior** (NEW - per FR-013, FR-014, FR-15):

- Spawns with `BallFrozen` marker component during level transition
- Physics disabled (GravityScale = 0, Velocity = 0) while frozen
- `BallFrozen` removed after paddle growth completes
- Physics activate (GravityScale = 1) when unfrozen

---

### Brick

**Purpose**: Destructible entities that the ball must clear to complete level

**Location**: `src/lib.rs` (marker component)

**Structure**:

```rust
#[derive(Component)]
struct Brick;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BrickTypeId(pub u8);  // Type variant for texture system
```

**Spawning**:

- Spawned from level matrix (cell value ≥ 3)
- Position calculated using grid coordinates
- Dimensions derived from CELL_WIDTH and CELL_HEIGHT:
  - Width: `CELL_HEIGHT * 0.9` (90% of cell height for spacing) → 1.35 units
  - Depth: `CELL_WIDTH * 0.9` (90% of cell width for spacing) → 1.8 units
  - Height: 0.3 units (constant)

**Type Variants**:

- `BrickTypeId(3)` → basic brick
- `BrickTypeId(4)` → alternate brick type (if defined in texture manifest)
- Higher values map to additional brick types

**Level Transition Behavior** (NEW - per FR-012, FR-015, FR-016):

- **CRITICAL**: Bricks must spawn **before** ball physics activate
- Bricks spawned during fade-in phase of level transition
- Visible before or simultaneously with paddle/ball entities

---

## Level Transition State Machine

### LevelAdvanceState

**Purpose**: Manages the timing and sequencing of level transitions

**Location**: `src/level_loader.rs`

**Structure**:

```rust
#[derive(Resource)]
pub struct LevelAdvanceState {
    pub timer: Timer,                     // Initial delay before spawning paddle/ball
    pub active: bool,                     // Transition in progress
    pub growth_spawned: bool,             // Tiny paddle+ball spawned, waiting for growth
    pub pending: Option<LevelDefinition>, // Next level awaiting brick spawn
}
```

**State Sequence** (TARGET - per FR-015):

1. **Level Clear Detected**
   - Last brick destroyed
   - `advance_level_when_cleared` system triggers transition
   - `pending` set to next LevelDefinition
   - `active` = true
   - Fade overlay spawns

2. **Brick Spawn Phase** (NEW BEHAVIOR)
   - Bricks from `pending` level spawned immediately
   - Bricks visible during fade-in
   - Timer starts (1.0 second delay)

3. **Paddle/Ball Spawn Phase**
   - Timer expires
   - Tiny paddle and frozen ball spawned (with `BallFrozen` marker)
   - `PaddleGrowing` component added with 1-second timer
   - `growth_spawned` = true

4. **Paddle Growth Phase**
   - `PaddleGrowing.timer` counts down (1.0 second)
   - Paddle scale interpolates from tiny to full size
   - Ball remains frozen (no physics)

5. **Physics Activation Phase**
   - Paddle growth completes
   - `BallFrozen` marker removed from ball
   - Ball physics activate (GravityScale = 1)
   - Gameplay begins
   - `active` = false, transition complete

**Key Timing Constraints**:

- Bricks must be visible before ball can move (FR-012, FR-016)
- Ball frozen for entire paddle growth duration (FR-13, FR-014)
- Total transition time ≤ 2 seconds (SC-010)

---

### FadeOverlay

**Purpose**: Full-screen UI element providing visual feedback during level transitions

**Location**: `src/level_loader.rs`

**Structure**:

```rust
#[derive(Component)]
struct FadeOverlay {
    timer: Timer,     // Duration: 2.0 seconds
    fade_in: bool,    // true = fade to black, false = fade to transparent
}
```

**Behavior**:

- Spawned when level transition begins
- Fades to black (alpha 0.0 → 0.8) over 1.0 second
- Holds at 80% opacity briefly
- Fades out (alpha 0.8 → 0.0) over remaining time
- Bricks must appear during fade-in phase (FR-017, FR-018)
- Overlay remains visible until bricks fully spawned (FR-019)

---

## Grid Visualization

### GridOverlay

**Purpose**: Debug wireframe overlay showing 20x20 grid structure

**Location**: `src/systems/grid_debug.rs`

**Structure**:

```rust
#[derive(Component)]
pub struct GridOverlay;
```

**Rendering**:

- Spawns GRID_WIDTH (20) vertical lines at Z positions: `-PLANE_W/2 + i * CELL_WIDTH`
- Spawns GRID_HEIGHT (20) horizontal lines at X positions: `-PLANE_H/2 + j * CELL_HEIGHT`
- Lines extend full height/width of play area
- Material: wireframe with configurable color

**Toggle**:

- Space key enables/disables grid overlay
- System: `toggle_grid_debug` in `src/systems/grid_debug.rs`

**Requirements** (per FR-004, FR-023):

- Must render exactly 20 lines in each direction after grid change
- Must maintain current wireframe visual style (color, thickness)

---

## Data Flow

### Level Loading Flow

```text
1. Load RON file (native) or embedded string (WASM)
   ↓
2. Deserialize into LevelDefinition
   ↓
3. Validate matrix dimensions (expect 20x20)
   ├─ If wrong: log warning, apply padding/truncation
   └─ If correct: proceed
   ↓
4. Store in CurrentLevel resource
   ↓
5. Clear existing entities (bricks, paddle, ball)
   ↓
6. Iterate matrix: for each non-zero cell
   ├─ Calculate world position from (row, col)
   ├─ Spawn entity based on cell value (paddle, ball, brick)
   └─ Apply physics components (colliders, rigid bodies)
   ↓
7. Level ready for gameplay
```

### Level Transition Flow (NEW)

```text
1. Last brick destroyed
   ↓
2. advance_level_when_cleared detects clear
   ↓
3. Load next LevelDefinition into pending
   ↓
4. Spawn FadeOverlay (fade to black)
   ↓
5. Spawn bricks from pending level (BEFORE paddle/ball)
   ↓
6. Start timer (1.0 second)
   ↓
7. Timer expires → spawn tiny paddle + frozen ball
   ↓
8. Add PaddleGrowing component (1.0 second)
   ↓
9. Paddle grows, ball frozen
   ↓
10. Growth completes → remove BallFrozen marker
   ↓
11. Activate ball physics (GravityScale = 1)
   ↓
12. Fade overlay completes and despawns
   ↓
13. Gameplay begins
```

---

## Position Calculations

### Grid to World Coordinates

**Formula**:

```rust
let x = -PLANE_H / 2.0 + (row as f32 * CELL_HEIGHT) + CELL_HEIGHT / 2.0;
let z = -PLANE_W / 2.0 + (col as f32 * CELL_WIDTH) + CELL_WIDTH / 2.0;
let y = 0.0;  // All entities spawn at ground level
```

**Example (20x20 grid)**:

- Cell (0, 0): (-13.5, 0.0, -19.0) [top-left]
- Cell (0, 19): (-13.5, 0.0, +19.0) [top-right]
- Cell (19, 0): (+13.5, 0.0, -19.0) [bottom-left]
- Cell (19, 19): (+13.5, 0.0, +19.0) [bottom-right]
- Cell (10, 10): (0.75, 0.0, 1.0) [near center]

**Spacing**:

- Entities spawn at cell centers (+ CELL_WIDTH/2, + CELL_HEIGHT/2 offsets)
- Bricks sized at 90% of cell dimensions to create visible gaps
- Gap width: 0.1 * CELL_WIDTH = 0.2 units (Z-axis)
- Gap height: 0.1 * CELL_HEIGHT = 0.15 units (X-axis)

---

## Asset File Format

### level_XXX.ron Structure

**Current Format** (22x22):

```ron
(
    number: 1,
    gravity: Some((0.0, -15.0, 0.0)),
    matrix: [
        // 22 rows, each with 22 columns
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        // ... 20 more rows
    ],
    presentation: Some((/* texture config */))
)
```

**Target Format** (20x20):

```ron
(
    number: 1,
    gravity: Some((0.0, -15.0, 0.0)),
    matrix: [
        // 20 rows, each with 20 columns
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        // ... 18 more rows
    ],
    presentation: Some((/* texture config */))
)
```

**Cell Value Encoding**:

- `0`: Empty cell (no entity spawned)
- `1`: Paddle spawn position
- `2`: Ball spawn position
- `3`: Basic brick (type 0)
- `4`: Alternate brick (type 1)
- `5+`: Additional brick types (if defined)

---

## Backward Compatibility

### Handling Legacy 22x22 Levels

**Strategy** (per FR-024, FR-025, FR-026):

- **Parse-time**: Accept any matrix dimensions (no strict deserialization failure)
- **Spawn-time**: Apply padding/truncation to ensure 20x20 behavior

**Padding Logic**:

```rust
// If matrix has < 20 rows, pad with empty rows
while matrix.len() < 20 {
    matrix.push(vec![0; 20]);
}

// For each row: if < 20 cols, pad with 0
for row in &mut matrix {
    while row.len() < 20 {
        row.push(0);
    }
}
```

**Truncation Logic**:

```rust
// If matrix has > 20 rows, truncate and warn
if matrix.len() > 20 {
    warn!("Level matrix has {} rows; truncating to 20", matrix.len());
    matrix.truncate(20);
}

// For each row: if > 20 cols, truncate and warn
for (i, row) in matrix.iter_mut().enumerate() {
    if row.len() > 20 {
        warn!("Row {} has {} columns; truncating to 20", i, row.len());
        row.truncate(20);
    }
}
```

**Error Messages**:

- Format: `"Level matrix wrong dimensions; expected 20x20, got {rows}x{cols}"`
- Log level: WARN (not ERROR, since we recover)

---

## Testing Scenarios

### Unit Tests

- Parse 20x20 level file → verify matrix dimensions
- Parse 22x22 level file → verify padding/truncation
- Parse malformed level (15x18) → verify padding
- Validate position calculation for each corner cell
- Verify CELL_WIDTH = 2.0 and CELL_HEIGHT = 1.5 after constant change

### Integration Tests

- Load level_001.ron → verify all entities spawn
- Transition level_001 → level_002 → verify timing
- Clear level → verify brick spawn before ball physics
- Enable grid overlay → verify 20 lines per direction

### WASM Tests

- Verify embedded_level_str() returns updated 20x20 strings
- Load level in browser → verify entities spawn correctly
- Verify WASM binary size impact (22x22 vs 20x20 embedded data)

---

## References

- `src/lib.rs`: Grid constant definitions, marker components
- `src/level_loader.rs`: LevelDefinition, spawning logic, transition state machine
- `src/systems/grid_debug.rs`: Grid overlay rendering
- `src/systems/level_switch.rs`: Level discovery, switching requests
- `assets/levels/level_001.ron`: First level asset
- `assets/levels/level_002.ron`: Second level asset
- `specs/003-map-format/spec.md`: Feature requirements
- `specs/003-map-format/research.md`: Technical decisions
