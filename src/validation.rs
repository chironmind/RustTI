//! Internal validation helpers for input checking
//!
//! This module provides centralized validation functions to ensure consistent
//! error messages and behavior across all technical indicator calculations.
//! These helpers return Results with uniform error messages when validation fails.

/// Validates that a slice is not empty
///
/// # Arguments
///
/// * `name` - Human-readable name of the data (e.g., "prices", "highs", "lows")
/// * `slice` - The slice to validate
///
/// # Errors
///
/// Returns `TechnicalIndicatorError::EmptyData` if the slice is empty
#[inline]
pub fn assert_non_empty<T>(name: &str, slice: &[T]) -> crate::Result<()> {
    if slice.is_empty() {
        return Err(crate::TechnicalIndicatorError::EmptyData {
            name: name.to_string(),
        });
    }
    Ok(())
}

/// Validates that multiple slices have the same length
///
/// # Arguments
///
/// * `slices` - Array of tuples containing (name, slice) pairs
///
/// # Errors
///
/// Returns `TechnicalIndicatorError::MismatchedLength` if slices have different lengths
///
/// # Example
///
/// ```ignore
/// assert_same_len(&[("high", high), ("low", low), ("close", close)])?;
/// ```
#[inline]
pub fn assert_same_len<T>(slices: &[(&str, &[T])]) -> crate::Result<()> {
    if slices.is_empty() {
        return Ok(());
    }

    let expected_len = slices[0].1.len();

    for (_name, slice) in slices {
        let len = slice.len();
        if len != expected_len {
            // Build error info with all lengths
            let names: Vec<(String, usize)> = slices
                .iter()
                .map(|(n, s)| (n.to_string(), s.len()))
                .collect();
            return Err(crate::TechnicalIndicatorError::MismatchedLength { names });
        }
    }
    Ok(())
}

/// Validates that a period is valid for the given data length
///
/// # Arguments
///
/// * `period` - The period parameter
/// * `data_len` - Length of the data slice
///
/// # Errors
///
/// Returns `TechnicalIndicatorError::InvalidPeriod` if period is 0 or greater than data_len
#[inline]
pub fn assert_period(period: usize, data_len: usize) -> crate::Result<()> {
    if period == 0 {
        return Err(crate::TechnicalIndicatorError::InvalidPeriod {
            period,
            data_len,
            reason: "must be greater than 0".to_string(),
        });
    }

    if period > data_len {
        return Err(crate::TechnicalIndicatorError::InvalidPeriod {
            period,
            data_len,
            reason: "cannot be longer than data length".to_string(),
        });
    }
    Ok(())
}

