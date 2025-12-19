# Quickstart: Systems Constitution Refactor

**Branch**: `copilot/refactor-legacy-code-systems` | **Feature**: 011-refactor-systems

## Overview

This guide provides quick verification steps for the systems refactor work.

## Prerequisites

- Rust toolchain (1.81+)
- Bevy dependencies installed
- Repository cloned and on correct branch

## Verification Workflow

### 1. Run Tests

```bash
# Run all tests
cargo test

# Run only systems compliance tests
cargo test systems_

# Run specific test file
cargo test --test systems_fallible
```

### 2. Format Check

```bash
# Check formatting
cargo fmt --all --check

# Auto-format
cargo fmt --all
```

### 3. Clippy Linting

```bash
# Run clippy with all features
cargo clippy --all-targets --all-features

# Fix auto-fixable issues
cargo clippy --all-targets --all-features --fix
```

### 4. Bevy Lint

```bash
# Run bevy-specific linting
bevy lint
```

### 5. Build Verification

```bash
# Debug build
cargo build

# Release build
cargo build --release

# WASM build
cargo build --target wasm32-unknown-unknown --release
```

### 6. Manual Testing

```bash
# Run the game (native)
cargo run

# Run with dev features
cargo run --features dev
```

## Constitution Compliance Checks

### Fallible Systems Checklist

- [ ] All systems return `Result<(), Box<dyn Error>>` or equivalent
- [ ] No `.unwrap()` calls on query results
- [ ] No `.single()` or `.single_mut()` without error handling
- [ ] Use `get_single()` with `?` operator or early return

### Change Detection Checklist

- [ ] Reactive systems use `Changed<T>` filters
- [ ] No per-frame updates when data unchanged
- [ ] Empty query checks for early return

### Asset Handle Caching Checklist

- [ ] Assets loaded once in startup systems
- [ ] Handles stored in Resources
- [ ] No repeated `asset_server.load()` calls in spawn/update loops

### System Organization Checklist

- [ ] System sets defined with `*Systems` suffix
- [ ] `.configure_sets()` used to order sets
- [ ] Individual systems grouped by set
- [ ] No over-chaining of individual systems

### Plugin Architecture Checklist

- [ ] Each subsystem has a Plugin implementation
- [ ] Plugins are self-contained
- [ ] Resources registered in plugin `build()`
- [ ] Systems registered in plugin `build()`

## Common Issues

### Issue: Tests fail with query errors

**Solution**: Check that systems use fallible query patterns (`get_single()` with `?`)

### Issue: Clippy warns about unused Result

**Solution**: Ensure system functions return `Result` and callers handle errors

### Issue: Bevy lint warns about system organization

**Solution**: Use system sets instead of chaining individual systems

### Issue: WASM build fails

**Solution**: Verify `getrandom_backend="wasm_js"` is set in RUSTFLAGS

## Related Documentation

- [Constitution](.specify/memory/constitution.md) - Architectural principles
- [Plan](plan.md) - Implementation strategy
- [Tasks](tasks.md) - Detailed task breakdown
- [Compliance Audit](compliance-audit.md) - Violation findings
