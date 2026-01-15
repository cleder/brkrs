# Data Model: Audio Wall Delay Fix

## Entities

### Ball

- **Description**: The moving object that can collide with walls.
- **Fields**: Entity ID, Transform, Velocity, PhysicsBody, etc.

### Wall

- **Description**: The boundary object that triggers audio feedback on collision.
- **Fields**: Entity ID, Transform, Collider, etc.

### BallWallHit Event

- **Description**: Dedicated message/event type for wall hit sound playback.
- **Fields**:
  - `ball: Entity` (the ball that hit the wall)
  - `wall: Entity` (the wall that was hit)
  - `impulse: Vec3` (collision impulse, optional for audio intensity)
  - `timestamp: f64` (optional, for latency measurement)

## Relationships

- Ball can collide with multiple walls.
- Each BallWallHit event is associated with one ball and one wall.

## Validation Rules

- BallWallHit must only be emitted for ball-wall collisions (not paddle, brick, etc.).
- Ball and wall entities must exist at the time of event emission.
- Audio system must process every BallWallHit event, subject to concurrency limits.

## State Transitions

- On collision, BallWallHit event is emitted.
- Audio system receives event and attempts to play wall hit sound.
- If concurrency limit is reached, event is logged/skipped.

---

This data model covers all entities, events, and relationships required for the Audio Wall Delay Fix feature.
