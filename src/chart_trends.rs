//! # Chart Trends
//!
//! The `chart_trends` module provides utilities for detecting, analyzing, and breaking downtrends in price charts.
//! These functions help identify overall direction, peaks, valleys, and trend segments in a time series.
//!
//! ## When to Use
//! Use chart trend indicators when you want to:
//! - Decompose a price series into upward/downward trends
//! - Find peaks and valleys for support/resistance analysis
//! - Quantify the overall or local trend direction of an asset
//!
//! ## Structure
//! Unlike other modules, `chart_trends` does not have `single` or `bulk` submodules.
//! All functions operate over slices and return either trend breakdowns or locations of key points.
//!
//! ## Included Functions
//! - [`break_down_trends`]: Segments the chart into distinct up/down trends
//! - [`overall_trend`]: Returns the overall trend (slope) for all price points
//! - [`peak_trend`]: Calculates the trend based on local peaks
//! - [`peaks`]: Finds all local maxima (peaks) in the series
//! - [`valley_trend`]: Calculates the trend based on local valleys
//! - [`valleys`]: Finds all local minima (valleys) in the series
//!
//! ## API Details
//! - All functions work on slices of `f64` prices (or equivalent).
//! - Returns are typically vectors of trend segments or indices/values of peaks/valleys.
//! - See each function's documentation for examples, panics, and usage tips.
//!
//! ---

use crate::basic_indicators::single::{max, mean, min};
use crate::validation::{assert_period, assert_same_len};

/// Calculates all peaks over a given period
///
/// # Arguments
///
/// * `prices` - Slice of prices
/// * `period` - Period over which to find the peak
/// * `closest_neighbor` - Minimum distance between peaks
///
/// # Returns
///
/// A vector of tuples, each containing (peak_value, peak_index)
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
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0];
/// let period: usize = 3;
/// let closest_neighbor: usize = 1;
/// let peaks = centaur_technical_indicators::chart_trends::peaks(&highs, period, closest_neighbor);
/// assert_eq!(vec![(107.0, 2)], peaks);
///
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 109.0];
/// let period: usize = 3;
/// let peaks = centaur_technical_indicators::chart_trends::peaks(&highs, period, closest_neighbor);
/// assert_eq!(vec![(107.0, 2), (109.0, 5)], peaks);
///
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 109.0];
/// let period: usize = 6;
/// let peaks = centaur_technical_indicators::chart_trends::peaks(&highs, period, closest_neighbor);
/// assert_eq!(vec![(109.0, 5)], peaks);
///
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 107.0];
/// let period: usize = 3;
/// let peaks = centaur_technical_indicators::chart_trends::peaks(&highs, period, closest_neighbor);
/// assert_eq!(vec![(107.0, 2), (107.0, 5)], peaks);
///
/// // If there are 2 peaks it will take the most recent one
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 107.0];
/// let period: usize = 6;
/// let peaks = centaur_technical_indicators::chart_trends::peaks(&highs, period, closest_neighbor);
/// assert_eq!(vec![(107.0, 5)], peaks);
/// ```
pub fn peaks(prices: &[f64], period: usize, closest_neighbor: usize) -> Vec<(f64, usize)> {
    let length = prices.len();
    assert_period(period, length);

    let mut peaks: Vec<(f64, usize)> = Vec::new();
    let mut last_peak_idx: usize = 0;
    let mut last_peak: f64 = 0.0;

    for i in 0..=length - period {
        let window = &prices[i..i + period];
        let peak = max(window);
        let local_idx = window.iter().rposition(|&x| x == peak).unwrap();
        let idx = i + local_idx;

        if last_peak_idx != 0 {
            if idx <= last_peak_idx + closest_neighbor {
                if peak < last_peak {
                    last_peak_idx = idx;
                } else if peak > last_peak {
                    peaks.pop();
                    peaks.push((peak, idx));
                    last_peak_idx = idx;
                    last_peak = peak;
                }
            } else if !peaks.contains(&(peak, idx)) {
                peaks.push((peak, idx));
                last_peak_idx = idx;
                last_peak = peak;
            }
        } else {
            peaks.push((peak, idx));
            last_peak_idx = idx;
            last_peak = peak;
        }
    }
    peaks
}

