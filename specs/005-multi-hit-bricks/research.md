# Research: Multi-Hit Bricks

**Feature**: 005-multi-hit-bricks
**Date**: 2025-11-29

## Overview

This document captures research findings for implementing multi-hit bricks (indices 10-13) that require multiple ball collisions to transition through damage states before becoming destructible.

## Key Decisions

### 1. State Representation

**Decision**: Use existing `BrickTypeId(u8)` component for state tracking

**Rationale**:

- `BrickTypeId` already exists and is set during brick spawn
- The type ID directly maps to visual appearance via texture manifest
- No additional component needed; index value IS the state
- Transition: decrement `BrickTypeId` value on collision (13→12→11→10→20)

**Alternatives Considered**:

- New `MultiHitState { hits_remaining: u8 }` component: Rejected because it duplicates information already in `BrickTypeId` and would require syncing two sources of truth
- Separate `HitCounter` component: Rejected for same reason; adds complexity without benefit

### 2. Collision Handling Approach

**Decision**: Extend `mark_brick_on_ball_collision` system to check for multi-hit bricks before marking for despawn

**Rationale**:

- Collision detection already identifies ball-brick collisions
- Current system marks bricks for immediate despawn
- New logic: if `BrickTypeId` in range 10-13, mutate instead of marking for despawn
- If `BrickTypeId` is 10, transition to 20 (simple stone)
- Simple stones (20) continue through existing despawn path

**Alternatives Considered**:

- Separate observer system: Adds overhead and complexity; collision is already identified in existing system
- Event-driven transformation: Would require new event type and handler; unnecessary indirection

### 3. Visual Transition

**Decision**: Material swap on `BrickTypeId` change via existing texture manifest system

**Rationale**:

- `TypeVariantRegistry` already supports per-type materials for bricks
- When `BrickTypeId` changes, a watcher system detects the change and updates `MeshMaterial3d`
- Pattern already established for `BallTypeId` material swapping
- Need to add watcher for `BrickTypeId` changes (similar to ball-type watcher)

**Alternatives Considered**:

- Immediate material lookup on collision: Would require passing material resources to collision system; violates separation of concerns
- Replace entity entirely: Wasteful; component mutation is more efficient

### 4. Audio Feedback

**Decision**: Emit event on multi-hit collision for audio system to handle

**Rationale**:

- Sound 29 should play on multi-hit brick collisions
- Current audio system (if implemented) uses events
- Decouples collision logic from audio playback

**Implementation Note**: Audio system may not be fully implemented yet. Document the event emission; audio playback can be added when audio system is ready.

### 5. Scoring

**Decision**: Award points on each multi-hit collision (50 pts) and on final destruction (25 pts)

**Rationale**:

- Per spec: 50 points per hit during multi-hit phase
- Simple stone destruction awards standard 25 points
- Scoring system (if implemented) should listen for brick hit/destroy events

**Implementation Note**: Score tracking may not be fully implemented. Document the scoring events; actual score updates can be added when scoring system is ready.

### 6. Level Completion Logic

**Decision**: Multi-hit bricks (10-13) should have `CountsTowardsCompletion` component

**Rationale**:

- Existing `advance_level_when_cleared` queries for bricks with `CountsTowardsCompletion`
- Multi-hit bricks are destructible, so they should count
- When they transition to simple stone (20), they KEEP the component
- Level completes when all `CountsTowardsCompletion` entities are despawned

**Implementation Note**: Current code adds `CountsTowardsCompletion` for all bricks except index 90 (indestructible). Indices 10-13 will automatically get this component since they are < 90 and >= 3.

## Dependencies

### Existing Code to Modify

1. **`src/lib.rs`** - `mark_brick_on_ball_collision` system
   - Add check for multi-hit brick indices (10-13)
   - Mutate `BrickTypeId` instead of marking for despawn
   - Transition 10→20 special case

2. **`src/level_format/mod.rs`** - Add constants
   - `MULTI_HIT_BRICK_1: u8 = 10`
   - `MULTI_HIT_BRICK_2: u8 = 11`
   - `MULTI_HIT_BRICK_3: u8 = 12`
   - `MULTI_HIT_BRICK_4: u8 = 13`

3. **`assets/textures/manifest.ron`** - Add type variants
   - Brick type variants for indices 10, 11, 12, 13
   - Map to Stonehit1.gif, Stonehit2.gif, Stonehit3.gif, Stonehit4.gif textures

### New Code to Create

1. **`src/systems/multi_hit.rs`** - Brick type watcher system
   - Detect `BrickTypeId` changes on bricks
   - Update material from type registry
   - Emit audio event for multi-hit sound

2. **`tests/multi_hit_bricks.rs`** - Integration tests
   - Test state transitions (13→12→11→10→20→destroyed)
   - Test level completion with multi-hit bricks
   - Test material changes on transitions

## Best Practices Applied

### Bevy ECS Patterns

- **Change Detection**: Use `.changed()` filter to detect `BrickTypeId` mutations
- **Component Queries**: Query for `(Entity, &mut BrickTypeId, With<Brick>)` in collision system
- **System Ordering**: Multi-hit handling before despawn system (`despawn_marked_entities`)

### Performance Considerations

- No allocations in collision handler hot path
- Material lookup is O(1) via `TypeVariantRegistry` HashMap
- Change detection avoids unnecessary material updates

### WASM Compatibility

- No filesystem operations in game logic
- Level files already support indices 10-13 (embedded in WASM builds)
- Texture assets loaded via Bevy's asset system (WASM compatible)

## Open Questions (Resolved)

| Question | Resolution |
|----------|------------|
| Should transition animation be interpolated? | No, immediate visual swap per spec edge case |
| How to handle simultaneous multi-ball hits? | Each hit processes separately; multiple decrements per frame possible |
| Material assets available? | Assume will be provided; use fallback materials until ready |

## References

- `docs/bricks.md` - Brick type documentation (indices 10-13)
- `src/lib.rs` - Existing collision handling
- `src/systems/textures/` - Texture manifest and type variant system
