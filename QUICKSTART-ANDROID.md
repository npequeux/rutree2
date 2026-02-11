# Quick Start: Testing rutree2 on Android

This guide will get you started testing `rutree2` on your Android phone in just a few minutes.

## Option A: Download Pre-built Binary (Easiest)

**Note:** Android binaries are now included in releases starting from the next release.

1. **Download the binary** for your device:
   
   In Termux or via browser, download:
   ```bash
   # For ARM64 devices (most modern phones from 2016+)
   curl -L -o rutree2 https://github.com/npequeux/rutree2/releases/latest/download/rutree2-android-arm64.tar.gz
   tar -xzf rutree2-android-arm64.tar.gz
   
   # OR for ARMv7 devices (older phones)
   curl -L -o rutree2 https://github.com/npequeux/rutree2/releases/latest/download/rutree2-android-armv7.tar.gz
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

**"Not: command not found" error:**
- This occurs if you downloaded an error page instead of a binary (usually from an invalid URL)
- Starting from the next release, use the direct download links in Option A
- Alternatively, build from source using Option B

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