/// Calculates all valleys for a given period.
///
/// # Arguments
///
/// * `prices` - Slice of prices
/// * `period` - Period over which to find the valley
/// * `closest_neighbor` - Minimum distance between valleys
///
/// # Returns
///
/// A vector of tuples, each containing (valley_value, valley_index)
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
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0];
/// let period: usize = 3;
/// let closest_neighbor: usize = 1;
/// let valleys = centaur_technical_indicators::chart_trends::valleys(&lows, period, closest_neighbor);
/// assert_eq!(vec![(95.0, 2)], valleys);
///
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0, 93.0];
/// let period: usize = 3;
/// let valleys = centaur_technical_indicators::chart_trends::valleys(&lows, period, closest_neighbor);
/// assert_eq!(vec![(95.0, 2), (93.0, 5)], valleys);
///
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0, 93.0];
/// let period: usize = 6;
/// let valleys = centaur_technical_indicators::chart_trends::valleys(&lows, period, closest_neighbor);
/// assert_eq!(vec![(93.0, 5)], valleys);
///
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0, 95.0];
/// let period: usize = 3;
/// let valleys = centaur_technical_indicators::chart_trends::valleys(&lows, period, closest_neighbor);
/// assert_eq!(vec![(95.0, 2), (95.0, 5)], valleys);
///
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0, 95.0];
/// let period: usize = 6;
/// let valleys = centaur_technical_indicators::chart_trends::valleys(&lows, period, closest_neighbor);
/// assert_eq!(vec![(95.0, 5)], valleys);
/// ```
pub fn valleys(prices: &[f64], period: usize, closest_neighbor: usize) -> Vec<(f64, usize)> {
    let length = prices.len();
    assert_period(period, length);

    let mut valleys: Vec<(f64, usize)> = Vec::new();
    let mut last_valley_idx: usize = 0;
    let mut last_valley: f64 = 0.0;

    for i in 0..=length - period {
        let window = &prices[i..i + period];
        let valley = min(window);
        let local_idx = window.iter().rposition(|&x| x == valley).unwrap();
        let idx = i + local_idx;

        if last_valley_idx != 0 {
            if idx <= last_valley_idx + closest_neighbor {
                if valley > last_valley {
                    last_valley_idx = idx;
                } else if valley < last_valley {
                    valleys.pop();
                    valleys.push((valley, idx));
                    last_valley_idx = idx;
                    last_valley = valley;
                }
            } else if !valleys.contains(&(valley, idx)) {
                valleys.push((valley, idx));
                last_valley_idx = idx;
                last_valley = valley;
            }
        } else {
            valleys.push((valley, idx));
            last_valley_idx = idx;
            last_valley = valley;
        }
    }
    valleys
}

/// OLS simple linear regression function
fn get_trend_line(p: &[(f64, usize)]) -> (f64, f64) {
    let length = p.len() as f64;
    let mean_x = p.iter().map(|&(_, x)| x as f64).sum::<f64>() / length;
    let mean_y = p.iter().map(|&(y, _)| y).sum::<f64>() / length;

    let (num, den) = p.iter().fold((0.0, 0.0), |(num, den), &(y, x)| {
        let x = x as f64;
        let dx = x - mean_x;
        (num + dx * (y - mean_y), den + dx * dx)
    });

    let slope = num / den;
    let intercept = mean_y - (slope * mean_x);
    (slope, intercept)
}

/// Returns the slope and intercept of the trend line fitted to peaks.
///
/// # Arguments
///
/// * `prices` - Slice of prices
/// * `period` - Period over which to calculate the peaks
///
/// # Returns
///
/// A tuple containing (slope, intercept) of the trend line
///
/// # Examples
///
/// ```rust
/// let highs = vec![103.0, 102.0, 107.0, 104.0, 100.0, 109.0];
/// let period: usize = 3;
/// let peak_trend = centaur_technical_indicators::chart_trends::peak_trend(&highs, period);
/// assert_eq!((0.6666666666666666, 105.66666666666667), peak_trend);
/// ```
#[inline]
pub fn peak_trend(prices: &[f64], period: usize) -> (f64, f64) {
    let peaks = peaks(prices, period, 1);
    get_trend_line(&peaks)
}

/// Calculates the slope and intercept of the trend line fitted to valleys.
///
/// # Arguments
///
/// * `prices` - Slice of prices
/// * `period` - Period over which to calculate the valleys
///
/// # Returns
///
/// A tuple containing (slope, intercept) of the trend line
///
/// # Examples
///
/// ```rust
/// let lows = vec![98.0, 101.0, 95.0, 100.0, 97.0, 93.0];
/// let period: usize = 3;
/// let valley_trend = centaur_technical_indicators::chart_trends::valley_trend(&lows, period);
/// assert_eq!((-0.6666666666666666, 96.33333333333333), valley_trend);
/// ```
#[inline]
pub fn valley_trend(prices: &[f64], period: usize) -> (f64, f64) {
    let valleys = valleys(prices, period, 1);
    get_trend_line(&valleys)
}

