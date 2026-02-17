# Quick Start: Testing rutree2 on Android

This guide will get you started testing `rutree2` on your Android phone in just a few minutes.

## Option A: Download Pre-built Binary (Easiest)

**⚠️ IMPORTANT:** Before using this option, verify that a proper release with Android binaries exists:

1. Visit https://github.com/npequeux/rutree2/releases
2. Look for a release with a version tag (like v1.0.0, v0.1.0, etc.)
3. Confirm that `rutree2-android-arm64.tar.gz` or `rutree2-android-armv7.tar.gz` files are listed as assets

If no proper versioned release exists yet, use **Option B to build from source** instead.

1. **Download the binary** for your device:
   
   In Termux or via browser, download:
   ```bash
   # For ARM64 devices (most modern phones from 2016+)
   # Replace VERSION with the actual version (e.g., v1.0.0)
   VERSION="v1.0.0"  # Check https://github.com/npequeux/rutree2/releases for the latest version
   
   wget -O rutree2-android-arm64.tar.gz \
     "https://github.com/npequeux/rutree2/releases/download/${VERSION}/rutree2-android-arm64.tar.gz" || \
     { echo "Error: Download failed for version ${VERSION}."; \
       echo "Please check https://github.com/npequeux/rutree2/releases for available versions"; \
       echo "If no versioned release exists, use Option B to build from source"; exit 1; }
   tar -xzf rutree2-android-arm64.tar.gz
   
   # OR for ARMv7 devices (older phones)
   wget -O rutree2-android-armv7.tar.gz \
     "https://github.com/npequeux/rutree2/releases/download/${VERSION}/rutree2-android-armv7.tar.gz" || \
     { echo "Error: Download failed for version ${VERSION}."; \
       echo "Please check https://github.com/npequeux/rutree2/releases for available versions"; \
       echo "If no versioned release exists, use Option B to build from source"; exit 1; }
   tar -xzf rutree2-android-armv7.tar.gz
   ```

2. **Make it executable and run**:
   ```bash
   chmod +x rutree2
   ./rutree2
   ```

3. **That's it!** You can now use rutree2:
   ```bash
   # Show current directory
   ./rutree2
   
   # Show with hidden files
   ./rutree2 --all
   
   # Browse your downloads
   ./rutree2 /sdcard/Download
   ```

## Option B: Build from Source

If you want to build from source or if pre-built binaries aren't available yet:

1. **Install prerequisites on your computer**:
   ```bash
   # Install cargo-ndk
   cargo install cargo-ndk
   
   # Install Android targets
   rustup target add aarch64-linux-android
   ```

2. **Build the binary** (on your computer):
   ```bash
   # Clone the repository
   git clone https://github.com/npequeux/rutree2.git
   cd rutree2
   
   # Build for Android ARM64 (most modern phones)
   cargo ndk -t arm64-v8a build --release
   ```

3. **Transfer to your Android device** - See Option C or D below for transfer methods.

## Option C: Using Termux (After Building or Downloading)

**Prerequisites:**
- Connect your phone to your computer via USB
- Enable USB debugging on your phone:
  - Go to Settings → About Phone
  - Tap "Build Number" 7 times to enable Developer Options
  - Go to Settings → Developer Options
  - Enable "USB Debugging"

1. **Install Termux** on your Android device:
   - Download from [F-Droid](https://f-droid.org/packages/com.termux/) (recommended)
   - Or get it from Google Play Store

2. **Transfer and use**:
   ```bash
   # On your computer: transfer the binary
   adb push target/aarch64-linux-android/release/rutree2 /sdcard/
   
   # In Termux on your phone
   cp /sdcard/rutree2 ~/
   chmod +x ~/rutree2
   
   # Test it
   ./rutree2
   ```

3. **That's it!** You can now use rutree2:
   ```bash
   # Show current directory
   ./rutree2
   
   # Show with hidden files
   ./rutree2 --all
   
   # Browse your downloads
   ./rutree2 /sdcard/Download
   ```

## Option D: Direct Testing via ADB (No Termux needed)

1. **Transfer and run via ADB shell**:
   ```bash
   # On your computer
   adb push target/aarch64-linux-android/release/rutree2 /data/local/tmp/
   adb shell "cd /data/local/tmp && chmod +x rutree2 && ./rutree2 /sdcard"
   ```

## Finding Your Device Architecture

If you're not sure which binary to use, in Termux or ADB shell run:
```bash
uname -m
```

- `aarch64` → Use `target/aarch64-linux-android/release/rutree2`
- `armv7l` or `armv8l` → Use `target/armv7-linux-androideabi/release/rutree2`

Most modern Android phones (2016+) use `aarch64`.

## Usage Examples

Once installed, try these commands:

```bash
# Basic usage
./rutree2

# Show all files including hidden
./rutree2 -a

# Limit depth to 2 levels
./rutree2 -d 2

# Browse a specific folder
./rutree2 /sdcard/DCIM
./rutree2 /sdcard/Download

# Combine options
./rutree2 -a -d 3 /sdcard/Documents
```

## Troubleshooting

**"Download failed" or 404 error:**
- **Cause:** No versioned release has been created yet, or the version number is incorrect
- **Solution:** 
  1. Visit https://github.com/npequeux/rutree2/releases to see all available releases
  2. Look for a release with a version tag (e.g., v1.0.0, v0.1.0)
  3. Update the VERSION variable in the download command to match an existing release
  4. If no versioned releases exist, use Option B to build from source instead

**"Not: command not found" or extraction errors:**
- This occurs if you downloaded an HTML error page instead of the actual binary
- **Solution:** 
  1. Verify download before extracting: run `file rutree2-android-arm64.tar.gz`
  2. It should show "gzip compressed data", not "HTML document"
  3. If you see "HTML document", the download failed - check the release version and try again

**"Permission denied" error:**
- Run: `chmod +x rutree2`

**Cannot access /sdcard in Termux:**
- Run: `termux-setup-storage` and grant permissions

**Binary doesn't work:**
- Check your architecture: `uname -m`
- Make sure you're using the correct binary for your device

## Need Help?

See the full [ANDROID.md](ANDROID.md) documentation for more details and advanced usage.

## Alternative: Installing as an APK (Advanced)

For users who want a more integrated Android app experience, you can create a simple APK wrapper using Termux:Widget or similar tools. However, since `rutree2` is a command-line tool, the recommended installation methods are Options A-D above.

If you're interested in creating a full Android app wrapper with a GUI, you would need to:
1. Create an Android app project that embeds the native binary
2. Add a terminal emulator or file browser UI
3. Package it as an APK using Android Studio

This is beyond the scope of this quick start guide, but the terminal-based installation (Option A with Termux) provides the most straightforward way to use rutree2 on Android devices.
