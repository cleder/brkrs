# Data Model: Ball, Paddle, Brick Physics Config

## Entities

### BallPhysicsConfig (Resource)

- restitution: f32
- friction: f32
- linear_damping: f32
- angular_damping: f32

// All gameplay-relevant fields must be explicitly listed.
Extension fields are allowed but must be justified and documented.

### PaddlePhysicsConfig (Resource)

- restitution: f32
- friction: f32
- linear_damping: f32
- angular_damping: f32

// Paddle config mirrors Ball config for consistency.
If fields differ, document rationale.

### BrickPhysicsConfig (Resource)

- restitution: f32
- friction: f32

// Brick config omits damping fields as bricks are static.
If fields are added, document rationale.

## Relationships

- Each config is a Bevy resource, registered at startup.
- All spawn systems for balls, paddles, and bricks must query the relevant config resource and apply its values to the collider/rigidbody.

## Validation Rules

- All physics values must be finite, non-negative, and within reasonable gameplay bounds.
- If a config is missing at runtime, the game must panic with a clear error message.

## State Transitions

- Config values are static for the duration of a run (no runtime mutation or hot-reload).
