#!/bin/bash

# Exit on error
set -e

# Clean previous builds
echo "Cleaning previous builds..."
cargo clean

# Build for Linux (current platform)
echo "Building for Linux..."
cargo build --release

# Build for Windows (cross-compilation)
echo "Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu

# Print the results
echo "Build completed!"
echo "Linux binary: target/release/csv-to-sqlite"
echo "Windows binary: target/x86_64-pc-windows-gnu/release/csv-to-sqlite.exe"
