# Testing Guide for rutree2

This guide covers both unit testing and Android testing for rutree2.

## Unit Tests

### Running Tests

Run the complete test suite:
```bash
cargo test
```

Run tests with verbose output:
```bash
cargo test -- --nocapture
```

Run a specific test:
```bash
cargo test test_cli_default_values
```

### Test Coverage

The project includes 14 comprehensive unit tests covering:

1. **CLI Parsing Tests** (5 tests)
   - Default values
   - All flag (`--all`)
   - Depth option (`--depth`)
   - Color option (`--color`)
   - Path argument

2. **Colorization Tests** (4 tests)
   - Archive files (`.zip`, `.tar`, etc.)
   - Image files (`.png`, `.jpg`, etc.)
   - Video files (`.mp4`, `.mkv`, etc.)
   - Executable files (Unix permissions)
   - Directory coloring

3. **Tree Display Tests** (5 tests)
   - File handling
   - Depth limiting
   - Hidden files (show/hide)
   - Symbolic links
   - Directory traversal

### Writing New Tests

Tests are located in `src/main.rs` in the `#[cfg(test)]` module. To add a new test:

```rust
#[test]
fn test_your_feature() {
    // Test setup
    let temp_dir = std::env::temp_dir().join("rutree2_test_feature");
    fs::create_dir_all(&temp_dir).unwrap();

    // Test execution
    let result = your_function(&temp_dir);

    // Assertions
    assert!(result.is_ok());

    // Cleanup
    fs::remove_dir_all(&temp_dir).unwrap();
}
```

### Linting and Code Quality

Run clippy for code quality checks:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Check code formatting:
```bash
cargo fmt -- --check
```

Format code automatically:
```bash
cargo fmt
```

---

## ✅ Android Version Ready for Testing!

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
