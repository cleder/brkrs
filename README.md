# Brkrs

[![CI](https://img.shields.io/github/actions/workflow/status/SairajJadhav08/brkrs/ci.yaml?branch=main&label=CI)](https://github.com/SairajJadhav08/brkrs/actions/workflows/ci.yaml)
[![Documentation Status](https://readthedocs.org/projects/brkrs/badge/?version=latest)](https://brkrs.readthedocs.io/en/latest/?badge=latest)
[![License: AGPL 3.0](https://img.shields.io/badge/license-AGPL%203.0-blue.svg)](./LICENSE-AGPL-3.0)

A Breakout/Arkanoid-style game built with **Rust** 🦀 and the **Bevy** 🐥 engine. This project is intended to be published on [crates.io](https://crates.io).

## Demo

You can play a web version on [GitHub Pages](https://cleder.github.io/brkrs/).

## Documentation

Full documentation is available at **[brkrs.readthedocs.io](https://brkrs.readthedocs.io/)**.

## Technical Overview

Built with **Rust** and **Bevy**, `brkrs` leverages Bevy's ECS (Entity Component System) architecture for game logic and Rapier3D for physics simulation. The game renders in 3D while maintaining 2D gameplay mechanics, utilizing Bevy's flexible rendering pipeline and component-based design for extensible brick behaviors and game state management.

## Testing

Run the test suite with `cargo test` to verify game systems, level loading, and physics behavior.

## How to Run

```bash
cargo run --release
```
