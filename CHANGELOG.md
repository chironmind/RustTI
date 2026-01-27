# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]
### Added
- Reference URLS to doc strings

## [1.0.0] - 2026-01-07
### Changed
- **BREAKING:** Rebranded from RustTI to Centaur Technical Indicators
  - Package name changed from `rust_ti` to `centaur_technical_indicators`
  - This is a new package on crates.io with fresh versioning (1.0.0)
  - All functionality remains the same, only branding has changed
  - Updated repository and documentation URLs to reflect Centaur Labs branding
- **BREAKING:** `panic!` replaced with `Result<>` types in several functions for better error handling

### Removed
- Removed unused `deviation.rs` file

---

## Everything below this line is from RustTI changelog

## [2.2.0] - 2025-10-19
### Added
- Added new deviation indicators:
  - log_standard_deviation
  - student_t_adjusted_std
  - laplace_std_equivalent
  - cauchy_iqr_scale
- AbsDevConfig and DeviationAggregate to allow caller to specify which aggregate to use for absolute deviation calculations

### Changed
- Updated DeviationModel to include new deviation types, and CustomAbsoluteDeviation that allows caller to specify which central point and aggregate to use
- absolute_deviation now uses AbsDevConfig to allow caller to specify which aggregate to use

## [2.1.5] - 2025-10-07
### Added
- Added new indicator: Price distribution

### Changed
- Minor document updates
- `break_down_trends` made more reliable and easier to use 
  - Added a config struct to hold parameters
  - Fixed internal logic to be more robust

## [2.1.4] - 2025-08-07
### Changes
- Minor document updates

### Fixes
- Fixed Welles' Volatility System, in some edge cases it would try to make an immediate pivot after establishing a SaR, which caused a crash. It has been updated to try for an extra period to confirm trend direction

---

## [2.1.3] - 2025-08-04
### Changes
- Minor document updates
- Made directional movement system error message clearer

---

## [2.1.2] - 2025-07-27
### Changed
- Minor document updates

---

## [2.1.1] - 2025-07-22
### Fixed
- Chaikin Oscillator was taking the first Accumulation Distribution instead of the last

### Changed
- Minor doc updates

---

## [2.1.0] - 2025-07-20
### Added
- Added benchmarks to README
- Added tutorials to README

### Changed
- Removed unused loop from valleys
- Inlined functions to improve runtime

---

## [2.0.0] - 2025-07-03
### Added
- Expanded and improved documentation for core modules, including comprehensive doc comments and usage examples for `basic_indicators`, `candle_indicators`, `chart_trends`, and `correlation_indicators`.
- Additional inline documentation and usage instructions in the README.md and CONTRIBUTING.md files, clarifying usage philosophy and adding mascot introduction.
- New doc tests and panic handling for invalid period lengths and other edge cases in indicator functions.

### Changed
- Major refactor of argument signatures: Many functions (especially in `basic_indicators`, `chart_trends`, `correlation_indicators`) now take plain values (e.g., period: usize) instead of references (e.g., &usize).
- Improved error handling and panic messages across all indicator modules for consistency and clarity.
- Numerous functions now use iterators and more idiomatic Rust for windowed calculations and internal logic.
- Refined and clarified module-level and function-level documentation throughout the codebase.
- Refactored custom type handling to use more idiomatic Rust enums and structures.
- Updated tests across modules to cover new error handling and edge cases.

### Removed
- Deprecated legacy argument patterns (e.g., passing reference to period) across most modules for a cleaner API.
- Removed repetitive or redundant docstrings in favor of more centralized, clearer documentation
- Removed main and visa from examples to fall in line with diataxis, clearer tutorials and how tos will be put in another repo

---

## [1.4.2] - 2024-06-27
### Added
- Improved `peaks` and `valleys` function: now avoids producing peaks/valleys when the period shifted and was within a given period of the previous one.

### Changed
- Documentation updates for several indicators.

---

## [1.4.1] - 2024-05-10
### Fixed
- Fixed bug in exponential moving average calculation.
- Minor code formatting improvements.

---

## [1.4.0] - 2024-04-01
### Added
- New indicator: McGinley Dynamic Bands.
- Added configuration options for moving averages.
- Added S&P 500 and Visa usage examples.

### Changed
- Refactored indicator modules for improved organization.

### Fixed
- Calculation bug in RSI fixed.
- Typo corrections in documentation.

---

## [1.3.0] - 2023-12-20
### Added
- Support for more than 70 unique technical indicators.
- Personalised moving average type.
- Bulk and single calculation modes for all indicators.
- Improved error handling for invalid input.

### Changed
- Major refactor of moving average module for flexibility.

---

## [1.2.0] - 2023-07-15
### Added
- Candle indicators: Ichimoku Cloud, McGinley Dynamic Bands/Envelopes, Moving Constant Bands, Donchian Channels, Keltner Channel, Supertrend.
- Chart trend indicators: breakdown, peaks, valleys, trend detection.
- Correlation and momentum indicators (Chaikin Oscillator, MACD, etc).

---

## [1.1.0] - 2023-03-30
### Added
- Standard indicators: Simple, Smoothed, Exponential Moving Averages, Bollinger Bands, MACD, RSI.
- Basic statistical indicators: mean, median, mode, standard deviation, variance, min, max, etc.

---

## [1.0.0] - 2023-01-10
### Added
- Initial release of RustTI.
- Core library structure with modular technical indicator functions.
- Full documentation on docs.rs.
- Unit tests and hand-calculation verification spreadsheets.

---
