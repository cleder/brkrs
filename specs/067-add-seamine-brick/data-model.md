# Data Model: Sea Mine Brick

## Entities & Components

- SeaMineBrick
  - Fields: `BrickTypeId(31)`, `CountsTowardsCompletion`, `Brick`
  - Behavior: On ball collision, emits `SpawnSeaMineMessage` and despawns as a normal destructible brick.

- SeaMineHazard
  - Components:
    - `SeaMine` marker
    - `Transform`, `Visibility`
    - `RigidBody::Dynamic`, `Collider`, `Velocity`
    - `GravityScale(0.0)`, `Restitution`, `Ccd`, `LockedAxes::TRANSLATION_LOCKED_Y`
  - Behavior:
    - Spawns with arbitrary XZ velocity and spin
    - Maintains minimum linear speed of 3.0 u/s
    - Maintains minimum angular speed of 180 deg/s
    - Detonates on wall, paddle, or brick index > 90 contact

- SeaMineExplosion
  - Fields: `position: Vec3`, `radius: f32 = 30.0`, `cause: SeaMineTriggerCause`
  - Behavior: Drives ball/paddle destruction and the Hanabi burst.

- SeaMineTriggerCause
  - Enum values: `Wall`, `Paddle`, `BrickGt90`
  - Behavior: Identifies the collision that armed the detonation.

- SeaMineParticleAssets
  - Resource fields: `explosion_effect: Handle<EffectAsset>`
  - Behavior: Loaded once at startup and reused for all bursts.

## Messages & Observers

- Message: `SpawnSeaMineMessage`
  - Fields: `position: Vec3`, `brick_entity: Entity`, `source_brick_type: u8`
  - Semantics: Buffered spawn request emitted when brick 31 is destroyed.

- Message: `SeaMineDetonationMessage`
  - Fields: `entity: Entity`, `position: Vec3`, `cause: SeaMineTriggerCause`, `radius: f32`
  - Semantics: Buffered gameplay resolution request consumed by damage and life-loss systems.

- Observer Event: `SeaMineExplosionTriggered`
  - Fields: `position: Vec3`, `radius: f32`
  - Semantics: Immediate visual burst trigger for Hanabi.

## Validation Rules

- Index rule: Brick 31 is the sea mine brick.
- Spawn rule: Exactly one sea mine hazard spawns per destroyed sea mine brick.
- Motion rule: Linear speed never drops below 3.0 u/s while active.
- Spin rule: Angular speed never drops below 180 deg/s while active.
- Blast rule: Only balls and the paddle inside 30 units are removed by the explosion effect.
- Life-loss rule: Paddle destruction from an explosion causes exactly one life loss.