/// Calculates the slope and intercept of the trend line fitted to all prices.
///
/// # Arguments
///
/// * `prices` - Slice of prices
///
/// # Returns
///
/// A tuple containing (slope, intercept) of the trend line
///
/// # Examples
///
/// ```rust
/// let prices = vec![100.0, 102.0, 103.0, 101.0, 100.0];
/// let overall_trend = centaur_technical_indicators::chart_trends::overall_trend(&prices);
/// assert_eq!((-0.1, 101.4), overall_trend);
/// ```
#[inline]
pub fn overall_trend(prices: &[f64]) -> (f64, f64) {
    let indexed_prices: Vec<(f64, usize)> =
        prices.iter().enumerate().map(|(i, &y)| (y, i)).collect();
    get_trend_line(&indexed_prices)
}

/// Configuration for trend break detection.
///
/// # Fields
/// * `max_outliers` - Consecutive candidate break points allowed to be treated as outliers
///     (skipped) before a segment is forcibly split.
/// * `soft_adj_r_squared_minimum` - Below this adjusted R² (AND with other soft conditions),
///     a *soft* break is considered.
/// * `hard_adj_r_squared_minimum` - Below this adjusted R² alone triggers a *hard* break.
/// * `soft_rmse_multiplier` - Relative RMSE growth factor (vs previous accepted RMSE)
///     required (with other soft factors) to flag a soft break.
/// * `hard_rmse_multiplier` - Larger deterioration factor that alone helps force a hard break.
/// * `soft_durbin_watson_min` / `soft_durbin_watson_max` - Soft residual autocorrelation band.
/// * `hard_durbin_watson_min` / `hard_durbin_watson_max` - Hard residual autocorrelation band.
///     Values far from 2.0 imply structured residuals (model misspecification).
///
/// # Notes
/// - Adjust `max_outliers` to tolerate transient spikes without fragmenting segments.
/// - Durbin–Watson: (0, 4); near 2 = little autocorrelation, < 1 or > 3 => strong correlation.

#[derive(Copy, Clone, Debug)]
pub struct TrendBreakConfig {
    pub max_outliers: usize,
    pub soft_adj_r_squared_minimum: f64,
    pub hard_adj_r_squared_minimum: f64,
    pub soft_rmse_multiplier: f64,
    pub hard_rmse_multiplier: f64,
    pub soft_durbin_watson_min: f64,
    pub soft_durbin_watson_max: f64,
    pub hard_durbin_watson_min: f64,
    pub hard_durbin_watson_max: f64,
}

impl Default for TrendBreakConfig {
    fn default() -> Self {
        Self {
            max_outliers: 1,
            soft_adj_r_squared_minimum: 0.25,
            hard_adj_r_squared_minimum: 0.05,
            soft_rmse_multiplier: 1.3,
            hard_rmse_multiplier: 2.0,
            soft_durbin_watson_min: 1.0,
            soft_durbin_watson_max: 3.0,
            hard_durbin_watson_min: 0.7,
            hard_durbin_watson_max: 3.3,
        }
    }
}

