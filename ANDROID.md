# Android Build Instructions

This document explains how to build and use `rutree2` on Android devices.

## Prerequisites

1. **Android NDK** - Download from [Android Developer site](https://developer.android.com/ndk/downloads)
2. **cargo-ndk** - Install with: `cargo install cargo-ndk`
3. **Android targets for Rust** - Install with: `cargo make install-android-targets`

## Building for Android

### Build for all architectures:
```bash
cargo make build-android
```

This builds binaries for:
- `arm64-v8a` (aarch64) - Modern 64-bit ARM devices
- `armeabi-v7a` (armv7) - Older 32-bit ARM devices
- `x86` - 32-bit x86 emulators/devices
- `x86_64` - 64-bit x86 emulators/devices

### Build for specific architecture:
```bash
# Most modern phones (recommended)
cargo make build-android-arm64

# Or use cargo-ndk directly
cargo ndk -t arm64-v8a build --release
```

## Using on Android Device

### Method 1: Using Termux (Recommended)

1. **Install Termux** from [F-Droid](https://f-droid.org/packages/com.termux/) (recommended) or Google Play Store

2. **Transfer the binary** to your device:
   ```bash
   # On your computer, push the binary to your device
   adb push target/aarch64-linux-android/release/rutree2 /sdcard/
   ```

3. **In Termux**, install and run:
   ```bash
   # Copy from sdcard to Termux home
   cp /sdcard/rutree2 ~/
   chmod +x ~/rutree2
   
   # Run it
   ~/rutree2
   ~/rutree2 --help
   ~/rutree2 /sdcard/Download
   ```

### Method 2: Using ADB Shell

1. **Push the binary**:
   ```bash
   adb push target/aarch64-linux-android/release/rutree2 /data/local/tmp/
   ```

2. **Run via ADB**:
   ```bash
   adb shell
   cd /data/local/tmp
   chmod +x rutree2
   ./rutree2
   ```

### Method 3: Direct Installation (Root Required)

If your device is rooted, you can install the binary to `/system/bin`:
```bash
adb root
adb remount
adb push target/aarch64-linux-android/release/rutree2 /system/bin/
adb shell chmod 755 /system/bin/rutree2
```

## Binary Locations

After building, binaries are located at:
- ARM64: `target/aarch64-linux-android/release/rutree2`
- ARMv7: `target/armv7-linux-androideabi/release/rutree2`
- x86: `target/i686-linux-android/release/rutree2`
- x86_64: `target/x86_64-linux-android/release/rutree2`

## Determining Your Device Architecture

In Termux or ADB shell, run:
```bash
uname -m
```

Common outputs:
- `aarch64` → Use ARM64 binary
- `armv7l` or `armv8l` → Use ARMv7 binary
- `x86_64` → Use x86_64 binary
- `i686` → Use x86 binary

## Usage Examples

```bash
# Display current directory tree
./rutree2

# Show hidden files
./rutree2 --all

# Limit depth
./rutree2 --depth 2

# Browse external storage
./rutree2 /sdcard

# Browse Downloads folder
./rutree2 /sdcard/Download
```

## Troubleshooting

### Permission Denied
- Ensure the binary has execute permissions: `chmod +x rutree2`
- Termux has limited access to external storage on Android 11+. Use `termux-setup-storage` to grant permissions.

### Binary Not Found
- Check you're using the correct architecture for your device
- Verify the file was transferred successfully: `ls -lh rutree2`

### Cannot Access Certain Directories
- Some directories require root access (e.g., `/data`)
- On non-rooted devices, you can access:
  - `/sdcard` (external storage)
  - `/data/local/tmp` (via ADB)
  - Termux home directory (`~`)

## Notes

- The Android binary is standalone and has no external dependencies
- File size is approximately 1.1 MB (unstripped) or 732 KB (stripped)
- Compatible with Android 5.0 (API level 21) and above
- Works in both Termux and standard Android shell environments
