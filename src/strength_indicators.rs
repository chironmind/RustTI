//! # Strength Indicators
//!
//! This module provides functions to assess the strength and conviction of price movements and trends using volume and price-based calculations.
//!
//! ## When to Use
//! Use these indicators when you want to:
//! - Compute Relative Vigor Index for market conviction analysis
//!
//! ## Structure
//! - **single**: Functions that return a single value for a slice of prices.
//! - **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
//!
//! ## Included Indicators
//!
//! ### Bulk
//!
//! - [`accumulation_distribution`](bulk::accumulation_distribution): Calculates the Accumulation/Distribution line
//! - [`positive_volume_index`](bulk::positive_volume_index): Calculates the Positive Volume Index (PVI)
//! - [`negative_volume_index`](bulk::negative_volume_index): Calculates the Negative Volume Index (NVI)
//! - [`relative_vigor_index`](bulk::relative_vigor_index): Calculates the Relative Vigor Index (RVI)
//!
//! ### Single
//!
//! - [`accumulation_distribution`](single::accumulation_distribution): Calculates the Accumulation/Distribution value
//! - [`volume_index`](single::volume_index): Generic calculation for use in PVI/NVI.
//! - [`relative_vigor_index`](single::relative_vigor_index): Calculates the Relative Vigor Index (RVI)
//!
//! ## API Details
//! - Functions in `bulk` operate on slices and return vectors of results.
//! - Functions in `single` operate on single values or slices and return a single value.
//! - See function-level docs for arguments, panics, and usage examples.
//!
//! ---

/// **single**: Functions that return a single value for a slice of prices.
pub mod single {
    use crate::basic_indicators::single::{median, mode};
    use crate::moving_average::single::moving_average;
    use crate::validation::{assert_non_empty, assert_period, assert_same_len, unsupported_type};
    use crate::{ConstantModelType, MovingAverageType};

    /// Calculates the accumulation distribution
    ///
    /// # Arguments
    ///
    /// * `high` - High
    /// * `low` - Low
    /// * `close` - Close
    /// * `volume` - Volume
    /// * `previous_accumulation_distribution` - Previous AD (0.0 if none)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let high = 103.0;
    /// let low = 99.0;
    /// let close = 102.0;
    /// let volume = 1000.0;
    /// let previous = 0.0;
    ///
    /// let accumulation_distribution =
    ///     centaur_technical_indicators::strength_indicators::single::accumulation_distribution(
    ///         high,
    ///         low,
    ///         close,
    ///         volume,
    ///         previous
    ///     );
    /// assert_eq!(500.0, accumulation_distribution);
    ///
    /// let high = 102.0;
    /// let low = 99.0;
    /// let close = 100.0;
    /// let volume = 1500.0;
    /// let accumulation_distribution =
    ///     centaur_technical_indicators::strength_indicators::single::accumulation_distribution(
    ///         high,
    ///         low,
    ///         close,
    ///         volume,
    ///         accumulation_distribution
    ///     );
    /// assert_eq!(0.0, accumulation_distribution);
    /// ```
    #[inline]
    pub fn accumulation_distribution(
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        previous_accumulation_distribution: f64,
    ) -> f64 {
        let money_flow_multiplier = ((close - low) - (high - close)) / (high - low);
        let money_flow_volume = money_flow_multiplier * volume;
        previous_accumulation_distribution + money_flow_volume
    }

