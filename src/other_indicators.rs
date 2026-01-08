//! # Other Indicators
//!
//! The `other_indicators` module provides technical analysis tools that do not fit neatly
//! into the main categories like momentum, trend, or volatility. These calculations often
//! serve as foundational measures or are used as components of broader strategies.
//!
//! ## When to Use
//! Use these functions when you need to:
//! - Calculate foundational metrics for risk, volatility, or bar strength
//! - Analyze price movement range or return over a period
//! - Incorporate less common but valuable technical measures into your models
//!
//! ## Structure
//! - **single**: Functions that return a single value for a slice of prices.
//! - **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
//!
//! ## Included Indicators
//!
//! ### Bulk
//! - [`return_on_investment`](bulk::return_on_investment): Calculates ROI over each period
//! - [`true_range`](bulk::true_range): Computes the True Range for each period
//! - [`average_true_range`](bulk::average_true_range): Average True Range (ATR)
//! - [`internal_bar_strength`](bulk::internal_bar_strength): Internal Bar Strength
//!
//! ### Single
//! - [`return_on_investment`](single::return_on_investment): ROI
//! - [`true_range`](bulk::true_range): True Range
//! - [`average_true_range`](bulk::average_true_range): ATR
//! - [`internal_bar_strength`](single::internal_bar_strength): Internal Bar Strength
//!
//! ## API Details
//! - All functions operate on slices of `f64` prices (and sometimes highs/lows/closes).
//! - Function-level docs explain arguments, panics, and provide usage examples.
//!
//! ---

/// **single**: Functions that return a single value for a slice of prices.
pub mod single {
    use crate::basic_indicators::single::{median, mode};
    use crate::moving_average::single::moving_average;
    use crate::{ConstantModelType, MovingAverageType};

    /// Calculates the final value and percentage return of a investment
    ///
    /// # Arguments
    ///
    /// * `start_price` - Initial price of the asset
    /// * `end_price` - Final price of the asset
    /// * `investment` - Amount invested at start
    ///
    /// # Returns
    ///
    /// A tuple containing (final_investment_value, percent_return)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let start_price = 100.0;
    /// let end_price = 110.0;
    /// let initial_investment = 1000.0;
    /// let return_on_investment =
    ///     centaur_technical_indicators::other_indicators::single::return_on_investment(
    ///         start_price,
    ///         end_price,
    ///         initial_investment
    ///     );
    /// assert_eq!((1100.0, 10.0), return_on_investment);
    ///
    /// // new_price shifts end_price to be the new start price
    /// let new_price = 98.0;
    /// let return_on_investment =
    ///     centaur_technical_indicators::other_indicators::single::return_on_investment(
    ///         end_price,
    ///         new_price,
    ///         return_on_investment.0
    ///     );
    /// assert_eq!((980.0, -10.909090909090908), return_on_investment);
    /// ```
    #[inline]
    pub fn return_on_investment(start_price: f64, end_price: f64, investment: f64) -> (f64, f64) {
        let initial_investment = investment / start_price;
        let final_investment_value = end_price * initial_investment;
        let percent_return = ((final_investment_value - investment) / investment) * 100.0;
        (final_investment_value, percent_return)
    }

    /// Calculates the True Rangea (TR)
    ///
    /// # Arguments
    ///
    /// * `previous_close` - Previous period close
    /// * `high` - Current period high
    /// * `low` - Current period low
    ///
    /// # Examples
    ///
    /// ```rust
    /// let high_low_tr = centaur_technical_indicators::other_indicators::single::true_range(
    ///     110.0,
    ///     115.0,
    ///     105.0
    /// );
    /// assert_eq!(10.0, high_low_tr);
    ///
    /// let high_close_tr = centaur_technical_indicators::other_indicators::single::true_range(
    ///     105.0,
    ///     115.0,
    ///     110.0
    /// );
    /// assert_eq!(10.0, high_close_tr);
    ///
    ///
    /// let close_low_tr = centaur_technical_indicators::other_indicators::single::true_range(
    ///     115.0,
    ///     110.0,
    ///     105.0
    /// );
    /// assert_eq!(10.0, close_low_tr);
    /// ```
    #[inline]
    pub fn true_range(close: f64, high: f64, low: f64) -> f64 {
        let h_l_tr = high - low;
        let h_c_tr = high - close;
        let c_l_tr = close - low;
        if h_l_tr >= h_c_tr && h_l_tr >= c_l_tr {
            h_l_tr
        } else if h_c_tr >= c_l_tr {
            h_c_tr
        } else {
            c_l_tr
        }
    }

