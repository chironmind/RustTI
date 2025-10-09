//! # Correlation Indicators
//!
//! The `correlation_indicators` module provides functions to measure the co-movement
//! and statistical relationship between two different price series or assets.
//!
//! ## When to Use
//! Use correlation indicators when you want to:
//! - Quantify how closely two assets move together
//! - Assess diversification or hedging effectiveness
//! - Explore relationships between assets
//!
//! ## Structure
//! - **single**: Functions that return a single value for a slice of prices.
//! - **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
//!
//! ## Included Indicators
//!
//! ### Bulk
//! - [`correlate_asset_prices`](bulk::correlate_asset_prices): Correlation between two price series
//!
//! ### Single
//! - [`correlate_asset_prices`](single::correlate_asset_prices): Correlation between two price series
//!
//! ## API Details
//! - All functions require two slices of `f64` prices (for the two assets).
//! - See each function for further details, panics, and usage examples.
//!
//! ---

/// **single**: Functions that return a single value for a slice of prices
pub mod single {
    use crate::basic_indicators::single::{absolute_deviation, median, mode, standard_deviation};
    use crate::moving_average::single::moving_average;
    use crate::volatility_indicators::single::ulcer_index;
    use crate::{
        AbsDevConfig, CentralPoint, ConstantModelType, DeviationAggregate, DeviationModel,
        MovingAverageType,
    };

