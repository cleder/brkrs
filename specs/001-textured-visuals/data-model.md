# Data Model: Textured Visuals Overhaul

## VisualAssetProfile

- **Purpose**: Canonical default texture/material definition per object class (ball, paddle, bricks, sidewalls, ground, background).
- **Fields**:
  - `id: String` – unique key (e.g., `ball/default`).
  - `albedo_path: AssetPath` – PNG or KTX path relative to `assets/textures/`.
  - `normal_path: Option<AssetPath>` – optional normal map.
  - `roughness: f32` – 0.0–1.0 clamp; default 0.5.
  - `metallic: f32` – 0.0–1.0 clamp; default 0.0.
  - `uv_scale: Vec2` – tiling factors; default (1,1).
  - `uv_offset: Vec2` – offset applied before sampling.
  - `fallback_chain: Vec<String>` – ordered list of backup profile IDs.
- **Validation Rules**:
  - File paths must exist at build time or fallback_chain must be non-empty.
  - Roughness/metallic clamped to [0,1].
- **Relationships**:
  - Referenced by `TypeVariantDefinition` via `profile_id`.
  - Overrides provided per level via `LevelTextureSet.overrides`.

## LevelTextureSet

- **Purpose**: Per-level overrides for ground plane, background, sidewalls, and optional tinting.
- **Fields**:
  - `level_number: u32` – matches `LevelDefinition.number`.
  - `ground_profile: Option<String>` – references `VisualAssetProfile.id`.
  - `background_profile: Option<String>` – references `VisualAssetProfile.id`.
  - `sidewall_profile: Option<String>` – references `VisualAssetProfile.id`.
  - `tint: Option<Color>` – multiplies final material color.
  - `notes: Option<String>` – freeform, helps artists.
- **Validation Rules**:
  - Missing profiles default to canonical entries.
  - Tint components clamped to [0,1].
- **Relationships**:
  - Loaded alongside `LevelDefinition`; stored in new resource `LevelPresentation`.

## TypeVariantDefinition

- **Purpose**: Map gameplay type IDs to visual profiles.
- **Fields**:
  - `object_class: Enum` – `Ball` or `Brick`.
  - `type_id: u8` – matches matrix values or gameplay enums.
  - `profile_id: String` – references `VisualAssetProfile.id`.
  - `emissive_color: Option<Color>` – applied on top of albedo.
  - `animation: Option<AnimationDescriptor>` – future-proof for wobble/glow.
- **Validation Rules**:
  - `profile_id` must resolve to a `VisualAssetProfile`.
  - Duplicate `(object_class, type_id)` pairs rejected.

## FallbackRegistry (Runtime Resource)

- **Fields**:
  - `ball: Handle<StandardMaterial>` etc. for each object class.
  - `invocations: HashSet<String>` – tracks which fallbacks were used this session.
- **Behavior**:
  - Provides `log_once(id)` helper so warnings emit only once per session.

## TextureManifest (File Schema)

- **Structure**:

```ron
(
  profiles: [VisualAssetProfile, ...],
  type_variants: [TypeVariantDefinition, ...],
)
```

- **Validation Rules**:
  - All IDs unique.
  - All references resolvable.

## LevelSwitchState

- **Purpose**: Track list of playable levels and current index so the **L** shortcut can wrap safely.
- **Fields**:
  - `ordered_levels: Vec<LevelDefinition>` – preloaded definitions.
  - `current_index: usize` – matches `CurrentLevel`.
  - `pending_switch: bool` – prevents concurrent transitions.
- **Relationships**:
  - Mutated by new `level_switcher` system and by existing advance logic.
