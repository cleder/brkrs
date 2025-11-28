# Data model: Indestructible bricks (LevelDefinition)

## Entities

- LevelDefinition
  - Description: A 2D matrix representation of a level layout; each cell is an integer tile index mapping to a BrickType or other tile.
  - Fields:
    - id: string (file or level identifier)
    - matrix: [Link text](integer)] — required. Each inner array represents a row of tile indexes.
    - metadata: map<string, string> — optional, authoring metadata and versioning

- BrickType
  - Description: The logical classification of a tile index.
  - Fields:
    - index: integer — unique numeric mapping used in LevelDefinition
    - name: string — human readable classification (e.g., simple, indestructible, multi-hit)
    - durability: integer|null — number of hits before destruction (null for indestructible)
    - countsTowardsCompletion: boolean — whether this brick contributes to level completion

- BrickInstance
  - Description: A runtime entity created from a BrickType at a given coordinate
  - Fields:
    - position: {x: int, y: int}
    - brick_type_index: integer
    - runtime_state: map<string, any> (durabilityRemaining, flags)

- LevelCompletionCounter
  - Description: Runtime construct that tracks remaining destructible bricks to determine level completion.
  - Fields:
    - total_destructible: integer
    - remaining_destructible: integer

## Validation rules

- LevelDefinition.matrix must be rectangular (all rows same length).
- Each numeric token in matrix must map to a defined BrickType index; unknown indexes are validation errors.
- For migration: tooling must detect and update any `3` to `20` for repository assets; tests should assert post-migration parity with pre-migration semantics.

## State transitions

- Normal hit on destructible Bricks: durabilityRemaining -> 0 -> destroyed -> decrement remaining_destructible.
- Hit on indestructible Bricks: no durability change and no decrement of remaining_destructible.
