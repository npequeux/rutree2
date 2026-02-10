# ✅ Android Version Ready for Testing!

The Android version of `rutree2` has been successfully built and is ready to test on your phone!

## 📱 Three Easy Ways to Test on Your Phone

### Option 1: Using Termux (Easiest!)

1. **Install Termux** from [F-Droid](https://f-droid.org/packages/com.termux/) or Google Play Store
2. **Transfer the binary** from your computer to your phone:
   ```bash
   adb push target/aarch64-linux-android/release/rutree2 /sdcard/
   ```
3. **In Termux on your phone**, run:
   ```bash
   cp /sdcard/rutree2 ~/
   chmod +x ~/rutree2
   ./rutree2
   ```

### Option 2: Using ADB Shell

```bash
# On your computer, transfer and run:
adb push target/aarch64-linux-android/release/rutree2 /data/local/tmp/
adb shell "cd /data/local/tmp && chmod +x rutree2 && ./rutree2 /sdcard"
```

### Option 3: Build It Yourself Later

The build system is now set up! You can rebuild anytime:
```bash
./build-android.sh
# or
cargo make build-android
```

## 📦 What's Available

Four Android binaries have been built at:
- `target/aarch64-linux-android/release/rutree2` (ARM64 - **for most modern phones**)
- `target/armv7-linux-androideabi/release/rutree2` (ARMv7 - older phones)
- `target/i686-linux-android/release/rutree2` (x86 - emulators)
- `target/x86_64-linux-android/release/rutree2` (x86_64 - emulators)

**Most phones use ARM64**, so use `target/aarch64-linux-android/release/rutree2`.

## 📖 Full Documentation

- **QUICKSTART-ANDROID.md** - Quick setup guide
- **ANDROID.md** - Complete documentation with all installation methods
- **build-android.sh** - Automated build script

## ✨ Example Commands to Try on Your Phone

```bash
# Show current directory tree
./rutree2

# Show hidden files
./rutree2 --all

# Browse your downloads folder
./rutree2 /sdcard/Download

# Browse photos with depth limit
./rutree2 --depth 2 /sdcard/DCIM
```

Enjoy your Android version of rutree2! 🎉
