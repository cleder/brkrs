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

- Manual testing required for gameplay features
- Integration testing for critical game mechanics when feasible
- Test both native and WASM builds before releases
- every function should be unit tested

- Performance profiling for new systems

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

**Version**: 1.1.0 | **Ratified**: 2025-10-30 | **Last Amended**: 2025-11-29