    /// Calculates the Average True Range (ATR)
    ///
    /// # Arguments
    ///
    /// * `close` - Slice of previous closes prices.
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `close.len()` != `high.len()` !=  `low.len()`
    ///     * `close.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let close = vec![110.0, 105.0, 115.0];
    /// let high = vec![115.0, 115.0, 110.0];
    /// let low = vec![105.0, 110.0, 105.0];
    ///
    /// let average_true_range =
    ///     centaur_technical_indicators::other_indicators::single::average_true_range(
    ///         &close,
    ///         &high,
    ///         &low,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingAverage
    ///     );
    /// assert_eq!(10.0, average_true_range);
    ///
    /// let exponential_atr =
    ///     centaur_technical_indicators::other_indicators::single::average_true_range(
    ///         &close,
    ///         &high,
    ///         &low,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage
    ///     );
    /// assert_eq!(10.0, exponential_atr);
    /// ```
    #[inline]
    pub fn average_true_range(
        close: &[f64],
        high: &[f64],
        low: &[f64],
        constant_model_type: ConstantModelType,
    ) -> f64 {
        let length = close.len();
        if length != high.len() || length != low.len() {
            panic!(
                "Length of close ({}), high ({}), and low ({}) must match",
                length,
                high.len(),
                low.len()
            )
        };
        if close.is_empty() {
            panic!("Prices cannot be empty")
        };

        let trs: Vec<f64> = close
            .iter()
            .zip(high.iter())
            .zip(low.iter())
            .map(|((c, h), l)| true_range(*c, *h, *l))
            .collect();

        match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(&trs, MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(&trs, MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(&trs, MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => moving_average(
                &trs,
                MovingAverageType::Personalised {
                    alpha_num,
                    alpha_den,
                },
            ),
            ConstantModelType::SimpleMovingMedian => median(&trs),
            ConstantModelType::SimpleMovingMode => mode(&trs),
            _ => panic!("Unsupported ConstantModelType"),
        }
    }

    /// Calculates the internal bar strength
    ///
    /// # Arguments
    ///
    /// * `high` - High
    /// * `low` - Low
    /// * `close` - Close
    ///
    /// # Examples
    ///
    /// ```rust
    /// let high = 110.0;
    /// let low = 90.0;
    /// let close = 100.0;
    ///
    /// let internal_bar_strength = centaur_technical_indicators::other_indicators::single::internal_bar_strength(
    ///     high,
    ///     low,
    ///     close
    /// );
    ///
    /// assert_eq!(0.5, internal_bar_strength);
    /// ```
    #[inline]
    pub fn internal_bar_strength(high: f64, low: f64, close: f64) -> f64 {
        (close - low) / (high - low)
    }
}

/// **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
pub mod bulk {
    use crate::basic_indicators::bulk::{median, mode};
    use crate::moving_average::bulk::moving_average;
    use crate::other_indicators::single;
    use crate::{ConstantModelType, MovingAverageType};

    /// Calculates the return on investment and percent return
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `investment` - Initial investment
    ///
    /// # Returns
    ///
    /// A vector of tuples, each containing (final_investment_value, percent_return)
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0];
    /// let initial_investment = 1000.0;
    ///
    /// let return_on_investment =
    ///     centaur_technical_indicators::other_indicators::bulk::return_on_investment(
    ///         &prices,
    ///         initial_investment
    ///     );
    /// assert_eq!(
    ///     vec![
    ///         (1020.0, 2.0),
    ///         (1030.0, 0.9803921568627451),
    ///         (1010.0, -1.9417475728155338),
    ///         (990.0, -1.9801980198019802),
    ///         (990.0, 0.0),
    ///         (1020.0, 3.030303030303030303)
    ///     ], return_on_investment
    /// );
    /// ```
    #[inline]
    pub fn return_on_investment(prices: &[f64], investment: f64) -> Vec<(f64, f64)> {
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };
        let mut rois = Vec::with_capacity(prices.len() - 1);
        let mut roi = single::return_on_investment(prices[0], prices[1], investment);
        rois.push(roi);
        for i in 2..prices.len() {
            roi = single::return_on_investment(prices[i - 1], prices[i], rois[i - 2].0);
            rois.push(roi);
        }
        rois
    }

