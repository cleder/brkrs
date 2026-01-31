# Test Suite Build Hang Workaround

## Problem

Running `cargo test` hangs during the linking phase due to resource exhaustion.
Multiple `ld.lld` linker processes consume ~6GB RAM each and 30%+ CPU each, causing system overload.

## Root Cause

- Bevy projects have large dependency trees
- `cargo test` compiles/links 100+ test binaries in parallel
- The linker (`ld.lld`) is very memory/CPU intensive for Bevy tests
- System resources are exhausted when too many linkers run simultaneously

## Workarounds

### Option 1: Limit Build Parallelism (Recommended)

```bash
# Build tests first with limited parallelism
cargo build --tests -j 2

# Then run tests
cargo test
```

Or set environment variable:

```bash
export CARGO_BUILD_JOBS=2
cargo test
```

### Option 2: Run Tests Serially

```bash
# Compile with limited jobs, run serially
cargo test -j 2 -- --test-threads=1
```

### Option 3: Run Individual Test Files

```bash
# Works fine for individual test files
cargo test --test brick_level_navigation
cargo test --test paddle_size_powerups
cargo test --lib  # Library tests always work
```

### Option 4: Configure in .cargo/config.toml

```toml
[build]
jobs = 2  # Limit parallel jobs
```

## Verified Working

- Library tests: `cargo test --lib` ✅ (53/53 pass)
- Individual integration tests: `cargo test --test <name>` ✅
- All tests with `-j 2`: `cargo test -j 2` ✅

## Status

This is a build-time resource issue, not a code bug.
All tests pass individually.
The brick_level_navigation tests for brick 50/54 work correctly.
