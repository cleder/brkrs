# Quickstart: Game States

**Feature**: 027-game-states **Audience**: Developers implementing or testing the game states feature **Last Updated**: 2026-02-08

## Overview

This feature implements a comprehensive game state management system with 7 states (Main Menu, Playing, Paused, Fade Out, Fade In, Level Transition, Game Over) controlled by message-based transitions.
It handles game flow, pause/resume, life loss with fade animations, level transitions, and main menu navigation.

---

## Prerequisites

- Rust 1.81+ with cargo
- Bevy 0.17.3 workspace already set up
- Familiarity with Bevy ECS and message systems
- TDD workflow: tests must be written and approved before implementation

---

## Setup (5 minutes)

### 1. Create Module Files

```bash
# Create game state module
touch src/game_state.rs

# Create state transition systems
touch src/systems/game_state_transitions.rs

# Create UI modules
mkdir -p src/systems/ui
touch src/systems/ui/main_menu.rs
touch src/systems/ui/game_over.rs
```

### 2. Register Module in lib.rs

```rust
// src/lib.rs
pub mod game_state;

// ... existing code ...
use game_state::GameStatesPlugin;

pub fn build_app() -> App {
    let mut app = App::new();
    // ... existing plugins ...
    app.add_plugins(GameStatesPlugin);
    app
}
```

### 3. Create Test Files (TDD Requirement)

```bash
# Create test files FIRST (TDD)
touch tests/game_state_transitions.rs
touch tests/life_loss_flow.rs
touch tests/pause_state.rs
touch tests/main_menu.rs
```

---

## Core Implementation (30 minutes)

### Step 1: Define GameState Resource

**File**: `src/game_state.rs`

```rust
use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

#[derive(Resource, Debug, Clone)]
pub struct GameSession {
    pub current_level: u32,
    pub lives_remaining: u32,
    pub score: u32,
}

impl Default for GameSession {
    fn default() -> Self {
        Self {
            current_level: 1,
            lives_remaining: 3,
            score: 0,
        }
    }
}
```

### Step 2: Define State Transition Message

**File**: `src/game_state.rs` (continued)

```rust
use bevy::ecs::event::Message;

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

### Step 3: Implement State Transition System

**File**: `src/systems/game_state_transitions.rs`

```rust
use bevy::prelude::*;
use crate::game_state::{GameState, StateTransitionRequest};

pub fn process_state_transitions(
    mut state: ResMut<GameState>,
    mut reader: MessageReader<StateTransitionRequest>,
) {
    for request in reader.read() {
        if is_valid_transition(&state, &request.target_state) {
            info!("State transition: {:?} -> {:?}", *state, request.target_state);
            *state = request.target_state;
        } else {
            warn!(
                "Invalid state transition: {:?} -> {:?}",
                *state, request.target_state
            );
        }
    }
}

fn is_valid_transition(current: &GameState, target: &GameState) -> bool {
    use GameState::*;
    matches!(
        (current, target),
        (MainMenu, Playing)
            | (Playing, Paused)
            | (Playing, FadeOut)
            | (Paused, Playing)
            | (FadeOut, FadeIn)
            | (FadeOut, GameOver)
            | (FadeIn, Playing)
            | (GameOver, MainMenu)
    )
}
```

### Step 4: Create Plugin

**File**: `src/game_state.rs` (add to end)

```rust
pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameState>()
            .init_resource::<GameSession>()
            .add_message::<StateTransitionRequest>()
            .add_systems(Update, (
                systems::game_state_transitions::process_state_transitions,
                // More systems added incrementally
            ));
    }
}
```

---

## Testing (15 minutes per test suite)

### Test 1: Basic State Transitions

**File**: `tests/game_state_transitions.rs`

```rust
use bevy::prelude::*;
use brkrs::{GameState, StateTransitionRequest};

#[test]
fn test_main_menu_to_playing_transition() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<GameState>()
        .add_message::<StateTransitionRequest>()
        .add_systems(Update, process_state_transitions);

    // Send transition request
    let mut writer = app.world_mut().resource_mut::<MessageWriter<StateTransitionRequest>>();
    writer.write(StateTransitionRequest {
        target_state: GameState::Playing,
        context: None,
    });

    // Update to process message
    app.update();

    // Verify state changed
    let state = app.world().resource::<GameState>();
    assert_eq!(*state, GameState::Playing);
}

