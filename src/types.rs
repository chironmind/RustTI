/// What central value to use for calculations.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CentralPoint {
    Mean,
    Median,
    Mode,
}

/// How to aggregate a set of absolute deviations.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeviationAggregate {
    Mean,
    Median,
    Mode,
}

/// Configuration that controls how absolute deviations are computed.
/// Example: center = Median, aggregate = Median => true MedianAD (median of |x - median|).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AbsDevConfig {
    pub center: CentralPoint,
    pub aggregate: DeviationAggregate,
}

/// Type of moving average.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MovingAverageType {
    Simple,
    Smoothed,
    Exponential,
    Personalised { alpha_num: f64, alpha_den: f64 },
}

/// Determines which constant model to use for a center point.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ConstantModelType {
    SimpleMovingAverage,
    SmoothedMovingAverage,
    ExponentialMovingAverage,
    PersonalisedMovingAverage { alpha_num: f64, alpha_den: f64 },
    SimpleMovingMedian,
    SimpleMovingMode,
}

/// How to measure deviation from a center point.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DeviationModel {
    StandardDeviation,
    MeanAbsoluteDeviation,
    MedianAbsoluteDeviation,
    ModeAbsoluteDeviation,
    CustomAbsoluteDeviation{config: AbsDevConfig},
    UlcerIndex,
    LogStandardDeviation,
    StudentT { df: f64 },
    LaplaceStdEquivalent,
    CauchyIQRScale,
    EmpiricalQuantileRange { low: f64, high: f64, precision: f64 },
}

/// Trade position.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Position {
    Short,
    Long,
}
