# Data Model: Refactor Entity Spawning

**Feature**: `012-refactor-entity-spawning`

## Components

### `MainCamera`

- **Type**: Component (Marker)
- **Location**: `src/systems/spawning.rs`
- **Description**: Marks the primary 3D camera used for rendering the game scene.
- **Fields**: None (Unit struct).
- **Usage**: Used in queries to find the main camera (e.g., for raycasting or camera shake).

## Entities

### Camera Entity

- **Components**:
  - `Camera3d`: Standard Bevy camera bundle.
  - `Transform`: Positioned at `(0.0, 37.0, 0.0)`, looking at origin.
  - `MainCamera`: Marker.

### Ground Plane Entity

- **Components**:
  - `Mesh3d`: Plane mesh (`PLANE_H` x `PLANE_W`).
  - `MeshMaterial3d`: Silver color.
  - `GroundPlane`: Marker (existing).

### Light Entity

- **Components**:
  - `PointLight`: Shadows enabled, high intensity.
  - `Transform`: Positioned at `(-4.0, 20.0, 2.0)`.
