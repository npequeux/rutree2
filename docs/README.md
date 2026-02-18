# rutree2

A Rust command-line tool inspired by `lstree` for displaying directory structures in a tree format.

**Now with Android support!** 📱 See [ANDROID.md](ANDROID.md) for build and installation instructions.

## Features

- Display directory trees with proper indentation and tree characters
- **Interactive Mode**: Collapsible/expandable tree view with keyboard navigation
- **Visualize symbolic links** with `->` indicator showing both origin and destination
- Show hidden files with the `-a` or `--all` flag
- Limit traversal depth with the `-d` or `--depth` option
- Sort entries alphabetically
- **Enhanced color-coded output** based on file types and permissions
- Clean, readable output with visual tree structure

## Installation

### Download Pre-built Binaries

Pre-built binaries are available for multiple platforms:

- **[Latest Release](https://github.com/npequeux/rutree2/releases/latest)** - Download the latest stable release
- **[All Releases](https://github.com/npequeux/rutree2/releases)** - Browse all available releases

Available platforms:
- **Windows** (x86_64): `rutree2-windows-x86_64.zip`
- **Linux** (x86_64): `rutree2-linux-x86_64.tar.gz`
- **macOS** (x86_64 Intel): `rutree2-macos-x86_64.tar.gz`
- **macOS** (ARM64 Apple Silicon): `rutree2-macos-arm64.tar.gz`

### Build from Source

```bash
cargo build --release
```

The binary will be available at `target/release/rutree2`.

### CI Artifacts

Development builds are automatically created for every commit and pull request:

- **[CI Workflow Runs](https://github.com/npequeux/rutree2/actions/workflows/ci.yml)** - View test results and download artifacts from CI builds
- **[All Actions](https://github.com/npequeux/rutree2/actions)** - Browse all workflow runs

CI builds are created for:
- Ubuntu Linux
- macOS (Intel and ARM)
- Windows

### Android Builds

For Android devices, see [ANDROID.md](ANDROID.md) for detailed build and installation instructions.

After building for Android, binaries are available at:
- ARM64 (most modern devices): `target/aarch64-linux-android/release/rutree2`
- ARMv7 (older devices): `target/armv7-linux-androideabi/release/rutree2`
- x86_64 (emulators): `target/x86_64-linux-android/release/rutree2`
- x86 (older emulators): `target/i686-linux-android/release/rutree2`

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

# Interactive mode
rutree2 -i

# Combine options
rutree2 --all --depth 1 /path/to/directory
```

## Interactive Mode

Launch interactive mode with the `-i` flag for a collapsible/expandable tree view:

```bash
rutree2 -i
```

**Controls:**
- **↑/↓** - Navigate up/down
- **→/Enter** - Expand directory or toggle
- **←** - Collapse directory
- **q** - Quit

Features visual highlighting, on-screen legend, and works with all standard options.

## Options

- `PATH` - Path to display (defaults to current directory)
- `-a, --all` - Show hidden files
- `-d, --depth <DEPTH>` - Maximum depth to traverse
- `-i, --interactive` - Launch interactive collapsible/expandable tree mode
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