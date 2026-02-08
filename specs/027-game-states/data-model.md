# Data Model: Game States

**Feature**: 027-game-states **Date**: 2026-02-08 **Purpose**: Define entities, components, and resources for state management

## Entities

### 1. FadeOverlay

**Purpose**: Visual overlay for fade-in/fade-out animations

**Components**:

- `Node` - UI layout positioning (full-screen)
- `BackgroundColor` - Black color with variable alpha for fade effect
- `FadeTimer` - Custom component tracking fade animation progress
- `FadeDirection` - Custom component indicating fade-in vs fade-out
- `DespawnOnExit(GameState::FadeOut)` or `DespawnOnExit(GameState::FadeIn)` - Automatic cleanup

**Lifecycle**:

- Spawned when entering FadeOut or FadeIn states
- Despawned automatically when exiting those states
- Alpha channel animated from 0.0→1.0 (fade out) or 1.0→0.0 (fade in)

**Relationships**: None (standalone UI entity)

---

### 2. MainMenuRoot

**Purpose**: Root UI node for main menu screen

**Components**:

- `Node` - UI layout container (centered, column layout)
- `DespawnOnExit(GameState::MainMenu)` - Cleanup when leaving main menu

**Children**:

- `NewGameButton` entity
- `QuitButton` entity

**Lifecycle**:

- Spawned on entering MainMenu state
- Despawned when transitioning to Playing

**Relationships**: Parent of button entities

---

### 3. NewGameButton

**Purpose**: Button to start new game

**Components**:

- `Button` - Bevy UI button interaction
- `Node` - Layout properties
- `BackgroundColor` - Visual styling
- `Interaction` - Tracks hover/click state (changed detection)
- `NewGameButtonMarker` - Custom marker component for query filtering

**Children**: `Text` entity with "New Game" label

**Lifecycle**: Spawned/despawned with MainMenuRoot parent

**Relationships**: Child of MainMenuRoot

---

### 4. QuitButton

**Purpose**: Button to exit application

**Components**:

- `Button` - Bevy UI button interaction
- `Node` - Layout properties
- `BackgroundColor` - Visual styling
- `Interaction` - Tracks hover/click state
- `QuitButtonMarker` - Custom marker component

**Children**: `Text` entity with "Quit" label

**Lifecycle**: Spawned/despawned with MainMenuRoot parent

**Relationships**: Child of MainMenuRoot

---

### 5. GameOverRoot

**Purpose**: UI container for game over screen

**Components**:

- `Node` - UI layout container
- `DespawnOnExit(GameState::GameOver)` - Cleanup

**Children**: Text displaying "Game Over" and optional buttons

**Lifecycle**: Spawned when entering GameOver state

**Relationships**: Parent of game over UI elements

---

## Components

### FadeTimer

```rust
#[derive(Component)]
pub struct FadeTimer {
    pub timer: Timer,
}

impl FadeTimer {
    pub fn new(duration_secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration_secs, TimerMode::Once),
        }
    }

    pub fn tick(&mut self, delta: Duration) -> &Self {
        self.timer.tick(delta);
        self
    }

    pub fn finished(&self) -> bool {
        self.timer.finished()
    }

    pub fn fraction(&self) -> f32 {
        self.timer.fraction()
    }
}
```

**Purpose**: Tracks progress of fade animations **Validation**: Duration must be 0.5-1.0 seconds (per spec) **State Transitions**: None (simple timer wrapper)

---

### FadeDirection

```rust
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FadeDirection {
    In,   // 1.0 -> 0.0 alpha (fade to transparent)
    Out,  // 0.0 -> 1.0 alpha (fade to opaque)
}
```

**Purpose**: Indicates whether fade is in or out **Validation**: Must be one of two enum variants **State Transitions**: Set when spawning FadeOverlay entity

---

### NewGameButtonMarker / QuitButtonMarker

```rust
#[derive(Component)]
pub struct NewGameButtonMarker;

#[derive(Component)]
pub struct QuitButtonMarker;
```

**Purpose**: Marker components for efficient button queries **Validation**: None (zero-sized types) **State Transitions**: None

---

## Resources

### GameState

```rust
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Playing,
    Paused,
    FadeOut,
    FadeIn,
    LevelTransition,
    GameOver,
}

impl Default for GameState {
    fn default() -> Self {
        Self::MainMenu
    }
}
```

