# Data Model: Brick Type Decals (014)

## Entities

### Brick

- id: Entity
- type: BrickType (enum)
- transform: Transform
- material: Handle<StandardMaterial>
- decal: Option<Decal>

### Decal

- id: Entity or struct
- texture: Handle<Image>
- normal_map: Option<Handle<Image>>
- position: Vec3 (centered on top face)
- brick_type: BrickType (enum)

### BrickType (enum)

- Standard
- Indestructible
- MultiHit
- ... (extensible)

## Relationships

- Each Brick may have one Decal, assigned by type.
- Each Decal is associated with a BrickType.

## Validation Rules

- Every brick type must have a decal or fallback/default.
- Decal must be centered on the top face of the brick.
- If normal_map is missing, fallback to flat decal and log warning.

## State Transitions

- On level load: Assign decals to bricks based on type.
- On asset reload: Update decals if assets change.
