//! # Basic Indicators
//!
//! The `basic_indicators` module provides foundational statistical calculations for time series price data.
//! These are essential building blocks for more advanced technical indicators and can be used directly.
//!
//! ## When to Use
//! Use these functions when you need raw statistics (mean, median, mode, etc.) or want to compose your own indicators.
//!
//! ## Structure
//! - **single**: Functions that return a single value for a slice of prices.
//! - **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
//!
//! ## Included Indicators
//!
//! ### Bulk
//! - [`absolute_deviation`](bulk::absolute_deviation): Mean/Median/Mode absolute deviation over each period
//! - [`log`](bulk::log): Natural logarithm of each price
//! - [`log_difference`](bulk::log_difference): Difference in log(price) at t and t-1
//! - [`mean`](bulk::mean): Average
//! - [`median`](bulk::median): Median
//! - [`mode`](bulk::mode): Mode
//! - [`price_distribution`](bulk::price_distribution): Distribution of prices (count of each unique price) over each period
//! - [`standard_deviation`](bulk::standard_deviation): Standard deviation
//! - [`variance`](bulk::variance): Variance
//!
//! ### Single
//! - [`absolute_deviation`](single::absolute_deviation): Mean/Median/Mode absolute deviation
//! - [`log_difference`](single::log_difference): Log difference between two prices
//! - [`max`](single::max): Maximum price
//! - [`mean`](single::mean): Mean price
//! - [`median`](single::median): Median price
//! - [`min`](single::min): Minimum price
//! - [`mode`](single::mode): Mode price
//! - [`price_distribution`](single::price_distribution): Distribution of prices (count of each unique price)
//! - [`standard_deviation`](single::standard_deviation): Standard deviation
//! - [`variance`](single::variance): Variance
//!
//! ---

/// **single**: Functions that return a single value for a slice of prices
pub mod single {
    use crate::CentralPoint;
    use std::cmp::Ordering;
    use std::collections::HashMap;

