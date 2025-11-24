# Data Model: Brkrs Complete Game

**Feature**: 001-complete-game
**Created**: 2025-10-31
**Purpose**: ECS component and entity definitions

## Overview

This document defines all ECS components, entities, resources, and their
relationships for the Brkrs game. The design follows Bevy's ECS paradigm
with physics-driven gameplay using Rapier3D.

## Entity Archetypes

### Paddle Entity

**Purpose**: Player-controlled object that deflects the ball

**Components**:

```rust
// Marker component
#[derive(Component)]
pub struct Paddle;

// Bundle for spawning
#[derive(Bundle)]
pub struct PaddleBundle {
    paddle: Paddle,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    // Rapier3D physics
    rigid_body: RigidBody,              // KinematicPositionBased
    collider: Collider,                 // Capsule
    locked_axes: LockedAxes,            // TRANSLATION_LOCKED_Y
    friction: Friction,
    controller: KinematicCharacterController,
    ccd: Ccd,                           // Continuous collision detection
    // Collision tracking
    colliding_entities: CollidingEntities,
}
```

**Validation Rules**:

- Transform.translation.y MUST equal 2.0
- Rotation limited to ±45 degrees on Y-axis
- Position constrained within play area boundaries

**State Transitions**: None (always active during gameplay)

### Ball Entity

**Purpose**: Moving object that destroys bricks and must be kept in play

**Components**:

```rust
// Marker component
#[derive(Component)]
pub struct Ball;

// Ball type affects speed limits
#[derive(Component, Clone, Copy)]
pub enum BallType {
    Standard,
    GolfBall,    // High speed limit
    BeachBall,   // Low speed limit
}

// Bundle for spawning
#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    ball_type: BallType,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    // Rapier3D physics
    rigid_body: RigidBody,              // Dynamic
    collider: Collider,                 // Ball
    restitution: Restitution,           // Bounciness
    friction: Friction,
    damping: Damping,                   // Velocity decay
    locked_axes: LockedAxes,            // TRANSLATION_LOCKED_Y
    ccd: Ccd,                           // Prevent tunneling
    external_impulse: ExternalImpulse,  // For "english" effect
    gravity_scale: GravityScale,        // Set to 1.0 for horizontal "gravity"
    // Collision tracking
    colliding_entities: CollidingEntities,
    active_events: ActiveEvents,        // COLLISION_EVENTS
}
```

**Validation Rules**:

- Transform.translation.y MUST equal 2.0
- Velocity magnitude limited by BallType
  - Standard: max 15 units/s
  - GolfBall: max 25 units/s
  - BeachBall: max 10 units/s

**State Transitions**:

- In play → Lost (when Y < lower boundary)
- Normal → Respawning (after life lost)

### Brick Entity

**Purpose**: Destructible obstacles arranged in grid

**Components**:

```rust
// Marker component
#[derive(Component)]
pub struct Brick;

// Brick type determines behavior
#[derive(Component, Clone, Copy, Debug)]
pub enum BrickType {
    Standard,
    MultiHit { durability: u8 },
    SpeedUp,
    SpeedDown,
    Explosive { radius: f32 },
    Indestructible,
    Regenerating { timer: f32 },
    Teleporter { target: GridPos },
    // ... 30+ more types as needed
}

// Grid position for spatial queries
#[derive(Component, Clone, Copy)]
pub struct GridPosition {
    pub x: u8,  // 0-21
    pub y: u8,  // 0-21
}

// Current durability for multi-hit bricks
#[derive(Component)]
pub struct Durability(pub u8);

// Bundle for spawning
#[derive(Bundle)]
pub struct BrickBundle {
    brick: Brick,
    brick_type: BrickType,
    grid_pos: GridPosition,
    durability: Durability,  // Optional, depends on type
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    // Rapier3D physics
    collider: Collider,     // Cuboid (static)
    // Some bricks may have sensors instead
}
```

**Validation Rules**:

- GridPosition x, y MUST be in range 0-21
- Transform position derived from GridPosition
- Durability MUST match BrickType requirements
- Indestructible bricks never despawn

**State Transitions**:

- Active → Damaged (multi-hit, durability decreases)
- Active → Destroyed (removed from world)
- Destroyed → Regenerating (for regenerating type)
- Regenerating → Active (after timer expires)

### Border/Wall Entity

**Purpose**: Play area boundaries that reflect the ball

**Components**:

```rust
// Marker component
#[derive(Component)]
pub struct Border;

// Border type distinguishes walls from goal
#[derive(Component, Clone, Copy)]
pub enum BorderType {
    UpperWall,   // Ball reflects
    LeftWall,    // Ball reflects
    RightWall,   // Ball reflects
    LowerGoal,   // Ball lost (life decreases)
}

// Bundle for spawning
#[derive(Bundle)]
pub struct BorderBundle {
    border: Border,
    border_type: BorderType,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    // Rapier3D physics
    collider: Collider,  // Cuboid (static)
}
```

