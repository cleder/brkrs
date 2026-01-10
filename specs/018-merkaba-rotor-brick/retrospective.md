# Retrospective: Merkaba Hazard Implementation

**Date:** 2026-01-08 **Feature:** Merkaba Rotor Brick Hazard **Summary:** Implementation of a rotating, floating "Merkaba" hazard (Star Tetrahedron) that bounces around the map, destructs on goal collision, and causes life loss on paddle collision without being physically pushed by the paddle.

## Objectives & Challenges

The primary goal was to introduce a dynamic hazard that interacts with the physics system as a "trigger" rather than a "solid obstacle" for the paddle, while still acting as a solid object for bouncing off walls and the ball.

### 1. Visual Rendering & Geometry

- **Challenge:** The custom mesh (dual-tetrahedron) appeared to have missing faces when viewed from certain angles due to backface culling on the single-sided mesh.
- **Resolution:** Created a secondary "inverted" mesh by flipping the vertices and reversing the winding order.
  This ensured efficient double-sided rendering without altering global material settings.
- **Key Learning:** When generating procedurally "hollow" 3D shapes, explicitly spawning an inverted mesh is often cleaner than relying on two-sided material configurations in engines that default to backface culling.

### 2. Physics & Collision Architecture

- **Challenge A (The "Pushing" Bug):** The Kinematic Paddle treated the Dynamic Merkaba as a solid obstacle, pushing it around the map instead of passing through it.
- **Challenge B (KCC Override):** Even after applying `SolverGroups` to prevent force resolution, the `KinematicCharacterController` (KCC) continued to treat the Merkaba as an obstacle during its movement tests.
- **Resolution:** A three-layer physics configuration was required:
    1. **`SolverGroups`:** Prevent physical impulse resolution between Paddle (Group 1) and Merkaba (Group 2).
    2. **`CollisionGroups`:** Explicitly assign the Merkaba to `Group::GROUP_2` (fixing a bug where it defaulted to `Group::ALL`).
    3. **`KinematicCharacterController` Filtering:** Explicitly set `filter_groups` on the Paddle's controller to ignore `Group::GROUP_2`.
- **Key Learning:** The `KinematicCharacterController` operates independently of the entity's attached `Collider` filters.
  It requires its own explicit `filter_groups`.
  Also, entities default to `Group::ALL` unless `CollisionGroups` is explicitly added, which can silently break filtering logic.

### 3. Game State Management & Regressions

- **Challenge:** Physics fixes worked on initial load but failed after a Level Restart ('R' key).
- **Resolution:** Traced the issue to `src/level_loader.rs`.
  The `reset_level_state` function was clearing standard entities (Bricks, Balls, Paddles) but failing to despawn Merkabas.
  Old, malformed entities persisted in the scene.
- **Resolution:** Updated `reset_level_state` to explicitly query and `despawn` all `Merkaba` entities.
- **Key Learning:** Every new entity type added to the game requires updates to the cleanup/reset logic. "Respawn" (player death) logic and "Restart" (level reload) logic often reside in different systems and must be kept in sync.

### 4. Compilation & Dependencies

- **Challenge:** Compilation failed due to missing imports (`CollisionGroups`) when porting logic between files.
- **Resolution:** Added `use bevy_rapier3d::prelude::CollisionGroups;` to `src/systems/merkaba.rs`.
- **Key Learning:** Physics configuration structs are effectively "types" that must be imported.
  Always check imports when copying `SolverGroups` or `CollisionGroups` logic.

## Final Physics Configuration Matrix

| Entity  | Component | Setting | Purpose |
| :--- | :--- | :--- | :--- |
| **Merkaba** | `CollisionGroups` | `Group::GROUP_2`, `Group::ALL` | Identifies entity as "Group 2" in the world. |
| **Merkaba** | `SolverGroups` | `Group::GROUP_2`, `ALL ^ Group::GROUP_1` | Prevents physical bounce against Paddle (Group 1). |
| **Paddle** | `SolverGroups` | `Group::GROUP_1`, `ALL` | Standard physics interaction. |
| **Paddle** | `KinematicCharacterController` | `filter_groups: (Group 1, ALL ^ Group 2)` | Ensures **movement** logic ignores Merkaba (Group 2). |

### 5. Coordinate & Transform Misreads (Post-Release Regression)

- **Symptom:** Merkabas visually spawned at the same spot and appeared to move on the wrong axis.
- **Root Causes:**
  - `enforce_z_plane_constraint` was clamping `translation.z` to `0.0`, forcing every merkaba onto the same Z line regardless of spawn column.
  - Spawning added both `Transform` and `GlobalTransform`; Bevy’s transform propagation overwrote the manual global value, hiding the correct spawn coordinates.
  - Mixed comments/spec wording (“y-direction”, “screen horizontal z”) led to flipping X/Z in fixes despite the velocity code being correct.
- **Resolution:**
  - Removed manual `GlobalTransform` on spawn; let Bevy derive it from `Transform`.
  - Disabled the incorrect z-plane clamp (was asserting Z≈0 even though level columns live on Z).
  - Confirmed velocity is `vx = base_speed` (X primary), `vz = tan(angle) * base_speed` (small variance), matching original implementation.
- **Key Learning:** When diagnosing position/direction bugs, inspect post-spawn transforms and constraints before changing velocity math; avoid setting `GlobalTransform` directly.
