# rutree2

A Rust command-line tool inspired by `lstree` for displaying directory structures in a tree format.

**Now with Android support!** 📱 See [ANDROID.md](ANDROID.md) for build and installation instructions.

## Features

- Display directory trees with proper indentation and tree characters
- Show hidden files with the `-a` or `--all` flag
- Limit traversal depth with the `-d` or `--depth` option
- Sort entries alphabetically
- Clean, readable output with visual tree structure

## Installation

### Pre-built Binaries

Download pre-built binaries from the [Releases page](https://github.com/npequeux/rutree2/releases):

**Desktop:**
- Windows (x86_64)
- Linux (x86_64)
- macOS (x86_64 and ARM64)

**Android:**
- ARM64 (aarch64) - Most modern devices
- ARMv7 - Older 32-bit ARM devices
- x86_64 - 64-bit x86 emulators/devices
- x86 - 32-bit x86 emulators/devices

For Android usage instructions, see [ANDROID.md](ANDROID.md).

### Build from source:
```bash
cargo build --release
```

The binary will be available at `target/release/rutree2`.

### Build Tasks

This project uses [cargo-make](https://github.com/sagiegurari/cargo-make) for task automation. First, install cargo-make:

```bash
cargo install cargo-make
```

Available tasks:
- `cargo make format` - Format code using rustfmt
- `cargo make check` - Check code without building
- `cargo make clippy` - Run clippy linter
- `cargo make build` - Build the project (runs format first)
- `cargo make build-release` - Build in release mode (runs format first)
- `cargo make test` - Run tests (runs format first)
- `cargo make clean` - Clean build artifacts
- `cargo make all` - Run all tasks: format, check, clippy, build, and test

**Android Build Tasks:**
- `cargo make install-android-targets` - Install Android target architectures
- `cargo make build-android` - Build for all Android architectures
- `cargo make build-android-arm64` - Build for ARM64 Android devices (most common)

See [ANDROID.md](ANDROID.md) for detailed Android build and installation instructions.

## Usage

```bash
# Display the current directory
rutree2

# Display a specific directory
rutree2 /path/to/directory

# Show hidden files
rutree2 --all

# Limit depth to 2 levels
rutree2 --depth 2

# Combine options
rutree2 --all --depth 1 /path/to/directory
```

## Options

- `PATH` - Path to display (defaults to current directory)
- `-a, --all` - Show hidden files
- `-d, --depth <DEPTH>` - Maximum depth to traverse
- `-h, --help` - Print help information

## Example Output

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── src/
│   └── main.rs
└── target/
    └── debug/
```

## License

MIT License - see LICENSE file for details