    /// Calculates the true range
    ///
    /// # Arguments
    ///
    /// * `close` - Slice of previous closes
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `close.len()` != `high.len()` != `low.len()`
    ///     * `close.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let close = vec![110.0, 105.0, 115.0];
    /// let high = vec![115.0, 115.0, 110.0];
    /// let low = vec![105.0, 110.0, 105.0];
    ///
    /// let true_range = centaur_technical_indicators::other_indicators::bulk::true_range(
    ///     &close,
    ///     &high,
    ///     &low
    /// );
    /// assert_eq!(vec![10.0, 10.0, 10.0], true_range);
    /// ```
    #[inline]
    pub fn true_range(close: &[f64], high: &[f64], low: &[f64]) -> Vec<f64> {
        let length = close.len();
        if length != high.len() || length != low.len() {
            panic!(
                "Length of close ({}), high ({}), and low ({}) must match",
                length,
                high.len(),
                low.len()
            )
        };
        if close.is_empty() {
            panic!("Prices cannot be empty")
        };

        (0..length)
            .map(|i| single::true_range(close[i], high[i], low[i]))
            .collect()
    }

    /// Calculates the Average True Range (ATR)
    ///
    /// # Arguments
    ///
    /// * `close` - Slice of previous closes
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    /// * `period` - Period over which to calculate the ATR
    ///
    /// # Panics
    ///
    /// `average_true_range` will panic if:
    ///     * `close.len()` != `high.len()` != `low.len()`
    ///     * `close.is_empty()`
    ///     * `period.len()` > lengths
    ///
    /// # Examples
    ///
    /// ```rust
    /// let close = vec![110.0, 105.0, 115.0, 120.0, 125.0];
    /// let high = vec![115.0, 115.0, 110.0, 130.0, 135.0];
    /// let low = vec![105.0, 110.0, 105.0, 110.0, 130.0];
    /// let period: usize = 3;
    ///
    /// let average_true_range = centaur_technical_indicators::other_indicators::bulk::average_true_range(
    ///     &close,
    ///     &high,
    ///     &low,
    ///     centaur_technical_indicators::ConstantModelType::SimpleMovingAverage,
    ///     period
    /// );
    /// assert_eq!(vec![10.0, 13.333333333333334, 13.333333333333334], average_true_range);
    ///
    /// let exponential_atr = centaur_technical_indicators::other_indicators::bulk::average_true_range(
    ///     &close,
    ///     &high,
    ///     &low,
    ///     centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage,
    ///     period
    /// );
    /// assert_eq!(vec![10.0, 15.714285714285714, 12.857142857142858], exponential_atr);
    /// ```
    #[inline]
    pub fn average_true_range(
        close: &[f64],
        high: &[f64],
        low: &[f64],
        constant_model_type: ConstantModelType,
        period: usize,
    ) -> Vec<f64> {
        let length = close.len();
        if period > length {
            panic!(
                "Period ({}) cannot be longer than length of prices ({})",
                period, length
            )
        };
        if length != high.len() || length != low.len() {
            panic!(
                "Length of close ({}), high ({}), and low ({}) must match",
                length,
                high.len(),
                low.len()
            )
        };
        if close.is_empty() {
            panic!("Prices cannot be empty")
        };

        (0..=length - period)
            .map(|i| {
                single::average_true_range(
                    &close[i..i + period],
                    &high[i..i + period],
                    &low[i..i + period],
                    constant_model_type,
                )
            })
            .collect()
    }

