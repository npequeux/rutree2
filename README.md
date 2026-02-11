# rutree2

[![Rust CI](https://github.com/npequeux/rutree2/workflows/Rust%20CI/badge.svg)](https://github.com/npequeux/rutree2/actions/workflows/ci.yml)
[![Documentation](https://github.com/npequeux/rutree2/workflows/Documentation/badge.svg)](https://github.com/npequeux/rutree2/actions/workflows/docs.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust command-line tool inspired by `lstree` for displaying directory structures in a tree format.

**Now with Android support!** 📱 See [ANDROID.md](ANDROID.md) for build and installation instructions.

## Documentation

- [API Documentation](https://npequeux.github.io/rutree2/) - Rust API documentation (auto-generated)
- [Android Build Guide](ANDROID.md) - Detailed Android build and installation instructions
- [Quick Start (Android)](QUICKSTART-ANDROID.md) - Fast-track Android setup
- [Testing Guide](TESTING.md) - Information about testing the project
- [CI/CD Improvements](CI_IMPROVEMENTS.md) - Details about documentation publishing, lint checks, and workflows

> **Note:** To enable documentation publishing, go to repository Settings → Pages → Source and select "GitHub Actions".

## Features

- Display directory trees with proper indentation and tree characters
- **Visualize symbolic links** with `->` indicator showing both origin and destination
- Show hidden files with the `-a` or `--all` flag
- Limit traversal depth with the `-d` or `--depth` option
- Sort entries alphabetically
- Color-coded output based on file types and permissions:
  - Directories: Blue (bold)
  - Executable files: Green
  - World-writable files: Yellow (warning)
  - Symbolic links: Cyan
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
- **Android** (ARM64): `rutree2-android-arm64.tar.gz`
- **Android** (ARMv7): `rutree2-android-armv7.tar.gz`

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

**Pre-built Android binaries are now available in releases!** Download them directly from the [latest release](https://github.com/npequeux/rutree2/releases/latest).

For detailed Android installation instructions, see [QUICKSTART-ANDROID.md](QUICKSTART-ANDROID.md) for the quickest setup, or [ANDROID.md](ANDROID.md) for comprehensive build and installation instructions.

Available Android binaries in releases:
- ARM64 (most modern devices): `rutree2-android-arm64.tar.gz`
- ARMv7 (older devices): `rutree2-android-armv7.tar.gz`

If you prefer to build from source, binaries are available at:
- ARM64: `target/aarch64-linux-android/release/rutree2`
- ARMv7: `target/armv7-linux-androideabi/release/rutree2`
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
- `cargo make lint` - Run all linting tasks (format + clippy)
- `cargo make build` - Build the project (runs format first)
- `cargo make build-release` - Build in release mode (runs format first)
- `cargo make test` - Run tests (runs format first)
- `cargo make doc` - Generate documentation
- `cargo make doc-open` - Generate and open documentation in browser
- `cargo make audit` - Run security audit (requires cargo-audit)
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
- `-C, --color <COLOR>` - Use colors to distinguish file types and permissions (auto, always, never) [default: auto]
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

### Symbolic Links

Symbolic links are displayed with a `->` indicator showing the target. Directory symlinks are marked with a trailing `/`. When traversing symlinks to directories, the tree displays their contents:

```
.
├── README.md
├── absolute_link -> /etc/hostname
├── broken_link -> missing_file
├── docs/
│   └── config/
│       └── settings.yaml
├── link_to_readme -> README.md
├── main.rs
└── shortcuts/ -> docs
    └── config/
        └── settings.yaml
```

In this example, `shortcuts/` is a symbolic link pointing to the `docs` directory, and the tree follows the link to show its contents.

## License

MIT License - see LICENSE file for details