**Validation Rules**:

- Borders MUST fully enclose play area
- LowerGoal may have zero-width collider (visual only)

**State Transitions**: None (static)

## Global Resources

### GameConfig

**Purpose**: Physics tuning parameters and game constants

```rust
#[derive(Resource)]
pub struct GameConfig {
    // Grid layout
    pub grid_width: u8,          // 22
    pub grid_height: u8,         // 22
    pub cell_size: f32,          // World units per cell

    // Physics tuning
    pub gravity: Vec3,           // Horizontal "gravity" for ball
    pub paddle_speed: f32,       // Mouse sensitivity multiplier
    pub steering_factor: f32,    // "English" strength
    pub ball_restitution: f32,   // Bounciness
    pub paddle_friction: f32,    // Grip on ball

    // Gameplay
    pub starting_lives: u8,      // Default 3
    pub respawn_delay: f32,      // Seconds before ball respawns
}
```

### CurrentLevel

**Purpose**: Tracks active level state

```rust
#[derive(Resource)]
pub struct CurrentLevel {
    pub number: u8,              // 1-77
    pub bricks_remaining: usize, // Destructible bricks left
    pub total_bricks: usize,     // Original count for this level
}
```

### Lives

**Purpose**: Player's remaining attempts

```rust
#[derive(Resource)]
pub struct Lives {
    pub count: u8,
    pub max: u8,
}
```

### Score

**Purpose**: Player's accumulated points

```rust
#[derive(Resource)]
pub struct Score(pub u32);
```

### LevelDefinitions

**Purpose**: Cached level data loaded from assets

```rust
#[derive(Resource)]
pub struct LevelDefinitions {
    pub levels: Vec<LevelData>,
}

#[derive(Deserialize, Clone)]
pub struct LevelData {
    pub number: u8,
    pub bricks: Vec<BrickPlacement>,
}

#[derive(Deserialize, Clone)]
pub struct BrickPlacement {
    pub grid_pos: GridPosition,
    pub brick_type: BrickType,
}
```

## Component Relationships

### Spatial Queries

```text
GridPosition → Transform
  - Grid coordinates map to world positions
  - Conversion: world_x = (grid_x - 11) * cell_size
  - Allows efficient spatial lookups for brick neighbors

Transform → GridPosition (reverse)
  - Used for collision detection results
  - Determines which grid cell contains a point
```

### Physics Relationships

```text
Collider → CollidingEntities
  - Rapier maintains list of current collisions
  - Used to detect ball-paddle, ball-brick contacts

RigidBody → ExternalImpulse
  - Systems apply impulses to influence physics
  - Used for "english" effect and special brick behaviors

KinematicCharacterController → Transform
  - Controller modifies transform based on input
  - Used for smooth paddle movement
```

### Gameplay Relationships

```text
Ball → Lives
  - When ball exits lower boundary, Lives decreases
  - Game over when Lives reaches 0

Brick → CurrentLevel.bricks_remaining
  - Destroying brick decrements counter
  - Level complete when counter reaches 0

BrickType → Durability
  - MultiHit bricks have durability component
  - Other types may not have this component
```

## State Flow Diagrams

### Ball Lifecycle

```text
[Spawned] → [In Play] → [Colliding] → [In Play]
                ↓
         [Below Boundary]
                ↓
           [Despawned]
                ↓
     (Lives decreased)
                ↓
         [Respawned] → [In Play]
```

### Brick Lifecycle

```text
[Spawned] → [Active] → [Hit by Ball] → [Damaged] → [Active]
                                            ↓
                                    (Durability = 0)
                                            ↓
                                      [Destroyed]
                                            ↓
                                      [Despawned]
```

### Level Progression

```text
[Level N Loaded] → [Bricks Spawned] → [Playing]
                                           ↓
                                (All bricks destroyed)
                                           ↓
                                  [Level Transition]
                                           ↓
                                  [Level N+1 Loaded]
```

## Query Patterns

Common ECS queries used in systems:

```rust
// Paddle control system
Query<(&mut KinematicCharacterController, &mut Transform), With<Paddle>>

// Ball physics system
Query<(&mut ExternalImpulse, &Transform, &BallType), With<Ball>>

// Brick collision system
Query<(Entity, &BrickType, &mut Durability, &GridPosition), With<Brick>>

// Collision detection
Query<&KinematicCharacterControllerOutput, With<Paddle>>
Query<Entity, (With<Ball>, With<CollidingEntities>)>

// Border checking
Query<(&BorderType, &Transform), With<Border>>

// UI display
Query<&Transform, With<Ball>>  // For camera following
Res<Lives>, Res<Score>, Res<CurrentLevel>  // For HUD
```

## Next Steps

With the data model defined, proceed to:

1. Create contracts/events.md for event definitions
2. Write quickstart.md for development workflow
3. Update agent context with component/system patterns
