<!--
SYNC IMPACT REPORT
- Version change: 1.2.0 → 1.3.0
- Modified principles: Added "VIII. Bevy 0.17 Mandates & Prohibitions"
- Added sections: "VIII. Bevy 0.17 Mandates & Prohibitions" (new)
- Removed sections: None
- Templates requiring updates:
  - .specify/templates/plan-template.md: ✅ updated
  - .specify/templates/spec-template.md: ✅ updated
  - .specify/templates/tasks-template.md: ✅ updated
  - .specify/templates/agent-file-template.md: ✅ updated
  - .github/agents/speckit.tasks.agent.md: ✅ updated
  - .github/agents/speckit.implement.agent.md: ✅ updated
- Follow-up TODOs:
  - Add CI job to validate tests-first commit pattern and enforce failing tests proof: TODO(.github/workflows/*)
  - Validate any open plans/specs for compliance with TDD + Bevy 0.17 mandates (spot-check in /specs/)
-->

# Brkrs Constitution

## Core Principles

### I. Entity-Component-System Architecture (ECS-First)

All game features MUST be implemented using Bevy's ECS paradigm:

- Game logic expressed as systems operating on components
- State stored in components attached to entities
- Systems MUST be pure functions of their query inputs
- Avoid storing mutable state outside the ECS where possible
- Leverage Bevy's change detection for reactive behavior

**Rationale**: ECS architecture provides performance, modularity, and maintainability.
It enables parallel execution, makes dependencies explicit, and aligns with Bevy's design philosophy.

### II. Physics-Driven Gameplay

All game mechanics MUST rely on the physics engine (Rapier3D):

- Use physics forces, impulses, and collisions for game interactions
- Avoid manual transform manipulation for gameplay objects
- Leverage physics properties (restitution, friction, damping) for tuning
- Collision detection MUST drive game events
- Physics configuration MUST be exposed for gameplay tuning

**Rationale**: Physics-driven gameplay ensures realistic, emergent behavior.
It reduces manual coding of movement and collision logic, and provides a consistent, predictable foundation for game mechanics.

### III. Modular Feature Design

Each game feature MUST be implemented as independently testable modules:

- Features defined as distinct systems or system sets
- Clear component markers for feature entities
- Features MUST be addable/removable without breaking core gameplay
- Event-driven communication between features
- No tight coupling between game features

**Rationale**: Modularity enables incremental development, easier testing, and future extensibility.
Features can be developed in parallel and disabled/enabled independently for debugging.

### IV. Performance-First Implementation

Game code MUST meet 60 FPS performance targets:

- Profile systems to identify bottlenecks
- Use Bevy's parallel execution capabilities
- Minimize allocations in hot loops
- Leverage Bevy's asset system for resource management
- Test on target platforms (native + WASM) early
- Use debug builds with opt-level=3 for dependencies during development

**Rationale**: Games require consistent frame rates for good player experience.
Early performance awareness prevents costly refactoring later.
WASM targets have stricter performance constraints.

### V. Cross-Platform Compatibility

Code MUST support native (Linux/Windows/macOS) and WASM targets:

- Use conditional compilation for platform-specific features
- Test WASM builds regularly (not just native)
- Avoid platform-specific APIs without fallbacks
- Document platform-specific limitations clearly
- Assets MUST be optimized for web delivery

**Rationale**: The project targets both native and web platforms.
WASM support broadens accessibility.
Platform-specific issues caught late are expensive to fix.

## Performance Standards

**Frame Rate**: MUST maintain 60 FPS on target hardware

- Native: Modern desktop (last 5 years)
- WASM: Chrome/Firefox on moderate hardware

**Build Performance**:

- Debug builds: Fast iteration (<30s incremental)
- Release builds: Optimized assets and code
- Use dynamic linking in development for faster compile times

**Asset Management**:

- Textures: Appropriate resolution for display size
- Meshes: LOAD where applicable
- Audio: Compressed formats for web delivery

## Development Workflow

**Version Control**:

- Feature branches from main development branch
- Descriptive commit messages referencing features/fixes
- No direct commits to protected branches

**Code Quality**:

- Rust compiler warnings MUST be addressed (no `allow` without
  justification)
- Follow Clippy suggestions unless explicitly justified
- Use `rustfmt` for consistent formatting
- Use `bevy lint` to lint for bevy specifics
- Document non-obvious design decisions in code comments

**Testing Strategy**:

- All implementation MUST follow strict Test-Driven Development (TDD): write tests first, confirm tests fail (red), obtain approval for tests, then implement until tests pass (green).
- Manual testing remains required for gameplay features where human judgement is needed.
- Integration testing for critical game mechanics is mandatory where feasible.
- Test both native and WASM builds before releases and include WASM-specific tests where behavior differs.
- Every public function and critical internal path MUST have unit tests; feature behaviors MUST have integration/acceptance tests.
- Performance profiling for new systems and performance regression tests are REQUIRED for performance-sensitive changes.

**Documentation**:

- README MUST describe how to build and run
- Feature specifications in `.specify/` for complex additions
- In-code documentation for public APIs
- Update documentation when behavior changes

### VI. Comprehensive Rustdoc Documentation

All public modules, functions, types, and traits MUST have rustdoc documentation:

- **Module-level docs** (`//!`): Explain the module's purpose, when to use
  it, and how it fits into the overall architecture
- **Function docs** (`///`): Describe WHY the function exists and WHEN to
  use it, NOT HOW it is implemented
- **Type docs** (`///`): Explain what the type represents and its role in
  the system
- **Examples**: Include usage examples for non-trivial public APIs
- **Panics/Errors**: Document panic conditions and error cases

**Focus on Intent, Not Implementation**:

- Document the problem being solved, NOT the algorithm used
- Explain when callers should choose this function over alternatives
- Describe preconditions, postconditions, and invariants
- Implementation details belong in code comments (`//`), not rustdoc

**Rationale**: Rustdoc is embedded in the published documentation site.
Clear, purpose-focused documentation enables developers to understand the codebase without reading implementation code.
It supports onboarding, code review, and long-term maintainability.

### VII. Test-Driven Development (TDD-First)

All code implementations MUST follow strict Test-Driven Development (TDD):

- **Write tests first**: Unit tests and acceptance tests MUST be authored
  and committed before any implementation code for the same feature.
- **Red phase REQUIRED**: Tests MUST be confirmed to FAIL (red) prior to
  writing implementation code.
- **Approval gate**: Tests MUST be validated and explicitly approved by the
  feature owner or requestor (a maintainer or designated reviewer) before
  implementation begins.
- **Test coverage**: Every new public function and critical internal path
  MUST have unit tests; feature-level behaviors MUST have integration or
  acceptance tests that exercise user scenarios (including WASM where
  applicable).
- **CI enforcement**: CI pipelines MUST run the test suite and reject merges
  that do not include the tests-first proof (a failing test commit followed
  by a passing commit) or that remove tests.
- **Document test intent**: Tests MUST include clear, declarative names and
  comments describing the intended behavior and acceptance criteria.

**Rationale**: Enforcing TDD ensures that behavior is specified explicitly before implementation, reduces regressions, and improves design quality.
It provides objective verification (tests) for requirement fulfillment and discourages untestable or speculative changes.

### VIII. Bevy 0.17 Mandates & Prohibitions

#### Bevy 0.17 ECS Architecture Mandates

- **Fallible Systems:** All systems MUST return `Result` and use the `?` operator for error propagation.
  Query methods like `single()`, `single_mut()`, and `get_many()` MUST have the `?` operator applied to their `Result` return values, never `.unwrap()`.
- **Required Components:** Component structs MUST use `#[require(Transform, Visibility)]` attributes to automatically include mandatory dependencies.
  NEVER manually spawn entities with redundant component bundles.
- **Query Specificity:** All queries MUST include precise filters using `With<T>` and `Without<T>` to enable system parallelism.
  Queries accessing `&mut Transform` MUST differentiate entity types to prevent scheduling conflicts.
- **Change Detection:** Systems that react to component changes MUST use `Changed<T>` filters in queries.
  UI update systems MUST ONLY execute when source data changes, not every frame.
- **Message vs Event Distinction:** Use `#[derive(Message)]` for buffered event queues with `MessageWriter`/ `MessageReader`.
  Use `#[derive(Event)]` exclusively for observer patterns with `commands.observe()`.
  NEVER conflate these two patterns.
- **Component Mutation Over Insertion:** State changes MUST modify existing component data (e.g., enum variants) rather than inserting/removing components.
  This prevents archetype thrashing.
- **Relationship API:** Access parent entities using `ChildOf::parent()` method.
  Use `commands.entity(parent).add_children()` and `.remove::<Children>()` for hierarchy manipulation.
- **Error Recovery Patterns:** Use `let Ok(value) = result else { return Ok(()); }` for expected failures.
  Use `let Some(value) = option else { return Ok(()); }` for missing optional data.
- **System Organization:** Define system sets with `*Systems` suffix (e.g., `GameplaySystems::Input`).
  Use `.configure_sets()` with `.chain()` only between sets, not individual systems.
  Group parallelizable systems within the same set.
- **Plugin-Based Architecture:** Organize features into Plugin structs with `.build()` methods that register all related systems, resources, and schedules.
  Each plugin MUST be self-contained.
- **Asset Handle Reuse:** Load assets once in startup systems and store handles in Resources.
  NEVER call `asset_server.load()` repeatedly for the same asset path in spawn systems.
- **State-Scoped Cleanup:** Use `DespawnOnExit(State)` component for entities that should despawn when exiting a state.
  Use `despawn_entities_on_exit_state::<S>()` for automatic cleanup registration.

#### Bevy 0.17 3D Graphics Mandates

- **Camera Setup:** Spawn cameras with separate `Hdr` component.
  Use `commands.spawn((Camera3d::default(), Hdr, Transform::from_xyz(...)))`.
  NEVER use a non-existent `hdr` field on `Camera3d`.
- **Lighting Configuration:** Use `DirectionalLight` with `shadows_enabled: true` for outdoor scenes.
  Set `AmbientLight` as a resource with appropriate brightness values (typically 200.0-1000.0).
- **Material Organization:** Store material handles in dedicated Resource structs.
  Use `StandardMaterial` with explicit `metallic` and `perceptual_roughness` values rather than defaults.
- **Mesh3d Components:** Spawn 3D entities with `Mesh3d(handle)` and `MeshMaterial3d(handle)` components.
  Include `Transform` as a required component on entity markers.
- **Import Path Correctness:** Import from `bevy::camera` for camera types, `bevy::light` for light types, `bevy::mesh` for `Mesh`, and `bevy::anti_alias` for FXAA/SMAA/TAA plugins.

#### Bevy 0.17 Performance Mandates

- **Development Profile Optimization:** Set `opt-level = 1` and `strip = "debuginfo"` in `[profile.dev]`.
  Set `opt-level = 3` in `[profile.dev.package."*"]` to optimize dependencies.
- **Dynamic Linking:** Enable `bevy/dynamic_linking` feature in development builds.
  Use `cargo run --features dev` with a dev feature flag.
- **Resource Initialization Timing:** Initialize render resources in `RenderStartup` schedule, not `Plugin::finish()`.
  Convert `FromWorld` implementations to initialization systems that use `Commands::insert_resource()`.
- **System Ordering with Dependencies:** When render resources depend on each other, use `.after(init_other_resource)` ordering constraints in `RenderStartup`.

#### Bevy 0.17 ECS Prohibitions

- **NO Panicking Queries:** NEVER use `.unwrap()` on query results.
  NEVER use deprecated `get_single()` methods.
  ALWAYS return `Result` and use `?` operator.
- **NO Broad EntityRef Queries:** NEVER query `Query<EntityRef>` without filters.
  This iterates ALL entities and destroys performance.
  ALWAYS specify component filters.
- **NO Message/Event Confusion:** NEVER use `#[derive(Message)]` with observers.
  NEVER use `#[derive(Event)]` with `MessageWriter`/ `MessageReader`.
  These are distinct systems with incompatible APIs.
- **NO Manual Component Bundles:** NEVER manually spawn `(Transform, Visibility, GlobalTransform)` tuples.
  ALWAYS use `#[require(Transform, Visibility)]` on component markers.
- **NO Archetype Thrashing:** NEVER frequently insert/remove components in update loops (e.g., adding/removing `Running` every frame).
  ALWAYS mutate enum-based state components instead.
- **NO Deprecated Hierarchy API:** NEVER use `Parent` component or `*parent.deref()`.
  NEVER use `.replace_children()`.
  ALWAYS use `ChildOf::parent()` and `.remove::<Children>()` followed by `.add_children()`.
- **NO Resource-Based Entity Data:** NEVER store per-entity data (health, position, inventory) in Resources.
  ALWAYS use Components attached to entities for ECS benefits.
- **NO Universal Query Updates:** NEVER run UI update systems every frame without `Changed<T>` filters.
  This wastes CPU on unchanged data.
- **NO Static Mutable State:** NEVER use `static mut` variables for game state.
  This causes undefined behavior with Bevy's parallel executor.
  ALWAYS use Resources or Components.
- **NO Repeated Asset Loading:** NEVER call `asset_server.load()` inside spawn loops.
  ALWAYS load assets once in startup/resource initialization and clone handles.
- **NO Over-Chaining Systems:** NEVER use `.chain()` on individual system tuples within a single set.
  ONLY chain system sets.
  Individual systems within a set SHOULD run in parallel unless explicitly ordered.
- **NO `Option<Single<T>>` Assumptions:** NEVER assume `Option<Single<T>>` skips on multiple entities (0.15 behavior).
  In 0.17 it returns `None`.
  ALWAYS explicitly check `QuerySingleError::MultipleEntities` if you need skip behavior.
- **NO Old Import Paths:** NEVER import from `bevy::render::camera`, `bevy::render::view`, or `bevy_core_pipeline` for cameras, visibility, or post-processing.
  These crates have been reorganized.

#### Bevy 0.17 Migration Prohibitions

- **NO `Handle::Weak:`** NEVER use `Handle::weak_from_u128()`.
  ALWAYS use `uuid_handle!("uuid-string")` macro for constant handles.
- **NO Old System Set Names:** NEVER use `TransformSystem`, `RenderSet`, `UiSystem` without the `Systems` suffix.
  ALWAYS use `TransformSystems`, `RenderSystems`, `UiSystems`.
- **NO Camera HDR Field:** NEVER attempt to set an `hdr` field on `Camera3d` (e.g., the old `Camera3d { hdr: true }` pattern).
  ALWAYS spawn `Hdr` as a separate component alongside `Camera3d`.
- **NO `EventWriter::send():`** NEVER use `.send()` on `EventWriter` (deprecated 0.16).
  ALWAYS use `.write()` or migrate to `MessageWriter` if using buffered events.
- **NO Window.cursor_options:** NEVER access `window.cursor_options`.
  This field has been split into a separate `CursorOptions` component on the same entity.
- **NO Anchor on Sprite Struct:** NEVER set `sprite.anchor`. `Anchor` is now a required component.
  Use `Anchor::CENTER` constants instead of `Anchor::Center` enum variants.
- **NO SimpleExecutor:** NEVER use `SimpleExecutor` (deprecated).
  ALWAYS use `SingleThreadedExecutor` or `MultiThreadedExecutor`.
- **NO Manual Type Registration:** NEVER call `.register_type::<T>()` for non-generic types when `reflect_auto_register` feature is enabled.
  Bevy registers these automatically.
- **NO Web Builds Without getrandom Config:** NEVER build for `wasm32-unknown-unknown` without setting `RUSTFLAGS='--cfg getrandom_backend="wasm_js"'`.
  This is required due to getrandom 0.3 changes.

## Governance

This constitution establishes the architectural and quality standards for the Brkrs project.
All contributions MUST comply with these principles.

**Amendment Process**:

- Constitutional changes require documentation of rationale
- Version bumped according to semantic versioning (MAJOR.MINOR.PATCH)
- MAJOR: Principle removal/redefinition, architecture changes
- MINOR: New principle additions, new standards
- PATCH: Clarifications, wording improvements

**Compliance**:

- Code reviews MUST verify adherence to principles
- Complexity that violates principles MUST be explicitly justified
- Performance regressions MUST be investigated and resolved
- Cross-platform compatibility MUST be verified before release

**Continuous Improvement**:

- Principles may be refined based on project learnings
- Performance targets may be adjusted based on platform evolution
- Development workflow may be optimized as team/tools evolve

**Version**: 1.3.0 | **Ratified**: 2025-10-30 | **Last Amended**: 2025-12-19
