# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.7.0] - 2026-02-21

### Changed
- Version bump for new release generation

## [2.6.0] - 2026-02-18

### Changed
- version ([5a44ae1](https://github.com/npequeux/rutree2/commit/5a44ae1d8d32abae504119c05b08047b70ae7ec5))

## [2.5.0] - 2026-02-18

### Changed
- feat: add code coverage with optimized CI/CD ([0eb1996](https://github.com/npequeux/rutree2/commit/0eb19965623ea9554c76671ca42d41a949a0a3e2))

## [2.4.0] - 2026-02-18

### Changed
- chore: update Cargo.lock ([10f824e](https://github.com/npequeux/rutree2/commit/10f824e58a1e82f5e14f46f1a4a8e3d7ae15fc62))

## [2.3.0] - 2026-02-18

### Changed
- small fix ([4b062b7](https://github.com/npequeux/rutree2/commit/4b062b7a59b9254215c40a97faef0a660e5c738f))

## [2.2.0] - 2026-02-18

### Changed
- chore: update Cargo.lock ([1644f2e](https://github.com/npequeux/rutree2/commit/1644f2e3e2477c3cb46b48f4f1d9f9678768edeb))

## [2.1.0] - 2026-02-18

### Changed
- fix: Add #[allow(dead_code)] to ColoredCompat trait ([fed1e13](https://github.com/npequeux/rutree2/commit/fed1e13f92683b00ce2d97526349f884bb5653d3))

## [2.0.0] - 2026-02-18

### Added
- **Interactive Mode**: New `-i` / `--interactive` flag for collapsible/expandable tree navigation
  - Navigate with arrow keys (↑/↓/←/→)
  - Expand/collapse directories with Enter or right arrow
  - Collapse with left arrow
  - Visual selection highlighting with yellow background
  - On-screen legend overlay showing all keyboard shortcuts
  - Seamless integration with existing options (--all, --depth, etc.)
- Enhanced tree visualization with `[+]` and `[-]` indicators for collapsed/expanded state

### Changed
- Updated to use `ratatui` 0.30.0 for modern TUI rendering
- Fixed deprecated API usage (changed `size()` to `area()` in ratatui)
- Improved tree flattening algorithm to properly track node paths for interactive mutation
- Major version bump to 2.0.0 to reflect significant new interactive feature

### Technical
- Added `crossterm` for terminal control in interactive mode
- Implemented efficient path-based node tracking for expand/collapse operations
- Enhanced TreeNode structure with expansion state and proper reference handling

## [1.7.0] - 2026-02-18

### Changed
- Fixing navigation partly ([1a1c34b](https://github.com/npequeux/rutree2/commit/1a1c34b0a0068f645a982bdbef2645e7232a1b32))

## [1.6.0] - 2026-02-18

### Changed
- Fixing build issue ([3c1bc5b](https://github.com/npequeux/rutree2/commit/3c1bc5bedd74dc8442b7b876904f2463cd163868))

## [1.5.0] - 2026-02-17

### Changed
- Fix: repair block structure, delimiters, and interactive tree constants. All tests passing. ([20a9027](https://github.com/npequeux/rutree2/commit/20a90278899bf60b906baf0252fa3fce4470ea7e))

## [1.4.0] - 2026-02-17

### Changed
- Merge pull request #93 from npequeux/copilot/test-fix-all-actions ([89e6225](https://github.com/npequeux/rutree2/commit/89e62250c085ff45ec50c5424d984d3fb16feff7))

## [1.3.0] - 2026-02-17

### Changed
- Merge pull request #89 from npequeux:copilot/full-code-review-and-improvements ([4ee9f72](https://github.com/npequeux/rutree2/commit/4ee9f72df235b8ef5fbba6b10b0e43a41ab67695))

## [1.2.0] - 2026-02-17

### Changed
- Add comprehensive documentation for code review and testing ([88881a5](https://github.com/npequeux/rutree2/commit/88881a5ddc2f22b2ca595d3cf33c278aa6e3ad28))

## [1.1.2] - 2026-02-17

### Changed
- Resolved issue #88: Make a full code review and improve code . fix all actions issue

## [1.1.1] - 2026-02-17

### Changed
- Resolved issue #84: Make a full code review and improve code

## [1.1.0] - 2026-02-11

### Changed
- Update Cargo.lock to match version 1.0.1 in Cargo.toml ([84aa927](https://github.com/npequeux/rutree2/commit/84aa92786ca2f9134f2d30c42149a6f0b7f238e8))

## [1.0.1] - 2026-02-11

### Changed
- Resolved issue #58: Commits and push action fail

## [1.0.0] - 2026-02-11

### Changed
- Version bump to 1.0.0 for stable release
- Project is now considered stable and production-ready

## [0.1.0] - 2026-02-10

### Added
- Initial release of rutree2
- Display directory structures in a tree format
- Show hidden files with `-a` or `--all` flag
- Limit traversal depth with `-d` or `--depth` option
- Sort entries alphabetically
- Clean, readable output with visual tree structure
- Cross-platform support (Windows, Linux, macOS)
- Android support for ARM64, ARMv7, x86_64, and x86 architectures
- Automated CI/CD pipeline for building and testing
- Pre-built binaries for multiple platforms

[1.0.0]: https://github.com/npequeux/rutree2/releases/tag/v1.0.0
[0.1.0]: https://github.com/npequeux/rutree2/releases/tag/v0.1.0
