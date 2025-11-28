# Feature Specification: Textured Visuals Overhaul

**Feature Branch**: `001-textured-visuals`
**Created**: 2025-11-26
**Status**: Draft
**Input**: User description: "Introduce fully textured visuals for all major gameplay objects (ball, paddle, bricks, sidewalls (limiting the playing field), background, and per-level ground plane) with reliable fallback behavior and simple asset-swapping for artists. The ground plane can be customized per level, ball and bricks depend on their type"

## User Scenarios & Testing *(mandatory)*

<!--
IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
you should still have a viable MVP (Minimum Viable Product) that delivers value.

Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
Think of each story as a standalone slice of functionality that can be:

- Developed independently
- Tested independently
- Deployed independently
- Demonstrated to users independently
-->

### User Story 1 - Textured Baseline Objects (Priority: P1)

Players launching any level immediately see fully textured major gameplay objects (ball, paddle, bricks, sidewalls, background) without placeholder geometry, even when some art files are missing or corrupted.

**Why this priority**: This story guarantees the minimum viable visual upgrade that players and stakeholders will notice right away; without it, later customization has no foundation.

**Independent Test**: Start the game with the default asset pack, remove one optional texture, load a level, and verify every tracked object still renders with a textured material while a single warning is emitted for the missing asset.

**Acceptance Scenarios**:

1. **Given** the default texture manifest is installed, **When** a level loads, **Then** the ball, paddle, all wall bricks, sidewalls, and background render using their assigned textured materials within the first second of gameplay.
2. **Given** a texture file referenced by the manifest is missing or unreadable, **When** the level loads, **Then** the system applies a canonical fallback texture for that object class and logs a single warning without blocking play.
3. **Given** a player replays the same level, **When** assets are already cached, **Then** no objects flash untextured between runs.

---

### User Story 2 - Type-Driven Materials (Priority: P2)

Ball variants and brick types automatically use textures that match their gameplay classification (e.g., heavy ball, explosive bricks) so players can read game state visually.

**Why this priority**: Visual differentiation reduces mistakes and communicates power-ups or threats without extra UI, which improves usability and polish.

**Independent Test**: Spawn two ball types and three brick types via debug commands and verify each switches to the corresponding texture immediately when its type changes, without requiring a level reload.

**Acceptance Scenarios**:

1. **Given** a level that mixes brick types defined in the matrix, **When** gameplay begins, **Then** each brick displays the texture tied to its type id (e.g., armored vs. explosive) and preserves it through hits.
2. **Given** the player activates a ball power-up that changes the ball type, **When** the type flag updates, **Then** the visible texture swaps to the variant assigned to that type within 0.1 seconds and without flicker.
3. **Given** a type is missing a specific texture override, **When** the object spawns, **Then** it inherits the default material for its class while logging that a type-specific asset is absent.

---

### User Story 3 - Per-Level Presentation Pack (Priority: P3)

Level designers and artists can give each level a distinct look by assigning ground plane textures, backgrounds, and optional sidewall skins per level and by swapping art files without code changes.

**Why this priority**: Distinct presentation keeps the campaign fresh and empowers non-programmers to iterate on visuals independently.

**Independent Test**: Configure three different levels with unique ground and backdrop textures, restart the game after swapping one texture file, and verify the new art appears in only the intended level.

**Acceptance Scenarios**:

1. **Given** a level definition that references a custom ground plane and background, **When** the level loads, **Then** those overrides apply while unaffected levels continue to use defaults.
2. **Given** an artist replaces a texture file (or updates a manifest entry) and restarts the level, **When** gameplay resumes, **Then** the new art appears without modifying code or rebuilding the game.
3. **Given** a level omits per-level overrides, **When** it loads, **Then** the baseline texture pack applies automatically with no runtime errors.

---

### User Story 4 - Level Switch Preview (Priority: P4)