#[test]
fn test_invalid_transition_rejected() {
    let mut app = App::new();
    // ... setup ...

    // Try invalid transition (Pause from MainMenu)
    let mut writer = app.world_mut().resource_mut::<MessageWriter<StateTransitionRequest>>();
    writer.write(StateTransitionRequest {
        target_state: GameState::Paused,
        context: None,
    });

    app.update();

    // Verify state unchanged
    let state = app.world().resource::<GameState>();
    assert_eq!(*state, GameState::MainMenu);
}
```

### Test 2: Multi-Frame Persistence

**File**: `tests/pause_state.rs`

```rust
#[test]
fn test_pause_state_persists_across_frames() {
    let mut app = App::new();
    // ... setup ...

    // Transition to Paused
    // ... send message ...
    app.update();

    // Run 10 more frames
    for _ in 0..10 {
        app.update();
    }

    // Verify still paused
    let state = app.world().resource::<GameState>();
    assert_eq!(*state, GameState::Paused);
}
```

---

## Running the Feature

### Development Mode

```bash
# Run with game states enabled
cargo run --features dev

# Expected: Main menu appears with New Game and Quit buttons
```

### Testing

```bash
# Run all tests
cargo test

# Run state-specific tests
cargo test game_state_transitions
cargo test life_loss_flow
cargo test pause_state
```

### Debugging

```bash
# Enable trace logging for state transitions
RUST_LOG=brkrs::systems::game_state_transitions=trace cargo run
```

---

## Common Tasks

### Add a New State

1. Add variant to `GameState` enum
2. Update `is_valid_transition()` function
3. Add transition messages for new state
4. Create systems that run conditionally in new state
5. Write tests for new state transitions

### Add State-Specific UI

1. Create UI spawn system with `run_if(in_state(GameState::YourState))`
2. Add `DespawnOnExit(GameState::YourState)` to root UI entity
3. Add system to plugin in `Update` schedule

### Pause Additional Systems

```rust
app.add_systems(Update, (
    your_system.run_if(in_state(GameState::Playing)),
));
```

---

## Troubleshooting

### "Invalid state transition" warnings

**Cause**: Sending transition request from invalid current state **Solution**: Check transition matrix in data-model.md, ensure current state allows target state

### State not changing

**Cause**: Message not being read (schedule ordering issue) **Solution**: Ensure `process_state_transitions` system is in Update schedule and MessageReader exists

### Entities not despawning on state exit

**Cause**: Missing `DespawnOnExit` component **Solution**: Add `DespawnOnExit(GameState::YourState)` component to entities that should cleanup

### Fade animation not smooth

**Cause**: Timer not ticking or alpha calculation incorrect **Solution**: Verify `Time` resource delta is being passed to `timer.tick()`, check alpha formula

---

## Integration Points

### With Physics System

```rust
app.add_systems(Update, (
    physics_step.run_if(in_state(GameState::Playing)),
));
```

### With Input System

```rust
app.add_systems(Update, (
    handle_pause_input
        .run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
));
```

### With Level Loading

```rust
fn transition_to_next_level(
    // ... params ...
    mut writer: MessageWriter<StateTransitionRequest>,
) {
    writer.write(StateTransitionRequest {
        target_state: GameState::FadeOut,
        context: Some(StateTransitionContext::LevelComplete { next_level: 5 }),
    });
}
```

---

## Performance Notes

- State transitions: O(1) enum assignment
- Message reading: O(N) where N = messages per frame (typically 0-2)
- UI updates: Only on state change (use `Changed<GameState>` filter)
- Entity cleanup: Automatic via Bevy's DespawnOnExit

**Expected Frame Time**: <1ms for all state management systems combined

---

## References

- [Feature Specification](spec.md)
- [Data Model](data-model.md)
- [State Transition Contracts](contracts/state-transitions.md)
- [Research Notes](research.md)
- Bevy Messages Documentation: <https://docs.rs/bevy/0.17.3/bevy/ecs/event/struct.MessageWriter.html>
- Constitution: `.specify/memory/constitution.md`

---

## Next Steps After Implementation

1. ✅ Write and commit failing tests (red phase)
2. ✅ Get test approval from feature owner
3. ✅ Implement systems to make tests pass (green phase)
4. ✅ Run `cargo test` to verify all tests pass
5. ✅ Run `cargo clippy` and `cargo fmt`
6. ✅ Test in native and WASM builds
7. ✅ Update main game loop to use GameStatesPlugin
8. ✅ Document any deviations or learnings in retrospective
