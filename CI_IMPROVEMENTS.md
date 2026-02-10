# CI/CD and Documentation Improvements

This document describes the CI/CD and documentation improvements added to the rutree2 project.

## Overview

The following enhancements have been made:
- Automated documentation publishing to GitHub Pages
- Enhanced CI/CD workflows with better organization and security checks
- Automated dependency management with Dependabot
- New build tasks for documentation and linting

## Documentation Publishing

### GitHub Pages Workflow (.github/workflows/docs.yml)

A new workflow has been added that automatically builds and publishes the Rust API documentation to GitHub Pages.

**Triggers:**
- Push to main branch
- Version tags (v*.*.*)
- Manual workflow dispatch

**Features:**
- Builds documentation using `cargo doc`
- Creates an index redirect for easy access
- Deploys to GitHub Pages with proper permissions
- Uses caching for faster builds

**Access:** Once enabled, documentation will be available at: https://npequeux.github.io/rutree2/

## Enhanced CI Workflow

The CI workflow (.github/workflows/ci.yml) has been reorganized into separate jobs for better organization and parallel execution:

### Lint Job
- Runs code formatting checks (`cargo fmt --check`)
- Runs Clippy with all targets and features enabled
- Fast feedback on code quality issues

### Security Job
- Runs `cargo audit` to check for known security vulnerabilities in dependencies
- Helps maintain a secure codebase

### Test Job
- Runs on multiple platforms (Ubuntu, macOS, Windows)
- Executes all tests
- Builds release binaries

**Benefits:**
- Parallel execution for faster feedback
- Clear separation of concerns
- Better visibility into which checks pass or fail

## Dependabot Configuration

Automated dependency updates have been configured via `.github/dependabot.yml`:

**Updates:**
- Cargo dependencies (weekly)
- GitHub Actions versions (weekly)

**Features:**
- Automatic pull requests for dependency updates
- Proper labeling of dependency PRs
- Helps keep dependencies secure and up-to-date

## New Makefile Tasks

The following tasks have been added to `Makefile.toml`:

### Documentation Tasks
- `cargo make doc` - Generate documentation
- `cargo make doc-open` - Generate and open documentation in browser

### Linting Tasks
- `cargo make lint` - Run all linting tasks (format + clippy)

### Security Tasks
- `cargo make audit` - Run security audit (requires cargo-audit installation)

## Setup Instructions

### For Documentation Publishing

To enable GitHub Pages for documentation:

1. Go to your repository settings
2. Navigate to "Pages" under "Code and automation"
3. Under "Build and deployment", select:
   - Source: "GitHub Actions"
4. The documentation will be published automatically on the next push to main or tag

### For Security Audits

To run security audits locally, install cargo-audit:

```bash
cargo install cargo-audit
```

Then run:
```bash
cargo make audit
# or
cargo audit
```

### For Dependabot

Dependabot is automatically enabled for all GitHub repositories. No additional setup is required. It will start creating pull requests for dependency updates according to the schedule defined in `.github/dependabot.yml`.

## Badges in README

The README now includes badges for:
- CI status
- Documentation build status
- License

These provide quick visibility into the project's health and status.

## Benefits

1. **Better Code Quality**: Automated linting and formatting checks ensure consistent code style
2. **Enhanced Security**: Regular dependency audits and updates help identify and fix vulnerabilities
3. **Better Documentation**: Automated API documentation publishing makes it easier for users and contributors
4. **Faster Development**: Parallel CI jobs provide faster feedback
5. **Easier Maintenance**: Dependabot reduces manual effort in keeping dependencies updated

## Workflow Diagram

```
Push to Main/Tag
    │
    ├─→ CI Workflow
    │   ├─→ Lint Job (formatting, clippy)
    │   ├─→ Security Job (cargo audit)
    │   └─→ Test Job (multi-platform tests & builds)
    │
    └─→ Documentation Workflow
        ├─→ Build Docs (cargo doc)
        └─→ Deploy to GitHub Pages
```

## Maintenance

### Updating Dependencies

Dependabot will automatically create PRs for dependency updates. Review and merge these PRs regularly to keep dependencies up-to-date.

### Monitoring Security

The security job in CI will fail if vulnerabilities are detected. Address these promptly by:
1. Checking the audit output
2. Updating affected dependencies
3. If no fix is available, assess the risk and document the decision

### Documentation Updates

Documentation is automatically rebuilt and published when:
- Code is pushed to main
- A new version tag is created
- The workflow is manually triggered

No manual steps are required for documentation publishing.
