# Automatic Release Process

This repository has been configured to automatically generate a new release whenever an issue is closed and marked as completed.

## How It Works

When an issue is closed with the status "completed" (not just closed), a GitHub Actions workflow automatically:

1. **Determines the version bump type** based on issue labels:
   - **Major** (X.0.0): Issues labeled with `breaking`
   - **Minor** (0.X.0): Issues labeled with `feature` or `enhancement`
   - **Patch** (0.0.X): Issues labeled with `bug` or `fix`, or no specific label
   
2. **Updates project files**:
   - Increments the version in `Cargo.toml`
   - Adds a new entry to `CHANGELOG.md` with the issue details
   
3. **Creates and pushes a git tag** in the format `vX.Y.Z`

4. **Triggers the release workflow** which:
   - Creates a GitHub release
   - Builds binaries for all supported platforms
   - Uploads binaries as release assets

## Version Bump Strategy

The workflow follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version when you make incompatible API changes (breaking changes)
- **MINOR** version when you add functionality in a backward compatible manner
- **PATCH** version when you make backward compatible bug fixes

### Controlling Version Bumps

To control which version component is bumped, add one of these labels to your issue:

- `breaking` → Major version bump (1.0.0 → 2.0.0)
- `feature` or `enhancement` → Minor version bump (1.0.0 → 1.1.0)
- `bug` or `fix` → Patch version bump (1.0.0 → 1.0.1)

If no label is specified, a **patch** version bump is applied by default.

## Manual Releases

You can also trigger a release manually without closing an issue:

1. Go to the [Actions tab](https://github.com/npequeux/rutree2/actions/workflows/auto-release.yml)
2. Click "Run workflow"
3. Enter the issue number and select the version bump type
4. Click "Run workflow"

## Workflow File

The automatic release workflow is defined in `.github/workflows/auto-release.yml`.

## Example

If you close issue #42 titled "Fix memory leak in tree traversal" with the `bug` label:

1. The workflow detects the closed issue
2. Version is bumped from 1.0.0 → 1.0.1 (patch bump for bug fix)
3. `Cargo.toml` is updated to version 1.0.1
4. `CHANGELOG.md` gets a new entry:
   ```markdown
   ## [1.0.1] - 2026-02-11
   
   ### Changed
   - Resolved issue #42: Fix memory leak in tree traversal
   ```
5. Tag `v1.0.1` is created and pushed
6. A comment is added to the issue with a link to the release
7. The release workflow builds and publishes binaries

## Disabling Automatic Releases

If you need to close an issue without triggering a release, simply close it as "not planned" instead of "completed", or remove the workflow file.

## Troubleshooting

### The workflow didn't trigger
- Make sure the issue was closed as "completed" (not just closed or closed as "not planned")
- Check that the workflow file exists and is valid
- Verify workflow permissions in repository settings

### Version bump was incorrect
- Check the issue labels - they determine the bump type
- You can manually adjust the version by creating another issue or using manual workflow dispatch

### Build failures
- Check the release workflow logs in the Actions tab
- The release will be created but binaries might be missing if builds fail
- Fix the build issues and manually re-run the workflow or create a new patch release
