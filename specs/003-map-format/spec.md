# Feature Specification: Map Format Change (22x22 to 20x20)

**Feature Branch**: `003-map-format`
**Created**: 2025-11-27
**Status**: Draft
**Input**: User description: "change the map format from 22 x 22 to 20 x 20 and modify the the map loading behaviour"

## Clarifications

### Session 2025-11-27

- Q: Current behavior when level finishes - ball spawns and starts moving under gravity before level loads, creating empty-field motion. Should level load first and be displayed before ball physics begin? → A: Yes - level (including bricks) should load and be fully visible first, ball remains frozen until paddle growth animation completes, then gameplay begins
- Q: Level Transition Visual Feedback - Should there be a visual transition effect when switching levels? → A: Players see a brief fade-to-black transition (current behavior) then bricks appear before fade-out completes
- Q: Paddle Growth Animation Duration - How long should the paddle growth animation take during level transitions? → A: Keep current 1-second duration
- Q: Cell Size Adjustment Impact - How should brick visual size change when moving from 22x22 to 20x20 grid? → A: Keep exact mathematical cell size (PLANE_H/20 × PLANE_W/20) even if bricks appear slightly different
- Q: Grid Overlay Color/Style - Should the debug grid overlay visual style change? → A: Keep current wireframe style
- Q: Error Handling for Malformed Grids - How should system handle level files with incorrect dimensions? → A: Log warning and attempt to load with padding/truncation (fill missing cells with empty/0, ignore excess)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Existing Levels Load with 20x20 Format (Priority: P1)

Game developers can create and load level files using a 20x20 grid instead of the current 22x22 format, enabling more compact level designs and reducing memory footprint.

**Why this priority**: This is the core functionality change - without migrating the grid format, all other aspects of the feature cannot function. Existing levels must continue to work after the change.

**Independent Test**: Can be fully tested by loading an existing level file (level_001.ron, level_002.ron) after updating their matrix dimensions to 20x20, and verifying that the game loads without errors and entities spawn in correct positions.

**Acceptance Scenarios**:

1. **Given** a level file with a 20x20 matrix, **When** the game loads the level, **Then** all entities (paddle, ball, bricks) spawn at correct grid positions
2. **Given** a level file with incorrect dimensions (not 20x20), **When** the game attempts to load it, **Then** a clear warning message is logged indicating the expected dimensions
3. **Given** multiple level files with 20x20 format, **When** progressing through levels, **Then** each level loads correctly with proper entity placement

---

### User Story 2 - Grid Visualization Updates (Priority: P2)

Players and developers can toggle the debug grid overlay and see a 20x20 grid that accurately represents the new play area dimensions.

**Why this priority**: Visual debugging tools must match the new grid format to maintain developer productivity and help with level design. This is secondary to the core loading functionality.

**Independent Test**: Can be tested by enabling wireframe mode (Space key) and verifying that exactly 20 horizontal and 20 vertical grid lines are drawn, matching the play area boundaries.

**Acceptance Scenarios**:

1. **Given** the game is running with wireframe mode enabled, **When** viewing the play area, **Then** a 20x20 grid overlay is displayed matching the game area boundaries
2. **Given** entities are spawned on the grid, **When** wireframe mode is enabled, **Then** entities align precisely with the 20x20 grid cell centers

---

### User Story 3 - Level Transition Sequence Control (Priority: P2)

Players experience smooth level transitions where the new level is fully loaded and visible before gameplay begins, preventing premature ball movement on empty fields.

**Why this priority**: Critical for user experience - without proper sequencing, players see confusing behavior (ball moving in empty space before level appears). This affects gameplay clarity and professional polish.

**Independent Test**: Can be tested by clearing all bricks in a level and observing that: (1) bricks for the next level appear before ball physics begin, (2) ball remains frozen during paddle growth animation, (3) gameplay only starts after paddle growth completes.

**Acceptance Scenarios**:

1. **Given** a player completes the current level, **When** transitioning to the next level, **Then** all bricks load and are visible before the ball begins moving
2. **Given** the level transition sequence starts, **When** the paddle begins its growth animation, **Then** the ball remains frozen (no gravity or physics applied) until growth completes
3. **Given** the paddle growth animation completes, **When** the animation finishes, **Then** ball physics activate and gameplay begins with the ball responding to gravity
4. **Given** bricks are spawning for the new level, **When** the level loads, **Then** bricks appear before or simultaneously with the paddle/ball spawn, never after

---

### User Story 4 - Backward Compatibility Warning (Priority: P3)

Level designers receive clear feedback when attempting to load legacy 22x22 level files, helping them understand what needs to be updated.

**Why this priority**: Provides helpful migration guidance but doesn't block core functionality. Users can manually update their level files based on clear error messages.

**Independent Test**: Can be tested by attempting to load an old 22x22 level file and verifying that a descriptive error message is logged explaining the format change and expected dimensions.

**Acceptance Scenarios**:

1. **Given** a level file with 22x22 dimensions, **When** the game attempts to load it, **Then** a warning message is logged stating "Level matrix wrong dimensions; expected 20x20, got 22x22"
2. **Given** a level file with arbitrary wrong dimensions (e.g., 18x18), **When** loading fails, **Then** the error message clearly indicates both expected (20x20) and actual dimensions

