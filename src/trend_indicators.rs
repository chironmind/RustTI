//! # Trend Indicators
//!
//! This module provides functions to analyze the direction and persistence of price movements in financial time series.
//!
//! ## When to Use
//! Use these indicators when you want to:
//! - Identify or confirm the direction (uptrend, downtrend, sideways) of a market or asset
//! - Detect trend reversals or continuations
//! - Quantify the strength or consistency of a trend for strategy design or signal filtering
//! - Calculate trend-following stop-and-reverse systems (e.g., Parabolic SAR)
//! - Explore relationships between price and volume for trend confirmation
//!
//! ## Structure
//! - **single**: Functions that return a single value for a slice of prices.
//! - **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
//!
//! ## Included Indicators
//!
//! ### Bulk
//!
//! - [`aroon_down`](bulk::aroon_down): Calculates the Aroon Down
//! - [`aroon_indicator`](bulk::aroon_indicator): Calculates Aroon Up, Down, and Oscillator
//! - [`aroon_oscillator`](bulk::aroon_oscillator): Calculates the Aroon Oscillator
//! - [`aroon_up`](bulk::aroon_up): Calculates the Aroon Up
//! - [`parabolic_time_price_system`](bulk::parabolic_time_price_system): Computes the Parabolic Time Price System (Welles Wilder's SAR variant)
//! - [`directional_movement_system`](bulk::directional_movement_system): Computes Directional Movement (+DI, -DI, ADX, ADXR)
//! - [`volume_price_trend`](bulk::volume_price_trend): Computes the Volume Price Trend
//! - [`true_strength_index`](bulk::true_strength_index): Computes the True Strength Index (TSI)
//!
//! ### Single
//!
//! - [`aroon_down`](single::aroon_down): Calculates the Aroon Down
//! - [`aroon_indicator`](single::aroon_indicator): Calculates Aroon Up, Down, and Oscillator
//! - [`aroon_oscillator`](single::aroon_oscillator): Calculates the Aroon Oscillator
//! - [`aroon_up`](single::aroon_up): Calculates the Aroon Up
//! - [`long_parabolic_time_price_system`](single::long_parabolic_time_price_system): Computes Parabolic SAR for long positions
//! - [`short_parabolic_time_price_system`](single::short_parabolic_time_price_system): Computes Parabolic SAR for short positions
//! - [`volume_price_trend`](single::volume_price_trend): Computes the Volume Price Trend
//! - [`true_strength_index`](single::true_strength_index): Computes the True Strength Index (TSI)
//!
//! ## API Details
//! - See function-level documentation for arguments, panics, and usage examples.
//!
//! ---

/// **single**: Functions that return a single value for a slice of prices.
pub mod single {
    use crate::basic_indicators::bulk::median as bulk_median;
    use crate::basic_indicators::bulk::mode as bulk_mode;
    use crate::basic_indicators::single::median as single_median;
    use crate::basic_indicators::single::mode as single_mode;
    use crate::basic_indicators::single::{max, min};
    use crate::moving_average::bulk::moving_average as bulk_ma;
    use crate::moving_average::single::moving_average as single_ma;
    use crate::{ConstantModelType, MovingAverageType};

    /// Calculates the Aroon up
    ///
    /// # Arguments
    ///
    /// * `highs` - Slice of highs
    ///
    /// # Panics
    ///
    /// Panics if `highs.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0];
    /// let aroon_up = rust_ti::trend_indicators::single::aroon_up(&highs);
    /// assert_eq!(50.0, aroon_up);
    /// ```
    #[inline]
    pub fn aroon_up(highs: &[f64]) -> f64 {
        if highs.is_empty() {
            panic!("Highs cannot be empty")
        };

        let period = highs.len() - 1; // current period should be excluded from length
        let period_max = max(highs);
        let periods_since_max = period - highs.iter().rposition(|&x| x == period_max).unwrap();
        100.0 * ((period as f64 - periods_since_max as f64) / period as f64)
    }

    /// Calculates the Aroon down
    ///
    /// # Arguments
    ///
    /// * `low` - Slice of lows
    ///
    /// # Panics
    ///
    /// Panics if `low.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let lows = vec![98.0, 95.0, 101.0, 100.0, 97.0];
    /// let aroon_down = rust_ti::trend_indicators::single::aroon_down(&lows);
    /// assert_eq!(25.0, aroon_down);
    /// ```
    #[inline]
    pub fn aroon_down(lows: &[f64]) -> f64 {
        if lows.is_empty() {
            panic!("Lows cannot be empty")
        };

        let period = lows.len() - 1; // current period should be excluded from length
        let period_min = min(lows);
        let periods_since_min = period - lows.iter().rposition(|&x| x == period_min).unwrap();
        100.0 * ((period as f64 - periods_since_min as f64) / period as f64)
    }

    /// Calculates the Aroon Oscillator
    ///
    /// # Arguments
    ///
    /// * `aroon_up` - Aroon up for the period
    /// * `aroon_down` - Aroon down for the period
    ///
    /// # Examples
    ///
    /// ```rust
    /// let aroon_up = 50.0;
    /// let aroon_down = 25.0;
    /// let aroon_oscillator =
    ///     rust_ti::trend_indicators::single::aroon_oscillator(
    ///         aroon_up,
    ///         aroon_down
    ///     );
    /// assert_eq!(25.0, aroon_oscillator);
    /// ```
    #[inline(always)]
    pub fn aroon_oscillator(aroon_up: f64, aroon_down: f64) -> f64 {
        aroon_up - aroon_down
    }

    /// Calculates the Aroon Indicator
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    ///
    /// # Panics
    ///
    /// `high.len()` != `low.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0];
    /// let lows = vec![98.0, 95.0, 101.0, 100.0, 97.0];
    /// let aroon_indicator =
    ///     rust_ti::trend_indicators::single::aroon_indicator(&highs, &lows);
    /// assert_eq!((50.0, 25.0, 25.0), aroon_indicator);
    /// ```
    #[inline]
    pub fn aroon_indicator(highs: &[f64], lows: &[f64]) -> (f64, f64, f64) {
        if highs.len() != lows.len() {
            panic!(
                "Length of highs ({}) must match length of lows ({})",
                highs.len(),
                lows.len()
            )
        };

        let aroon_up = aroon_up(highs);
        let aroon_down = aroon_down(lows);
        let aroon_oscillaor = aroon_oscillator(aroon_up, aroon_down);
        (aroon_up, aroon_down, aroon_oscillaor)
    }

    /// Calculates the long Stop and Reverse (SaR) point for the Parabolic Time Price System
    ///
    /// # Arguments
    ///
    /// * `previous_sar` - Previous SaR (if none use period low)
    /// * `extreme_point` - Highest high for the period
    /// * `acceleration_factor` - SaR acceleration
    /// * `low` - Lowest low for t or t-1
    ///
    /// # Examples
    ///
    /// ```rust
    /// let previous_sar = 50.09306;
    /// let extreme_point = 52.35;
    /// let acceleration_factor = 0.02;
    /// let low = 50.6;
    ///
    /// let parabolic_time_price_system =
    ///     rust_ti::trend_indicators::single::long_parabolic_time_price_system(
    ///         previous_sar,
    ///         extreme_point,
    ///         acceleration_factor,
    ///         low
    ///     );
    /// assert_eq!(50.1381988, parabolic_time_price_system);
    ///
    /// let previous_sar = 51.96;
    /// let extreme_point = 54.2;
    /// let acceleration_factor = 0.12;
    /// let low = 52.1;
    ///
    /// let parabolic_time_price_system =
    ///     rust_ti::trend_indicators::single::long_parabolic_time_price_system(
    ///         previous_sar,
    ///         extreme_point,
    ///         acceleration_factor,
    ///         low
    ///     );
    /// assert_eq!(52.1, parabolic_time_price_system);
    /// ```
    #[inline]
    pub fn long_parabolic_time_price_system(
        previous_sar: f64,
        extreme_point: f64,
        acceleration_factor: f64,
        low: f64,
    ) -> f64 {
        let sar = previous_sar + acceleration_factor * (extreme_point - previous_sar);
        sar.min(low)
    }