    /// Calculates the mean (average) of a slice of prices
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let mean = rust_ti::basic_indicators::single::mean(&prices);
    /// assert_eq!(101.5, mean);
    /// ```
    #[inline]
    pub fn mean(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices ({:?}) is empty", prices);
        };
        prices.iter().sum::<f64>() / prices.len() as f64
    }

    /// Calculates the median (middle value) of a slice of prices.
    ///
    /// Orders numbers and takes the middle value. For even length, takes the average of two middles.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Odd number of prices
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let median = rust_ti::basic_indicators::single::median(&prices);
    /// assert_eq!(101.0, median);
    ///
    /// // Even number of prices
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let median = rust_ti::basic_indicators::single::median(&prices);
    /// assert_eq!(101.5, median);
    /// ```
    #[inline]
    pub fn median(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices ({:?}) is empty", prices);
        };

        let mut values: Vec<f64> = prices.iter().copied().filter(|f| !f.is_nan()).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        let mid = values.len() / 2;

        if values.len() % 2 == 0 {
            (values[mid - 1] + values[mid]) / 2.0
        } else {
            values[mid]
        }
    }

    /// Calculates the mode (most common price) of a slice of prices.
    ///
    /// Rounds prices to the nearest integer for frequency counting.
    /// If multiple modes exist, returns their average.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 101.0, 101.0, 100.0];
    /// let mode = rust_ti::basic_indicators::single::mode(&prices);
    /// assert_eq!(100.5, mode); // 100.0 and 101.0 occur equally often, so average is 100.5
    ///
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let mode = rust_ti::basic_indicators::single::mode(&prices);
    /// assert_eq!(100.0, mode); // 100.0 occurs most often
    /// ```
    #[inline]
    pub fn mode(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices ({:?}) is empty", prices);
        };
        let mut frequency: HashMap<i64, usize> = HashMap::new();
        for &price in prices {
            *frequency.entry(price.round() as i64).or_insert(0) += 1;
        }
        let max_count = frequency.values().copied().max().unwrap();
        let modes: Vec<i64> = frequency
            .iter()
            .filter_map(|(&value, &count)| {
                if count == max_count {
                    Some(value)
                } else {
                    None
                }
            })
            .collect();

        modes.iter().sum::<i64>() as f64 / modes.len() as f64
    }

    /// Calculates the difference between the natural logarithm at t and t-1
    ///
    /// # Arguments
    ///
    /// * `price_t` - price at t
    /// * `price_t_1` - price at t-1
    ///
    /// # Panics
    ///
    /// If `price_t` or `price_t_1` is <= 0.0
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let log_difference = rust_ti::basic_indicators::single::log_difference(prices[3], prices[2]);
    /// assert_eq!(-0.01960847138837618, log_difference);
    /// ```
    #[inline]
    pub fn log_difference(price_t: f64, price_t_1: f64) -> f64 {
        if price_t <= 0.0 || price_t_1 <= 0.0 {
            panic!(
                "price_t ({}) and price_t_1 ({}) need to be greater than 0.0",
                price_t, price_t_1
            );
        }
        price_t.ln() - price_t_1.ln()
    }

    /// Calculates the variance of a slice of prices
    ///
    /// Assumes a normal distribution
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let variance = rust_ti::basic_indicators::single::variance(&prices);
    /// assert_eq!(1.25, variance);
    /// ```
    #[inline]
    pub fn variance(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices ({:?}) is empty", prices);
        }
        let prices_mean = mean(prices);
        let mean_diff_sq: Vec<f64> = prices.iter().map(|x| (x - prices_mean).powi(2)).collect();
        mean(&mean_diff_sq)
    }

    /// Calculates the standard deviation of a slice of prices
    ///
    /// Assumes a normal distribution
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let standard_deviation = rust_ti::basic_indicators::single::standard_deviation(&prices);
    /// assert_eq!(1.118033988749895, standard_deviation);
    /// ```
    #[inline]
    pub fn standard_deviation(prices: &[f64]) -> f64 {
        variance(prices).sqrt()
    }

    /// Calculates the absolute deviation from the mean, median, or mode.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `central_point` - Variant of [`CentralPoint`]
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let mean_absolute_deviation =
    ///     rust_ti::basic_indicators::single::absolute_deviation(&prices, rust_ti::CentralPoint::Mean);
    /// // The answer is `1.04` but `f64` implementation we get `1.0400000000000005`
    /// assert_eq!(1.0400000000000005, mean_absolute_deviation);
    ///
    /// let median_absolute_deviation =
    ///     rust_ti::basic_indicators::single::absolute_deviation(&prices, rust_ti::CentralPoint::Median);
    /// assert_eq!(1.0, median_absolute_deviation);
    ///
    /// let mode_absolute_deviation =
    ///     rust_ti::basic_indicators::single::absolute_deviation(&prices, rust_ti::CentralPoint::Mode);
    /// assert_eq!(1.2, mode_absolute_deviation);
    /// ```
    #[inline]
    pub fn absolute_deviation(prices: &[f64], central_point: CentralPoint) -> f64 {
        if prices.is_empty() {
            panic!("Prices is empty")
        };
        let mid_point = match central_point {
            CentralPoint::Mean => mean(prices),
            CentralPoint::Median => median(prices),
            CentralPoint::Mode => mode(prices),
            _ => panic!("Unsupported central_point {:?}", central_point),
        };
        prices.iter().map(|x| (x - mid_point).abs()).sum::<f64>() / prices.len() as f64
    }

    /// Calculates the maximum of a slice of prices (ignoring NaN)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let max = rust_ti::basic_indicators::single::max(&prices);
    /// assert_eq!(103.0, max);
    /// ```
    #[inline]
    pub fn max(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices is empty")
        };
        prices
            .iter()
            .copied()
            .filter(|f| !f.is_nan())
            .fold(f64::NAN, f64::max)
    }

    /// Calculates the minimum of a slice of prices (ignores NaN)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let min = rust_ti::basic_indicators::single::min(&prices);
    /// assert_eq!(100.0, min);
    /// ```
    #[inline]
    pub fn min(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices is empty")
        };
        prices
            .iter()
            .copied()
            .filter(|f| !f.is_nan())
            .fold(f64::NAN, f64::min)
    }

    /// Calculates the distribution of prices (count of each unique price) in a slice
    ///
    /// Returns a vector of tuples containing (price, count) sorted by price in ascending order.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 100.0, 103.0, 102.0, 100.0];
    /// let distribution = rust_ti::basic_indicators::single::price_distribution(&prices, 1.0);
    /// assert_eq!(vec![(100.0, 3), (102.0, 2), (103.0, 1)], distribution);
    /// ```
    #[inline]
    pub fn price_distribution(prices: &[f64], precision: f64) -> Vec<(f64, usize)> {
        if prices.is_empty() {
            panic!("Prices ({:?}) is empty", prices);
        }
        if !(precision > 0.0) {
            panic!("precision ({}) must be > 0.0 and not NaN", precision);
        }

        let mut frequency: HashMap<i64, usize> = HashMap::new();
        for &price in prices {
            if !price.is_nan() {
                // Use a scaling factor to handle floating point precision
                let key = (price / precision).round() as i64;
                *frequency.entry(key).or_insert(0) += 1;
            }
        }
        
        let mut result: Vec<(f64, usize)> = frequency
            .into_iter()
            .map(|(key, count)| ((key as f64) * precision, count))
            .collect();
        
        // Sort by price in ascending order
        result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
        
        result
    }
}

