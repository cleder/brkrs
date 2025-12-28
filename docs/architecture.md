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
| Grid Debug | Development visualization | `src/systems/grid_debug.rs` |
| Spawning | Initial scene setup (camera, light, ground) | `src/systems/spawning.rs` |

### Component Structure

```text
Entity: Ball
├── Transform
├── RigidBody (Dynamic)
├── Collider (Sphere)
├── Velocity
└── Ball (marker component)

Entity: Paddle
├── Transform
├── RigidBody (Kinematic)
├── Collider (Box)
└── Paddle (marker component)

Entity: Brick
├── Transform
├── RigidBody (Fixed)
├── Collider (Box)
├── Brick (marker component)
└── [Optional] Indestructible
```

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

### Plane Constraint

All gameplay occurs on a 2D plane at Y=2.0:

- Entities use `LockedAxes::TRANSLATION_LOCKED_Y`
- Camera positioned above, looking down
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

**Persistence**: Score accumulates across level transitions, resets on game restart

**Messages vs Observers (Bevy 0.17+)**

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
