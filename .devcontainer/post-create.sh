#!/usr/bin/env bash
set -euo pipefail

# Install system dependencies required for Bevy and native crates
sudo apt-get update
sudo apt-get install --no-install-recommends -y \
    pkg-config \
    libasound2-dev \
    libudev-dev \
    libwayland-dev \
    libxkbcommon-dev

# Set PKG_CONFIG_PATH for native libraries
export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig"
echo 'export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig"' >> ~/.bashrc

# Fetch dependencies to pre-populate the cache
cargo fetch --locked

# Pre-compile dependencies and tests to warm up the build cache
cargo test --no-run --all-features || true