/// **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
pub mod bulk {
    use crate::basic_indicators::single;
    use crate::CentralPoint;

    /// Calculates the mean (averages) of a slice of prices over a given period
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `period` - Period over which to calculate the mean
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `period` == 0
    ///     * `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![101.0, 102.0, 103.0, 101.0];
    /// let mean = rust_ti::basic_indicators::bulk::mean(&prices, 3);
    /// assert_eq!(vec![102.0, 102.0], mean);
    /// ```
    #[inline]
    pub fn mean(prices: &[f64], period: usize) -> Vec<f64> {
        if period == 0 {
            panic!("Period ({}) must be greater than 0", period);
        }
        if period > prices.len() {
            panic!(
                "Period ({}) cannot be longer than the length of prices ({})",
                period,
                prices.len()
            );
        };
        let mut result = Vec::with_capacity(prices.len());
        for window in prices.windows(period) {
            result.push(single::mean(window))
        }
        result
    }

    /// Calculates the median (middle value) of a slice of prices over a given periods.
    ///
    /// If the number of prices is even it will take the average of the two middle values.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `period` - Period over which to calculate the median
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `period` == 0
    ///     * `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![101.0, 102.0, 103.0, 101.0];
    /// let median = rust_ti::basic_indicators::bulk::median(&prices, 3);
    /// assert_eq!(vec![102.0, 102.0], median);
    /// ```
    #[inline]
    pub fn median(prices: &[f64], period: usize) -> Vec<f64> {
        if period == 0 {
            panic!("Period ({}) must be greater than 0", period);
        };
        if period > prices.len() {
            panic!(
                "Period ({}) cannot be longer than the length of provided prices ({})",
                period,
                prices.len()
            );
        };
        let mut result = Vec::with_capacity(prices.len());
        for window in prices.windows(period) {
            result.push(single::median(window))
        }
        result
    }

    /// Calculates the mode (most common price) of a slice of prices over a given period.
    ///
    /// If multiple modes are found it will the average of those
    /// numbers.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `period` - Period over which to calculate the mode
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `period` == 0
    ///     * `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![101.0, 102.0, 101.0, 102.0];
    /// let mode = rust_ti::basic_indicators::bulk::mode(&prices, 3);
    /// assert_eq!(vec![101.0, 102.0], mode);
    /// ```
    #[inline]
    pub fn mode(prices: &[f64], period: usize) -> Vec<f64> {
        if period == 0 {
            panic!("Period ({}) must be greater than 0", period);
        };
        if period > prices.len() {
            panic!(
                "Period ({}) cannot be longer than the length of provided prices ({})",
                period,
                prices.len()
            );
        };
        let mut result = Vec::with_capacity(prices.len());
        for window in prices.windows(period) {
            result.push(single::mode(window))
        }
        result
    }

    /// Calculates the natural logarithm of slice of prices
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty.()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![101.0, 102.0, 103.0, 101.0];
    /// let log = rust_ti::basic_indicators::bulk::log(&prices);
    /// assert_eq!(
    ///     vec![4.61512051684126, 4.624972813284271, 4.634728988229636, 4.61512051684126],
    ///     log
    /// );
    /// ```
    #[inline]
    pub fn log(prices: &[f64]) -> Vec<f64> {
        if prices.is_empty() {
            panic!("Prices ({:?}) is empty", prices);
        }
        prices.iter().map(|&p| p.ln()).collect()
    }

