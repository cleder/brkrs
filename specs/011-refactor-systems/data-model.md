# Data Model: Refactor Systems for Constitution Compliance

## Entities & Messages

- Module: `crate::signals`
  - `UiBeep`: `#[derive(Message)]`; payload: none (marker)
  - `BrickDestroyed`: `#[derive(Message)]`; fields:
    - `brick_entity: Entity`
    - `brick_type: u8`
    - `destroyed_by: Option<Entity>`

## System Sets

- `AudioSystems` (new)
  - Startup: audio config/assets init
  - Update: message consumers (`UiBeep`, `BrickDestroyed`), cleanup finished sounds
- `PaddleSizeSystems` (new)
  - Detect → Update (timers) → Cleanup (remove expired) → Visual (Changed/Removed driven) → Audio (optional)
- `TextureOverrideSystems` (new)
  - Refresh (on manifest/bank changes) → Apply (on `LevelPresentation` change)
- `RespawnSystems` (existing)
  - Keep sets; replace tuple `.chain()` inside `Visual` with ordering between sets

## Required Components

Apply `#[require(Transform, Visibility)]` to marker components:

- `Paddle`
- `Ball`
- `GridOverlay`
- `Border`
- `GroundPlane`

## Change Detection Rules

- `paddle_size::update_paddle_visual_feedback`: run only on `Changed<PaddleSizeEffect>`
- `paddle_size::restore_paddle_visual`: use `RemovedComponents<PaddleSizeEffect>`
- `textures::materials::apply_canonical_materials_to_existing_entities`: gate on `Changed<CanonicalMaterialHandles>`/`Changed<TypeVariantRegistry>` and `OnAdd` for entities
- `grid_debug::toggle_grid_visibility`: only on `Changed<WireframeConfig>`

## Asset Handle Reuse

- Load manifest/assets once at startup; store handles in Resources (e.g., `AudioAssets`, `ProfileMaterialBank`, `CanonicalMaterialHandles`).
- Do not call `asset_server.load()` inside spawn loops; clone existing handles.
