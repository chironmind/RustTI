# Migration Implementation Guide

## Status: Foundation Complete ✅

The foundation for migrating from `panic!` to `Result` types is complete:
- ✅ **validation.rs**: All validation functions return `Result<()>`
- ✅ **error.rs**: Error types ready to use
- ✅ **Assessment complete**: See `ERROR_HANDLING_ASSESSMENT.md`

## Next Steps

### Option 1: Complete the Migration (Recommended)

Follow the systematic approach outlined in `ERROR_HANDLING_ASSESSMENT.md`:

#### Phase 1: Start Small
Begin with `moving_average.rs` (only 4 public functions):

```bash
# 1. Update the code following patterns in migration_helper.sh
# 2. Check compilation
cargo check --lib

# 3. Fix all compilation errors
# 4. Update tests
# 5. Run tests for this module
cargo test --lib moving_average

# 6. When passing, commit
git add src/moving_average.rs
git commit -m "Migrate moving_average to Result types"
```

#### Phase 2-4: Continue Systematically
Follow the order in `migration_helper.sh`, testing after each module.

#### Phase 5: Final Integration
- Update `examples/reference.rs`
- Update README examples
- Write migration guide for users
- Run full test suite
- Update version to 2.0.0

### Option 2: Wait for Feedback

If you want maintainer/stakeholder input before proceeding:

1. Review `ERROR_HANDLING_ASSESSMENT.md`
2. Discuss the breaking change implications
3. Get agreement on v2.0.0 approach
4. Then proceed with Option 1

## What's Done

### validation.rs Changes

All validation functions now return `Result` types:

```rust
// Before
pub fn assert_non_empty<T>(name: &str, slice: &[T]) {
    if slice.is_empty() {
        panic!("{} cannot be empty", name);
    }
}

// After
pub fn assert_non_empty<T>(name: &str, slice: &[T]) -> crate::Result<()> {
    if slice.is_empty() {
        return Err(crate::TechnicalIndicatorError::EmptyData {
            name: name.to_string(),
        });
    }
    Ok(())
}
```

Functions updated:
- `assert_non_empty`
- `assert_same_len`
- `assert_period`
- `assert_positive`
- `assert_range`
- `assert_min_value`
- `assert_all_positive`
- `assert_min_period`
- `assert_min_length`
- `assert_positive_usize`
- `unsupported_type` (now returns error instead of `!`)

### Test Pattern Changes

Tests updated to check for Result types instead of panics:

```rust
// Before
#[test]
#[should_panic(expected = "prices cannot be empty")]
fn test_assert_non_empty_fail() {
    let empty: Vec<f64> = vec![];
    assert_non_empty("prices", &empty);
}

// After
#[test]
fn test_assert_non_empty_fail() {
    let empty: Vec<f64> = vec![];
    let result = assert_non_empty("prices", &empty);
    assert!(result.is_err());
    match result {
        Err(crate::TechnicalIndicatorError::EmptyData { name }) => {
            assert_eq!(name, "prices");
        }
        _ => panic!("Expected EmptyData error"),
    }
}
```

## Common Patterns for Remaining Work

### 1. Update Function Signatures

```rust
// Before
pub fn indicator(prices: &[f64], period: usize) -> f64 {
    assert_non_empty("prices", prices);
    assert_period(period, prices.len());
    // ... calculation
    result
}

// After
pub fn indicator(prices: &[f64], period: usize) -> crate::Result<f64> {
    assert_non_empty("prices", prices)?;
    assert_period(period, prices.len())?;
    // ... calculation
    Ok(result)
}
```

### 2. Update Calls to Other Functions

```rust
// Before
let avg = mean(prices);
let dev = standard_deviation(prices);

// After
let avg = mean(prices)?;
let dev = standard_deviation(prices)?;
```

### 3. Update Match Expressions

```rust
// Before
let value = match some_enum {
    Variant1 => calc1(data),
    Variant2 => calc2(data),
    _ => unsupported_type("TypeName"),
};

// After
let value = match some_enum {
    Variant1 => calc1(data)?,
    Variant2 => calc2(data)?,
    _ => return Err(unsupported_type("TypeName")),
};
```

### 4. Update Documentation

```rust
// Before
/// # Panics
///
/// Panics if `prices.is_empty()` or `period` > `prices.len()`

// After
/// # Errors
///
/// Returns `TechnicalIndicatorError::EmptyData` if `prices.is_empty()`
/// Returns `TechnicalIndicatorError::InvalidPeriod` if `period` > `prices.len()`
```

### 5. Update Tests

For successful cases:
```rust
// Before
let result = indicator(&prices, 14);
assert_eq!(expected, result);

// After
let result = indicator(&prices, 14).unwrap();
assert_eq!(expected, result);
```

For error cases:
```rust
// Before
#[test]
#[should_panic]
fn test_error() {
    indicator(&[], 14);
}

// After
#[test]
fn test_error() {
    let result = indicator(&[], 14);
    assert!(result.is_err());
}
```

## Estimated Effort Per Module

| Module | LOC | Functions | Tests | Estimated Time |
|--------|-----|-----------|-------|----------------|
| moving_average | 469 | 4 | 16 | 1-2 hours |
| volatility_indicators | 416 | ~6 | ~20 | 2-3 hours |
| chart_trends | 635 | ~10 | ~25 | 3-4 hours |
| correlation_indicators | 654 | ~6 | ~15 | 2-3 hours |
| other_indicators | 1,154 | ~8 | ~30 | 3-4 hours |
| strength_indicators | 1,170 | ~8 | ~35 | 3-4 hours |
| basic_indicators | 2,008 | 29 | 82 | 6-8 hours |
| trend_indicators | 2,128 | ~15 | ~55 | 6-8 hours |
| candle_indicators | 2,470 | ~12 | ~60 | 8-10 hours |
| momentum_indicators | 4,651 | 32 | 178 | 12-16 hours |

**Total estimate**: 46-62 hours of focused work

With testing, documentation, and integration: **2-3 weeks** of systematic work

## Tools and Scripts

- `migration_helper.sh`: Shows status and patterns
- `ERROR_HANDLING_ASSESSMENT.md`: Complete analysis and strategy
- This file: Step-by-step implementation guide

## Questions?

Refer to `ERROR_HANDLING_ASSESSMENT.md` for:
- Detailed rationale
- Risk assessment
- User migration guide
- Benefits analysis

## Summary

**Foundation is ready.** The systematic work can begin at any time. Each module follows the same pattern, so the process becomes faster with each one completed.

**Recommendation**: Start with `moving_average.rs` to validate the approach, then proceed systematically through the remaining modules.