    /// Calculates the difference between the natural logarithm at t and t-1
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let log_difference = rust_ti::basic_indicators::bulk::log_difference(&prices);
    /// assert_eq!(
    ///     vec![0.019802627296178876, 0.009756174945365181, -0.01960847138837618],
    ///     log_difference
    /// );
    /// ```
    #[inline]
    pub fn log_difference(prices: &[f64]) -> Vec<f64> {
        if prices.is_empty() {
            panic!("Prices ({:?}) is empty", prices);
        }
        prices
            .windows(2)
            .map(|w| single::log_difference(w[1], w[0]))
            .collect()
    }

    /// Calculates the variance of slice of prices over a given period.
    ///
    /// Assumes a normal distribution
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `period` - Period over which to calculate the variance
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `period` == 0
    ///     * `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let period: usize = 3;
    /// let variance = rust_ti::basic_indicators::bulk::variance(&prices, period);
    /// assert_eq!(vec![1.5555555555555556, 0.6666666666666666], variance);
    /// ```
    #[inline]
    pub fn variance(prices: &[f64], period: usize) -> Vec<f64> {
        if period == 0 {
            panic!("Period ({}) must be greater than 0", period)
        };
        if period > prices.len() {
            panic!(
                "Period ({}) cannot be longer than the length of provided prices ({})",
                period,
                prices.len()
            );
        };
        let mut result = Vec::with_capacity(prices.len());
        for window in prices.windows(period) {
            result.push(single::variance(window))
        }
        result
    }

    /// Calculates the standard deviation of a slice of prices over a given period
    ///
    /// Assumes a normal distribution
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `period` - Period over which to calculate the standard deviation
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `period` == 0
    ///     * `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let period: usize = 3;
    /// let standard_deviation =
    ///     rust_ti::basic_indicators::bulk::standard_deviation(&prices, period);
    /// assert_eq!(vec![1.247219128924647, 0.816496580927726], standard_deviation);
    /// ```
    // TODO: Allow for distributions other than normal distributions
    #[inline]
    pub fn standard_deviation(prices: &[f64], period: usize) -> Vec<f64> {
        if period == 0 {
            panic!("Period ({}) must be greater than 0", period)
        };
        if period > prices.len() {
            panic!(
                "Period ({}) cannot be longer than the length of provided prices ({})",
                period,
                prices.len()
            );
        };
        let mut result = Vec::with_capacity(prices.len());
        for window in prices.windows(period) {
            result.push(single::standard_deviation(window));
        }
        result
    }

    /// Calculates the absolute deviation from the mean, median, or mode over a given period.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `period` - Period over which to calculate the standard deviation
    /// * `central_point` - Variant of [`CentralPoint`]
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `period` == 0
    ///     * `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let period: usize = 3;
    ///
    /// let mean_absolute_deviation =
    ///     rust_ti::basic_indicators::bulk::absolute_deviation(
    ///         &prices,
    ///         period,
    ///         rust_ti::CentralPoint::Mean
    ///     );
    /// assert_eq!(
    ///     vec![1.1111111111111096, 0.6666666666666666, 1.1111111111111096],
    ///     mean_absolute_deviation
    /// );
    ///
    /// let median_absolute_deviation =
    ///     rust_ti::basic_indicators::bulk::absolute_deviation(
    ///         &prices,
    ///         period,
    ///         rust_ti::CentralPoint::Median
    ///     );
    /// assert_eq!(vec![1.0, 0.6666666666666666, 1.0], median_absolute_deviation);
    ///
    /// let mode_absolute_deviation =
    ///     rust_ti::basic_indicators::bulk::absolute_deviation(
    ///         &prices,
    ///         period,
    ///         rust_ti::CentralPoint::Mode
    ///     );
    /// assert_eq!(
    ///     vec![1.1111111111111096, 0.6666666666666666, 1.1111111111111096],
    ///     mode_absolute_deviation
    /// );
    /// ```
    #[inline]
    pub fn absolute_deviation(
        prices: &[f64],
        period: usize,
        central_point: CentralPoint,
    ) -> Vec<f64> {
        if period == 0 {
            panic!("Period ({}) must be greater than 0", period)
        };
        if period > prices.len() {
            panic!(
                "Period ({}) cannot be longer than the length of provided prices ({})",
                period,
                prices.len()
            );
        };
        prices
            .windows(period)
            .map(|w| single::absolute_deviation(w, central_point))
            .collect()
    }

