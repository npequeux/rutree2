# Release Checklist for Maintainers

This document explains how to create a proper release to fix the "404 download error" issue.

## The Problem

Users are getting 404 errors when trying to download pre-built binaries because:

1. GitHub's `/releases/latest` endpoint only works with **semantic version tags** (e.g., v1.0.0, v2.3.1)
2. The repository currently only has a "main" tag release, which is not recognized by this endpoint
3. Download URLs using `/releases/latest/download/...` will fail until a proper version tag is created

## The Solution

Create and push a semantic version tag (like v1.0.0) to trigger the automated release workflow.

## Steps to Create the First Release

### Prerequisites

Before creating a release, verify:

- ✅ Code builds successfully: `cargo build --release`
- ✅ All tests pass: `cargo test`
- ✅ Version in `Cargo.toml` is set correctly (currently: 1.0.0)
- ✅ `CHANGELOG.md` is updated with release notes

### Create and Push the Version Tag

The repository is already configured for version 1.0.0, so you can create the release tag:

```bash
# Ensure you're on the main branch
git checkout main
git pull origin main

# Create an annotated tag
git tag -a v1.0.0 -m "Release v1.0.0

First stable release of rutree2.

Features:
- Display directory structures in a tree format
- Support for multiple platforms (Linux, macOS, Windows, Android)
- Customizable depth limiting
- Hidden file display options
- Colorized output"

# Push the tag to GitHub (this triggers the release workflow)
git push origin v1.0.0
```

### What Happens Next

When you push the tag, the GitHub Actions release workflow will automatically:

1. ✨ Create a GitHub release at: https://github.com/npequeux/rutree2/releases/tag/v1.0.0
2. 🔨 Build binaries for all platforms:
   - Windows (x86_64)
   - Linux (x86_64)
   - macOS Intel (x86_64)
   - macOS Apple Silicon (ARM64)
   - Android (ARM64, ARMv7)
3. 📦 Upload all binaries as release assets
4. ✅ Make the release available at: https://github.com/npequeux/rutree2/releases/latest

### Verify the Release

After the workflow completes (check https://github.com/npequeux/rutree2/actions):

1. Visit https://github.com/npequeux/rutree2/releases/latest
2. Confirm it shows release v1.0.0
3. Verify all binary assets are attached:
   - rutree2-windows-x86_64.zip
   - rutree2-linux-x86_64.tar.gz
   - rutree2-macos-x86_64.tar.gz
   - rutree2-macos-arm64.tar.gz
   - rutree2-android-arm64.tar.gz
   - rutree2-android-armv7.tar.gz
4. Test a download URL:
   ```bash
   curl -I -L https://github.com/npequeux/rutree2/releases/latest/download/rutree2-linux-x86_64.tar.gz
   # Should return HTTP 200, not 404
   ```

### Alternative: Use the Helper Script

The repository includes a helper script:

```bash
# This script will create and push the tag with proper formatting
./create-release.sh 1.0.0
```

## For Future Releases

When creating subsequent releases (v1.1.0, v2.0.0, etc.):

1. Update the version in `Cargo.toml`
2. Update `CHANGELOG.md` with the new version's changes
3. Commit these changes
4. Create and push a new version tag following the steps above

## Troubleshooting

**"Tag already exists"**:
```bash
# Delete the local tag
git tag -d v1.0.0

# Delete the remote tag (if it was pushed)
git push origin :refs/tags/v1.0.0

# Recreate the tag
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

**"Workflow failed"**:
- Check the workflow logs at: https://github.com/npequeux/rutree2/actions
- Common issues:
  - Build failures: Check Rust compilation errors
  - Android NDK issues: Verify NDK setup in the workflow
  - Upload failures: Check GitHub token permissions

## Related Documentation

- [RELEASE_INSTRUCTIONS.md](RELEASE_INSTRUCTIONS.md) - Detailed release process
- [RELEASE_READY.md](RELEASE_READY.md) - Current release status
- [create-release.sh](create-release.sh) - Helper script for creating releases
