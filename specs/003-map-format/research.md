# Research: Map Format Change (22x22 to 20x20)

**Feature**: 003-map-format **Created**: 2025-11-27 **Purpose**: Identify technical unknowns and establish best practices for grid dimension changes

## Phase 0: Knowledge Gaps

### 1. Grid Constant Dependencies

**Question**: What systems and modules depend on GRID_WIDTH and GRID_HEIGHT constants?

**Current Knowledge**:

- Constants defined in `src/lib.rs` (GRID_WIDTH=22, GRID_HEIGHT=22)
- Used in `src/level_loader.rs` for validation and entity positioning
- Used in `src/systems/grid_debug.rs` for overlay rendering
- Cell dimensions calculated as CELL_WIDTH = PLANE_W / GRID_WIDTH, CELL_HEIGHT = PLANE_H / GRID_HEIGHT

**Research Needed**:

- [ ] Are there any implicit assumptions about grid dimensions in physics calculations?
- [ ] Do any collision detection systems hardcode 22x22 assumptions?
- [ ] Are grid dimensions referenced in any asset files or configuration?

**Impact**: High - Missing dependencies could cause runtime errors

---

### 2. Level Transition State Machine

**Question**: What is the exact state machine sequence for level transitions, and where should brick spawning occur?

**Current Knowledge**:

- Level transitions use `LevelAdvanceState` resource
- Paddle growth animation takes 1 second (PADDLE_GROWTH_DURATION)
- Ball should be frozen until paddle growth completes
- Fade overlay provides visual feedback

**Research Needed**:

- [ ] What are the exact states in `LevelAdvanceState`?
- [ ] Which system spawns bricks, and when is it scheduled?
- [ ] How is ball physics controlled (GravityScale, Velocity, BallFrozen marker)?
- [ ] What triggers the transition from frozen ball to active physics?
- [ ] Is there a system that manages the fade overlay timing?

**Impact**: High - Core requirement FR-012 to FR-019 depend on understanding this sequence

---

### 3. WASM Asset Embedding

**Question**: How are level files embedded in WASM builds, and what needs updating?

**Current Knowledge**:

- `src/level_loader.rs` has `embedded_level_str()` function using `include_str!`
- Current implementation: `"level_001" => include_str!("../assets/levels/level_001.ron")`
- WASM has no filesystem access, requires compile-time embedding

**Research Needed**:

- [ ] Are there any caching or preprocessing steps for embedded strings?
- [ ] Do embedded strings need explicit file path validation?
- [ ] Will changing RON file contents (22x22 → 20x20) affect WASM binary size significantly?

**Impact**: Medium - Must ensure WASM builds work with new level format

---

### 4. Entity Positioning Calculations

**Question**: How are entity positions calculated from grid coordinates, and will 20x20 maintain proper spacing?

**Current Knowledge**:

- Position calculation uses: `row * CELL_HEIGHT + offset_y`, `col * CELL_WIDTH + offset_x`
- Offsets are from `-PLANE_H/2` and `-PLANE_W/2` (center origin)
- PLANE_H = 30.0, PLANE_W = 40.0
- 22x22 gives CELL_HEIGHT ≈ 1.364, CELL_WIDTH ≈ 1.818
- 20x20 gives CELL_HEIGHT = 1.5, CELL_WIDTH = 2.0

**Research Needed**:

- [ ] Will larger cell sizes (1.5 vs 1.364 height) cause any visual issues with brick textures?
- [ ] Do any systems assume minimum/maximum cell dimensions for collision detection?
- [ ] Are entity scales (0.9 multiplier for spacing) still appropriate with new dimensions?

**Impact**: Medium - Affects visual appearance and gameplay feel (SC-007)

---

### 5. Level Validation and Error Handling

**Question**: What is the best strategy for handling malformed level files (wrong dimensions)?

**Current Knowledge**:

- Spec requires padding with 0 (empty cells) for undersized levels (FR-024)
- Spec requires truncation with warning for oversized levels (FR-025)
- Spec requires attempting to load malformed levels rather than rejecting (FR-026)

**Research Needed**:

- [ ] Should padding/truncation happen at parse time or spawn time?
- [ ] What should happen if a level has zero rows or zero columns?
- [ ] Should there be a fallback "safe" level if parsing fails completely?
- [ ] Are there performance implications of runtime padding/truncation vs preprocessing?

**Impact**: Low-Medium - Affects developer experience and robustness

---

### 6. Testing Strategy for Grid Changes

**Question**: What test scenarios are needed to verify 20x20 grid changes work correctly?

**Current Knowledge**:

- Project uses `cargo test` for unit tests
- Success criteria require 100% accuracy for dimension detection (SC-004)
- Must verify WASM builds work (SC-006)
- Must verify level transitions prevent premature ball movement (SC-008, SC-009)

**Research Needed**:

- [ ] Are there existing unit tests for level loading that need updating?
- [ ] How can we test level transition timing in an automated way?
- [ ] What's the best way to validate WASM embedded strings without manual browser testing?
- [ ] Should we add property-based tests for padding/truncation logic?

**Impact**: Medium - Ensures changes don't break existing functionality

---

## Best Practices

### Grid Dimension Changes

**Recommended Approach**:

1. Update constants in single source of truth (`src/lib.rs`)
2. Rely on derived constants (CELL_WIDTH, CELL_HEIGHT) throughout codebase
3. Avoid hardcoding dimensions in multiple locations
4. Use compile-time checks where possible (const assertions)

**Rationale**: Minimizes chance of missed updates, makes future changes easier

---

### Level Transition Timing

**Recommended Approach**:

1. Define explicit state machine states (e.g., Spawning, GrowingPaddle, Active)
2. Use marker components for ball state (BallFrozen, BallActive)
3. Schedule brick spawning system before ball physics systems
4. Gate physics systems with run conditions checking ball state

**Rationale**: Makes sequencing explicit and testable, prevents timing race conditions

---

### WASM Asset Updates

**Recommended Approach**:

1. Keep embedded strings in sync with asset files using build script validation
2. Document which files need updating when changing level format
3. Add compile-time size checks for embedded strings (detect empty/malformed)

**Rationale**: Prevents WASM builds from silently failing with wrong data

---

## Assumptions to Validate

### Assumption 1: Physics Systems Don't Depend on Grid Dimensions

**Assumption**: Physics calculations (collision, velocity, ball bounce) work independently of grid size

**Validation Method**: Review bevy_rapier3d usage and ensure no hardcoded 22x22 assumptions in physics setup

**Risk if Wrong**: HIGH - Physics behavior could change unexpectedly

---

### Assumption 2: Cell Size Increase Won't Affect Gameplay

**Assumption**: Increasing cell dimensions from ~1.36x1.82 to 1.5x2.0 won't affect gameplay difficulty or feel

**Validation Method**: Playtest both grid sizes and compare brick hit patterns and difficulty

**Risk if Wrong**: MEDIUM - May require rebalancing levels or adjusting ball speed

---

### Assumption 3: Padding/Truncation Won't Cause Performance Issues

**Assumption**: Runtime padding/truncation of malformed levels has negligible performance cost

**Validation Method**: Benchmark level loading with various malformed inputs

**Risk if Wrong**: LOW - Can fall back to stricter validation if needed

---

### Assumption 4: Transition Timing Can Be Controlled via System Ordering

**Assumption**: Bevy's system scheduling can guarantee bricks spawn before ball physics activate

**Validation Method**: Test level transitions with logging to verify system execution order

**Risk if Wrong**: HIGH - Core requirement FR-012 to FR-019 depend on this

---

## Technical Decisions

### Decision 1: Where to Apply Padding/Truncation

**Options**:

- A) Parse-time: Modify matrix during RON deserialization
- B) Spawn-time: Apply padding/truncation when iterating matrix to spawn entities
- C) Preprocessing: Validate and fix level files in build script

**Recommendation**: B (Spawn-time)

**Rationale**:

- Simpler implementation (no custom serde deserializer)
- Works consistently for both file-based and embedded levels
- Allows logging with full context about which level failed
- Can be tested independently of serialization logic

---

### Decision 2: Ball Physics Control Mechanism

**Options**:

- A) Marker component: Add/remove `BallFrozen` component
- B) GravityScale toggle: Set GravityScale to 0/1
- C) Velocity clamping: Manually set velocity to zero each frame
- D) System run conditions: Disable physics systems until ready

**Recommendation**: A (Marker component) + D (Run conditions)

**Rationale**:

- Marker component is explicit and testable
- Run conditions prevent accidental physics updates
- Matches Bevy ECS best practices (composition over properties)
- Easy to query in debugging (can see frozen state in entity inspector)

---

### Decision 3: WASM Embedded String Validation

**Options**:

- A) No validation: Trust that embedded strings are correct
- B) Compile-time checks: Use const assertions on string length
- C) Build script: Parse RON at build time and verify dimensions
- D) Runtime checks: Validate embedded strings same as file-based levels

**Recommendation**: D (Runtime checks)

**Rationale**:

- Already implementing validation for file-based levels (FR-001, FR-002)
- Keeps validation logic unified (single code path)
- Compile-time checks can't parse RON format
- Build script adds complexity without much benefit

---

## Follow-up Questions for Spec

None - all clarifications addressed in spec.md clarification session.

---

## References

- `src/lib.rs`: Grid constant definitions
- `src/level_loader.rs`: Level loading and validation logic
- `src/systems/grid_debug.rs`: Debug overlay rendering
- `src/systems/level_switch.rs`: Level transition state machine
- `specs/003-map-format/spec.md`: Feature requirements and user stories