/// Calculates price trends and their slopes and intercepts.
///
/// # Arguments
///
/// * `prices` - Slice of prices
/// * `trend_break_config` - Configuration thresholds (see [`TrendBreakConfig`])
///
/// # Panics
///
/// Panics if `prices.is_empty()`
///
/// # Examples
///
/// ```rust
/// let prices = vec![
///     100.0, 102.0, 103.0, 101.0, 99.0, 99.0, 102.0,
///     103.0, 106.0, 107.0, 105.0, 104.0, 101.0, 97.0, 100.0
/// ];
/// let trend_break_config = centaur_technical_indicators::chart_trends::TrendBreakConfig {
///     max_outliers: 1,
///     soft_adj_r_squared_minimum: 0.25,
///     hard_adj_r_squared_minimum: 0.05,
///     soft_rmse_multiplier: 1.2,
///     hard_rmse_multiplier: 1.8,
///     soft_durbin_watson_min: 1.0,
///     soft_durbin_watson_max: 3.0,
///     hard_durbin_watson_min: 0.5,
///     hard_durbin_watson_max: 3.5,
/// };
///
/// let trend_break_down = centaur_technical_indicators::chart_trends::break_down_trends(
///     &prices,
///     trend_break_config
/// );
///
/// assert_eq!(
///     vec![
///         (0, 2, 1.5, 100.16666666666667),
///         (2, 4, -2.0, 107.0),
///         (4, 9, 1.7714285714285714, 91.15238095238095),
///         (9, 14, -1.4459459459459458, 119.5945945945946)
///     ], trend_break_down);
/// ```
pub fn break_down_trends(
    prices: &[f64],
    trend_break_config: TrendBreakConfig,
) -> Vec<(usize, usize, f64, f64)> {
    if prices.is_empty() {
        panic!("Prices cannot be empty");
    };

    let mut outliers: Vec<usize> = Vec::new();
    let mut trends: Vec<(usize, usize, f64, f64)> = Vec::new();
    let mut current_slope = 0.0;
    let mut current_intercept = 0.0;
    let mut start_index: usize = 0;
    let mut end_index: usize = 1;
    let mut indexed_points: Vec<(f64, usize)> = Vec::new();
    let mut previous_rmse = f64::MAX;

    for (index, &price) in prices.iter().enumerate() {
        indexed_points.push((price, index));

        if index == 0 {
            continue;
        }
        if index > end_index {
            let current_trend = get_trend_line(&indexed_points);
            let (adjusted_r_squared, rmse, durbin_watson) =
                goodness_of_fit(&indexed_points, &current_trend);

            let soft_break = (adjusted_r_squared < trend_break_config.soft_adj_r_squared_minimum)
                && (rmse > trend_break_config.soft_rmse_multiplier * previous_rmse)
                && (durbin_watson < trend_break_config.soft_durbin_watson_min
                    || durbin_watson > trend_break_config.soft_durbin_watson_max); // Autocorrelation detected

            let hard_break = adjusted_r_squared < trend_break_config.hard_adj_r_squared_minimum
                || rmse > trend_break_config.hard_rmse_multiplier * previous_rmse
                || (durbin_watson < trend_break_config.hard_durbin_watson_min
                    || durbin_watson > trend_break_config.hard_durbin_watson_max); // Strong autocorrelation

            if soft_break || hard_break {
                if outliers.len() < trend_break_config.max_outliers {
                    outliers.push(index);
                    indexed_points.pop();
                    continue;
                };
                trends.push((start_index, end_index, current_slope, current_intercept));
                start_index = end_index;
                end_index = index;
                indexed_points = (start_index..=index).map(|x| (prices[x], x)).collect();
                let current_trend = get_trend_line(&indexed_points);
                current_slope = current_trend.0;
                current_intercept = current_trend.1;
                // if list bigger than 2
                if indexed_points.len() > 2 {
                    (_, previous_rmse, _) = goodness_of_fit(&indexed_points, &current_trend);
                } else {
                    previous_rmse = f64::MAX;
                };
                outliers.clear();
            } else {
                previous_rmse = rmse;
                current_slope = current_trend.0;
                current_intercept = current_trend.1;
            }
        }
        end_index = index;
    }
    trends.push((start_index, end_index, current_slope, current_intercept));
    trends
}

