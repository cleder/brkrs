# State Transition Contracts

**Feature**: 027-game-states **Date**: 2026-02-08 **Purpose**: Define message contracts and state transition API

## Message API

### StateTransitionRequest

**Message Type**: Buffered message (MessageWriter/MessageReader)

**Definition**:

```rust
#[derive(Message, Debug, Clone, Copy)]
pub struct StateTransitionRequest {
    pub target_state: GameState,
    pub context: Option<StateTransitionContext>,
}

#[derive(Debug, Clone, Copy)]
pub enum StateTransitionContext {
    LifeLoss,
    LevelComplete { next_level: u32 },
    NewGame,
    ReturnToMenu,
}
```

**Sent By**:

- UI button handlers
- Gameplay event systems (ball lost, level complete)
- Input handlers (pause/resume)

**Consumed By**: `process_state_transitions` system in Update schedule

**Timing**: Read in the frame after sending (buffered)

**Example Usage**:

```rust
// From button click handler
fn handle_new_game_button(
    query: Query<&Interaction, (Changed<Interaction>, With<NewGameButtonMarker>)>,
    mut writer: MessageWriter<StateTransitionRequest>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            writer.write(StateTransitionRequest {
                target_state: GameState::Playing,
                context: Some(StateTransitionContext::NewGame),
            });
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

### process_state_transitions

**Schedule**: Update **Runs**: Every frame **Reads**:

- `MessageReader<StateTransitionRequest>`
- `GameState` resource (current state)
- `GameSession` resource (for validation)

**Writes**:

- `GameState` resource (new state)
- Logging (warnings for invalid transitions)

**Guarantees**:

- Processes all messages buffered in current frame
- Validates each transition before applying
- Logs invalid transitions as warnings
- Updates GameState atomically

**Preconditions**: GameState resource must exist (initialized at startup) **Postconditions**: GameState reflects last valid transition request

---

### check_fade_out_completion

**Schedule**: Update **Runs**: Only when GameState == FadeOut **Reads**:

- `Query<&FadeTimer>` (fade overlay timer)
- `GameState` resource
- `GameSession` resource (lives count)

**Writes**:

- `MessageWriter<StateTransitionRequest>` (next transition)

**Guarantees**:

- Checks if fade timer has completed
- If complete and context is LifeLoss:
  - If lives > 0: sends FadeIn transition
  - If lives == 0: sends GameOver transition
- If complete and context is LevelComplete: sends FadeIn transition

**Preconditions**: FadeOverlay entity with FadeTimer must exist in FadeOut state **Postconditions**: Transition message sent when timer completes

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

**Schedule**: Update **Runs**: When GameState == MainMenu **Reads**:

- `Query<&Interaction, (Changed<Interaction>, With<NewGameButtonMarker>)>`
- `Query<&Interaction, (Changed<Interaction>, With<QuitButtonMarker>)>`

**Writes**:

- `MessageWriter<StateTransitionRequest>` (for New Game)
- `EventWriter<AppExit>` (for Quit)

**Guarantees**:

- Detects button click (Interaction::Pressed)
- Sends appropriate message or event

**Preconditions**: Button entities exist in MainMenu state **Postconditions**: Transition message sent on click

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
