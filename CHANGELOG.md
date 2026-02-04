# Changelog

All notable changes to monogirl are documented here.

## [0.4.2] - 2026-04-05

### Changed
- Optimized CPE batch scheduling for large transaction sets
- Improved epoch boundary handling in proof verification

### Fixed
- Resolved edge case in conflict resolution with overlapping account sets
- Fixed parallel proof depth calculation for sparse dependency graphs

## [0.4.0] - 2026-03-15

### Added
- Epoch-aware proof generation with automatic validity windows
- Batch CPE bundle submission support in TypeScript SDK

### Changed
- Refactored state manager for reduced account size

## [0.3.0] - 2026-02-20

### Added
- CLI tool for bundle creation and proof verification
- Dependency graph partitioning for conflict detection

### Fixed
- Memory alignment issues in CPE state accounts

## [0.2.0] - 2026-02-01

### Added
- TypeScript SDK with full CPE bundle lifecycle management
- Jest test suite for SDK

## [0.1.0] - 2026-01-15

### Added
- Initial Anchor program with CPE state management
- Core math library for dependency graph scheduling
- Basic parallel execution proof generation

