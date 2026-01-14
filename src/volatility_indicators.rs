//! # Volatility Indicators
//!
//! This module provides functions for measuring the volatility of an asset—how much and how quickly prices move over time.
//!
//! ## When to Use
//! Use these indicators when you want to:
//! - Analyze the degree of price fluctuations or drawdowns in an asset
//! - Quantify downside volatility (Ulcer Index) instead of standard deviation
//! - Identify changing risk conditions to adjust strategy parameters
//!
//! ## Structure
//! - **single**: Functions that return a single value for a slice of prices.
//! - **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
//!
//! ## Included Indicators
//!
//! ## Bulk
//!
//! - [`ulcer_index`](bulk::ulcer_index): Calculates the Ulcer Index
//! - [`volatility_system`](bulk::volatility_system): Implements Welles Wilder's volatility system, including ATR and SAR logic
//!
//! ## Single
//!
//! - [`ulcer_index`](single::ulcer_index): Calculates the Ulcer Index for an entire slice
//!
//! ## API Details
//! - See function-level documentation for arguments, panics, and usage examples.
//!
//! ---

/// **single**: Functions that return a single value for a slice of prices.
pub mod single {
    use crate::basic_indicators::single::max;
    use crate::validation::{assert_non_empty, assert_period, assert_same_len, unsupported_type};

    /// Calculates the Ulcer Index
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Returns
    ///
    /// The calculated indicator value
    ///
    /// # Errors
    ///
    /// Returns `TechnicalIndicatorError::EmptyData` if `prices` is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let ulcer_index = centaur_technical_indicators::volatility_indicators::single::ulcer_index(&prices).unwrap();
    /// assert_eq!(1.9417475728155338, ulcer_index);
    /// ```
    #[inline]
    pub fn ulcer_index(prices: &[f64]) -> crate::Result<f64> {
        assert_non_empty("prices", prices)?;

        let mut sum_sq = 0.0;
        for (i, price) in prices.iter().enumerate().skip(1) {
            let period_max = max(&prices[..=i])?;
            let percentage_drawdown = ((price - period_max) / period_max) * 100.0;
            sum_sq += percentage_drawdown.powi(2);
        }
        Ok((sum_sq / prices.len() as f64).sqrt())
    }
}

/// **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
pub mod bulk {
    use crate::basic_indicators::single::{max, min};
    use crate::chart_trends::overall_trend;
    use crate::other_indicators::bulk::average_true_range;
    use crate::validation::{assert_non_empty, assert_period, assert_same_len, unsupported_type};
    use crate::volatility_indicators::single;
    use crate::{ConstantModelType, Position};

    /// Calculates the Ulcer Index
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `period` - Period over which to calculate the Ulcer index
    ///
    /// # Returns
    ///
    /// A vector of calculated values
    ///
    /// # Errors
    ///
    /// Returns `TechnicalIndicatorError::InvalidPeriod` if `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0];
    /// let period: usize = 5;
    /// let ulcer_index =
    ///     centaur_technical_indicators::volatility_indicators::bulk::ulcer_index(&prices, period).unwrap();
    /// assert_eq!(
    ///     vec![1.9417475728155338, 2.6051277407764535, 2.641062234705911],
    ///     ulcer_index
    /// );
    /// ```
    #[inline]
    pub fn ulcer_index(prices: &[f64], period: usize) -> crate::Result<Vec<f64>> {
        let length = prices.len();
        assert_period(period, length)?;

        let mut ulcer_indexes = Vec::with_capacity(length - period + 1);
        for window in prices.windows(period) {
            ulcer_indexes.push(single::ulcer_index(window)?);
        }
        Ok(ulcer_indexes)
    }

