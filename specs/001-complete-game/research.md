# Research: Brkrs Complete Game

**Feature**: 001-complete-game
**Created**: 2025-10-31
**Purpose**: Architecture decisions and best practices for Bevy + Rapier3D
game development

## Overview

This document consolidates research findings for implementing an
Arkanoid/Breakout game using Bevy 0.16.0 and Rapier3D 0.31.0. All
decisions align with the project constitution's ECS-first, physics-driven,
and modular design principles.

## Key Architecture Decisions

### 1. ECS Component Organization

**Decision**: Organize components by entity type (paddle, ball, brick) rather
than by property type (transform, physics, rendering)

**Rationale**:

- Improves code locality and maintainability
- Aligns with Bevy's bundle pattern
- Makes it easier to locate all properties for a specific game object
- Reduces cross-file dependencies

**Alternatives Considered**:

- **Property-based organization** (rejected): Would scatter related logic
  across multiple files, making features harder to trace
- **Single components.rs file** (rejected): Would become too large with 37+
  brick types

**Implementation**:

```text
components/
├── paddle.rs     # Paddle marker, PaddleVelocity
├── ball.rs       # Ball marker, BallType (for speed limits)
├── brick.rs      # Brick, BrickType, Durability, BrickBehavior
├── border.rs     # Border marker
└── game_state.rs # Lives, Score, CurrentLevel
```

### 2. Physics Constraint Strategy

**Decision**: Use Rapier's `LockedAxes::TRANSLATION_LOCKED_Y` for all
gameplay objects to enforce 2D plane constraint at Y=2.0

**Rationale**:

- Physics engine enforces constraint automatically
- Prevents floating-point drift from accumulating
- More reliable than manual Y-position clamping
- Allows 3D rendering while maintaining 2D gameplay

**Alternatives Considered**:

- **Manual Y-clamping in systems** (rejected): Prone to drift, requires
  checking every frame
- **2D physics** (rejected): Loses 3D visual aesthetics, harder to migrate
  to full 3D later

**Implementation Details**:

- All `RigidBody` entities get `LockedAxes::TRANSLATION_LOCKED_Y`
- Camera positioned at Y=37.0 looking down
- 3D models render with lighting/shadows for depth perception

### 3. Ball "English" (Steering) Implementation

**Decision**: Apply impulse to ball based on paddle velocity at collision
moment, added to Rapier's natural collision response

- Maintains physics realism while adding gameplay depth
- Impulse magnitude can be adjusted via resource/config file

**Alternatives Considered**:

- **Direct velocity modification** (rejected): Fights physics engine,
  unpredictable interactions
- **Velocity multiplier** (rejected): Can cause unrealistic speed changes

**Implementation**:

```rust
// Pseudocode - actual implementation in systems/ball_physics.rs
fn on_paddle_collision(
) -> Vec3 {
    // Project paddle velocity onto collision plane
    let tangent_velocity = paddle_velocity -
        paddle_velocity.dot(collision_normal) * collision_normal;

    // Apply steering impulse

**Decision**: Use Rust enums for brick types with behavior trait pattern

- Compiler ensures all types handled in match statements
- Easy to add new brick types
- Behavior trait allows extensible collision handling

**Alternatives Considered**:

- **String-based types** (rejected): Runtime errors, no compile-time safety
- **Component composition only** (rejected): Harder to query specific brick
  types

**Implementation**:

```

```rust
// Pseudocode - actual implementation in components/brick.rs

## [derive(Component, Debug, Clone, Copy)]

pub enum BrickType {
    Standard,
    MultiHit { durability: u8 },
    SpeedUp,
    SpeedDown,
    Explosive { radius: f32 },
    // ... 32+ more types
}

trait BrickBehavior {
    fn on_collision(&self, ball: Entity, commands: &mut Commands);
}
```

### 5. Level Data Format

**Decision**: Use RON (Rusty Object Notation) for level definitions

**Rationale**:

