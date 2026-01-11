# Events Schema: Gravity Switching Bricks

**Feature**: 020-gravity-bricks | **Date**: 2026-01-10 | **Phase**: 1 Design

## Message Flow Diagram

```text
┌─────────────────────────────────────────────────────────────────────────┐
│                         GRAVITY BRICK LIFECYCLE                          │
└─────────────────────────────────────────────────────────────────────────┘

LEVEL START
    │
    ├─ Load LevelDefinition from RON
    │   └─ Extract gravity field (or use Vec3::ZERO fallback)
    │
    ├─ Create GravityConfiguration resource
    │   ├─ current = level_default
    │   └─ level_default = extracted value
    │
    └─ Spawn brick entities
        └─ For indices 21-25: attach GravityBrick component with gravity value

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

GAMEPLAY - GRAVITY BRICK DESTRUCTION
    │
    ├─ Ball collides with gravity brick (21-25)
    │
    ├─ brick_destruction_system (Schedule: PostUpdate)
    │   ├─ Detect destroyed brick via change detection
    │   ├─ Query: entities with Brick component + BrickDestroyed marker
    │   ├─ Check if index is in [21, 25]
    │   │
    │   ├─ Read GravityBrick component
    │   │   ├─ For indices 21-24: use pre-defined gravity from component
    │   │   └─ For index 25 (Queer): generate random gravity using rand crate
    │   │       └─ X: uniform random [-2.0, +15.0]
    │   │       └─ Y: 0.0
    │   │       └─ Z: uniform random [-5.0, +5.0]
    │   │
    │   ├─ Create GravityChanged message
    │   │   └─ GravityChanged { gravity: Vec3 }
    │   │
    │   └─ Write message via MessageWriter<GravityChanged>
    │       └─ Message enters buffered queue
    │
    ├─ [Next schedule phase or same frame]
    │
    ├─ gravity_application_system (Schedule: PhysicsUpdate or similar)
    │   ├─ Read messages via MessageReader<GravityChanged>
    │   ├─ For each received GravityChanged message:
    │   │   └─ Update GravityConfiguration::current = gravity
    │   │
    │   └─ Apply gravity to ball physics
    │       └─ Modify ball's Rapier GravityScale or apply direct force
    │
    └─ [Next physics frame]
        └─ Ball falls/floats with new gravity

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

GAMEPLAY - BALL LOSS / LIFE RESET
    │
    ├─ Ball collides with bottom boundary or deadly brick
    │
    ├─ ball_lives_system (detects life loss)
    │   ├─ Decrement lives count
    │   └─ Trigger life loss event/message
    │
    ├─ gravity_reset_on_life_loss_system (Schedule: PostUpdate or Listen to event)
    │   ├─ Detect ball loss signal
    │   │
    │   ├─ Query GravityConfiguration resource
    │   │
    │   ├─ Reset gravity to level default
    │   │   └─ GravityConfiguration::current = GravityConfiguration::level_default
    │   │
    │   └─ [Optional: Send GravityChanged message with default value for logging]
    │
    ├─ Despawn current ball entity
    │
    └─ [Next ball spawn]
        ├─ Create new ball entity
        └─ Ball physics uses GravityConfiguration::current (now reset to default)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

LEVEL COMPLETE / NEW LEVEL
    │
    └─ Unload level, reload GravityConfiguration for new level (repeat from LEVEL START)

```

## Message Queue Dynamics

### Single Gravity Brick Destroyed

```text
Frame N:
  ├─ brick_destruction_system runs
  │   └─ Writes: GravityChanged { gravity: (0.0, 10.0, 0.0) }
  │       └─ Message enters queue: [(0.0, 10.0, 0.0)]
  │
  └─ End of frame

Frame N+1:
  ├─ gravity_application_system runs
  │   ├─ Reads from MessageReader (cursor = 0)
  │   ├─ Reads: GravityChanged { gravity: (0.0, 10.0, 0.0) }
  │   ├─ Updates: GravityConfiguration::current = (0.0, 10.0, 0.0)
  │   └─ Cursor advances (message consumed)
  │
  └─ Queue now empty
```

