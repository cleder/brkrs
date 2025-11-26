# Research: Textured Visuals Overhaul

## Decision 1: RON-based Texture Manifest + Asset Profiles

- **Decision**: Represent the canonical texture manifest as a RON file (`assets/textures/manifest.ron`) describing `VisualAssetProfile` entries for each object class (ball, paddle, bricks, sidewalls, ground, background) plus optional per-type overrides.
- **Rationale**: RON already backs the existing level definitions and keeps authoring consistent for designers who edit data by hand. It is strongly typed via Serde, integrates with Bevy's asset loader without extra tooling, and round-trips well in source control.
- **Alternatives considered**:
  - **JSON** – familiar but noisier (quotes everywhere), harder to add comments, and inconsistent with current pipeline.
  - **Separate TOML files per asset** – would explode the number of files and slow iteration.

## Decision 2: Fallback Texture Registry & Lazy Material Baking

- **Decision**: Maintain a `FallbackRegistry` resource that houses default `StandardMaterial` handles (solid albedo + optional normal) per object class. When a requested texture fails to load, spawn the fallback immediately, log once per session, and lazily bake the "real" material when/if the asset stream recovers.
- **Rationale**: Guarantees that every mesh is textured within a single frame even on WASM (where streaming texture formats are limited). Logging once avoids spam while giving QA actionable info.
- **Alternatives considered**:
  - **Blocking loads** – waiting on IO would hitch gameplay and violate Constitution IV (performance-first).
  - **Placeholder warning materials per entity** – harder for QA to know which asset failed because color variations get confusing.

## Decision 3: Type-Driven Material Lookups via ECS Resources

- **Decision**: Store `TypeVariantDefinition` data in a Bevy resource and let systems query it when spawning or mutating entity types, ensuring ECS-first implementation.
- **Rationale**: Keeps lookups cache-friendly and testable; allows headless tests to load definitions without a full asset server.
- **Alternatives considered**:
  - **Spread logic in individual systems** – would duplicate mapping logic and make testing painful.

## Decision 4: Level Switch Shortcut via Event + LevelLoader Hooks

- **Decision**: Introduce a `LevelSwitchRequested` event triggered by `KeyCode::L`. A new system in `LevelLoaderPlugin` listens for the event, loads the next level definition (wrapping after the last), and re-runs the same spawn/resets as the normal advance flow.
- **Rationale**: Fits Brkrs' modular constitution—input handling stays separate, while LevelLoader owns level state. Reusing the existing pipelines avoids duplicate cleanup code.
- **Alternatives considered**:
  - **Directly mutating level resources inside the input system** – risk of violating ECS boundaries and causing race conditions with ongoing transitions.
  - **Adding a console command only** – would not meet the fast iteration need for QA/gamepad users.

## Decision 5: Testing Approach (Native + WASM parity)

- **Decision**: Extend `cargo test` suites with new `TextureManifest` parsing tests, fallback-behavior tests (headless), and a `level_switcher.rs` integration test that fakes `KeyCode::L`. Verify asset coverage manually plus automated screenshot comparisons where possible.
- **Rationale**: Aligns with the constitution's testing clause and ensures WASM parity before merging.
- **Alternatives considered**:
  - **Manual testing only** – too fragile and slow for regression coverage.
