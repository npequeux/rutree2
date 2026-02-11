# Automatic Release Process

This repository has been configured to automatically generate a new release after each commit to the main branch.

## How It Works

### Automatic Version Bumping on Each Commit

When a commit is pushed to the main branch, a GitHub Actions workflow automatically:

1. **Determines the version bump type** based on commit message:
   - **Major** (X.0.0): Commits with `[major]` in the commit message
   - **Minor** (0.X.0): Default for all commits (unless specified otherwise)
   - **Patch** (0.0.X): Commits with `[patch]` in the commit message
   
2. **Updates project files**:
   - Increments the version in `Cargo.toml`
   - Adds a new entry to `CHANGELOG.md` with the commit details
   
3. **Creates and pushes a git tag** in the format `vX.Y.Z`

4. **Triggers the release workflow** which:
   - Creates a GitHub release
   - Builds binaries for all supported platforms
   - Uploads binaries as release assets

## Version Bump Strategy

The workflow follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version when you make incompatible API changes (breaking changes)
- **MINOR** version when you add functionality in a backward compatible manner (default)
- **PATCH** version when you make backward compatible bug fixes

### Controlling Version Bumps

To control which version component is bumped, add a marker to your commit message:

- `[major]` → Major version bump (1.0.0 → 2.0.0)
- `[patch]` → Patch version bump (1.0.0 → 1.0.1)
- No marker → Minor version bump (1.0.0 → 1.1.0) - **This is the default**

### Skipping Automatic Releases

To prevent a commit from triggering an automatic release, include `[skip-release]` in your commit message.

Examples:
```bash
git commit -m "Update documentation [skip-release]"
git commit -m "Fix typo in README [skip-release]"
```

Note: Commits to documentation files (*.md), docs/ directory, .github/, and LICENSE are automatically excluded from triggering releases.

## Alternative: Issue-Based Releases

The repository also supports issue-based releases (legacy method). When an issue is closed as "completed", the workflow can still create a release based on issue labels:

- `breaking` label → Major version bump
- `feature` or `enhancement` label → Minor version bump
- `bug` or `fix` label → Patch version bump

This method is maintained for backward compatibility but the commit-based method is now the primary release mechanism.

## Manual Releases

You can also trigger a release manually without a commit or issue:

1. Go to the [Actions tab](https://github.com/npequeux/rutree2/actions/workflows/auto-release.yml)
2. Click "Run workflow"
3. Enter the issue number and select the version bump type
4. Click "Run workflow"

## Workflow Files

- **Commit-based releases**: `.github/workflows/auto-version-commit.yml` (primary)
- **Issue-based releases**: `.github/workflows/auto-release.yml` (legacy)

## Examples

### Example 1: Regular Commit (Minor Bump)

If you commit a change with message "Add new feature for filtering":

1. The workflow detects the commit
2. Version is bumped from 1.0.0 → 1.1.0 (minor bump - default)
3. `Cargo.toml` is updated to version 1.1.0
4. `CHANGELOG.md` gets a new entry:
   ```markdown
   ## [1.1.0] - 2026-02-11
   
   ### Changed
   - Add new feature for filtering ([abc1234](https://github.com/...))
   ```
5. Tag `v1.1.0` is created and pushed
6. The release workflow builds and publishes binaries

### Example 2: Major Breaking Change

If you commit a change with message "Redesign API [major]":

1. Version is bumped from 1.1.0 → 2.0.0 (major bump)
2. All files updated accordingly
3. Tag `v2.0.0` is created

### Example 3: Patch Fix

If you commit a change with message "Fix bug in tree traversal [patch]":

1. Version is bumped from 2.0.0 → 2.0.1 (patch bump)
2. All files updated accordingly
3. Tag `v2.0.1` is created

### Example 4: Skip Release

If you commit a change with message "Update README [skip-release]":

1. No version bump occurs
2. No release is created
3. Changes are just committed normally

## Disabling Automatic Releases

If you need to disable automatic releases temporarily:
- Add `[skip-release]` to your commit messages
- Or disable the workflow in repository settings

To permanently disable:
- Remove or disable the workflow file `.github/workflows/auto-version-commit.yml`

## Troubleshooting

### The workflow didn't trigger
- Make sure you pushed to the main branch
- Check that your commit doesn't match the excluded paths (*.md, docs/, .github/, LICENSE)
- Check that your commit message doesn't contain `[skip-release]`
- Verify the commit message doesn't start with "Release v" (auto-generated release commits)
- Check that the workflow file exists and is valid
- Verify workflow permissions in repository settings

### Version bump was incorrect
- Check your commit message for `[major]` or `[patch]` markers
- Remember: default is minor bump
- You can manually adjust the version by creating another commit with the appropriate marker

### Build failures
- Check the release workflow logs in the Actions tab
- The release will be created but binaries might be missing if builds fail
- Fix the build issues and manually re-run the workflow or push a new commit with `[patch]`