    /// Calculates the short Stop and Reverse (SaR) point for the Parabolic Time Price System
    ///
    /// # Arguments
    ///
    /// * `previous_sar` - Previous SaR (if none use period high)
    /// * `extreme_point` - Lowest low for the period
    /// * `acceleration_factor` - SaR acceleration
    /// * `high` - Highest high for t or t-1
    ///
    /// # Examples
    ///
    /// ```rust
    /// let previous_sar = 58.0;
    /// let extreme_point = 56.3;
    /// let acceleration_factor = 0.02;
    /// let high = 50.6;
    ///
    /// let parabolic_time_price_system =
    ///     rust_ti::trend_indicators::single::short_parabolic_time_price_system(
    ///         previous_sar,
    ///         extreme_point,
    ///         acceleration_factor,
    ///         high
    ///     );
    /// assert_eq!(57.966, parabolic_time_price_system);
    ///
    /// let previous_sar = 57.7816384;
    /// let extreme_point = 55.5;
    /// let acceleration_factor = 0.08;
    /// let low = 58.1;
    ///
    /// let parabolic_time_price_system =
    ///     rust_ti::trend_indicators::single::short_parabolic_time_price_system(
    ///         previous_sar,
    ///         extreme_point,
    ///         acceleration_factor,
    ///         low
    ///     );
    /// assert_eq!(58.1, parabolic_time_price_system);
    /// ```
    #[inline]
    pub fn short_parabolic_time_price_system(
        previous_sar: f64,
        extreme_point: f64,
        acceleration_factor: f64,
        high: f64,
    ) -> f64 {
        let sar = previous_sar - acceleration_factor * (previous_sar - extreme_point);
        sar.max(high)
    }

    /// Calculates the Volume Price Trend (VPT)
    ///
    /// # Arguments
    ///
    /// * `current_price` - Current price
    /// * `previous_price` - Previous prices
    /// * `volume` - Current volume
    /// * `previous_volume_price_trend` - Previous VPT (if none use 0.0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let current_price = 102.0;
    /// let previous_price = 101.0;
    /// let volume = 1000.0;
    /// let volume_price_trend = rust_ti::trend_indicators::single::volume_price_trend(
    ///     current_price,
    ///     previous_price,
    ///     volume,
    ///     0.0
    /// );
    /// assert_eq!(9.900990099009901, volume_price_trend);
    ///
    /// let next_price = 100.0;
    /// let next_volume = 1500.0;
    /// let volume_price_trend = rust_ti::trend_indicators::single::volume_price_trend(
    ///     next_price,
    ///     current_price,
    ///     next_volume,
    ///     volume_price_trend
    /// );
    /// assert_eq!(-19.510774606872452, volume_price_trend);
    /// ```
    #[inline]
    pub fn volume_price_trend(
        current_price: f64,
        previous_price: f64,
        volume: f64,
        previous_volume_price_trend: f64,
    ) -> f64 {
        previous_volume_price_trend + (volume * ((current_price - previous_price) / previous_price))
    }

    /// Calculates the True Strength Index (TSI)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `first_constant_model` - Variant of [`ConstantModelType`]
    /// * `first_period` - Period over which to apply the first smoothing
    /// * `second_constant_model` - Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `prices.is_empty()`
    ///     * `prices.len()` > `first_period` + 1
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 115.0, 118.0, 120.0, 125.0, 117.0, 113.0, 115.0];
    /// let true_strength_index = rust_ti::trend_indicators::single::true_strength_index(
    ///     &prices,
    ///     rust_ti::ConstantModelType::ExponentialMovingAverage,
    ///     5_usize,
    ///     rust_ti::ConstantModelType::ExponentialMovingAverage,
    /// );
    ///
    /// assert_eq!(-0.25821030430852665, true_strength_index);
    /// ```
    pub fn true_strength_index(
        prices: &[f64],
        first_constant_model: ConstantModelType,
        first_period: usize,
        second_constant_model: ConstantModelType,
    ) -> f64 {
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };
        let length = prices.len();
        if length < first_period + 1 {
            panic!(
                "Length of prices ({}) needs to be equal or greater than the sum ({}) of first_period ({}) and second_period({})",
                length, first_period + 1, first_period, 1
            )
        };

        let mut price_momentum = Vec::with_capacity(length - 1);
        let mut abs_price_momentum = Vec::with_capacity(length - 1);
        for i in 1..length {
            let diff = prices[i] - prices[i - 1];
            price_momentum.push(diff);
            abs_price_momentum.push(diff.abs());
        }

        let (initial_smoothing, abs_initial_smoothing) = match first_constant_model {
            ConstantModelType::SimpleMovingAverage => (
                bulk_ma(&price_momentum, MovingAverageType::Simple, first_period),
                bulk_ma(&abs_price_momentum, MovingAverageType::Simple, first_period),
            ),
            ConstantModelType::SmoothedMovingAverage => (
                bulk_ma(&price_momentum, MovingAverageType::Smoothed, first_period),
                bulk_ma(
                    &abs_price_momentum,
                    MovingAverageType::Smoothed,
                    first_period,
                ),
            ),
            ConstantModelType::ExponentialMovingAverage => (
                bulk_ma(
                    &price_momentum,
                    MovingAverageType::Exponential,
                    first_period,
                ),
                bulk_ma(
                    &abs_price_momentum,
                    MovingAverageType::Exponential,
                    first_period,
                ),
            ),
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => (
                bulk_ma(
                    &price_momentum,
                    MovingAverageType::Personalised {
                        alpha_num,
                        alpha_den,
                    },
                    first_period,
                ),
                bulk_ma(
                    &abs_price_momentum,
                    MovingAverageType::Personalised {
                        alpha_num,
                        alpha_den,
                    },
                    first_period,
                ),
            ),
            ConstantModelType::SimpleMovingMedian => (
                bulk_median(&price_momentum, first_period),
                bulk_median(&abs_price_momentum, first_period),
            ),
            ConstantModelType::SimpleMovingMode => (
                bulk_mode(&price_momentum, first_period),
                bulk_mode(&abs_price_momentum, first_period),
            ),
            _ => panic!("Not a supported constant model type"),
        };

        let (second_smoothing, abs_second_smoothing) = match second_constant_model {
            ConstantModelType::SimpleMovingAverage => (
                single_ma(&initial_smoothing, MovingAverageType::Simple),
                single_ma(&abs_initial_smoothing, MovingAverageType::Simple),
            ),
            ConstantModelType::SmoothedMovingAverage => (
                single_ma(&initial_smoothing, MovingAverageType::Smoothed),
                single_ma(&abs_initial_smoothing, MovingAverageType::Smoothed),
            ),
            ConstantModelType::ExponentialMovingAverage => (
                single_ma(&initial_smoothing, MovingAverageType::Exponential),
                single_ma(&abs_initial_smoothing, MovingAverageType::Exponential),
            ),
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => (
                single_ma(
                    &initial_smoothing,
                    MovingAverageType::Personalised {
                        alpha_num,
                        alpha_den,
                    },
                ),
                single_ma(
                    &abs_initial_smoothing,
                    MovingAverageType::Personalised {
                        alpha_num,
                        alpha_den,
                    },
                ),
            ),
            ConstantModelType::SimpleMovingMedian => (
                single_median(&initial_smoothing),
                single_median(&abs_initial_smoothing),
            ),
            ConstantModelType::SimpleMovingMode => (
                single_mode(&initial_smoothing),
                single_mode(&abs_initial_smoothing),
            ),
            _ => panic!("Not a supported constant model type"),
        };
        if abs_second_smoothing == 0.0 {
            0.0
        } else {
            second_smoothing / abs_second_smoothing
        }
    }
}

/// **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
pub mod bulk {
    use crate::basic_indicators::bulk::{median, mode};
    use crate::basic_indicators::single::{max, min};
    use crate::moving_average::bulk::moving_average;
    use crate::other_indicators::bulk::true_range;
    use crate::trend_indicators::single;
    use crate::{ConstantModelType, MovingAverageType, Position};