    /// Calculates the internal bar strength
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `close` - Slice of closing prices
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `high.len()` != `low.len()` != `close.len()`
    ///     * high.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let high = vec![110.0, 115.0, 120.0, 130.0, 135.0];
    /// let low = vec![90.0, 110.0, 105.0, 110.0, 120.0];
    /// let close = vec![100.0, 115.0, 115.0, 120.0, 125.0];
    ///
    /// let internal_bar_strength =
    ///     centaur_technical_indicators::other_indicators::bulk::internal_bar_strength(
    ///         &high,
    ///         &low,
    ///         &close
    ///     );
    ///
    /// assert_eq!(
    ///     vec![0.5, 1.0, 0.6666666666666666, 0.5, 0.33333333333333333],
    ///     internal_bar_strength
    /// );
    /// ```
    #[inline]
    pub fn internal_bar_strength(high: &[f64], low: &[f64], close: &[f64]) -> Vec<f64> {
        let length = high.len();
        if length != low.len() || length != close.len() {
            panic!(
                "Lengths of high ({}), low ({}), and close ({}) must be equal",
                length,
                low.len(),
                close.len()
            )
        };
        if high.is_empty() {
            panic!("Prices cannot be empty");
        };

        (0..length)
            .map(|i| single::internal_bar_strength(high[i], low[i], close[i]))
            .collect()
    }