    /// Calculates a generic volume_index used in Positive and Negative Volume Index
    ///
    /// # Arguments
    ///
    /// * `current_close` - Current close
    /// * `previous_close` - Previous close
    /// * `previous_volume_index` - Previous PVI/NVI value (0.0 if none)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let current_close = 105.0;
    /// let previous_close = 100.0;
    ///
    /// let volume_index = centaur_technical_indicators::strength_indicators::single::volume_index(
    ///     current_close,
    ///     previous_close,
    ///     0.0
    /// );
    ///
    /// assert_eq!(0.052500000000000005, volume_index);
    ///
    /// let next_close = 103.0;
    /// let volume_index = centaur_technical_indicators::strength_indicators::single::volume_index(
    ///     next_close,
    ///     current_close,
    ///     volume_index
    /// );
    ///
    /// assert_eq!(0.051500000000000004, volume_index);
    /// ```
    #[inline]
    pub fn volume_index(
        current_close: f64,
        previous_close: f64,
        previous_volume_index: f64,
    ) -> f64 {
        let change = (current_close - previous_close) / previous_close;
        if previous_volume_index == 0.0 {
            change + change * change
        } else {
            previous_volume_index + (change * previous_volume_index)
        }
    }

    /// Calculates the Relative Vigor Index (RVI)
    ///
    /// # Arguments
    ///
    /// * `open` - Slice of opening prices
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `close` - Slice of closing prices
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `open.len()` != `high.len()` != `low,len()` != `close.len()`
    ///     * `open.is_empty()`
    ///     * `open.len()` < 4
    ///
    /// # Examples
    ///
    /// ```rust
    /// let open = vec![95.0, 105.0, 110.0, 115.0, 120.0, 115.0, 110.0, 100.0];
    /// let high = vec![110.0, 115.0, 120.0, 130.0, 135.0, 130.0, 120.0, 105.0];
    /// let low = vec![90.0, 110.0, 105.0, 110.0, 120.0, 105.0, 95.0, 85.0];
    /// let close = vec![100.0, 115.0, 115.0, 120.0, 125.0, 110.0, 100.0, 90.0];
    ///
    /// let relative_vigor_index =
    ///     centaur_technical_indicators::strength_indicators::single::relative_vigor_index(
    ///         &open,
    ///         &high,
    ///         &low,
    ///         &close,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingAverage
    ///     );
    ///
    /// assert_eq!(0.10185185185185186, relative_vigor_index);
    /// ```
    pub fn relative_vigor_index(
        open: &[f64],
        high: &[f64],
        low: &[f64],
        close: &[f64],
        constant_model_type: ConstantModelType,
    ) -> f64 {
        let length = open.len();
        if length != high.len() || length != low.len() || length != close.len() {
            panic!(
                "Length of open ({}), high ({}), low ({}), and close ({}) must be equal",
                length,
                high.len(),
                low.len(),
                close.len()
            )
        };
        assert_non_empty("open", open);
if length < 4 {
            panic!("Prices must be at least 4 in length")
        };

        let mut close_open_diff = Vec::with_capacity(length);
        let mut high_low_diff = Vec::with_capacity(length);

        for i in 0..length {
            close_open_diff.push(close[i] - open[i]);
            high_low_diff.push(high[i] - low[i]);
        }

        let mut numerator = Vec::with_capacity(length - 3);
        let mut denominator = Vec::with_capacity(length - 3);

        for i in 3..length {
            numerator.push(
                (close_open_diff[i]
                    + (2.0 * close_open_diff[i - 1])
                    + (2.0 * close_open_diff[i - 2])
                    + close_open_diff[i - 3])
                    / 6.0,
            );
            denominator.push(
                (high_low_diff[i]
                    + (2.0 * high_low_diff[i - 1])
                    + (2.0 * high_low_diff[i - 2])
                    + high_low_diff[i - 3])
                    / 6.0,
            );
        }

        let (smoothed_numerator, smoothed_denominator) = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => (
                moving_average(&numerator, MovingAverageType::Simple),
                moving_average(&denominator, MovingAverageType::Simple),
            ),
            ConstantModelType::SmoothedMovingAverage => (
                moving_average(&numerator, MovingAverageType::Smoothed),
                moving_average(&denominator, MovingAverageType::Smoothed),
            ),
            ConstantModelType::ExponentialMovingAverage => (
                moving_average(&numerator, MovingAverageType::Exponential),
                moving_average(&denominator, MovingAverageType::Exponential),
            ),
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => (
                moving_average(
                    &numerator,
                    MovingAverageType::Personalised {
                        alpha_num,
                        alpha_den,
                    },
                ),
                moving_average(
                    &denominator,
                    MovingAverageType::Personalised {
                        alpha_num,
                        alpha_den,
                    },
                ),
            ),
            ConstantModelType::SimpleMovingMedian => (median(&numerator), median(&denominator)),
            ConstantModelType::SimpleMovingMode => (mode(&numerator), mode(&denominator)),
            _ => unsupported_type("ConstantModelType"),
        };

