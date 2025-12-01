# Brkrs

[![CI](https://github.com/cleder/brkrs/workflows/CI/badge.svg)](https://github.com/cleder/brkrs/actions)
[![Documentation Status](https://readthedocs.org/projects/brkrs/badge/?version=latest)](https://brkrs.readthedocs.io/en/latest/?badge=latest)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/cleder/brkrs)

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
