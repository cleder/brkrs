# Data Model: Extra Ball Brick (Brick 41)

## Entities & Components

### Brick 41 (Extra Ball)

- **Fields**: `id: u32 = 41`, `durability: u8 = 1`, `score_value: i32 = 0`, `destruction_sound: AudioHandleRef`.
- **Relationships**: Uses standard brick collision components; despawns through existing brick destruction pipeline after first hit.
- **Behavioral flags**: Grants +1 life on first valid hit; no score change.
- **Asset paths**: Unique sound `assets/audio/brick_41_extra_life.ogg`, fallback `assets/audio/brick_generic_destroy.ogg`.

### Player Lives Counter (Resource/Component)

- **Fields**: `current: u32`, `max: u32`.
- **Relationships**: Updated via life-award message; UI reads current value; clamps to `max` on increment.
- **Config source**: Lives cap (`max: u32`) loaded from `GameConfig` Resource or `config/lives.ron`.

## Messages / Events

- **LifeAwardMessage**: `{ delta: i32 }` (use `delta = +1`; consumer clamps to max).
- **AudioMessage**: `{ sound: AudioHandleRef, channel: Channel, volume: f32 }` (reuse existing audio bus; dedicated handle for brick 41 destruction; fallback to generic brick sound).
- **BrickHit flow**: Existing collision message triggers brick-specific behavior; brick 41 handler consumes hit once, enqueues life and audio messages, and marks brick for despawn.

## Coordinate System Notes

- Gameplay plane: XZ; Y locked via physics constraints.
- Direction mapping: +Z toward bricks (gameplay forward), -Z toward paddle, ±X lateral.
  No new movement introduced by this feature.