    /// Calculates the distribution of prices (count of each unique price) over a given period
    ///
    /// For each sliding window of the specified period, returns a vector of tuples containing
    /// (price, count) sorted by price in ascending order.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `period` - Period over which to calculate the price distribution
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `period` == 0
    ///     * `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 100.0, 103.0, 102.0];
    /// let distribution = rust_ti::basic_indicators::bulk::price_distribution(&prices, 3, 1.0);
    /// assert_eq!(
    ///     vec![
    ///         vec![(100.0, 2), (102.0, 1)],
    ///         vec![(100.0, 1), (102.0, 1), (103.0, 1)],
    ///         vec![(100.0, 1), (102.0, 1), (103.0, 1)]
    ///     ],
    ///     distribution
    /// );
    /// ```
    #[inline]
    pub fn price_distribution(prices: &[f64], period: usize, precision: f64) -> Vec<Vec<(f64, usize)>> {
        if period == 0 {
            panic!("Period ({}) must be greater than 0", period);
        }
        if period > prices.len() {
            panic!(
                "Period ({}) cannot be longer than the length of provided prices ({})",
                period,
                prices.len()
            );
        }
        
        prices
            .windows(period)
            .map(|w| single::price_distribution(w, precision))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_mean() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(100.352, single::mean(&prices));
    }

    #[test]
    fn single_mean_identical_prices() {
        let prices = vec![100.0, 100.0, 100.0];
        assert_eq!(100.0, single::mean(&prices));
    }

    #[test]
    #[should_panic]
    fn single_mean_empty_prices() {
        let prices = Vec::new();
        single::mean(&prices);
    }

    #[test]
    fn bulk_mean() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 3;
        assert_eq!(
            vec![100.39666666666666, 100.45666666666666, 100.36666666666667],
            bulk::mean(&prices, period)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_mean_long_period_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 30;
        bulk::mean(&prices, period);
    }

