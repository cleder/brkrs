# Implementation Notes: Systems Constitution Refactor

**Branch**: `copilot/refactor-legacy-code-systems` | **Date**: 2025-12-19

## Progress Log

### 2025-12-19: Phase 1 & 2 Complete ✓

**Phase 1 - Setup Complete**:
- Created feature specification (spec.md)
- Created implementation plan (plan.md)
- Created comprehensive compliance audit (compliance-audit.md)
- Created detailed refactoring plan (refactoring-plan.md)
- Created quickstart guide (quickstart.md)
- Created task breakdown (tasks.md)

**Phase 2 - Foundation Complete**:
- Documented fallible systems pattern in src/systems/mod.rs
- Created docs/systems.md with comprehensive system patterns
- Defined error handling policies (query failures, resource failures)
- Documented Constitution compliance patterns

**Audit Findings**:
- 13+ system functions need Result return types
- 1 panic risk in respawn.rs:603 (.unwrap() on Option)
- 2 components need #[require()] attributes  
- 6 plugins lack system sets organization
- 2 subsystems lack dedicated plugins

**Next Steps**:
1. Phase 3: Write and run US1 tests (compliance audit verification)
2. Get approval for failing tests
3. Phase 4: Implement refactoring (add Result types, fix panic, etc.)
4. Phase 5: Behavior preservation testing

---

## Key Decisions

### Error Type Strategy

Using `Result<(), Box<dyn std::error::Error>>` as the standard return type for systems. This provides:
- Compatibility with `?` operator
- Flexibility for different error types
- Clear error propagation
- Minimal boilerplate

Alternative considered: Custom `SystemsError` enum
- Pros: More specific error types
- Cons: More boilerplate, overkill for simple early returns
- Decision: Use Box<dyn Error> for now, can add typed errors later if needed

### System Sets Organization

Following the respawn.rs pattern (best practice in codebase):
- Define `*Systems` enum for each plugin
- Use `.configure_sets()` to order sets
- Group related systems in same set
- Only chain sets, not individual systems

---

## Open Questions

1. Should we use a custom `SystemsError` enum or `Box<dyn Error>`?
   - **Decision**: Start with `Box<dyn Error>`, evaluate if custom enum needed later

2. Should GridOverlay be in lib.rs or its own component file?
   - **TODO**: Check where GridOverlay is currently defined

3. Should we create a docs/systems.md file or add to docs/developer-guide.md?
   - **Decision**: Create docs/systems.md (parallel to docs/ui-systems.md)

---

## Testing Notes

### Test Files to Create

1. `tests/systems_compliance_audit.rs` - Verify audit artifact completeness
2. `tests/systems_fallible.rs` - Verify all systems return Result
3. `tests/systems_assets.rs` - Verify asset handle caching
4. `tests/systems_change_detection.rs` - Verify Changed<T> usage (if applicable)
5. `tests/systems_audio.rs` - Audio behavior tests
6. `tests/systems_scoring.rs` - Scoring behavior tests  
7. `tests/systems_paddle_size.rs` - Paddle size effects tests
8. `tests/systems_respawn.rs` - Respawn behavior tests

### Manual Testing Checklist

- [ ] Native build and run
- [ ] WASM build and browser test
- [ ] Audio plays on game events
- [ ] Scoring awards points correctly
- [ ] Paddle size effects work (shrink/enlarge)
- [ ] Ball respawns correctly
- [ ] Lives tracking works
- [ ] Level switching works
- [ ] Multi-hit bricks transition correctly
- [ ] Cheat mode toggles

---

## Code Patterns

### Fallible System Pattern

```rust
fn my_system(
    query: Query<&Component>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Early return on expected failures
    if query.is_empty() {
        return Ok(());
    }
    
    // Use ? operator for queries
    let component = query.get_single()?;
    
    // ... system logic
    
    Ok(())
}
```

### Required Components Pattern

```rust
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct MyMarker;

// Spawning
commands.spawn(MyMarker); // Transform and Visibility added automatically
```

### System Sets Pattern

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum MySystems {
    Input,
    Logic,
    Cleanup,
}

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (
            MySystems::Input,
            MySystems::Logic,
            MySystems::Cleanup,
        ).chain());
        
        app.add_systems(Update, (
            system1.in_set(MySystems::Input),
            system2.in_set(MySystems::Logic),
            system3.in_set(MySystems::Cleanup),
        ));
    }
}
```

---

## References

- Constitution: `.specify/memory/constitution.md` (v1.3.0)
- UI Refactor: `specs/010-refactor/` (pattern to follow)
- Bevy 0.17 Migration Guide: https://bevyengine.org/learn/migration-guides/0-16-to-0-17/
- Respawn System: `src/systems/respawn.rs` (best practice example for system sets)
