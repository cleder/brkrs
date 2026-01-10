# Contracts: Brick 41 Extra Life & Audio

## Life Award Message

- **Type**: `LifeAwardMessage`
- **Payload**: `{ delta: i32 }` (use `+1` for this brick)
- **Producer**: Brick 41 hit handler (on first valid collision)
- **Consumer**: Lives system (clamps to configured max; updates UI/analytics via existing paths)
- **Reliability**: Buffered Message (read next schedule step); idempotent via single-despawn guard.

## Audio Trigger Message

- **Type**: `AudioMessage` (existing audio bus)
- **Payload**: `{ sound: AudioHandleRef, channel: Channel::Sfx, volume: f32 }`
- **Producer**: Brick 41 hit handler (same frame as life message enqueue)
- **Consumer**: Audio playback system (plays once; ignores if handle missing and falls back to generic brick sound handle when provided)
- **Reliability**: Buffered Message; duplicates prevented by brick despawn.

## Destruction Flow Contract

- **Precondition**: Brick 41 has durability 1 and is active in the level grid.
- **Trigger**: First ball collision routed through standard brick hit pipeline.
- **Actions**:
  1. Enqueue `LifeAwardMessage { delta: +1 }`.
  2. Enqueue `AudioMessage` with brick-41-specific handle (or fallback generic brick sound if specific handle missing).
  3. Mark brick for despawn; remove colliders/visuals per existing brick destruction flow.
- **Postconditions**:
  - Lives incremented up to max.
  - Score unchanged (0 points).
  - Brick no longer collidable or visible; no further messages emitted from subsequent collisions.
