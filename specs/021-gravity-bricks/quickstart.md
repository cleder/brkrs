# Developer Quickstart: Gravity Indicator UI

**Date**: 2026-01-11 | **Feature**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)

## Overview

This guide helps developers understand, test, and extend the gravity indicator UI feature.

## Prerequisites

- Familiarity with Bevy 0.17.3 ECS architecture
- Understanding of [spec.md](spec.md) user stories
- Knowledge of Bevy UI system (`ImageNode`, `Node`)
- Physics-enabled levels with `GravityConfiguration` resource

## Quick Verification

1. **Visual Check**:

   ```bash
   cargo run
   # Launch any level with gravity physics
   # Look for weight icon in bottom-left corner
   ```

2. **Expected Behavior**:
   - Icon appears at (12px, 12px) from bottom-left
   - Displays appropriate weight (0/2/10/20) or "?" if unknown
   - Updates immediately when gravity changes (e.g., hitting gravity brick)

## Architecture Overview

**Module**: `src/ui/gravity_indicator.rs` **Plugin Registration**: `src/ui/mod.rs` → `UiPlugin` **Assets**: `assets/textures/default/weight-{0,2,10,20,question}.png`

### Key Components

```rust
// Marker component for the indicator entity
#[derive(Component)]
pub struct GravityIndicator;

// Texture cache (loaded at startup)
#[derive(Resource)]
pub struct GravityIndicatorTextures {
    pub question: Handle<Image>,
    pub weight0: Handle<Image>,
    pub weight2: Handle<Image>,
    pub weight10: Handle<Image>,
    pub weight20: Handle<Image>,
}
```

### System Flow

```text
Startup → setup_ui_assets
    ↓
    Loads 5 textures into GravityIndicatorTextures
    ↓
Update → spawn_gravity_indicator (runs once)
    ↓
    Waits for GravityConfiguration + GravityIndicatorTextures
    ↓
    Spawns indicator entity with ImageNode + Node + GravityIndicator
    ↓
Update → update_gravity_indicator (runs on Changed<GravityConfiguration>)
    ↓
    Maps new gravity → level → texture
    ↓
    Updates ImageNode with new texture handle
```

## Testing

### Unit Tests

Run gravity mapping logic tests:

```bash
cargo test gravity_indicator::tests --lib
```

**Test Coverage**:

- Exact level boundaries (0.0, 2.0, 10.0, 20.0)
- Tolerance edges (1.5, 2.49, 9.51, 10.4)
- Mixed axes (highest magnitude wins)
- Unknown values (outside tolerance)

### Integration Tests

Run full UI integration tests:

```bash
cargo test test_gravity_indicator --test '*'
```

**Test Coverage**:

- Spawn timing correctness
- Idempotence (no duplicate indicators)
- Update correctness
- Multi-frame persistence
- Level transition behavior

### Manual Testing

1. **Level Launch**: Start game → verify indicator appears with correct icon
2. **Gravity Change**: Destroy gravity brick → verify icon updates immediately
3. **Level Transition**: Complete level → next level → verify icon updates to new default
4. **Pause**: Pause game → verify indicator persists
5. **Life Loss**: Lose life → verify indicator resets to level default

## Debugging

### Indicator Not Appearing

**Check 1**: Textures loaded?

```bash
# Look for warnings in console:
# "GravityIndicatorTextures not ready yet"
```

**Check 2**: GravityConfiguration exists?

```rust
// Add debug logging in spawn_gravity_indicator:
info!("GravityConfiguration: {:?}", gravity_cfg);
```

**Check 3**: Already spawned?

```rust
// Check query result:
info!("Existing indicators: {}", existing.iter().count());
```

### Incorrect Icon Displayed

**Debug mapping logic**:

```rust
// Add logging in update_gravity_indicator:
let level = map_gravity_to_level(gravity_cfg.current);
info!("Gravity {:?} mapped to {:?}", gravity_cfg.current, level);
```

**Common issues**:

- **Y-axis gravity**: Y component is ignored (only X/Z matter)
- **Tolerance edge cases**: 9.4 rounds to 9, but 0.6 away from 9 → Unknown
- **Mixed axes**: Highest magnitude wins (X=2, Z=10 → shows L10)

### Icon Not Updating

**Check change detection**:

```rust
// Verify GravityConfiguration is marked changed:
info!("Gravity changed: {}", gravity_cfg.is_changed());
```

**Common causes**:

- Change detection only fires when resource is mutated
- If `current` field not modified, no update triggered
- Ensure gravity brick destruction or level loader mutates `GravityConfiguration`

## Extension Points

### Adding New Gravity Levels

