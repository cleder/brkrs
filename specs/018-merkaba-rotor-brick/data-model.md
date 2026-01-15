# Data Model: Merkaba Rotor Brick

## Entities & Components

- RotorBrick (index 36)
  - Fields: `index = 36`, `is_destroyed`
  - Behavior: On ball collision, emit `SpawnMerkabaMessage` and despawn as a normal brick.

- Merkaba
  - Components:
    - `Transform` (z fixed), `Visibility`
    - `RigidBody::Dynamic`, `Collider` (ball/convex), `Velocity`
    - `GravityScale(0.0)`, `Restitution`
    - `Merkaba` (marker)
    - `RotationZ` (continuous rotation around z-axis)
  - Relationships: Child entities for dual tetrahedron meshes (upright + inverted).

- AudioLoopState (Resource)
  - Fields: `merkaba_count: u32`, `loop_playing: bool`
  - Behavior: Start/stop helicopter loop based on `merkaba_count > 0`.

## Messages & Events

- Message: `SpawnMerkabaMessage`
  - Fields: `position: Vec3`, `angle_variance_deg: f32` (±20°), `delay_s: f32 (=0.5)`
  - Semantics: Buffered; processed by spawn system which applies delayed entity creation.

- Event: `MerkabaCollisionEvent`
  - Fields: `surface: enum {Wall, Brick, Paddle}`
  - Semantics: Immediate; triggers distinct collision sounds.

- Event: `PlayerLifeLost`
  - Fields: `cause: enum {MerkabaPaddleContact, Other}`
  - Semantics: Immediate; despawns balls and merkabas.

## Validation Rules

- Min x-speed: After collisions and at intervals, enforce `abs(velocity.x) >= 3.0`.
- Plane constraint: `transform.translation.z` remains within a narrow band (e.g., `≈ 0`).
- Spawn location: Merkaba spawns at precise position of destroyed rotor brick.
- Loop lifecycle: Helicopter loop starts when first merkaba spawns; stops when last despawns.