        smoothed_numerator / smoothed_denominator
    }
}

/// **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
pub mod bulk {
    use crate::strength_indicators::single;
    use crate::validation::{assert_non_empty, assert_period, assert_same_len};
    use crate::ConstantModelType;

    /// Calculates the accumulation distribution
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `close` - Slice of closing prices
    /// * `volumes` - Slice of volumes
    /// * `previous_accumulation_distribution` - Previous AD (0.0 if none)
    ///
    /// # Returns
    ///
    /// A vector of calculated values
    ///
    /// # Panics
    ///
    /// Panics if `high.len()` != `low.len()` != `close.len()` != `volumes.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let high = vec![103.0, 102.0, 105.0];
    /// let low = vec![99.0, 99.0, 100.0];
    /// let close = vec![102.0, 100.0, 103.0];
    /// let volume = vec![1000.0, 1500.0, 1200.0];
    /// let previous = 0.0;
    ///
    /// let accumulation_distribution =
    ///     centaur_technical_indicators::strength_indicators::bulk::accumulation_distribution(
    ///         &high,
    ///         &low,
    ///         &close,
    ///         &volume,
    ///         previous
    ///     );
    /// assert_eq!(vec![500.0, 0.0, 240.0], accumulation_distribution);
    /// ```
    #[inline]
    pub fn accumulation_distribution(
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
        previous_accumulation_distribution: f64,
    ) -> Vec<f64> {
        let length = close.len();
        if length != high.len() || length != close.len() || length != volume.len() {
            panic!("Length of close prices ({}) must match length of high ({}), low ({}), and volume ({})", length, high.len(), close.len(), volume.len());
        };
        let mut ads = Vec::with_capacity(length);
        let mut ad = single::accumulation_distribution(
            high[0],
            low[0],
            close[0],
            volume[0],
            previous_accumulation_distribution,
        );
        ads.push(ad);
        for i in 1..length {
            ad = single::accumulation_distribution(high[i], low[i], close[i], volume[i], ad);
            ads.push(ad);
        }
        ads
    }

    /// Calculates the Positive Volume Index (PVI)
    ///
    /// # Arguments
    ///
    /// * `close` - Slice of closing prices
    /// * `volume` - Slice of volumes
    /// * `previous_positive_volume_index` - Previous PVI (0.0 if none)
    ///
    /// # Returns
    ///
    /// A vector of calculated values
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `close.len()` != `volume.len()`
    ///     * `close.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let close = vec![100.0, 115.0, 118.0, 120.0, 125.0];
    /// let volume = vec![1000.0, 1200.0, 1300.0, 1100.0, 1100.0];
    ///
    /// let positive_volume_index =
    ///     centaur_technical_indicators::strength_indicators::bulk::positive_volume_index(
    ///         &close,
    ///         &volume,
    ///         0.0
    ///     );
    ///
    /// assert_eq!(vec![0.1725, 0.177, 0.177, 0.177], positive_volume_index);
    ///
    /// let next_close = vec![125.0, 122.0, 115.0, 120.0];
    /// let next_volume = vec![1100.0, 1000.0, 1500.0, 1600.0];
    ///
    /// let positive_volume_index =
    ///     centaur_technical_indicators::strength_indicators::bulk::positive_volume_index(
    ///         &next_close,
    ///         &next_volume,
    ///         *positive_volume_index.last().unwrap()
    ///     );
    ///
    /// assert_eq!(vec![0.177, 0.16684426229508195, 0.1740983606557377], positive_volume_index);
    /// ```
    #[inline]
    pub fn positive_volume_index(
        close: &[f64],
        volume: &[f64],
        previous_positive_volume_index: f64,
    ) -> Vec<f64> {
        let length = close.len();
        if length != volume.len() {
            panic!(
                "Length of close ({}) and volume ({}) need to be equal",
                length,
                volume.len()
            )
        };
        assert_non_empty("close", close);
let mut pvis = Vec::with_capacity(length - 1);
        let mut prev = previous_positive_volume_index;

        for i in 1..length {
            if volume[i] > volume[i - 1] {
                prev = single::volume_index(close[i], close[i - 1], prev);
            }
            pvis.push(prev);
        }
        pvis
    }