1. **Add texture**: `assets/textures/default/weight-{value}.png`
2. **Extend enum**:

   ```rust
   pub enum GravityLevel {
       // ... existing ...
       L5, // New level
   }
   ```

3. **Update mapping logic**:

   ```rust
   fn map_gravity_to_level(g: Vec3) -> GravityLevel {
       // Add case for 5:
       5 => GravityLevel::L5,
   }
   ```

4. **Add texture to resource**:

   ```rust
   pub struct GravityIndicatorTextures {
       // ... existing ...
       pub weight5: Handle<Image>,
   }
   ```

5. **Update selection logic**:

   ```rust
   fn select_texture(...) -> &Handle<Image> {
       match level {
           // ... existing ...
           L5 => &textures.weight5,
       }
   }
   ```

6. **Add tests**: Cover new level in unit + integration tests

### Customizing Position

Edit `spawn_gravity_indicator` system:

```rust
Node {
    position_type: PositionType::Absolute,
    left: Val::Px(12.0),   // Change this for horizontal offset
    bottom: Val::Px(12.0), // Change this for vertical offset
    ..Default::default()
}
```

**Example**: Move to top-left:

```rust
Node {
    position_type: PositionType::Absolute,
    left: Val::Px(12.0),
    top: Val::Px(12.0),  // Changed from bottom
    ..Default::default()
}
```

### Supporting Y-Axis Gravity

**Current**: Y-axis is ignored (only X/Z considered)

**Modification**:

1. Update `map_gravity_to_level`:

   ```rust
   pub fn map_gravity_to_level(g: Vec3) -> GravityLevel {
       let x = g.x.round() as i32;
       let y = g.y.round() as i32; // Add this
       let z = g.z.round() as i32;

       // Include y in checks:
       let valid_y = (g.y - y as f32).abs() <= 0.5;

       // Add y to max_abs:
       let mut max_abs = 0;
       if valid_x { max_abs = max_abs.max(x.abs()); }
       if valid_y { max_abs = max_abs.max(y.abs()); } // Add this
       if valid_z { max_abs = max_abs.max(z.abs()); }
   }
   ```

2. Update tests to cover Y-axis scenarios
3. Update spec.md to document Y-axis support

## Performance Notes

### Resource Impact

- **Textures**: 5 × ~4KB PNG = ~20KB memory (negligible)
- **Entity**: 1 entity with 3 components (ImageNode + Node + GravityIndicator)
- **Systems**: 2 systems (spawn runs once, update runs on change)

### Change Detection Efficiency

- `spawn_gravity_indicator`: Runs every frame until indicator spawned, then short-circuits via `!existing.is_empty()`
- `update_gravity_indicator`: Only runs when `GravityConfiguration.is_changed()` returns true

**Cost**: Minimal.
Update system skipped 99% of frames (gravity rarely changes).

### Optimization Opportunities

None needed.
Feature is already highly efficient:

- No per-frame calculations
- No complex queries
- No hierarchies
- Change-detection prevents unnecessary updates

## Troubleshooting Checklist

- [ ] Textures exist at `assets/textures/default/weight-*.png`
- [ ] `GravityIndicatorTextures` resource initialized
- [ ] `GravityConfiguration` resource exists
- [ ] Indicator entity spawned (check with `Query<Entity, With<GravityIndicator>>`)
- [ ] System ordering correct (`spawn_gravity_indicator` before `update_gravity_indicator`)
- [ ] Change detection working (`gravity_cfg.is_changed()` returns true)
- [ ] Mapping logic correct (unit tests passing)
- [ ] No duplicate indicators (spawn idempotence working)

## References

- **Spec**: [spec.md](spec.md) - User requirements and acceptance criteria
- **Plan**: [plan.md](plan.md) - Implementation plan and constitution check
- **Data Model**: [data-model.md](data-model.md) - ECS component/resource design
- **Research**: [research.md](research.md) - Design decisions and alternatives

## Common Questions

**Q**: Why only X/Z axes?
**A**: Gravity bricks only affect horizontal gravity.
Y-axis is reserved for default downward gravity (not visualized).

**Q**: Why ±0.5 tolerance?
**A**: Allows floating-point imprecision while rejecting clearly mismatched values.
Example: 2.1 → valid (0.1 away), 2.6 → invalid (0.6 away).

**Q**: What happens with gravity outside known levels?
**A**: Displays "?" (unknown/question mark icon) until recognized level detected.

**Q**: How to test multi-frame persistence?
**A**: Integration test `test_gravity_indicator_multi_frame_persistence` runs 10-frame simulation and verifies icon unchanged.

**Q**: Can I use this pattern for other indicators?
**A**: Yes!
Copy structure: marker component → spawn once → update on change detection → use `ImageNode` for icon swapping.
