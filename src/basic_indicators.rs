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
//! - [`cauchy_iqr_scale`](bulk::cauchy_iqr_scale): Cauchy IQR-based scale parameter over each period
//! - [`laplace_std_equivalent`](bulk::laplace_std_equivalent): Laplace standard deviation equivalent over each period
//! - [`log`](bulk::log): Natural logarithm of each price
//! - [`log_difference`](bulk::log_difference): Difference in log(price) at t and t-1
//! - [`log_standard_deviation`](bulk::log_standard_deviation): Log standard deviation over each period
//! - [`mean`](bulk::mean): Average
//! - [`median`](bulk::median): Median
//! - [`mode`](bulk::mode): Mode
//! - [`price_distribution`](bulk::price_distribution): Distribution of prices (count of each unique price) over each period
//! - [`standard_deviation`](bulk::standard_deviation): Standard deviation
//! - [`student_t_adjusted_std`](bulk::student_t_adjusted_std): Student's t-adjusted standard deviation over each period
//! - [`variance`](bulk::variance): Variance
//!
//! ### Single
//! - [`absolute_deviation`](single::absolute_deviation): Mean/Median/Mode absolute deviation
//! - [`cauchy_iqr_scale`](single::cauchy_iqr_scale): Cauchy IQR-based scale parameter
//! - [`laplace_std_equivalent`](single::laplace_std_equivalent): Laplace standard deviation equivalent
//! - [`log_difference`](single::log_difference): Log difference between two prices
//! - [`log_standard_deviation`](single::log_standard_deviation): Log standard deviation
//! - [`max`](single::max): Maximum price
//! - [`mean`](single::mean): Mean price
//! - [`median`](single::median): Median price
//! - [`min`](single::min): Minimum price
//! - [`mode`](single::mode): Mode price
//! - [`price_distribution`](single::price_distribution): Distribution of prices (count of each unique price)
//! - [`standard_deviation`](single::standard_deviation): Standard deviation
//! - [`student_t_adjusted_std`](single::student_t_adjusted_std): Student's t-adjusted standard deviation
//! - [`variance`](single::variance): Variance
//!
//! ---

/// **single**: Functions that return a single value for a slice of prices
pub mod single {
    use crate::validation::{
        assert_min_value, assert_non_empty, assert_positive,
        unsupported_type,
    };
    use crate::{AbsDevConfig, CentralPoint, DeviationAggregate};
    use std::cmp::Ordering;
    use std::collections::HashMap;

