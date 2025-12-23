# API Contract: Brick Type Decals (014)

## ECS System Contracts

### System: assign_brick_decals

- Input: All brick entities with BrickType
- Output: Each brick entity has a Decal component assigned, with correct texture and normal map
- Error: If decal or normal map missing, assign default/fallback and log warning

### System: update_decal_assets

- Input: Asset reload event
- Output: Decal components updated with new asset handles

## Test Contract

- Test: All brick types in a test level have correct decals assigned
- Test: Decals are centered and visible on top face
- Test: Normal/bump mapping is applied and visible under lighting
- Test: Fallback/default decal is used if asset missing
