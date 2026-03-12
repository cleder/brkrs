# Architecture

This document provides an overview of the brkrs game architecture.

## High-Level Design

brkrs is built on the [Bevy](https://bevyengine.org/) game engine using an Entity-Component-System (ECS) architecture with Rapier3D for physics simulation.

```text
┌─────────────────────────────────────────────────────────────┐
│                      Game Application                        │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │
│  │  Input   │  │  Level   │  │ Physics  │  │   UI     │    │
│  │  System  │  │  Loader  │  │ (Rapier) │  │  System  │    │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘    │
│       │             │             │             │           │
│       └─────────────┴──────┬──────┴─────────────┘           │
│                            │                                 │
│                    ┌───────┴───────┐                        │
│                    │  Bevy ECS     │                        │
│                    │  (Entities,   │                        │
│                    │   Components, │                        │
│                    │   Resources)  │                        │
│                    └───────────────┘                        │
└─────────────────────────────────────────────────────────────┘
```

## Core Principles

The project follows these architectural principles (from the project Constitution):

1. **ECS-First**: All game logic uses Bevy's ECS paradigm
2. **Physics-Driven**: Gameplay relies on Rapier3D for collisions and movement
3. **Modular Features**: Each feature is independently testable
4. **Performance-First**: 60 FPS target on native and WASM
5. **Cross-Platform**: Supports native (Linux/Windows/macOS) and WASM
6. **Rustdoc**: Public APIs are documented (intent-focused)
7. **TDD-First**: Tests are written first, fail (red), and are approved before implementation
8. **Bevy 0.17 mandates**: Follow Bevy 0.17 ECS/graphics/performance rules and prohibitions

## System Overview

### Core Systems

| System | Purpose | Location |
|--------|---------|----------|
| Level Loader | Parses RON files, spawns entities | `src/level_loader.rs` |
| Pause System | Freezes physics, shows overlay | `src/pause.rs` |
| Respawn | Ball respawn after loss | `src/systems/respawn.rs` |
| Level Switch | Transitions between levels | `src/systems/level_switch.rs` |
| Scoring | Tracks points, awards milestone bonuses | `src/systems/scoring.rs` |
| Audio System | Plays sound effects for collisions, level transitions, milestones | `src/systems/audio.rs` |
| Cheat Mode | Developer/testing feature for quick level exploration | `src/systems/cheat_mode.rs` |
| Paddle Size | Handles paddle size powerup effects (shrink/enlarge) | `src/systems/paddle_size.rs` |
| Multi-Hit Bricks | Manages multi-hit brick durability and transitions | `src/systems/multi_hit.rs` |
| Textures | Loads and manages textures, per-level material overrides | `src/systems/textures/` |
| Grid Debug | Development visualization | `src/systems/grid_debug.rs` |
| Spawning | Initial scene setup (camera, light, ground) | `src/systems/spawning.rs` |

### Component Structure

Entities use centralized physics configuration resources instead of hardcoded values:

```text
Entity: Ball
├── Transform
├── RigidBody (Dynamic)
├── Collider (Sphere)
├── Velocity
├── Restitution (from BallPhysicsConfig.restitution)
├── Friction (from BallPhysicsConfig.friction)
├── Damping (from BallPhysicsConfig.linear_damping/angular_damping)
└── Ball (marker component)

Entity: Paddle
├── Transform
├── RigidBody (Kinematic)
├── Collider (Box)
├── Restitution (from PaddlePhysicsConfig.restitution)
├── Friction (from PaddlePhysicsConfig.friction)
└── Paddle (marker component)

Entity: Brick
├── Transform
├── RigidBody (Fixed)
├── Collider (Box)
├── Restitution (from BrickPhysicsConfig.restitution)
├── Friction (from BrickPhysicsConfig.friction)
├── Brick (marker component)
└── [Optional] Indestructible

Entity: Merkaba (Hazard)
├── Transform
├── RigidBody (Dynamic)
├── Collider (Cylinder)
├── CollisionGroups (Group 2, ALL)
├── SolverGroups (Group 2, ALL ^ Group 1)
├── Velocity
└── Merkaba (marker component)
```

**Physics Configuration Resources:**

- `BallPhysicsConfig` — Ball physics properties (restitution, friction, damping)
- `PaddlePhysicsConfig` — Paddle physics properties
- `BrickPhysicsConfig` — Brick physics properties

All configs include validation methods to ensure physics values are reasonable and prevent runtime errors.

### State Machine

```text
        ┌─────────────┐
        │   Menu      │ (planned)
        └──────┬──────┘
               │ Start Game
               ▼
        ┌─────────────┐
   ┌───►│   Playing   │◄───┐
   │    └──────┬──────┘    │
   │           │ ESC       │ Click
   │           ▼           │
   │    ┌─────────────┐    │
   │    │   Paused    │────┘
   │    └─────────────┘
   │
   │    Level Complete
   └────────────────────
```

## Physics Architecture

### Coordinate System

**Bevy Convention**: Right-handed Y-up coordinate system

- **Y-axis**: Vertical (up/down), locked for gameplay entities
- **X-axis**: Lateral movement (left/right)
- **Z-axis**: Forward/backward movement (+Z toward camera, -Z into screen)

**Game Implementation**: Top-down view with movement on XZ plane

- Camera positioned at Y=37 looking down at origin
- Entities use `LockedAxes::TRANSLATION_LOCKED_Y` to constrain movement to XZ plane
- From player perspective: +Z = forward (toward goal/bricks), -Z = backward (toward paddle)
- Gameplay "forward" refers to +Z direction, not Bevy's `Transform::forward()` API (-Z)

### Plane Constraint

All gameplay occurs on the XZ horizontal plane at Y=2.0:

- Entities use `LockedAxes::TRANSLATION_LOCKED_Y`
- Camera positioned above at Y=37, looking down
- 3D rendering provides depth and shadows

### Collision Handling

```text
Ball ──collision──► Brick
                      │
                      ▼
              Check Indestructible?
                   /     \
                  No      Yes
                  │        │
                  ▼        ▼
              Destroy   Bounce
              Brick     Only
```

### Paddle Physics

- Mouse movement controls paddle position
- Mouse scroll rotates the paddle
- Recent mouse velocity applies "english" to ball on contact

## Level System

### Loading Flow

```text
assets/levels/level_001.ron
         │
         ▼
   LevelDefinition
   (RON parsing)
         │
         ▼
   Entity Spawning
   (per cell in matrix)
         │
         ▼
   Physics Setup
   (colliders, rigid bodies)
```

### Level Transitions

1. Clear current level entities (except camera, UI)
2. Parse new level file
3. Spawn new entities
4. Reset ball/paddle positions if needed

## Game State Resources

### Life Loss System

The life loss system uses a two-stage event architecture to decouple ball detection from life determination:

```text
Ball Lost Event Sources:
  - Ball hits LowerGoal (bottom boundary)
  - Paddle collides with Merkaba (hazard enemy)
  - Paddle touches Hazard Brick (types 42/91)
         |
         v
  detect_ball_loss / collision handlers
         |
         v
   BallLostEvent (message)
    - ball: Entity
    - cause: LifeLossCause (LowerGoal | MerkabaCollision | PaddleHazard)
    - ball_spawn: SpawnTransform
         |
         v
   determine_life_loss (system)
         |
         +---> Check cause:
         |       - LowerGoal: Only if remaining_balls == 0
         |       - Merkaba/PaddleHazard: Always
         |
         v (conditionally)
   LifeLostEvent (message)
         |
         +---> handle_life_loss_events (game state)
         |       - Triggers FadeOut transition
         |
         +---> enqueue_respawn_requests
               - Decrements LivesState.lives_remaining
               - Queues respawn sequence
               - Emits GameOverRequested if lives == 0
```

**Key Rules:**

- **LowerGoal losses**: Only trigger life loss when the **last ball** is lost (multi-ball support)
- **Merkaba/PaddleHazard**: **Always** trigger life loss, regardless of remaining balls
- **Source of Truth**: `LivesState.lives_remaining` (u8) maintained by respawn system
- **Game State Sync**: `GameSession.lives_remaining` (u32) synced from LivesState during state transitions
- **New Game Reset**: Starting New Game from Game Over resets `GameSession`, `LivesState`, and `ScoreState` to baseline values (level 1, 3 lives, score 0)

### Scoring System

The scoring system tracks cumulative points throughout a game session:

```text
Brick Destroyed
      |
      v
BrickDestroyed (message)
      |
      v
award_points_system
      |
      v
ScoreState (resource)
   - current_score: u32
   - last_milestone_reached: u32
      |
      v
detect_milestone_system
      |
      v (if milestone crossed)
MilestoneReached (message)
      |
      v
award_milestone_ball_system
      |
      v
LivesState.lives_remaining += 1
```

**Point Values**: Defined in `docs/bricks.md`, ranging from 25-300 points per brick

**Milestones**: Every 5000 points awards an extra life

**Special Cases**:

- Question brick (53): Random 25-300 points
- Extra Ball brick (41): 0 points (grants life via separate mechanism)
- Magnet bricks (55-56): 0 points (effect-only)

**Persistence**: Score accumulates across level transitions, resets when starting a New Game

### Messages vs Observers (Bevy 0.17+)

See the constitution's "Bevy 0.17 Event, Message, and Observer Clarification" for authoritative guidance.

- **Messages** (`#[derive(Message)]`) are for double-buffered, frame-agnostic data streams (e.g., scoring, telemetry).
  Produced via `MessageWriter`, consumed via `MessageReader`.
  Use for work that can be batched or delayed to the next schedule step.
  Not for immediate side-effects.
- **Observers** (with `#[derive(Event)]`, `On<T>`, `Trigger<T>`, or observer systems) are for immediate or next-frame reactions (e.g., UI, sound, spawning).
  Use for real-time, reactive logic that needs full system access and instant feedback.

**Key rules:**

- Use Messages for batchable, cross-frame work; Observers for instant, reactive logic.
- Never create observer systems that listen to Messages; only Events/Triggers are valid for observers.
- Always justify your choice in specs/plans (see constitution for rationale and examples).

## UI System

### Pause Overlay

The pause overlay is a separate UI layer that:

- Appears on ESC press
- Freezes physics simulation
- Shows resume instruction
- Dismisses on mouse click

### Respawn System

The respawn system manages ball loss detection, life management, and respawn sequencing through ordered system sets:

**System Sets (executed in order):**

1. **Detect** — Ball loss detection and life determination
   - `detect_ball_loss`: Detects ball collisions with LowerGoal, emits BallLostEvent
   - `determine_life_loss`: Checks remaining balls, emits LifeLostEvent conditionally
   - `life_loss_logging`: Logs life loss events
   - `apply_paddle_shrink`: Applies visual feedback (paddle shrink animation)

2. **Schedule** — Respawn scheduling and queue management
   - `enqueue_respawn_requests`: Decrements lives, queues respawn
   - `process_respawn_queue`: Schedules timed respawn
   - `log_respawn_scheduled`: Logs respawn events

3. **Execute** — Respawn execution
   - `respawn_executor`: Spawns new ball at scheduled time

4. **Visual** — Visual effects
   - `respawn_visual_trigger`: Triggers respawn overlay
   - `animate_respawn_visual`: Animates fadeout overlay

5. **Control** — Input control restoration
   - `restore_paddle_control`: Unlocks paddle input after respawn

**Lives Management:**

- `LivesState.lives_remaining` (u8) — Authoritative life counter
- `GameSession.lives_remaining` (u32) — Synced during game state transitions
- Starting lives: 3
- Maximum lives: 5 (clamped)
- Milestone bonuses: +1 life every 5000 points

### State Handling

```text
GameState::Playing
    │ ESC
    ▼
GameState::Paused
    │
    ├── Hide cursor (native)
    ├── Freeze physics
    └── Show overlay
    │
    │ Click
    ▼
GameState::Playing
    │
    ├── Show cursor
    ├── Resume physics
    └── Hide overlay
```

## Cross-Platform Considerations

### WASM Differences

| Feature | Native | WASM |
|---------|--------|------|
| Window mode switching | ✓ Fullscreen toggle | ✗ Not supported |
| Audio | Full support | Web Audio API |
| File I/O | Direct | Embedded assets |
| Performance | Full speed | ~60-80% native |

### Asset Embedding

For WASM builds, assets are embedded at compile time.
The build process:

1. Compiles to `wasm32-unknown-unknown` target
2. Runs `wasm-bindgen` for JS interop
3. Assets bundled into the WASM binary

## Further Reading

- [Bevy Book](https://bevyengine.org/learn/book/introduction/)
- [Rapier Physics Documentation](https://rapier.rs/docs/)
- {doc}`developer-guide` for development setup
- {doc}`contributing` for code contribution guidelines
