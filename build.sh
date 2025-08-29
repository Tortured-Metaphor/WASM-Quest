#!/bin/bash

echo "Building Medieval Platformer WASM..."

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM module
wasm-pack build --target web --out-dir pkg

echo "Build complete! To run the game:"
echo "1. Start a local web server: python3 -m http.server 8000"
echo "2. Open http://localhost:8000 in your browser"