    /// Calculates the aroon up
    ///
    /// # Arguments
    ///
    /// * `highs` - Slice of highs
    /// * `period` - Period over which to calculate the Aroon up
    ///
    /// # Panics
    ///
    /// Panics if `period` > `highs.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 102.0, 99.0];
    /// let period: usize = 5;
    /// let aroon_up = rust_ti::trend_indicators::bulk::aroon_up(&highs, period);
    /// assert_eq!(vec![50.0, 25.0, 0.0], aroon_up);
    /// ```
    #[inline]
    pub fn aroon_up(highs: &[f64], period: usize) -> Vec<f64> {
        let length = highs.len();
        if length < period {
            panic!(
                "Period ({}) cannot be longer than length of highs ({})",
                period, length
            )
        };

        let mut aroon_ups = Vec::with_capacity(length - period + 1);
        for window in highs.windows(period) {
            aroon_ups.push(single::aroon_up(window));
        }
        aroon_ups
    }

    /// Calculates the aroon down
    ///
    /// # Arguments
    ///
    /// * `low` - Slice of lows
    /// * `period` - Period over which to calculate the Aroon down
    ///
    /// # Panics
    ///
    /// Panics if `period` > `low.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let lows = vec![98.0, 95.0, 101.0, 100.0, 97.0, 98.0, 97.0];
    /// let period: usize = 5;
    /// let aroon_down = rust_ti::trend_indicators::bulk::aroon_down(&lows, period);
    /// assert_eq!(vec![25.0, 0.0, 100.0], aroon_down);
    /// ```
    #[inline]
    pub fn aroon_down(lows: &[f64], period: usize) -> Vec<f64> {
        let length = lows.len();
        if length < period {
            panic!(
                "Period ({}) cannot be longer than length of lows ({})",
                period, length
            )
        };

        let mut aroon_downs = Vec::with_capacity(length - period + 1);
        for window in lows.windows(period) {
            aroon_downs.push(single::aroon_down(window));
        }
        aroon_downs
    }

    /// Calculates the aroon oscillators
    ///
    /// # Arguments
    ///
    /// * `aroon_up` - Slice of Aroon ups
    /// * `aroon_down` - Slice Aroon downs
    ///
    /// # Panics
    ///
    /// Panics if `aroon_up.len()` != `aroon_down.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let aroon_up = vec![50.0, 25.0, 0.0];
    /// let aroon_down = vec![25.0, 0.0, 100.0];
    /// let aroon_oscillator =
    ///     rust_ti::trend_indicators::bulk::aroon_oscillator(
    ///         &aroon_up,
    ///         &aroon_down
    ///     );
    /// assert_eq!(vec![25.0, 25.0, -100.0], aroon_oscillator);
    /// ```
    #[inline]
    pub fn aroon_oscillator(aroon_up: &[f64], aroon_down: &[f64]) -> Vec<f64> {
        let length = aroon_up.len();
        if length != aroon_down.len() {
            panic!(
                "Length of Aroon up ({}) and Aroon down ({}) must match",
                length,
                aroon_down.len()
            )
        };

        (0..length)
            .map(|i| single::aroon_oscillator(aroon_up[i], aroon_down[i]))
            .collect()
    }

    /// Calculates the aroon indicator
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `period` - Period over which to calculate the Aroon indicator
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * `high.len()` != `low.len()`
    /// * lengths < `period`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 102.0, 99.0];
    /// let lows = vec![98.0, 95.0, 101.0, 100.0, 97.0, 98.0, 97.0];
    /// let period: usize = 5;
    ///
    /// let aroon_indicator =
    ///     rust_ti::trend_indicators::bulk::aroon_indicator(
    ///         &highs,
    ///         &lows,
    ///         period
    ///     );
    /// assert_eq!(
    ///     vec![(50.0, 25.0, 25.0), (25.0, 0.0, 25.0), (0.0, 100.0, -100.0)],
    ///     aroon_indicator
    /// );
    /// ```
    #[inline]
    pub fn aroon_indicator(highs: &[f64], lows: &[f64], period: usize) -> Vec<(f64, f64, f64)> {
        let length = highs.len();
        if length != lows.len() {
            panic!(
                "Length of highs ({}) must match length of lows ({})",
                highs.len(),
                lows.len()
            )
        };
        if length < period {
            panic!(
                "Period ({}) cannot be longer than lengths of highs and lows ({})",
                period, length
            )
        };

        let loop_max = length - period + 1;
        (0..loop_max)
            .map(|i| single::aroon_indicator(&highs[i..i + period], &lows[i..i + period]))
            .collect()
    }

