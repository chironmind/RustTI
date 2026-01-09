//! Error types for Centaur Technical Indicators
//!
//! This module defines error types that may be used in the future.
//! Currently, the library uses panics for validation failures with centralized
//! validation helpers in the `validation` module.

#![allow(dead_code)]

use std::fmt;

/// The main error type for technical indicator calculations
#[derive(Debug, Clone, PartialEq)]
pub enum TechnicalIndicatorError {
    /// Input data is empty when it shouldn't be
    EmptyData {
        name: String,
    },
    /// Multiple input slices have mismatched lengths
    MismatchedLength {
        names: Vec<(String, usize)>,
    },
    /// Period parameter is invalid (e.g., zero or larger than data length)
    InvalidPeriod {
        period: usize,
        data_len: usize,
        reason: String,
    },
    /// A numeric value is out of acceptable range
    InvalidValue {
        name: String,
        value: f64,
        reason: String,
    },
    /// An enum variant is not supported in this context
    UnsupportedType {
        type_name: String,
    },
    /// Custom error message for edge cases
    Custom {
        message: String,
    },
}

impl fmt::Display for TechnicalIndicatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TechnicalIndicatorError::EmptyData { name } => {
                write!(f, "{} cannot be empty", name)
            }
            TechnicalIndicatorError::MismatchedLength { names } => {
                write!(f, "Mismatched lengths: ")?;
                for (i, (name, len)) in names.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}={}", name, len)?;
                }
                Ok(())
            }
            TechnicalIndicatorError::InvalidPeriod { period, data_len, reason } => {
                write!(
                    f,
                    "Invalid period {}: {} (data length: {})",
                    period, reason, data_len
                )
            }
            TechnicalIndicatorError::InvalidValue { name, value, reason } => {
                write!(f, "Invalid value for {}: {} ({})", name, value, reason)
            }
            TechnicalIndicatorError::UnsupportedType { type_name } => {
                write!(f, "Unsupported type: {}", type_name)
            }
            TechnicalIndicatorError::Custom { message } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl std::error::Error for TechnicalIndicatorError {}

/// Convenience type alias for Results in this library
pub type Result<T> = std::result::Result<T, TechnicalIndicatorError>;
