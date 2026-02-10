#!/bin/bash
# Build script for Android binaries

set -e

echo "Building rutree2 for Android..."
echo ""

# Check if cargo-ndk is installed
if ! command -v cargo-ndk &> /dev/null; then
    echo "cargo-ndk not found. Installing..."
    cargo install cargo-ndk
fi

# Check if Android targets are installed
echo "Checking Rust Android targets..."
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

# Build for all architectures
echo ""
echo "Building for all Android architectures..."
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 build --release

echo ""
echo "Build complete! Binaries are located at:"
echo "  - ARM64 (most devices):  target/aarch64-linux-android/release/rutree2"
echo "  - ARMv7 (older devices): target/armv7-linux-androideabi/release/rutree2"
echo "  - x86:                   target/i686-linux-android/release/rutree2"
echo "  - x86_64:                target/x86_64-linux-android/release/rutree2"
echo ""
echo "To transfer to your Android device:"
echo "  adb push target/aarch64-linux-android/release/rutree2 /sdcard/"
echo ""
echo "See ANDROID.md for detailed usage instructions."
