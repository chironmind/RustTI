//! # Momentum Indicators
//!
//! The `momentum_indicators` module provides functions to measure the speed, strength,
//! and direction of price movements in time series data.
//! These indicators are commonly used to identify overbought/oversold conditions, trend continuation, or potential reversals.
//!
//! ## When to Use
//! Use momentum indicators to:
//! - Gauge the strength and velocity of price trends
//! - Identify bullish or bearish momentum
//! - Spot early signals for possible price reversals or continuations
//!
//! ## Structure
//! - **single**: Functions that return a single value for a slice of prices.
//! - **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
//!
//! ## Included Indicators
//!
//! ### Bulk
//! - [`chaikin_oscillator`](bulk::chaikin_oscillator): Chaikin Oscillator
//! - [`commodity_channel_index`](bulk::commodity_channel_index): Commodity Channel Index (CCI)
//! - [`macd_line`](bulk::macd_line): Moving Average Convergence Divergence (MACD) line
//! - [`mcginley_dynamic_commodity_channel_index`](bulk::mcginley_dynamic_commodity_channel_index): McGinley variant of CCI
//! - [`mcginley_dynamic_macd_line`](bulk::mcginley_dynamic_macd_line): McGinley variant of MACD
//! - [`money_flow_index`](bulk::money_flow_index): Money Flow Index (MFI)
//! - [`on_balance_volume`](bulk::on_balance_volume): On-Balance Volume (OBV)
//! - [`rate_of_change`](bulk::rate_of_change): Rate of Change (ROC)
//! - [`relative_strength_index`](bulk::relative_strength_index): Relative Strength Index (RSI)
//! - [`signal_line`](bulk::signal_line): Signal line for MACD
//! - [`slow_stochastic`](bulk::slow_stochastic): Slow Stochastic Oscillator
//! - [`slowest_stochastic`](bulk::slowest_stochastic): Slowest Stochastic Oscillator
//! - [`stochastic_oscillator`](bulk::stochastic_oscillator): Stochastic Oscillator
//! - [`williams_percent_r`](bulk::williams_percent_r): Williams %R
//! - [`percentage_price_oscillator`](bulk::percentage_price_oscillator): Percentage Price Oscillator (PPO)
//! - [`chande_momentum_oscillator`](bulk::chande_momentum_oscillator): Chande Momentum Oscillator (CMO)
//!
//! ### Single
//! - [`chaikin_oscillator`](single::chaikin_oscillator): Chaikin Oscillator
//! - [`commodity_channel_index`](single::commodity_channel_index): Commodity Channel Index (CCI)
//! - [`macd_line`](single::macd_line): MACD line
//! - [`mcginley_dynamic_commodity_channel_index`](single::mcginley_dynamic_commodity_channel_index): McGinley variant of CCI
//! - [`mcginley_dynamic_macd_line`](single::mcginley_dynamic_macd_line): McGinley variant of MACD
//! - [`money_flow_index`](single::money_flow_index): Money Flow Index (MFI)
//! - [`on_balance_volume`](single::on_balance_volume): On-Balance Volume (OBV)
//! - [`rate_of_change`](single::rate_of_change): Rate of Change (ROC)
//! - [`relative_strength_index`](single::relative_strength_index): Relative Strength Index (RSI)
//! - [`signal_line`](single::signal_line): Signal line for MACD
//! - [`slow_stochastic`](single::slow_stochastic): Slow Stochastic Oscillator
//! - [`slowest_stochastic`](single::slowest_stochastic): Slowest Stochastic Oscillator
//! - [`stochastic_oscillator`](single::stochastic_oscillator): Stochastic Oscillator
//! - [`williams_percent_r`](single::williams_percent_r): Williams %R
//! - [`percentage_price_oscillator`](single::percentage_price_oscillator): Percentage Price Oscillator (PPO)
//! - [`chande_momentum_oscillator`](single::chande_momentum_oscillator): Chande Momentum Oscillator (CMO)
//!
//! ## API Details
//! - All indicators accept slices of `f64` prices and relevant parameters (periods, multipliers, etc.).
//! - Returns are vectors (for `bulk`) or scalars (for `single`).
//! - All edge cases and panics are explained at the function level.
//!
//! ---

/// **single**: Functions that return a single value for a slice of prices.
pub mod single {
    use crate::basic_indicators::single::{
        absolute_deviation, cauchy_iqr_scale, laplace_std_equivalent, log_standard_deviation, max,
        median, min, mode, standard_deviation, student_t_adjusted_std,
    };
    use crate::moving_average::single::{mcginley_dynamic, moving_average};
    use crate::strength_indicators::single::accumulation_distribution;
    use crate::volatility_indicators::single::ulcer_index;
    use crate::{
        AbsDevConfig, CentralPoint, ConstantModelType, DeviationAggregate, DeviationModel,
        MovingAverageType,
    };
    use std::cmp::Ordering;

    /// Calculates the Relative Strength Index (RSI)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    ///
    /// let defaut_rsi =
    ///     centaur_technical_indicators::momentum_indicators::single::relative_strength_index(
    ///         &prices,
    ///         centaur_technical_indicators::ConstantModelType::SmoothedMovingAverage
    ///     );
    /// assert_eq!(39.99999999999999, defaut_rsi);
    ///
    /// let ema_rsi =
    ///     centaur_technical_indicators::momentum_indicators::single::relative_strength_index(
    ///         &prices,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage
    ///     );
    /// assert_eq!(38.46153846153846, ema_rsi);
    ///
    /// let moving_median_rsi =
    ///     centaur_technical_indicators::momentum_indicators::single::relative_strength_index(
    ///         &prices,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingMedian
    ///     );
    /// assert_eq!(42.857142857142854, moving_median_rsi);
    /// ```
    #[inline]
    pub fn relative_strength_index(prices: &[f64], constant_model_type: ConstantModelType) -> f64 {
        let (previous_gains, previous_loss) = previous_gains_loss(prices);
        if previous_gains.is_empty() {
            return 0.0;
        }
        if previous_loss.is_empty() {
            return 100.0;
        }

        let (previous_average_gains, previous_average_loss) = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => (
                moving_average(&previous_gains, MovingAverageType::Simple),
                moving_average(&previous_loss, MovingAverageType::Simple),
            ),
            ConstantModelType::SmoothedMovingAverage => (
                moving_average(&previous_gains, MovingAverageType::Smoothed),
                moving_average(&previous_loss, MovingAverageType::Smoothed),
            ),
            ConstantModelType::ExponentialMovingAverage => (
                moving_average(&previous_gains, MovingAverageType::Exponential),
                moving_average(&previous_loss, MovingAverageType::Exponential),
            ),
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => (
                moving_average(
                    &previous_gains,
                    MovingAverageType::Personalised {
                        alpha_num,
                        alpha_den,
                    },
                ),
                moving_average(
                    &previous_loss,
                    MovingAverageType::Personalised {
                        alpha_num,
                        alpha_den,
                    },
                ),
            ),
            ConstantModelType::SimpleMovingMedian => {
                (median(&previous_gains), median(&previous_loss))
            }
            ConstantModelType::SimpleMovingMode => (mode(&previous_gains), mode(&previous_loss)),
            _ => panic!("Unsupported ConstantModelType"),
        };