### Multiple Gravity Bricks Destroyed (Rapid Succession)

```text
Frame N:
  ├─ Ball destroys gravity brick #1 (Zero Gravity)
  │   └─ brick_destruction_system writes: GravityChanged { gravity: (0.0, 0.0, 0.0) }
  │       └─ Queue: [(0.0, 0.0, 0.0)]
  │
  └─ [Still Frame N] Ball immediately hits gravity brick #2 (10G)
      └─ brick_destruction_system writes: GravityChanged { gravity: (0.0, 10.0, 0.0) }
          └─ Queue: [(0.0, 0.0, 0.0), (0.0, 10.0, 0.0)]

Frame N+1:
  ├─ gravity_application_system runs
  │   ├─ Reads both messages in order
  │   ├─ Processes: GravityChanged { gravity: (0.0, 0.0, 0.0) }
  │   │   └─ Updates: GravityConfiguration::current = (0.0, 0.0, 0.0)
  │   │
  │   ├─ Processes: GravityChanged { gravity: (0.0, 10.0, 0.0) }
  │   │   └─ Updates: GravityConfiguration::current = (0.0, 10.0, 0.0)
  │   │
  │   └─ Final state: GravityConfiguration::current = (0.0, 10.0, 0.0) ✓
  │
  └─ Physics frame applies final gravity

Result: Last gravity change wins (deterministic, correct behavior)
```

## Event Integration Points

### Existing Events Used

| Event | Source System | Consumer | Purpose |
|-------|---------------|----------|---------|
| `BrickDestroyed` or similar | `brick_destruction` | `gravity_brick_handler` | Detect gravity brick destruction |
| Ball loss/life decrement | `ball_lives` | `gravity_reset_on_life_loss` | Detect when to reset gravity |
| Physics update | Rapier schedule | Implicit gravity application | Apply gravity force to ball |

### New Messages Introduced

| Message | Writer | Reader | Purpose |
|---------|--------|--------|---------|
| `GravityChanged` | `brick_destruction_system` | `gravity_application_system` | Communicate gravity updates to physics |

### No New Observers Required

Gravity mechanics do not require `Trigger<T>` or Observers.
Messages are sufficient for:

- Buffered, frame-agnostic gravity updates
- Deterministic ordering (first in, first processed)
- Decoupling brick destruction from physics application
- Compatibility with physics simulation schedule

## System Execution Order

```text
Schedule: Update (or PostUpdate)
  │
  ├─ [Other systems]
  │
  ├─ brick_destruction_system
  │   ├─ Input: Brick components, BrickDestroyed marker, GravityBrick component
  │   └─ Output: MessageWriter<GravityChanged>
  │
  └─ [Other systems]

Schedule: PhysicsUpdate (or next frame)
  │
  ├─ gravity_application_system
  │   ├─ Input: MessageReader<GravityChanged>, GravityConfiguration resource
  │   └─ Output: Updated GravityConfiguration::current
  │
  ├─ Rapier physics simulation
  │   ├─ Input: GravityConfiguration::current (used implicitly)
  │   └─ Output: Updated ball velocity/position
  │
  └─ [Other physics systems]

Schedule: PostUpdate (or when life loss detected)
  │
  └─ gravity_reset_on_life_loss_system
      ├─ Input: Life loss event/signal, GravityConfiguration resource
      └─ Output: Reset GravityConfiguration::current = level_default
```

## Data Flow Summary

