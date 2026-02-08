# State Transition Contracts

**Feature**: 027-game-states **Date**: 2026-02-08 **Purpose**: Define state transition API and system contracts

## State Transition API

### NextState Resource

**Type**: Bevy built-in state transition mechanism

**Definition**:

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

// Optional context resource
#[derive(Resource, Debug, Clone, Copy)]
pub enum StateTransitionContext {
    LifeLoss,
    LevelComplete { next_level: u32 },
    NewGame,
    ReturnToMenu,
}
```

**Triggered By**:

- UI button handlers (using `ResMut<NextState<GameState>>`)
- Gameplay event systems (ball lost, level complete)
- Input handlers (pause/resume)

**Timing**: State transitions occur immediately at the end of the current frame

**Example Usage**:

```rust
// From button click handler
fn handle_new_game_button(
    query: Query<&Interaction, (Changed<Interaction>, With<NewGameButtonMarker>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            commands.insert_resource(StateTransitionContext::NewGame);
            next_state.set(GameState::Playing);
        }
    }
}
```

---

## State Transition Semantics

### Valid Transitions

| From State | To State | Context Required | Side Effects |
|------------|----------|------------------|--------------|
| MainMenu | Playing | NewGame | Initialize GameSession, spawn level |
| Playing | Paused | None | Freeze physics, show pause UI |
| Paused | Playing | None | Resume physics, hide pause UI |
| Playing | FadeOut | LifeLoss or LevelComplete | Spawn FadeOverlay, start timer |
| FadeOut | FadeIn | None (automatic) | Check lives, respawn ball if lives>0, despawn old level if level complete |
| FadeOut | GameOver | None (automatic) | Only if lives==0 after check |
| FadeOut | FadeIn | None (automatic) | Only for level complete flow |
| FadeIn | Playing | None (automatic) | Despawn FadeOverlay, enable gameplay |
| GameOver | MainMenu | ReturnToMenu | Reset GameSession |

### Invalid Transitions (Logged as Warnings)

- Pause from any state except Playing
- Any transition to FadeOut except from Playing
- Any transition to FadeIn except from FadeOut completion
- Direct Playing→GameOver (must go through FadeOut)
- Direct GameOver→Playing (must go through MainMenu)

---

## State Machine Guarantees

### Atomicity

- State changes occur within 1 frame of message processing
- No partial state transitions (state changes are atomic)

### Idempotence

- Requesting transition to current state is no-op (logged as info)
- Multiple identical requests in same frame handled as single transition

### Consistency

- State resource always in valid state (one of 7 enum variants)
- Invalid transitions rejected with warning logs
- Side effects (entity spawn/despawn, physics enable/disable) always synchronized with state

### Ordering

- Messages processed in FIFO order within a frame
- Later messages override earlier ones if both valid

---

## System Contracts

### validate_state_transition (helper function)

**Schedule**: Called by systems before using `NextState` **Runs**: On-demand **Reads**:

- `State<GameState>` (current state)
- Target state parameter

**Returns**: `bool` (whether transition is valid)

**Guarantees**:

- Validates transition against allowed transitions matrix
- Logs invalid transitions as warnings
- Returns false for invalid transitions

**Example**:

```rust
fn validate_state_transition(
    current: &GameState,
    target: &GameState,
) -> bool {
    use GameState::*;
    let valid = matches!(
        (current, target),
        (MainMenu, Playing)
            | (Playing, Paused)
            | (Playing, FadeOut)
            | (Paused, Playing)
            | (FadeOut, FadeIn)
            | (FadeOut, GameOver)
            | (FadeIn, Playing)
            | (GameOver, MainMenu)
    );

    if !valid {
        warn!("Invalid transition: {:?} -> {:?}", current, target);
    }
    valid
}
```

**Preconditions**: None **Postconditions**: Logs warning if invalid

---

### check_fade_out_completion

**Schedule**: OnExit(GameState::FadeOut) **Runs**: When exiting FadeOut state (after fade animation completes) **Reads**:

- `Option<Res<StateTransitionContext>>` (transition context)
- `Res<GameSession>` (lives count)

**Writes**:

- `ResMut<NextState<GameState>>` (next state)
- `Commands` (to remove context resource)

**Guarantees**:

- Checks transition context to determine next state
- If context is LifeLoss:
  - If lives > 0: transitions to FadeIn
  - If lives == 0: transitions to GameOver
- If context is LevelComplete: transitions to FadeIn
- Removes context resource after consuming

**Preconditions**: Called automatically by Bevy when exiting FadeOut state **Postconditions**: NextState set appropriately, context resource removed

---

### update_fade_overlay

**Schedule**: Update **Runs**: When GameState is FadeOut or FadeIn **Reads**:

- `Time` resource (delta time)
- `Query<(&mut BackgroundColor, &mut FadeTimer, &FadeDirection)>`

**Writes**:

- `BackgroundColor.0` (alpha channel)
- `FadeTimer` (tick progress)

**Guarantees**:

- Ticks fade timer by delta time
- Updates alpha: FadeOut (0→1), FadeIn (1→0)
- Alpha calculated as timer.fraction() or 1.0 - timer.fraction()

**Preconditions**: FadeOverlay entity exists **Postconditions**: Alpha updated smoothly over timer duration

---

### handle_main_menu_buttons

**Schedule**: Update **Runs**: When GameState == MainMenu (using `run_if(in_state(GameState::MainMenu))`) **Reads**:

- `Query<&Interaction, (Changed<Interaction>, With<NewGameButtonMarker>)>`
- `Query<&Interaction, (Changed<Interaction>, With<QuitButtonMarker>)>`

**Writes**:

- `ResMut<NextState<GameState>>` (for New Game)
- `EventWriter<AppExit>` (for Quit)
- `Commands` (to set context resource)

**Guarantees**:

- Detects button click (Interaction::Pressed)
- Sets NextState or sends AppExit event
- Sets transition context for New Game

**Preconditions**: Button entities exist in MainMenu state **Postconditions**: State transition initiated on click

---

## Error Handling

### Invalid Transition Requests

**Behavior**: Log warning, maintain current state

**Example Log**:

```text
WARN game_state_transitions: Invalid state transition requested: Paused -> FadeOut (not allowed)
```

### Missing Entities

**Behavior**: Systems with entity queries skip gracefully (early return if query is empty)

**Example**:

```rust
fn check_fade_out_completion(
    fade_query: Query<&FadeTimer>,
    // ...
) {
    let Some(timer) = fade_query.iter().next() else {
        return; // No fade overlay, skip
    };
    // ...
}
```

### Resource Unavailability

**Behavior**: Use `Option<Res<T>>` or `Option<ResMut<T>>`, return early if None

---

## Testing Contracts

### Message Delivery

**Test**: Send StateTransitionRequest, verify received in next frame **Assertion**: `reader.read().count() == 1`

### Transition Validation

**Test**: Send invalid transition (e.g., Pause from MainMenu) **Assertion**: `state.0 == GameState::MainMenu` (unchanged) + warning logged

### Side Effect Synchronization

**Test**: Transition to FadeOut, verify FadeOverlay entity exists **Assertion**: `fade_query.iter().count() == 1`

### Multi-Frame Persistence

**Test**: Set GameState::Paused, run 10 frames, verify state unchanged **Assertion**: `state.0 == GameState::Paused` after 10 app.update() calls

### Life Loss Branching

**Test 1**: lives=2, FadeOut complete → verify FadeIn message sent **Test 2**: lives=0, FadeOut complete → verify GameOver message sent

---

## Performance Contracts

- State transition processing: O(N) where N = number of transition messages in frame (typically 0-2)
- Fade animation: O(M) where M = number of fade overlay entities (always 0 or 1)
- Button interaction: O(B) where B = number of buttons (typically 2 in MainMenu)
- Entity cleanup: O(E) where E = number of DespawnOnExit entities (handled by Bevy automatically)

**Frame Budget**: All state transition systems combined must complete within 1ms to maintain 60 FPS **Memory**: StateTransitionRequest messages buffered until read (typically <10 per frame)

---

## Versioning

**Version 1.0**: Initial implementation **Breaking Changes**: Any change to StateTransitionRequest message structure or transition matrix **Backwards Compatibility**: Not applicable (internal game systems)
