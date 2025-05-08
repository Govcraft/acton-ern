# Changelog

All notable changes to the Acton ERN project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0]

### Added
- Serialization/deserialization support via optional `serde` feature

### Changed
- Improved error handling with more specific error types
- Enhanced validation rules for ERN components
- Refactored internal structure for better maintainability
- Updated documentation with more examples and use cases

### Fixed
- Resolved issues with parsing complex ERNs
- Fixed edge cases in validation logic
- Addressed performance bottlenecks in builder pattern

## [0.9.0] - 2025-05-07

### Added
- Initial pre-release version
- Core ERN component models (Domain, Category, Account, Root, Part)
- Basic validation rules
- Error handling framework
- ERN builder implementation
- ERN parser implementation
- Support for different ID types (SHA1Name, Timestamp, UnixTime)
- Methods for adding parts, changing roots
- Parent-child relationship checking
- ERN combination operations
- Feature flags for optional components
- Comprehensive test suite
- Documentation and examples

### Changed
- Migrated from alpha version (2.1.1-alpha) to pre-release versioning (0.9.0)
- Restructured codebase for better organization
- Updated dependencies to latest versions
- Improved API ergonomics

## [2.1.1-alpha] - Previous Version

Initial alpha release with basic functionality.