    /// Calculates the positivity indicator and its signal line
    ///
    /// # Arguments
    ///
    /// * `open` - Slice of opening prices
    /// * `previous_close` - Slice of closing prices
    /// * `signal_period` - Period yp calculate the signal
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `open.len()` != `previous_close'len()`
    ///     * `open.is_empty()`
    ///     * `signal_period` is greater than length of prices
    ///
    /// # Examples
    ///
    /// ```rust
    /// let open =
    ///     vec![5278.24, 5314.48, 5357.8, 5343.81, 5341.22, 5353.0, 5409.13];
    /// let previous_close =
    ///     vec![5283.4, 5291.34, 5354.03, 5352.96, 5346.99, 5360.79, 5375.32];
    /// let signal_period: usize = 5;
    ///
    /// let positivity_indicator =
    ///     centaur_technical_indicators::other_indicators::bulk::positivity_indicator(
    ///         &open,
    ///         &previous_close,
    ///         signal_period,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingAverage
    /// );
    ///
    /// assert_eq!(
    ///     vec![
    ///         (-0.10791117993487043, 0.026244711276039178),
    ///         (-0.14531440328757447, 0.01671470717528643),
    ///         (0.6289858092169471, 0.05504820197025553)
    ///     ], positivity_indicator
    /// );
    /// ```
    pub fn positivity_indicator(
        open: &[f64],
        previous_close: &[f64],
        signal_period: usize,
        constant_model_type: ConstantModelType,
    ) -> Vec<(f64, f64)> {
        let length = open.len();
        if length != previous_close.len() {
            panic!(
                "Length of open ({}) and close ({}) must be equal",
                length,
                previous_close.len()
            )
        };
        if open.is_empty() {
            panic!("Prices cannot be empty")
        };
        if signal_period > length {
            panic!(
                "Period ({}) cannot be longer than length of prices ({})",
                signal_period, length
            )
        };

        let pis: Vec<f64> = (0..length)
            .map(|i| ((open[i] - previous_close[i]) / previous_close[i]) * 100.0)
            .collect();

        let signal_line = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(&pis, MovingAverageType::Simple, signal_period)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(&pis, MovingAverageType::Smoothed, signal_period)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(&pis, MovingAverageType::Exponential, signal_period)
            }
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => moving_average(
                &pis,
                MovingAverageType::Personalised {
                    alpha_num,
                    alpha_den,
                },
                signal_period,
            ),
            ConstantModelType::SimpleMovingMedian => median(&pis, signal_period),
            ConstantModelType::SimpleMovingMode => mode(&pis, signal_period),
            _ => panic!("Unsupported ConstantModelType"),
        };

        signal_line
            .iter()
            .enumerate()
            .map(|(i, &sig)| (pis[i + signal_period - 1], sig))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_return_on_investment() {
        let start_price = 100.46;
        let end_price = 100.53;
        let investment = 1000.0;
        assert_eq!(
            (1000.6967947441768, 0.06967947441768274),
            single::return_on_investment(start_price, end_price, investment)
        );
    }

    #[test]
    fn bulk_return_on_investment() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let investment = 1000.0;
        assert_eq!(
            vec![
                (1000.6967947441768, 0.06967947441768274),
                (999.2036631495122, -0.14920919128619353),
                (997.3123631296038, -0.18928073321378402),
                (997.5114473422257, 0.01996207206307317)
            ],
            bulk::return_on_investment(&prices, investment)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_return_on_investment_panic() {
        let prices = Vec::new();
        let investment = 1000.0;
        bulk::return_on_investment(&prices, investment);
    }

    #[test]
    fn single_true_range_high_low() {
        assert_eq!(
            0.8299999999999983,
            single::true_range(100.46, 101.12, 100.29)
        );
    }

    #[test]
    fn single_true_range_high_close() {
        assert_eq!(0.769999999999996, single::true_range(100.53, 101.3, 100.87));
    }

    #[test]
    fn single_true_range_close_low() {
        assert_eq!(
            0.4399999999999977,
            single::true_range(100.38, 100.11, 99.94)
        );
    }

    #[test]
    fn bulk_true_range() {
        let close = vec![100.46, 100.53, 100.38];
        let high = vec![101.12, 101.3, 100.11];
        let low = vec![100.29, 100.87, 99.94];
        assert_eq!(
            vec![0.8299999999999983, 0.769999999999996, 0.4399999999999977],
            bulk::true_range(&close, &high, &low)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_true_range_close_length_panic() {
        let close = vec![100.53, 100.38];
        let high = vec![101.12, 101.3, 100.11];
        let low = vec![100.29, 100.87, 99.94];
        bulk::true_range(&close, &high, &low);
    }

    #[test]
    #[should_panic]
    fn bulk_true_range_high_length_panic() {
        let close = vec![100.46, 100.53, 100.38];
        let high = vec![101.12, 100.11];
        let low = vec![100.29, 100.87, 99.94];
        bulk::true_range(&close, &high, &low);
    }

    #[test]
    #[should_panic]
    fn bulk_true_range_low_length_panic() {
        let close = vec![100.46, 100.53, 100.38];
        let high = vec![101.12, 101.3, 100.11];
        let low = vec![100.29, 99.94];
        bulk::true_range(&close, &high, &low);
    }

    #[test]
    #[should_panic]
    fn bulk_true_range_empty_panic() {
        let close = Vec::new();
        let high = vec![101.12, 101.3, 100.11];
        let low = vec![100.29, 100.87, 99.94];
        bulk::true_range(&close, &high, &low);
    }

    #[test]
    fn single_average_true_range_simple() {
        let close = vec![100.46, 100.53, 100.38];
        let high = vec![101.12, 101.3, 100.11];
        let low = vec![100.29, 100.87, 99.94];
        assert_eq!(
            0.6799999999999974,
            single::average_true_range(
                &close,
                &high,
                &low,
                crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn single_average_true_range_smoothed() {
        let close = vec![100.46, 100.53, 100.38];
        let high = vec![101.12, 101.3, 100.11];
        let low = vec![100.29, 100.87, 99.94];
        assert_eq!(
            0.6263157894736815,
            single::average_true_range(
                &close,
                &high,
                &low,
                crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn single_average_true_range_exponential() {
        let close = vec![100.46, 100.53, 100.38];
        let high = vec![101.12, 101.3, 100.11];
        let low = vec![100.29, 100.87, 99.94];
        assert_eq!(
            0.5899999999999973,
            single::average_true_range(
                &close,
                &high,
                &low,
                crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn single_average_true_range_personalised() {
        let close = vec![100.46, 100.53, 100.38];
        let high = vec![101.12, 101.3, 100.11];
        let low = vec![100.29, 100.87, 99.94];
        assert_eq!(
            0.5322388059701466,
            single::average_true_range(
                &close,
                &high,
                &low,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                }
            )
        );
    }

    #[test]
    fn single_average_true_range_median() {
        let close = vec![100.46, 100.53, 100.38];
        let high = vec![101.12, 101.3, 100.11];
        let low = vec![100.29, 100.87, 99.94];
        assert_eq!(
            0.769999999999996,
            single::average_true_range(
                &close,
                &high,
                &low,
                crate::ConstantModelType::SimpleMovingMedian
            )
        );
    }

    #[test]
    fn single_average_true_range_mode() {
        let close = vec![100.46, 100.53, 100.38];
        let high = vec![101.12, 101.3, 100.11];
        let low = vec![100.29, 100.87, 99.94];
        assert_eq!(
            1.0,
            single::average_true_range(
                &close,
                &high,
                &low,
                crate::ConstantModelType::SimpleMovingMode
            )
        );
    }

    #[test]
    #[should_panic]
    fn single_average_true_range_close_length_panic() {
        let close = vec![100.53, 100.38];
        let high = vec![101.12, 101.3, 100.11];
        let low = vec![100.29, 100.87, 99.94];
        single::average_true_range(
            &close,
            &high,
            &low,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    #[should_panic]
    fn single_average_true_range_high_length_panic() {
        let close = vec![100.46, 100.53, 100.38];
        let high = vec![101.12, 100.11];
        let low = vec![100.29, 100.87, 99.94];
        single::average_true_range(
            &close,
            &high,
            &low,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    #[should_panic]
    fn single_average_true_range_low_length_panic() {
        let close = vec![100.46, 100.53, 100.38];
        let high = vec![101.12, 101.3, 100.11];
        let low = vec![100.29, 99.94];
        single::average_true_range(
            &close,
            &high,
            &low,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    #[should_panic]
    fn single_average_true_range_empty_panic() {
        let close = Vec::new();
        let high = vec![101.12, 101.3, 100.11];
        let low = vec![100.29, 100.87, 99.94];
        single::average_true_range(
            &close,
            &high,
            &low,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    fn bulk_average_true_range() {
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let high = vec![101.12, 101.3, 100.11, 100.55, 100.43];
        let low = vec![100.29, 100.87, 99.94, 99.86, 99.91];
        let period: usize = 3;
        assert_eq!(
            vec![0.6799999999999974, 0.6333333333333305, 0.5500000000000019],
            bulk::average_true_range(
                &close,
                &high,
                &low,
                crate::ConstantModelType::SimpleMovingAverage,
                period
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_average_true_range_panic_period() {
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let high = vec![101.12, 101.3, 100.11, 100.55, 100.43];
        let low = vec![100.29, 100.87, 99.94, 99.86, 99.91];
        let period: usize = 30;
        bulk::average_true_range(
            &close,
            &high,
            &low,
            crate::ConstantModelType::SimpleMovingAverage,
            period,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_average_true_range_panic_close_length() {
        let close = vec![100.46, 100.53, 100.19, 100.21];
        let high = vec![101.12, 101.3, 100.11, 100.55, 100.43];
        let low = vec![100.29, 100.87, 99.94, 99.86, 99.91];
        let period: usize = 3;
        bulk::average_true_range(
            &close,
            &high,
            &low,
            crate::ConstantModelType::SimpleMovingAverage,
            period,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_average_true_range_panic_high_length() {
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let high = vec![101.12, 101.3, 100.11, 100.43];
        let low = vec![100.29, 100.87, 99.94, 99.86, 99.91];
        let period: usize = 3;
        bulk::average_true_range(
            &close,
            &high,
            &low,
            crate::ConstantModelType::SimpleMovingAverage,
            period,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_average_true_range_panic_low_length() {
        let close = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let high = vec![101.12, 101.3, 100.11, 100.55, 100.43];
        let low = vec![100.29, 99.94, 99.86, 99.91];
        let period: usize = 3;
        bulk::average_true_range(
            &close,
            &high,
            &low,
            crate::ConstantModelType::SimpleMovingAverage,
            period,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_average_true_range_panic_empty() {
        let close = Vec::new();
        let high = vec![101.12, 101.3, 100.11, 100.55, 100.43];
        let low = vec![100.29, 100.87, 99.94, 99.86, 99.91];
        let period: usize = 3;
        bulk::average_true_range(
            &close,
            &high,
            &low,
            crate::ConstantModelType::SimpleMovingAverage,
            period,
        );
    }

    #[test]
    fn single_internal_bar_strengh() {
        let close = 100.55;
        let high = 102.32;
        let low = 100.14;
        assert_eq!(
            0.1880733944954119,
            single::internal_bar_strength(high, low, close)
        );
    }

    #[test]
    fn bulk_internal_bar_strength() {
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(
            vec![
                0.1880733944954119,
                0.017543859649123535,
                0.7727272727272783,
                0.5521472392638039,
                0.8780487804878055
            ],
            bulk::internal_bar_strength(&high, &low, &close)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_internal_bar_strength_panic_close_length() {
        let close = vec![100.55, 99.01, 100.43, 101.0];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96];
        bulk::internal_bar_strength(&high, &low, &close);
    }

    #[test]
    #[should_panic]
    fn bulk_internal_bar_strength_panic_high_length() {
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76];
        let high = vec![102.32, 100.69, 100.83, 101.73];
        let low = vec![100.14, 98.98, 99.07, 100.1, 99.96];
        bulk::internal_bar_strength(&high, &low, &close);
    }

    #[test]
    #[should_panic]
    fn bulk_internal_bar_strength_panic_low_length() {
        let close = vec![100.55, 99.01, 100.43, 101.0, 101.76];
        let high = vec![102.32, 100.69, 100.83, 101.73, 102.01];
        let low = vec![100.14, 98.98, 99.07, 100.1];
        bulk::internal_bar_strength(&high, &low, &close);
    }

    #[test]
    #[should_panic]
    fn bulk_internal_bar_strength_panic_empty() {
        let close = Vec::new();
        let high = Vec::new();
        let low = Vec::new();
        bulk::internal_bar_strength(&high, &low, &close);
    }

    #[test]
    fn bulk_positivity_indicator_ma() {
        let open = vec![5278.24, 5314.48, 5357.8, 5343.81, 5341.22, 5353.0, 5409.13];
        let previous_close = vec![5283.4, 5291.34, 5354.03, 5352.96, 5346.99, 5360.79, 5375.32];
        let signal_period: usize = 5;
        assert_eq!(
            vec![
                (-0.10791117993487043, 0.026244711276039178),
                (-0.14531440328757447, 0.01671470717528643),
                (0.6289858092169471, 0.05504820197025553)
            ],
            bulk::positivity_indicator(
                &open,
                &previous_close,
                signal_period,
                crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn bulk_positivity_indicator_sma() {
        let open = vec![5278.24, 5314.48, 5357.8, 5343.81, 5341.22, 5353.0, 5409.13];
        let previous_close = vec![5283.4, 5291.34, 5354.03, 5352.96, 5346.99, 5360.79, 5375.32];
        let signal_period: usize = 5;
        assert_eq!(
            vec![
                (-0.10791117993487043, -0.004667175210233987),
                (-0.14531440328757447, -0.0374414205397291),
                (0.6289858092169471, 0.11452727085189565)
            ],
            bulk::positivity_indicator(
                &open,
                &previous_close,
                signal_period,
                crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn bulk_positivity_indicator_ema() {
        let open = vec![5278.24, 5314.48, 5357.8, 5343.81, 5341.22, 5353.0, 5409.13];
        let previous_close = vec![5283.4, 5291.34, 5354.03, 5352.96, 5346.99, 5360.79, 5375.32];
        let signal_period: usize = 5;
        assert_eq!(
            vec![
                (-0.10791117993487043, -0.03082127868198756),
                (-0.14531440328757447, -0.0713945013484951),
                (0.6289858092169471, 0.17175495314835065)
            ],
            bulk::positivity_indicator(
                &open,
                &previous_close,
                signal_period,
                crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn bulk_positivity_indicator_pma() {
        let open = vec![5278.24, 5314.48, 5357.8, 5343.81, 5341.22, 5353.0, 5409.13];
        let previous_close = vec![5283.4, 5291.34, 5354.03, 5352.96, 5346.99, 5360.79, 5375.32];
        let signal_period: usize = 5;
        assert_eq!(
            vec![
                (-0.10791117993487043, -0.07654434206147799),
                (-0.14531440328757447, -0.1152171021135638),
                (0.6289858092169471, 0.3001081065924838)
            ],
            bulk::positivity_indicator(
                &open,
                &previous_close,
                signal_period,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                }
            )
        );
    }

    #[test]
    fn bulk_positivity_indicator_median() {
        let open = vec![5278.24, 5314.48, 5357.8, 5343.81, 5341.22, 5353.0, 5409.13];
        let previous_close = vec![5283.4, 5291.34, 5354.03, 5352.96, 5346.99, 5360.79, 5375.32];
        let signal_period: usize = 5;
        assert_eq!(
            vec![
                (-0.10791117993487043, -0.0976643827838107),
                (-0.14531440328757447, -0.10791117993487043),
                (0.6289858092169471, -0.10791117993487043)
            ],
            bulk::positivity_indicator(
                &open,
                &previous_close,
                signal_period,
                crate::ConstantModelType::SimpleMovingMedian
            )
        );
    }

    #[test]
    fn bulk_positivity_indicator_mode() {
        let open = vec![5278.24, 5314.48, 5357.8, 5343.81, 5341.22, 5353.0, 5409.13];
        let previous_close = vec![5283.4, 5291.34, 5354.03, 5352.96, 5346.99, 5360.79, 5375.32];
        let signal_period: usize = 5;
        assert_eq!(
            vec![
                (-0.10791117993487043, 0.0),
                (-0.14531440328757447, 0.0),
                (0.6289858092169471, 0.0)
            ],
            bulk::positivity_indicator(
                &open,
                &previous_close,
                signal_period,
                crate::ConstantModelType::SimpleMovingMode
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_positivity_indicator_panic_length() {
        let open = vec![5278.24, 5314.48, 5343.81, 5341.22, 5353.0, 5409.13];
        let previous_close = vec![5283.4, 5291.34, 5354.03, 5352.96, 5346.99, 5360.79, 5375.32];
        let signal_period: usize = 5;
        bulk::positivity_indicator(
            &open,
            &previous_close,
            signal_period,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_positivity_indicator_panic_empty() {
        let open = Vec::new();
        let previous_close = Vec::new();
        let signal_period: usize = 5;
        bulk::positivity_indicator(
            &open,
            &previous_close,
            signal_period,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_positivity_indicator_panic_period() {
        let open = vec![5278.24, 5314.48, 5357.8, 5343.81, 5341.22, 5353.0, 5409.13];
        let previous_close = vec![5283.4, 5291.34, 5354.03, 5352.96, 5346.99, 5360.79, 5375.32];
        let signal_period: usize = 50;
        bulk::positivity_indicator(
            &open,
            &previous_close,
            signal_period,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }
}