        if previous_average_loss == 0.0 {
            0.0
        } else {
            100.0 - (100.0 / (1.0 + (previous_average_gains / previous_average_loss)))
        }
    }

    /// Calculates the stochastic oscillator
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
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let stochastic_oscillator =
    ///     centaur_technical_indicators::momentum_indicators::single::stochastic_oscillator(&prices);
    /// assert_eq!(0.0, stochastic_oscillator);
    /// ```
    #[inline]
    pub fn stochastic_oscillator(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            panic!("Prices is empty");
        };
        let mut ordered_prices = prices
            .iter()
            .filter_map(|f| if f.is_nan() { None } else { Some(*f) })
            .collect::<Vec<f64>>();
        ordered_prices.sort_by(cmp_f64);
        let min = ordered_prices[0];
        let max = ordered_prices.last().unwrap();
        100.0 * ((prices.last().unwrap() - min) / (max - min))
    }

    /// Calculates the slow stochastic
    ///
    /// # Arguments
    ///
    /// * `stochastics` - Slice of stochastics
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// Panics if `stochastics.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let stochstic_oscillators = [0.0, 50.0, 100.0];
    ///
    /// let simple_ma_slow_stochastic =
    ///     centaur_technical_indicators::momentum_indicators::single::slow_stochastic(
    ///         &stochstic_oscillators,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingAverage
    ///     );
    /// assert_eq!(50.0, simple_ma_slow_stochastic);
    ///
    /// let sma_slow_stochastic =
    ///     centaur_technical_indicators::momentum_indicators::single::slow_stochastic(
    ///         &stochstic_oscillators,
    ///         centaur_technical_indicators::ConstantModelType::SmoothedMovingAverage
    ///     );
    /// assert_eq!(63.15789473684211, sma_slow_stochastic);
    ///
    /// let median_slow_stochastic =
    ///     centaur_technical_indicators::momentum_indicators::single::slow_stochastic(
    ///         &stochstic_oscillators,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingMedian
    ///     );
    /// assert_eq!(50.0, median_slow_stochastic);
    /// ```
    #[inline]
    pub fn slow_stochastic(stochastics: &[f64], constant_model_type: ConstantModelType) -> f64 {
        if stochastics.is_empty() {
            panic!("stochastics cannot be empty");
        };

        match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(stochastics, MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(stochastics, MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(stochastics, MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => moving_average(
                stochastics,
                MovingAverageType::Personalised {
                    alpha_num,
                    alpha_den,
                },
            ),
            ConstantModelType::SimpleMovingMedian => median(stochastics),
            ConstantModelType::SimpleMovingMode => mode(stochastics),
            _ => panic!("Unsupported ConstantModelType"),
        }
    }

    /// Calculates the slowest stochastic
    ///
    /// # Arguments
    ///
    /// * `slow_stochastics` - Slice of slow stochastics
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// Panics if `slow_stochastics.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let slow_stochstic = [30.0, 20.0, 10.0];
    ///
    /// let simple_ma_slowest_stochastic =
    ///     centaur_technical_indicators::momentum_indicators::single::slowest_stochastic(
    ///         &slow_stochstic,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingAverage
    ///     );
    /// assert_eq!(20.0, simple_ma_slowest_stochastic);
    ///
    /// let sma_slowest_stochastic =
    ///     centaur_technical_indicators::momentum_indicators::single::slowest_stochastic(
    ///         &slow_stochstic,
    ///         centaur_technical_indicators::ConstantModelType::SmoothedMovingAverage
    ///     );
    /// assert_eq!(17.368421052631582, sma_slowest_stochastic);
    ///
    /// let median_slowest_stochastic =
    ///     centaur_technical_indicators::momentum_indicators::single::slowest_stochastic(
    ///         &slow_stochstic,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingMedian
    ///     );
    /// assert_eq!(20.0, median_slowest_stochastic);
    /// ```
    #[inline]
    pub fn slowest_stochastic(
        slow_stochastics: &[f64],
        constant_model_type: ConstantModelType,
    ) -> f64 {
        if slow_stochastics.is_empty() {
            panic!("slow stochastics cannot be empty");
        };

        match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(slow_stochastics, MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(slow_stochastics, MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(slow_stochastics, MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => moving_average(
                slow_stochastics,
                MovingAverageType::Personalised {
                    alpha_num,
                    alpha_den,
                },
            ),
            ConstantModelType::SimpleMovingMedian => median(slow_stochastics),
            ConstantModelType::SimpleMovingMode => mode(slow_stochastics),
            _ => panic!("Unsupported ConstantModelType"),
        }
    }

    /// Calculates the Williams %R
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `close` - Close price for the observed period
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `high.is_empty()` or `low.is_empty()`
    ///     * `high.len()` != `low.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let high = [200.0, 215.0, 206.0];
    /// let low = [175.0, 189.0, 182.0];
    /// let close = 192.0;
    /// let williams_percent_r =
    ///     centaur_technical_indicators::momentum_indicators::single::williams_percent_r(
    ///         &high,
    ///         &low,
    ///         close
    ///     );
    /// assert_eq!(-57.49999999999999, williams_percent_r);
    /// ```
    #[inline]
    pub fn williams_percent_r(high: &[f64], low: &[f64], close: f64) -> f64 {
        if high.is_empty() || low.is_empty() {
            panic!("high and low cannot be empty")
        };
        if high.len() != low.len() {
            panic!(
                "Length of high ({}) and low ({}) must match",
                high.len(),
                low.len()
            )
        };
        let max_high = max(high);
        let min_low = min(low);
        -100.0_f64 * ((max_high - close) / (max_high - min_low))
    }

    /// Calculates the Money Flow Index (MFI)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `volume` - Slice of transaction volumes
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `prices.is_empty()` or `volume.is_empty()`
    ///     * `prices.len()` != `volume.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let volume = vec![1000.0, 1500.0, 1200.0, 900.0, 1300.0];
    /// let money_flow_index =
    ///     centaur_technical_indicators::momentum_indicators::single::money_flow_index(
    ///         &prices,
    ///         &volume
    ///     );
    /// assert_eq!(56.771463119709786, money_flow_index);
    /// ```
    #[inline]
    pub fn money_flow_index(prices: &[f64], volume: &[f64]) -> f64 {
        if prices.is_empty() || volume.is_empty() {
            panic!("Prices and volume cannot be empty");
        }
        let length = prices.len();
        if length != volume.len() {
            panic!(
                "Length of prices ({}) needs to length of volume ({})",
                length,
                volume.len()
            );
        };

        let mut raw_money_flow = Vec::with_capacity(length);
        for i in 0..length {
            raw_money_flow.push(prices[i] * volume[i]);
        }

        let mut positive_money_flow = 0.0;
        let mut negative_money_flow = 0.0;

        for i in 1..length {
            if raw_money_flow[i] > raw_money_flow[i - 1] {
                positive_money_flow += raw_money_flow[i];
            } else if raw_money_flow[i] < raw_money_flow[i - 1] {
                negative_money_flow += raw_money_flow[i];
            }
        }

        if negative_money_flow == 0.0 {
            100.0
        } else {
            100.0 - (100.0 / (1.0 + (positive_money_flow / negative_money_flow)))
        }
    }

    /// Calculates the rate of change (RoC)
    ///
    /// # Arguments
    ///
    /// * `current_price` - Current price
    /// * `previous_price` - Previous price
    ///
    /// # Examples
    ///
    /// ```rust
    /// let current_price = 120.0;
    /// let previous_price = 100.0;
    /// let rate_of_change =
    ///     centaur_technical_indicators::momentum_indicators::single::rate_of_change(
    ///         current_price,
    ///         previous_price
    ///     );
    /// assert_eq!(20.0, rate_of_change);
    ///
    /// let current_price = 100.0;
    /// let previous_price = 120.0;
    /// let rate_of_change =
    ///     centaur_technical_indicators::momentum_indicators::single::rate_of_change(
    ///         current_price,
    ///         previous_price
    ///     );
    /// assert_eq!(-16.666666666666664, rate_of_change);
    /// ```
    #[inline]
    pub fn rate_of_change(current_price: f64, previous_price: f64) -> f64 {
        ((current_price - previous_price) / previous_price) * 100.0
    }

    /// Calculates the on balance volume (OBV)
    ///
    /// The standard price to use it the closing price. If there is no previous on balance volume
    /// pass in 0.
    ///
    /// # Arguments
    ///
    /// * `current_price` - Current price
    /// * `previous_price` - Previous price
    /// * `current_volume` - Current volume
    /// * `previous_on_balance_volume` - Previous on balance volume. Use 0.0 if no previous OBV.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let current_price = 120.0;
    /// let previous_price = 100.0;
    /// let current_volume = 1500.0;
    /// let on_balance_volume =
    ///     centaur_technical_indicators::momentum_indicators::single::on_balance_volume(
    ///         current_price,
    ///         previous_price,
    ///         current_volume,
    ///         0.0
    ///     );
    /// assert_eq!(1500.0 ,on_balance_volume);
    ///
    /// let current_price = 100.0;
    /// let previous_price = 120.0;
    /// let current_volume = 1000.0;
    /// let on_balance_volume =
    ///     centaur_technical_indicators::momentum_indicators::single::on_balance_volume(
    ///         current_price,
    ///         previous_price,
    ///         current_volume,
    ///         1500.0);
    /// assert_eq!(500.0, on_balance_volume);
    /// ```
    #[inline]
    pub fn on_balance_volume(
        current_price: f64,
        previous_price: f64,
        current_volume: f64,
        previous_on_balance_volume: f64,
    ) -> f64 {
        let mut volume = 0.0;
        if current_price > previous_price {
            volume += current_volume;
        } else if current_price < previous_price {
            volume -= current_volume;
        };
        previous_on_balance_volume + volume
    }

    /// Calculates the Commodity Channel Index (CCI)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    /// * `deviation_model` - Variant of [`DeviationModel`]
    /// * `constant_multiplier` - Scale factor (Usually 0.015)
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let constant_multiplier = 0.015;
    ///
    /// let default_cci =
    ///     centaur_technical_indicators::momentum_indicators::single::commodity_channel_index(
    ///         &prices,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingAverage,
    ///         centaur_technical_indicators::DeviationModel::MeanAbsoluteDeviation,
    ///         constant_multiplier
    ///     );
    /// assert_eq!(-111.11111111111111, default_cci);
    ///
    /// let ema_sd_cci =
    ///     centaur_technical_indicators::momentum_indicators::single::commodity_channel_index(
    ///         &prices,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage,
    ///         centaur_technical_indicators::DeviationModel::StandardDeviation,
    ///         constant_multiplier
    ///     );
    /// assert_eq!(-75.96091804215764, ema_sd_cci);
    ///
    /// let median_mad_cci =
    ///     centaur_technical_indicators::momentum_indicators::single::commodity_channel_index(
    ///         &prices,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingMedian,
    ///         centaur_technical_indicators::DeviationModel::MedianAbsoluteDeviation,
    ///         constant_multiplier
    ///     );  
    /// assert_eq!(-133.33333333333334, median_mad_cci);
    /// ```
    #[inline]
    pub fn commodity_channel_index(
        prices: &[f64],
        constant_model_type: ConstantModelType,
        deviation_model: DeviationModel,
        constant_multiplier: f64,
    ) -> f64 {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
        };

        let moving_constant = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(prices, MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(prices, MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(prices, MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => moving_average(
                prices,
                MovingAverageType::Personalised {
                    alpha_num,
                    alpha_den,
                },
            ),
            ConstantModelType::SimpleMovingMedian => median(prices),
            ConstantModelType::SimpleMovingMode => mode(prices),
            _ => panic!("Unsupported ConstantModelType"),
        };

        let deviation = match deviation_model {
            DeviationModel::StandardDeviation => standard_deviation(prices),
            DeviationModel::MeanAbsoluteDeviation => absolute_deviation(
                prices,
                AbsDevConfig {
                    center: CentralPoint::Mean,
                    aggregate: DeviationAggregate::Mean,
                },
            ),
            DeviationModel::MedianAbsoluteDeviation => absolute_deviation(
                prices,
                AbsDevConfig {
                    center: CentralPoint::Median,
                    aggregate: DeviationAggregate::Median,
                },
            ),
            DeviationModel::ModeAbsoluteDeviation => absolute_deviation(
                prices,
                AbsDevConfig {
                    center: CentralPoint::Mode,
                    aggregate: DeviationAggregate::Mode,
                },
            ),
            DeviationModel::CustomAbsoluteDeviation { config } => {
                absolute_deviation(prices, config)
            }
            DeviationModel::UlcerIndex => ulcer_index(prices),
            DeviationModel::LogStandardDeviation => log_standard_deviation(prices),
            DeviationModel::StudentT { df } => student_t_adjusted_std(prices, df),
            DeviationModel::LaplaceStdEquivalent => laplace_std_equivalent(prices),
            DeviationModel::CauchyIQRScale => cauchy_iqr_scale(prices),
            DeviationModel::EmpiricalQuantileRange {
                low,
                high,
                precision,
            } => crate::basic_indicators::single::empirical_quantile_range_from_distribution(
                prices, precision, low, high,
            ),
            #[allow(unreachable_patterns)]
            _ => panic!("Unsupported DeviationModel"),
        };
        if deviation == 0.0 {
            0.0
        } else {
            (prices.last().unwrap() - moving_constant) / (constant_multiplier * deviation)
        }
    }

    /// Calculates the McGinley dynamic commodity channel index
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `previous_mcginley_dynamic` - Previous McGinley dynamic. 0.0 if none
    /// * `deviation_model` - Variant of [`DeviationModel`]
    /// * `constant_multiplier` - Scale factor. Normally 0.015
    ///
    /// # Panics
    ///
    /// Panics if `prices.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let constant_multiplier = 0.015;
    ///
    /// let mcginley_cci =
    ///     centaur_technical_indicators::momentum_indicators::single::mcginley_dynamic_commodity_channel_index(
    ///         &prices,
    ///         0.0,
    ///         centaur_technical_indicators::DeviationModel::MeanAbsoluteDeviation,
    ///         constant_multiplier
    ///     );
    /// assert_eq!((0.0, 99.0), mcginley_cci);
    ///
    /// let prices = vec![102.0, 103.0, 101.0, 99.0, 102.0];
    /// let mcginley_cci =
    ///     centaur_technical_indicators::momentum_indicators::single::mcginley_dynamic_commodity_channel_index(
    ///         &prices,
    ///         mcginley_cci.1,
    ///         centaur_technical_indicators::DeviationModel::MeanAbsoluteDeviation,
    ///         constant_multiplier
    ///     );
    /// assert_eq!((146.8770632107927, 99.53246533805869), mcginley_cci);
    /// ```
    #[inline]
    pub fn mcginley_dynamic_commodity_channel_index(
        prices: &[f64],
        previous_mcginley_dynamic: f64,
        deviation_model: DeviationModel,
        constant_multiplier: f64,
    ) -> (f64, f64) {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
        };

        let last_price = prices.last().copied().unwrap();

        let mcginley_dynamic =
            mcginley_dynamic(last_price, previous_mcginley_dynamic, prices.len());

        let deviation = match deviation_model {
            DeviationModel::StandardDeviation => standard_deviation(prices),
            DeviationModel::MeanAbsoluteDeviation => absolute_deviation(
                prices,
                AbsDevConfig {
                    center: CentralPoint::Mean,
                    aggregate: DeviationAggregate::Mean,
                },
            ),
            DeviationModel::MedianAbsoluteDeviation => absolute_deviation(
                prices,
                AbsDevConfig {
                    center: CentralPoint::Median,
                    aggregate: DeviationAggregate::Median,
                },
            ),
            DeviationModel::ModeAbsoluteDeviation => absolute_deviation(
                prices,
                AbsDevConfig {
                    center: CentralPoint::Mode,
                    aggregate: DeviationAggregate::Mode,
                },
            ),
            DeviationModel::CustomAbsoluteDeviation { config } => {
                absolute_deviation(prices, config)
            }
            DeviationModel::UlcerIndex => ulcer_index(prices),
            DeviationModel::LogStandardDeviation => log_standard_deviation(prices),
            DeviationModel::StudentT { df } => student_t_adjusted_std(prices, df),
            DeviationModel::LaplaceStdEquivalent => laplace_std_equivalent(prices),
            DeviationModel::CauchyIQRScale => cauchy_iqr_scale(prices),
            DeviationModel::EmpiricalQuantileRange {
                low,
                high,
                precision,
            } => crate::basic_indicators::single::empirical_quantile_range_from_distribution(
                prices, precision, low, high,
            ),
            #[allow(unreachable_patterns)]
            _ => panic!("Unsupported DeviationModel"),
        };
        if deviation == 0.0 {
            (0.0, mcginley_dynamic)
        } else {
            (
                (last_price - mcginley_dynamic) / (constant_multiplier * deviation),
                mcginley_dynamic,
            )
        }
    }

    /// Calculates the Moving Average Convergence Divergence (MACD) line
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `short_period` - Length of the short period
    /// * `short_period_model` - Variant of [`ConstantModelType`]
    /// * `long_period_model` - Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// `macd_line` will panic if:
    /// * `prices.is_empty()`
    /// * `short_period` >= `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    ///
    /// let macd =
    ///     centaur_technical_indicators::momentum_indicators::single::macd_line(
    ///         &prices,
    ///         3_usize,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage
    ///     );
    /// assert_eq!(-0.46851726472581845, macd);
    ///
    /// let macd =
    ///     centaur_technical_indicators::momentum_indicators::single::macd_line(
    ///         &prices,
    ///         3_usize,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingAverage,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingMedian
    ///     );
    /// assert_eq!(0.0, macd);
    /// ```
    #[inline]
    pub fn macd_line(
        prices: &[f64],
        short_period: usize,
        short_period_model: ConstantModelType,
        long_period_model: ConstantModelType,
    ) -> f64 {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
        };
        let length = prices.len();
        if short_period >= length {
            panic!(
                "Short period ({}) cannot be greater or equal to long period ({})",
                short_period, length
            );
        };

        let short_period_slice = &prices[length - short_period..];
        let short_period_average = match short_period_model {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(short_period_slice, MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(short_period_slice, MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(short_period_slice, MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => moving_average(
                short_period_slice,
                MovingAverageType::Personalised {
                    alpha_num,
                    alpha_den,
                },
            ),
            ConstantModelType::SimpleMovingMedian => median(short_period_slice),
            ConstantModelType::SimpleMovingMode => mode(short_period_slice),
            _ => panic!("Unsupported ConstantModelType"),
        };

        let long_period_average = match long_period_model {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(prices, MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(prices, MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(prices, MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => moving_average(
                prices,
                MovingAverageType::Personalised {
                    alpha_num,
                    alpha_den,
                },
            ),
            ConstantModelType::SimpleMovingMedian => median(prices),
            ConstantModelType::SimpleMovingMode => mode(prices),
            _ => panic!("Unsupported ConstantModelType"),
        };
        short_period_average - long_period_average
    }

    /// Calculates the MACD signal line
    ///
    /// # Arguments
    ///
    /// * `macds` - Slice of MACDs
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// Panics if `macds.is_empty()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let macds = vec![-0.0606702775897219, -0.0224170616113781, 0.0057887610020515];
    ///
    /// let ema_signal_line =
    ///     centaur_technical_indicators::momentum_indicators::single::signal_line(
    ///         &macds,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage
    ///     );
    /// assert_eq!(-0.011764193829181728, ema_signal_line);
    ///
    /// let median_signal_line =
    ///     centaur_technical_indicators::momentum_indicators::single::signal_line(
    ///         &macds,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingMedian
    ///     );
    /// assert_eq!(-0.0224170616113781, median_signal_line);
    /// ```
    #[inline]
    pub fn signal_line(macds: &[f64], constant_model_type: ConstantModelType) -> f64 {
        if macds.is_empty() {
            panic!("MACDs cannot be empty");
        };
        match constant_model_type {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(macds, MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(macds, MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(macds, MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => moving_average(
                macds,
                MovingAverageType::Personalised {
                    alpha_num,
                    alpha_den,
                },
            ),
            ConstantModelType::SimpleMovingMedian => median(macds),
            ConstantModelType::SimpleMovingMode => mode(macds),
            _ => panic!("Unsupported ConstantModelType"),
        }
    }

    /// Calculates the McGinley dynamic MACD line
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `short_period` - The length of the short period
    /// * `previous_short_mcginley` - Previous McGinley dynamic for the short model.
    /// * `previous_long_mcginley` - Previous McGinley dynamic for the long model.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `prices.is_empty()`
    ///     * `short_period` >= `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let short_period: usize = 3;
    ///
    /// let mcginley_dynamic_macd =
    ///     centaur_technical_indicators::momentum_indicators::single::mcginley_dynamic_macd_line(
    ///         &prices,
    ///         short_period,
    ///         0.0,
    ///         0.0
    ///     );
    /// assert_eq!((0.0, 99.0, 99.0), mcginley_dynamic_macd);
    ///
    /// let prices = vec![102.0, 103.0, 101.0, 99.0, 102.0];
    /// let mcginley_dynamic_macd =
    ///     centaur_technical_indicators::momentum_indicators::single::mcginley_dynamic_macd_line(
    ///         &prices,
    ///         short_period,
    ///         mcginley_dynamic_macd.1,
    ///         mcginley_dynamic_macd.2);
    /// assert_eq!((0.35497689203913296, 99.88744223009782, 99.53246533805869), mcginley_dynamic_macd);
    /// ```
    #[inline]
    pub fn mcginley_dynamic_macd_line(
        prices: &[f64],
        short_period: usize,
        previous_short_mcginley: f64,
        previous_long_mcginley: f64,
    ) -> (f64, f64, f64) {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
        };

        if short_period >= prices.len() {
            panic!(
                "Short period ({}) cannot be longer or equal to length of prices ({})",
                short_period,
                prices.len()
            );
        };

        let latest_price = *prices.last().unwrap();
        if previous_short_mcginley == 0.0 && previous_long_mcginley == 0.0 {
            return (0.0, latest_price, latest_price);
        };

        let long_mcginley = mcginley_dynamic(latest_price, previous_long_mcginley, prices.len());
        let short_mcginley = mcginley_dynamic(latest_price, previous_short_mcginley, short_period);
        let macd = short_mcginley - long_mcginley;
        (macd, short_mcginley, long_mcginley)
    }

    /// Calculates the Chaikin Oscillator
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of price highs
    /// * `low` - Slice of price lows
    /// * `close` - Slice of closing prices
    /// * `volume` - Slice of transction volumes
    /// * `short_period` - Short period for the Accumulation Distribution
    /// * `previous_accumulation_distribution` - Previous accumulation distribution
    /// * `short_period_model` - Variant of [`ConstantModelType`]
    /// * `long_period_model` - Variant of [`ConstantModelType`]  
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `high.len()` != `low.len()` != `close.len()` != `volume.len()`
    ///     * If lengths <= `short_period`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let high = vec![103.0, 102.0, 105.0, 109.0, 106.0];
    /// let low = vec![99.0, 99.0, 100.0, 103.0, 98.0];
    /// let close = vec![102.0, 100.0, 103.0, 106.0, 100.0];
    /// let volume = vec![1000.0, 1500.0, 1200.0, 1500.0, 2000.0];
    /// let short_period: usize = 3;
    /// let previous = 0.0;
    ///
    /// let chaikin_oscillator =
    ///     centaur_technical_indicators::momentum_indicators::single::chaikin_oscillator(
    ///         &high,
    ///         &low,
    ///         &close,
    ///         &volume,
    ///         short_period,
    ///         previous,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage
    ///     );
    /// assert_eq!((-179.95937711577525, -760.0), chaikin_oscillator);
    ///
    /// let previous = 500.0;
    /// let chaikin_oscillator =
    ///     centaur_technical_indicators::momentum_indicators::single::chaikin_oscillator(
    ///         &high,
    ///         &low,
    ///         &close,
    ///         &volume,
    ///         short_period,
    ///         previous,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingAverage,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingMedian
    ///     );
    /// assert_eq!((-333.3333333333333, -260.0), chaikin_oscillator);
    /// ```
    pub fn chaikin_oscillator(
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
        short_period: usize,
        previous_accumulation_distribution: f64,
        short_period_model: ConstantModelType,
        long_period_model: ConstantModelType,
    ) -> (f64, f64) {
        let long_period = high.len();
        if long_period != low.len() || long_period != close.len() || long_period != volume.len() {
            panic!(
                "Length of highs ({}), lows ({}), close ({}), and volume ({}) must match",
                long_period,
                low.len(),
                close.len(),
                volume.len()
            )
        };

        if long_period <= short_period {
            panic!(
                "Long period ({}) cannot be smaller or equal to short period ({})",
                long_period, short_period
            )
        };

        let mut ad = Vec::with_capacity(long_period);
        ad.push(accumulation_distribution(
            high[0],
            low[0],
            close[0],
            volume[0],
            previous_accumulation_distribution,
        ));
        for i in 1..long_period {
            ad.push(accumulation_distribution(
                high[i],
                low[i],
                close[i],
                volume[i],
                ad[i - 1],
            ));
        }
        let short_period_slice = &ad[long_period - short_period..];

        let short_period_average = match short_period_model {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(short_period_slice, MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(short_period_slice, MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(short_period_slice, MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => moving_average(
                short_period_slice,
                MovingAverageType::Personalised {
                    alpha_num,
                    alpha_den,
                },
            ),
            ConstantModelType::SimpleMovingMedian => median(short_period_slice),
            ConstantModelType::SimpleMovingMode => mode(short_period_slice),
            _ => panic!("Unsupported ConstantModelType"),
        };

        let long_period_average = match long_period_model {
            ConstantModelType::SimpleMovingAverage => {
                moving_average(&ad, MovingAverageType::Simple)
            }
            ConstantModelType::SmoothedMovingAverage => {
                moving_average(&ad, MovingAverageType::Smoothed)
            }
            ConstantModelType::ExponentialMovingAverage => {
                moving_average(&ad, MovingAverageType::Exponential)
            }
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => moving_average(
                &ad,
                MovingAverageType::Personalised {
                    alpha_num,
                    alpha_den,
                },
            ),
            ConstantModelType::SimpleMovingMedian => median(&ad),
            ConstantModelType::SimpleMovingMode => mode(&ad),
            _ => panic!("Unsupported ConstantModelType"),
        };

        (short_period_average - long_period_average, ad[ad.len() - 1])
    }

    /// Calculates the Percentage Price Oscillator (PPO)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `short_period` - Length of short period
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     *  `prices.is_empty()`
    ///     * `prices.len()` <= `short_period`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 103.0, 106.0, 100.0, 97.0];
    /// let short_period: usize = 3;
    ///
    /// let percentage_price_oscillator =
    ///     centaur_technical_indicators::momentum_indicators::single::percentage_price_oscillator(
    ///         &prices,
    ///         short_period,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage
    ///     );
    ///
    /// assert_eq!(-1.0681349863189704 , percentage_price_oscillator);
    /// ```
    #[inline]
    pub fn percentage_price_oscillator(
        prices: &[f64],
        short_period: usize,
        constant_model_type: ConstantModelType,
    ) -> f64 {
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };
        let long_period = prices.len();
        if short_period > long_period {
            panic!(
                "Length of prices ({}) must be longer than short period ({})",
                long_period, short_period
            )
        };

        let short_period_slice = &prices[long_period - short_period..];
        let (short_period, long_period) = match constant_model_type {
            ConstantModelType::SimpleMovingAverage => (
                moving_average(short_period_slice, MovingAverageType::Simple),
                moving_average(prices, MovingAverageType::Simple),
            ),
            ConstantModelType::SmoothedMovingAverage => (
                moving_average(short_period_slice, MovingAverageType::Smoothed),
                moving_average(prices, MovingAverageType::Smoothed),
            ),
            ConstantModelType::ExponentialMovingAverage => (
                moving_average(short_period_slice, MovingAverageType::Exponential),
                moving_average(prices, MovingAverageType::Exponential),
            ),
            ConstantModelType::PersonalisedMovingAverage {
                alpha_num,
                alpha_den,
            } => (
                moving_average(
                    short_period_slice,
                    MovingAverageType::Personalised {
                        alpha_num,
                        alpha_den,
                    },
                ),
                moving_average(
                    prices,
                    MovingAverageType::Personalised {
                        alpha_num,
                        alpha_den,
                    },
                ),
            ),
            ConstantModelType::SimpleMovingMedian => (median(short_period_slice), median(prices)),
            ConstantModelType::SimpleMovingMode => (mode(short_period_slice), mode(prices)),
            _ => panic!("Unsupported ConstantModelType"),
        };

        ((short_period - long_period) / long_period) * 100.0
    }

    /// Calculates the Chande Momentum Oscillator
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
    /// let prices = vec![100.0, 103.0, 106.0, 100.0, 97.0];
    /// let chande_momentum_oscillator =
    ///     centaur_technical_indicators::momentum_indicators::single::chande_momentum_oscillator(&prices);
    /// assert_eq!(-20.0, chande_momentum_oscillator)
    /// ```
    #[inline]
    pub fn chande_momentum_oscillator(prices: &[f64]) -> f64 {
        let (previous_gains, previous_loss) = previous_gains_loss(prices);
        if previous_gains.is_empty() {
            return -100.0;
        }
        if previous_loss.is_empty() {
            return 100.0;
        }

        let gains_sum: f64 = previous_gains.iter().sum();
        let loss_sum: f64 = previous_loss.iter().sum();
        ((gains_sum - loss_sum) / (gains_sum + loss_sum)) * 100.0
    }

    #[inline]
    fn previous_gains_loss(prices: &[f64]) -> (Vec<f64>, Vec<f64>) {
        if prices.is_empty() {
            panic!("Prices is empty");
        };
        let len = prices.len();
        let mut previous_gains = Vec::with_capacity(len - 1);
        let mut previous_loss = Vec::with_capacity(len - 1);
        for i in 1..len {
            let diff = prices[i] - prices[i - 1];
            if diff > 0.0 {
                previous_gains.push(diff);
            } else if diff < 0.0 {
                previous_loss.push(prices[i - 1] - prices[i]);
            };
        }
        (previous_gains, previous_loss)
    }

    #[inline]
    fn cmp_f64(a: &f64, b: &f64) -> Ordering {
        if a < b {
            Ordering::Less
        } else if a > b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

/// **bulk**: Functions that compute values of a slice of prices over a period and return a vector.
pub mod bulk {
    use crate::momentum_indicators::single;
    use crate::{ConstantModelType, DeviationModel};

    /// Calculates the Relative strength index (RSI)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    /// * `period` - Period over which to calculate the RSI
    ///
    /// # Panics
    ///
    /// Panics `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let period: usize = 3;
    /// let defaut_rsi =
    ///     centaur_technical_indicators::momentum_indicators::bulk::relative_strength_index(
    ///         &prices,
    ///         centaur_technical_indicators::ConstantModelType::SmoothedMovingAverage,
    ///         period
    ///     );
    /// assert_eq!(vec![100.0, 33.33333333333333, 0.0], defaut_rsi);
    ///
    /// let ema_rsi =
    ///     centaur_technical_indicators::momentum_indicators::bulk::relative_strength_index(
    ///         &prices,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage,
    ///         period
    ///     );
    /// assert_eq!(vec![100.0, 33.33333333333333, 0.0], ema_rsi);
    ///
    /// let moving_median_rsi =
    ///     centaur_technical_indicators::momentum_indicators::bulk::relative_strength_index(
    ///         &prices,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingMedian,
    ///         period
    ///     );
    /// assert_eq!(vec![100.0, 33.33333333333333, 0.0], moving_median_rsi);
    ///
    /// let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
    /// let period: usize = 5;
    ///
    /// let personalised_rsi =
    ///     centaur_technical_indicators::momentum_indicators::bulk::relative_strength_index(
    ///         &prices,
    ///         centaur_technical_indicators::ConstantModelType::PersonalisedMovingAverage{alpha_num: 5.0, alpha_den:
    ///         4.0},
    ///         period
    ///     );
    /// assert_eq!(
    ///     vec![34.51776649746324, 12.837837837836929, 34.51776649746182, 61.26126126126377],
    ///     personalised_rsi
    /// );
    /// ```
    #[inline]
    pub fn relative_strength_index(
        prices: &[f64],
        constant_model_type: ConstantModelType,
        period: usize,
    ) -> Vec<f64> {
        let length = prices.len();
        if period > length {
            panic!(
                "Period ({}) is longer than length of prices ({})",
                period, length
            );
        };

        let mut rsis = Vec::with_capacity(length - period + 1);
        for window in prices.windows(period) {
            rsis.push(single::relative_strength_index(window, constant_model_type));
        }
        rsis
    }

    /// Calculates the Stochastic Oscillator
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `period` - Period over which to calculate the stochastic oscillator
    ///
    /// # Panics
    ///
    /// Panics if the `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let period: usize = 3;
    /// let stochastic_oscillator =
    ///     centaur_technical_indicators::momentum_indicators::bulk::stochastic_oscillator(&prices, period);
    /// assert_eq!(vec![100.0, 0.0, 0.0], stochastic_oscillator);
    /// ```
    #[inline]
    pub fn stochastic_oscillator(prices: &[f64], period: usize) -> Vec<f64> {
        let length = prices.len();
        if period > length {
            panic!(
                "Period ({}) cannot be greater than length ({}) of prices",
                period, length
            );
        };

        let mut so = Vec::with_capacity(length - period + 1);
        for window in prices.windows(period) {
            so.push(single::stochastic_oscillator(window));
        }
        so
    }

    /// Calculates the slow Stochastic
    ///
    /// # Arguments
    ///
    /// * `stochastics` - Slice of Stochastic Oscillators
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    /// * `period` - Period over which to calculate the slow stochastic
    ///
    /// # Panics
    ///
    /// Panics if `period` > `stochastics.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let stochstic_oscillators = [0.0, 25.0, 50.0, 33.0, 73.0];
    /// let period: usize = 3;
    ///
    /// let simple_ma_slow_stochastic =
    ///     centaur_technical_indicators::momentum_indicators::bulk::slow_stochastic(
    ///         &stochstic_oscillators,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingAverage,
    ///         period
    ///     );
    /// assert_eq!(vec![25.0, 36.0, 52.0], simple_ma_slow_stochastic);
    ///
    /// let sma_slow_stochastic =
    /// centaur_technical_indicators::momentum_indicators::bulk::slow_stochastic(
    ///     &stochstic_oscillators,
    ///     centaur_technical_indicators::ConstantModelType::SmoothedMovingAverage,
    ///     period
    /// );
    /// assert_eq!(
    ///     vec![31.578947368421055, 36.684210526315795, 55.526315789473685],
    ///     sma_slow_stochastic
    /// );
    ///
    /// let median_slow_stochastic =
    ///     centaur_technical_indicators::momentum_indicators::bulk::slow_stochastic(
    ///         &stochstic_oscillators,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingMedian,
    ///         period
    ///     );
    /// assert_eq!(vec![25.0, 33.0, 50.0], median_slow_stochastic);
    /// ```
    #[inline]
    pub fn slow_stochastic(
        stochastics: &[f64],
        constant_model_type: ConstantModelType,
        period: usize,
    ) -> Vec<f64> {
        let length = stochastics.len();
        if period > length {
            panic!(
                "Period ({}) cannot be greater than length ({}) of stochastics",
                period, length
            );
        };
        let mut sso = Vec::with_capacity(length - period + 1);
        for window in stochastics.windows(period) {
            sso.push(single::slow_stochastic(window, constant_model_type));
        }
        sso
    }

    /// Calculates the slowest Stochastic
    ///
    /// # Arguments
    ///
    /// * `slow_stochastics` - Slice of slow stochastics
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    /// * `period` - Period over which to calculate the slowest stochastic oscillator
    ///
    /// # Panics
    ///
    /// Panics if `period` > `slow_stochastics.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let slow_stochstics = [75.0, 60.0, 73.0, 58.0];
    /// let period: usize = 3;
    ///
    /// let ma_slowest_stochastic =
    ///     centaur_technical_indicators::momentum_indicators::bulk::slowest_stochastic(
    ///         &slow_stochstics,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingAverage,
    ///         period
    ///     );
    /// assert_eq!(vec![69.33333333333333, 63.666666666666664], ma_slowest_stochastic);
    ///
    /// let sma_slowest_stochastic =
    ///     centaur_technical_indicators::momentum_indicators::bulk::slow_stochastic(
    ///         &slow_stochstics,
    ///         centaur_technical_indicators::ConstantModelType::SmoothedMovingAverage,
    ///         period
    ///     );
    /// assert_eq!(vec![69.31578947368422, 63.15789473684211], sma_slowest_stochastic);
    ///
    /// let median_slowest_stochastic =
    ///     centaur_technical_indicators::momentum_indicators::bulk::slow_stochastic(
    ///         &slow_stochstics,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingMedian,
    ///         period
    ///     );
    /// assert_eq!(vec![73.0, 60.0], median_slowest_stochastic);
    /// ```
    #[inline]
    pub fn slowest_stochastic(
        slow_stochastics: &[f64],
        constant_model_type: ConstantModelType,
        period: usize,
    ) -> Vec<f64> {
        let length = slow_stochastics.len();
        if period > length {
            panic!(
                "Period ({}) cannot be greater than length ({}) of stochastics",
                period, length
            );
        };
        let mut sso = Vec::with_capacity(length - period + 1);
        for window in slow_stochastics.windows(period) {
            sso.push(single::slowest_stochastic(window, constant_model_type));
        }
        sso
    }

    /// Calculates the Williams %R
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `close` - Slice of closing prices
    /// * `period` - Period over which to calculate the Williams %R
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `high.len()` != `low.len()` != `close.len()`
    ///     * `period` > lengths
    ///
    /// # Examples
    ///
    /// ```rust
    /// let high = vec![200.0, 210.0, 205.0, 190.0];
    /// let low = vec![175.0, 192.0, 200.0, 174.0];
    /// let close = vec![192.0, 200.0, 201.0, 187.0];
    /// let period: usize = 3;
    /// let williams_percent_r =
    ///     centaur_technical_indicators::momentum_indicators::bulk::williams_percent_r(
    ///         &high,
    ///         &low,
    ///         &close,
    ///         period
    ///     );
    /// assert_eq!(vec![-25.71428571428571, -63.888888888888886], williams_percent_r);
    /// ```
    #[inline]
    pub fn williams_percent_r(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Vec<f64> {
        let length = close.len();
        if length != high.len() || length != low.len() {
            panic!(
                "Length of close ({}) needs to match length of high ({}), and length of close ({})",
                length,
                high.len(),
                low.len()
            );
        };
        if period > length {
            panic!(
                "Length of price ({}) must be greater or equal to period ({})",
                length, period
            )
        };

        let loop_max = length - period + 1;
        let mut wprs = Vec::with_capacity(loop_max);
        for i in 0..loop_max {
            wprs.push(single::williams_percent_r(
                &high[i..i + period],
                &low[i..i + period],
                close[i + period - 1],
            ));
        }
        wprs
    }

    /// Calculates the Money Flow Index (MFI)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `volume` - Slice of volumes
    /// * `period` - Period over which to calculate the MFI
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `period` > `prices.len()`
    ///     * `prices.len()` != `volume.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let volume = vec![1000.0, 1500.0, 1200.0, 900.0, 1300.0];
    /// let period: usize = 3;
    /// let money_flow_index =
    ///     centaur_technical_indicators::momentum_indicators::bulk::money_flow_index(
    ///         &prices,
    ///         &volume,
    ///         period
    ///     );
    /// assert_eq!(vec![55.314533622559644, 0.0, 58.60655737704918], money_flow_index);
    /// ```
    #[inline]
    pub fn money_flow_index(prices: &[f64], volume: &[f64], period: usize) -> Vec<f64> {
        let length = prices.len();
        if period > length {
            panic!(
                "Period ({}) cannot be longer than length of prices ({})",
                period, length
            );
        };
        if length != volume.len() {
            panic!(
                "Length of prices ({}) must match length of volume ({})",
                length,
                volume.len()
            );
        };

        let loop_max = length - period + 1;
        let mut mfis = Vec::with_capacity(loop_max);

        for i in 0..loop_max {
            mfis.push(single::money_flow_index(
                &prices[i..i + period],
                &volume[i..i + period],
            ));
        }
        mfis
    }

    /// Calculates the Rate of Change (RoC)
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
    /// let prices = vec![100.0, 120.0, 100.0];
    /// let rate_of_change = centaur_technical_indicators::momentum_indicators::bulk::rate_of_change(&prices);
    /// assert_eq!(vec![20.0, -16.666666666666664], rate_of_change);
    /// ```
    #[inline]
    pub fn rate_of_change(prices: &[f64]) -> Vec<f64> {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
        }
        let mut rocs = Vec::with_capacity(prices.len() - 1);
        for pair in prices.windows(2) {
            rocs.push(single::rate_of_change(pair[1], pair[0]));
        }
        rocs
    }

    /// Calculates the On Balance Volume (OBV)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `volume` - Slice of volumes
    /// * `previous_on_balance_volume` - Previous OBV (use 0.0 if none)
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `prices.is_empty()`
    ///     * `prices.len()` != `volume.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0];
    /// let volume = vec![1000.0, 1500.0, 1200.0, 900.0, 1300.0, 1400.0];
    /// let on_balance_volume =
    ///     centaur_technical_indicators::momentum_indicators::bulk::on_balance_volume(
    ///         &prices,
    ///         &volume,
    ///         0.0
    ///     );
    /// assert_eq!(vec![1500.0, 2700.0, 1800.0, 500.0, 500.0], on_balance_volume);
    /// ```
    #[inline]
    pub fn on_balance_volume(
        prices: &[f64],
        volume: &[f64],
        previous_on_balance_volume: f64,
    ) -> Vec<f64> {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
        };
        let length = prices.len();
        if length != volume.len() {
            panic!(
                "Length of prices ({}) and volume ({}) must match",
                length,
                volume.len()
            );
        };
        let mut obvs = Vec::with_capacity(length - 1);
        let mut obv =
            single::on_balance_volume(prices[1], prices[0], volume[1], previous_on_balance_volume);
        obvs.push(obv);
        for i in 2..length {
            obv = single::on_balance_volume(prices[i], prices[i - 1], volume[i], obv);
            obvs.push(obv);
        }
        obvs
    }

    /// Calculates the Commodity Channel Index (CCI)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    /// * `deviation_model` - Variant of [`DeviationModel`]
    /// * `constant_multiplier` - Scale factor (normally 0.015)
    /// * `period` - Period over which to calculate the CCI
    ///
    /// # Panics
    ///
    /// Panics if `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0];
    /// let constant_multiplier = 0.015;
    /// let period: usize = 3;
    ///
    /// let default_cci =
    ///     centaur_technical_indicators::momentum_indicators::bulk::commodity_channel_index(
    ///         &prices,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingAverage,
    ///         centaur_technical_indicators::DeviationModel::MeanAbsoluteDeviation,
    ///         constant_multiplier,
    ///         period
    ///     );
    /// assert_eq!(
    ///     vec![79.99999999999983, -100.00000000000001, -100.00000000000001],
    ///     default_cci
    /// );
    ///
    /// let ema_sd_cci =
    ///     centaur_technical_indicators::momentum_indicators::bulk::commodity_channel_index(
    ///         &prices,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage,
    ///         centaur_technical_indicators::DeviationModel::StandardDeviation,
    ///         constant_multiplier,
    ///         period
    ///     );
    /// assert_eq!(
    ///     vec![38.1801774160603, -58.32118435197994, -46.65694748158418],
    ///     ema_sd_cci
    /// );
    ///
    /// let median_mad_cci =
    ///     centaur_technical_indicators::momentum_indicators::bulk::commodity_channel_index(
    ///         &prices,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingMedian,
    ///         centaur_technical_indicators::DeviationModel::MedianAbsoluteDeviation,
    ///         constant_multiplier,
    ///         period
    ///     );  
    /// assert_eq!(
    ///     vec![66.66666666666667, -66.66666666666667, -66.66666666666667],
    ///     median_mad_cci
    /// );
    /// ```
    #[inline]
    pub fn commodity_channel_index(
        prices: &[f64],
        constant_model_type: ConstantModelType,
        deviation_model: DeviationModel,
        constant_multiplier: f64,
        period: usize,
    ) -> Vec<f64> {
        let length = prices.len();
        if period > length {
            panic!(
                "Period ({}) cannot be longer than lenght ({}) of prices",
                period, length
            );
        };

        let mut ccis = Vec::with_capacity(length - period + 1);
        for window in prices.windows(period) {
            ccis.push(single::commodity_channel_index(
                window,
                constant_model_type,
                deviation_model,
                constant_multiplier,
            ));
        }
        ccis
    }

    /// Calculates the McGinley dynamic Commodity Channel Index
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `previous_mcginley_dynamic` - Previous McGinley dynamic (0.0 if none)
    /// * `deviation_model` - Variant of [`DeviationModel`]
    /// * `constant_multiplier` - Scale factor (normally 0.015)
    /// * `period` - The period over which to calculate the CCI
    ///
    /// # Panics
    ///
    /// Panics if `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 102.0];
    /// let constant_multiplier = 0.015;
    ///
    /// let mcginley_cci =
    ///     centaur_technical_indicators::momentum_indicators::bulk::mcginley_dynamic_commodity_channel_index(
    ///         &prices,
    ///         0.0,
    ///         centaur_technical_indicators::DeviationModel::MeanAbsoluteDeviation,
    ///         constant_multiplier,
    ///         5_usize
    ///     );
    /// assert_eq!(vec![(0.0, 99.0), (146.8770632107927, 99.53246533805869)], mcginley_cci);
    /// ```
    #[inline]
    pub fn mcginley_dynamic_commodity_channel_index(
        prices: &[f64],
        previous_mcginley_dynamic: f64,
        deviation_model: DeviationModel,
        constant_multiplier: f64,
        period: usize,
    ) -> Vec<(f64, f64)> {
        let length = prices.len();
        if period > length {
            panic!(
                "Period ({}) cannot be longer the length of prices ({})",
                period, length
            );
        };

        let mut ccis = Vec::with_capacity(length - period + 1);
        let mut cci = single::mcginley_dynamic_commodity_channel_index(
            &prices[..period],
            previous_mcginley_dynamic,
            deviation_model,
            constant_multiplier,
        );
        ccis.push(cci);
        for i in 1..=length - period {
            cci = single::mcginley_dynamic_commodity_channel_index(
                &prices[i..i + period],
                cci.1,
                deviation_model,
                constant_multiplier,
            );
            ccis.push(cci);
        }
        ccis
    }

    /// Calculates the Moving Average Convergence Divergence (MACD) line
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `short_period` - The length of the short period
    /// * `short_period_model` - Variant of [`ConstantModelType`]
    /// * `long_period` - The length of the long period
    /// * `long_period_model` - Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * `short_period` > `long_period`
    /// * `long_period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0];
    /// let short_period: usize = 3;
    /// let long_period: usize = 5;
    ///
    /// let macd =
    ///     centaur_technical_indicators::momentum_indicators::bulk::macd_line(
    ///         &prices,
    ///         short_period,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage,
    ///         long_period,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage
    ///     );
    /// assert_eq!(
    ///     vec![-0.46851726472581845, -0.7379823967501835, 0.031821259309410266],
    ///     macd
    /// );
    ///
    /// let macd =
    ///     centaur_technical_indicators::momentum_indicators::bulk::macd_line(
    ///         &prices,
    ///         short_period,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingAverage,
    ///         long_period,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingMedian);
    /// assert_eq!(vec![0.0, -1.3333333333333286, -1.0], macd);
    /// ```
    #[inline]
    pub fn macd_line(
        prices: &[f64],
        short_period: usize,
        short_period_model: ConstantModelType,
        long_period: usize,
        long_period_model: ConstantModelType,
    ) -> Vec<f64> {
        if short_period > long_period {
            panic!(
                "Short period ({}) cannot be longer than long period ({})",
                short_period, long_period
            )
        };

        let length = prices.len();
        if long_period > length {
            panic!(
                "Long period ({}) cannot be shorter than length of prices ({})",
                long_period, length
            );
        };

        let mut macds = Vec::with_capacity(length - long_period + 1);
        for window in prices.windows(long_period) {
            macds.push(single::macd_line(
                window,
                short_period,
                short_period_model,
                long_period_model,
            ));
        }
        macds
    }

    /// Calculates the MACD signal line
    /// divergence.
    ///
    /// # Arguments
    ///
    /// * `macds` - Slice of MACDs
    /// * `constant_model_type` - Variant of [`ConstantModelType`]
    /// * `period` - Period over which to calculate the signal line
    ///
    /// # Panics
    ///
    /// Panics if `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let macds = vec![
    ///     -0.0606702775897219, -0.0224170616113781, 0.0057887610020515,
    ///     0.0318212593094103, -0.737982396750169
    /// ];
    /// let period: usize = 3;
    ///
    /// let ema_signal_line =
    ///     centaur_technical_indicators::momentum_indicators::bulk::signal_line(
    ///         &macds,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage,
    ///         period
    ///     );
    /// assert_eq!(
    ///     vec![-0.011764193829181728, 0.0166350710900523, -0.4117854724828291],
    ///     ema_signal_line
    /// );
    ///
    /// let median_signal_line =
    ///     centaur_technical_indicators::momentum_indicators::bulk::signal_line(
    ///         &macds,
    ///         centaur_technical_indicators::ConstantModelType::SimpleMovingMedian,
    ///         period
    ///     );
    /// assert_eq!(
    ///     vec![-0.0224170616113781, 0.0057887610020515, 0.0057887610020515],
    ///     median_signal_line
    /// );
    /// ```
    #[inline]
    pub fn signal_line(
        macds: &[f64],
        constant_model_type: ConstantModelType,
        period: usize,
    ) -> Vec<f64> {
        let length = macds.len();
        if period > length {
            panic!(
                "Period ({}) cannot be longer than length ({}) of prices",
                period, length
            )
        };

        let mut signals = Vec::with_capacity(length - period + 1);
        for window in macds.windows(period) {
            signals.push(single::signal_line(window, constant_model_type));
        }
        signals
    }

    /// Calculates the McGinley dynamic MACD line
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `short_period` - The length of the short period
    /// * `previous_short_mcginley` - Previous short model McGinley dynamic (if none use 0.0)
    /// * `long_period` - The length of the long period
    /// * `previous_long_mcginley` - Previous long model McGinley dynamic (if none use 0.0)
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `prices.is_empty()`
    ///     * `long_period` > `prices.len()`
    ///     * `short_period` >= `long_period`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0];
    /// let short_period: usize = 3;
    /// let long_period: usize = 5;
    /// let mcginley_dynamic_macd =
    ///     centaur_technical_indicators::momentum_indicators::bulk::mcginley_dynamic_macd_line(
    ///         &prices,
    ///         short_period,
    ///         0.0,
    ///         long_period,
    ///         0.0
    ///     );
    /// assert_eq!(
    ///     vec![
    ///         (0.0, 99.0, 99.0),
    ///         (0.0, 99.0, 99.0),
    ///         (0.35497689203913296, 99.88744223009782, 99.53246533805869)
    ///     ], mcginley_dynamic_macd
    /// );
    /// ```
    #[inline]
    pub fn mcginley_dynamic_macd_line(
        prices: &[f64],
        short_period: usize,
        previous_short_mcginley: f64,
        long_period: usize,
        previous_long_mcginley: f64,
    ) -> Vec<(f64, f64, f64)> {
        if prices.is_empty() {
            panic!("Prices cannot be empty");
        };

        let length = prices.len();
        if long_period > length {
            panic!(
                "Long period ({}) cannot be longer than length ({}) of prices",
                long_period, length
            );
        };

        if short_period >= long_period {
            panic!(
                "Short period ({}) cannot be greater or equal to long period ({})",
                short_period, long_period
            );
        };

        let loop_max = length - long_period + 1;
        let mut macds = Vec::with_capacity(loop_max);
        let mut macd = single::mcginley_dynamic_macd_line(
            &prices[..long_period],
            short_period,
            previous_short_mcginley,
            previous_long_mcginley,
        );
        macds.push(macd);

        for i in 1..loop_max {
            macd = single::mcginley_dynamic_macd_line(
                &prices[i..long_period + i],
                short_period,
                macd.1,
                macd.2,
            );
            macds.push(macd);
        }
        macds
    }

    /// Calculates the  Chaikin Oscillator (CO)
    ///
    /// # Arguments
    ///
    /// * `high` - Slice of highs
    /// * `low` - Slice of lows
    /// * `close` - Slice of closing prices
    /// * `volume` - Slice of volumes
    /// * `short_period` - Short period over which to calculate the AD
    /// * `long_period` - Long period over which to calculate the AD
    /// * `previous_accumulation_distribution` - Previous AD value (if none use 0.0)
    /// * `short_period_model` - Variant of [`ConstantModelType`]
    /// * `long_period_model` - Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * `high.len()` != `low.len()` != `close.len()` != `volume.len()`
    /// * `long_period` > lengths
    /// * `short_period` >= `long_period`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let high = vec![103.0, 102.0, 105.0, 109.0, 106.0, 102.0, 107.0];
    /// let low = vec![99.0, 99.0, 100.0, 103.0, 98.0, 94.0, 96.0];
    /// let close = vec![102.0, 100.0, 103.0, 106.0, 100.0, 97.0, 105.0];
    /// let volume = vec![1000.0, 1500.0, 1200.0, 1500.0, 2000.0, 3000.0, 1250.0];
    /// let short_period: usize = 3;
    /// let long_period: usize = 5;
    /// let previous = 0.0;
    ///
    /// let chaikin_oscillator = centaur_technical_indicators::momentum_indicators::bulk::chaikin_oscillator(
    ///     &high,
    ///     &low,
    ///     &close,
    ///     &volume,
    ///     short_period,
    ///     long_period,
    ///     previous,
    ///     centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage,
    ///     centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage
    /// );
    /// assert_eq!(
    ///     vec![(-179.95937711577525, -760.0), (-339.790115098172, -2770.0), (-203.39139533452362, -3484.5454545454545)],
    ///     chaikin_oscillator
    /// );
    ///
    /// let previous = 500.0;
    /// let chaikin_oscillator = centaur_technical_indicators::momentum_indicators::bulk::chaikin_oscillator(
    ///     &high,
    ///     &low,
    ///     &close,
    ///     &volume,
    ///     short_period,
    ///     long_period,
    ///     previous,
    ///     centaur_technical_indicators::ConstantModelType::SimpleMovingAverage,
    ///     centaur_technical_indicators::ConstantModelType::SimpleMovingMedian
    ///     );
    /// assert_eq!(
    ///     vec![(-333.3333333333333, -260.0), (-676.6666666666667, -2270.0), (-280.30303030303, -2984.5454545454545)],
    ///     chaikin_oscillator
    /// );
    /// ```
    #[inline]
    pub fn chaikin_oscillator(
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
        short_period: usize,
        long_period: usize,
        previous_accumulation_distribution: f64,
        short_period_model: ConstantModelType,
        long_period_model: ConstantModelType,
    ) -> Vec<(f64, f64)> {
        let length = high.len();
        if length != low.len() || length != close.len() || length != volume.len() {
            panic!(
                "Lengths of highs ({}), lows ({}), close ({}), and volume ({}) must match",
                length,
                low.len(),
                close.len(),
                volume.len()
            )
        };

        if length < long_period {
            panic!(
                "Length of prices ({}) must me greater or equal to long period ({})",
                length, long_period
            )
        };

        if short_period >= long_period {
            panic!(
                "Short period ({}) cannot be greater or equal to long period ({})",
                short_period, long_period
            );
        };

        let loop_max = length - long_period + 1;
        let mut cos = Vec::with_capacity(loop_max);
        let mut co = single::chaikin_oscillator(
            &high[..long_period],
            &low[..long_period],
            &close[..long_period],
            &volume[..long_period],
            short_period,
            previous_accumulation_distribution,
            short_period_model,
            long_period_model,
        );
        cos.push(co);

        for i in 1..loop_max {
            co = single::chaikin_oscillator(
                &high[i..i + long_period],
                &low[i..i + long_period],
                &close[i..i + long_period],
                &volume[i..i + long_period],
                short_period,
                co.1,
                short_period_model,
                long_period_model,
            );
            cos.push(co);
        }
        cos
    }

    /// Calculates the Percentage Price Oscillator (PPO)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `short_period` - Length of short period.
    /// * `long_period` - Length of long period
    /// * `constant_model_type` Variant of [`ConstantModelType`]
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `short_period` >= `long_period`
    ///     * `prices.is_empty()`
    ///     * 'long_period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 103.0, 106.0, 100.0, 97.0, 105.0, 102.0];
    /// let short_period: usize = 3;
    /// let long_period: usize = 5;
    ///
    /// let percentage_price_oscillator =
    ///     centaur_technical_indicators::momentum_indicators::bulk::percentage_price_oscillator(
    ///         &prices,
    ///         short_period,
    ///         long_period,
    ///         centaur_technical_indicators::ConstantModelType::ExponentialMovingAverage
    ///     );
    ///
    /// assert_eq!(
    ///     vec![-1.0681349863189704, -0.06036684467148623, 0.14936271906531415],
    ///     percentage_price_oscillator
    /// );
    /// ```
    #[inline]
    pub fn percentage_price_oscillator(
        prices: &[f64],
        short_period: usize,
        long_period: usize,
        constant_model_type: ConstantModelType,
    ) -> Vec<f64> {
        let length = prices.len();
        if short_period >= long_period {
            panic!(
                "Length of prices ({}) must be longer than short period ({})",
                long_period, short_period
            )
        };
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };
        if long_period > length {
            panic!(
                "Long period ({}) cannot be greater than length of prices ({})",
                long_period, length
            )
        };

        let mut ppos = Vec::with_capacity(length - long_period + 1);

        for window in prices.windows(long_period) {
            ppos.push(single::percentage_price_oscillator(
                window,
                short_period,
                constant_model_type,
            ));
        }
        ppos
    }

    /// Calculates the Chande Momentum Oscillator (CMO)
    ///
    /// # Arguments
    ///
    /// * `prices` - Slice of prices
    /// * `period` - Period over which to calculate the CMO
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     * `prices.is_empty()`
    ///     * `period` > `prices.len()`
    ///
    /// # Examples
    ///
    /// ```rust
    /// let prices = vec![100.0, 103.0, 106.0, 100.0, 97.0, 105.0, 102.0];
    /// let chande_momentum_oscillator =
    ///     centaur_technical_indicators::momentum_indicators::bulk::chande_momentum_oscillator(
    ///         &prices,
    ///         5_usize
    ///     );
    /// assert_eq!(vec![-20.0, 10.0, -20.0], chande_momentum_oscillator)
    /// ```
    #[inline]
    pub fn chande_momentum_oscillator(prices: &[f64], period: usize) -> Vec<f64> {
        let length = prices.len();
        if prices.is_empty() {
            panic!("Prices cannot be empty")
        };
        if period > length {
            panic!(
                "Period ({}) cannot be greater than length of prices ({})",
                period, length
            )
        };

        let mut cmos = Vec::with_capacity(length - period + 1);
        for window in prices.windows(period) {
            cmos.push(single::chande_momentum_oscillator(window));
        }
        cmos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_ma_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            49.2537313432832,
            single::relative_strength_index(&prices, crate::ConstantModelType::SimpleMovingAverage)
        );
    }

    #[test]
    fn single_short_median_rsi() {
        // Because there are too few values, ends up being the means
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            49.2537313432832,
            single::relative_strength_index(&prices, crate::ConstantModelType::SimpleMovingMedian)
        );
    }

    #[test]
    fn single_long_median_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        assert_eq!(
            37.5,
            single::relative_strength_index(&prices, crate::ConstantModelType::SimpleMovingMedian)
        );
    }

    #[test]
    fn single_small_mode_rsi() {
        // Mode rounds the values, the difference being so small all rounds down to 0.0
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            0.0,
            single::relative_strength_index(&prices, crate::ConstantModelType::SimpleMovingMode)
        );
    }

    #[test]
    fn single_large_mode_rsi() {
        let prices = vec![100.0, 103.0, 106.0, 107.0, 108.0, 105.0, 102.0];
        assert_eq!(
            39.99999999999999,
            single::relative_strength_index(&prices, crate::ConstantModelType::SimpleMovingMode)
        );
    }

    #[test]
    fn single_smoothed_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            43.01075268817234,
            single::relative_strength_index(
                &prices,
                crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn single_exponential_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            39.495798319328436,
            single::relative_strength_index(
                &prices,
                crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn single_personalised_rsi() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(
            35.6725146198842,
            single::relative_strength_index(
                &prices,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 4.0,
                    alpha_den: 3.0
                }
            )
        );
    }

    #[test]
    fn single_only_price_rise_rsi() {
        let prices = vec![100.0, 101.0, 102.0, 103.0];
        assert_eq!(
            100.0,
            single::relative_strength_index(&prices, crate::ConstantModelType::SimpleMovingAverage)
        );
    }

    #[test]
    fn single_only_price_fall_rsi() {
        let prices = vec![103.0, 102.0, 101.0, 100.0];
        assert_eq!(
            0.0,
            single::relative_strength_index(&prices, crate::ConstantModelType::SimpleMovingAverage)
        );
    }

    #[test]
    #[should_panic]
    fn single_rsi_panic() {
        let prices = Vec::new();
        single::relative_strength_index(&prices, crate::ConstantModelType::SimpleMovingAverage);
    }

    #[test]
    fn bulk_simple_ma_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                49.2537313432832,
                20.930232558140005,
                27.6595744680842,
                36.111111111111335
            ],
            bulk::relative_strength_index(
                &prices,
                crate::ConstantModelType::SimpleMovingAverage,
                period
            )
        );
    }

    #[test]
    fn bulk_smoothed_ma_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                43.01075268817234,
                17.187499999999886,
                31.168831168830664,
                47.05882352941291
            ],
            bulk::relative_strength_index(
                &prices,
                crate::ConstantModelType::SmoothedMovingAverage,
                period
            )
        );
    }

    #[test]
    fn bulk_exponential_ma_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                39.495798319328436,
                15.2941176470584,
                32.71028037383145,
                53.03030303030472
            ],
            bulk::relative_strength_index(
                &prices,
                crate::ConstantModelType::ExponentialMovingAverage,
                period
            )
        );
    }

    #[test]
    fn bulk_personalised_ma_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                35.6725146198842,
                13.385826771652745,
                34.13173652694594,
                59.375000000002316
            ],
            bulk::relative_strength_index(
                &prices,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 4.0,
                    alpha_den: 3.0
                },
                period
            )
        );
    }

    #[test]
    fn bulk_simple_median_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![
                49.2537313432832,
                20.930232558140005,
                27.6595744680842,
                36.111111111111335
            ],
            bulk::relative_strength_index(
                &prices,
                crate::ConstantModelType::SimpleMovingMedian,
                period
            )
        );
    }

    #[test]
    fn bulk_simple_mode_rsi() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![0.0, 0.0, 0.0, 0.0],
            bulk::relative_strength_index(
                &prices,
                crate::ConstantModelType::SimpleMovingMode,
                period
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_rsi_panic() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 50;
        bulk::relative_strength_index(
            &prices,
            crate::ConstantModelType::SimpleMovingAverage,
            period,
        );
    }

    #[test]
    fn single_stochastic_oscillator_min() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!(0.0, single::stochastic_oscillator(&prices));
    }

    #[test]
    fn single_stochastic_oscillator_max() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.66];
        assert_eq!(100.0, single::stochastic_oscillator(&prices));
    }

    #[test]
    fn single_stochastic_oscillator() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.34];
        assert_eq!(42.42424242424281, single::stochastic_oscillator(&prices));
    }

    #[test]
    #[should_panic]
    fn single_stochastic_oscillator_panic() {
        let prices = Vec::new();
        single::stochastic_oscillator(&prices);
    }

    #[test]
    fn bulk_stochastic_oscillator() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 5;
        assert_eq!(
            vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394],
            bulk::stochastic_oscillator(&prices, period)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_stochastic_oscillator_bulk() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let period: usize = 50;
        bulk::stochastic_oscillator(&prices, period);
    }

    #[test]
    fn single_ma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.871517027863632,
            single::slow_stochastic(&stochastics, crate::ConstantModelType::SimpleMovingAverage)
        );
    }

    #[test]
    fn single_sma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            29.02078726227347,
            single::slow_stochastic(
                &stochastics,
                crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn single_ema_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            33.284579311601206,
            single::slow_stochastic(
                &stochastics,
                crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn single_pma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            39.872151259403616,
            single::slow_stochastic(
                &stochastics,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                }
            )
        );
    }

    #[test]
    fn single_median_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.05882352941029,
            single::slow_stochastic(&stochastics, crate::ConstantModelType::SimpleMovingMedian)
        );
    }

    #[test]
    fn single_mode_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.75,
            single::slow_stochastic(&stochastics, crate::ConstantModelType::SimpleMovingMode)
        );
    }

    #[test]
    #[should_panic]
    fn single_slow_stochastic_panic() {
        let stochastics = Vec::new();
        single::slow_stochastic(&stochastics, crate::ConstantModelType::SimpleMovingMode);
    }

    #[test]
    fn bulk_ma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![14.705882352940193, 30.49535603715151],
            bulk::slow_stochastic(
                &stochastics,
                crate::ConstantModelType::SimpleMovingAverage,
                period
            )
        );
    }

    #[test]
    fn bulk_sma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![19.969040247676816, 35.75036662864623],
            bulk::slow_stochastic(
                &stochastics,
                crate::ConstantModelType::SmoothedMovingAverage,
                period
            )
        );
    }

    #[test]
    fn bulk_ema_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![23.529411764704548, 38.832375055285944],
            bulk::slow_stochastic(
                &stochastics,
                crate::ConstantModelType::ExponentialMovingAverage,
                period
            )
        );
    }

    #[test]
    fn bulk_pma_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![29.192273924493655, 42.98322628344476],
            bulk::slow_stochastic(
                &stochastics,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                },
                period
            )
        );
    }

    #[test]
    fn bulk_median_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![5.882352941175241, 38.23529411764534],
            bulk::slow_stochastic(
                &stochastics,
                crate::ConstantModelType::SimpleMovingMedian,
                period
            )
        );
    }

    #[test]
    fn bulk_mode_slow_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![14.666666666666666, 30.3333333333333332],
            bulk::slow_stochastic(
                &stochastics,
                crate::ConstantModelType::SimpleMovingMode,
                period
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_slow_stochastic_panic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 30;
        bulk::slow_stochastic(
            &stochastics,
            crate::ConstantModelType::SimpleMovingMode,
            period,
        );
    }

    #[test]
    fn single_ma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.871517027863632,
            single::slowest_stochastic(&stochastics, crate::ConstantModelType::SimpleMovingAverage)
        );
    }

    #[test]
    fn single_sma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            29.02078726227347,
            single::slowest_stochastic(
                &stochastics,
                crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn single_ema_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            33.284579311601206,
            single::slowest_stochastic(
                &stochastics,
                crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn single_pma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            39.872151259403616,
            single::slowest_stochastic(
                &stochastics,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                }
            )
        );
    }

    #[test]
    fn single_median_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.05882352941029,
            single::slowest_stochastic(&stochastics, crate::ConstantModelType::SimpleMovingMedian)
        );
    }

    #[test]
    fn single_mode_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        assert_eq!(
            22.75,
            single::slowest_stochastic(&stochastics, crate::ConstantModelType::SimpleMovingMode)
        );
    }

    #[test]
    #[should_panic]
    fn single_slowest_stochastic_panic() {
        let stochastics = Vec::new();
        single::slowest_stochastic(&stochastics, crate::ConstantModelType::SimpleMovingMode);
    }

    #[test]
    fn bulk_ma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![14.705882352940193, 30.49535603715151],
            bulk::slowest_stochastic(
                &stochastics,
                crate::ConstantModelType::SimpleMovingAverage,
                period
            )
        );
    }

    #[test]
    fn bulk_sma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![19.969040247676816, 35.75036662864623],
            bulk::slowest_stochastic(
                &stochastics,
                crate::ConstantModelType::SmoothedMovingAverage,
                period
            )
        );
    }

    #[test]
    fn bulk_ema_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![23.529411764704548, 38.832375055285944],
            bulk::slowest_stochastic(
                &stochastics,
                crate::ConstantModelType::ExponentialMovingAverage,
                period
            )
        );
    }

    #[test]
    fn bulk_pma_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![29.192273924493655, 42.98322628344476],
            bulk::slowest_stochastic(
                &stochastics,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                },
                period
            )
        );
    }

    #[test]
    fn bulk_median_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![5.882352941175241, 38.23529411764534],
            bulk::slowest_stochastic(
                &stochastics,
                crate::ConstantModelType::SimpleMovingMedian,
                period
            )
        );
    }

    #[test]
    fn bulk_mode_slowest_stochastic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 3;
        assert_eq!(
            vec![14.666666666666666, 30.3333333333333332],
            bulk::slowest_stochastic(
                &stochastics,
                crate::ConstantModelType::SimpleMovingMode,
                period
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_slowest_stochastic_panic() {
        let stochastics = vec![0.0, 5.882352941175241, 38.23529411764534, 47.36842105263394];
        let period: usize = 30;
        bulk::slowest_stochastic(
            &stochastics,
            crate::ConstantModelType::SimpleMovingMode,
            period,
        );
    }

    #[test]
    fn single_williams_percent_r() {
        let high = [100.93, 101.58, 101.25];
        let low = [100.37, 100.57, 100.94];
        let close = 101.13;
        assert_eq!(
            -37.190082644628525,
            single::williams_percent_r(&high, &low, close)
        );
    }

    #[test]
    fn bulk_williams_percent_r() {
        let high = vec![100.93, 101.58, 101.25, 101.72];
        let low = vec![100.37, 100.57, 100.94, 100.89];
        let close = vec![100.49, 101.06, 101.13, 100.95];
        assert_eq!(
            vec![-37.190082644628525, -66.95652173912976],
            bulk::williams_percent_r(&high, &low, &close, 3_usize)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_williams_percent_r_high_panic() {
        let high = vec![101.58, 101.25];
        let low = vec![100.37, 100.57, 100.94];
        let close = vec![100.49, 101.06, 101.13];
        bulk::williams_percent_r(&high, &low, &close, 3_usize);
    }

    #[test]
    #[should_panic]
    fn bulk_williams_percent_r_low_panic() {
        let high = vec![100.93, 101.58, 101.25];
        let low = vec![100.37, 100.57, 100.94, 100.59];
        let close = vec![100.49, 101.06, 101.13];
        bulk::williams_percent_r(&high, &low, &close, 3_usize);
    }

    #[test]
    #[should_panic]
    fn bulk_williams_percent_r_close_panic() {
        let high = vec![100.93, 101.58, 101.25];
        let low = vec![100.57, 100.94, 100.59];
        let close = vec![101.06, 101.13];
        bulk::williams_percent_r(&high, &low, &close, 3_usize);
    }

    #[test]
    #[should_panic]
    fn bulk_williams_percent_r_period_panic() {
        let high = vec![101.58, 101.25, 100.93];
        let low = vec![100.37, 100.57, 100.94];
        let close = vec![100.49, 101.06, 101.13];
        bulk::williams_percent_r(&high, &low, &close, 30_usize);
    }

    #[test]
    fn single_money_flow_index() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let volume = vec![1200.0, 1400.0, 1450.0, 1100.0, 900.0, 875.0, 1025.0, 1100.0];
        assert_eq!(
            63.40886336843541,
            single::money_flow_index(&prices, &volume)
        );
    }

    #[test]
    fn single_money_flow_index_only_negative() {
        let prices = vec![100.38, 100.19, 100.12];
        let volume = vec![1100.0, 900.0, 875.0];
        assert_eq!(0.0, single::money_flow_index(&prices, &volume));
    }

    #[test]
    fn single_money_flow_index_only_positive() {
        let prices = vec![100.2, 100.46, 100.53];
        let volume = vec![1200.0, 1400.0, 1450.0];
        assert_eq!(100.0, single::money_flow_index(&prices, &volume));
    }

    #[test]
    #[should_panic]
    fn single_money_flow_index_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        let volume = vec![1200.0, 1400.0, 1450.0, 1100.0, 900.0, 875.0, 1025.0, 1100.0];
        single::money_flow_index(&prices, &volume);
    }

    #[test]
    #[should_panic]
    fn single_money_flow_index_empty_price_panic() {
        let prices = Vec::new();
        let volume = vec![1200.0, 1400.0, 1450.0, 1100.0, 900.0, 875.0, 1025.0, 1100.0];
        single::money_flow_index(&prices, &volume);
    }

    #[test]
    #[should_panic]
    fn single_money_flow_index_empty_volume_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        let volume = Vec::new();
        single::money_flow_index(&prices, &volume);
    }

    #[test]
    fn bulk_money_flow_index() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let volume = vec![1200.0, 1400.0, 1450.0, 1100.0, 900.0, 875.0, 1025.0, 1100.0];
        let period: usize = 5;
        assert_eq!(
            vec![
                58.811420498704834,
                33.5840199520207,
                26.291946512503486,
                54.5117755343317
            ],
            bulk::money_flow_index(&prices, &volume, period)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_money_flow_index_length_panic() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let volume = vec![1400.0, 1450.0, 1100.0, 900.0, 875.0, 1025.0, 1100.0];
        let period: usize = 5;
        bulk::money_flow_index(&prices, &volume, period);
    }

    #[test]
    #[should_panic]
    fn bulk_money_flow_index_period_panic() {
        let prices = vec![
            100.2, 100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28,
        ];
        let volume = vec![1200.0, 1400.0, 1450.0, 1100.0, 900.0, 875.0, 1025.0, 1100.0];
        let period: usize = 50;
        bulk::money_flow_index(&prices, &volume, period);
    }

    #[test]
    fn single_rate_of_change_positive() {
        let current_price = 100.46;
        let previous_price = 100.2;
        assert_eq!(
            0.25948103792414257,
            single::rate_of_change(current_price, previous_price)
        );
    }

    #[test]
    fn single_rate_of_change_negative() {
        let current_price = 100.19;
        let previous_price = 100.38;
        assert_eq!(
            -0.18928073321378536,
            single::rate_of_change(current_price, previous_price)
        );
    }

    #[test]
    fn single_rate_of_change_equal() {
        let current_price = 100.32;
        let previous_price = 100.32;
        assert_eq!(0.0, single::rate_of_change(current_price, previous_price));
    }

    #[test]
    fn bulk_rate_of_change() {
        let prices = vec![100.2, 100.46, 100.38, 100.19, 100.32, 100.32];
        assert_eq!(
            vec![
                0.25948103792414257,
                -0.07963368504877394,
                -0.18928073321378536,
                0.12975346841001642,
                0.0
            ],
            bulk::rate_of_change(&prices)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_rate_of_change_panic() {
        let prices = Vec::new();
        bulk::rate_of_change(&prices);
    }

    #[test]
    fn single_on_balance_volume_positive_no_previous() {
        let current_price = 100.46;
        let previous_price = 100.2;
        let volume = 1500.0;
        let previous_obv = 0.0;
        assert_eq!(
            1500.0,
            single::on_balance_volume(current_price, previous_price, volume, previous_obv)
        );
    }

    #[test]
    fn single_on_balance_volume_positive_previous() {
        let current_price = 100.46;
        let previous_price = 100.2;
        let volume = 1500.0;
        let previous_obv = 500.0;
        assert_eq!(
            2000.0,
            single::on_balance_volume(current_price, previous_price, volume, previous_obv)
        );
    }

    #[test]
    fn single_on_balance_volume_negative_no_previous() {
        let current_price = 100.19;
        let previous_price = 100.32;
        let volume = 1500.0;
        let previous_obv = 0.0;
        assert_eq!(
            -1500.0,
            single::on_balance_volume(current_price, previous_price, volume, previous_obv)
        );
    }

    #[test]
    fn single_on_balance_volume_negative_previous() {
        let current_price = 100.19;
        let previous_price = 100.38;
        let volume = 1500.0;
        let previous_obv = 500.0;
        assert_eq!(
            -1000.0,
            single::on_balance_volume(current_price, previous_price, volume, previous_obv)
        );
    }

    #[test]
    fn single_on_balance_volume_equal_no_previous() {
        let current_price = 100.32;
        let previous_price = 100.32;
        let volume = 1500.0;
        let previous_obv = 0.0;
        assert_eq!(
            0.0,
            single::on_balance_volume(current_price, previous_price, volume, previous_obv)
        );
    }

    #[test]
    fn single_on_balance_volume_equal_previous() {
        let current_price = 100.32;
        let previous_price = 100.32;
        let volume = 1500.0;
        let previous_obv = 500.0;
        assert_eq!(
            500.0,
            single::on_balance_volume(current_price, previous_price, volume, previous_obv)
        );
    }

    #[test]
    fn bulk_on_balance_volume_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let volume = vec![1400.0, 1450.0, 1100.0, 900.0, 875.0];
        assert_eq!(
            vec![1450.0, 350.0, -550.0, 325.0],
            bulk::on_balance_volume(&prices, &volume, 0.0)
        );
    }

    #[test]
    fn bulk_on_balance_volume_previous() {
        let prices = vec![100.53, 100.38, 100.19, 100.21];
        let volume = vec![1450.0, 1100.0, 900.0, 875.0];
        assert_eq!(
            vec![350.0, -550.0, 325.0],
            bulk::on_balance_volume(&prices, &volume, 1450.0)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_on_balance_volume_no_price_panic() {
        let prices = Vec::new();
        let volume = vec![1400.0, 1450.0, 1100.0, 900.0, 875.0];
        bulk::on_balance_volume(&prices, &volume, 0.0);
    }

    #[test]
    #[should_panic]
    fn bulk_on_balance_volume_no_volume_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        let volume = Vec::new();
        bulk::on_balance_volume(&prices, &volume, 0.0);
    }

    #[test]
    #[should_panic]
    fn single_commodity_channel_index_panic() {
        let prices = Vec::new();
        single::commodity_channel_index(
            &prices,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::DeviationModel::MeanAbsoluteDeviation,
            0.015,
        );
    }

    #[test]
    fn single_ma_mean_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -77.92207792208092,
            single::commodity_channel_index(
                &prices,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::MeanAbsoluteDeviation,
                0.015
            )
        );
    }

    #[test]
    fn single_ma_median_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -64.0,
            single::commodity_channel_index(
                &prices,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::MedianAbsoluteDeviation,
                0.015
            )
        );
    }

    #[test]
    fn single_ma_mode_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            0.0,
            single::commodity_channel_index(
                &prices,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::ModeAbsoluteDeviation,
                0.015
            )
        );
    }

    #[test]
    fn single_ma_std_dev_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -71.3483546791537,
            single::commodity_channel_index(
                &prices,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::StandardDeviation,
                0.015
            )
        );
    }

    #[test]
    fn single_ma_ulcer_index_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -44.00422507932252,
            single::commodity_channel_index(
                &prices,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::UlcerIndex,
                0.015
            )
        );
    }
    #[test]
    fn single_sma_mean_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -57.79560753382917,
            single::commodity_channel_index(
                &prices,
                crate::ConstantModelType::SmoothedMovingAverage,
                crate::DeviationModel::MeanAbsoluteDeviation,
                0.015
            )
        );
    }

    #[test]
    fn single_ema_mean_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -42.87971112616663,
            single::commodity_channel_index(
                &prices,
                crate::ConstantModelType::ExponentialMovingAverage,
                crate::DeviationModel::MeanAbsoluteDeviation,
                0.015
            )
        );
    }

    #[test]
    fn single_pma_mean_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -19.132669714320674,
            single::commodity_channel_index(
                &prices,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                },
                crate::DeviationModel::MeanAbsoluteDeviation,
                0.015
            )
        );
    }

    #[test]
    fn single_median_mean_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -91.99134199134296,
            single::commodity_channel_index(
                &prices,
                crate::ConstantModelType::SimpleMovingMedian,
                crate::DeviationModel::MeanAbsoluteDeviation,
                0.015
            )
        );
    }

    #[test]
    fn single_mode_mean_ad_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            113.63636363636031,
            single::commodity_channel_index(
                &prices,
                crate::ConstantModelType::SimpleMovingMode,
                crate::DeviationModel::MeanAbsoluteDeviation,
                0.015
            )
        );
    }

    #[test]
    fn bulk_commodity_channel_index() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            vec![-100.0, -100.00000000000804, -41.66666666666519],
            bulk::commodity_channel_index(
                &prices,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::MeanAbsoluteDeviation,
                0.015,
                3_usize
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_commodity_channel_index_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            vec![-100.0, -100.00000000000804, -41.66666666666519],
            bulk::commodity_channel_index(
                &prices,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::DeviationModel::MeanAbsoluteDeviation,
                0.015,
                30_usize
            )
        );
    }

    #[test]
    fn single_mcginley_cci_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (0.0, 100.21),
            single::mcginley_dynamic_commodity_channel_index(
                &prices,
                0.0,
                crate::DeviationModel::MeanAbsoluteDeviation,
                0.015
            )
        );
    }

    #[test]
    fn single_mcginley_cci_previous_mean_absolute_deviation() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!(
            (56.90977560811997, 100.23190366735862),
            single::mcginley_dynamic_commodity_channel_index(
                &prices,
                100.21,
                crate::DeviationModel::MeanAbsoluteDeviation,
                0.015
            )
        );
    }

    #[test]
    fn single_mcginley_cci_previous_median_absolute_deviation() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!(
            (53.39171675234611, 100.23190366735862),
            single::mcginley_dynamic_commodity_channel_index(
                &prices,
                100.21,
                crate::DeviationModel::MedianAbsoluteDeviation,
                0.015
            )
        );
    }

    #[test]
    fn single_mcginley_cci_previous_mode_absolute_deviation() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!(
            (0.0, 100.23190366735862),
            single::mcginley_dynamic_commodity_channel_index(
                &prices,
                100.21,
                crate::DeviationModel::ModeAbsoluteDeviation,
                0.015
            )
        );
    }

    #[test]
    fn single_mcginley_cci_previous_standard_deviation() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!(
            (47.47490364820863, 100.23190366735862),
            single::mcginley_dynamic_commodity_channel_index(
                &prices,
                100.21,
                crate::DeviationModel::StandardDeviation,
                0.015
            )
        );
    }

    #[test]
    fn single_mcginley_cci_previous_ulcer_index() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!(
            (24.747413068246022, 100.23190366735862),
            single::mcginley_dynamic_commodity_channel_index(
                &prices,
                100.21,
                crate::DeviationModel::UlcerIndex,
                0.015
            )
        );
    }

    #[test]
    #[should_panic]
    fn single_mcginley_cci_panic() {
        let prices = Vec::new();
        single::mcginley_dynamic_commodity_channel_index(
            &prices,
            100.21,
            crate::DeviationModel::MedianAbsoluteDeviation,
            0.015,
        );
    }

    #[test]
    fn bulk_mcginley_cci_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                (0.0, 100.21),
                (56.90977560811997, 100.23190366735862),
                (42.20998599356397, 100.24150449277387)
            ],
            bulk::mcginley_dynamic_commodity_channel_index(
                &prices,
                0.0,
                crate::DeviationModel::MeanAbsoluteDeviation,
                0.015,
                5_usize
            )
        );
    }

    #[test]
    fn bulk_mcginley_cci_previous() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                (56.90977560811997, 100.23190366735862),
                (42.20998599356397, 100.24150449277387)
            ],
            bulk::mcginley_dynamic_commodity_channel_index(
                &prices,
                100.21,
                crate::DeviationModel::MeanAbsoluteDeviation,
                0.015,
                5_usize
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_mcginley_cci_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::mcginley_dynamic_commodity_channel_index(
            &prices,
            0.0,
            crate::DeviationModel::MeanAbsoluteDeviation,
            0.015,
            50_usize,
        );
    }

    #[test]
    fn single_ema_macd() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -0.06067027758972188,
            single::macd_line(
                &prices,
                3_usize,
                crate::ConstantModelType::ExponentialMovingAverage,
                crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn single_sma_macd() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -0.07733259851198682,
            single::macd_line(
                &prices,
                3_usize,
                crate::ConstantModelType::SmoothedMovingAverage,
                crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn single_pma_macd() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -0.02938702437832319,
            single::macd_line(
                &prices,
                3_usize,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                },
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                }
            )
        );
    }

    #[test]
    fn single_ma_macd() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -0.0940000000000083,
            single::macd_line(
                &prices,
                3_usize,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn single_median_macd() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            -0.1700000000000017,
            single::macd_line(
                &prices,
                3_usize,
                crate::ConstantModelType::SimpleMovingMedian,
                crate::ConstantModelType::SimpleMovingMedian
            )
        );
    }

    #[test]
    fn single_mode_macd() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            0.0,
            single::macd_line(
                &prices,
                3_usize,
                crate::ConstantModelType::SimpleMovingMode,
                crate::ConstantModelType::SimpleMovingMode
            )
        );
    }

    #[test]
    #[should_panic]
    fn single_macd_panic() {
        let prices = Vec::new();
        single::macd_line(
            &prices,
            3_usize,
            crate::ConstantModelType::ExponentialMovingAverage,
            crate::ConstantModelType::ExponentialMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn single_macd_panic_period() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        single::macd_line(
            &prices,
            30_usize,
            crate::ConstantModelType::ExponentialMovingAverage,
            crate::ConstantModelType::ExponentialMovingAverage,
        );
    }

    #[test]
    fn bulk_macd() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                -0.06067027758972188,
                -0.022417061611406552,
                0.005788761002008869
            ],
            bulk::macd_line(
                &prices,
                3_usize,
                crate::ConstantModelType::ExponentialMovingAverage,
                5_usize,
                crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_macd_short_period_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::macd_line(
            &prices,
            30_usize,
            crate::ConstantModelType::ExponentialMovingAverage,
            5_usize,
            crate::ConstantModelType::ExponentialMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_macd_long_period_panic() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::macd_line(
            &prices,
            3_usize,
            crate::ConstantModelType::ExponentialMovingAverage,
            50_usize,
            crate::ConstantModelType::ExponentialMovingAverage,
        );
    }

    #[test]
    fn single_ema_signal() {
        let macds = vec![
            -0.06067027758972188,
            -0.022417061611406552,
            0.005788761002008869,
        ];
        assert_eq!(
            -0.011764193829214216,
            single::signal_line(&macds, crate::ConstantModelType::ExponentialMovingAverage)
        );
    }

    #[test]
    fn single_sma_signal() {
        let macds = vec![
            -0.06067027758972188,
            -0.022417061611406552,
            0.005788761002008869,
        ];
        assert_eq!(
            -0.01710971742153932,
            single::signal_line(&macds, crate::ConstantModelType::SmoothedMovingAverage)
        );
    }

    #[test]
    fn single_pma_signal() {
        let macds = vec![
            -0.06067027758972188,
            -0.022417061611406552,
            0.005788761002008869,
        ];
        assert_eq!(
            -0.004072696773434995,
            single::signal_line(
                &macds,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                }
            )
        );
    }

    #[test]
    fn single_ma_signal() {
        let macds = vec![
            -0.06067027758972188,
            -0.022417061611406552,
            0.005788761002008869,
        ];
        assert_eq!(
            -0.025766192733039855,
            single::signal_line(&macds, crate::ConstantModelType::SimpleMovingAverage)
        );
    }

    #[test]
    fn single_median_signal() {
        let macds = vec![
            -0.06067027758972188,
            -0.022417061611406552,
            0.005788761002008869,
        ];
        assert_eq!(
            -0.022417061611406552,
            single::signal_line(&macds, crate::ConstantModelType::SimpleMovingMedian)
        );
    }

    #[test]
    fn single_mode_signal() {
        let macds = vec![
            -0.06067027758972188,
            -0.022417061611406552,
            0.005788761002008869,
        ];
        assert_eq!(
            0.0,
            single::signal_line(&macds, crate::ConstantModelType::SimpleMovingMode)
        );
    }

    #[test]
    #[should_panic]
    fn single_signal_panic() {
        let macds = Vec::new();
        single::signal_line(&macds, crate::ConstantModelType::ExponentialMovingAverage);
    }

    #[test]
    #[should_panic]
    fn bulk_signal_panic() {
        let macds = vec![
            -0.06067027758972188,
            -0.022417061611406552,
            0.005788761002008869,
        ];
        bulk::signal_line(
            &macds,
            crate::ConstantModelType::ExponentialMovingAverage,
            30_usize,
        );
    }

    #[test]
    fn single_mcginley_macd_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        assert_eq!(
            (0.0, 100.21, 100.21),
            single::mcginley_dynamic_macd_line(&prices, 3_usize, 0.0, 0.0)
        );
    }

    #[test]
    fn single_mcginley_macd_previous() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32];
        assert_eq!(
            (0.014602444905747802, 100.24650611226437, 100.23190366735862),
            single::mcginley_dynamic_macd_line(&prices, 3_usize, 100.21, 100.21)
        );
    }

    #[test]
    #[should_panic]
    fn single_mcginley_macd_panic_short_period() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21];
        single::mcginley_dynamic_macd_line(&prices, 30_usize, 0.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn single_mcginley_macd_panic_no_prices() {
        let prices = Vec::new();
        single::mcginley_dynamic_macd_line(&prices, 3_usize, 0.0, 0.0);
    }

    #[test]
    fn bulk_mcginley_macd_no_previous() {
        let prices = vec![100.46, 100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                (0.0, 100.21, 100.21),
                (0.014602444905747802, 100.24650611226437, 100.23190366735862),
                (0.01615134009865926, 100.25765583287253, 100.24150449277387)
            ],
            bulk::mcginley_dynamic_macd_line(&prices, 3_usize, 0.0, 5_usize, 0.0)
        );
    }

    #[test]
    fn bulk_mcginley_macd_previous() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        assert_eq!(
            vec![
                (0.014602444905747802, 100.24650611226437, 100.23190366735862),
                (0.01615134009865926, 100.25765583287253, 100.24150449277387)
            ],
            bulk::mcginley_dynamic_macd_line(&prices, 3_usize, 100.21, 5_usize, 100.21)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_mcginley_macd_panic_long_period() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::mcginley_dynamic_macd_line(&prices, 3_usize, 100.21, 50_usize, 100.21);
    }

    #[test]
    #[should_panic]
    fn bulk_mcginley_macd_panic_short_period() {
        let prices = vec![100.53, 100.38, 100.19, 100.21, 100.32, 100.28];
        bulk::mcginley_dynamic_macd_line(&prices, 30_usize, 100.21, 5_usize, 100.21);
    }

    #[test]
    #[should_panic]
    fn bulk_mcginley_macd_panic_no_prices() {
        let prices = Vec::new();
        bulk::mcginley_dynamic_macd_line(&prices, 3_usize, 100.21, 5_usize, 100.21);
    }

    #[test]
    fn single_chaikin_oscillator_ma() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0];
        assert_eq!(
            (29.535626025665053, 304.76047677253655),
            single::chaikin_oscillator(
                &highs,
                &lows,
                &close,
                &volume,
                3_usize,
                0.0,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn single_chaikin_oscillator_sma() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0];
        assert_eq!(
            (52.583143395495696, 304.76047677253655),
            single::chaikin_oscillator(
                &highs,
                &lows,
                &close,
                &volume,
                3_usize,
                0.0,
                crate::ConstantModelType::SmoothedMovingAverage,
                crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn single_chaikin_oscillator_ema() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0];
        assert_eq!(
            (58.83861961460434, 304.76047677253655),
            single::chaikin_oscillator(
                &highs,
                &lows,
                &close,
                &volume,
                3_usize,
                0.0,
                crate::ConstantModelType::ExponentialMovingAverage,
                crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn single_chaikin_oscillator_pma() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0];
        assert_eq!(
            (51.277072031524625, 304.76047677253655),
            single::chaikin_oscillator(
                &highs,
                &lows,
                &close,
                &volume,
                3_usize,
                0.0,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                },
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                }
            )
        );
    }

    #[test]
    fn single_chaikin_oscillator_median() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0];
        assert_eq!(
            (21.535323383069596, 304.76047677253655),
            single::chaikin_oscillator(
                &highs,
                &lows,
                &close,
                &volume,
                3_usize,
                0.0,
                crate::ConstantModelType::SimpleMovingMedian,
                crate::ConstantModelType::SimpleMovingMedian
            )
        );
    }

    #[test]
    fn single_chaikin_oscillator_mode() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0];
        assert_eq!(
            (29.53333333333333, 304.76047677253655),
            single::chaikin_oscillator(
                &highs,
                &lows,
                &close,
                &volume,
                3_usize,
                0.0,
                crate::ConstantModelType::SimpleMovingMode,
                crate::ConstantModelType::SimpleMovingMode
            )
        );
    }

    #[test]
    fn single_chaikin_oscillator_ma_previous() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0];
        assert_eq!(
            (29.53562602566504, 398.52047677253654),
            single::chaikin_oscillator(
                &highs,
                &lows,
                &close,
                &volume,
                3_usize,
                93.76,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn single_chaikin_oscillator_different_models() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0];
        assert_eq!(
            (22.17005097965847, 304.76047677253655),
            single::chaikin_oscillator(
                &highs,
                &lows,
                &close,
                &volume,
                3_usize,
                0.0,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::ConstantModelType::SimpleMovingMedian
            )
        );
    }

    #[test]
    #[should_panic]
    fn single_chaikin_oscillator_short_period_panic() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0];
        single::chaikin_oscillator(
            &highs,
            &lows,
            &close,
            &volume,
            30_usize,
            0.0,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::ConstantModelType::SimpleMovingMedian,
        );
    }

    #[test]
    #[should_panic]
    fn single_chaikin_oscillator_highs_panic() {
        let highs = vec![100.53, 100.68, 100.79, 100.88];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0];
        single::chaikin_oscillator(
            &highs,
            &lows,
            &close,
            &volume,
            3_usize,
            0.0,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::ConstantModelType::SimpleMovingMedian,
        );
    }

    #[test]
    #[should_panic]
    fn single_chaikin_oscillator_lows_panic() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88];
        let lows = vec![99.97, 100.28, 100.12, 100.07];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0];
        single::chaikin_oscillator(
            &highs,
            &lows,
            &close,
            &volume,
            3_usize,
            0.0,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::ConstantModelType::SimpleMovingMedian,
        );
    }

    #[test]
    #[should_panic]
    fn single_chaikin_oscillator_close_panic() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07];
        let close = vec![100.01, 100.44, 100.39, 100.71];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0];
        single::chaikin_oscillator(
            &highs,
            &lows,
            &close,
            &volume,
            3_usize,
            0.0,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::ConstantModelType::SimpleMovingMedian,
        );
    }

    #[test]
    #[should_panic]
    fn single_chaikin_oscillator_volume_panic() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        let volume = vec![268.0, 319.0, 381.0, 414.0];
        single::chaikin_oscillator(
            &highs,
            &lows,
            &close,
            &volume,
            3_usize,
            0.0,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::ConstantModelType::SimpleMovingMedian,
        );
    }

    #[test]
    fn bulk_chaikin_oscillator_no_previous() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88, 100.51, 100.17];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07, 99.86, 99.60];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71, 100.35, 100.12];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0, 396.0, 362.0];
        assert_eq!(
            vec![
                (22.17005097965847, 304.76047677253655),
                (212.4639442861619, 848.8528216769286),
                (233.52784525415495, 1588.098366482492)
            ],
            bulk::chaikin_oscillator(
                &highs,
                &lows,
                &close,
                &volume,
                3_usize,
                5_usize,
                0.0,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::ConstantModelType::SimpleMovingMedian
            )
        );
    }

    #[test]
    fn bulk_chaikin_oscillator_previous() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88, 100.51, 100.17];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07, 99.86, 99.60];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71, 100.35, 100.12];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0, 396.0, 362.0];
        assert_eq!(
            vec![
                (22.17005097965844, 404.76047677253655),
                (212.4639442861619, 948.8528216769286),
                (233.52784525415473, 1688.098366482492)
            ],
            bulk::chaikin_oscillator(
                &highs,
                &lows,
                &close,
                &volume,
                3_usize,
                5_usize,
                100.0,
                crate::ConstantModelType::SimpleMovingAverage,
                crate::ConstantModelType::SimpleMovingMedian
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_chaikin_oscillator_long_period_panic() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88, 100.51, 100.17];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07, 99.86, 99.60];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71, 100.35, 100.12];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0, 396.0, 362.0];
        bulk::chaikin_oscillator(
            &highs,
            &lows,
            &close,
            &volume,
            3_usize,
            50_usize,
            0.0,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::ConstantModelType::SimpleMovingMedian,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_chaikin_oscillator_period_panic() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88, 100.51, 100.17];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07, 99.86, 99.60];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71, 100.35, 100.12];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0, 396.0, 362.0];
        bulk::chaikin_oscillator(
            &highs,
            &lows,
            &close,
            &volume,
            30_usize,
            5_usize,
            0.0,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::ConstantModelType::SimpleMovingMedian,
        );
    }
    #[test]
    #[should_panic]
    fn bulk_chaikin_oscillator_high_panic() {
        let highs = vec![100.53, 100.68, 100.79, 100.88, 100.51, 100.17];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07, 99.86, 99.60];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71, 100.35, 100.12];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0, 396.0, 362.0];
        bulk::chaikin_oscillator(
            &highs,
            &lows,
            &close,
            &volume,
            3_usize,
            5_usize,
            0.0,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::ConstantModelType::SimpleMovingMedian,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_chaikin_oscillator_low_panic() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88, 100.51, 100.17];
        let lows = vec![99.62, 99.97, 100.12, 100.07, 99.86, 99.60];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71, 100.35, 100.12];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0, 396.0, 362.0];
        bulk::chaikin_oscillator(
            &highs,
            &lows,
            &close,
            &volume,
            3_usize,
            5_usize,
            0.0,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::ConstantModelType::SimpleMovingMedian,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_chaikin_oscillator_close_panic() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88, 100.51, 100.17];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07, 99.86, 99.60];
        let close = vec![100.01, 100.44, 100.63, 100.71, 100.35, 100.12];
        let volume = vec![268.0, 319.0, 381.0, 414.0, 376.0, 396.0, 362.0];
        bulk::chaikin_oscillator(
            &highs,
            &lows,
            &close,
            &volume,
            3_usize,
            5_usize,
            0.0,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::ConstantModelType::SimpleMovingMedian,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_chaikin_oscillator_volume_panic() {
        let highs = vec![100.53, 100.68, 100.73, 100.79, 100.88, 100.51, 100.17];
        let lows = vec![99.62, 99.97, 100.28, 100.12, 100.07, 99.86, 99.60];
        let close = vec![100.01, 100.44, 100.39, 100.63, 100.71, 100.35, 100.12];
        let volume = vec![268.0, 319.0, 381.0, 376.0, 396.0, 362.0];
        bulk::chaikin_oscillator(
            &highs,
            &lows,
            &close,
            &volume,
            3_usize,
            5_usize,
            0.0,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::ConstantModelType::SimpleMovingMedian,
        );
    }

    #[test]
    fn single_percentge_price_oscillator_ma() {
        let prices = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        assert_eq!(
            0.1400560224089579,
            single::percentage_price_oscillator(
                &prices,
                3_usize,
                crate::ConstantModelType::SimpleMovingAverage
            )
        );
    }

    #[test]
    fn single_percentge_price_oscillator_sma() {
        let prices = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        assert_eq!(
            0.1131763552115475,
            single::percentage_price_oscillator(
                &prices,
                3_usize,
                crate::ConstantModelType::SmoothedMovingAverage
            )
        );
    }

    #[test]
    fn single_percentge_price_oscillator_ema() {
        let prices = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        assert_eq!(
            0.08979623002617415,
            single::percentage_price_oscillator(
                &prices,
                3_usize,
                crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    fn single_percentge_price_oscillator_pma() {
        let prices = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        assert_eq!(
            0.0485562141788203,
            single::percentage_price_oscillator(
                &prices,
                3_usize,
                crate::ConstantModelType::PersonalisedMovingAverage {
                    alpha_num: 5.0,
                    alpha_den: 4.0
                }
            )
        );
    }

    #[test]
    fn single_percentge_price_oscillator_median() {
        let prices = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        assert_eq!(
            0.1891676622859396,
            single::percentage_price_oscillator(
                &prices,
                3_usize,
                crate::ConstantModelType::SimpleMovingMedian
            )
        );
    }

    #[test]
    fn single_percentge_price_oscillator_mode() {
        let prices = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        assert_eq!(
            1.0,
            single::percentage_price_oscillator(
                &prices,
                3_usize,
                crate::ConstantModelType::SimpleMovingMode
            )
        );
    }

    #[test]
    #[should_panic]
    fn single_percentage_price_oscillator_panic_period() {
        let prices = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        single::percentage_price_oscillator(
            &prices,
            30_usize,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    #[should_panic]
    fn single_percentage_price_oscillator_panic_empty() {
        let prices = Vec::new();
        single::percentage_price_oscillator(
            &prices,
            0_usize,
            crate::ConstantModelType::SimpleMovingMode,
        );
    }

    #[test]
    fn bulk_percentage_price_oscillator() {
        let prices = vec![100.01, 100.44, 100.39, 100.63, 100.71, 100.35, 100.12];
        assert_eq!(
            vec![
                0.08979623002617415,
                -0.008380468415664317,
                -0.08769552039759204
            ],
            bulk::percentage_price_oscillator(
                &prices,
                3_usize,
                5_usize,
                crate::ConstantModelType::ExponentialMovingAverage
            )
        );
    }

    #[test]
    #[should_panic]
    fn bulk_percentage_price_oscillator_panic_short_period() {
        let prices = vec![100.01, 100.44, 100.39, 100.63, 100.71, 100.35, 100.12];
        bulk::percentage_price_oscillator(
            &prices,
            30_usize,
            5_usize,
            crate::ConstantModelType::ExponentialMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_percentage_price_oscillator_panic_long_period() {
        let prices = vec![100.01, 100.44, 100.39, 100.63, 100.71, 100.35, 100.12];
        bulk::percentage_price_oscillator(
            &prices,
            3_usize,
            50_usize,
            crate::ConstantModelType::ExponentialMovingAverage,
        );
    }

    #[test]
    #[should_panic]
    fn bulk_percentage_price_oscillator_panic_empty() {
        let prices = Vec::new();
        bulk::percentage_price_oscillator(
            &prices,
            3_usize,
            5_usize,
            crate::ConstantModelType::ExponentialMovingAverage,
        );
    }

    #[test]
    fn single_chande_momentum_oscillator() {
        let prices = vec![100.01, 100.44, 100.39, 100.63, 100.71];
        assert_eq!(
            87.50000000000044,
            single::chande_momentum_oscillator(&prices)
        );
    }

    #[test]
    fn single_chande_momentum_oscillator_fall() {
        let prices = vec![100.1, 100.0, 99.9, 99.8, 99.7];
        assert_eq!(-100.0, single::chande_momentum_oscillator(&prices));
    }

    #[test]
    fn single_chande_momentum_oscillator_rise() {
        let prices = vec![100.1, 100.2, 100.3, 100.4, 100.5];
        assert_eq!(100.0, single::chande_momentum_oscillator(&prices));
    }

    #[test]
    fn bulk_chande_momentum_oscillator() {
        let prices = vec![100.01, 100.44, 100.39, 100.63, 100.71, 100.35, 100.12];
        assert_eq!(
            vec![87.50000000000044, -12.328767123288312, -29.67032967032981],
            bulk::chande_momentum_oscillator(&prices, 5_usize)
        );
    }

    #[test]
    #[should_panic]
    fn bulk_chande_momentum_oscillator_panic_period() {
        let prices = vec![100.01, 100.44, 100.39, 100.63, 100.71, 100.35, 100.12];
        bulk::chande_momentum_oscillator(&prices, 50_usize);
    }

    #[test]
    #[should_panic]
    fn bulk_chande_momentum_oscillator_panic_empty() {
        let prices = Vec::new();
        bulk::chande_momentum_oscillator(&prices, 5_usize);
    }

    // Tests for new deviation models
    #[test]
    fn commodity_channel_index_log_std() {
        let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 98.0, 100.0];
        let result = single::commodity_channel_index(
            &prices,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::DeviationModel::LogStandardDeviation,
            0.015,
        );
        assert_eq!(-1805.2605574498584, result);
    }

    #[test]
    fn commodity_channel_index_student_t() {
        let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 98.0, 100.0];
        let result = single::commodity_channel_index(
            &prices,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::DeviationModel::StudentT { df: 5.0 },
            0.015,
        );
        assert_eq!(-13.912166872805114, result);
    }

    #[test]
    fn commodity_channel_index_laplace_std() {
        let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 98.0, 100.0];
        let result = single::commodity_channel_index(
            &prices,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::DeviationModel::LaplaceStdEquivalent,
            0.015,
        );
        assert_eq!(-20.20305089104431, result);
    }

    #[test]
    fn commodity_channel_index_cauchy_iqr() {
        let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 98.0, 100.0];
        let result = single::commodity_channel_index(
            &prices,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::DeviationModel::CauchyIQRScale,
            0.015,
        );
        assert_eq!(-19.04761904761914, result);
    }

    #[test]
    fn mcginley_dynamic_cci_log_std() {
        let prices = vec![100.0, 102.0, 103.0, 101.0, 99.0, 98.0, 100.0];
        let result = single::mcginley_dynamic_commodity_channel_index(
            &prices,
            100.5,
            crate::DeviationModel::LogStandardDeviation,
            0.015,
        );
        assert_eq!((-1799.197740117997, 100.42713210709822), result);
    }

    #[test]
    fn single_sma_cci_with_empirical_iqr() {
        // prices [1,2,3,4], SMA = 2.5, last=4 => numerator=1.5
        // IQR (0.25,0.75) with precision=1.0 => 1.5 (from basic test)
        // CCI = 1.5 / (0.015 * 1.5) = 66.666...
        let prices = vec![1.0, 2.0, 3.0, 4.0];
        let cci = single::commodity_channel_index(
            &prices,
            crate::ConstantModelType::SimpleMovingAverage,
            crate::DeviationModel::EmpiricalQuantileRange {
                low: 0.25,
                high: 0.75,
                precision: 1.0,
            },
            0.015,
        );
        assert_eq!(50.0, cci);
    }
}
