#!/bin/bash
# Build script for WASM version of brkrs
# This generates the necessary JavaScript files from the compiled WASM binary

set -e

echo "Building brkrs for WASM..."

# Check if wasm-bindgen is installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "Error: wasm-bindgen is not installed."
    echo "Install it with: cargo install wasm-bindgen-cli"
    exit 1
fi

# Check if WASM binary exists
WASM_BINARY="target/wasm32-unknown-unknown/release/brkrs.wasm"
if [ ! -f "$WASM_BINARY" ]; then
    echo "Error: WASM binary not found at $WASM_BINARY"
    echo "Build it first with: cargo build --release --target wasm32-unknown-unknown"
    exit 1
fi

# Generate JavaScript bindings
echo "Generating JavaScript bindings..."
wasm-bindgen --out-dir . --target web "$WASM_BINARY"

# Copy assets if they exist
if [ -d "../assets" ]; then
    echo "Copying assets..."
    cp -r ../assets . 2>/dev/null || true
fi

echo "Build complete! Generated files:"
echo "  - brkrs.js"
echo "  - brkrs_bg.wasm"
echo ""
echo "You can now open index.html in a web server (not file://)"
echo "Example: python3 -m http.server 8080"

