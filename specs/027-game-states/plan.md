# Implementation Plan: Game States

**Branch**: `027-game-states` | **Date**: 2026-02-08 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/027-game-states/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command.
See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement a comprehensive game state management system with 7 distinct states (Main Menu, Playing, Paused, Fade Out, Fade In, Level Transition, Game Over) that controls game flow using message-based transitions.
The system must freeze gameplay during paused states, manage life loss with fade animations and lives checking, support minimal main menu navigation, and provide robust state transition validation with warning logging.

## Technical Context

**Language/Version**: Rust 1.81 (edition 2021) **Primary Dependencies**: Bevy 0.17.3, bevy_rapier3d 0.32.0, tracing 0.1 **Storage**: N/A (in-memory ECS state only) **Testing**: cargo test (integration tests following TDD workflow) **Target Platform**: Native (Linux/Windows/macOS) and WASM **Project Type**: Single project (Bevy game) **Performance Goals**: 60 FPS maintained during all state transitions and fade animations **Constraints**: State transitions within 1 frame, fade animations 0.5-1.0s, no entity orphaning **Scale/Scope**: 7 game states, ~10 state transition messages, minimal UI (2 menu buttons)

## Constitution Check

*GATE: Must pass before Phase 0 research.*
*Re-check after Phase 1 design.*

### Test-Driven Development (TDD) Compliance

