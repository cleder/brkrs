# Contracts: Internal Messaging

This feature does not expose external HTTP/gRPC APIs.
Instead, it standardizes internal Messages.

## Messages (crate::signals)

- UiBeep
  - Type: Message
  - Purpose: Short UI/audio feedback cue
  - Payload: none

- BrickDestroyed
  - Type: Message
  - Purpose: Inform scoring/audio of brick destruction
  - Fields:
    - brick_entity: Entity
    - brick_type: u8
    - destroyed_by: Option<Entity>

## Engine Events

- AssetEvent<Image>
  - Consumed as: Event via observers (not a Message)
  - Purpose: Texture sampler repeat configuration, manifest hydration logging
