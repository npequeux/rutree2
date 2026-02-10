# Quick Start: Testing rutree2 on Android

This guide will get you started testing `rutree2` on your Android phone in just a few minutes.

## Option A: Using Termux (Easiest - No Computer Required!)

1. **Install Termux** on your Android device:
   - Download from [F-Droid](https://f-droid.org/packages/com.termux/) (recommended)
   - Or get it from Google Play Store

2. **Download the pre-built binary**:
   
   In Termux, run:
   ```bash
   # Download the ARM64 binary (works on most modern phones)
   curl -L -o rutree2 https://github.com/npequeux/rutree2/releases/download/latest/rutree2-android-arm64
   
   # Make it executable
   chmod +x rutree2
   
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

## Option B: Transfer from Computer (If you built it yourself)

1. **Connect your phone** to your computer via USB

2. **Enable USB debugging** on your phone:
   - Go to Settings → About Phone
   - Tap "Build Number" 7 times to enable Developer Options
   - Go to Settings → Developer Options
   - Enable "USB Debugging"

3. **Transfer the binary**:
   ```bash
   # On your computer
   adb push target/aarch64-linux-android/release/rutree2 /sdcard/
   ```

4. **Use it in Termux**:
   ```bash
   # In Termux on your phone
   cp /sdcard/rutree2 ~/
   chmod +x ~/rutree2
   ./rutree2
   ```

## Option C: Direct Testing via ADB (No Termux needed)

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

**"Permission denied" error:**
- Run: `chmod +x rutree2`

**Cannot access /sdcard in Termux:**
- Run: `termux-setup-storage` and grant permissions

**Binary doesn't work:**
- Check your architecture: `uname -m`
- Make sure you're using the correct binary for your device

## Need Help?

See the full [ANDROID.md](ANDROID.md) documentation for more details and advanced usage.