Artists and QA can cycle through levels in sequence by pressing the **L** key so they can quickly verify how textures change per level without restarting the application.

**Why this priority**: Rapid validation of per-level art accelerates iteration; while not core to gameplay, it dramatically shortens the feedback loop for the visual overhaul.

**Independent Test**: Launch the game, press **L** repeatedly, confirm each press loads the next level (wrapping after the last) with the proper texture set, and ensure gameplay resumes immediately.

**Acceptance Scenarios**:

1. **Given** multiple levels are defined, **When** the player presses **L**, **Then** the next level loads within 2 seconds and spawns fully textured entities.
2. **Given** the current level is the last available, **When** the player presses **L**, **Then** the system wraps to level 1 (or an agreed fallback) without crashing or losing texture state.
3. **Given** the game is in the middle of a respawn or overlay animation, **When** **L** is pressed, **Then** the system either queues the switch or safely interrupts, ensuring no partial texture data leaks between levels.

---

[Add more user stories as needed, each with an assigned priority]

### Edge Cases

- Missing or corrupted texture files should fall back to the canonical default texture for that object class while emitting a single warning per session.
- A level referencing a texture id that does not exist must continue loading using defaults and mark the discrepancy for designers.
- Asset bundles larger than the memory budget should be detected at load time so QA receives guidance before shipping oversized textures.
- When artists add new textures but forget to update UV scaling metadata, the system should clamp to safe defaults to avoid stretching.
- Headless or low-spec builds (e.g., wasm) must gracefully switch to simplified materials if a texture format is unsupported.
- When level switching occurs via the **L** key during active gameplay, the system must either confirm the switch or debounce rapid presses so the queue cannot overflow.
- If **L** is pressed while no additional levels exist (e.g., only one defined), the system should reload the same level and still ensure textures reinitialize correctly.

## Implementation Lessons Learned

### Material Hot-Reload Strategy

**Learning**: When the texture manifest changes, creating entirely new material handles breaks existing entity references. Materials must be updated in-place rather than recreated to ensure live hot-reload without requiring level restart.

**Impact**: Artists can now edit `manifest.ron` UV scales, roughness values, or texture paths and see changes immediately in the running game without pressing 'L' or restarting.

**Implementation Note**: Material bank rebuild logic must reuse existing handles and call `materials.get_mut()` to update properties rather than `materials.add()` to create new handles.

### Texture Addressing Mode for Tiling

**Learning**: By default, Bevy textures use `ClampToEdge` addressing mode, which causes textures to appear stretched or positioned in corners when UV scale exceeds 1.0. Tiling requires explicit `Repeat` mode on the sampler.

**Impact**: Ground plane and wall textures now tile correctly when UV scale is increased (e.g., `(10.0, 8.0)`), creating seamless repeated patterns instead of a single stretched image.

**Implementation Note**: A dedicated system watches for `AssetEvent::Added` on images loaded for texture profiles and updates their sampler to use `ImageAddressMode::Repeat` for all axes.

### Asynchronous Asset Loading Timing

**Learning**: Entities spawned during `Startup` (like paddle and bricks) may render before the texture manifest finishes loading asynchronously, resulting in initial frames showing fallback materials even though proper textures are defined.

**Impact**: Game now shows textures immediately on startup without requiring a level switch. Players no longer see a flash of red/gray materials before textures appear.

**Implementation Note**: A dedicated system monitors when `CanonicalMaterialHandles` becomes ready and triggers once to update all existing paddle and brick materials. Queries must be disjoint using `Without<T>` to avoid conflicts.

<!--
IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
you should still have a viable MVP (Minimum Viable Product) that delivers value.

Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
Think of each story as a standalone slice of functionality that can be:

- Developed independently
- Tested independently
- Deployed independently
- Demonstrated to users independently
-->
**Impact**: Feature is now active by default, allowing immediate testing and iteration without remembering to add `--features texture_manifest` to every cargo command.
