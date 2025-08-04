# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