---

### Edge Cases

- What happens when a level file has rows of inconsistent length (e.g., first row has 20 cells, second has 18)? → Pad short rows with 0 (empty cells)
- How does the system handle empty matrix (0x0)? → Log warning, load as empty level with fallback paddle/ball spawns
- What if a level file has correct row count (20) but wrong column count? → Apply padding/truncation per FR-024/FR-025
- How are entity positions recalculated for different grid dimensions to maintain proper spacing? → Use exact mathematical cell size (PLANE_H/20, PLANE_W/20)
- What happens to hardcoded references to "22x22" in documentation, comments, and error messages? → Must be updated to "20x20" per FR-009
- What if paddle growth animation is interrupted or canceled during level transition? → Ball remains frozen until animation completes or times out
- How does the system handle level loading failures during the transition sequence (e.g., corrupted level file)? → Apply padding/truncation recovery per FR-024/FR-025/FR-026
- What happens if a player exits the game during the level transition sequence? → Standard game exit, no special handling needed

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST validate level matrix dimensions as 20x20 (20 rows, each with 20 columns)
- **FR-002**: System MUST log a warning message when level matrix dimensions don't match expected 20x20 format
- **FR-003**: System MUST include actual dimensions in error messages when validation fails
- **FR-024**: When level has fewer than 20 rows or columns, system MUST pad missing cells with value 0 (empty)
- **FR-025**: When level has more than 20 rows or columns, system MUST truncate excess cells and log which cells were ignored
- **FR-026**: System MUST attempt to load malformed levels rather than rejecting them entirely
- **FR-004**: Grid debug overlay MUST render exactly 20 horizontal and 20 vertical lines
- **FR-023**: Grid debug overlay MUST maintain current wireframe visual style (color, thickness, material)
- **FR-005**: Entity spawn positions MUST be calculated based on 20x20 grid cell layout
- **FR-006**: Cell dimensions MUST be recalculated to fit the play area (PLANE_H × PLANE_W) divided into 20x20 grid
- **FR-007**: All level definition files (level_001.ron, level_002.ron) MUST be updated to 20x20 format
- **FR-008**: System MUST handle embedded level files (WASM builds) with 20x20 format
- **FR-009**: Documentation and code comments referencing grid dimensions MUST be updated to reflect 20x20 format
- **FR-010**: Grid cell width MUST be calculated as PLANE_W / 20.0
- **FR-011**: Grid cell height MUST be calculated as PLANE_H / 20.0
- **FR-020**: Brick dimensions MUST use exact mathematical cell size without visual scaling adjustments
- **FR-021**: Brick width MUST be (PLANE_H / 20.0) × 0.9 (90% of cell height for spacing)
- **FR-022**: Brick depth MUST be (PLANE_W / 20.0) × 0.9 (90% of cell width for spacing)
- **FR-012**: When level completes and next level loads, bricks MUST spawn before ball physics activate
- **FR-013**: Ball MUST remain frozen (no physics applied) during paddle growth animation in level transitions
- **FR-014**: Ball physics (gravity and movement) MUST activate only after paddle growth animation completes
- **FR-015**: Level transition sequence MUST follow this order: (1) spawn bricks, (2) spawn frozen ball and tiny paddle, (3) paddle grows, (4) ball physics activate
- **FR-016**: Bricks MUST be visible on screen before or simultaneously with paddle/ball entities during level transitions
- **FR-017**: Level transitions MUST use fade-to-black overlay animation (existing behavior)
- **FR-018**: Bricks MUST appear during fade-in phase, before fade overlay fully disappears
- **FR-019**: Fade overlay MUST remain visible until bricks are fully spawned and ready to display

### Key Entities

- **LevelDefinition**: Contains the level matrix (2D array of unsigned 8-bit integers) that defines entity placement in a 20x20 grid; accessed as matrix\[row\]\[col\] where row and col each range from 0-19
- **Grid Cell**: Represents a single cell in the 20x20 grid; has calculated width (PLANE_W/20) and height (PLANE_H/20); entities spawn at cell centers
- **Level Matrix**: 20x20 2D array where each value indicates entity type (0=empty, 1=paddle, 2=ball, 3+=brick types)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All existing level files load successfully with 20x20 format within the same loading time as 22x22 format
- **SC-002**: Grid debug overlay displays exactly 20 lines in each direction when enabled
- **SC-003**: Entity spawn positions in 20x20 grid maintain the same relative spacing as in 22x22 grid (entities don't overlap or appear outside play area)
- **SC-004**: Level validation detects and logs dimension mismatches with 100% accuracy (all incorrect dimensions are caught)
- **SC-005**: Zero references to "22x22" or "22 x 22" remain in code, comments, or documentation after migration
- **SC-006**: WASM builds successfully load embedded level files with 20x20 format
- **SC-007**: Game maintains the same visual appearance and gameplay feel with 20x20 grid as it had with 22x22 grid
- **SC-008**: In 100% of level transitions, bricks are visible before ball physics begin (zero occurrences of ball moving on empty field)
- **SC-009**: Ball remains completely stationary (zero velocity) during entire paddle growth animation period
- **SC-010**: Level transition sequence completes within 2 seconds from level clear to gameplay start
