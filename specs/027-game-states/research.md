# Research: Game States

**Feature**: 027-game-states **Date**: 2026-02-08 **Purpose**: Resolve technical unknowns and establish implementation approach

## Research Questions

### 1. Bevy State Management Best Practices

**Question**: What is the recommended pattern for implementing game state machines in Bevy 0.17?

**Answer**:

- **Resource-based State**: Use a `Resource` enum to represent the current game state
- **States derive**: Use Bevy's built-in States system with `NextState<GameState>` for transitions
- **State-scoped Systems**: Use `run_if(in_state(GameState::Playing))` predicates to conditionally run systems
- **Entity Cleanup**: Use `DespawnOnExit(state)` component for automatic entity cleanup on state transitions

**Decision**: Implement `GameState` using Bevy's built-in `States` derive with `NextState<GameState>` for transitions and `OnEnter`/`OnExit` schedules for state-specific setup/cleanup.

**Rationale**: Bevy's States system is the idiomatic way to handle app state in 0.17.
It provides built-in transition management, schedule integration (`OnEnter`, `OnExit`), and run conditions (`in_state`).
This eliminates need for custom transition handling and integrates seamlessly with Bevy's scheduling.

---

### 2. Fade Animation Implementation

**Question**: How should fade-out/fade-in animations be implemented without blocking state transitions?

**Answer**:

- **Timer Resource**: Use `Timer` component or resource to track fade animation progress
- **Opacity Manipulation**: Modify UI overlay entity's `BackgroundColor` alpha channel
- **State Transition on Complete**: Set `NextState` when timer completes
- **Non-blocking**: Animation runs in parallel with state machine checks (timer just tracks progress)

**Decision**: Create a `FadeOverlay` entity with `Timer` component.
Update alpha channel each frame.
When timer completes, set the next state via `NextState<GameState>`.

---

### 3. Physics Freeze During Pause

**Question**: What's the cleanest way to freeze physics simulation when paused?

**Answer**:

- **Option A**: Manually store all `Velocity` components and set to zero, restore on resume
- **Option B**: Use Rapier's time scaling feature to set `RapierConfiguration.timestep_mode` to paused
- **Option C**: Use Bevy's `run_if` system conditions to skip physics systems when paused

**Decision**: Option C - Use `run_if(in_state(GameState::Playing))` condition on physics update systems.
Maintains all entity state automatically.

**Alternative Considered**: Option B was considered but requires Rapier-specific configuration and couples state management to physics engine internals.

---

### 4. Lives Check Timing & State Branching

**Question**: How should the Fade Out state branch to either Game Over or Fade In based on lives?
**Answer**:

- **Timer Completion System**: System that reads fade timer and checks lives when timer completes
- **Conditional NextState**: If lives > 0, set `NextState(FadeIn)`; if lives == 0, set `NextState(GameOver)`
- **Single Branching Point**: One system handles the lives check logic after fade animation completes
**Rationale**: Centralizes branching logic in one place.
Clear separation of concerns (timer tracking vs. decision logic).
Follows States-based architecture.

---

### 5. Invalid Transition Logging

**Question**: What's the best pattern for logging invalid state transitions as warnings?

**Answer**:

- **Match Pattern**: Use match on `(current_state, requested_transition)` tuple
**Decision**: Implement `is_valid_transition(current: &GameState, next: &GameState) -> bool` helper function that returns validity.
Log warning and skip transition if invalid.

**Rationale**: Explicit validation function makes transition logic testable.
`tracing::warn!` provides structured logging with context.
Early return pattern is idiomatic Rust.

---

### 6. Main Menu UI Implementation

**Question**: How should the main menu UI be implemented with minimal complexity?

**Answer**:

- **Bevy UI**: Use Bevy's built-in UI system with `Node`, `Button`, `Text` components
- **Interaction System**: System that queries `Interaction` component changes on buttons
- **NextState Setting**: Button click handlers set `NextState` for transitions
- **Entity Hierarchy**: Root `Node` with two child `Button` entities (New Game, Quit)

**Decision**: Use Bevy UI with two button entities.
System detects `Changed<Interaction>` and sets `NextState(GameState::Playing)` or sends `AppExit` event.

**Rationale**: Bevy UI is built-in, well-tested, and sufficient for simple menus.
`Interaction` component provides button state tracking.
Minimal external dependencies.

---

## Technology Choices

| Component | Choice | Alternatives Considered | Rationale |
|-----------|--------|------------------------|-----------|
| State Storage | States derive | Resource enum, component-based | Built-in state machine, integrates with schedules and run conditions |
| State Transitions | NextState<GameState> | Observers, direct mutation | Idiomatic Bevy approach for app state changes |
| Pause Mechanism | run_if conditions | Manual velocity storage, time scaling | Cleanest Bevy-idiomatic approach; automatic state preservation |
| Fade Animation | Timer + opacity | Custom shader, external animation lib | Simple, performant, built-in Bevy features |
| UI Framework | Bevy UI | egui, custom rendering | Built-in, sufficient for minimal menu, fewer dependencies |

## Best Practices Summary

### Bevy States-Based State Machine

```rust
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    FadeOut,
    FadeIn,
    LevelTransition,
    GameOver,
}

// Transition using NextState resource
fn request_state_transition(
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
    target: GameState,
) {
    if is_valid_transition(current_state.get(), &target) {
        next_state.set(target);
    } else {
        warn!("Invalid transition: {:?} -> {:?}", current_state.get(), target);
    }
}
```

### Conditional System Execution & State Schedules

```rust
// Using run_if conditions
app.add_systems(Update, (
    physics_update_system.run_if(in_state(GameState::Playing)),
    input_system.run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
));

// Using OnEnter/OnExit schedules for state transitions
app.add_systems(OnEnter(GameState::FadeOut), spawn_fade_overlay);
app.add_systems(OnExit(GameState::FadeOut), check_lives_and_transition);
app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu);
app.add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
```

### Fade Animation Pattern

```rust
) {
    for (mut color, mut timer) in query.iter_mut() {
        timer.tick(time.delta());
        let progress = timer.fraction();
        color.0.set_alpha(progress); // Fade in: 0.0 -> 1.0
    }
}
```

## Open Questions

None remaining.
All technical unknowns resolved.

## References

- Bevy 0.17 documentation: <https://docs.rs/bevy/0.17.3>
- Bevy UI guide: <https://bevyengine.org/learn/book/getting-started/ui/>
- Bevy states system docs: <https://docs.rs/bevy/0.17.3/bevy/ecs/schedule/trait.States.html>
- Constitution: `.specify/memory/constitution.md`