**Purpose**: Tracks current game state **Validation**: Must be one of 7 valid states **State Transitions**: Changed via state transition messages **Lifecycle**: Initialized at app startup, persists throughout runtime

---

### GameSession

```rust
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

**Purpose**: Tracks persistent game session data across state transitions **Validation**:

- `lives_remaining` >= 0 (checked during life loss flow)
- `current_level` >= 1
**State Transitions**:
- Lives decremented during FadeOut → FadeIn transition
- Level incremented during level completion
- Reset to default when starting new game from MainMenu
**Lifecycle**: Initialized at app startup, mutated during gameplay

---

## Messages

### StateTransitionRequest

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

**Purpose**: Request state transition from any system **Validation**: Target state must be reachable from current state (validated in transition system) **Sent By**:

- Button click handlers (MainMenu → Playing, GameOver → MainMenu)
- Ball lost handler (Playing → FadeOut with LifeLoss context)
- Level complete handler (Playing → FadeOut with LevelComplete context)
- Pause/resume input handler (Playing ↔ Paused)
**Read By**: `process_state_transitions` system

---

## State Transition Matrix

| Current State | Valid Next States | Trigger |
|---------------|-------------------|---------|
| MainMenu | Playing | NewGame button |
| MainMenu | [Exit] | Quit button |
| Playing | Paused | Pause input |
| Playing | FadeOut | Ball lost or level complete |
| Paused | Playing | Resume input |
| FadeOut | FadeIn | Timer complete + lives > 0 + LifeLoss context |
| FadeOut | GameOver | Timer complete + lives == 0 + LifeLoss context |
| FadeOut | FadeIn | Timer complete + LevelComplete context |
| FadeIn | Playing | Timer complete |
| GameOver | MainMenu | Return to menu / New game button |

**Invalid Transitions** (logged as warnings):

- Pause from any state except Playing
- FadeOut/FadeIn directly from Paused
- Playing directly from GameOver (must go through MainMenu)

---

## Data Flow

### Life Loss Flow

```text
Playing State (lives=2)
  ↓ (ball lost message)
FadeOut State
  ↓ (fade timer updates, alpha 0→1)
Timer Complete
  ↓ (check lives: 2 > 0)
FadeIn State (lives=1, ball respawned)
  ↓ (fade timer updates, alpha 1→0)
Timer Complete
  ↓
Playing State (lives=1)
```

### Game Over Flow

```text
Playing State (lives=1)
  ↓ (ball lost message)
FadeOut State
  ↓ (fade timer updates)
Timer Complete
  ↓ (check lives: 0 == 0)
GameOver State
  ↓ (user input)
MainMenu State
```

### Level Complete Flow

```text
Playing State (level=N)
  ↓ (all bricks destroyed)
FadeOut State
  ↓ (fade timer completes)
LevelTransition State (load level N+1)
  ↓
FadeIn State
  ↓ (fade timer completes)
Playing State (level=N+1)
```

---

## Validation Rules

1. **State Transitions**: Only valid transitions from transition matrix are allowed
2. **Lives Check**: Must occur after FadeOut timer completes, not before
3. **Entity Cleanup**: All entities with `DespawnOnExit(state)` must be removed when exiting that state
4. **Timer Duration**: Fade timers must be 0.5-1.0 seconds
5. **UI Spawning**: Main menu and game over UI entities only spawned in corresponding states
6. **Message Idempotence**: Receiving same transition message twice has no effect (already in target state)

---

## Performance Considerations

- **State checks**: O(1) enum comparison
- **Entity queries**: Minimal with marker components (`With<NewGameButtonMarker>`)
- **UI updates**: Only on `Changed<GameState>`, not every frame
- **Message buffering**: MessageReader handles batching automatically
- **Entity cleanup**: Automatic via DespawnOnExit, no manual iteration

---

## Testing Strategy

- **State Transitions**: Test all valid and invalid transitions
- **Multi-Frame Persistence**: Verify GameState persists across 10+ frames without unwanted changes
- **Life Loss**: Test both lives>0 (FadeIn) and lives==0 (GameOver) branches
- **Entity Lifecycle**: Verify entities spawn/despawn correctly with state changes
- **UI Interaction**: Test button clicks send correct messages
- **Timer Accuracy**: Verify fade animations complete within spec'd duration
