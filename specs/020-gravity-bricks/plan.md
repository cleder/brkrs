# Implementation Plan: Gravity Switching Bricks

**Branch**: `020-gravity-bricks` | **Date**: 2026-01-10 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/020-gravity-bricks/spec.md`

## Summary

Implement 5 gravity brick types (indices 21-25) that dynamically modify the ball's physics gravity when destroyed.
The core mechanic applies gravity changes via the `GravityChanged` message to the physics system.
Gravity resets to level default (zero gravity fallback) on ball loss.
Supports varied difficulty: zero gravity (float), moon gravity (2G), earth gravity (10G), high gravity (20G), and random direction gravity (Queer Gravity).
This feature enhances level design flexibility and gameplay challenge progression.

## Technical Context

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, `rand` crate for RNG **Storage**: In-memory ECS state only (no persistent storage); level metadata in RON format (`assets/levels/*.ron`) **Testing**: `cargo test`; test-driven development required (tests before implementation) **Target Platform**: Native (Linux/Windows/macOS) + WASM **Project Type**: Single project (monolithic game with ECS systems) **Performance Goals**: 60 FPS (gravity calculations are lightweight; physics updates via Rapier 3D) **Constraints**: Physics state must never corrupt; gravity applies only to ball's rigid body; no frame synchronization required for reset timing **Scale/Scope**: 5 gravity brick types, 1 new message type (`GravityChanged`), 2-3 new systems (gravity brick destruction handler, gravity reset on life loss, optional: gravity configuration loader)

## Constitution Check

### ECS Architecture Compliance ✅

- Game logic expressed as systems operating on components
- Physics-driven via Rapier 3D rigid body forces
- Feature components (gravity brick markers) attached to brick entities
- Systems are pure functions of query inputs
- Change detection used for reactive behavior (ball loss detection)

**Verdict**: ✅ Compliant

### Bevy 0.17 Event & Message System Compliance ✅

**Event System Choice: Messages (`MessageWriter<GravityChanged>`)**

- **Justification**: Gravity changes are buffered, frame-agnostic updates that affect physics simulation.
  - Multiple gravity brick destructions in sequence are batched and applied deterministically
  - Physics updates occur in a dedicated schedule phase (Rapier's simulate phase)
  - No immediate UI/sound reactivity required for gravity change itself
  - Messages allow proper decoupling of brick destruction from physics gravity application

- **Alternative Rejected**: Observers would be overkill for buffered physics updates; they add unnecessary reactivity overhead and don't align with the physics simulation schedule.

**Implementation Details**:

- Define `#[derive(Message)] pub struct GravityChanged { pub gravity: Vec3 }`
- Write via `MessageWriter<GravityChanged>` in brick destruction system
- Read via `MessageReader<GravityChanged>` in physics gravity update system
- No custom `Trigger<T>` or `Event` derive needed; Messages are correct choice

**Verdict**: ✅ Compliant with Bevy 0.17 Message-Event Separation mandate

### Coordinate System Compliance ✅

- **Axes Used**: Y-axis (vertical, gravity direction), XZ plane (horizontal, zero gravity respects these)
- **Gravity Convention**: Positive Y = up (Bevy standard); gravity applied as negative Y acceleration for normal gravity
- **Ball Physics**: Uses `Velocity` and `GravityScale` components from `bevy_rapier3d`; gravity changes modify the effective gravity vector applied to physics simulation
- **No `LockedAxes` Needed**: Ball is not constrained; gravity applies uniformly to X, Y, Z as specified

**Verdict**: ✅ Coordinate system clearly documented; no constraints needed

### Physics & Safety Compliance ✅

- Systems do not panic on query outcomes; use `?` operator and `Result` types
- Queries filter entities correctly (`With<Ball>`, `With<RigidBody>`, etc.)
- Assets (brick textures, audio) loaded once and stored in Resources
- No manual transform manipulation for physics bodies; use Rapier forces/velocities
- No dangerous `.unwrap()` on physics queries

**Verdict**: ✅ Compliant with physics-driven gameplay and safety requirements

### TDD & Testing Requirement ✅

- Tests MUST be written before implementation
- A failing-test commit (red) MUST exist in branch history before implementation commits
- Tests MUST cover:
  - Gravity application for each brick type (21-25)
  - Gravity reset on ball loss
  - Score updates for each gravity brick type
  - Sequential gravity changes (message batching)
  - Zero gravity fallback when level config undefined
  - Queer Gravity RNG within specified ranges
- Tests reviewed and approved before implementation begins

**Verdict**: ✅ TDD methodology required; testing strategy defined below

**Constitution Check Overall**: ✅ **PASS** - Feature complies with all Bevy 0.17 mandates, ECS architecture, physics-driven design, and TDD requirements.

## Project Structure

### Documentation (this feature)

```text
specs/020-gravity-bricks/
├── spec.md                  # Feature specification ✅ Complete
├── plan.md                  # This file (implementation plan)
├── research.md              # Phase 0 research (PENDING)
├── data-model.md            # Phase 1 data model (PENDING)
├── quickstart.md            # Phase 1 quickstart guide (PENDING)
├── contracts/               # Phase 1 API contracts (PENDING)
│   ├── gravity-message.rs   # GravityChanged message definition
│   └── events-schema.md     # Message flow diagram
├── checklists/
│   └── requirements.md      # ✅ Quality checklist (complete)
└── tasks.md                 # Phase 2 task breakdown (PENDING - from /speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── lib.rs                   # Main library
├── main.rs                  # Game entry point
├── systems/
│   ├── mod.rs               # System module exports
│   ├── gravity.rs           # NEW: GravityChanged message reader + physics gravity application
│   ├── brick_destruction.rs # MODIFY: Add gravity brick detection + message writing
│   ├── ball_lives.rs        # MODIFY: Add gravity reset trigger on life loss
│   └── [existing systems]
├── components/
│   ├── mod.rs               # Component module exports
│   ├── brick.rs             # MODIFY: Add GravityBrick marker component
│   └── [existing components]
├── physics_config.rs        # MODIFY: Add GravityConfiguration resource
└── [existing modules]

tests/
├── gravity_bricks.rs        # NEW: Comprehensive gravity brick tests
├── [existing test files]
```

**Structure Decision**: Single project (monolithic game).
All gravity brick logic integrated into existing modular system architecture.
No new projects or sub-packages needed.
Changes are localized to:

- New `gravity.rs` system module
- Modifications to `brick_destruction.rs` (add gravity brick handler)
- Modifications to `ball_lives.rs` (add gravity reset)
- New `GravityBrick` component in `components/brick.rs`
- New `GravityConfiguration` resource in `physics_config.rs`
- New test module `gravity_bricks.rs`

### Level Metadata Structure

```text
assets/levels/
├── level_1.ron              # MODIFY: Add default_gravity field (optional)
├── level_2.ron              # MODIFY: Add default_gravity field (optional)
├── [existing level files]   # Unchanged (auto-fallback to zero gravity)
```

**Level Metadata Format** (RON):

```ron
// assets/levels/level_1.ron
(
    name: "Level 1",
    bricks: [
        // ... brick definitions ...
    ],
    default_gravity: Some((0.0, 10.0, 0.0)),  // NEW: Optional gravity config
    // ... other fields ...
)
```

Fallback: If `default_gravity` is `None` or missing, system defaults to `(0.0, 0.0, 0.0)` (zero gravity).

## Complexity Tracking

No complexity violations.
Feature is straightforward:

- Single new message type (`GravityChanged`)
- Three system modifications (gravity application, brick destruction handler, life loss handler)
- One new component (`GravityBrick` marker)
- One new resource (`GravityConfiguration`)
- Level metadata optional extension (backwards compatible)

| Item | Status | Justification |
|------|--------|---------------|
| Message system vs Observers | ✅ Messages chosen | Buffered physics updates, no immediate reactivity required |
| Physics scope (ball-only) | ✅ Pragmatic | Preserves gameplay balance; paddle/enemy physics unchanged |
| RNG system (`rand` crate) | ✅ Existing dependency | Deterministic, testable; leverages standard Rust RNG |
| Zero gravity fallback | ✅ Non-breaking | Existing levels auto-fallback; no migration burden |

## Implementation Strategy

### Phase 0: Research (PENDING)

Deliverable: `research.md` documenting:

- Bevy 0.17.3 physics gravity application best practices (Rapier 3D)
- Message queue depth and scheduling implications
- RNG distribution testing (uniform random in specified ranges for Queer Gravity)
- Level metadata loading patterns in existing codebase
- Existing brick destruction event/message patterns

### Phase 1: Design & Data Model (PENDING)

Deliverables:

- `data-model.md`: Entity/component/resource definitions
- `contracts/gravity-message.rs`: Message definition
- `contracts/events-schema.md`: Message flow diagram
- `quickstart.md`: Integration quick start guide

**Key Design Decisions**:

1. **GravityChanged Message**

   ```rust
   #[derive(Message, Clone, Copy, Debug)]
   pub struct GravityChanged {
       pub gravity: Vec3,
   }
   ```

2. **GravityConfiguration Resource**

   ```rust
   #[derive(Resource, Clone, Copy, Debug)]
   pub struct GravityConfiguration {
       pub current: Vec3,
       pub level_default: Vec3,
   }
   ```

3. **GravityBrick Component**

   ```rust
   #[derive(Component, Clone, Copy, Debug)]
   pub struct GravityBrick {
       pub index: u32,  // 21-25
       pub gravity: Vec3,
   }
   ```

4. **System Schedule & Ordering**

   **System Registration**:
   - `gravity_configuration_loader_system`: `Startup` (load level default gravity once)
   - `brick_destruction_gravity_handler`: `Update` (detect destroyed gravity bricks, write messages)
   - `gravity_application_system`: `PhysicsUpdate` (read messages, update gravity config)
   - `gravity_reset_on_life_loss_system`: `PostUpdate`, **before** ball respawn system (reset gravity before next ball spawns)

   **Critical Ordering Constraint**:

   ```text
   Update Schedule
       ↓
   [brick_destruction_gravity_handler sends GravityChanged messages]
       ↓
   PhysicsUpdate Schedule
       ↓
   [gravity_application_system reads messages, updates GravityConfiguration::current]
       ↓
   PostUpdate Schedule
       ↓
   [gravity_reset_on_life_loss_system resets to level_default on ball loss]
       ↓
   [Ball respawn occurs with reset gravity]
   ```

   This ensures gravity messages are processed before physics simulation, and gravity is reset before next ball spawn.

### Phase 2: Task Breakdown (PENDING)

Will be generated by `/speckit.tasks` command.
Expected tasks:

1. Define `GravityChanged` message and supporting types
2. Implement gravity brick detection and message writing in brick destruction system
3. Implement gravity application system reading from `MessageReader<GravityChanged>`
4. Implement gravity reset system listening for ball loss events
5. Implement `GravityConfiguration` resource loading from level metadata
6. Add `GravityBrick` component marker and brick index handling
7. Write comprehensive tests (TDD - before implementation)
8. Update level RON files with optional `default_gravity` field
9. Integration testing with existing brick destruction and ball physics
10. Performance profiling and WASM compatibility verification

## Next Steps

1. **Phase 0 Research**: Run research task to document physics patterns, message scheduling, RNG best practices
2. **Phase 1 Design**: Create data-model.md, message contracts, quickstart guide
3. **Phase 2 Planning**: Run `/speckit.tasks` to break down into atomic implementation tasks
4. **TDD Testing**: Write failing tests before implementation begins
5. **Implementation**: Follow task breakdown with tests-first discipline
6. **Integration**: Verify gravity mechanics work with existing brick destruction, ball physics, level loading
7. **Validation**: Run full test suite, profile performance (60 FPS target), test WASM build

---

**Status**: ✅ Implementation plan complete.
Ready for Phase 0 research or direct task breakdown.