    /// Calculates the mean (average) of a slice of prices
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Returns
    ///
    /// The mean (average) value of the prices
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let mean = centaur_technical_indicators::basic_indicators::single::mean(&prices);
    /// assert_eq!(101.5, mean);
    /// ```
    #[inline]
    pub fn mean(prices: &[f64]) -> f64 {
        assert_non_empty("prices", prices);
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
    /// # Returns
    ///
    /// The median value of the prices
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
    /// let median = centaur_technical_indicators::basic_indicators::single::median(&prices);
    /// assert_eq!(101.0, median);
    ///
    /// // Even number of prices
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let median = centaur_technical_indicators::basic_indicators::single::median(&prices);
    /// assert_eq!(101.5, median);
    /// ```
    #[inline]
    pub fn median(prices: &[f64]) -> f64 {
        assert_non_empty("prices", prices);

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
    /// # Returns
    ///
    /// The mode (most common value) of the prices
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 101.0, 101.0, 100.0];
    /// let mode = centaur_technical_indicators::basic_indicators::single::mode(&prices);
    /// assert_eq!(100.5, mode); // 100.0 and 101.0 occur equally often, so average is 100.5
    ///
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let mode = centaur_technical_indicators::basic_indicators::single::mode(&prices);
    /// assert_eq!(100.0, mode); // 100.0 occurs most often
    /// ```
    #[inline]
    pub fn mode(prices: &[f64]) -> f64 {
        assert_non_empty("prices", prices);
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
    /// # Returns
    ///
    /// The logarithmic difference between the two prices
    ///
    /// # Panics
    ///
    /// If `price_t` or `price_t_1` is <= 0.0
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let log_difference = centaur_technical_indicators::basic_indicators::single::log_difference(prices[3], prices[2]);
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
    /// # Returns
    ///
    /// The variance of the prices
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let variance = centaur_technical_indicators::basic_indicators::single::variance(&prices);
    /// assert_eq!(1.25, variance);
    /// ```
    #[inline]
    pub fn variance(prices: &[f64]) -> f64 {
        assert_non_empty("prices", prices);
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
    /// # Returns
    ///
    /// The standard deviation of the prices
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let standard_deviation = centaur_technical_indicators::basic_indicators::single::standard_deviation(&prices);
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
    /// # Returns
    ///
    /// The absolute deviation value based on the specified configuration
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
    ///     centaur_technical_indicators::basic_indicators::single::absolute_deviation(
    ///         &prices,
    ///         centaur_technical_indicators::AbsDevConfig{ center: centaur_technical_indicators::CentralPoint::Mean, aggregate: centaur_technical_indicators::DeviationAggregate::Mean }
    ///     );
    /// // The answer is `1.04` but `f64` implementation we get `1.0400000000000005`
    /// assert_eq!(1.0400000000000005, mean_absolute_deviation);
    ///
    /// let median_absolute_deviation =
    ///     centaur_technical_indicators::basic_indicators::single::absolute_deviation(
    ///         &prices,
    ///         centaur_technical_indicators::AbsDevConfig{ center: centaur_technical_indicators::CentralPoint::Median, aggregate: centaur_technical_indicators::DeviationAggregate::Median }
    ///    );
    /// assert_eq!(1.0, median_absolute_deviation);
    ///
    /// let mode_absolute_deviation =
    ///     centaur_technical_indicators::basic_indicators::single::absolute_deviation(
    ///         &prices,
    ///         centaur_technical_indicators::AbsDevConfig{ center: centaur_technical_indicators::CentralPoint::Mode, aggregate: centaur_technical_indicators::DeviationAggregate::Mode }
    ///   );
    /// assert_eq!(0.0, mode_absolute_deviation);
    /// ```
    #[inline]
    pub fn absolute_deviation(prices: &[f64], config: AbsDevConfig) -> f64 {
        assert_non_empty("prices", prices);
        let mid_point = match config.center {
            CentralPoint::Mean => mean(prices),
            CentralPoint::Median => median(prices),
            CentralPoint::Mode => mode(prices),
            _ => unsupported_type("CentralPoint"),
        };

        let devs: Vec<f64> = prices.iter().map(|&x| (x - mid_point).abs()).collect();

        match config.aggregate {
            DeviationAggregate::Mean => mean(&devs),
            DeviationAggregate::Median => median(&devs),
            DeviationAggregate::Mode => mode(&devs),
        }
    }

    /// Calculates the log standard deviation of a slice of prices.
    ///
    /// Computes the standard deviation of log-transformed prices.
    /// Useful for analyzing multiplicative/percentage-based volatility.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices (must all be positive)
    ///
    /// # Returns
    ///
    /// The standard deviation of the log-transformed prices
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `prices.is_empty()`
    ///     * Any price is <= 0
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::f64::consts::E;
    /// let prices = vec![1.0, E, E.powi(2)];
    /// let log_std = centaur_technical_indicators::basic_indicators::single::log_standard_deviation(&prices);
    /// assert!(log_std > 0.0);
    /// ```
    #[inline]
    pub fn log_standard_deviation(prices: &[f64]) -> f64 {
        assert_non_empty("prices", prices);
        let mut logs = Vec::with_capacity(prices.len());
        for &x in prices {
            if x <= 0.0 {
                panic!("prices requires all positive values; found {}", x);
            }
            logs.push(x.ln());
        }
        standard_deviation(&logs)
    }

    /// Calculates the Student's t-adjusted standard deviation.
    ///
    /// Adjusts the sample standard deviation by the factor sqrt(df/(df-2))
    /// to match the standard deviation of a Student's t-distribution.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `df` - Degrees of freedom (must be > 2)
    ///
    /// # Returns
    ///
    /// The Student's t-adjusted standard deviation
    ///
    /// # Panics
    ///
    /// Panics if `df` <= 2.0
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![1.0, 2.0, 3.0];
    /// let student_std = centaur_technical_indicators::basic_indicators::single::student_t_adjusted_std(&prices, 5.0);
    /// assert!(student_std > 0.0);
    /// ```
    #[inline]
    pub fn student_t_adjusted_std(prices: &[f64], df: f64) -> f64 {
        assert_min_value("degrees_of_freedom", df, 2.0);
        let s = standard_deviation(prices);
        s * (df / (df - 2.0)).sqrt()
    }

    /// Calculates the Laplace standard deviation equivalent.
    ///
    /// Estimates the scale parameter of a Laplace distribution as sqrt(2) * MAD,
    /// where MAD is the median absolute deviation from the median.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Returns
    ///
    /// The Laplace standard deviation equivalent
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![0.0, 1.0, 2.0, 3.0, 4.0];
    /// let laplace_std = centaur_technical_indicators::basic_indicators::single::laplace_std_equivalent(&prices);
    /// assert!(laplace_std > 0.0);
    /// ```
    #[inline]
    pub fn laplace_std_equivalent(prices: &[f64]) -> f64 {
        // b_hat = MAD about median; σ_laplace = sqrt(2) * b
        let mad = absolute_deviation(
            prices,
            AbsDevConfig {
                center: CentralPoint::Median,
                aggregate: DeviationAggregate::Median,
            },
        );
        mad * 2.0f64.sqrt()
    }

    /// Calculates the Cauchy IQR-based scale parameter.
    ///
    /// Estimates the scale parameter (gamma) of a Cauchy distribution as (Q3 - Q1) / 2,
    /// where Q1 and Q3 are the first and third quartiles.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices (must have at least 4 values)
    ///
    /// # Returns
    ///
    /// The Cauchy IQR-based scale parameter
    ///
    /// # Panics
    ///
    /// Panics if `prices.len()` < 4
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![1.0, 2.0, 3.0, 4.0];
    /// let cauchy_scale = centaur_technical_indicators::basic_indicators::single::cauchy_iqr_scale(&prices);
    /// assert!(cauchy_scale > 0.0);
    /// ```
    #[inline]
    pub fn cauchy_iqr_scale(prices: &[f64]) -> f64 {
        if prices.len() < 4 {
            panic!(
                "prices must be at least 4 in length; received {}",
                prices.len()
            );
        }
        // Compute Q1, Q3 via sorted slice and Tukey hinges (simple, fast)
        let mut v = prices.to_vec();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = v.len();
        let mid = n / 2;
        let (lower, upper) = if n % 2 == 0 {
            (&v[..mid], &v[mid..])
        } else {
            (&v[..mid], &v[mid + 1..])
        };
        let q1 = percentile50(lower); // median of lower half
        let q3 = percentile50(upper); // median of upper half
        (q3 - q1) / 2.0
    }

    #[inline]
    fn percentile50(slice: &[f64]) -> f64 {
        let m = slice.len();
        if m == 0 {
            return f64::NAN;
        }
        if m % 2 == 1 {
            slice[m / 2]
        } else {
            0.5 * (slice[m / 2 - 1] + slice[m / 2])
        }
    }

    /// Calculates the maximum of a slice of prices (ignoring NaN)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Returns
    ///
    /// The maximum value in the prices slice
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let max = centaur_technical_indicators::basic_indicators::single::max(&prices);
    /// assert_eq!(103.0, max);
    /// ```
    #[inline]
    pub fn max(prices: &[f64]) -> f64 {
        assert_non_empty("prices", prices);
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
    /// # Returns
    ///
    /// The minimum value in the prices slice
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let min = centaur_technical_indicators::basic_indicators::single::min(&prices);
    /// assert_eq!(100.0, min);
    /// ```
    #[inline]
    pub fn min(prices: &[f64]) -> f64 {
        assert_non_empty("prices", prices);
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
    /// * `precision` - Precision to group prices (e.g., 1.0 for whole numbers, 0.1 for one decimal place)
    ///
    /// # Returns
    ///
    /// A vector of tuples containing (value, index)
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 100.0, 103.0, 102.0, 100.0];
    /// let distribution = centaur_technical_indicators::basic_indicators::single::price_distribution(&prices, 1.0);
    /// assert_eq!(vec![(100.0, 3), (102.0, 2), (103.0, 1)], distribution);
    /// ```
    #[inline]
    pub fn price_distribution(prices: &[f64], precision: f64) -> Vec<(f64, usize)> {
        assert_non_empty("prices", prices);
        assert_positive("precision", precision);

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

    #[inline]
    fn empirical_quantile_from_distribution(prices: &[f64], precision: f64, q: f64) -> f64 {
        if !(q > 0.0 && q < 1.0) {
            panic!("quantile ({}) must be in range (0, 1)", q);
        }
        let hist = price_distribution(prices, precision);
        let n: usize = hist.iter().map(|(_, c)| *c).sum();
        if n == 0 {
            return f64::NAN;
        }
        // Rank using (n - 1) interpolation baseline
        let target = q * (n.saturating_sub(1)) as f64;

        // Walk cumulative counts
        let mut cum = 0usize;
        for (i, (price, count)) in hist.iter().enumerate() {
            let prev_cum = cum;
            cum += *count;

            if (target as usize) < cum {
                // Inside this bucket. Interpolate toward the next bucket center if any.
                let within = if *count > 1 {
                    // Fraction within this bucket: distance from prev_cum to target
                    (target - prev_cum as f64) / (*count as f64)
                } else {
                    0.0
                };
                if i + 1 < hist.len() {
                    let (next_price, _) = hist[i + 1];
                    return price + within.clamp(0.0, 1.0) * (next_price - price);
                } else {
                    return *price;
                }
            }
        }
        // Fallback (shouldn’t happen): return last price
        hist.last().map(|(p, _)| *p).unwrap_or(f64::NAN)
    }

    /// Computes an empirical quantile from the histogram produced by `price_distribution`,
    /// using linear interpolation across adjacent buckets.
    ///
    /// The histogram is constructed by bucketing values to the provided `precision`. For example,
    /// `precision = 1.0` groups by whole numbers; `precision = 0.01` groups by cents.
    ///
    /// Quantile definition:
    /// - Uses target rank `q * (n - 1)` where `n` is the total count in the histogram.
    /// - Walks cumulative counts until the bucket containing the rank is found.
    /// - Interpolates linearly toward the next bucket center by the within-bucket fraction.
    ///   If no next bucket exists (last bucket), returns the current bucket center.
    ///
    /// # Returns
    ///
    /// The quantile range (difference between high and low quantiles)
    ///
    /// Panics:
    /// - If `q` is not in (0, 1).
    /// - If `precision <= 0.0` or `precision` is NaN (via `price_distribution`).
    ///
    /// Examples
    /// ```
    /// let prices = vec![1.0, 2.0, 3.0, 4.0];
    /// let q25 = centaur_technical_indicators::basic_indicators::single::empirical_quantile_range_from_distribution(&prices, 1.0, 0.25, 0.75);
    /// assert_eq!(2.0, q25);
    /// ```
    #[inline]
    pub fn empirical_quantile_range_from_distribution(
        prices: &[f64],
        precision: f64,
        low: f64,
        high: f64,
    ) -> f64 {
        assert_positive("precision", precision);
        if !(low > 0.0 && low < 1.0 && high > 0.0 && high < 1.0 && low < high) {
            panic!(
                "Invalid quantile bounds: low ({}) and high ({}) must be in (0,1) and low < high",
                low, high
            );
        }
        let ql = empirical_quantile_from_distribution(prices, precision, low);
        let qh = empirical_quantile_from_distribution(prices, precision, high);
        qh - ql
    }
}

/// **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
pub mod bulk {
    use crate::basic_indicators::single;
    use crate::validation::{assert_non_empty, assert_period};
    use crate::AbsDevConfig;

    /// Calculates the mean (averages) of a slice of prices over a given period
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `period` - Period over which to calculate the mean
    ///
    /// # Returns
    ///
    /// A vector of calculated values
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
    /// let mean = centaur_technical_indicators::basic_indicators::bulk::mean(&prices, 3);
    /// assert_eq!(vec![102.0, 102.0], mean);
    /// ```
    #[inline]
    pub fn mean(prices: &[f64], period: usize) -> Vec<f64> {
        assert_period(period, prices.len());
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
    /// # Returns
    ///
    /// A vector of calculated values
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
    /// let median = centaur_technical_indicators::basic_indicators::bulk::median(&prices, 3);
    /// assert_eq!(vec![102.0, 102.0], median);
    /// ```
    #[inline]
    pub fn median(prices: &[f64], period: usize) -> Vec<f64> {
        assert_period(period, prices.len());
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
    /// # Returns
    ///
    /// A vector of calculated values
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
    /// let mode = centaur_technical_indicators::basic_indicators::bulk::mode(&prices, 3);
    /// assert_eq!(vec![101.0, 102.0], mode);
    /// ```
    #[inline]
    pub fn mode(prices: &[f64], period: usize) -> Vec<f64> {
        assert_period(period, prices.len());
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
    /// # Returns
    ///
    /// A vector of calculated values
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty.()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![101.0, 102.0, 103.0, 101.0];
    /// let log = centaur_technical_indicators::basic_indicators::bulk::log(&prices);
    /// assert_eq!(
    ///     vec![4.61512051684126, 4.624972813284271, 4.634728988229636, 4.61512051684126],
    ///     log
    /// );
    /// ```
    #[inline]
    pub fn log(prices: &[f64]) -> Vec<f64> {
        assert_non_empty("prices", prices);
        prices.iter().map(|&p| p.ln()).collect()
    }

    /// Calculates the difference between the natural logarithm at t and t-1
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    ///
    /// # Returns
    ///
    /// A vector of calculated values
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0];
    /// let log_difference = centaur_technical_indicators::basic_indicators::bulk::log_difference(&prices);
    /// assert_eq!(
    ///     vec![0.019802627296178876, 0.009756174945365181, -0.01960847138837618],
    ///     log_difference
    /// );
    /// ```
    #[inline]
    pub fn log_difference(prices: &[f64]) -> Vec<f64> {
        assert_non_empty("prices", prices);
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
    /// # Returns
    ///
    /// A vector of calculated values
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
    /// let variance = centaur_technical_indicators::basic_indicators::bulk::variance(&prices, period);
    /// assert_eq!(vec![1.5555555555555556, 0.6666666666666666], variance);
    /// ```
    #[inline]
    pub fn variance(prices: &[f64], period: usize) -> Vec<f64> {
        assert_period(period, prices.len());
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
    /// # Returns
    ///
    /// A vector of calculated values
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
    ///     centaur_technical_indicators::basic_indicators::bulk::standard_deviation(&prices, period);
    /// assert_eq!(vec![1.247219128924647, 0.816496580927726], standard_deviation);
    /// ```
    #[inline]
    pub fn standard_deviation(prices: &[f64], period: usize) -> Vec<f64> {
        assert_period(period, prices.len());
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
    /// # Returns
    ///
    /// A vector of calculated values
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
    /// use centaur_technical_indicators::{CentralPoint, DeviationAggregate};
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
    /// let period: usize = 3;
    ///
    /// let mean_absolute_deviation =
    ///     centaur_technical_indicators::basic_indicators::bulk::absolute_deviation(
    ///         &prices,
    ///         period,
    ///         centaur_technical_indicators::AbsDevConfig{ center: CentralPoint::Mean, aggregate: DeviationAggregate::Mean }
    ///     );
    /// assert_eq!(
    ///     vec![1.1111111111111096, 0.6666666666666666, 1.1111111111111096],
    ///     mean_absolute_deviation
    /// );
    ///
    /// let median_absolute_deviation =
    ///     centaur_technical_indicators::basic_indicators::bulk::absolute_deviation(
    ///         &prices,
    ///         period,
    ///         centaur_technical_indicators::AbsDevConfig{ center: CentralPoint::Median, aggregate: DeviationAggregate::Median }
    ///     );
    /// assert_eq!(vec![1.0, 1.0, 1.0], median_absolute_deviation);
    ///
    /// let mode_absolute_deviation =
    ///     centaur_technical_indicators::basic_indicators::bulk::absolute_deviation(
    ///         &prices,
    ///         period,
    ///         centaur_technical_indicators::AbsDevConfig{ center: CentralPoint::Mode, aggregate: DeviationAggregate::Mode }
    ///     );
    /// assert_eq!(
    ///     vec![1.0, 1.0, 1.0],
    ///     mode_absolute_deviation
    /// );
    /// ```
    #[inline]
    pub fn absolute_deviation(prices: &[f64], period: usize, config: AbsDevConfig) -> Vec<f64> {
        assert_period(period, prices.len());
        prices
            .windows(period)
            .map(|w| single::absolute_deviation(w, config))
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
    /// * `precision` - Precision to group prices (e.g., 1.0 for whole numbers, 0.1 for one decimal place)
    /// * `period` - Period over which to calculate the price distribution
    ///
    /// # Returns
    ///
    /// A vector of tuples containing (value, index)
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
    /// let distribution = centaur_technical_indicators::basic_indicators::bulk::price_distribution(&prices, 3, 1.0);
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
    pub fn price_distribution(
        prices: &[f64],
        period: usize,
        precision: f64,
    ) -> Vec<Vec<(f64, usize)>> {
        assert_period(period, prices.len());
        prices
            .windows(period)
            .map(|w| single::price_distribution(w, precision))
            .collect()
    }

    /// Calculates the log standard deviation of a slice of prices over a given period.
    ///
    /// Computes the standard deviation of log-transformed prices in each window.
    /// Useful for analyzing multiplicative/percentage-based volatility.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices (must be positive)
    /// * `period` - Period over which to calculate the log standard deviation
    ///
    /// # Returns
    ///
    /// A vector of calculated values
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `period` == 0
    ///     * `period` > `prices.len()`
    ///     * Any price in a window is <= 0
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let log_std = centaur_technical_indicators::basic_indicators::bulk::log_standard_deviation(&prices, 3);
    /// assert_eq!(3, log_std.len());
    /// ```
    #[inline]
    pub fn log_standard_deviation(prices: &[f64], period: usize) -> Vec<f64> {
        assert_period(period, prices.len());
        let mut result = Vec::with_capacity(prices.len());
        for window in prices.windows(period) {
            result.push(single::log_standard_deviation(window))
        }
        result
    }

    /// Calculates the Student's t-adjusted standard deviation over a given period.
    ///
    /// Adjusts the sample standard deviation by the factor sqrt(df/(df-2))
    /// to match the standard deviation of a Student's t-distribution.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `period` - Period over which to calculate the standard deviation
    /// * `df` - Degrees of freedom (must be > 2)
    ///
    /// # Returns
    ///
    /// A vector of calculated values
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `period` == 0
    ///     * `period` > `prices.len()`
    ///     * `df` <= 2.0
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let student_std = centaur_technical_indicators::basic_indicators::bulk::student_t_adjusted_std(&prices, 3, 5.0);
    /// assert_eq!(3, student_std.len());
    /// ```
    #[inline]
    pub fn student_t_adjusted_std(prices: &[f64], period: usize, df: f64) -> Vec<f64> {
        assert_period(period, prices.len());
        let mut result = Vec::with_capacity(prices.len());
        for window in prices.windows(period) {
            result.push(single::student_t_adjusted_std(window, df))
        }
        result
    }

    /// Calculates the Laplace standard deviation equivalent over a given period.
    ///
    /// Estimates the scale parameter of a Laplace distribution as sqrt(2) * MAD,
    /// where MAD is the median absolute deviation from the median.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `period` - Period over which to calculate the Laplace std equivalent
    ///
    /// # Returns
    ///
    /// A vector of calculated values
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
    /// let prices = vec![0.0, 1.0, 2.0, 3.0, 4.0];
    /// let laplace_std = centaur_technical_indicators::basic_indicators::bulk::laplace_std_equivalent(&prices, 3);
    /// assert_eq!(3, laplace_std.len());
    /// ```
    #[inline]
    pub fn laplace_std_equivalent(prices: &[f64], period: usize) -> Vec<f64> {
        assert_period(period, prices.len());
        let mut result = Vec::with_capacity(prices.len());
        for window in prices.windows(period) {
            result.push(single::laplace_std_equivalent(window))
        }
        result
    }

    /// Calculates the Cauchy IQR-based scale parameter over a given period.
    ///
    /// Estimates the scale parameter (gamma) of a Cauchy distribution as (Q3 - Q1) / 2,
    /// where Q1 and Q3 are the first and third quartiles.
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `period` - Period over which to calculate the Cauchy scale (must be >= 4)
    ///
    /// # Returns
    ///
    /// A vector of calculated values
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `period` < 4
    ///     * `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    /// let cauchy_scale = centaur_technical_indicators::basic_indicators::bulk::cauchy_iqr_scale(&prices, 4);
    /// assert_eq!(3, cauchy_scale.len());
    /// ```
    #[inline]
    pub fn cauchy_iqr_scale(prices: &[f64], period: usize) -> Vec<f64> {
        if period < 4 {
            panic!(
                "Period ({}) must be at least 4 for Cauchy IQR scale",
                period
            );
        }
        if period > prices.len() {
            panic!(
                "Period ({}) cannot be longer than the length of prices ({})",
                period,
                prices.len()
            );
        }
        let mut result = Vec::with_capacity(prices.len());
        for window in prices.windows(period) {
            result.push(single::cauchy_iqr_scale(window))
        }
        result
    }

    /// Empirical quantile range `q_high - q_low` computed from the price histogram.
    ///
    /// This function is a building block for an empirical deviation model: choose a lower and
    /// upper quantile (e.g., 0.25 and 0.75 for IQR) and a `precision` that matches the instrument’s
    /// tick size to get a robust, distribution-free scale for each window or slice.
    ///
    /// - Histogram: prices are grouped to `precision` and counted by `price_distribution`.
    /// - Quantiles: computed via [`empirical_quantile_from_distribution`] with linear interpolation.
    /// - Result: `q(high) - q(low)` as a width (not a variance-derived standard deviation).
    ///
    /// # Returns
    ///
    /// A vector of calculated values
    ///
    /// Panics:
    /// - If `precision <= 0.0` or NaN.
    /// - If `low`, `high` are not in (0, 1) or `low >= high`.
    ///
    /// Examples
    /// ```
    /// // IQR for [1,2,3,4] at precision 1.0 is 3.25 - 1.75 = 1.5
    /// let prices = vec![1.0, 2.0, 3.0, 4.0];
    /// let iqr = centaur_technical_indicators::basic_indicators::bulk::empirical_quantile_range_from_distribution(&prices, 3, 1.0, 0.25, 0.75);
    /// assert_eq!(vec![1.0, 1.0], iqr);
    /// ```
    #[inline]
    pub fn empirical_quantile_range_from_distribution(
        prices: &[f64],
        period: usize,
        precision: f64,
        low: f64,
        high: f64,
    ) -> Vec<f64> {
        assert_period(period, prices.len());
        prices
            .windows(period)
            .map(|w| single::empirical_quantile_range_from_distribution(w, precision, low, high))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::E;

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
            single::absolute_deviation(
                &prices,
                crate::AbsDevConfig {
                    center: crate::CentralPoint::Mean,
                    aggregate: crate::DeviationAggregate::Mean
                }
            )
        );
        assert_eq!(
            0.15000000000000568,
            single::absolute_deviation(
                &prices,
                crate::AbsDevConfig {
                    center: crate::CentralPoint::Median,
                    aggregate: crate::DeviationAggregate::Median
                }
            )
        );
        assert_eq!(
            0.0,
            single::absolute_deviation(
                &prices,
                crate::AbsDevConfig {
                    center: crate::CentralPoint::Mode,
                    aggregate: crate::DeviationAggregate::Mode
                }
            )
        );
    }

    #[test]
    #[should_panic]
    fn singe_absolute_deviation_panic() {
        let prices = Vec::new();
        single::absolute_deviation(
            &prices,
            crate::AbsDevConfig {
                center: crate::CentralPoint::Mean,
                aggregate: crate::DeviationAggregate::Mean,
            },
        );
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
            bulk::absolute_deviation(
                &prices,
                period,
                crate::AbsDevConfig {
                    center: crate::CentralPoint::Mean,
                    aggregate: crate::DeviationAggregate::Mean
                }
            )
        );
        assert_eq!(
            vec![
                0.07000000000000739,
                0.07000000000000739,
                0.15000000000000568
            ],
            bulk::absolute_deviation(
                &prices,
                period,
                crate::AbsDevConfig {
                    center: crate::CentralPoint::Median,
                    aggregate: crate::DeviationAggregate::Median
                }
            )
        );
        assert_eq!(
            vec![0.0, 0.0, 0.0],
            bulk::absolute_deviation(
                &prices,
                period,
                crate::AbsDevConfig {
                    center: crate::CentralPoint::Mode,
                    aggregate: crate::DeviationAggregate::Mode
                }
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_absolute_deviation_long_period_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 30;
        bulk::absolute_deviation(
            &prices,
            period,
            crate::AbsDevConfig {
                center: crate::CentralPoint::Median,
                aggregate: crate::DeviationAggregate::Median,
            },
        );
    }

    #[test]
    #[should_panic]
    fn bulk_absolute_deviation_no_period_panic() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let period: usize = 30;
        bulk::absolute_deviation(
            &prices,
            period,
            crate::AbsDevConfig {
                center: crate::CentralPoint::Median,
                aggregate: crate::DeviationAggregate::Median,
            },
        );
    }

    #[test]
    fn single_max() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(100.53, single::max(&prices));
    }

    #[test]
    #[should_panic]
    fn single_max_panic() {
        let prices = Vec::new();
        single::max(&prices);
    }

    #[test]
    fn single_min() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(100.19, single::min(&prices));
    }

    #[test]
    #[should_panic]
    fn single_min_panic() {
        let prices = Vec::new();
        single::min(&prices);
    }

    #[test]
    fn single_price_distribution() {
        let prices = vec![100.0, 102.0, 100.0, 103.0, 102.0, 100.0];
        let distribution = single::price_distribution(&prices, 1.0);
        assert_eq!(vec![(100.0, 3), (102.0, 2), (103.0, 1)], distribution);
    }

    #[test]
    fn single_price_distribution_unique() {
        let prices = vec![100.0, 101.0, 102.0, 103.0];
        let distribution = single::price_distribution(&prices, 1.0);
        assert_eq!(
            vec![(100.0, 1), (101.0, 1), (102.0, 1), (103.0, 1)],
            distribution
        );
    }

    #[test]
    fn single_price_distribution_same() {
        let prices = vec![100.0, 100.0, 100.0];
        let distribution = single::price_distribution(&prices, 1.0);
        assert_eq!(vec![(100.0, 3)], distribution);
    }

    #[test]
    #[should_panic]
    fn single_price_distribution_panic() {
        let prices = Vec::new();
        single::price_distribution(&prices, 1.0);
    }

    #[test]
    #[should_panic]
    fn single_price_distribution_bad_precision() {
        single::price_distribution(&[1.0], 0.0);
    }

    #[test]
    fn single_price_distribution_precision_examples() {
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
        assert_eq!(
            vec![(5949.400000000001, 1)],
            single::price_distribution(&prices, 0.1)
        );
    }

    #[test]
    fn single_price_distribution_half_precision() {
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
    fn single_price_distribution_nan_ignored() {
        let prices = vec![100.0, f64::NAN, 100.4, 100.49, 100.51];
        // precision 0.5 -> 100.0, 100.5 buckets
        assert_eq!(
            vec![(100.0, 1), (100.5, 3)],
            single::price_distribution(&prices, 0.5)
        );
    }

    #[test]
    fn bulk_price_distribution() {
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
    fn bulk_price_distribution_half_precision() {
        let prices = vec![100.2, 100.46, 100.53, 101.08];
        // period 3, precision 0.5
        // windows:
        // [100.2,100.46,100.53] -> 100.0 (1), 100.5 (2)
        // [100.46,100.53,101.08] -> 100.5 (2), 101.0 (1)
        let distribution = bulk::price_distribution(&prices, 3, 0.5);
        assert_eq!(
            vec![vec![(100.0, 1), (100.5, 2)], vec![(100.5, 2), (101.0, 1)],],
            distribution
        );
    }

    #[test]
    #[should_panic]
    fn bulk_price_distribution_period_too_long() {
        let prices = vec![100.0, 102.0, 100.0];
        bulk::price_distribution(&prices, 5, 1.0);
    }

    #[test]
    #[should_panic]
    fn bulk_price_distribution_zero_period() {
        let prices = vec![100.0, 102.0, 100.0];
        bulk::price_distribution(&prices, 0, 1.0);
    }

    #[test]
    #[should_panic]
    fn bulk_price_distribution_bad_precision() {
        let prices = vec![100.0, 101.0, 102.0];
        bulk::price_distribution(&prices, 2, -1.0);
    }

    #[test]
    fn log_standard_deviation_simple_series() {
        // prices = [1, e, e^2] -> logs = [0, 1, 2], sample std = 1
        let prices = vec![1.0, E, E.powi(2)];
        let s = single::log_standard_deviation(&prices);
        assert_eq!(0.816496580927726, s);
    }

    #[test]
    #[should_panic]
    fn log_standard_deviation_panics_on_non_positive() {
        let prices = vec![1.0, 0.0];
        let _ = single::log_standard_deviation(&prices);
    }

    #[test]
    fn student_t_adjusted_std_factor_works() {
        // base series with sample std = 1.0
        let prices = vec![1.0, 2.0, 3.0];
        // df = 5 => adjustment sqrt(df/(df-2)) = sqrt(5/3)
        let df = 5.0;
        let s = single::student_t_adjusted_std(&prices, df);
        assert_eq!(1.0540925533894598, s);
    }

    #[test]
    #[should_panic]
    fn student_t_adjusted_std_panics_on_low_df() {
        let prices = vec![1.0, 2.0, 3.0];
        let _ = single::student_t_adjusted_std(&prices, 2.0);
    }

    #[test]
    fn laplace_std_equivalent_matches_sqrt2_mad() {
        // median = 1, deviations = [1,1,1,0,1,1,1], MAD = 1 => σ_laplace = √2
        let prices = vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0];
        let s = single::laplace_std_equivalent(&prices);
        let expected = 2.0_f64.sqrt();
        assert!(
            (s - expected).abs() < 1e-12,
            "expected {}, got {}",
            expected,
            s
        );
    }

    #[test]
    fn cauchy_iqr_scale_basic() {
        // [1,2,3,4], Q1 = 1.5, Q3 = 3.5 => IQR = 2 => gamma = 1
        let prices = vec![1.0, 2.0, 3.0, 4.0];
        let s = single::cauchy_iqr_scale(&prices);
        assert!((s - 1.0).abs() < 1e-12, "expected 1.0, got {}", s);
    }

    #[test]
    #[should_panic]
    fn cauchy_iqr_scale_panics_on_short_input() {
        let prices = vec![1.0, 2.0, 3.0];
        let _ = single::cauchy_iqr_scale(&prices);
    }

    // Bulk tests for new functions

    #[test]
    fn bulk_log_standard_deviation() {
        let prices = vec![1.0, E, E.powi(2), E.powi(3), E.powi(4)];
        let log_std = bulk::log_standard_deviation(&prices, 3);
        assert_eq!(3, log_std.len());
        // Each window should have similar behavior to single version
        assert!(log_std[0] > 0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_log_standard_deviation_zero_period() {
        let prices = vec![1.0, 2.0, 3.0];
        let _ = bulk::log_standard_deviation(&prices, 0);
    }

    #[test]
    #[should_panic]
    fn bulk_log_standard_deviation_period_too_long() {
        let prices = vec![1.0, 2.0, 3.0];
        let _ = bulk::log_standard_deviation(&prices, 5);
    }

    #[test]
    #[should_panic]
    fn bulk_log_standard_deviation_panics_on_non_positive() {
        let prices = vec![1.0, 0.0, 2.0, 3.0];
        let _ = bulk::log_standard_deviation(&prices, 2);
    }

    #[test]
    fn bulk_student_t_adjusted_std() {
        let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let df = 5.0;
        let student_std = bulk::student_t_adjusted_std(&prices, 3, df);
        assert_eq!(3, student_std.len());
        // Each value should be adjusted by sqrt(df/(df-2))
        assert!(student_std[0] > 0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_student_t_adjusted_std_zero_period() {
        let prices = vec![1.0, 2.0, 3.0];
        let _ = bulk::student_t_adjusted_std(&prices, 0, 5.0);
    }

    #[test]
    #[should_panic]
    fn bulk_student_t_adjusted_std_period_too_long() {
        let prices = vec![1.0, 2.0, 3.0];
        let _ = bulk::student_t_adjusted_std(&prices, 5, 5.0);
    }

    #[test]
    #[should_panic]
    fn bulk_student_t_adjusted_std_panics_on_low_df() {
        let prices = vec![1.0, 2.0, 3.0, 4.0];
        let _ = bulk::student_t_adjusted_std(&prices, 2, 2.0);
    }

    #[test]
    fn bulk_laplace_std_equivalent() {
        let prices = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let laplace_std = bulk::laplace_std_equivalent(&prices, 3);
        assert_eq!(3, laplace_std.len());
        assert!(laplace_std[0] > 0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_laplace_std_equivalent_zero_period() {
        let prices = vec![1.0, 2.0, 3.0];
        let _ = bulk::laplace_std_equivalent(&prices, 0);
    }

    #[test]
    #[should_panic]
    fn bulk_laplace_std_equivalent_period_too_long() {
        let prices = vec![1.0, 2.0, 3.0];
        let _ = bulk::laplace_std_equivalent(&prices, 5);
    }

    #[test]
    fn bulk_cauchy_iqr_scale() {
        let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let cauchy_scale = bulk::cauchy_iqr_scale(&prices, 4);
        assert_eq!(3, cauchy_scale.len());
        assert!(cauchy_scale[0] > 0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_cauchy_iqr_scale_period_less_than_four() {
        let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let _ = bulk::cauchy_iqr_scale(&prices, 3);
    }

    #[test]
    #[should_panic]
    fn bulk_cauchy_iqr_scale_period_too_long() {
        let prices = vec![1.0, 2.0, 3.0, 4.0];
        let _ = bulk::cauchy_iqr_scale(&prices, 5);
    }
    #[test]
    fn single_empirical_quantile_range_from_distribution_simple() {
        // For [1,2,3,4] with precision 1.0, q25=1.75, q75=3.25 => IQR=1.5 (linear interpolation)
        let prices = vec![1.0, 2.0, 3.0, 4.0];
        let iqr = single::empirical_quantile_range_from_distribution(&prices, 1.0, 0.25, 0.75);
        assert_eq!(2.0, iqr,);
    }

    #[test]
    fn bulk_empirical_quantile_range_from_distribution() {
        let prices = vec![1.0, 2.0, 3.0, 4.0];
        let v = bulk::empirical_quantile_range_from_distribution(&prices, 3, 1.0, 0.25, 0.75);
        // windows: [1,2,3] -> IQR=1.0; [2,3,4] -> IQR=1.0
        assert_eq!(vec![1.0, 1.0], v);
    }

    #[test]
    #[should_panic]
    fn single_empirical_quantile_invalid_bounds() {
        let prices = vec![1.0, 2.0, 3.0];
        let _ = single::empirical_quantile_range_from_distribution(&prices, 1.0, 0.8, 0.2);
    }
}