    /// Calculates Welles volatility system
    ///
    /// # Arguments
    ///
    /// * `highs` - Slice of highs
    /// * `lows` - Slice of lows
    /// * `close` - Slice of closing prices
    /// * `period` - Period over which to calculate the volatility system
    /// * `constant_multiplier` - Mulitplier for ATR
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    ///
    /// # Returns
    ///
    /// A vector of calculated values
    ///
    /// # Errors
    ///
    /// Returns `TechnicalIndicatorError::InvalidLength` if `close.len()` != `highs.len()` != `lows.len()`
    /// Returns `TechnicalIndicatorError::EmptyData` if `close.is_empty()`
    /// Returns `TechnicalIndicatorError::InvalidPeriod` if lengths < `period`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let highs = vec![
    ///     4383.33, 4393.57, 4364.2, 4339.54, 4276.56, 4255.84,
    ///     4259.38, 4232.42, 4183.6, 4156.7, 4177.47, 4195.55,
    ///     4245.64, 4319.72, 4373.62, 4372.21, 4386.26, 4391.2,
    ///     4393.4, 4418.03, 4421.76, 4508.67, 4521.17, 4511.99,
    ///     4520.12, 4557.11, 4542.14, 4568.43, 4560.31, 4560.52,
    ///     4568.14
    /// ];
    ///
    /// let lows = vec![
    ///     4342.37, 4337.54, 4303.84, 4269.69, 4223.03, 4189.22,
    ///     4219.43, 4181.42, 4127.9, 4103.78, 4132.94, 4153.12,
    ///     4197.74, 4268.26, 4334.23, 4347.53, 4355.41, 4359.76,
    ///     4343.94, 4353.34, 4393.82, 4458.97, 4495.31, 4487.83,
    ///     4499.66, 4510.36, 4525.51, 4545.05, 4552.8, 4546.32,
    ///     4540.51
    /// ];
    ///
    /// let close = vec![
    ///     4373.63, 4373.2, 4314.6, 4278.0, 4224.16, 4217.04,
    ///     4247.68, 4186.77, 4137.23, 4117.37, 4166.82, 4193.8,
    ///     4237.86, 4317.78, 4358.34, 4365.98, 4378.38, 4382.78,
    ///     4347.35, 4415.24, 4411.55, 4495.7, 4502.88, 4508.24,
    ///     4514.02, 4547.38, 4538.19, 4556.62, 4559.34, 4550.43,
    ///     4554.89
    /// ];
    ///
    /// let period: usize = 5;
    /// let constant_multiplier = 3.0;
    ///
    /// let volatility_system =
    ///     centaur_technical_indicators::volatility_indicators::bulk::volatility_system(
    ///         &highs,
    ///         &lows,
    ///         &close,
    ///         period,
    ///         constant_multiplier,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingAverage
    ///     ).unwrap();
    ///
    /// assert_eq!(
    ///     vec![
    ///         4392.598, 4407.994, 4398.3460000000005, 4392.7300000000005,
    ///         4384.240000000001, 4383.874, 4370.620000000001, 4372.108000000001,
    ///         4370.248000000001, 4367.704000000001, 4359.586000000001, 4234.824,
    ///         4241.771999999999, 4251.648, 4252.848, 4237.668000000001, 4235.712,
    ///         4224.402, 4227.75, 4242.93, 4269.468, 4258.182000000001, 4278.024,
    ///         4279.512, 4289.5019999999995, 4293.258, 4304.73
    ///     ], volatility_system
    /// );
    /// ```
    pub fn volatility_system(
        highs: &[f64],
        lows: &[f64],
        close: &[f64],
        period: usize,
        constant_multiplier: f64,
        constant_model_type: ConstantModelType,
    ) -> crate::Result<Vec<f64>> {
        let length = close.len();
        assert_same_len(&[("close", close), ("highs", highs), ("lows", lows)])?;
        assert_non_empty("close", close)?;
        assert_period(period, length)?;

        let typical_price: Vec<f64> = (0..length)
            .map(|i| (highs[i] + lows[i] + close[i]) / 3.0)
            .collect();

        let mut sars = Vec::with_capacity(length - period + 1);
        let mut position;
        let mut significant_close;
        let mut previous_period = period;

        let trend = overall_trend(&typical_price[..previous_period])?;
        let atr = average_true_range(close, highs, lows, constant_model_type, period)?;
        let arc: Vec<f64> = atr.iter().map(|x| x * constant_multiplier).collect();

        if trend.0 < 0.0 {
            significant_close = min(&close[..previous_period])?;
            position = Position::Short;
            sars.push(significant_close + arc[0]);
            sars.push(significant_close + arc[1]);
        } else {
            significant_close = max(&close[..previous_period])?;
            position = Position::Long;
            sars.push(significant_close - arc[0]);
            sars.push(significant_close - arc[1]);
        };

        for i in 2..arc.len() {
            let max_period = i + period - 1;
            if position == Position::Short {
                if close[max_period] > sars[i - 1] {
                    position = Position::Long;
                    significant_close = max(&close[previous_period..max_period])?;
                    previous_period = max_period;
                    sars.push(significant_close - arc[i]);
                } else {
                    sars.push(significant_close + arc[i]);
                }
            } else if position == Position::Long {
                if close[max_period] < sars[i - 1] {
                    position = Position::Short;
                    significant_close = min(&close[previous_period..max_period])?;
                    previous_period = max_period;
                    sars.push(significant_close + arc[i]);
                } else {
                    sars.push(significant_close - arc[i]);
                }
            } else {
                return Err(unsupported_type("Position"));
            }
        }
        Ok(sars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_ulcer_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(0.21816086938686668, single::ulcer_index(&prices).unwrap());
    }

    #[test]
    fn single_ucler_index_panic() {
        let prices = Vec::new();
        let result = single::ulcer_index(&prices);
        assert!(result.is_err());
    }

    #[test]
    fn bulk_ulcer_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![0.21816086938686668, 0.2373213243162752, 0.12490478596260104],
            bulk::ulcer_index(&prices, 5_usize).unwrap()
        );
    }

    #[test]
    fn bulk_ulcer_index_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        let result = bulk::ulcer_index(&prices, 50_usize);
        assert!(result.is_err());
    }

    #[test]
    fn bulk_volatility_system_long_start() {
        let highs = vec![100.83, 100.91, 101.03, 101.27, 100.52];
        let lows = vec![100.59, 100.72, 100.84, 100.91, 99.85];
        let close = vec![100.76, 100.88, 100.96, 101.14, 100.01];
        let period: usize = 3;
        assert_eq!(
            vec![100.54666666666667, 100.46666666666667, 101.95333333333333],
            bulk::volatility_system(
                &highs,
                &lows,
                &close,
                period,
                2.0,
                crate::ConstantModelType::SimpleMovingAverage
            )
            .unwrap()
        );
    }

    #[test]
    fn bulk_volatility_system_short_start() {
        let highs = vec![101.27, 101.03, 100.91, 100.83, 101.54];
        let lows = vec![100.91, 100.84, 100.72, 100.59, 100.68];
        let close = vec![101.14, 100.96, 100.88, 100.76, 101.37];
        let period: usize = 3;
        assert_eq!(
            vec![101.37333333333332, 101.29333333333332, 99.9],
            bulk::volatility_system(
                &highs,
                &lows,
                &close,
                period,
                2.0,
                crate::ConstantModelType::SimpleMovingAverage
            )
            .unwrap()
        );
    }

    #[test]
    fn bulk_volatility_system_panic_high_length() {
        let highs = vec![101.27, 101.03, 100.83, 101.54];
        let lows = vec![100.91, 100.84, 100.72, 100.59, 100.68];
        let close = vec![101.14, 100.96, 100.88, 100.76, 101.37];
        let period: usize = 3;
        let result = bulk::volatility_system(
            &highs,
            &lows,
            &close,
            period,
            2.0,
            crate::ConstantModelType::SimpleMovingAverage,
        );
        assert!(result.is_err());
    }

    #[test]
    fn bulk_volatility_system_panic_low_length() {
        let highs = vec![101.27, 101.03, 100.91, 100.83, 101.54];
        let lows = vec![100.91, 100.84, 100.72, 100.68];
        let close = vec![101.14, 100.96, 100.88, 100.76, 101.37];
        let period: usize = 3;
        let result = bulk::volatility_system(
            &highs,
            &lows,
            &close,
            period,
            2.0,
            crate::ConstantModelType::SimpleMovingAverage,
        );
        assert!(result.is_err());
    }

    #[test]
    fn bulk_volatility_system_panic_close_length() {
        let highs = vec![101.27, 101.03, 100.91, 100.83, 101.54];
        let lows = vec![100.91, 100.84, 100.72, 100.59, 100.68];
        let close = vec![101.14, 100.96, 100.88, 101.37];
        let period: usize = 3;
        let result = bulk::volatility_system(
            &highs,
            &lows,
            &close,
            period,
            2.0,
            crate::ConstantModelType::SimpleMovingAverage,
        );
        assert!(result.is_err());
    }

    #[test]
    fn bulk_volatility_system_panic_empty() {
        let highs = Vec::new();
        let lows = Vec::new();
        let close = Vec::new();
        let period: usize = 3;
        let result = bulk::volatility_system(
            &highs,
            &lows,
            &close,
            period,
            2.0,
            crate::ConstantModelType::SimpleMovingAverage,
        );
        assert!(result.is_err());
    }

    #[test]
    fn bulk_volatility_system_panic_period() {
        let highs = vec![101.27, 101.03, 100.91, 100.83, 101.54];
        let lows = vec![100.91, 100.84, 100.72, 100.59, 100.68];
        let close = vec![101.14, 100.96, 100.88, 100.76, 101.37];
        let period: usize = 30;
        let result = bulk::volatility_system(
            &highs,
            &lows,
            &close,
            period,
            2.0,
            crate::ConstantModelType::SimpleMovingAverage,
        );
        assert!(result.is_err());
    }
}
