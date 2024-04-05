# Changelog

## [0.1.0]

### Added

- Create `Datetime` struct to store timestamp
- Add `format_iso8601` to convert timestamp to String (also implement `Display`)
- Add `std` flag to make it compatible for no_std
- Add tests to test the formatting code
- Add property tests to ensure compatibility with `chrono` and `time`