    /// Calculates the Parabolic time price system Stop and Reverse (SaR) points
    ///
    /// # Arguments
    ///
    /// * `highs` - Slice of highs.
    /// * `lows` - Slice of lows.
    /// * `acceleration_factor_start` - Initial acceleration factor
    /// * `acceleration_factor_max` - Maximum acceleration factor
    /// * `acceleration_factor_step` - Acceleration increment
    /// * `start_position` - Variant of [Position]
    /// * `previous_sar`- Previous SaR (0.0 if none)
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `highs.len()` != `lows.len()`
    ///     * `highs.is_empty()` or `lows.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let highs = vec![
    ///     52.35, 52.1, 51.8, 52.1, 52.5, 52.8, 52.5, 53.5, 53.5, 53.8, 54.2,
    ///     53.4, 53.5, 54.4, 55.2, 55.7, 57.0, 57.5, 58.0, 57.7, 58.0, 57.5, 57.0, 56.7,
    ///     57.5, 56.7, 56.0, 56.2, 54.8, 55.5, 54.7, 54.0, 52.5, 51.0, 51.5, 51.7, 53.0
    /// ];
    /// let lows = vec![
    ///     51.5, 51.0, 50.5, 51.25, 51.7, 51.85, 51.5, 52.5, 52.5, 53.0, 52.5,
    ///     52.5, 52.1, 53.0, 54.0, 55.0, 56.0, 56.5, 57.0, 56.5, 57.3, 56.7, 56.3, 56.2,
    ///     56.0, 55.5, 55.0, 54.9, 54.0, 54.5, 53.8, 53.0, 51.5, 50.0, 50.5, 50.2, 51.5
    /// ];
    /// let acceleration_factor_start = 0.02;
    /// let acceleration_factor_max = 0.2;
    /// let acceleration_factor_step = 0.02;
    ///
    /// let parabolic_time_price_system =
    ///     rust_ti::trend_indicators::bulk::parabolic_time_price_system(
    ///         &highs,
    ///         &lows,
    ///         acceleration_factor_start,
    ///         acceleration_factor_max,
    ///         acceleration_factor_step,
    ///         rust_ti::Position::Long,
    ///         50.0
    ///     );
    /// assert_eq!(
    ///     vec![
    ///     50.047, 50.093059999999994, 50.1381988, 50.182434824, 50.27513743104,
    ///     50.4266291851776, 50.56903143406695, 50.803508919341596, 51.01922820579427,
    ///     51.29730538521484, 51.64562873898906, 51.95215329031037, 52.1, 52.1,
    ///     52.596000000000004, 53.154720000000005, 53.923776000000004, 54.639020800000004,
    ///     55.311216640000005, 55.848973312000005, 56.279178649600006, 56.623342919680006,
    ///     57.966, 57.895360000000004, 57.781638400000006, 57.599107328, 57.3391965952,
    ///     57.046493003776, 56.61998398324736, 56.25318622559273, 55.86067642949789,
    ///     55.34575467218827, 54.57660373775062, 53.66128299020049, 52.929026392160395,
    ///     52.34322111372832, 50.06
    ///     ],
    ///     parabolic_time_price_system);
    ///
    /// let highs = vec![
    ///     52.3, 52.0, 52.35, 52.1, 51.8, 52.1, 52.5, 52.8, 52.5, 53.5, 53.5, 53.8, 54.2,
    ///     53.4, 53.5, 54.4, 55.2, 55.7, 57.0, 57.5, 58.0, 57.7, 58.0, 57.5, 57.0, 56.7,
    ///     57.5, 56.7, 56.0, 56.2, 54.8, 55.5, 54.7, 54.0, 52.5, 51.0, 51.5, 51.7, 53.0
    /// ];
    /// let lows = vec![
    ///     50.0, 51.0, 51.5, 51.0, 50.5, 51.25, 51.7, 51.85, 51.5, 52.5, 52.5, 53.0, 52.5,
    ///     52.5, 52.1, 53.0, 54.0, 55.0, 56.0, 56.5, 57.0, 56.5, 57.3, 56.7, 56.3, 56.2,
    ///     56.0, 55.5, 55.0, 54.9, 54.0, 54.5, 53.8, 53.0, 51.5, 50.0, 50.5, 50.2, 51.5
    /// ];
    ///
    /// let parabolic_time_price_system =
    ///     rust_ti::trend_indicators::bulk::parabolic_time_price_system(
    ///         &highs,
    ///         &lows,
    ///         acceleration_factor_start,
    ///         acceleration_factor_max,
    ///         acceleration_factor_step,
    ///         rust_ti::Position::Short,
    ///         0.0
    ///     );
    /// assert_eq!(
    ///     vec![
    ///         52.3, 52.3, 50.047, 50.093059999999994, 50.1381988, 50.182434824,
    ///         50.27513743104, 50.4266291851776, 50.56903143406695, 50.803508919341596,
    ///         51.01922820579427, 51.29730538521484, 51.64562873898906, 51.95215329031037,
    ///         52.1, 52.1, 52.596000000000004, 53.154720000000005, 53.923776000000004,
    ///         54.639020800000004, 55.311216640000005, 55.848973312000005, 56.279178649600006,
    ///         56.623342919680006, 57.966, 57.895360000000004, 57.781638400000006,
    ///         57.599107328, 57.3391965952, 57.046493003776, 56.61998398324736,
    ///         56.25318622559273, 55.86067642949789, 55.34575467218827, 54.57660373775062,
    ///         53.66128299020049, 52.929026392160395, 52.34322111372832, 50.06
    ///     ],
    ///     parabolic_time_price_system);
    /// ```
    pub fn parabolic_time_price_system(
        highs: &[f64],
        lows: &[f64],
        acceleration_factor_start: f64,
        acceleration_factor_max: f64,
        acceleration_factor_step: f64,
        start_position: Position,
        previous_sar: f64,
    ) -> Vec<f64> {
        if highs.is_empty() || lows.is_empty() {
            panic!("Highs or lows cannot be empty")
        };
        let length = highs.len();
        if length != lows.len() {
            panic!(
                "Highs ({}) and lows ({}) must be the same length",
                length,
                lows.len()
            )
        };

        // Due to the nature of floats some floats when increased aren't increased exactly
        // For example instead of 0.2 when increasing the acceleration factor by 0.02 we
        // get 0.19999999999999998 which is a problem because when the max is 0.2 that
        // number is less that the max so it would be increased, the temporary solution for
        // this is to substract 0.0000001 from the max, this shouldn't impact the
        // calculation but will resolve this issue.
        let acceleration_factor_max = acceleration_factor_max - 0.0000001;
        let mut acceleration_factor = acceleration_factor_start;
        let mut sars = Vec::with_capacity(length);

        let mut position = start_position;
        let mut position_start = 0;

        if position == Position::Long {
            if previous_sar == 0.0 {
                sars.push(single::long_parabolic_time_price_system(
                    lows[0],
                    highs[0],
                    acceleration_factor,
                    lows[0],
                ));
            } else {
                sars.push(single::long_parabolic_time_price_system(
                    previous_sar,
                    highs[0],
                    acceleration_factor,
                    lows[0],
                ));
            }
        } else if position == Position::Short {
            if previous_sar == 0.0 {
                sars.push(single::short_parabolic_time_price_system(
                    highs[0],
                    lows[0],
                    acceleration_factor,
                    highs[0],
                ));
            } else {
                sars.push(single::short_parabolic_time_price_system(
                    previous_sar,
                    lows[0],
                    acceleration_factor,
                    highs[0],
                ));
            }
        };

        for i in 1..length {
            let previous_sar = sars[i - 1];
            if position == Position::Short && highs[i] > previous_sar {
                position = Position::Long;
                let period_max = highs[i];
                let previous_min = min(&lows[i - 1..=i]);
                acceleration_factor = acceleration_factor_start;
                let pivoted_sar = min(&lows[position_start..i]);
                position_start = i;
                sars.push(single::long_parabolic_time_price_system(
                    pivoted_sar,
                    period_max,
                    acceleration_factor,
                    previous_min,
                ));
            } else if position == Position::Short {
                let mut period_min = min(&lows[position_start..i]);
                if period_min > lows[i] {
                    period_min = lows[i];
                    if acceleration_factor <= acceleration_factor_max {
                        acceleration_factor += acceleration_factor_step;
                    };
                };
                let previous_max = max(&highs[i - 1..i + 1]);
                sars.push(single::short_parabolic_time_price_system(
                    previous_sar,
                    period_min,
                    acceleration_factor,
                    previous_max,
                ));
            } else if position == Position::Long && lows[i] < previous_sar {
                position = Position::Short;
                let period_min = lows[i];
                acceleration_factor = acceleration_factor_start;
                let previous_max = max(&highs[i - 1..=i]);
                let pivoted_sar = max(&highs[position_start..i]);
                position_start = i;
                sars.push(single::short_parabolic_time_price_system(
                    pivoted_sar,
                    period_min,
                    acceleration_factor,
                    previous_max,
                ));
            } else if position == Position::Long {
                let mut period_max = max(&highs[position_start..i]);
                if period_max < highs[i] {
                    period_max = highs[i];
                    if acceleration_factor <= acceleration_factor_max {
                        acceleration_factor += acceleration_factor_step;
                    };
                };
                let previous_min = min(&lows[i - 1..i + 1]);
                sars.push(single::long_parabolic_time_price_system(
                    previous_sar,
                    period_max,
                    acceleration_factor,
                    previous_min,
                ));
            }
        }
        sars
    }

