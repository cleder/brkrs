# Profiling & performance smoke tests

This document describes how to profile and run a basic smoke-test that measures frame update timings for level loading and an update loop.
It's intended to help detect obvious regressions after feature changes such as indestructible bricks.

Manual profiling

- Use a CPU profiler (e.g., cargo flamegraph) to get a sample-based profile while the game runs.

```bash
# Install flamegraph (Linux):
cargo install flamegraph

# Run a sample profile of the release executable
cargo flamegraph --manifest-path Cargo.toml --release -- ./target/release/brkrs
```

Automated smoke test

- The repository includes a lightweight smoke test `tests/profile_smoke.rs` which measures average update duration for a small headless run.
  The test *prints* timings and does not fail on thresholds (to avoid flaky CI failures).
  Use the test output for quick comparisons.

Local metrics and CI

- In CI you can run the smoke test and capture its output as a log artifact; for deeper analysis collect profiler artifacts.
- If you find a performance regression, look for increased allocation spikes or system hotspots and open an issue with the flamegraph + test output.