    /// Calculates the correlation between two assets prices.
    ///
    /// # Arguments
    ///
    /// * `prices_asset_a` - Slice of prices
    /// * `prices_asset_b` - Slice of prices
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    /// * `deviation_model` - Variant of [`DeviationModel`]
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * `prices_asset_a.is_empty()` or `prices_asset_b.is_empty()`
    /// * `prices_asset_a.len()` != `prices_asset_b.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices_a = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let prices_b = vec![200.0, 204.0, 206.0, 202.0, 198.0];
    ///
    /// let correlation =
    ///     rust_ti::correlation_indicators::single::correlate_asset_prices(
    ///         &prices_a,
    ///         &prices_b,
    ///         rust_ti::ConstantModelType::SimpleMovingAverage,
    ///         rust_ti::DeviationModel::StandardDeviation
    ///     );
    /// // This should be 1.0 but due to how Rust calculates floats, this is close as we get
    /// assert_eq!(0.9999999999999998, correlation);
    ///
    /// let correlation =
    ///     rust_ti::correlation_indicators::single::correlate_asset_prices(
    ///         &prices_a,
    ///         &prices_b,
    ///         rust_ti::ConstantModelType::ExponentialMovingAverage,
    ///         rust_ti::DeviationModel::UlcerIndex);
    /// assert_eq!(1.1410137845061807, correlation);
    /// ```
    pub fn correlate_asset_prices(
        prices_asset_a: &[f64],
        prices_asset_b: &[f64],
        constant_model_type: ConstantModelType,
        deviation_model: DeviationModel,
    ) -> f64 {
        if prices_asset_a.is_empty() || prices_asset_b.is_empty() {
            panic!("Prices cannot be empty")
        };

        let length = prices_asset_a.len();
        if length != prices_asset_b.len() {
            panic!(
                "Length of asset a ({}) must match length of asset b ({})",
                length,
                prices_asset_b.len()
            )
        };

        let asset_a_average = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(prices_asset_a, MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(prices_asset_a, MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(prices_asset_a, MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => moving_average(
                prices_asset_a,
                MovingAverageType::Personalised {
                    alpha_num,
                    alpha_den,
                },
            ),
            ConstantModelType::SimpleMovingMedian => median(prices_asset_a),
            ConstantModelType::SimpleMovingMode => mode(prices_asset_a),
            _ => panic!("Unsupported ConstantModelType"),
        };

        let asset_b_average = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(prices_asset_b, MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(prices_asset_b, MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(prices_asset_b, MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => moving_average(
                prices_asset_b,
                MovingAverageType::Personalised {
                    alpha_num,
                    alpha_den,
                },
            ),
            ConstantModelType::SimpleMovingMedian => median(prices_asset_b),
            ConstantModelType::SimpleMovingMode => mode(prices_asset_b),
            _ => panic!("Unsupported ConstantModelType"),
        };

        let joint_average_return: f64 = (0..length)
            .map(|i| (prices_asset_a[i] - asset_a_average) * (prices_asset_b[i] - asset_b_average))
            .sum();

        let covariance = joint_average_return / length as f64;

        let asset_a_deviation = match deviation_model {
            DeviationModel::StandardDeviation => standard_deviation(prices_asset_a),
            DeviationModel::MeanAbsoluteDeviation => absolute_deviation(
                prices_asset_a,
                AbsDevConfig {
                    center: CentralPoint::Mean,
                    aggregate: DeviationAggregate::Mean,
                },
            ),
            DeviationModel::MedianAbsoluteDeviation => absolute_deviation(
                prices_asset_a,
                AbsDevConfig {
                    center: CentralPoint::Median,
                    aggregate: DeviationAggregate::Median,
                },
            ),
            DeviationModel::ModeAbsoluteDeviation => absolute_deviation(
                prices_asset_a,
                AbsDevConfig {
                    center: CentralPoint::Mode,
                    aggregate: DeviationAggregate::Mode,
                },
            ),
            DeviationModel::UlcerIndex => ulcer_index(prices_asset_a),
            _ => panic!("Unsupported DeviationModel"),
        };

        let asset_b_deviation = match deviation_model {
            DeviationModel::StandardDeviation => standard_deviation(prices_asset_b),
            DeviationModel::MeanAbsoluteDeviation => absolute_deviation(
                prices_asset_b,
                AbsDevConfig {
                    center: CentralPoint::Mean,
                    aggregate: DeviationAggregate::Mean,
                },
            ),
            DeviationModel::MedianAbsoluteDeviation => absolute_deviation(
                prices_asset_b,
                AbsDevConfig {
                    center: CentralPoint::Median,
                    aggregate: DeviationAggregate::Median,
                },
            ),
            DeviationModel::ModeAbsoluteDeviation => absolute_deviation(
                prices_asset_b,
                AbsDevConfig {
                    center: CentralPoint::Mode,
                    aggregate: DeviationAggregate::Mode,
                },
            ),
            DeviationModel::UlcerIndex => ulcer_index(prices_asset_b),
            _ => panic!("Unsupported DeviationModel"),
        };
        covariance / (asset_a_deviation * asset_b_deviation)
    }
}

/// **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
pub mod bulk {
    use crate::correlation_indicators::single;
    use crate::{ConstantModelType, DeviationModel};

    /// Calculates the correlation between two asset prices over a period
    ///
    /// # Arguments
    ///
    /// * `prices_asset_a` - Slice of prices
    /// * `prices_asset_b` - Slice of prices
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    /// * `deviation_model` - Variant of [`DeviationModel`]
    /// * `period`: Period over which to calculate the correlation
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `prices_asset_a.len()` != `prices_asset_b.len()`
    ///     * `period` > `slices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices_a = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0];
    /// let prices_b = vec![200.0, 204.0, 206.0, 202.0, 198.0, 193.0, 189.0];
    /// let period: usize = 5;
    ///
    /// let correlation =
    ///     rust_ti::correlation_indicators::bulk::correlate_asset_prices(
    ///         &prices_a,
    ///         &prices_b,
    ///         rust_ti::ConstantModelType::SimpleMovingAverage,
    ///         rust_ti::DeviationModel::StandardDeviation,
    ///         period
    ///     );
    /// // The first result  should be 1.0 but due to how Rust calculates floats, this is close as we get
    /// assert_eq!(vec![0.9999999999999998, 0.9340577351598457, 0.34094365457352693], correlation);
    ///
    /// let correlation =
    ///     rust_ti::correlation_indicators::bulk::correlate_asset_prices(
    ///         &prices_a,
    ///         &prices_b,
    ///         rust_ti::ConstantModelType::ExponentialMovingAverage,
    ///         rust_ti::DeviationModel::UlcerIndex,
    ///         period
    ///     );
    /// assert_eq!(vec![1.1410137845061807, 0.9904422924841779, 0.2785701491571082], correlation);
    /// ```
    #[inline]
    pub fn correlate_asset_prices(
        prices_asset_a: &[f64],
        prices_asset_b: &[f64],
        constant_model_type: ConstantModelType,
        deviation_model: DeviationModel,
        period: usize,
    ) -> Vec<f64> {
        let length = prices_asset_a.len();
        if length != prices_asset_b.len() {
            panic!(
                "Length of asset a ({}) must match length of asset b ({})",
                length,
                prices_asset_b.len()
            )
        };
        if period > length {
            panic!(
                "Period ({}) cannot be longer than length of prices ({})",
                period, length
            )
        };

        (0..=length - period)
            .map(|i| {
                single::correlate_asset_prices(
                    &prices_asset_a[i..i + period],
                    &prices_asset_b[i..i + period],
                    constant_model_type,
                    deviation_model,
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_correlation_ma_std_dev() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92];
        assert_eq!(
            0.9042213658878326,
            single::correlate_asset_prices(
                &prices_a,
                &prices_b,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::StandardDeviation
            )
        );
    }

    #[test]
    fn single_correlation_sma_std_dev() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92];
        assert_eq!(
            0.9791080346628417,
            single::correlate_asset_prices(
                &prices_a,
                &prices_b,
                crate::ConstantModelType::SmoothedMovingAverage,
                crate::DeviationModel::StandardDeviation
            )
        );
    }

    #[test]
    fn single_correlation_ema_std_dev() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92];
        assert_eq!(
            1.121845018991745,
            single::correlate_asset_prices(
                &prices_a,
                &prices_b,
                crate::ConstantModelType::ExponentialMovingAverage,
                crate::DeviationModel::StandardDeviation
            )
        );
    }

    #[test]
    fn single_correlation_pma_std_dev() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92];
        assert_eq!(
            1.468978482735914,
            single::correlate_asset_prices(
                &prices_a,
                &prices_b,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                },
                crate::DeviationModel::StandardDeviation
            )
        );
    }

    #[test]
    fn single_correlation_median_std_dev() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92];
        assert_eq!(
            0.8763922185678863,
            single::correlate_asset_prices(
                &prices_a,
                &prices_b,
                crate::ConstantModelType::SimpleMovingMedian,
                crate::DeviationModel::StandardDeviation
            )
        );
    }

    #[test]
    fn single_correlation_mode_std_dev() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92];
        assert_eq!(
            0.8439113000163907,
            single::correlate_asset_prices(
                &prices_a,
                &prices_b,
                crate::ConstantModelType::SimpleMovingMode,
                crate::DeviationModel::StandardDeviation
            )
        );
    }

    #[test]
    fn single_correlation_ma_mean_ad_dev() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92];
        assert_eq!(
            1.1165699299573548,
            single::correlate_asset_prices(
                &prices_a,
                &prices_b,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::MeanAbsoluteDeviation
            )
        );
    }

    #[test]
    fn single_correlation_ma_median_ad_dev() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92];
        assert_eq!(
            0.8918502283104672,
            single::correlate_asset_prices(
                &prices_a,
                &prices_b,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::MedianAbsoluteDeviation
            )
        );
    }

    #[test]
    fn single_correlation_ma_mode_ad_dev() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92];
        assert_eq!(
            f64::INFINITY,
            single::correlate_asset_prices(
                &prices_a,
                &prices_b,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::ModeAbsoluteDeviation
            )
        );
    }

    #[test]
    fn single_correlation_ulcer_index_ad_dev() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92];
        assert_eq!(
            0.23702330943345767,
            single::correlate_asset_prices(
                &prices_a,
                &prices_b,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::UlcerIndex
            )
        );
    }

    #[test]
    #[should_panic]
    fn single_correlation_empty_a_panic() {
        let prices_a = vec![];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92];
        single::correlate_asset_prices(
            &prices_a,
            &prices_b,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::DeviationModel::StandardDeviation,
        );
    }

    #[test]
    #[should_panic]
    fn single_correlation_empty_b_panic() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let prices_b = Vec::new();
        single::correlate_asset_prices(
            &prices_a,
            &prices_b,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::DeviationModel::StandardDeviation,
        );
    }

    #[test]
    #[should_panic]
    fn single_correlation_a_length_panic() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92];
        single::correlate_asset_prices(
            &prices_a,
            &prices_b,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::DeviationModel::StandardDeviation,
        );
    }

    #[test]
    #[should_panic]
    fn single_correlation_b_length_panic() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6];
        single::correlate_asset_prices(
            &prices_a,
            &prices_b,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::DeviationModel::StandardDeviation,
        );
    }

    #[test]
    fn bulk_correlation() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92, 69.54, 73.81];
        assert_eq!(
            vec![0.9042213658878326, 0.9268640506930989, 0.5300870380836703],
            bulk::correlate_asset_prices(
                &prices_a,
                &prices_b,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::StandardDeviation,
                5_usize
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_correlation_period_panic() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92, 69.54, 73.81];
        assert_eq!(
            vec![0.9042213658878326, 0.9268640506930989, 0.5300870380836703],
            bulk::correlate_asset_prices(
                &prices_a,
                &prices_b,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::StandardDeviation,
                50_usize
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_correlation_size_a_panic() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92, 69.54, 73.81];
        assert_eq!(
            vec![0.9042213658878326, 0.9268640506930989, 0.5300870380836703],
            bulk::correlate_asset_prices(
                &prices_a,
                &prices_b,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::StandardDeviation,
                5_usize
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_correlation_size_b_panic() {
        let prices_a = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        let prices_b = vec![74.71, 71.98, 68.33, 63.6, 65.92, 69.54];
        assert_eq!(
            vec![0.9042213658878326, 0.9268640506930989, 0.5300870380836703],
            bulk::correlate_asset_prices(
                &prices_a,
                &prices_b,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::StandardDeviation,
                5_usize
            )
        );
    }
}
