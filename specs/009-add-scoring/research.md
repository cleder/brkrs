# Phase 0 Research: Add Scoring System

**Date**: 16 December 2025 **Feature**: Add Scoring System **Branch**: 009-add-scoring

## Research Questions

### Q1: How do existing Bevy game systems track and update global state?

**Context**: Need to track cumulative score across levels within a game session.

**Findings**:

- Existing `LivesState` resource in `src/systems/respawn.rs` provides precedent for global game state
- Pattern: Define resource struct → `init_resource()` in plugin → systems mutate via `ResMut<T>`
- Resources persist across frame updates and system executions
- Change detection (`Res<T>` vs `ResMut<T>`) enables reactive UI updates

**Decision**: Use `ScoreState` resource following the `LivesState` pattern.

**Rationale**: Proven pattern in codebase; ECS-native; supports change detection for efficient UI updates.

**Alternatives Considered**:

- Component on a singleton entity: More complex queries, unnecessary indirection
- Event-based state: Stateless, would require rebuilding score from event history

---

### Q2: How should score values be mapped from brick types?

**Context**: docs/bricks.md contains point values for each brick index (10-57).
Need efficient lookup.

**Findings**:

- Brick types already have enum or identifier system in existing code
- Point values range from 25-300, with special cases (Question brick: random, Extra Ball: no points)
- Total of ~48 brick types with documented scores

**Decision**: Define `brick_points()` function mapping brick type/index to point value.
Use match expression for compile-time verification.

**Rationale**: Simple, type-safe, performant (compiled to jump table).
Centralizes point values for maintainability.

**Alternatives Considered**:

- Runtime hashmap: Slower, unnecessary allocation
- Store points in brick components: Duplication, harder to maintain consistency with docs

---

### Q3: How to detect brick destruction events for score accumulation?

**Context**: Need to know when bricks are destroyed to award points.

**Findings**:

- Existing brick destruction logic emits events or despawns entities
- Similar pattern in `detect_ball_loss` system that listens to collision events
- Bevy's message system (used for `LifeLostEvent`, `GameOverRequested`) provides event communication

**Decision**: Define `BrickDestroyed` message with brick type/entity info.
Emit from brick destruction system, consume in scoring system.

**Rationale**: Follows established messaging pattern; decouples brick logic from scoring; event-driven updates match performance goals.

**Alternatives Considered**:

- Poll for despawned brick entities: Inefficient, violates performance-first principle
- Direct function call: Tight coupling, harder to test, breaks modularity

---

### Q4: How to implement milestone detection for ball spawns at 5000-point intervals?

**Context**: Every 5000 points, the player gets an extra ball (life).

**Findings**:

- Existing `LivesState` resource in `src/systems/respawn.rs` tracks remaining balls
- Ball award logic: increment `LivesState.lives_remaining` counter
- Milestone check: track `last_milestone_reached` in `ScoreState`, compare to `current_score / 5000`

**Decision**: Track last milestone in `ScoreState`.
When `current_score / 5000 > last_milestone_reached`, emit `MilestoneReached` message.
Lives system listens and increments `LivesState.lives_remaining`.

**Rationale**: Decouples scoring from ball/lives management; reuses existing lives counter infrastructure; supports multiple milestones naturally.
Extra ball is awarded through lives system, not physical ball spawn.

**Alternatives Considered**:

- Direct ball spawning: Violates separation of concerns, creates unnecessary entities
- Modify ball spawn logic: Creates coupling, harder to test milestone system independently

---

### Q5: How to implement real-time score display UI?

**Context**: Score must be visible at all times, update within one frame (<16ms at 60 FPS).

**Findings**:

- Existing `LivesCounterUi` component in `src/ui/lives_counter.rs` provides UI pattern
- Bevy UI uses entity with `TextBundle` component
- Change detection on `ScoreState` resource triggers UI updates efficiently
- Orbitron font already used for consistent styling

**Decision**: Create `ScoreDisplayUi` marker component.
Spawn text entity at startup.
Update system queries `(Query<&mut Text, With<ScoreDisplayUi>>, Res<ScoreState>)` with change detection filter.

**Rationale**: Mirrors proven lives counter pattern; change detection prevents unnecessary updates; meets performance criteria.

**Alternatives Considered**:

- Update every frame: Wastes CPU, violates performance-first principle
- Custom rendering: Overly complex for simple numeric display

---

### Q6: How to handle Question brick (index 53) random score (25-300 points)?

**Context**: Question brick awards random points in specified range.

**Findings**:

- Bevy provides `Res<GlobalRng>` for deterministic randomness
- Standard library `rand` crate integrated with Bevy's RNG system
- Uniform distribution available via `rng.gen_range(25..=300)`

**Decision**: Use `GlobalRng` resource with `gen_range(25..=300)` when Question brick destroyed.

**Rationale**: Bevy-native RNG; deterministic for testing; uniform distribution matches spec.

**Alternatives Considered**:

- External RNG crate: Unnecessary dependency
- Fixed value: Contradicts "random" requirement

---

## Summary of Decisions

| Decision | Implementation Approach |
|----------|------------------------|
| Score storage | `ScoreState` resource (u32 score, u32 last_milestone) |
| Brick point mapping | `brick_points(BrickType) -> u32` function with match expression |
| Destruction detection | `BrickDestroyed` message emitted by brick systems |
| Milestone detection | Compare `score / 5000 > last_milestone`, emit `MilestoneReached` message |
| Ball award logic | Increment `LivesState.lives_remaining` on `MilestoneReached` event |

---

## Integration Points

1. **Brick destruction systems**: Add `BrickDestroyed` message emission
2. **Ball/Lives award logic**: Add `MilestoneReached` message handler to increment `LivesState`
3. **Level transition**: Preserve `ScoreState` across level changes
4. **Game restart**: Reset `ScoreState` to initial values

---

## Performance Considerations

- **Score updates**: O(1) addition operation
- **Milestone detection**: O(1) integer division check
- **UI updates**: Change detection prevents unnecessary renders
- **Memory**: Two u32 fields in global resource (~8 bytes)
- **No allocations**: All operations use stack memory

---

## Testing Strategy

- Unit tests: Score accumulation, milestone detection, random range validation
- Integration tests: Brick destruction → score increase, milestone → ball spawn
- Manual verification: Play level, destroy bricks, observe score display and ball spawns
