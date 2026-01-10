# Quickstart: Extra Ball Brick (Brick 41)

1. **Update assets**: Add brick 41 metadata (durability 1, score 0, unique destruction sound handle) to level/brick registries; add audio asset path for the unique sound.
2. **Wire handlers**: In the brick hit system, add branch for id 41 that enqueues `LifeAwardMessage { delta: +1 }`, enqueues `AudioMessage` with the brick-41 sound (fallback to generic brick sound), and marks the brick for despawn.
3. **Clamp lives**: Ensure life consumer clamps to max lives; verify no score mutation for brick 41.
4. **Tests first**: Add failing tests covering life increment, score unchanged, unique sound once, and multi-ball single-grant; commit failing tests, then implement.
5. **Run checks**: `cargo test`, `cargo clippy --all-targets --all-features`, `cargo fmt --all`, `bevy lint`.
6. **WASM sanity**: Build/run WASM to confirm audio and life handling are platform-safe.
