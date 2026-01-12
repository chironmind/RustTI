//! Internal validation helpers for input checking
//!
//! This module provides centralized validation functions to ensure consistent
//! error messages and behavior across all technical indicator calculations.
//! These helpers panic with uniform error messages when validation fails.

/// Validates that a slice is not empty
///
/// # Arguments
///
/// * `name` - Human-readable name of the data (e.g., "prices", "highs", "lows")
/// * `slice` - The slice to validate
///
/// # Panics
///
/// Panics if the slice is empty with a consistent error message
#[inline]
pub fn assert_non_empty<T>(name: &str, slice: &[T]) {
    if slice.is_empty() {
        panic!("{} cannot be empty", name);
    }
}

/// Validates that multiple slices have the same length
///
/// # Arguments
///
/// * `slices` - Array of tuples containing (name, slice) pairs
///
/// # Panics
///
/// Panics if slices have mismatched lengths
///
/// # Example
///
/// ```ignore
/// assert_same_len(&[("high", high), ("low", low), ("close", close)]);
/// ```
#[inline]
pub fn assert_same_len<T>(slices: &[(&str, &[T])]) {
    if slices.is_empty() {
        return;
    }

    let expected_len = slices[0].1.len();

    for (name, slice) in slices {
        let len = slice.len();
        if len != expected_len {
            // Build error message with all lengths
            let lengths: Vec<String> = slices
                .iter()
                .map(|(n, s)| format!("{}={}", n, s.len()))
                .collect();
            panic!("Mismatched lengths: {}", lengths.join(", "));
        }
    }
}

/// Validates that a period is valid for the given data length
///
/// # Arguments
///
/// * `period` - The period parameter
/// * `data_len` - Length of the data slice
///
/// # Panics
///
/// Panics if period is 0 or greater than data_len
#[inline]
pub fn assert_period(period: usize, data_len: usize) {
    if period == 0 {
        panic!("Period ({}) must be greater than 0", period);
    }

    if period > data_len {
        panic!(
            "Period ({}) cannot be longer than the length of provided data ({})",
            period, data_len
        );
    }
}

/// Validates that a value is positive (> 0)
///
/// # Arguments
///
/// * `name` - Human-readable name of the value
/// * `value` - The value to validate
///
/// # Panics
///
/// Panics if the value is not positive
#[inline]
pub fn assert_positive(name: &str, value: f64) {
    if value <= 0.0 || value.is_nan() {
        panic!("{} ({}) must be greater than 0", name, value);
    }
}

/// Validates that a value is within a specific range
///
/// # Arguments
///
/// * `name` - Human-readable name of the value
/// * `value` - The value to validate
/// * `min` - Minimum acceptable value (exclusive)
/// * `max` - Maximum acceptable value (exclusive)
///
/// # Panics
///
/// Panics if the value is not in range
#[inline]
pub fn assert_range(name: &str, value: f64, min: f64, max: f64) {
    if value <= min || value >= max || value.is_nan() {
        panic!("{} ({}) must be in range ({}, {})", name, value, min, max);
    }
}

/// Validates that a value is greater than a minimum
///
/// # Arguments
///
/// * `name` - Human-readable name of the value
/// * `value` - The value to validate
/// * `min` - Minimum acceptable value (exclusive)
///
/// # Panics
///
/// Panics if value <= min
#[inline]
pub fn assert_min_value(name: &str, value: f64, min: f64) {
    if value <= min || value.is_nan() {
        panic!("{} ({}) must be greater than {}", name, value, min);
    }
}

/// Validates that all values in a slice are positive
///
/// # Arguments
///
/// * `name` - Human-readable name of the data
/// * `slice` - The slice to validate
///
/// # Panics
///
/// Panics with the first non-positive value found
#[inline]
pub fn assert_all_positive(name: &str, slice: &[f64]) {
    for &value in slice {
        if value <= 0.0 {
            panic!("{} requires all positive values; found {}", name, value);
        }
    }
}

/// Validates that a period is at least a minimum value
///
/// # Arguments
///
/// * `period` - The period parameter
/// * `min_period` - Minimum acceptable period
/// * `data_len` - Length of the data slice
///
/// # Panics
///
/// Panics if period < min_period or period > data_len
#[inline]
pub fn assert_min_period(period: usize, min_period: usize, data_len: usize) {
    if period < min_period {
        panic!("Period ({}) must be at least {}", period, min_period);
    }

    assert_period(period, data_len);
}

/// Validates that a type variant is supported
///
/// # Arguments
///
/// * `type_name` - Name of the type/variant
///
/// # Panics
///
/// Always panics with unsupported type message
#[inline]
pub fn unsupported_type(type_name: &str) -> ! {
    panic!("Unsupported type: {}", type_name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_non_empty_ok() {
        let non_empty = vec![1.0, 2.0, 3.0];
        assert_non_empty("prices", &non_empty); // Should not panic
    }

    #[test]
    #[should_panic(expected = "prices cannot be empty")]
    fn test_assert_non_empty_fail() {
        let empty: Vec<f64> = vec![];
        assert_non_empty("prices", &empty);
    }

    #[test]
    fn test_assert_same_len_ok() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        assert_same_len(&[("a", &a), ("b", &b)]); // Should not panic
    }

    #[test]
    #[should_panic(expected = "Mismatched lengths")]
    fn test_assert_same_len_fail() {
        let a = vec![1.0, 2.0, 3.0];
        let c = vec![7.0, 8.0];
        assert_same_len(&[("a", &a), ("c", &c)]);
    }

    #[test]
    fn test_assert_period_ok() {
        assert_period(5, 10); // Should not panic
    }

    #[test]
    #[should_panic(expected = "Period (0) must be greater than 0")]
    fn test_assert_period_zero() {
        assert_period(0, 10);
    }

    #[test]
    #[should_panic(expected = "Period (11) cannot be longer")]
    fn test_assert_period_too_long() {
        assert_period(11, 10);
    }

    #[test]
    fn test_assert_positive_ok() {
        assert_positive("value", 1.0); // Should not panic
    }

    #[test]
    #[should_panic(expected = "value (0) must be greater than 0")]
    fn test_assert_positive_zero() {
        assert_positive("value", 0.0);
    }

    #[test]
    fn test_assert_range_ok() {
        assert_range("quantile", 0.5, 0.0, 1.0); // Should not panic
    }

    #[test]
    #[should_panic(expected = "quantile (0) must be in range (0, 1)")]
    fn test_assert_range_at_min() {
        assert_range("quantile", 0.0, 0.0, 1.0);
    }

    #[test]
    fn test_assert_all_positive_ok() {
        let positive = vec![1.0, 2.0, 3.0];
        assert_all_positive("prices", &positive); // Should not panic
    }

    #[test]
    #[should_panic(expected = "prices requires all positive values; found 0")]
    fn test_assert_all_positive_has_zero() {
        let has_zero = vec![1.0, 0.0, 3.0];
        assert_all_positive("prices", &has_zero);
    }

    #[test]
    fn test_assert_min_period_ok() {
        assert_min_period(4, 4, 10); // Should not panic
    }

    #[test]
    #[should_panic(expected = "Period (3) must be at least 4")]
    fn test_assert_min_period_too_small() {
        assert_min_period(3, 4, 10);
    }
}