- Native Rust serialization format
- Integrates seamlessly with serde

- **JSON** (rejected): Less Rust-native, verbose for nested data

**Level File Structure**:
// assets/levels/level_001.ron
Level(
    bricks: [
        Brick(pos: (0, 0), type: Standard),
        Brick(pos: (1, 0), type: MultiHit(durability: 2)),
        // ...

],

```text

## 6. Game State Management

**Decision**: Use Bevy's built-in `States` system with enum for game modes

**Rationale**:

- Leverages Bevy's state machine infrastructure
- Automatic system scheduling based on state
- Type-safe state transitions
- Integrates with Bevy's `OnEnter`/`OnExit` system sets

**Alternatives Considered**:

- **Custom state management** (rejected): Reinvents the wheel, loses Bevy
  integration
- **Boolean flags** (rejected): Error-prone, no compile-time guarantees

**Implementation**:

```

```rust
// Pseudocode - actual implementation in components/game_state.rs

## [derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub enum GameState {
    Menu,
    Playing,
    Paused,
    LevelTransition,
    GameOver,
    Victory,
}
```

### 7. Mouse Input Handling

**Decision**: Use `AccumulatedMouseMotion` and `AccumulatedMouseScroll`
resources with sensitivity scaling

**Rationale**:

- Bevy provides frame-independent accumulation
- Handles different mouse DPI settings
- Easy to scale for different window sizes
- Works consistently across platforms

**Alternatives Considered**:

- **Raw mouse events** (rejected): Frame-dependent, DPI issues
- **Mouse position absolute** (rejected): Requires manual delta calculation,
  cursor wrapping issues

**Implementation**:

- Scale factor: `sensitivity = 100.0 / window.height().min(window.width())`
- Cursor locked during gameplay for infinite movement
- ESC key unlocks cursor for menu access

### 8. WASM Optimization Strategy

**Decision**: Use `wasm-opt` and asset compression for WASM builds

**Rationale**:

- Reduces initial load time
- Improves mobile browser compatibility
- Standard WASM optimization practice
- Bevy has good WASM support with dynamic linking disabled

**Implementation**:

```toml

## Cargo.toml profile for WASM

[profile.wasm-release]
inherits = "release"
opt-level = "z"  # Optimize for size
lto = true
codegen-units = 1
```

**Asset Optimization**:

- Compress textures (WebP or compressed PNG)
- Optimize models (reduce poly count for bricks)
- Lazy-load levels (don't bundle all 77 upfront)

## Best Practices Integration

### Bevy ECS Patterns

1. **Plugin Architecture**: Group related systems into plugins for modularity
2. **System Ordering**: Use explicit `.after()` and `.before()` for
   deterministic execution
3. **Resource Management**: Use `Res` for read-only, `ResMut` for mutable
   access
4. **Event Communication**: Prefer events over direct entity manipulation for
   loose coupling

### Rapier3D Integration

1. **Collision Groups**: Use groups to filter unnecessary collision checks
2. **Collision Events**: Subscribe to `CollisionEvent` for ball-brick/paddle
   interactions
3. **CCD**: Enable for fast-moving objects (ball) to prevent tunneling
4. **Restitution/Friction**: Tune per-entity for desired gameplay feel

### Performance Optimization

1. **Change Detection**: Use Bevy's change detection (`Changed<T>`) to avoid
   unnecessary processing
2. **Sparse Sets**: For frequently added/removed components (like temporary
   effects)
3. **Parallel Systems**: Mark independent systems as parallel-safe
4. **Entity Pooling**: Reuse ball entities rather than spawn/despawn

## Open Questions (Resolved)

All technical clarifications from the planning phase have been resolved
through the decisions above. No blocking unknowns remain.

## Next Steps

Proceed to Phase 1:

1. Create data-model.md with complete ECS component definitions
2. Define event contracts in contracts/events.md
3. Write quickstart.md for build/run instructions
4. Update agent context with Bevy + Rapier3D specifics
