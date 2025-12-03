# Retrospective: Multi-Hit Bricks (005-multi-hit-bricks)

**Date**: 2025-11-29 **Duration**: Single session **Status**: ✅ Complete (pending PR merge)

## Summary

Implemented multi-hit bricks (indices 10-13) that require multiple ball collisions to destroy.
Each hit transitions the brick to the next lower index until it becomes a simple stone (index 20), which is then destroyed on the next hit.

## What Was Delivered

### Core Implementation

- **Constants**: `MULTI_HIT_BRICK_1..4` (10-13) in `src/level_format/mod.rs`
- **Helper**: `is_multi_hit_brick()` function for type checking
- **Event**: `MultiHitBrickHit` event for collision notifications
- **Systems**:
  - `watch_brick_type_changes` for material updates on `Changed<BrickTypeId>`
  - `on_multi_hit_brick_sound` observer placeholder for audio

### Integration

- Modified `mark_brick_on_ball_collision` in `src/lib.rs` to:
  - Detect multi-hit bricks before marking for despawn
  - Mutate `BrickTypeId` component instead of destroying
  - Emit `MultiHitBrickHit` event for observers
  - Handle special case: index 10 → index 20 (simple stone)

### Assets & Tests

- Added `assets/levels/level_998.ron` test level
- Added type variants for indices 10-13 in `assets/textures/manifest.ron`
- Created `tests/multi_hit_bricks.rs` with 7 integration tests
- All 54 tests pass

### Documentation Housekeeping

- Moved 47 gif files from `docs/` to `docs/_static/images/`
- Updated all image references in `docs/bricks.md`

## What Went Well

1. **Speckit Workflow**: The structured approach (specify → clarify → plan → tasks → implement) provided clear guidance throughout
2. **Bevy 0.17 Patterns**: Successfully used modern Bevy patterns:
   - Observer pattern with `On<Event>` for custom events
   - `Changed<T>` queries for reactive material updates
   - ECS component mutation instead of entity recreation
3. **Existing Infrastructure**: The `TypeVariantRegistry` system made material swapping straightforward
4. **Test Coverage**: Integration tests caught issues early and verified all state transitions

## Challenges Encountered

1. **RON Format Quirks**:
   - RON doesn't support `//` comments - had to use `/* */` or remove comments
   - Learning: Always validate RON files with parser before committing

2. **Bevy 0.17 API Changes**:
   - `EventReader` is deprecated in favor of observers
   - Custom events use `#[derive(Event)]` + `app.add_observer()`
   - Initial instinct was wrong; had to research current patterns

3. **Doctest Environment Issues**:
   - CI failed with shared library loading error
   - Fixed by marking doctest as `no_run` (compiles but doesn't execute)
   - Learning: Bevy-dependent doctests often need `no_run` or `ignore`

4. **Test Parallelism**:
   - Environment variable tests (`BK_LEVEL`) can conflict in parallel
   - Used `--test-threads=1` for reliable local testing

## Patterns Worth Documenting

### Component Mutation Pattern

Instead of despawning and respawning entities to change state:

```rust
// Mutate the component in-place
brick_type.0 = new_type_id;
// Let Changed<BrickTypeId> systems handle visual updates
```

### Observer Pattern for Events

```rust
#[derive(Event)]
pub struct MyEvent { /* fields */ }

pub fn my_observer(trigger: On<MyEvent>) {
    let event = trigger.event();
    // Handle event
}

// In app setup:
app.add_observer(my_observer);
```

## Metrics

| Metric | Value |
|--------|-------|
| Files Changed | 65 |
| Lines Added | ~1,586 |
| Lines Removed | ~82 |
| New Tests | 7 |
| Total Tests Passing | 54 |
| Commits | 4 |

## Recommendations for Developer Guide

After reviewing `docs/developer-guide.md`, the following updates were made:

1. ✅ **Add `src/systems/multi_hit.rs`** to the repository structure section
2. ✅ **Document the observer pattern** in "Architecture overview" - it's the preferred way to handle custom events in Bevy 0.17
3. ✅ **Add note about doctest limitations** - Bevy-dependent doctests often need `no_run`
4. ✅ **Document test parallelism issues** - Recommend `--test-threads=1` for env-var-dependent tests

## Follow-up Items

- [ ] Merge PR after CI passes
- [x] Update developer guide with patterns learned
- [ ] Audio system integration (Sound 29) when audio is implemented → [#23](https://github.com/cleder/brkrs/issues/23)
- [ ] Consider adding visual effects for multi-hit transitions (particles, flash) → [#24](https://github.com/cleder/brkrs/issues/24)
