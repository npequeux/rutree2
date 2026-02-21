# Release v2.7.0 Instructions

This document provides instructions for completing the v2.7.0 release of rutree2.

## Summary

The codebase has been prepared for release v2.7.0 with the following changes:
- Version bumped from 2.6.0 to 2.7.0 in Cargo.toml
- CHANGELOG.md updated with release notes for v2.7.0
- Cargo.lock updated to reflect new version
- All tests verified passing (14/14)
- Build verified successful

## To Complete the Release

Once this PR is merged to the main branch, you need to create and push the release tag. There are two options:

### Option 1: Using the create-release.sh Script (Recommended)

After merging this PR to main:

```bash
git checkout main
git pull origin main
./create-release.sh 2.7.0
```

This script will:
1. Verify you're on the main branch
2. Check that the version in Cargo.toml matches (2.7.0)
3. Create an annotated tag v2.7.0
4. Push the tag to GitHub

### Option 2: Manual Tag Creation

After merging this PR to main:

```bash
git checkout main
git pull origin main

# Create the annotated tag
git tag -a v2.7.0 -m "Release v2.7.0

Version bump for new release generation.

This release includes:
- Updated version from 2.6.0 to 2.7.0
- CHANGELOG.md updated with release notes
- All tests passing (14/14)
- Build verified successful"

# Push the tag to trigger the release workflow
git push origin v2.7.0
```

## What Happens Next

When the v2.7.0 tag is pushed, the GitHub Actions release workflow (`.github/workflows/release.yml`) will automatically:

1. ✨ Create a GitHub release at: https://github.com/npequeux/rutree2/releases/tag/v2.7.0
2. 🔨 Build binaries for all platforms:
   - Windows (x86_64)
   - Linux (x86_64, static musl build)
   - macOS Intel (x86_64)
   - macOS Apple Silicon (ARM64)
   - Android (ARM64, ARMv7)
3. 📦 Upload all binaries as release assets
4. ✅ Make the release available at: https://github.com/npequeux/rutree2/releases/latest

## Verification

After the workflow completes (check https://github.com/npequeux/rutree2/actions):

1. Visit https://github.com/npequeux/rutree2/releases/tag/v2.7.0
2. Verify the release exists and all binary assets are attached
3. Test downloading a binary to ensure the artifacts are accessible

## Notes

- The automated workflow typically takes 5-10 minutes to complete
- You can monitor progress in the Actions tab: https://github.com/npequeux/rutree2/actions
- The release will be marked as the "latest" release automatically
