# Research: Game States

**Feature**: 027-game-states **Date**: 2026-02-08 **Purpose**: Resolve technical unknowns and establish implementation approach

## Research Questions

### 1. Bevy State Management Best Practices

**Question**: What is the recommended pattern for implementing game state machines in Bevy 0.17?

**Answer**:

- **Resource-based State**: Use a `Resource` enum to represent the current game state
- **Message-driven Transitions**: State changes triggered by `MessageWriter<StateTransitionRequest>`
- **State-scoped Systems**: Use `run_if(in_state(GameState::Playing))` predicates to conditionally run systems
- **Entity Cleanup**: Use `DespawnOnExit(state)` component for automatic entity cleanup on state transitions

**Decision**: Implement `GameState` as a `Resource` enum with message-driven transitions and conditional system execution based on current state.

**Rationale**: This pattern is idiomatic Bevy 0.17, provides clear state machine semantics, and integrates well with Bevy's scheduling system.
Resource-based state avoids archetype thrashing from component insertion/removal.

---

### 2. Fade Animation Implementation

**Question**: How should fade-out/fade-in animations be implemented without blocking state transitions?

**Answer**:

- **Timer Resource**: Use `Timer` component or resource to track fade animation progress
- **Opacity Manipulation**: Modify UI overlay entity's `BackgroundColor` alpha channel
- **State Transition on Complete**: Send state transition message when timer completes
- **Non-blocking**: Animation runs in parallel with state machine checks (timer just tracks progress)

**Decision**: Create a `FadeOverlay` entity with `Timer` component.
Update alpha channel each frame.
When timer completes, send next state transition message.

**Rationale**: Timers are idiomatic Bevy for time-based animations.
Opacity manipulation is performant and visually clear.
Non-blocking approach maintains frame rate.

---

### 3. Physics Freeze During Pause

**Question**: What's the cleanest way to freeze physics simulation when paused?

**Answer**:

- **Option A**: Manually store all `Velocity` components and set to zero, restore on resume
- **Option B**: Use Rapier's time scaling feature to set `RapierConfiguration.timestep_mode` to paused
- **Option C**: Use Bevy's `run_if` system conditions to skip physics systems when paused

**Decision**: Option C - Use `run_if(in_state(GameState::Playing))` condition on physics update systems.

**Rationale**: This is the cleanest Bevy-idiomatic approach.
No manual state saving/restoration required.
Physics systems simply don't run when not in Playing state.
Maintains all entity state automatically.

**Alternative Considered**: Option B was considered but requires Rapier-specific configuration and couples state management to physics engine internals.

---

### 4. Lives Check Timing & State Branching

**Question**: How should the Fade Out state branch to either Game Over or Fade In based on lives?

**Answer**:

- **Timer Completion System**: System that reads fade timer and checks lives when timer completes
- **Conditional Message**: If lives > 0, send `FadeIn` transition message; if lives == 0, send `GameOver` transition message
- **Single Branching Point**: One system handles the lives check logic after fade animation completes

**Decision**: Create `check_fade_out_completion` system that runs when `GameState == FadeOut`, checks timer completion, reads lives resource, and sends appropriate transition message.

**Rationale**: Centralizes branching logic in one place.
Clear separation of concerns (timer tracking vs. decision logic).
Follows message-based architecture.

---

### 5. Invalid Transition Logging

**Question**: What's the best pattern for logging invalid state transitions as warnings?

**Answer**:

- **Match Pattern**: Use match on `(current_state, requested_transition)` tuple
- **Valid Transitions Table**: Define valid transitions as const or static data structure
- **Logging**: Use `warn!()` macro from `tracing` crate for invalid transitions
- **Early Return**: Return early after logging (don't change state)

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
- **Message Sending**: Button click handlers send state transition messages
- **Entity Hierarchy**: Root `Node` with two child `Button` entities (New Game, Quit)

**Decision**: Use Bevy UI with two button entities.
System detects `Changed<Interaction>` and sends `StateTransitionRequest::ToPlaying` or `AppExit` event.

**Rationale**: Bevy UI is built-in, well-tested, and sufficient for simple menus.
`Interaction` component provides button state tracking.
Minimal external dependencies.

---

## Technology Choices

| Component | Choice | Alternatives Considered | Rationale |
|-----------|--------|------------------------|-----------|
| State Storage | Resource enum | Component-based, States plugin | Resource avoids archetype thrashing; enum provides type safety |
| State Transitions | MessageWriter/MessageReader | Observers, direct mutation | Messages provide buffering and testability; spec requires messages |
| Pause Mechanism | run_if conditions | Manual velocity storage, time scaling | Cleanest Bevy-idiomatic approach; automatic state preservation |
| Fade Animation | Timer + opacity | Custom shader, external animation lib | Simple, performant, built-in Bevy features |
| UI Framework | Bevy UI | egui, custom rendering | Built-in, sufficient for minimal menu, fewer dependencies |

## Best Practices Summary

### Message-Based State Machine

```rust
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    Playing,
    Paused,
    FadeOut,
    FadeIn,
    LevelTransition,
    GameOver,
}

#[derive(Message)]
pub struct StateTransitionRequest {
    pub target_state: GameState,
}

fn process_state_transitions(
    mut state: ResMut<GameState>,
    mut transitions: MessageReader<StateTransitionRequest>,
) {
    for transition in transitions.read() {
        if is_valid_transition(&state, &transition.target_state) {
            *state = transition.target_state;
        } else {
            warn!("Invalid transition: {:?} -> {:?}", *state, transition.target_state);
        }
    }
}
```

### Conditional System Execution

```rust
app.add_systems(Update, (
    physics_update_system.run_if(in_state(GameState::Playing)),
    input_system.run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
    ui_update_system.run_if(resource_changed::<GameState>),
));
```

### Fade Animation Pattern

```rust
fn update_fade_overlay(
    time: Res<Time>,
    mut query: Query<(&mut BackgroundColor, &mut FadeTimer)>,
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
- Message system RFC: (Bevy 0.17 changelog)
- Constitution: `.specify/memory/constitution.md`