/// Computes adjusted R², RMSE, and Durbin–Watson statistic for an OLS fit.
///
/// # Arguments
/// * `indexed_points` - Slice of `(price, index)` pairs
/// * `trend` - `(slope, intercept)` from `get_trend_line`
///
/// # Returns
/// `(adjusted_r_squared, rmse, durbin_watson)`
///
/// # Notes
/// - For `n < 2` returns `(0.0, 0.0, 2.0)` (neutral DW).
/// - Adjusted R² penalizes small samples; negative raw R² values are clamped to 0.0 here.
/// - RMSE is unnormalized; if you need scale invariance, normalize externally.
/// - Durbin–Watson near 2.0 suggests little autocorrelation; < 1 or > 3 signals structural issues.
fn goodness_of_fit(indexed_points: &[(f64, usize)], trend: &(f64, f64)) -> (f64, f64, f64) {
    let n = indexed_points.len();
    if n < 2 {
        return (0.0, 0.0, 2.0); // Bad fit indicators
    }

    let trend_line: Vec<f64> = indexed_points
        .iter()
        .map(|&(_, x)| trend.1 + trend.0 * x as f64)
        .collect();
    let observed_prices: Vec<f64> = indexed_points.iter().map(|&(y, _)| y).collect();

    let observed_mean = mean(&observed_prices);

    let (sum_sq_residuals, total_squares) = (0..n).fold((0.0, 0.0), |(ssr, tss), i| {
        let resid = observed_prices[i] - trend_line[i];
        let total = observed_prices[i] - observed_mean;
        (ssr + resid.powi(2), tss + total.powi(2))
    });

    // Calculate metrics
    let degrees_of_freedom = ((n as f64) - 2.0).max(1.0);

    let r_squared = if total_squares > 1e-10 {
        (1.0 - (sum_sq_residuals / total_squares)).max(0.0)
    } else {
        0.0
    };

    let adjusted_r_squared = if n > 2 {
        1.0 - ((1.0 - r_squared) * ((n - 1) as f64) / degrees_of_freedom)
    } else {
        r_squared
    };

    // Calculate Durbin-Watson for autocorrelation
    let durbin_watson = if n > 1 {
        let dw_num = (1..n).fold(0.0, |acc, i| {
            let diff =
                (observed_prices[i] - trend_line[i]) - (observed_prices[i - 1] - trend_line[i - 1]);
            acc + diff.powi(2)
        });
        if sum_sq_residuals > 1e-10 {
            dw_num / sum_sq_residuals
        } else {
            2.0
        }
    } else {
        2.0
    };

    // RMSE (root mean square error) - more interpretable than standard error
    let rmse = (sum_sq_residuals / n as f64).sqrt();

    (adjusted_r_squared, rmse, durbin_watson)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peaks_single_peak() {
        let highs = vec![101.26, 102.57, 102.32, 100.69];
        assert_eq!(vec![(102.57, 1)], peaks(&highs, 4_usize, 1usize));
    }

    #[test]
    fn peaks_multiple_peaks() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        assert_eq!(
            vec![(102.57, 1), (102.01, 6)],
            peaks(&highs, 4_usize, 1usize)
        );
    }

    #[test]
    fn peaks_multiple_peaks_same_period() {
        let highs = vec![101.26, 102.57, 102.57, 100.69, 100.83, 101.73, 102.01];
        assert_eq!(
            vec![(102.57, 2), (102.01, 6)],
            peaks(&highs, 4_usize, 1usize)
        );
    }

    #[test]
    #[should_panic]
    fn peaks_panic() {
        let highs = vec![101.26, 102.57, 102.57, 100.69, 100.83, 101.73, 102.01];
        peaks(&highs, 40_usize, 1usize);
    }

    #[test]
    fn valleys_single_valley() {
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(vec![(98.75, 1)], valleys(&lows, 7_usize, 1usize));
    }

    #[test]
    fn valleys_multiple_valleys() {
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(
            vec![(98.75, 1), (98.98, 3)],
            valleys(&lows, 4_usize, 1usize)
        );
    }

    #[test]
    fn valleys_multiple_valleys_same_period() {
        let lows = vec![98.75, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!(
            vec![(98.75, 1), (98.98, 3)],
            valleys(&lows, 4_usize, 1usize)
        );
    }

    #[test]
    #[should_panic]
    fn valleys_panic() {
        let lows = vec![98.75, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        valleys(&lows, 40_usize, 1usize);
    }

    #[test]
    fn peaks_trend() {
        let highs = vec![101.26, 102.57, 102.32, 100.69, 100.83, 101.73, 102.01];
        assert_eq!(
            (-0.11199999999999762, 102.68199999999999),
            peak_trend(&highs, 4_usize)
        );
    }

    #[test]
    fn valleys_trend() {
        let lows = vec![100.08, 98.75, 100.14, 98.98, 99.07, 100.1, 99.96];
        assert_eq!((0.11500000000000199, 98.635), valley_trend(&lows, 4_usize));
    }

    #[test]
    fn overall_trends() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        assert_eq!((-0.010000000000000852, 100.372), overall_trend(&prices));
    }

    #[test]
    fn break_down_trends_std_dev() {
        let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];
        let trend_break_config = TrendBreakConfig {
            max_outliers: 1,
            soft_adj_r_squared_minimum: 0.5,
            hard_adj_r_squared_minimum: 0.25,
            soft_rmse_multiplier: 1.2,
            hard_rmse_multiplier: 2.0,
            soft_durbin_watson_min: 1.0,
            soft_durbin_watson_max: 3.0,
            hard_durbin_watson_min: 0.5,
            hard_durbin_watson_max: 3.5,
        };
        let trend_break_down = break_down_trends(&prices, trend_break_config);
        assert_eq!(
            vec![
                (0, 2, 0.16499999999999915, 100.23166666666665),
                (2, 4, -0.1700000000000017, 100.87666666666668)
            ],
            trend_break_down
        );
    }
}
