# Research: Brick Type Decals (014)

## Decision: Decal Rendering Approach

- Use Bevy's standard material system with normal/bump mapping for decals, leveraging the parallax mapping example as a reference.
- Decals will be placed as separate mesh overlays or as part of the brick mesh, centered on the top face.

### Rationale

- Bevy 0.17.3 supports normal mapping and custom materials, as shown in the parallax_mapping.rs example.
- Using overlays or mesh layers allows for easy swapping and extensibility for new brick types.
- Normal/bump mapping provides the required embossed/engraved effect with minimal performance impact.

### Alternatives Considered

- Directly modifying brick textures: Less flexible, harder to extend for new types.
- Shader-based dynamic decals: More complex, not needed for static brick type hints.

## Decision: Asset Management

- Decal textures and normal maps will be stored in assets/textures/decals/ and referenced in RON level files.

### Rationale

- Keeps asset management consistent with existing project structure.
- Allows for easy addition of new decals and normal maps.

### Alternatives Considered

- Embedding decals in brick base textures: Reduces flexibility and increases asset duplication.

## Decision: ECS Integration

- Decal assignment will be handled by a system during level loading, attaching the correct decal/normal map to each brick entity based on its type.

### Rationale

- Keeps logic modular and testable.
- Aligns with ECS-first and modular feature design principles.

### Alternatives Considered

- Hardcoding decal assignment in brick spawning: Not extensible or testable.

## Decision: Testing

- Use cargo test and Bevy's test utilities to verify decal assignment and rendering logic.
- Visual regression tests for correct decal placement and normal mapping effects.

### Rationale

- Ensures all acceptance criteria are met and regressions are caught early.

### Alternatives Considered

- Manual testing only: Not sufficient for TDD and CI requirements.