/// Validates that a value is positive (> 0)
///
/// # Arguments
///
/// * `name` - Human-readable name of the value
/// * `value` - The value to validate
///
/// # Errors
///
/// Returns `TechnicalIndicatorError::InvalidValue` if the value is not positive
#[inline]
pub fn assert_positive(name: &str, value: f64) -> crate::Result<()> {
    if value <= 0.0 || value.is_nan() {
        return Err(crate::TechnicalIndicatorError::InvalidValue {
            name: name.to_string(),
            value,
            reason: "must be greater than 0".to_string(),
        });
    }
    Ok(())
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
/// # Errors
///
/// Returns `TechnicalIndicatorError::InvalidValue` if the value is not in range
#[inline]
pub fn assert_range(name: &str, value: f64, min: f64, max: f64) -> crate::Result<()> {
    if value <= min || value >= max || value.is_nan() {
        return Err(crate::TechnicalIndicatorError::InvalidValue {
            name: name.to_string(),
            value,
            reason: format!("must be in range ({}, {})", min, max),
        });
    }
    Ok(())
}

/// Validates that a value is greater than a minimum
///
/// # Arguments
///
/// * `name` - Human-readable name of the value
/// * `value` - The value to validate
/// * `min` - Minimum acceptable value (exclusive)
///
/// # Errors
///
/// Returns `TechnicalIndicatorError::InvalidValue` if value <= min
#[inline]
pub fn assert_min_value(name: &str, value: f64, min: f64) -> crate::Result<()> {
    if value <= min || value.is_nan() {
        return Err(crate::TechnicalIndicatorError::InvalidValue {
            name: name.to_string(),
            value,
            reason: format!("must be greater than {}", min),
        });
    }
    Ok(())
}

/// Validates that all values in a slice are positive
///
/// # Arguments
///
/// * `name` - Human-readable name of the data
/// * `slice` - The slice to validate
///
/// # Errors
///
/// Returns `TechnicalIndicatorError::InvalidValue` with the first non-positive value found
#[inline]
pub fn assert_all_positive(name: &str, slice: &[f64]) -> crate::Result<()> {
    for &value in slice {
        if value <= 0.0 {
            return Err(crate::TechnicalIndicatorError::InvalidValue {
                name: name.to_string(),
                value,
                reason: "requires all positive values".to_string(),
            });
        }
    }
    Ok(())
}

/// Validates that a period is at least a minimum value
///
/// # Arguments
///
/// * `period` - The period parameter
/// * `min_period` - Minimum acceptable period
/// * `data_len` - Length of the data slice
///
/// # Errors
///
/// Returns `TechnicalIndicatorError::InvalidPeriod` if period < min_period or period > data_len
#[inline]
pub fn assert_min_period(period: usize, min_period: usize, data_len: usize) -> crate::Result<()> {
    if period < min_period {
        return Err(crate::TechnicalIndicatorError::InvalidPeriod {
            period,
            data_len,
            reason: format!("must be at least {}", min_period),
        });
    }

    assert_period(period, data_len)?;
    Ok(())
}

/// Validates that a slice has a minimum length
///
/// # Arguments
///
/// * `name` - Human-readable name of the data
/// * `min_length` - Minimum required length
/// * `actual_length` - Actual length of the data
///
/// # Errors
///
/// Returns `TechnicalIndicatorError::InvalidPeriod` if actual_length < min_length
#[inline]
pub fn assert_min_length(name: &str, min_length: usize, actual_length: usize) -> crate::Result<()> {
    if actual_length < min_length {
        return Err(crate::TechnicalIndicatorError::InvalidPeriod {
            period: min_length,
            data_len: actual_length,
            reason: format!("{} must be at least {} in length", name, min_length),
        });
    }
    Ok(())
}

/// Validates that a usize value is positive (> 0)
///
/// # Arguments
///
/// * `name` - Human-readable name of the value
/// * `value` - The value to validate
///
/// # Errors
///
/// Returns `TechnicalIndicatorError::InvalidValue` if the value is 0
#[inline]
pub fn assert_positive_usize(name: &str, value: usize) -> crate::Result<()> {
    if value == 0 {
        return Err(crate::TechnicalIndicatorError::InvalidValue {
            name: name.to_string(),
            value: value as f64,
            reason: "must be greater than 0".to_string(),
        });
    }
    Ok(())
}

/// Returns an error indicating that a type variant is not supported
///
/// # Arguments
///
/// * `type_name` - Name of the type/variant
///
/// # Returns
///
/// Always returns `TechnicalIndicatorError::UnsupportedType`
#[inline]
pub fn unsupported_type(type_name: &str) -> crate::TechnicalIndicatorError {
    crate::TechnicalIndicatorError::UnsupportedType {
        type_name: type_name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_non_empty_ok() {
        let non_empty = vec![1.0, 2.0, 3.0];
        assert!(assert_non_empty("prices", &non_empty).is_ok());
    }

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

    #[test]
    fn test_assert_same_len_ok() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        assert!(assert_same_len(&[("a", &a), ("b", &b)]).is_ok());
    }

    #[test]
    fn test_assert_same_len_fail() {
        let a = vec![1.0, 2.0, 3.0];
        let c = vec![7.0, 8.0];
        let result = assert_same_len(&[("a", &a), ("c", &c)]);
        assert!(result.is_err());
        match result {
            Err(crate::TechnicalIndicatorError::MismatchedLength { names }) => {
                assert_eq!(names.len(), 2);
            }
            _ => panic!("Expected MismatchedLength error"),
        }
    }

    #[test]
    fn test_assert_period_ok() {
        assert!(assert_period(5, 10).is_ok());
    }

    #[test]
    fn test_assert_period_zero() {
        let result = assert_period(0, 10);
        assert!(result.is_err());
        match result {
            Err(crate::TechnicalIndicatorError::InvalidPeriod { period, .. }) => {
                assert_eq!(period, 0);
            }
            _ => panic!("Expected InvalidPeriod error"),
        }
    }

    #[test]
    fn test_assert_period_too_long() {
        let result = assert_period(11, 10);
        assert!(result.is_err());
        match result {
            Err(crate::TechnicalIndicatorError::InvalidPeriod {
                period, data_len, ..
            }) => {
                assert_eq!(period, 11);
                assert_eq!(data_len, 10);
            }
            _ => panic!("Expected InvalidPeriod error"),
        }
    }

    #[test]
    fn test_assert_positive_ok() {
        assert!(assert_positive("value", 1.0).is_ok());
    }

    #[test]
    fn test_assert_positive_zero() {
        let result = assert_positive("value", 0.0);
        assert!(result.is_err());
        match result {
            Err(crate::TechnicalIndicatorError::InvalidValue { name, value, .. }) => {
                assert_eq!(name, "value");
                assert_eq!(value, 0.0);
            }
            _ => panic!("Expected InvalidValue error"),
        }
    }

    #[test]
    fn test_assert_range_ok() {
        assert!(assert_range("quantile", 0.5, 0.0, 1.0).is_ok());
    }

    #[test]
    fn test_assert_range_at_min() {
        let result = assert_range("quantile", 0.0, 0.0, 1.0);
        assert!(result.is_err());
        match result {
            Err(crate::TechnicalIndicatorError::InvalidValue { name, value, .. }) => {
                assert_eq!(name, "quantile");
                assert_eq!(value, 0.0);
            }
            _ => panic!("Expected InvalidValue error"),
        }
    }

    #[test]
    fn test_assert_all_positive_ok() {
        let positive = vec![1.0, 2.0, 3.0];
        assert!(assert_all_positive("prices", &positive).is_ok());
    }

    #[test]
    fn test_assert_all_positive_has_zero() {
        let has_zero = vec![1.0, 0.0, 3.0];
        let result = assert_all_positive("prices", &has_zero);
        assert!(result.is_err());
        match result {
            Err(crate::TechnicalIndicatorError::InvalidValue { name, value, .. }) => {
                assert_eq!(name, "prices");
                assert_eq!(value, 0.0);
            }
            _ => panic!("Expected InvalidValue error"),
        }
    }

    #[test]
    fn test_assert_min_period_ok() {
        assert!(assert_min_period(4, 4, 10).is_ok());
    }

    #[test]
    fn test_assert_min_period_too_small() {
        let result = assert_min_period(3, 4, 10);
        assert!(result.is_err());
        match result {
            Err(crate::TechnicalIndicatorError::InvalidPeriod { period, .. }) => {
                assert_eq!(period, 3);
            }
            _ => panic!("Expected InvalidPeriod error"),
        }
    }
}