    /// Calculates the Negative Volume Index (NVI)
    ///
    /// # Arguments
    ///
    /// * `close` - Slice of closing prices
    /// * `volume` - Slice of volumes
    /// * `previous_negative_volume_index` - Previous NVI (0.0 if none)
    ///
    /// # Returns
    ///
    /// A vector of calculated values
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `close.len()` != `volume.len()`
    ///     * `close.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let close = vec![100.0, 115.0, 118.0, 120.0, 125.0];
    /// let volume = vec![1000.0, 1200.0, 1300.0, 1100.0, 1100.0];
    ///
    /// let negative_volume_index =
    ///     centaur_technical_indicators::strength_indicators::bulk::negative_volume_index(
    ///         &close,
    ///         &volume,
    ///         0.0
    ///     );
    ///
    /// assert_eq!(
    ///     vec![0.0, 0.0, 0.017236426314277506, 0.017236426314277506],
    ///     negative_volume_index
    /// );
    ///
    ///
    /// let next_close = vec![125.0, 122.0, 115.0, 120.0];
    /// let next_volume = vec![1100.0, 1000.0, 1500.0, 1600.0];
    ///
    /// let negative_volume_index =
    ///     centaur_technical_indicators::strength_indicators::bulk::negative_volume_index(
    ///         &next_close,
    ///         &next_volume,
    ///         *negative_volume_index.last().unwrap()
    ///     );
    ///
    /// assert_eq!(
    ///     vec![0.016822752082734847, 0.016822752082734847, 0.016822752082734847],
    ///     negative_volume_index
    /// );
    /// ```
    #[inline]
    pub fn negative_volume_index(
        close: &[f64],
        volume: &[f64],
        previous_negative_volume_index: f64,
    ) -> Vec<f64> {
        let length = close.len();
        if length != volume.len() {
            panic!(
                "Length of close ({}) and volume ({}) need to be equal",
                length,
                volume.len()
            )
        };
        assert_non_empty("close", close);
let mut nvis = Vec::with_capacity(length - 1);
        let mut prev = previous_negative_volume_index;

        for i in 1..length {
            if volume[i] < volume[i - 1] {
                prev = single::volume_index(close[i], close[i - 1], prev);
            }
            nvis.push(prev);
        }
        nvis
    }

