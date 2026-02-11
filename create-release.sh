#!/bin/bash
# Script to create and push a release tag for rutree2
#
# Usage: ./create-release.sh <version>
# Example: ./create-release.sh 1.0.0

set -e

if [ -z "$1" ]; then
    echo "Error: Version number required"
    echo "Usage: ./create-release.sh <version>"
    echo "Example: ./create-release.sh 1.0.0"
    exit 1
fi

VERSION="$1"
TAG="v${VERSION}"

# Verify we're on the main/master branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ] && [ "$CURRENT_BRANCH" != "master" ]; then
    echo "Warning: You are on branch '$CURRENT_BRANCH', not 'main' or 'master'"
    echo "Press Ctrl+C to cancel, or Enter to continue..."
    read
fi

# Check if tag already exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "Error: Tag $TAG already exists"
    echo "To recreate the tag, first delete it with:"
    echo "  git tag -d $TAG"
    echo "  git push origin :refs/tags/$TAG"
    exit 1
fi

# Verify version in Cargo.toml matches
CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
if [ "$CARGO_VERSION" != "$VERSION" ]; then
    echo "Error: Version mismatch!"
    echo "  Cargo.toml version: $CARGO_VERSION"
    echo "  Requested version:  $VERSION"
    echo ""
    echo "Please update Cargo.toml to version $VERSION and commit the change first."
    exit 1
fi

echo "Creating release tag: $TAG"
echo "Version in Cargo.toml: $CARGO_VERSION ✓"
echo ""

# Create the annotated tag
git tag -a "$TAG" -m "Release $TAG

First stable release of rutree2.

Features:
- Display directory structures in a tree format
- Support for multiple platforms (Linux, macOS, Windows, Android)
- Customizable depth limiting
- Hidden file display options
- Colorized output

See CHANGELOG.md for full details."

echo "✓ Tag $TAG created locally"
echo ""
echo "Pushing tag to GitHub..."

# Push the tag
git push origin "$TAG"

echo ""
echo "✓ Tag pushed successfully!"
echo ""
echo "The release workflow will now:"
echo "  1. Create a GitHub release"
echo "  2. Build binaries for all platforms"
echo "  3. Upload binaries as release assets"
echo ""
echo "Monitor the workflow at:"
echo "  https://github.com/npequeux/rutree2/actions"
echo ""
echo "Once complete, the release will be available at:"
echo "  https://github.com/npequeux/rutree2/releases/tag/$TAG"
echo "  https://github.com/npequeux/rutree2/releases/latest"