✅ **Tests First**: All acceptance scenarios from spec.md will be converted to failing tests before implementation ✅ **Red Phase Required**: Tests must fail initially (confirm state transitions don't exist yet) ✅ **Approval Gate**: Tests will be committed and reviewed before implementation begins ✅ **Coverage**: Integration tests for each user story + unit tests for state validation logic ✅ **Multi-Frame Persistence**: State changes (paused physics, level state, lives) will be tested across 10+ frames

### Bevy 0.17 Event System Compliance

✅ **Message System Choice**: Using `MessageWriter<StateTransition>` / `MessageReader<StateTransition>` for state transitions

- **Rationale**: State transitions are sequential, critical game logic that benefits from explicit, buffered message passing.
  Messages provide predictable scheduling (read in next frame), better testability (can verify messages were sent), and clear separation between state change requests and state machine logic.
- **NOT using Observers**: Observers are better for immediate reactions (particle effects, sound triggers).
  State transitions need controlled, sequential processing that Messages provide.

✅ **Message-Event Separation**:

- State transition requests use `#[derive(Message)]` with `MessageWriter`
- No mixing with `#[derive(Event)]` or observer systems

### Bevy 0.17 ECS Architecture Compliance

✅ **Fallible Systems**: State machine systems return `()` and use `let Some(x) = opt else { return; }` for missing resources ✅ **Query Specificity**: State-dependent queries will use `With<GameState>` filters as appropriate ✅ **Change Detection**: UI systems will use `Changed<GameState>` to update only when state changes ✅ **Component Mutation**: GameState implemented as enum-based Resource, not inserted/removed components (avoids archetype thrashing) ✅ **Hierarchy Safety**: Not applicable (no scene graph modifications in this feature) ✅ **Asset Handle Reuse**: Not applicable (no asset loading in state machine) ✅ **State-Scoped Cleanup**: Entity cleanup will use `DespawnOnExit(GameState::Playing)` for level-specific entities

### Bevy 0.17 Prohibitions Compliance

✅ **NO Panicking Queries**: All queries use `?` operator and return Results ✅ **NO Unconditional State Overwrites**: State machine only writes to GameState resource when transition messages are received (not every frame) ✅ **NO Message/Event Confusion**: Only using `#[derive(Message)]` with `MessageWriter`, no Event mixing ✅ **NO Universal Updates**: UI updates use `Changed<GameState>` filter ✅ **NO Static Mutable State**: All state stored in Resources/Components

### Additional Compliance

✅ **Coordinate System**: Not applicable (no spatial movement in state machine) ✅ **Initialization Idempotence**: State initialization only occurs once during app startup; subsequent state transitions mutate existing resource ✅ **Performance Standards**: State transitions target 1 frame latency, maintaining 60 FPS

### Gates Status

- [x] TDD workflow defined
- [x] Message system rationale documented  
- [x] No constitutional violations
- [x] Multi-frame persistence tests planned
- [x] State cleanup strategy defined

**Result**: ✅ PASS - Proceed to Phase 0

## Project Structure

### Documentation (this feature)

```text
specs/027-game-states/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── state-transitions.md  # State transition message contracts
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── lib.rs               # Add GameStatesPlugin export
├── main.rs              # Register GameStatesPlugin
├── game_state.rs        # NEW: GameState resource, state machine logic
└── systems/
    ├── mod.rs
    ├── game_state_transitions.rs  # NEW: State transition systems
    ├── pause.rs         # MODIFY: Integrate with GameState
    └── ui/
        ├── main_menu.rs # NEW: Main menu UI
        └── game_over.rs # NEW: Game over UI

tests/
├── game_state_transitions.rs  # NEW: State transition tests (TDD)
├── life_loss_flow.rs          # NEW: Life loss with fade tests
├── pause_state.rs             # NEW: Pause/resume tests
└── main_menu.rs               # NEW: Main menu navigation tests
```

**Structure Decision**: Single project structure following existing Bevy game organization.
Game state management is implemented as a new module `game_state.rs` with supporting systems in `systems/game_state_transitions.rs`.
Tests follow TDD workflow with integration tests in `tests/` directory.

## Phase 0: Research & Unknowns Resolution

✅ **Complete** - See [research.md](research.md)

**Key Decisions**:

- State storage: Resource-based enum (avoids archetype thrashing)
- State transitions: MessageWriter/MessageReader (buffered, testable)
- Pause mechanism: `run_if` system conditions (clean, automatic state preservation)
- Fade animations: Timer + opacity manipulation (simple, performant)
- UI framework: Bevy UI (built-in, minimal dependencies)

**Technical Unknowns Resolved**:

1. Bevy state machine pattern → Resource enum with message-driven transitions
2. Fade animation approach → Timer with BackgroundColor alpha channel
3. Physics freeze → run_if conditions on physics systems
4. Lives check timing → After fade completion in dedicated system
5. Invalid transition logging → tracing::warn! with early return
6. Main menu UI → Bevy UI with button entities

## Phase 1: Design & Contracts

✅ **Complete** - See artifacts below

### Data Model

**Document**: [data-model.md](data-model.md)

**Key Entities**:

- `FadeOverlay` - Full-screen UI with timer for fade animations
- `MainMenuRoot` - Container for menu buttons
- `NewGameButton`, `QuitButton` - Interactive menu buttons
- `GameOverRoot` - Game over screen container

**Key Components**:

- `FadeTimer` - Tracks fade animation progress (0.5-1.0s duration)
- `FadeDirection` - In or Out
- Marker components for button queries

**Resources**:

- `GameState` - Current state enum (7 variants)
- `GameSession` - Persistent data (level, lives, score)

**Messages**:

- `StateTransitionRequest` - Triggers state changes with optional context

**State Transition Matrix**: 7 states with defined valid transitions and triggers

### API Contracts

**Document**: [contracts/state-transitions.md](contracts/state-transitions.md)

**Message API**:

- `StateTransitionRequest` - Buffered message for all state changes
- `StateTransitionContext` - Enum providing context (LifeLoss, LevelComplete, etc.)

**System Contracts**:

- `process_state_transitions` - Validates and applies state changes
- `check_fade_out_completion` - Branches based on lives after fade
- `update_fade_overlay` - Animates fade transparency
- `handle_main_menu_buttons` - Processes button clicks

**Guarantees**:

- Atomicity: State changes within 1 frame
- Idempotence: Multiple identical requests = single transition
- Consistency: Invalid transitions logged and rejected
- Ordering: FIFO message processing

### Implementation Guide

**Document**: [quickstart.md](quickstart.md)

**Sections**:

- 5-minute setup instructions
- 30-minute core implementation guide
- 15-minute testing guide per test suite
- Common tasks and troubleshooting
- Integration points with existing systems
- Performance notes and references

## Phase 2: Agent Context Update

[To be executed: Run .specify/scripts/bash/update-agent-context.sh]

---

## Summary for Phase 2 (Task Breakdown)

**Status**: Ready for `/speckit.tasks` command

The specification is clarified, research complete, and design documented.
All technical unknowns resolved.
The next phase will generate implementation tasks based on:

1. **Data Model**: 5 entities, 3 custom components, 2 resources, 1 message type
2. **Systems**: 4 core systems plus UI interaction systems
3. **Tests**: 4 test suites covering all user stories
4. **Integration**: GameStatesPlugin for app registration

**Estimated Complexity**: Medium (7 states, message-based architecture, fade animations)

**Critical Path**: TDD tests → State machine → Fade animations → UI integration
