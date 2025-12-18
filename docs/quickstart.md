# Quickstart

Get brkrs running on your machine in under 10 minutes.

## Prerequisites

Before you begin, ensure you have:

- **Rust toolchain** (1.81 or later) — Install via [rustup](https://rustup.rs/)
- **Git** — For cloning the repository
- **Graphics drivers** — OpenGL 3.3+ or Vulkan support

### Platform-specific requirements

**Linux (Ubuntu/Debian)**:

```bash
sudo apt install build-essential pkg-config libasound2-dev libudev-dev
```

**Linux (Fedora)**:

```bash
sudo dnf install gcc-c++ alsa-lib-devel systemd-devel
```

**macOS**: Xcode Command Line Tools (usually pre-installed):

```bash
xcode-select --install
```

**Windows**: Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with "Desktop development with C++".

## Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/cleder/brkrs.git
   cd brkrs
   ```

2. **Build and run**

   ```bash
   cargo run --release
   ```

   The first build takes several minutes to compile dependencies.
   Subsequent builds are much faster.

3. **Play!**

   The game starts with Level 1.
   Use your mouse to control the paddle.

## Controls

| Action | Input |
|--------|-------|
| Move paddle | Mouse movement |
| Rotate paddle | Mouse scroll wheel |
| Pause game | ESC |
| Resume game | Left mouse click |
| Toggle cheat mode (developer/test) | `G` — toggles Cheat Mode, resets score to 0 and shows a "CHEAT MODE" indicator; if toggled during Game Over, sets lives to 3 and dismisses the Game Over overlay (does not reset the current level) |

## Playing a specific level

To start on a different level, modify the level number in the source or use the level switcher (if available).

Levels are stored in `assets/levels/` as RON files:

- `level_001.ron` — First level
- `level_002.ron` — Second level
- etc.

## Web version

A WASM build is available at: <https://cleder.github.io/brkrs/>

No installation required — just open the link in a modern browser (Chrome, Firefox, Safari, Edge).

## Next steps

- Having issues?
  See {doc} `troubleshooting`
- Want to contribute?
  Read the {doc} `developer-guide`
- Curious about the architecture?
  Check {doc} `architecture`
