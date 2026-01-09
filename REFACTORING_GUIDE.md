# Validation Refactoring Guide

This guide documents the centralized validation refactoring for consistent error messages.

## Completed Modules

- ✅ `validation.rs` - Centralized validation helpers
- ✅ `error.rs` - Error types (for future use)
- ✅ `moving_average.rs` - 6 panics refactored
- ✅ `basic_indicators.rs` - 42 panics refactored

**Total: 48 panics refactored, 168 remaining across 9 modules**

## Pattern to Apply

### 1. Add imports at module level

```rust
// For single module:
use crate::validation::{
    assert_all_positive, assert_min_value, assert_non_empty, assert_period,
    assert_positive, assert_range, assert_same_len, unsupported_type,
};

// For bulk module (subset as needed):
use crate::validation::{assert_non_empty, assert_period, assert_positive};
```

### 2. Replace common patterns

#### Empty slice checks
**Before:**
```rust
if prices.is_empty() {
    panic!("Prices is empty");
}
// or
if prices.is_empty() {
    panic!("Prices ({:?}) is empty", prices);
}
// or
if prices.is_empty() {
    panic!("Prices cannot be empty");
}
```

**After:**
```rust
assert_non_empty("prices", prices);
```

#### Period validation
**Before:**
```rust
if period == 0 {
    panic!("Period ({}) must be greater than 0", period);
}
if period > prices.len() {
    panic!("Period ({}) cannot be longer than the length of provided prices ({})", period, prices.len());
}
```

**After:**
```rust
assert_period(period, prices.len());
```

#### Mismatched lengths
**Before:**
```rust
if length != highs.len() || length != lows.len() {
    panic!("Mismatched lengths: close={}, highs={}, lows={}", length, highs.len(), lows.len());
}
```

**After:**
```rust
assert_same_len(&[("close", close), ("highs", highs), ("lows", lows)]);
```

#### Positive value checks
**Before:**
```rust
if precision <= 0.0 || precision.is_nan() {
    panic!("precision ({}) must be > 0.0 and not NaN", precision);
}
```

**After:**
```rust
assert_positive("precision", precision);
```

#### Value ranges
**Before:**
```rust
if q <= 0.0 || q >= 1.0 {
    panic!("Quantile ({}) must be in (0,1)", q);
}
```

**After:**
```rust
assert_range("quantile", q, 0.0, 1.0);
```

#### Minimum value checks
**Before:**
```rust
if df <= 2.0 {
    panic!("Degrees of freedom ({}) must be greater than 2", df);
}
```

**After:**
```rust
assert_min_value("degrees_of_freedom", df, 2.0);
```

#### Unsupported type variants
**Before:**
```rust
_ => panic!("Unsupported ConstantModelType"),
// or
_ => panic!("Unsupported DeviationModel"),
```

**After:**
```rust
_ => unsupported_type("ConstantModelType"),
// or
_ => unsupported_type("DeviationModel"),
```

## Remaining Modules

### Priority Order (by usage frequency)

1. **momentum_indicators.rs** (~40 panics)
   - Most imported by other modules
   - High priority

2. **trend_indicators.rs** (~30 panics)
   - Moderate complexity
   - Medium priority

3. **candle_indicators.rs** (~35 panics)
   - Many length matching checks
   - Medium priority

4. **volatility_indicators.rs** (~12 panics)
   - Simpler module
   - Lower complexity

5. **other_indicators.rs** (~25 panics)
   - Mixed validation patterns
   - Medium complexity

6. **strength_indicators.rs** (~15 panics)
   - Volume validation
   - Lower complexity

7. **correlation_indicators.rs** (~8 panics)
   - Smallest remaining module
   - Low complexity

8. **chart_trends.rs** (~3 panics)
   - Very few panics
   - Lowest priority

## Testing Strategy

After refactoring each module:

1. Run module-specific tests:
   ```bash
   cargo test --lib <module_name>::tests
   ```

2. Check for compilation errors:
   ```bash
   cargo check --lib
   ```

3. Run full test suite:
   ```bash
   cargo test --lib
   ```

4. All existing tests should continue to pass with no changes required

## Benefits

- **Consistency**: All error messages follow the same format
- **Maintainability**: Single source of truth for validation logic
- **Readability**: Intent is clearer with descriptive helper names
- **Testability**: Validation logic is tested independently
- **Future-proofing**: Easy to switch to Result types if needed (helpers can be updated to return Results instead of panicking)

## Notes

- The `error.rs` module is included for potential future migration to Result types
- Current implementation maintains panic behavior for backward compatibility
- All 575 tests pass after refactoring completed modules
- No breaking API changes - functions still panic on invalid input, just with consistent messages
