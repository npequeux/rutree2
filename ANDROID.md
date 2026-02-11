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

### Downloading Pre-built Releases

Pre-built Android binaries are available in [GitHub Releases](https://github.com/npequeux/rutree2/releases).

**⚠️ IMPORTANT:** Before downloading, verify that a versioned release with Android binaries exists by visiting the releases page.

1. **Check available releases**:
   - Visit https://github.com/npequeux/rutree2/releases
   - Look for a release with a version tag (like v1.0.0, v0.1.0, etc.)
   - Confirm that the archive files are listed as assets

2. **Download the appropriate archive for your device**:
   - `rutree2-android-arm64.tar.gz` - For most modern Android devices (64-bit ARM)
   - `rutree2-android-armv7.tar.gz` - For older Android devices (32-bit ARM)

3. **Transfer the archive to your device**:
   ```bash
   # Option 1: Use adb
   adb push rutree2-android-arm64.tar.gz /sdcard/

   # Option 2: Download directly in Termux (with version specification)
   # (In Termux terminal)
   cd ~
   VERSION="v1.0.0"  # Replace with actual version from https://github.com/npequeux/rutree2/releases
   
   wget -O rutree2-android-arm64.tar.gz \
     "https://github.com/npequeux/rutree2/releases/download/${VERSION}/rutree2-android-arm64.tar.gz" || \
     { echo "Error: Download failed for version ${VERSION}."; \
       echo "Please check https://github.com/npequeux/rutree2/releases for available versions"; \
       exit 1; }
   
   # Verify download is valid before extracting
   file rutree2-android-arm64.tar.gz  # Should show "gzip compressed data"
   ```

3. **Extract the archive**:
   ```bash
   # In Termux
   tar -xzf rutree2-android-arm64.tar.gz
   chmod +x rutree2
   ./rutree2 --help
   ```

**Note**: All release archives are automatically validated during the build process to ensure:
- The archive is properly gzip-compressed
- The archive can be extracted successfully
- The binary is present and executable
- The binary can run and show help/version information

If you encounter download or extraction issues, try:
1. Verify a versioned release exists at https://github.com/npequeux/rutree2/releases
2. Check the download is complete: `ls -lh rutree2-android-arm64.tar.gz`
3. Verify the file type: `file rutree2-android-arm64.tar.gz` (should show "gzip compressed data", not "HTML document")
4. Test the archive: `gzip -t rutree2-android-arm64.tar.gz`
5. If all else fails, build from source using the instructions below
4. Re-download if the file appears corrupted

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

### Download Issues

If you get a 404 error or "Download failed" message:

1. **Verify a proper versioned release exists**:
   - Visit https://github.com/npequeux/rutree2/releases
   - Look for a release with a version tag (e.g., v1.0.0, v0.1.0)
   - Confirm that `rutree2-android-arm64.tar.gz` or `rutree2-android-armv7.tar.gz` files are listed as assets
   - If no versioned release exists, you need to build from source (see "Building for Android" section)

2. **Check what you downloaded**:
   ```bash
   # Check file type - should show "gzip compressed data"
   file rutree2-android-arm64.tar.gz
   
   # If it shows "HTML document" or "ASCII text", you downloaded an error page
   # This means the release URL doesn't exist yet
   ```

3. **If release doesn't exist yet**:
   - Build from source instead (see "Building for Android" section above)
   - Or wait for the next release that includes Android binaries

### Archive Extraction Issues

If you get errors like "gzip: not in gzip format" or "tar: Child returned status 1" when extracting your downloaded archive:

**Note:** In the examples below, replace `ARCHIVE_NAME` with your specific archive filename:
- For ARM64 devices (most modern phones): `rutree2-android-arm64.tar.gz`
- For ARMv7 devices (older phones): `rutree2-android-armv7.tar.gz`

1. **Verify the archive is complete**:
   ```bash
   ls -lh ARCHIVE_NAME
   # Should show the downloaded archive file
   ```

2. **Check the file type**:
   ```bash
   file ARCHIVE_NAME
   # Should output: "gzip compressed data, from Unix..."
   ```

3. **Test the gzip compression**:
   ```bash
   gzip -t ARCHIVE_NAME
   # Should complete silently (no output = success)
   ```

4. **If the file is not gzipped** (e.g., you see "POSIX tar archive" instead):
   ```bash
   # Try extracting without -z flag
   tar -xf ARCHIVE_NAME
   ```

5. **Re-download the file**:
   - The download may have been interrupted or corrupted
   - Make sure you're downloading the actual binary file, not an HTML error page
   - Check that you downloaded the correct architecture for your device

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
