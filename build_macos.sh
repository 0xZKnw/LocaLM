#!/bin/bash
# Build script for macOS with Metal GPU support
# Run this on your Mac

set -e

echo "Building LocalClaw for macOS with Metal GPU support..."

# Install prerequisites if needed:
# brew install cmake rust

# Build with Metal (automatic on macOS)
# Metal is enabled by default when building on macOS - no special features needed
cargo build --release

# Or to ensure Metal is explicitly enabled:
# CMAKE_ARGS="-DGGML_METAL=on" cargo build --release

echo "Build complete!"
echo "Executable: target/release/clawrs"
echo ""
echo "To create a distributable app bundle, you can use:"
echo "cargo bundle --release"
