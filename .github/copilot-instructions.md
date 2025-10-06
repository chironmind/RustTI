# RustTI Copilot Instructions

## Repository Overview

**RustTI** is a comprehensive technical indicators library written in pure Rust for financial data analysis. The library provides over 70 configurable technical indicators across 11 specialized modules, supporting stocks, crypto, and any asset with arbitrary trading calendars.

**Key Characteristics:**
- **Type**: Library crate (not an application)
- **Language**: Pure Rust (edition 2021, version 2.1.4)
- **Dependencies**: None - completely self-contained
- **Size**: ~15,550 lines of code across 11 modules
- **Testing**: Extensive test suite with 533+ tests including unit tests and doc tests
- **License**: MIT License

## Build Instructions

### Prerequisites
- Rust toolchain (stable, beta, or nightly supported)
- No external dependencies required

### Essential Commands (Always run in repository root)

**Build and Validation (in order of speed):**
```bash
cargo check          # Fast validation (1-2s) - run first for syntax/type checking
cargo build           # Standard build (4-5s) - compiles the library
cargo test            # Run all 533 tests (6-7s) - validates functionality
cargo clippy          # Linting (0.14s) - catches common issues, some warnings expected
cargo fmt             # Code formatting - ensures consistent style
cargo doc --no-deps   # Generate documentation (0.09s) - creates API docs
```

**Examples and Benchmarks:**
```bash
cargo run --example reference  # Run the comprehensive reference example (demonstrates all indicators)
cargo bench --no-run          # Compile benchmarks (optional)
```

### Expected Build Behavior
- **One known warning**: `value assigned to 'end_index' is never read` in `src/chart_trends.rs:403` - this is harmless and doesn't affect functionality
- **Clippy warnings**: "too many arguments" warnings for some functions - these are style warnings, not errors
- **All commands should complete successfully** - if any fail, investigate the specific error

### Testing Notes
- Tests run in ~6 seconds and should always pass
- Tests include both unit tests and doc tests
- Hand-calculation verification spreadsheet available at `assets/rust_ti_hand_calcs.ods`
- **Never modify tests to make unrelated code pass** - tests are carefully validated

## Project Architecture and Layout

### Module Structure
```
src/
├── lib.rs                    # Main library entry point with module exports
├── types.rs                  # Shared enums (MovingAverageType, DeviationModel, etc.)
├── basic_indicators.rs       # Core statistical functions (mean, median, std dev)
├── candle_indicators.rs      # Price-based indicators (bands, envelopes, channels)
├── momentum_indicators.rs    # Momentum and oscillators (RSI, MACD, Stochastic)
├── moving_average.rs         # Moving averages (SMA, EMA, McGinley Dynamic)
├── trend_indicators.rs       # Trend analysis (Aroon, DMS, Parabolic SAR)
├── volatility_indicators.rs  # Volatility measures (Ulcer Index, volatility systems)
├── strength_indicators.rs    # Volume-based indicators (Accumulation/Distribution)
├── correlation_indicators.rs # Asset correlation metrics
├── chart_trends.rs          # Trend and peak/valley analysis
├── other_indicators.rs      # Miscellaneous (ROI, True Range, Internal Bar Strength)
└── standard_indicators.rs   # Common indicators (Bollinger Bands, standard MACD)
```

### Key Design Patterns
- **Dual Function Structure**: Each module provides both `single` and `bulk` calculations
  - `single`: Calculate indicator for one period or entire dataset
  - `bulk`: Calculate indicator over sliding windows for time series
- **Configuration Enums**: Extensive use of enums in `types.rs` for customization
- **Error Handling**: Functions panic on invalid inputs (empty data, invalid periods)

### Configuration Files
- **Cargo.toml**: Main project configuration, no external dependencies
- **.github/workflows/rust.yml**: CI pipeline (tests on stable/beta/nightly Rust)
- **.gitignore**: Only excludes `/target` directory
- **No additional config files** - standard Rust project layout

### GitHub Actions CI/CD
The workflow runs on every push/PR to main branch:
```yaml
Strategy: Test on stable, beta, and nightly Rust
Steps: checkout → setup Rust → cargo build --verbose → cargo test --verbose
```
**Always ensure your changes pass on stable Rust before submitting.**

## Development Guidelines

### Code Organization
- **Find indicators by category**: Use module names to locate specific indicators
- **Check `types.rs` first**: Understand available enums before implementing
- **Follow naming conventions**: 
  - Functions use snake_case
  - Test functions currently mix `test_` prefix and no prefix (ongoing refactoring)
  - Some functions use `high` parameter, others `highs` (standardization needed)

### Common File Locations
- **Examples**: `/examples/reference.rs` - comprehensive usage examples
- **Documentation**: Run `cargo doc --open` for local API docs
- **Tests**: Integrated within each module file using `#[cfg(test)]`
- **Assets**: `/assets/` contains banner and hand-calculation spreadsheet

### Validation Steps
1. **Always run `cargo check` first** - fastest way to catch syntax errors
2. **Run `cargo test` before making changes** - understand current test state
3. **Use `cargo clippy`** - catch common issues, but warnings are expected
4. **Test examples**: `cargo run --example reference` should complete without errors
5. **Verify documentation builds**: `cargo doc --no-deps` should succeed

### Performance Notes
- **Build time**: Very fast (~1-2s for full build)
- **Test time**: Moderate (~6s for full suite)
- **No heavy dependencies**: Pure Rust means no complex build requirements

## Common Tasks

### Adding New Indicators
1. Choose appropriate module based on indicator type
2. Implement both `single` and `bulk` versions
3. Add comprehensive tests with hand-calculated expected values
4. Update `assets/rust_ti_hand_calcs.ods` with verification calculations
5. Add doc tests with usage examples

### Modifying Existing Indicators
1. **Understand the math**: Check existing tests and documentation
2. **Preserve API compatibility**: Avoid breaking changes to public interfaces
3. **Update tests**: Ensure all existing tests continue to pass
4. **Check examples**: Verify `reference.rs` still works

### Debugging Build Issues
- **Import errors**: Check `lib.rs` for proper module exports
- **Type errors**: Verify enum usage against `types.rs` definitions
- **Test failures**: Use `cargo test -- --nocapture` for detailed output

## Trust These Instructions

These instructions are validated against the current codebase state. **Only search for additional information if these instructions are incomplete or incorrect.** The build commands, file locations, and architectural details have been tested and verified to work with the current repository state.

For additional context, see:
- `README.md` - Project overview and getting started guide
- `CONTRIBUTING.md` - Contribution guidelines and workflow
- [RustTI Tutorials](https://github.com/ChironMind/RustTI-tutorials) - External tutorial repository
- [RustTI Benchmarks](https://github.com/ChironMind/RustTI-benchmarks) - Performance testing repository