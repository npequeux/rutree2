# Release Instructions for v1.0.0

This document contains the steps to complete the v1.0.0 release of rutree2.

## Prerequisites

Before creating a release, ensure:
- ✅ Code has been tested and builds successfully
- ✅ CHANGELOG.md has been updated with release notes
- ✅ Version in Cargo.toml matches the release version (e.g., 1.0.0)

## Steps to Complete the Release

### Option 1: Push the Tag (Recommended - Automated)

The repository has an automated release workflow that will trigger when you push a tag:

```bash
# Create an annotated tag for the release
git tag -a v1.0.0 -m "Release v1.0.0"

# Push the tag to GitHub
git push origin v1.0.0
```

This will automatically:
1. Create a GitHub release
2. Build binaries for all supported platforms:
   - Windows (x86_64)
   - Linux (x86_64, static musl build)
   - macOS (x86_64 Intel and ARM64 Apple Silicon)
   - Android (ARM64, ARMv7)
3. Validate each archive to ensure integrity:
   - Verify the archive file exists and is not empty
   - Test that the archive is a valid gzip (or zip for Windows)
   - Verify the archive can be extracted successfully
   - Confirm expected files (binary, README.md) are present
4. Upload the binaries as release assets

### Option 2: Manual Release via GitHub UI

If you prefer to create the release manually:

1. Go to https://github.com/npequeux/rutree2/releases/new
2. Select or create tag: `v1.0.0`
3. Release title: `Release v1.0.0`
4. Description: Copy the content from the "Release Notes" section below
5. Click "Publish release"

Note: If using this option, the automated build workflow will still run and attach binaries to the release.

## Release Notes

Use the following for the GitHub release description:

```markdown
# rutree2 v1.0.0 - Stable Release

This is the first stable release of rutree2, a Rust command-line tool for displaying directory structures in a tree format.

## Features

- 🌳 Display directory trees with proper indentation and tree characters
- 👁️ Show hidden files with the `-a` or `--all` flag
- 📏 Limit traversal depth with the `-d` or `--depth` option
- 🔤 Sort entries alphabetically
- ✨ Clean, readable output with visual tree structure
- 🌍 Cross-platform support (Windows, Linux, macOS)
- 📱 Android support (ARM64, ARMv7, x86_64, x86)

## Installation

### Pre-built Binaries

Download the appropriate binary for your platform:

- **Windows (x86_64)**: `rutree2-windows-x86_64.zip`
- **Linux (x86_64)**: `rutree2-linux-x86_64.tar.gz`
- **macOS (Intel)**: `rutree2-macos-x86_64.tar.gz`
- **macOS (Apple Silicon)**: `rutree2-macos-arm64.tar.gz`

Extract the archive and add the binary to your PATH.

### Build from Source

```bash
cargo build --release
```

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

## Full Changelog

See [CHANGELOG.md](https://github.com/npequeux/rutree2/blob/v1.0.0/CHANGELOG.md) for complete details.
```

## Verification

After the release is published, verify:

1. The release appears at: https://github.com/npequeux/rutree2/releases/tag/v1.0.0
2. All platform binaries are attached as assets
3. The README.md links to the release work correctly

## Next Steps

After the release is published:
- Share the release announcement
- Update any documentation that references version numbers
- Plan for the next release (v1.1.0)