    /// Calculates the directional movement system
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `close` - Slice of closing prices
    /// * `period` - Period over which to calculate the DM
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `high.len()` != `low.len()` != `close.len()`
    ///     * `high.is_empty()`
    ///     * `period` > lengths
    ///
    /// # Examples
    ///
    /// ```rust
    /// let high = vec![
    ///     4383.33, 4393.57, 4364.2, 4339.54, 4276.56, 4255.84, 4259.38,
    ///     4232.42, 4183.6, 4156.7, 4177.47, 4195.55, 4245.64, 4319.72,
    ///     4373.62, 4372.21, 4386.26, 4391.2, 4393.4, 4418.03, 4421.76,
    ///     4508.67, 4521.17, 4511.99, 4520.12, 4557.11, 4542.14, 4568.43,
    ///     4560.31, 4560.52, 4568.14
    /// ];
    ///
    /// let low = vec![
    ///     4342.37, 4337.54, 4303.84, 4269.69, 4223.03, 4189.22, 4219.43,
    ///     4181.42, 4127.9, 4103.78, 4132.94, 4153.12, 4197.74, 4268.26,
    ///     4334.23, 4347.53, 4355.41, 4359.76, 4343.94, 4353.34, 4393.82,
    ///     4458.97, 4495.31, 4487.83, 4499.66, 4510.36, 4525.51, 4545.05,
    ///     4552.8, 4546.32, 4540.51
    /// ];
    ///
    /// let close = vec![
    ///     4373.63, 4373.2, 4314.6, 4278.0, 4224.16, 4217.04, 4247.68,
    ///     4186.77, 4137.23, 4117.37, 4166.82, 4193.8, 4237.86, 4317.78,
    ///     4358.34, 4365.98, 4378.38, 4382.78, 4347.35, 4415.24, 4411.55,
    ///     4495.7, 4502.88, 4508.24, 4514.02, 4547.38, 4538.19, 4556.62,
    ///     4559.34, 4550.43, 4554.89
    /// ];
    ///
    /// let period: usize = 5;
    ///
    /// let directional_movement_system =
    ///     rust_ti::trend_indicators::bulk::directional_movement_system(
    ///         &high,
    ///         &low,
    ///         &close,
    ///         period,
    ///         rust_ti::ConstantModelType::SimpleMovingAverage
    /// );
    ///
    /// assert_eq!(
    ///     vec![
    ///         (68.14077913392383, 10.081926099314382, 58.269764963691, 76.0576148830475),
    ///         (96.10562225864973, 0.0, 59.19525515976943, 74.33813493134635),
    ///         (95.28320217623542, 0.0, 66.14295450243883, 73.24907727490466),
    ///         (98.8882025941931, 0.0, 76.20120692962332, 69.40990834820704),
    ///         (82.65099538859455, 0.0, 94.84450144277015, 76.55713320323058),
    ///         (41.45717210783709, 8.997838698669414, 92.86664412129383, 76.03094964053163),
    ///         (21.688544152744587, 7.865950676213518, 82.22061451160306, 74.18178450702095),
    ///         (23.167628926509607, 7.740483413250127, 72.2032011824909, 74.20220405605711),
    ///         (53.850288939658775, 7.086861084979907, 67.55128616374488, 81.19789380325751),
    ///         (58.70434183321876, 7.268550424994554, 63.14429403337355, 78.00546907733369),
    ///         (66.42578632700847, 3.8887444762154897, 68.06545028176535, 75.1430323966842),
    ///         (75.12152308938734, 5.04995949230386, 76.19190094408756, 74.19755106328924),
    ///         (86.5812017013121, 4.480920146169353, 84.2410227134338, 75.89615443858933),
    ///         (43.04497235918126, 5.587927685642082, 84.29693158778632, 73.72061281057994),
    ///         (54.35378291977454, 5.693408433551885, 84.91130107903966, 76.4883756804025),
    ///         (62.241785060576625, 0.0, 87.12350070935402, 81.6577008267208),
    ///         (58.33871116437639, 5.974002028210937, 85.92748332644709, 85.08425301994043),
    ///         (37.95187465025111, 7.252378287633331, 81.47834482926781, 82.88763820852706)
    ///     ], directional_movement_system);
    /// ```
    pub fn directional_movement_system(
        high: &[f64],
        low: &[f64],
        close: &[f64],
        period: usize,
        constant_model_type: ConstantModelType,
    ) -> Vec<(f64, f64, f64, f64)> {
        let length = high.len();
        if length != low.len() || length != close.len() {
            panic!(
                "Length of high ({}), low ({}), and close ({}) need to be equal",
                length,
                low.len(),
                close.len()
            )
        };
        if high.is_empty() {
            panic!("Prices cannot be empty")
        };
        let length_min = 3 * period;
        if length_min > length {
            panic!(
                "Length of prices ({}) must be greater than ({})",
                length, length_min
            )
        };

        let mut positive_dm = Vec::with_capacity(length - 1);
        let mut negative_dm = Vec::with_capacity(length - 1);

        for i in 1..length {
            let high_diff = high[i] - high[i - 1];
            let low_diff = low[i - 1] - low[i];

            if high_diff > 0.0 && high_diff > low_diff {
                positive_dm.push(high_diff);
                negative_dm.push(0.0);
            } else if low_diff > 0.0 && low_diff > high_diff {
                negative_dm.push(low_diff);
                positive_dm.push(0.0);
            } else {
                positive_dm.push(0.0);
                negative_dm.push(0.0);
            };
        }

        let tr = true_range(&close[1..], &high[1..], &low[1..]);

        let mut positive_di: Vec<f64> = Vec::with_capacity(length - period);
        let mut negative_di: Vec<f64> = Vec::with_capacity(length - period);

        for i in period..length {
            let tr_sum: f64 = tr[i - period..i].iter().sum();
            let positive_dm_sum: f64 = positive_dm[i - period..i].iter().sum();
            let negative_dm_sum: f64 = negative_dm[i - period..i].iter().sum();
            positive_di.push((positive_dm_sum / tr_sum) * 100.0);
            negative_di.push((negative_dm_sum / tr_sum) * 100.0);
        }

        let dx: Vec<f64> = positive_di
            .iter()
            .zip(&negative_di)
            .map(|(&p, &n)| {
                let di_diff = (p - n).abs();
                let di_sum = p + n;
                (di_diff / di_sum) * 100.0
            })
            .collect();

        let adx = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(&dx, MovingAverageType::Simple, period)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(&dx, MovingAverageType::Smoothed, period)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(&dx, MovingAverageType::Exponential, period)
            }
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => moving_average(
                &dx,
                MovingAverageType::Personalised {
                    alpha_num,
                    alpha_den,
                },
                period,
            ),
            ConstantModelType::SimpleMovingMedian => median(&dx, period),
            ConstantModelType::SimpleMovingMode => mode(&dx, period),
            _ => panic!("Not a supported constant model type"),
        };

        let mut adxr = Vec::with_capacity(adx.len() - period - 1);
        for i in period..=adx.len() {
            adxr.push((adx[i - period] + adx[i - 1]) / 2.0);
        }

        let mut directional_movement_system = Vec::with_capacity(adxr.len());
        for i in 0..adxr.len() {
            directional_movement_system.push((
                // Because the period is used 3 times to get various indicators
                // we need to get to a point where all indicators exist but for some
                // indicators that means going forward 2 times the period and removing 2
                positive_di[i + (2 * period) - 2],
                negative_di[i + (2 * period) - 2],
                adx[i + period - 1],
                adxr[i],
            ));
        }
        directional_movement_system
    }

    /// Calculates the Volume Price Trend (VPT)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `volumes` - Slice of volumes
    /// * `previous_volume_price_trend` - Previous VPT (0.0 if none)
    ///
    /// # Panics
    ///
    /// Panics if `volumes.len()` != `prices.len() - 1`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = [101.0, 102.0, 100.0];
    /// let volumes = [1000.0, 1500.0];
    ///
    /// let volume_price_trend =
    ///     rust_ti::trend_indicators::bulk::volume_price_trend(
    ///         &prices,
    ///         &volumes,
    ///         0.0
    ///     );
    /// assert_eq!(
    ///     vec![9.900990099009901, -19.510774606872452],
    ///     volume_price_trend
    /// );
    ///
    /// let next_prices = [100.0, 98.0, 97.0];
    /// let next_volumes = [2000.0, 800.0];
    ///
    /// let volume_price_trend =
    ///     rust_ti::trend_indicators::bulk::volume_price_trend(
    ///         &next_prices,
    ///         &next_volumes,
    ///         volume_price_trend[1]
    ///     );
    /// assert_eq!(
    ///     vec![-59.51077460687245, -67.6740399129949],
    ///     volume_price_trend
    /// );
    /// ```
    #[inline]
    pub fn volume_price_trend(
        prices: &[f64],
        volumes: &[f64],
        previous_volume_price_trend: f64,
    ) -> Vec<f64> {
        let length = volumes.len();
        if length != prices.len() - 1 {
            panic!(
                "Length of volumes ({}) must equal length of prices ({}) - 1",
                length,
                prices.len()
            )
        };

        if volumes.is_empty() || prices.is_empty() {
            panic!("Volumes nor prices can be empty")
        };

        let mut vpts = Vec::with_capacity(length);
        let mut vpt = single::volume_price_trend(
            prices[1],
            prices[0],
            volumes[0],
            previous_volume_price_trend,
        );
        vpts.push(vpt);

        for i in 1..length {
            vpt = single::volume_price_trend(prices[i + 1], prices[i], volumes[i], vpt);
            vpts.push(vpt);
        }
        vpts
    }

    /// Calculates the True Strength Index (TSI)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `first_constant_model` - Variant of [`ConstantModelType`]
    /// * `first_period` - Period for first smoothing
    /// * `second_constant_model` - Variant of [`ConstantModelType`]
    /// * `second_period` - Period for second smoothing
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `prices.is_empty()`
    ///     * `prices.len()` < `first_period` + `second_period`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices =
    ///     vec![100.0, 115.0, 118.0, 120.0, 125.0, 117.0, 113.0, 115.0, 110.0, 107.0];
    /// let true_strength_index = rust_ti::trend_indicators::bulk::true_strength_index(
    ///     &prices,
    ///     rust_ti::ConstantModelType::ExponentialMovingAverage,
    ///     5_usize,
    ///     rust_ti::ConstantModelType::ExponentialMovingAverage,
    ///     3_usize
    /// );
    ///
    /// assert_eq!(
    ///     vec![-0.25821030430852665, -0.48120300751879697, -0.6691474966170501],
    ///     true_strength_index
    /// );
    /// ```
    #[inline]
    pub fn true_strength_index(
        prices: &[f64],
        first_constant_model: ConstantModelType,
        first_period: usize,
        second_constant_model: ConstantModelType,
        second_period: usize,
    ) -> Vec<f64> {
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };
        let length = prices.len();
        let period_sum = first_period + second_period;
        if length < period_sum {
            panic!(
                "Length of prices ({}) needs to be equal or greater than the sum ({}) of first_period ({}) and second_period({})",
                length, first_period + second_period, first_period, second_period
            )
        };

        let loop_max = length - period_sum + 1;

        (0..loop_max)
            .map(|i| {
                single::true_strength_index(
                    &prices[i..i + period_sum],
                    first_constant_model,
                    first_period,
                    second_constant_model,
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_aroon_up() {
        let highs = vec![101.26, 102.57, 102.32, 100.69];
        assert_eq!(33.33333333333333, single::aroon_up(&highs));
    }

    #[test]
    #[should_panic]
    fn singe_aroon_up_panic() {
        let highs = Vec::new();
        single::aroon_up(&highs);
    }

    #[test]
    fn bulk_aroon_up() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        assert_eq!(
            vec![33.33333333333333, 0.0, 0.0, 100.0],
            bulk::aroon_up(&highs, 4)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_aroon_up_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        bulk::aroon_up(&highs, 40);
    }

    #[test]
    fn single_aroon_down() {
        let lows = vec![100.08, 98.75, 100.14, 98.98];
        assert_eq!(33.33333333333333, single::aroon_down(&lows));
    }

    #[test]
    #[should_panic]
    fn single_aroon_down_panic() {
        let lows = Vec::new();
        single::aroon_down(&lows);
    }

    #[test]
    fn bulk_aroon_down() {
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(
            vec![33.33333333333333, 0.0, 33.33333333333333, 0.0],
            bulk::aroon_down(&lows, 4)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_aroon_down_panic() {
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        bulk::aroon_down(&lows, 40);
    }

    #[test]
    fn single_aroon_oscillator() {
        assert_eq!(
            0.0,
            single::aroon_oscillator(33.33333333333333, 33.33333333333333)
        );
    }

    #[test]
    fn bulk_aroon_oscillator() {
        let aroon_up = vec![33.33333333333333, 0.0, 0.0, 100.0];
        let aroon_down = vec![33.33333333333333, 0.0, 33.33333333333333, 0.0];
        assert_eq!(
            vec![0.0, 0.0, -33.33333333333333, 100.0],
            bulk::aroon_oscillator(&aroon_up, &aroon_down)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_aroon_oscillator_up_panic() {
        let aroon_up = vec![33.33333333333333, 0.0, 0.0];
        let aroon_down = vec![33.33333333333333, 0.0, 33.33333333333333, 0.0];
        bulk::aroon_oscillator(&aroon_up, &aroon_down);
    }

    #[test]
    #[should_panic]
    fn bulk_aroon_oscillator_down_panic() {
        let aroon_up = vec![33.33333333333333, 0.0, 0.0, 100.0];
        let aroon_down = vec![33.33333333333333, 0.0, 33.33333333333333];
        bulk::aroon_oscillator(&aroon_up, &aroon_down);
    }

    #[test]
    fn single_aroon_indicator() {
        let lows = vec![100.08, 98.75, 100.14, 98.98];
        let highs = vec![101.26, 102.57, 102.32, 100.69];
        assert_eq!(
            (33.33333333333333, 33.33333333333333, 0.0),
            single::aroon_indicator(&highs, &lows)
        );
    }

    #[test]
    #[should_panic]
    fn single_aroon_indicator_high_panic() {
        let lows = vec![100.08, 98.75, 100.14, 98.98];
        let highs = vec![101.26, 102.57, 102.32];
        single::aroon_indicator(&highs, &lows);
    }

    #[test]
    #[should_panic]
    fn single_aroon_indicator_low_panic() {
        let lows = vec![100.08, 98.75, 100.14];
        let highs = vec![101.26, 102.57, 102.32, 100.69];
        single::aroon_indicator(&highs, &lows);
    }

    #[test]
    fn bulk_aroon_indicator() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(
            vec![
                (33.33333333333333, 33.33333333333333, 0.0),
                (0.0, 0.0, 0.0),
                (0.0, 33.33333333333333, -33.33333333333333),
                (100.0, 0.0, 100.0)
            ],
            bulk::aroon_indicator(&highs, &lows, 4)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_aroon_indicator_high_panic() {
        let highs = vec![102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        bulk::aroon_indicator(&highs, &lows, 4);
    }

    #[test]
    #[should_panic]
    fn bulk_aroon_indicator_low_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        bulk::aroon_indicator(&highs, &lows, 4);
    }

    #[test]
    #[should_panic]
    fn bulk_aroon_indicator_period_panic() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        bulk::aroon_indicator(&highs, &lows, 40);
    }

    #[test]
    fn single_long_parabolic_price_time_system() {
        assert_eq!(
            100.6,
            single::long_parabolic_time_price_system(100.0, 110.0, 0.06, 105.0)
        );
    }

    #[test]
    fn single_long_parabolic_price_time_system_min() {
        assert_eq!(
            90.0,
            single::long_parabolic_time_price_system(100.0, 110.0, 0.06, 90.0)
        );
    }

    #[test]
    fn single_short_parabolic_price_time_system() {
        assert_eq!(
            99.6,
            single::short_parabolic_time_price_system(100.0, 90.0, 0.04, 95.0)
        );
    }

    #[test]
    fn single_short_parabolic_price_time_system_max() {
        assert_eq!(
            105.0,
            single::short_parabolic_time_price_system(100.0, 90.0, 0.04, 105.0)
        );
    }

    #[test]
    fn bulk_parabolic_time_price_system_long_switch_previous() {
        let highs = vec![100.64, 102.39, 101.51, 99.48, 96.93];
        let lows = vec![95.92, 96.77, 95.84, 91.22, 89.12];
        assert_eq!(
            vec![
                90.7812,
                91.245552,
                91.69132992,
                102.1666,
                101.64473600000001
            ],
            bulk::parabolic_time_price_system(
                &highs,
                &lows,
                0.02,
                0.2,
                0.02,
                crate::Position::Long,
                90.58
            )
        );
    }

    #[test]
    fn bulk_parabolic_time_price_system_long_switch_no_previous() {
        let highs = vec![100.64, 102.39, 101.51, 99.48, 96.93];
        let lows = vec![95.92, 96.77, 95.84, 91.22, 89.12];
        assert_eq!(
            vec![95.92, 95.92, 102.39, 101.9432, 101.17380800000001],
            bulk::parabolic_time_price_system(
                &highs,
                &lows,
                0.02,
                0.2,
                0.02,
                crate::Position::Long,
                0.0
            )
        );
    }

    #[test]
    fn bulk_parabolic_time_price_system_short_switch_previous() {
        let highs = vec![99.48, 96.93, 94.66, 102.79, 105.81];
        let lows = vec![91.22, 89.12, 87.35, 88.57, 90.64];
        assert_eq!(
            vec![102.1666, 101.64473600000001, 100.78705184, 87.35, 88.0884],
            bulk::parabolic_time_price_system(
                &highs,
                &lows,
                0.02,
                0.2,
                0.02,
                crate::Position::Short,
                102.39
            )
        );
    }

    #[test]
    fn bulk_parabolic_time_price_system_short_switch_no_previous() {
        let highs = vec![99.48, 96.93, 94.66, 102.79, 105.81];
        let lows = vec![91.22, 89.12, 87.35, 88.57, 90.64];
        assert_eq!(
            vec![99.48, 99.48, 98.7522, 87.35, 88.0884],
            bulk::parabolic_time_price_system(
                &highs,
                &lows,
                0.02,
                0.2,
                0.02,
                crate::Position::Short,
                0.0
            )
        );
    }

    #[test]
    fn bulk_parabolic_time_price_system_long_no_switch() {
        let highs = vec![100.64, 102.39, 101.51];
        let lows = vec![95.92, 96.77, 95.84];
        assert_eq!(
            vec![90.7812, 91.245552, 91.69132992],
            bulk::parabolic_time_price_system(
                &highs,
                &lows,
                0.02,
                0.2,
                0.02,
                crate::Position::Long,
                90.58
            )
        );
    }

    #[test]
    fn bulk_parabolic_time_price_system_short_no_switch() {
        let highs = vec![99.48, 96.93, 94.66];
        let lows = vec![91.22, 89.12, 87.35];
        assert_eq!(
            vec![102.1666, 101.64473600000001, 100.78705184],
            bulk::parabolic_time_price_system(
                &highs,
                &lows,
                0.02,
                0.2,
                0.02,
                crate::Position::Short,
                102.39
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_parabolic_time_price_system_panic_high_empty() {
        let highs = Vec::new();
        let lows = vec![95.92, 96.77, 95.84, 91.22, 89.12];
        assert_eq!(
            vec![
                90.7812,
                91.245552,
                91.69132992,
                102.1666,
                101.64473600000001
            ],
            bulk::parabolic_time_price_system(
                &highs,
                &lows,
                0.02,
                0.2,
                0.02,
                crate::Position::Long,
                90.58
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_parabolic_time_price_system_panic_low_empty() {
        let highs = vec![99.48, 96.93, 94.66, 102.79, 105.81];
        let lows = Vec::new();
        assert_eq!(
            vec![
                90.7812,
                91.245552,
                91.69132992,
                102.1666,
                101.64473600000001
            ],
            bulk::parabolic_time_price_system(
                &highs,
                &lows,
                0.02,
                0.2,
                0.02,
                crate::Position::Long,
                90.58
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_parabolic_time_price_system_panic_high_length() {
        let highs = vec![99.48, 96.93, 94.66, 102.79];
        let lows = vec![95.92, 96.77, 95.84, 91.22, 89.12];
        assert_eq!(
            vec![
                90.7812,
                91.245552,
                91.69132992,
                102.1666,
                101.64473600000001
            ],
            bulk::parabolic_time_price_system(
                &highs,
                &lows,
                0.02,
                0.2,
                0.02,
                crate::Position::Long,
                90.58
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_parabolic_time_price_system_panic_low_length() {
        let highs = vec![99.48, 96.93, 94.66, 102.79, 105.81];
        let lows = vec![95.92, 96.77, 95.84, 91.22];
        assert_eq!(
            vec![
                90.7812,
                91.245552,
                91.69132992,
                102.1666,
                101.64473600000001
            ],
            bulk::parabolic_time_price_system(
                &highs,
                &lows,
                0.02,
                0.2,
                0.02,
                crate::Position::Long,
                90.58
            )
        );
    }

    #[test]
    fn bulk_directional_movement_system_ma() {
        let highs = vec![
            100.83, 100.91, 101.03, 101.27, 100.52, 101.27, 101.03, 100.91, 100.83,
        ];
        let lows = vec![
            100.59, 100.72, 100.84, 100.91, 99.85, 100.91, 100.84, 100.72, 100.59,
        ];
        let close = vec![
            100.76, 100.88, 100.96, 101.14, 100.01, 101.14, 100.96, 100.88, 100.76,
        ];

        assert_eq!(
            vec![
                (
                    101.35135135135205,
                    25.675675675675546,
                    27.733956062965074,
                    39.31871283052075
                ),
                (
                    0.0,
                    51.61290322580615,
                    59.92907801418446,
                    42.118401465704885
                )
            ],
            bulk::directional_movement_system(
                &highs,
                &lows,
                &close,
                3_usize,
                crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn bulk_directional_movement_system_sma() {
        let highs = vec![
            100.83, 100.91, 101.03, 101.27, 100.52, 101.27, 101.03, 100.91, 100.83,
        ];
        let lows = vec![
            100.59, 100.72, 100.84, 100.91, 99.85, 100.91, 100.84, 100.72, 100.59,
        ];
        let close = vec![
            100.76, 100.88, 100.96, 101.14, 100.01, 101.14, 100.96, 100.88, 100.76,
        ];

        assert_eq!(
            vec![
                (
                    101.35135135135205,
                    25.675675675675546,
                    35.32133395242147,
                    36.779255271063406
                ),
                (0.0, 51.61290322580615, 70.43673012318037, 45.73378077439598)
            ],
            bulk::directional_movement_system(
                &highs,
                &lows,
                &close,
                3_usize,
                crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn bulk_directional_movement_system_ema() {
        let highs = vec![
            100.83, 100.91, 101.03, 101.27, 100.52, 101.27, 101.03, 100.91, 100.83,
        ];
        let lows = vec![
            100.59, 100.72, 100.84, 100.91, 99.85, 100.91, 100.84, 100.72, 100.59,
        ];
        let close = vec![
            100.76, 100.88, 100.96, 101.14, 100.01, 101.14, 100.96, 100.88, 100.76,
        ];

        assert_eq!(
            vec![
                (
                    101.35135135135205,
                    25.675675675675546,
                    40.3054340573803,
                    35.31343744877174
                ),
                (0.0, 51.61290322580615, 77.05167173252289, 48.30984349271556)
            ],
            bulk::directional_movement_system(
                &highs,
                &lows,
                &close,
                3_usize,
                crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn bulk_directional_movement_system_pma() {
        let highs = vec![
            100.83, 100.91, 101.03, 101.27, 100.52, 101.27, 101.03, 100.91, 100.83,
        ];
        let lows = vec![
            100.59, 100.72, 100.84, 100.91, 99.85, 100.91, 100.84, 100.72, 100.59,
        ];
        let close = vec![
            100.76, 100.88, 100.96, 101.14, 100.01, 101.14, 100.96, 100.88, 100.76,
        ];

        assert_eq!(
            vec![
                (
                    101.35135135135205,
                    25.675675675675546,
                    47.99680889790824,
                    33.38241876418677
                ),
                (
                    0.0,
                    51.61290322580615,
                    86.78945697046689,
                    52.614232280421646
                )
            ],
            bulk::directional_movement_system(
                &highs,
                &lows,
                &close,
                3_usize,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                }
            )
        );
    }

    #[test]
    fn bulk_directional_movement_system_median() {
        let highs = vec![
            100.83, 100.91, 101.03, 101.27, 100.52, 101.27, 101.03, 100.91, 100.83,
        ];
        let lows = vec![
            100.59, 100.72, 100.84, 100.91, 99.85, 100.91, 100.84, 100.72, 100.59,
        ];
        let close = vec![
            100.76, 100.88, 100.96, 101.14, 100.01, 101.14, 100.96, 100.88, 100.76,
        ];

        assert_eq!(
            vec![
                (
                    101.35135135135205,
                    25.675675675675546,
                    20.212765957446617,
                    34.75427030266704
                ),
                (
                    0.0,
                    51.61290322580615,
                    59.574468085106766,
                    39.89361702127669
                )
            ],
            bulk::directional_movement_system(
                &highs,
                &lows,
                &close,
                3_usize,
                crate::ConstantModelType::SimpleMovingMedian
            )
        );
    }

    #[test]
    fn bulk_directional_movement_system_mode() {
        let highs = vec![
            100.83, 100.91, 101.03, 101.27, 100.52, 101.27, 101.03, 100.91, 100.83,
        ];
        let lows = vec![
            100.59, 100.72, 100.84, 100.91, 99.85, 100.91, 100.84, 100.72, 100.59,
        ];
        let close = vec![
            100.76, 100.88, 100.96, 101.14, 100.01, 101.14, 100.96, 100.88, 100.76,
        ];

        assert_eq!(
            vec![
                (
                    101.35135135135205,
                    25.675675675675546,
                    27.666666666666668,
                    39.166666666666664
                ),
                (0.0, 51.61290322580615, 60.0, 42.0)
            ],
            bulk::directional_movement_system(
                &highs,
                &lows,
                &close,
                3_usize,
                crate::ConstantModelType::SimpleMovingMode
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_directional_movement_system_panic_high_length() {
        let highs = vec![
            100.83, 100.91, 101.03, 101.27, 100.52, 101.03, 100.91, 100.83,
        ];
        let lows = vec![
            100.59, 100.72, 100.84, 100.91, 99.85, 100.91, 100.84, 100.72, 100.59,
        ];
        let close = vec![
            100.76, 100.88, 100.96, 101.14, 100.01, 101.14, 100.96, 100.88, 100.76,
        ];

        bulk::directional_movement_system(
            &highs,
            &lows,
            &close,
            3_usize,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_directional_movement_system_panic_lows_length() {
        let highs = vec![
            100.83, 100.91, 101.03, 101.27, 100.52, 101.27, 101.03, 100.91, 100.83,
        ];
        let lows = vec![
            100.59, 100.72, 100.84, 100.91, 99.85, 100.91, 100.72, 100.59,
        ];
        let close = vec![
            100.76, 100.88, 100.96, 101.14, 100.01, 101.14, 100.96, 100.88, 100.76,
        ];

        bulk::directional_movement_system(
            &highs,
            &lows,
            &close,
            3_usize,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_directional_movement_system_panic_close_length() {
        let highs = vec![
            100.83, 100.91, 101.03, 101.27, 100.52, 101.27, 101.03, 100.91, 100.83,
        ];
        let lows = vec![
            100.59, 100.72, 100.84, 100.91, 99.85, 100.91, 100.84, 100.72, 100.59,
        ];
        let close = vec![
            100.76, 100.88, 101.14, 100.01, 101.14, 100.96, 100.88, 100.76,
        ];

        bulk::directional_movement_system(
            &highs,
            &lows,
            &close,
            3_usize,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_directional_movement_system_panic_empty() {
        let highs = Vec::new();
        let lows = Vec::new();
        let close = Vec::new();

        bulk::directional_movement_system(
            &highs,
            &lows,
            &close,
            3_usize,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_directional_movement_system_panic_period() {
        let highs = vec![
            100.83, 100.91, 101.03, 101.27, 100.52, 101.27, 101.03, 100.91,
        ];
        let lows = vec![
            100.59, 100.72, 100.84, 100.91, 99.85, 100.91, 100.84, 100.72,
        ];
        let close = vec![
            100.76, 100.88, 100.96, 101.14, 100.01, 101.14, 100.96, 100.88,
        ];

        bulk::directional_movement_system(
            &highs,
            &lows,
            &close,
            3_usize,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    fn single_volume_price_trend_no_previous() {
        assert_eq!(
            -11.379612133266974,
            single::volume_price_trend(99.01, 100.55, 743.0, 0.0)
        );
    }

    #[test]
    fn single_volume_price_trend_previous() {
        assert_eq!(
            4.023680463440446,
            single::volume_price_trend(100.43, 99.01, 1074.0, -11.379612133266974)
        );
    }

    #[test]
    fn bulk_volume_price_trend_no_previous() {
        let prices = vec![100.55, 99.01, 100.43, 101.0, 101.76];
        let volume = vec![743.0, 1074.0, 861.0, 966.0];
        assert_eq!(
            vec![
                -11.379612133266974,
                4.023680463440446,
                8.910367708287545,
                16.1792785993767
            ],
            bulk::volume_price_trend(&prices, &volume, 0.0)
        );
    }

    #[test]
    fn bulk_volume_price_trend_previous() {
        let prices = vec![100.55, 99.01, 100.43, 101.0, 101.76];
        let volume = vec![743.0, 1074.0, 861.0, 966.0];
        assert_eq!(
            vec![
                -1.3796121332669742,
                14.023680463440446,
                18.910367708287545,
                26.1792785993767
            ],
            bulk::volume_price_trend(&prices, &volume, 10.0)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_volume_price_trend_panic_length() {
        let prices = vec![100.55, 99.01, 101.0, 101.76];
        let volume = vec![743.0, 1074.0, 861.0, 966.0];
        bulk::volume_price_trend(&prices, &volume, 10.0);
    }

    #[test]
    #[should_panic]
    fn bulk_volume_price_trend_panic_volume_empty() {
        let prices = vec![100.55, 99.01, 100.43, 101.0, 101.76];
        let volume = Vec::new();
        bulk::volume_price_trend(&prices, &volume, 10.0);
    }

    #[test]
    #[should_panic]
    fn bulk_volume_price_trend_panic_prices_empty() {
        let prices = Vec::new();
        let volume = vec![743.0, 1074.0, 861.0, 966.0];
        bulk::volume_price_trend(&prices, &volume, 10.0);
    }

    #[test]
    fn single_true_strength_index_ma() {
        let prices = vec![100.14, 98.98, 99.07, 100.1, 99.96, 99.56, 100.72, 101.16];
        assert_eq!(
            0.3688989784336005,
            single::true_strength_index(
                &prices,
                crate::ConstantModelType::SimpleMovingAverage,
                5_usize,
                crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn single_true_strength_index_sma() {
        let prices = vec![100.14, 98.98, 99.07, 100.1, 99.96, 99.56, 100.72, 101.16];
        assert_eq!(
            0.5156567622865983,
            single::true_strength_index(
                &prices,
                crate::ConstantModelType::SmoothedMovingAverage,
                5_usize,
                crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn single_true_strength_index_ema() {
        let prices = vec![100.14, 98.98, 99.07, 100.1, 99.96, 99.56, 100.72, 101.16];
        assert_eq!(
            0.6031084483806584,
            single::true_strength_index(
                &prices,
                crate::ConstantModelType::ExponentialMovingAverage,
                5_usize,
                crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn single_true_strength_index_pma() {
        let prices = vec![100.14, 98.98, 99.07, 100.1, 99.96, 99.56, 100.72, 101.16];
        assert_eq!(
            0.7550056326977878,
            single::true_strength_index(
                &prices,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                },
                5_usize,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                }
            )
        );
    }

    #[test]
    fn single_true_strength_index_median() {
        let prices = vec![100.14, 98.98, 99.07, 100.1, 99.96, 99.56, 100.72, 101.16];
        assert_eq!(
            0.2249999999999778,
            single::true_strength_index(
                &prices,
                crate::ConstantModelType::SimpleMovingMedian,
                5_usize,
                crate::ConstantModelType::SimpleMovingMedian
            )
        );
    }

    #[test]
    fn single_true_strength_index_mode() {
        let prices = vec![100.14, 98.98, 99.07, 100.1, 99.96, 99.56, 100.72, 101.16];
        assert_eq!(
            0.0,
            single::true_strength_index(
                &prices,
                crate::ConstantModelType::SimpleMovingMode,
                5_usize,
                crate::ConstantModelType::SimpleMovingMode
            )
        );
    }

    #[test]
    #[should_panic]
    fn single_true_strength_index_panic_length() {
        let prices = vec![100.14, 98.98, 99.07, 100.1, 99.96];
        single::true_strength_index(
            &prices,
            crate::ConstantModelType::SimpleMovingMode,
            5_usize,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    #[should_panic]
    fn single_true_strength_index_panic_empty() {
        let prices = Vec::new();
        single::true_strength_index(
            &prices,
            crate::ConstantModelType::SimpleMovingMode,
            5_usize,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    fn bulk_true_strength_index() {
        let prices = vec![
            100.14, 98.98, 99.07, 100.1, 99.96, 99.56, 100.72, 101.16, 100.76, 100.3,
        ];
        assert_eq!(
            vec![0.6031084483806584, 0.43792017300550673, 0.06758060421426838],
            bulk::true_strength_index(
                &prices,
                crate::ConstantModelType::ExponentialMovingAverage,
                5_usize,
                crate::ConstantModelType::ExponentialMovingAverage,
                3_usize
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_true_strength_index_panic_length() {
        let prices = vec![100.14, 98.98, 99.07, 100.1, 99.96, 99.52, 101.16];
        bulk::true_strength_index(
            &prices,
            crate::ConstantModelType::SimpleMovingMode,
            5_usize,
            crate::ConstantModelType::SimpleMovingMode,
            3_usize,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_true_strength_index_panic_empty() {
        let prices = Vec::new();
        bulk::true_strength_index(
            &prices,
            crate::ConstantModelType::SimpleMovingMode,
            5_usize,
            crate::ConstantModelType::SimpleMovingMode,
            3_usize,
        );
    }
}