```text
┌─────────────────────┐
│  LevelDefinition    │  ← Loaded from RON
│  gravity            │
└──────────┬──────────┘
           │
           ↓
┌─────────────────────────────────────────┐
│     GravityConfiguration Resource       │
│  ┌─────────────────────────────────────┐│
│  │ current: Vec3                       ││  ← Updated by gravity_application_system
│  │ level_default: Vec3  ← from level   ││
│  └─────────────────────────────────────┘│
└──────────┬──────────────────────────────┘
           │
           ├─ Read by: Physics simulation
           │
           └─ Written by:
               ├─ gravity_application_system (on GravityChanged message)
               └─ gravity_reset_on_life_loss_system (on life loss)

┌──────────────────────┐
│  Brick Entities      │
│  (index 21-25)       │
│  ┌────────────────┐  │
│  │ GravityBrick   │  │ ← Read by: brick_destruction_system
│  │ component      │  │
│  └────────────────┘  │
└──────────┬───────────┘
           │
           ↓
┌─────────────────────────────────┐
│  brick_destruction_system       │
│  ┌──────────────────────────────┤
│  │ Detect: gravity brick dies   │
│  │ Read: GravityBrick.gravity   │
│  │ (or generate RNG for 25)     │
│  └──────────────────────────────┤
└─────────────────┬───────────────┘
                  │
                  ↓
           ┌──────────────────┐
           │ GravityChanged   │
           │ Message          │
           │ ┌────────────────┤
           │ │ gravity: Vec3  │
           │ └────────────────┤
           └──────────┬───────┘
                      │
                      ↓ (buffered queue)
           ┌──────────────────────────────────┐
           │ gravity_application_system       │
           │ ┌───────────────────────────────┐│
           │ │ Read: MessageReader<...>      ││
           │ │ Update: GravityConfiguration  ││
           │ │         ::current             ││
           │ └───────────────────────────────┘│
           └──────────┬───────────────────────┘
                      │
                      ↓
           ┌──────────────────────────────────┐
           │  Next Physics Frame              │
           │  Ball physics updated with       │
           │  new gravity vector              │
           └──────────────────────────────────┘
```

---

## Test Scenarios

### Scenario 1: Single Gravity Brick (Happy Path)

**Input**: Level with one gravity brick (index 23, 10G) **Expected**:

1. Gravity brick spawned with `GravityBrick { index: 23, gravity: (0.0, 10.0, 0.0) }`
2. Ball destroys brick
3. `GravityChanged { gravity: (0.0, 10.0, 0.0) }` message sent
4. `GravityConfiguration::current` updated to `(0.0, 10.0, 0.0)`
5. Ball falls at 10G acceleration next frame

### Scenario 2: Sequential Gravity Changes

**Input**: Level with three gravity bricks (21, 24, 22) destroyed in sequence **Expected**:

1. Destroy brick 21 → gravity becomes `(0.0, 0.0, 0.0)` (zero)
2. Destroy brick 24 → gravity becomes `(0.0, 20.0, 0.0)` (high)
3. Destroy brick 22 → gravity becomes `(0.0, 2.0, 0.0)` (light)
4. No state corruption; physics remains stable

### Scenario 3: Gravity Reset on Ball Loss

**Input**: Level with gravity brick 21 (zero gravity), default gravity `(0.0, 10.0, 0.0)` **Expected**:

1. Ball destroys gravity brick → gravity becomes `(0.0, 0.0, 0.0)`
2. Ball is lost
3. `gravity_reset_on_life_loss_system` detects loss
4. `GravityConfiguration::current` reset to `(0.0, 10.0, 0.0)`
5. Next ball spawns and falls at earth gravity

### Scenario 4: Queer Gravity RNG

**Input**: Level with gravity brick 25 (Queer), destroyed multiple times **Expected**:

1. First destruction → random gravity, e.g., `(8.3, 0.0, -2.1)`
2. Second destruction → different random gravity, e.g., `(-1.5, 0.0, 4.8)`
3. All X values in [-2.0, +15.0]
4. All Y values are exactly 0.0
5. All Z values in [-5.0, +5.0]
6. No correlation between consecutive RNG values

### Scenario 5: Level Without Gravity Field

**Input**: Level RON file without `gravity` field **Expected**:

1. `GravityConfiguration::level_default` set to `Vec3::ZERO`
2. Ball spawns and floats (zero gravity)
3. Gravity bricks can be destroyed to apply custom gravity
4. On ball loss, gravity resets to zero (float)
5. No errors or panics; graceful fallback