    #[test]
    #[should_panic]
    fn bulk_mean_no_period_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 0;
        bulk::mean(&prices, period);
    }

    #[test]
    fn single_median_odd() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(100.38, single::median(&prices));
    }

    #[test]
    fn single_median_even() {
        let prices = vec![100.2, 100.46, 100.53, 100.38];
        // Should be
        // assert_eq!(100.42, single::median(&prices));
        // but due to how floating points are calculated we have to assert on
        assert_eq!(100.41999999999999, single::median(&prices));
    }

    #[test]
    #[should_panic]
    fn single_median_panic() {
        let prices = Vec::new();
        single::median(&prices);
    }

    #[test]
    fn bulk_median() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 3;
        assert_eq!(vec![100.46, 100.46, 100.38], bulk::median(&prices, period));
    }

    #[test]
    #[should_panic]
    fn bulk_median_long_period_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 30;
        bulk::median(&prices, period);
    }

    #[test]
    #[should_panic]
    fn bulk_median_no_period_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 0;
        bulk::median(&prices, period);
    }

    #[test]
    fn single_mode_round_up() {
        let prices = vec![100.2, 100.46, 100.53, 101.08, 101.19];
        assert_eq!(101.0, single::mode(&prices));
    }

    #[test]
    fn single_mode_round_down() {
        let prices = vec![100.2, 100.46, 100.35, 101.08, 101.19];
        assert_eq!(100.0, single::mode(&prices));
    }

    #[test]
    fn single_mode_average() {
        let prices = vec![100.46, 100.35, 101.08, 101.19];
        assert_eq!(100.5, single::mode(&prices));
    }

    #[test]
    #[should_panic]
    fn single_mode_panic() {
        let prices = Vec::new();
        single::mode(&prices);
    }

    #[test]
    fn bulk_mode() {
        let prices = vec![100.2, 100.46, 100.53, 101.08, 101.19];
        let period: usize = 3;
        assert_eq!(vec![100.0, 101.0, 101.0], bulk::mode(&prices, period));
    }

    #[test]
    #[should_panic]
    fn bulk_mode_long_period_panic() {
        let prices = vec![100.2, 100.46, 100.53, 101.08, 101.19];
        let period: usize = 30;
        bulk::mode(&prices, period);
    }

    #[test]
    #[should_panic]
    fn bulk_mode_no_period_panic() {
        let prices = vec![100.2, 100.46, 100.53, 101.08, 101.19];
        let period: usize = 0;
        bulk::mode(&prices, period);
    }

    #[test]
    fn bulk_log() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            vec![
                4.607168188650764,
                4.609759638321899,
                4.610456190417329,
                4.608962984226787,
                4.607068383271171
            ],
            bulk::log(&prices)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_log_panic() {
        let prices = Vec::new();
        bulk::log(&prices);
    }

    #[test]
    fn single_log_difference() {
        assert_eq!(
            -0.0018946009556159993,
            single::log_difference(100.19, 100.38)
        );
    }

    #[test]
    #[should_panic]
    fn single_log_difference_panic() {
        single::log_difference(0.0, 100.38);
    }

    #[test]
    #[should_panic]
    fn single_log_difference_panic_2() {
        single::log_difference(100.19, -100.38);
    }

    #[test]
    fn bulk_log_difference() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            vec![
                0.0025914496711347823,
                0.0006965520954302917,
                -0.0014932061905419403,
                -0.0018946009556159993
            ],
            bulk::log_difference(&prices)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_log_difference_difference() {
        bulk::log_difference(&Vec::new());
    }

    #[test]
    fn single_variance() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(0.018695999999999734, single::variance(&prices));
    }

    #[test]
    #[should_panic]
    fn single_variance_panic() {
        let prices = Vec::new();
        single::variance(&prices);
    }

    #[test]
    fn bulk_variance() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period = 3;
        assert_eq!(
            vec![
                0.02015555555555502,
                0.0037555555555558295,
                0.019355555555555907
            ],
            bulk::variance(&prices, period)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_variance_long_period_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period = 30;
        bulk::variance(&prices, period);
    }

    #[test]
    #[should_panic]
    fn bulk_variance_no_period_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period = 0;
        bulk::variance(&prices, period);
    }

    #[test]
    fn single_standard_deviation() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(0.1367333170810967, single::standard_deviation(&prices));
    }

    #[test]
    #[should_panic]
    fn single_standard_deviation_panic() {
        let prices = Vec::new();
        single::standard_deviation(&prices);
    }

    #[test]
    fn bulk_standard_deviation() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period = 3;
        assert_eq!(
            vec![
                0.14197026292697715,
                0.06128258770283635,
                0.13912424503139598
            ],
            bulk::standard_deviation(&prices, period)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_standard_deviation_long_period_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period = 30;
        bulk::standard_deviation(&prices, period);
    }

    #[test]
    #[should_panic]
    fn bulk_standard_deviation_no_period_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period = 0;
        bulk::standard_deviation(&prices, period);
    }

    #[test]
    fn single_absolute_deviation() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            0.12559999999999719,
            single::absolute_deviation(&prices, crate::CentralPoint::Mean)
        );
        assert_eq!(
            0.11999999999999886,
            single::absolute_deviation(&prices, crate::CentralPoint::Median)
        );
        assert_eq!(
            0.3519999999999982,
            single::absolute_deviation(&prices, crate::CentralPoint::Mode)
        );
    }

    #[test]
    #[should_panic]
    fn singe_absolute_deviation_panic() {
        let prices = Vec::new();
        single::absolute_deviation(&prices, crate::CentralPoint::Mean);
    }

    #[test]
    fn bulk_absolute_deviation() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 3;

        assert_eq!(
            vec![
                0.1311111111111103,
                0.051111111111111995,
                0.11777777777777487
            ],
            bulk::absolute_deviation(&prices, period, crate::CentralPoint::Mean)
        );
        assert_eq!(
            vec![0.10999999999999943, 0.0500000000000019, 0.11333333333333447],
            bulk::absolute_deviation(&prices, period, crate::CentralPoint::Median)
        );
        assert_eq!(
            vec![0.3966666666666659, 0.45666666666666345, 0.36666666666666475],
            bulk::absolute_deviation(&prices, period, crate::CentralPoint::Mode)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_absolute_deviation_long_period_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 30;
        bulk::absolute_deviation(&prices, period, crate::CentralPoint::Mean);
    }

    #[test]
    #[should_panic]
    fn bulk_absolute_deviation_no_period_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 30;
        bulk::absolute_deviation(&prices, period, crate::CentralPoint::Mean);
    }

    #[test]
    fn test_single_max() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(100.53, single::max(&prices));
    }

    #[test]
    #[should_panic]
    fn test_single_max_panic() {
        let prices = Vec::new();
        single::max(&prices);
    }

    #[test]
    fn test_single_min() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(100.19, single::min(&prices));
    }

    #[test]
    #[should_panic]
    fn test_single_min_panic() {
        let prices = Vec::new();
        single::min(&prices);
    }

    #[test]
    fn test_single_price_distribution() {
        let prices = vec![100.0, 102.0, 100.0, 103.0, 102.0, 100.0];
        let distribution = single::price_distribution(&prices, 1.0);
        assert_eq!(vec![(100.0, 3), (102.0, 2), (103.0, 1)], distribution);
    }

    #[test]
    fn test_single_price_distribution_unique() {
        let prices = vec![100.0, 101.0, 102.0, 103.0];
        let distribution = single::price_distribution(&prices,1.0);
        assert_eq!(vec![(100.0, 1), (101.0, 1), (102.0, 1), (103.0, 1)], distribution);
    }

    #[test]
    fn test_single_price_distribution_same() {
        let prices = vec![100.0, 100.0, 100.0];
        let distribution = single::price_distribution(&prices, 1.0);
        assert_eq!(vec![(100.0, 3)], distribution);
    }

    #[test]
    #[should_panic]
    fn test_single_price_distribution_panic() {
        let prices = Vec::new();
        single::price_distribution(&prices, 1.0);
    }

    #[test]
    #[should_panic]
    fn test_single_price_distribution_bad_precision() {
        single::price_distribution(&[1.0], 0.0);
    }

    #[test]
    fn test_single_price_distribution_precision_examples() {
        let prices = vec![5949.41];
        assert_eq!(
            vec![(6000.0, 1)],
            single::price_distribution(&prices, 1000.0)
        );
        assert_eq!(
            vec![(5900.0, 1)],
            single::price_distribution(&prices, 100.0)
        );
        assert_eq!(vec![(5950.0, 1)], single::price_distribution(&prices, 10.0));
        assert_eq!(vec![(5949.0, 1)], single::price_distribution(&prices, 1.0));
        assert_eq!(vec![(5949.400000000001, 1)], single::price_distribution(&prices, 0.1));
    }

    #[test]
    fn test_single_price_distribution_half_precision() {
        let prices = vec![100.2, 100.46, 100.53, 101.08, 101.19];
        // precision 1.0
        assert_eq!(
            vec![(100.0, 2), (101.0, 3)],
            single::price_distribution(&prices, 1.0)
        );
        // precision 0.5
        assert_eq!(
            vec![(100.0, 1), (100.5, 2), (101.0, 2)],
            single::price_distribution(&prices, 0.5)
        );
    }

    #[test]
    fn test_single_price_distribution_nan_ignored() {
        let prices = vec![100.0, f64::NAN, 100.4, 100.49, 100.51];
        // precision 0.5 -> 100.0, 100.5 buckets
        assert_eq!(
            vec![(100.0, 1), (100.5, 3)],
            single::price_distribution(&prices, 0.5)
        );
    }

    #[test]
    fn test_bulk_price_distribution() {
        let prices = vec![100.0, 102.0, 100.0, 103.0, 102.0];
        let distribution = bulk::price_distribution(&prices, 3, 1.0);
        assert_eq!(
            vec![
                vec![(100.0, 2), (102.0, 1)],
                vec![(100.0, 1), (102.0, 1), (103.0, 1)],
                vec![(100.0, 1), (102.0, 1), (103.0, 1)]
            ],
            distribution
        );
    }

    #[test]
    fn test_bulk_price_distribution_half_precision() {
        let prices = vec![100.2, 100.46, 100.53, 101.08];
        // period 3, precision 0.5
        // windows:
        // [100.2,100.46,100.53] -> 100.0 (1), 100.5 (2)
        // [100.46,100.53,101.08] -> 100.5 (2), 101.0 (1)
        let distribution = bulk::price_distribution(&prices, 3, 0.5);
        assert_eq!(
            vec![
                vec![(100.0, 1), (100.5, 2)],
                vec![(100.5, 2), (101.0, 1)],
            ],
            distribution
        );
    }

    #[test]
    #[should_panic]
    fn test_bulk_price_distribution_period_too_long() {
        let prices = vec![100.0, 102.0, 100.0];
        bulk::price_distribution(&prices, 5, 1.0);
    }

    #[test]
    #[should_panic]
    fn test_bulk_price_distribution_zero_period() {
        let prices = vec![100.0, 102.0, 100.0];
        bulk::price_distribution(&prices, 0, 1.0);
    }

    #[test]
    #[should_panic]
    fn test_bulk_price_distribution_bad_precision() {
        let prices = vec![100.0, 101.0, 102.0];
        bulk::price_distribution(&prices, 2, -1.0);
    }
}
