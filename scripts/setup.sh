#!/bin/bash
# Setup script for RustForge OS development

set -e

echo "Setting up RustForge OS development environment..."

# Install Rust nightly
echo "Installing Rust nightly toolchain..."
rustup toolchain install nightly
rustup default nightly

# Install required components
echo "Installing required Rust components..."
rustup component add rust-src
rustup component add llvm-tools-preview

# Install bootimage
echo "Installing bootimage tool..."
cargo install bootimage

# Install QEMU (platform-specific)
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "Detected Linux. Install QEMU with:"
    echo "  sudo apt-get install qemu-system-x86"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Detected macOS. Install QEMU with:"
    echo "  brew install qemu"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    echo "Detected Windows. Download QEMU from:"
    echo "  https://www.qemu.org/download/#windows"
fi

echo ""
echo "Setup complete! Next steps:"
echo "  1. Install QEMU if not already installed"
echo "  2. Run 'make build' to build the project"
echo "  3. Run 'make run' to test the kernel in QEMU"