    /// Calculates the Relative Vigor Index (RVI)
    ///
    /// # Arguments
    ///
    /// * `open` - Slice of opening prices
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `close` - Slice of closing prices
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    /// * `period` - Period over which to calculate the RVI
    ///
    /// # Returns
    ///
    /// A vector of calculated values
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `open.len()` != `high.len()` != `low.len()` != `close.len()`
    ///     * `open.is_empty()`
    ///     * `period` > lengths
    ///     * `period` < 4
    ///
    /// # Examples
    ///
    /// ```rust
    /// let open = vec![95.0, 105.0, 110.0, 115.0, 120.0, 115.0, 110.0, 100.0, 90.0, 100.0];
    /// let high = vec![110.0, 115.0, 120.0, 130.0, 135.0, 130.0, 120.0, 105.0, 100.0, 120.0];
    /// let low = vec![90.0, 110.0, 105.0, 110.0, 120.0, 105.0, 95.0, 85.0, 70.0, 85.0];
    /// let close = vec![100.0, 115.0, 115.0, 120.0, 125.0, 110.0, 100.0, 90.0, 80.0, 110.0];
    /// let period: usize = 8;
    ///
    /// let relative_vigor_index = centaur_technical_indicators::strength_indicators::bulk::relative_vigor_index(
    ///     &open,
    ///     &high,
    ///     &low,
    ///     &close,
    ///     centaur_technical_indicators::ConstantModelType::SimpleMovingAverage,
    ///     period
    /// );
    ///
    /// assert_eq!(vec![0.10185185185185186, -0.06611570247933886, -0.17037037037037037], relative_vigor_index);
    /// ```
    #[inline]
    pub fn relative_vigor_index(
        open: &[f64],
        high: &[f64],
        low: &[f64],
        close: &[f64],
        constant_model_type: ConstantModelType,
        period: usize,
    ) -> Vec<f64> {
        let length = open.len();
        if length != high.len() || length != low.len() || length != close.len() {
            panic!(
                "Length of open ({}), high ({}), low ({}), and close ({}) must be equal",
                length,
                high.len(),
                low.len(),
                close.len()
            )
        };
        assert_non_empty("open", open);
if length < period {
            panic!(
                "Period ({}) less than or equal to length of prices ({})",
                period, length
            )
        };
        if period < 4 {
            panic!("Period ({}) needs to be greater or equal to 4", period)
        };

        let loop_max = length - period + 1;
        let mut rvis = Vec::with_capacity(loop_max);
        for i in 0..loop_max {
            rvis.push(single::relative_vigor_index(
                &open[i..i + period],
                &high[i..i + period],
                &low[i..i + period],
                &close[i..i + period],
                constant_model_type,
            ));
        }
        rvis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_accumulation_distribution_no_previous() {
        assert_eq!(
            -38.28571428571309,
            single::accumulation_distribution(100.53, 99.62, 100.01, 268.0, 0.0)
        )
    }

    #[test]
    fn single_accumulation_distribution_previous() {
        assert_eq!(
            0.6342857142869107,
            single::accumulation_distribution(100.53, 99.62, 100.01, 268.0, 38.92)
        )
    }

    #[test]
    fn bulk_accumulation_distribution_no_previous() {
        let highs = vec![100.53, 100.68];
        let lows = vec![99.62, 99.97];
        let close = vec![100.01, 100.44];
        let volume = vec![268.0, 319.0];
        assert_eq!(
            vec![-38.28571428571309, 65.05231388329526],
            bulk::accumulation_distribution(&highs, &lows, &close, &volume, 0.0)
        );
    }

    #[test]
    fn bulk_accumulation_distribution_previous() {
        let highs = vec![100.53, 100.68];
        let lows = vec![99.62, 99.97];
        let close = vec![100.01, 100.44];
        let volume = vec![268.0, 319.0];
        assert_eq!(
            vec![0.6342857142869107, 103.97231388329524],
            bulk::accumulation_distribution(&highs, &lows, &close, &volume, 38.92)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_accumulation_distribution_panic_high_length() {
        let highs = vec![100.53];
        let lows = vec![99.62, 99.97];
        let close = vec![100.01, 100.44];
        let volume = vec![268.0, 319.0];
        bulk::accumulation_distribution(&highs, &lows, &close, &volume, 0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_accumulation_distribution_panic_low_length() {
        let highs = vec![100.53, 100.68];
        let lows = vec![99.62];
        let close = vec![100.01, 100.44];
        let volume = vec![268.0, 319.0];
        bulk::accumulation_distribution(&highs, &lows, &close, &volume, 0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_accumulation_distribution_panic_close_length() {
        let highs = vec![100.53, 100.68];
        let lows = vec![99.62, 99.97];
        let close = vec![100.01];
        let volume = vec![268.0, 319.0];
        bulk::accumulation_distribution(&highs, &lows, &close, &volume, 0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_accumulation_distribution_panic_volume_length() {
        let highs = vec![100.53, 100.68];
        let lows = vec![99.62, 99.97];
        let close = vec![100.01, 100.44];
        let volume = vec![268.0];
        bulk::accumulation_distribution(&highs, &lows, &close, &volume, 0.0);
    }

    #[test]
    fn single_volume_index_no_previous() {
        assert_eq!(
            0.004318056345550251,
            single::volume_index(100.44, 100.01, 0.0)
        );
    }

    #[test]
    fn single_volume_index_previous() {
        assert_eq!(
            0.004324075141730827,
            single::volume_index(100.58, 100.44, 0.004318056345550251)
        );
    }

    #[test]
    fn bulk_positive_volume_index_no_previous() {
        let close = vec![100.14, 98.98, 99.07, 100.1];
        let volume = vec![1000.0, 1200.0, 1300.0, 1100.0];
        assert_eq!(
            vec![
                -0.011449598682475618,
                -0.011460009511748427,
                -0.011460009511748427
            ],
            bulk::positive_volume_index(&close, &volume, 0.0)
        );
    }

    #[test]
    fn bulk_positive_volume_index_previous() {
        let close = vec![100.1, 99.96, 99.56, 100.72, 101.16];
        let volume = vec![1100.0, 1100.0, 1000.0, 1500.0, 1600.0];
        assert_eq!(
            vec![
                -0.011460009511748427,
                -0.011460009511748427,
                -0.011593533125987359,
                -0.011644180014146955
            ],
            bulk::positive_volume_index(&close, &volume, -0.011460009511748427)
        );
    }

    #[test]
    fn bulk_positive_volume_index_all_positive() {
        let close = vec![100.14, 98.98, 99.07, 100.1];
        let volume = vec![1000.0, 1200.0, 1300.0, 1400.0];
        assert_eq!(
            vec![
                -0.011449598682475618,
                -0.011460009511748427,
                -0.011579155668981706
            ],
            bulk::positive_volume_index(&close, &volume, 0.0)
        );
    }

    #[test]
    fn bulk_positive_volume_index_all_negative() {
        let close = vec![100.14, 98.98, 99.07, 100.1];
        let volume = vec![1000.0, 900.0, 800.0, 700.0];
        assert_eq!(
            vec![0.0, 0.0, 0.0],
            bulk::positive_volume_index(&close, &volume, 0.0)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_positive_volume_index_panic_length() {
        let close = vec![100.14, 98.98, 100.1];
        let volume = vec![1000.0, 900.0, 800.0, 700.0];
        bulk::positive_volume_index(&close, &volume, 0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_positive_volume_index_panic_empty() {
        let close = Vec::new();
        let volume = Vec::new();
        bulk::positive_volume_index(&close, &volume, 0.0);
    }

    #[test]
    fn bulk_negative_volume_index_no_previous() {
        let close = vec![100.14, 98.98, 99.07, 100.1];
        let volume = vec![1000.0, 1200.0, 1300.0, 1100.0];
        assert_eq!(
            vec![0.0, 0.0, 0.010504780356171802],
            bulk::negative_volume_index(&close, &volume, 0.0)
        );
    }

    #[test]
    fn bulk_negative_volume_index_previous() {
        let close = vec![100.1, 99.96, 99.56, 100.72, 101.16];
        let volume = vec![1100.0, 1100.0, 1000.0, 1500.0, 1600.0];
        assert_eq!(
            vec![
                0.010504780356171802,
                0.010462744420372797,
                0.010462744420372797,
                0.010462744420372797
            ],
            bulk::negative_volume_index(&close, &volume, 0.010504780356171802)
        );
    }

    #[test]
    fn bulk_negative_volume_index_all_positive() {
        let close = vec![100.14, 98.98, 99.07, 100.1];
        let volume = vec![1000.0, 1200.0, 1300.0, 1400.0];
        assert_eq!(
            vec![0.0, 0.0, 0.0],
            bulk::negative_volume_index(&close, &volume, 0.0)
        );
    }

    #[test]
    fn bulk_negative_volume_index_all_negative() {
        let close = vec![100.14, 98.98, 99.07, 100.1];
        let volume = vec![1000.0, 900.0, 800.0, 700.0];
        assert_eq!(
            vec![
                -0.011449598682475618,
                -0.011460009511748427,
                -0.011579155668981706
            ],
            bulk::negative_volume_index(&close, &volume, 0.0)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_negative_volume_index_panic_length() {
        let close = vec![100.14, 98.98, 100.1];
        let volume = vec![1000.0, 900.0, 800.0, 700.0];
        bulk::negative_volume_index(&close, &volume, 0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_negative_volume_index_panic_empty() {
        let close = Vec::new();
        let volume = Vec::new();
        bulk::negative_volume_index(&close, &volume, 0.0);
    }

    #[test]
    fn single_relative_vigor_index_ma() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01, 102.75];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03];
        assert_eq!(
            0.2063784115302081,
            single::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn single_relative_vigor_index_sma() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01, 102.75];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03];
        assert_eq!(
            0.2424082260234505,
            single::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn single_relative_vigor_index_ema() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01, 102.75];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03];
        assert_eq!(
            0.2635196472571675,
            single::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn single_relative_vigor_index_pma() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01, 102.75];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03];
        assert_eq!(
            0.29194621866781373,
            single::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                }
            )
        );
    }

    #[test]
    fn single_relative_vigor_index_median() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01, 102.75];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03];
        assert_eq!(
            0.24558139534884124,
            single::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                crate::ConstantModelType::SimpleMovingMedian
            )
        );
    }

    #[test]
    fn single_relative_vigor_index_mode() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01, 102.75];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03];
        assert_eq!(
            0.0,
            single::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                crate::ConstantModelType::SimpleMovingMode
            )
        );
    }

    #[test]
    fn single_relative_vigor_index_minimum() {
        let open = vec![100.73, 99.62, 99.82, 100.38];
        let high = vec![102.32, 100.69, 100.83, 101.73];
        let low = vec![100.14, 98.98, 99.07, 100.1];
        let close = vec![100.55, 99.01, 100.43, 101.0];
        assert_eq!(
            0.04093023255814197,
            single::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    #[should_panic]
    fn single_relative_vigor_index_panic_length_open() {
        let open = vec![100.73, 99.62, 99.82];
        let high = vec![102.32, 100.69, 100.83, 101.73];
        let low = vec![100.14, 98.98, 99.07, 100.1];
        let close = vec![100.55, 99.01, 100.43, 101.0];
        single::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            crate::ConstantModelType::SimpleMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn single_relative_vigor_index_panic_length_high() {
        let open = vec![100.73, 99.62, 99.82, 100.38];
        let high = vec![102.32, 100.69, 100.83];
        let low = vec![100.14, 98.98, 99.07, 100.1];
        let close = vec![100.55, 99.01, 100.43, 101.0];
        single::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            crate::ConstantModelType::SimpleMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn single_relative_vigor_index_panic_length_low() {
        let open = vec![100.73, 99.62, 99.82, 100.38];
        let high = vec![102.32, 100.69, 100.83, 101.73];
        let low = vec![100.14, 98.98, 100.1];
        let close = vec![100.55, 99.01, 100.43, 101.0];
        single::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            crate::ConstantModelType::SimpleMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn single_relative_vigor_index_panic_length_close() {
        let open = vec![100.73, 99.62, 99.82, 100.38];
        let high = vec![102.32, 100.69, 100.83, 101.73];
        let low = vec![100.14, 98.98, 99.07, 100.1];
        let close = vec![100.55, 99.01, 100.40];
        single::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            crate::ConstantModelType::SimpleMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn single_relative_vigor_index_panic_length_overall() {
        let open = vec![100.73, 99.62, 99.82];
        let high = vec![102.32, 100.69, 100.83];
        let low = vec![100.14, 98.98, 99.07];
        let close = vec![100.55, 99.01, 100.43];
        single::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            crate::ConstantModelType::SimpleMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn single_relative_vigor_index_panic_empty() {
        let open = Vec::new();
        let high = Vec::new();
        let low = Vec::new();
        let close = Vec::new();
        single::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            crate::ConstantModelType::SimpleMovingAverage,
        );
    }

    #[test]
    fn bulk_relative_vigor_index_sinlgle() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01, 102.75];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03];
        assert_eq!(
            vec![0.2063784115302081],
            bulk::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                crate::ConstantModelType::SimpleMovingAverage,
                6_usize
            )
        );
    }

    #[test]
    fn bulk_relative_vigor_index_ma() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81, 101.85, 102.09];
        let high = vec![
            102.32, 100.69, 100.83, 101.73, 102.01, 102.75, 103.04, 102.94,
        ];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55, 101.17, 100.38];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03, 102.35, 101.51];
        assert_eq!(
            vec![0.2063784115302081, 0.27849970466627455, 0.23398946492930492],
            bulk::relative_vigor_index(
                &open,
                &high,
                &low,
                &close,
                crate::ConstantModelType::SimpleMovingAverage,
                6_usize
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_relative_vigor_index_panic_length_open() {
        let open = vec![100.73, 99.62, 99.82, 100.97, 101.81, 101.85, 102.09];
        let high = vec![
            102.32, 100.69, 100.83, 101.73, 102.01, 102.75, 103.04, 102.94,
        ];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55, 101.17, 100.38];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03, 102.35, 101.51];
        bulk::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            crate::ConstantModelType::SimpleMovingAverage,
            6_usize,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_relative_vigor_index_panic_length_high() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81, 101.85, 102.09];
        let high = vec![102.32, 100.69, 100.83, 102.01, 102.75, 103.04, 102.94];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55, 101.17, 100.38];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03, 102.35, 101.51];
        bulk::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            crate::ConstantModelType::SimpleMovingAverage,
            6_usize,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_relative_vigor_index_panic_length_low() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81, 101.85, 102.09];
        let high = vec![
            102.32, 100.69, 100.83, 101.73, 102.01, 102.75, 103.04, 102.94,
        ];
        let low = vec![18.98, 99.07, 100.1, 99.96, 100.55, 101.17, 100.38];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03, 102.35, 101.51];
        bulk::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            crate::ConstantModelType::SimpleMovingAverage,
            6_usize,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_relative_vigor_index_panic_length_close() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81, 101.85, 102.09];
        let high = vec![
            102.32, 100.69, 100.83, 101.73, 102.01, 102.75, 103.04, 102.94,
        ];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55, 101.17, 100.38];
        let close = vec![100.55, 99.01, 100.43, 101.76, 102.03, 102.35, 101.51];
        bulk::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            crate::ConstantModelType::SimpleMovingAverage,
            6_usize,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_relative_vigor_index_panic_period_high() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81, 101.85, 102.09];
        let high = vec![
            102.32, 100.69, 100.83, 101.73, 102.01, 102.75, 103.04, 102.94,
        ];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55, 101.17, 100.38];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03, 102.35, 101.51];
        bulk::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            crate::ConstantModelType::SimpleMovingAverage,
            60_usize,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_relative_vigor_index_panic_period_low() {
        let open = vec![100.73, 99.62, 99.82, 100.38, 100.97, 101.81, 101.85, 102.09];
        let high = vec![
            102.32, 100.69, 100.83, 101.73, 102.01, 102.75, 103.04, 102.94,
        ];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96, 100.55, 101.17, 100.38];
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76, 102.03, 102.35, 101.51];
        bulk::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            crate::ConstantModelType::SimpleMovingAverage,
            3_usize,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_relative_vigor_index_panic_empty() {
        let open = Vec::new();
        let high = Vec::new();
        let low = Vec::new();
        let close = Vec::new();
        bulk::relative_vigor_index(
            &open,
            &high,
            &low,
            &close,
            crate::ConstantModelType::SimpleMovingAverage,
            6_usize,
        );
    }
}
