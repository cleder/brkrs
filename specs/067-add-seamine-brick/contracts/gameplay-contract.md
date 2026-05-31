# Gameplay Contract: Sea Mine Brick

## Purpose

Defines the message and observer boundaries for the sea mine brick feature.

## Messages

### SpawnSeaMineMessage

Emitted when brick 31 is destroyed.

Fields:

- `position: Vec3`
- `brick_entity: Entity`
- `source_brick_type: u8`

Semantics:

- Buffered spawn request.
- Consumed by the sea mine spawn system.
- One message spawns one sea mine.

### SeaMineDetonationMessage

Emitted when a sea mine collides with a valid trigger.

Fields:

- `entity: Entity`
- `position: Vec3`
- `cause: SeaMineTriggerCause`
- `radius: f32`

Semantics:

- Buffered gameplay resolution.
- Consumed by damage and life-loss systems.
- Removes balls and the paddle inside the radius.

## Observer Events

### SeaMineExplosionTriggered

Used only for the immediate Hanabi burst.

Fields:

- `position: Vec3`
- `radius: f32`

Semantics:

- Fires immediately when detonation is resolved.
- Spawns the particle burst at the detonation point.
- Does not own gameplay state.

## Asset Contract

### SeaMineExplosionEffect

- Loaded once at startup.
- Stored in a resource.
- Reused for every detonation burst.
- Implemented with `bevy_hanabi`.

## Behavioral Guarantees

- Gameplay state changes use Messages.
- Visual burst uses an Observer.
- The Hanabi burst does not change score, lives, or entity lifetime.
- The detonation radius is 30 world units.
