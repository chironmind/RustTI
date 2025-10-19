//! # Distributions
//!
//! The `distributions` module provides probability distribution implementations for statistical analysis.
//! These distributions can be used for modeling returns, calculating probabilities, and risk assessment.
//!
//! ## When to Use
//! Use these distributions when you need to:
//! - Model asset returns or price movements
//! - Calculate probabilities and quantiles
//! - Perform statistical testing and hypothesis validation
//! - Assess risk and tail events
//!
//! ## Available Distributions
//! - [`Normal`]: Normal (Gaussian) distribution
//! - [`Cauchy`]: Cauchy distribution (heavy-tailed, no defined mean/variance)
//! - [`StudentT`]: Student's t-distribution (heavy-tailed with degrees of freedom)
//! - [`Laplace`]: Laplace (double exponential) distribution
//! - [`LogNormal`]: Log-normal distribution (for modeling positive values)
//!
//! ## Trait
//! All distributions implement the [`Distribution`] trait which provides:
//! - `pdf`: Probability density function
//! - `cdf`: Cumulative distribution function (monotonic)
//! - `mean`: Expected value (if defined)
//! - `variance`: Variance (if defined)
//! - `std_dev`: Standard deviation (if defined)
//!
//! ---

use std::f64::consts::{E, PI, SQRT_2};

/// Common trait for probability distributions
///
/// All distributions provide PDF and CDF calculations, along with
/// statistical moments where they are defined.
pub trait Distribution {
    /// Probability density function
    ///
    /// # Arguments
    ///
    /// * `x` - Point at which to evaluate the PDF
    ///
    /// # Returns
    ///
    /// The probability density at `x`
    fn pdf(&self, x: f64) -> f64;

    /// Cumulative distribution function
    ///
    /// The CDF is monotonically non-decreasing and returns values in [0, 1].
    ///
    /// # Arguments
    ///
    /// * `x` - Point at which to evaluate the CDF
    ///
    /// # Returns
    ///
    /// The cumulative probability P(X <= x)
    fn cdf(&self, x: f64) -> f64;

    /// Mean (expected value) of the distribution
    ///
    /// # Returns
    ///
    /// The mean if defined, `f64::NAN` if undefined
    fn mean(&self) -> f64;

    /// Variance of the distribution
    ///
    /// # Returns
    ///
    /// The variance if defined, `f64::NAN` if undefined
    fn variance(&self) -> f64;

    /// Standard deviation of the distribution
    ///
    /// # Returns
    ///
    /// The standard deviation if defined, `f64::NAN` if undefined
    fn std_dev(&self) -> f64 {
        let v = self.variance();
        if v.is_nan() {
            f64::NAN
        } else {
            v.sqrt()
        }
    }
}

/// Normal (Gaussian) distribution
///
/// Parameterized by mean (μ) and standard deviation (σ).
///
/// # Examples
///
/// ```rust
/// use rust_ti::distributions::{Distribution, Normal};
///
/// let normal = Normal::new(0.0, 1.0); // Standard normal
/// assert_eq!(normal.mean(), 0.0);
/// assert_eq!(normal.variance(), 1.0);
///
/// // PDF at mean is highest
/// let pdf_at_mean = normal.pdf(0.0);
/// assert!(pdf_at_mean > normal.pdf(1.0));
///
/// // CDF at mean is 0.5
/// let cdf_at_mean = normal.cdf(0.0);
/// assert!((cdf_at_mean - 0.5).abs() < 1e-6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Normal {
    pub mean: f64,
    pub std_dev: f64,
}

impl Normal {
    /// Create a new Normal distribution
    ///
    /// # Arguments
    ///
    /// * `mean` - Mean (μ) of the distribution
    /// * `std_dev` - Standard deviation (σ) of the distribution
    ///
    /// # Panics
    ///
    /// Panics if `std_dev <= 0.0`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_ti::distributions::Normal;
    ///
    /// let normal = Normal::new(100.0, 15.0);
    /// ```
    pub fn new(mean: f64, std_dev: f64) -> Self {
        if std_dev <= 0.0 {
            panic!("Standard deviation ({}) must be greater than 0.0", std_dev);
        }
        Normal { mean, std_dev }
    }

    /// Standard normal distribution (mean = 0, std_dev = 1)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_ti::distributions::Normal;
    ///
    /// let standard_normal = Normal::standard();
    /// ```
    pub fn standard() -> Self {
        Normal {
            mean: 0.0,
            std_dev: 1.0,
        }
    }
}

impl Distribution for Normal {
    fn pdf(&self, x: f64) -> f64 {
        let coefficient = 1.0 / (self.std_dev * (2.0 * PI).sqrt());
        let exponent = -((x - self.mean).powi(2)) / (2.0 * self.std_dev.powi(2));
        coefficient * E.powf(exponent)
    }

    fn cdf(&self, x: f64) -> f64 {
        // Using error function approximation for standard normal CDF
        let z = (x - self.mean) / (self.std_dev * SQRT_2);
        0.5 * (1.0 + erf(z))
    }

    fn mean(&self) -> f64 {
        self.mean
    }

    fn variance(&self) -> f64 {
        self.std_dev.powi(2)
    }
}

/// Cauchy distribution
///
/// Parameterized by location (x₀) and scale (γ).
/// The Cauchy distribution has heavy tails and undefined mean and variance.
///
/// # Examples
///
/// ```rust
/// use rust_ti::distributions::{Distribution, Cauchy};
///
/// let cauchy = Cauchy::new(0.0, 1.0); // Standard Cauchy
/// assert!(cauchy.mean().is_nan()); // Mean is undefined
/// assert!(cauchy.variance().is_nan()); // Variance is undefined
///
/// // CDF at location is 0.5
/// let cdf_at_location = cauchy.cdf(0.0);
/// assert!((cdf_at_location - 0.5).abs() < 1e-6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cauchy {
    pub location: f64,
    pub scale: f64,
}

impl Cauchy {
    /// Create a new Cauchy distribution
    ///
    /// # Arguments
    ///
    /// * `location` - Location parameter (x₀)
    /// * `scale` - Scale parameter (γ)
    ///
    /// # Panics
    ///
    /// Panics if `scale <= 0.0`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_ti::distributions::Cauchy;
    ///
    /// let cauchy = Cauchy::new(0.0, 1.0);
    /// ```
    pub fn new(location: f64, scale: f64) -> Self {
        if scale <= 0.0 {
            panic!("Scale ({}) must be greater than 0.0", scale);
        }
        Cauchy { location, scale }
    }

    /// Standard Cauchy distribution (location = 0, scale = 1)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_ti::distributions::Cauchy;
    ///
    /// let standard_cauchy = Cauchy::standard();
    /// ```
    pub fn standard() -> Self {
        Cauchy {
            location: 0.0,
            scale: 1.0,
        }
    }
}

impl Distribution for Cauchy {
    fn pdf(&self, x: f64) -> f64 {
        let numerator = 1.0;
        let denominator = PI * self.scale * (1.0 + ((x - self.location) / self.scale).powi(2));
        numerator / denominator
    }

    fn cdf(&self, x: f64) -> f64 {
        0.5 + (1.0 / PI) * ((x - self.location) / self.scale).atan()
    }

    fn mean(&self) -> f64 {
        f64::NAN // Mean is undefined for Cauchy distribution
    }

    fn variance(&self) -> f64 {
        f64::NAN // Variance is undefined for Cauchy distribution
    }
}

/// Student's t-distribution
///
/// Parameterized by degrees of freedom (ν).
/// Heavy-tailed distribution used for small sample sizes.
///
/// # Examples
///
/// ```rust
/// use rust_ti::distributions::{Distribution, StudentT};
///
/// let student_t = StudentT::new(5.0);
/// assert_eq!(student_t.mean(), 0.0);
/// assert!(!student_t.variance().is_nan());
///
/// // CDF at 0 is 0.5 (symmetric around 0)
/// let cdf_at_zero = student_t.cdf(0.0);
/// assert!((cdf_at_zero - 0.5).abs() < 1e-6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StudentT {
    pub degrees_of_freedom: f64,
}

impl StudentT {
    /// Create a new Student's t-distribution
    ///
    /// # Arguments
    ///
    /// * `degrees_of_freedom` - Degrees of freedom (ν)
    ///
    /// # Panics
    ///
    /// Panics if `degrees_of_freedom <= 0.0`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_ti::distributions::StudentT;
    ///
    /// let student_t = StudentT::new(10.0);
    /// ```
    pub fn new(degrees_of_freedom: f64) -> Self {
        if degrees_of_freedom <= 0.0 {
            panic!(
                "Degrees of freedom ({}) must be greater than 0.0",
                degrees_of_freedom
            );
        }
        StudentT { degrees_of_freedom }
    }
}

impl Distribution for StudentT {
    fn pdf(&self, x: f64) -> f64 {
        let nu = self.degrees_of_freedom;
        let coefficient = gamma((nu + 1.0) / 2.0) / ((nu * PI).sqrt() * gamma(nu / 2.0));
        let base = 1.0 + x.powi(2) / nu;
        coefficient * base.powf(-(nu + 1.0) / 2.0)
    }

    fn cdf(&self, x: f64) -> f64 {
        // Using approximation for Student's t CDF
        let nu = self.degrees_of_freedom;
        if x == 0.0 {
            return 0.5;
        }

        if x > 0.0 {
            0.5 + 0.5 * incomplete_beta(nu / (nu + x.powi(2)), nu / 2.0, 0.5)
        } else {
            0.5 - 0.5 * incomplete_beta(nu / (nu + x.powi(2)), nu / 2.0, 0.5)
        }
    }

    fn mean(&self) -> f64 {
        if self.degrees_of_freedom > 1.0 {
            0.0
        } else {
            f64::NAN
        }
    }

    fn variance(&self) -> f64 {
        let nu = self.degrees_of_freedom;
        if nu > 2.0 {
            nu / (nu - 2.0)
        } else if nu > 1.0 {
            f64::INFINITY
        } else {
            f64::NAN
        }
    }
}

/// Laplace (double exponential) distribution
///
/// Parameterized by location (μ) and scale (b).
/// Has heavier tails than normal distribution.
///
/// # Examples
///
/// ```rust
/// use rust_ti::distributions::{Distribution, Laplace};
///
/// let laplace = Laplace::new(0.0, 1.0);
/// assert_eq!(laplace.mean(), 0.0);
/// assert_eq!(laplace.variance(), 2.0);
///
/// // CDF at location is 0.5
/// let cdf_at_location = laplace.cdf(0.0);
/// assert!((cdf_at_location - 0.5).abs() < 1e-6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Laplace {
    pub location: f64,
    pub scale: f64,
}

impl Laplace {
    /// Create a new Laplace distribution
    ///
    /// # Arguments
    ///
    /// * `location` - Location parameter (μ)
    /// * `scale` - Scale parameter (b)
    ///
    /// # Panics
    ///
    /// Panics if `scale <= 0.0`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_ti::distributions::Laplace;
    ///
    /// let laplace = Laplace::new(0.0, 1.0);
    /// ```
    pub fn new(location: f64, scale: f64) -> Self {
        if scale <= 0.0 {
            panic!("Scale ({}) must be greater than 0.0", scale);
        }
        Laplace { location, scale }
    }

    /// Standard Laplace distribution (location = 0, scale = 1)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_ti::distributions::Laplace;
    ///
    /// let standard_laplace = Laplace::standard();
    /// ```
    pub fn standard() -> Self {
        Laplace {
            location: 0.0,
            scale: 1.0,
        }
    }
}

impl Distribution for Laplace {
    fn pdf(&self, x: f64) -> f64 {
        let exponent = -(x - self.location).abs() / self.scale;
        (1.0 / (2.0 * self.scale)) * E.powf(exponent)
    }

    fn cdf(&self, x: f64) -> f64 {
        if x < self.location {
            0.5 * E.powf((x - self.location) / self.scale)
        } else {
            1.0 - 0.5 * E.powf(-(x - self.location) / self.scale)
        }
    }

    fn mean(&self) -> f64 {
        self.location
    }

    fn variance(&self) -> f64 {
        2.0 * self.scale.powi(2)
    }
}

/// Log-normal distribution
///
/// Parameterized by μ and σ (parameters of the underlying normal distribution).
/// Used to model positive values where the logarithm is normally distributed.
///
/// # Examples
///
/// ```rust
/// use rust_ti::distributions::{Distribution, LogNormal};
///
/// let lognormal = LogNormal::new(0.0, 1.0);
/// assert!(!lognormal.mean().is_nan());
/// assert!(!lognormal.variance().is_nan());
///
/// // PDF is only positive for x > 0
/// assert!(lognormal.pdf(1.0) > 0.0);
/// assert_eq!(lognormal.pdf(-1.0), 0.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogNormal {
    pub mu: f64,
    pub sigma: f64,
}

impl LogNormal {
    /// Create a new Log-normal distribution
    ///
    /// # Arguments
    ///
    /// * `mu` - μ parameter (mean of underlying normal)
    /// * `sigma` - σ parameter (std dev of underlying normal)
    ///
    /// # Panics
    ///
    /// Panics if `sigma <= 0.0`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_ti::distributions::LogNormal;
    ///
    /// let lognormal = LogNormal::new(0.0, 1.0);
    /// ```
    pub fn new(mu: f64, sigma: f64) -> Self {
        if sigma <= 0.0 {
            panic!("Sigma ({}) must be greater than 0.0", sigma);
        }
        LogNormal { mu, sigma }
    }

    /// Standard Log-normal distribution (mu = 0, sigma = 1)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_ti::distributions::LogNormal;
    ///
    /// let standard_lognormal = LogNormal::standard();
    /// ```
    pub fn standard() -> Self {
        LogNormal {
            mu: 0.0,
            sigma: 1.0,
        }
    }
}

impl Distribution for LogNormal {
    fn pdf(&self, x: f64) -> f64 {
        if x <= 0.0 {
            return 0.0;
        }
        let coefficient = 1.0 / (x * self.sigma * (2.0 * PI).sqrt());
        let exponent = -((x.ln() - self.mu).powi(2)) / (2.0 * self.sigma.powi(2));
        coefficient * E.powf(exponent)
    }

    fn cdf(&self, x: f64) -> f64 {
        if x <= 0.0 {
            return 0.0;
        }
        let z = (x.ln() - self.mu) / (self.sigma * SQRT_2);
        0.5 * (1.0 + erf(z))
    }

    fn mean(&self) -> f64 {
        E.powf(self.mu + self.sigma.powi(2) / 2.0)
    }

    fn variance(&self) -> f64 {
        let exp_2mu_sigma2 = E.powf(2.0 * self.mu + self.sigma.powi(2));
        exp_2mu_sigma2 * (E.powf(self.sigma.powi(2)) - 1.0)
    }
}

// Helper functions for special mathematical functions

/// Error function approximation using Abramowitz and Stegun method
fn erf(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x >= 0.0 { 1.0 } else { -1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

/// Gamma function approximation using Lanczos approximation
fn gamma(z: f64) -> f64 {
    // Lanczos approximation coefficients
    const G: f64 = 7.0;
    const COEF: [f64; 9] = [
        0.99999999999980993,
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
        9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];

    if z < 0.5 {
        PI / ((PI * z).sin() * gamma(1.0 - z))
    } else {
        let z = z - 1.0;
        let mut x = COEF[0];
        for i in 1..9 {
            x += COEF[i] / (z + i as f64);
        }
        let t = z + G + 0.5;
        (2.0 * PI).sqrt() * t.powf(z + 0.5) * (-t).exp() * x
    }
}

/// Incomplete beta function approximation
fn incomplete_beta(x: f64, a: f64, b: f64) -> f64 {
    // Simple approximation for incomplete beta function
    // For more accuracy, a more sophisticated implementation would be needed
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }

    // Use continued fraction expansion
    let bt = if x == 0.0 || x == 1.0 {
        0.0
    } else {
        (gamma(a + b) / (gamma(a) * gamma(b))) * x.powf(a) * (1.0 - x).powf(b)
    };

    if x < (a + 1.0) / (a + b + 2.0) {
        bt * betacf(x, a, b) / a
    } else {
        1.0 - bt * betacf(1.0 - x, b, a) / b
    }
}

/// Continued fraction for incomplete beta function
fn betacf(x: f64, a: f64, b: f64) -> f64 {
    const MAX_ITER: usize = 100;
    const EPSILON: f64 = 1e-10;

    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;

    if d.abs() < 1e-30 {
        d = 1e-30;
    }
    d = 1.0 / d;
    let mut h = d;

    for m in 1..=MAX_ITER {
        let m_f64 = m as f64;
        let m2 = 2.0 * m_f64;

        let aa = m_f64 * (b - m_f64) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa * d;
        if d.abs() < 1e-30 {
            d = 1e-30;
        }
        c = 1.0 + aa / c;
        if c.abs() < 1e-30 {
            c = 1e-30;
        }
        d = 1.0 / d;
        h *= d * c;

        let aa = -(a + m_f64) * (qab + m_f64) * x / ((a + m2) * (qap + m2));
        d = 1.0 + aa * d;
        if d.abs() < 1e-30 {
            d = 1e-30;
        }
        c = 1.0 + aa / c;
        if c.abs() < 1e-30 {
            c = 1e-30;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;

        if (del - 1.0).abs() < EPSILON {
            break;
        }
    }

    h
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn normal_standard_properties() {
        let normal = Normal::standard();
        assert_eq!(normal.mean(), 0.0);
        assert_eq!(normal.variance(), 1.0);
        assert_eq!(normal.std_dev(), 1.0);
    }

    #[test]
    fn normal_pdf_symmetric() {
        let normal = Normal::standard();
        let pdf_pos = normal.pdf(1.0);
        let pdf_neg = normal.pdf(-1.0);
        assert!((pdf_pos - pdf_neg).abs() < EPSILON);
    }

    #[test]
    fn normal_cdf_at_mean() {
        let normal = Normal::standard();
        let cdf = normal.cdf(0.0);
        assert!((cdf - 0.5).abs() < EPSILON);
    }

    #[test]
    fn normal_cdf_monotonic() {
        let normal = Normal::standard();
        let cdf1 = normal.cdf(-1.0);
        let cdf2 = normal.cdf(0.0);
        let cdf3 = normal.cdf(1.0);
        assert!(cdf1 < cdf2);
        assert!(cdf2 < cdf3);
    }

    #[test]
    #[should_panic]
    fn normal_invalid_std_dev() {
        Normal::new(0.0, 0.0);
    }

    #[test]
    fn cauchy_undefined_moments() {
        let cauchy = Cauchy::standard();
        assert!(cauchy.mean().is_nan());
        assert!(cauchy.variance().is_nan());
        assert!(cauchy.std_dev().is_nan());
    }

    #[test]
    fn cauchy_cdf_at_location() {
        let cauchy = Cauchy::standard();
        let cdf = cauchy.cdf(0.0);
        assert!((cdf - 0.5).abs() < EPSILON);
    }

    #[test]
    fn cauchy_pdf_symmetric() {
        let cauchy = Cauchy::standard();
        let pdf_pos = cauchy.pdf(1.0);
        let pdf_neg = cauchy.pdf(-1.0);
        assert!((pdf_pos - pdf_neg).abs() < EPSILON);
    }

    #[test]
    fn cauchy_cdf_monotonic() {
        let cauchy = Cauchy::standard();
        let cdf1 = cauchy.cdf(-2.0);
        let cdf2 = cauchy.cdf(0.0);
        let cdf3 = cauchy.cdf(2.0);
        assert!(cdf1 < cdf2);
        assert!(cdf2 < cdf3);
    }

    #[test]
    #[should_panic]
    fn cauchy_invalid_scale() {
        Cauchy::new(0.0, -1.0);
    }

    #[test]
    fn student_t_mean_defined() {
        let student_t = StudentT::new(5.0);
        assert_eq!(student_t.mean(), 0.0);
    }

    #[test]
    fn student_t_variance_defined() {
        let student_t = StudentT::new(5.0);
        let var = student_t.variance();
        assert!((var - 5.0 / 3.0).abs() < EPSILON);
    }

    #[test]
    fn student_t_cdf_at_zero() {
        let student_t = StudentT::new(10.0);
        let cdf = student_t.cdf(0.0);
        assert!((cdf - 0.5).abs() < EPSILON);
    }

    #[test]
    fn student_t_pdf_symmetric() {
        let student_t = StudentT::new(10.0);
        let pdf_pos = student_t.pdf(1.0);
        let pdf_neg = student_t.pdf(-1.0);
        assert!((pdf_pos - pdf_neg).abs() < EPSILON);
    }

    #[test]
    #[should_panic]
    fn student_t_invalid_df() {
        StudentT::new(0.0);
    }

    #[test]
    fn laplace_properties() {
        let laplace = Laplace::standard();
        assert_eq!(laplace.mean(), 0.0);
        assert_eq!(laplace.variance(), 2.0);
    }

    #[test]
    fn laplace_cdf_at_location() {
        let laplace = Laplace::standard();
        let cdf = laplace.cdf(0.0);
        assert!((cdf - 0.5).abs() < EPSILON);
    }

    #[test]
    fn laplace_pdf_symmetric() {
        let laplace = Laplace::standard();
        let pdf_pos = laplace.pdf(1.0);
        let pdf_neg = laplace.pdf(-1.0);
        assert!((pdf_pos - pdf_neg).abs() < EPSILON);
    }

    #[test]
    fn laplace_cdf_monotonic() {
        let laplace = Laplace::standard();
        let cdf1 = laplace.cdf(-1.0);
        let cdf2 = laplace.cdf(0.0);
        let cdf3 = laplace.cdf(1.0);
        assert!(cdf1 < cdf2);
        assert!(cdf2 < cdf3);
    }

    #[test]
    #[should_panic]
    fn laplace_invalid_scale() {
        Laplace::new(0.0, 0.0);
    }

    #[test]
    fn lognormal_properties() {
        let lognormal = LogNormal::standard();
        assert!(!lognormal.mean().is_nan());
        assert!(!lognormal.variance().is_nan());
    }

    #[test]
    fn lognormal_pdf_positive_only() {
        let lognormal = LogNormal::standard();
        assert!(lognormal.pdf(1.0) > 0.0);
        assert_eq!(lognormal.pdf(0.0), 0.0);
        assert_eq!(lognormal.pdf(-1.0), 0.0);
    }

    #[test]
    fn lognormal_cdf_positive_only() {
        let lognormal = LogNormal::standard();
        assert_eq!(lognormal.cdf(0.0), 0.0);
        assert_eq!(lognormal.cdf(-1.0), 0.0);
        assert!(lognormal.cdf(1.0) > 0.0);
    }

    #[test]
    fn lognormal_cdf_monotonic() {
        let lognormal = LogNormal::standard();
        let cdf1 = lognormal.cdf(0.5);
        let cdf2 = lognormal.cdf(1.0);
        let cdf3 = lognormal.cdf(2.0);
        assert!(cdf1 < cdf2);
        assert!(cdf2 < cdf3);
    }

    #[test]
    #[should_panic]
    fn lognormal_invalid_sigma() {
        LogNormal::new(0.0, -1.0);
    }

    #[test]
    fn normal_custom_parameters() {
        let normal = Normal::new(10.0, 2.0);
        assert_eq!(normal.mean(), 10.0);
        assert_eq!(normal.variance(), 4.0);
        assert_eq!(normal.std_dev(), 2.0);

        let cdf = normal.cdf(10.0);
        assert!((cdf - 0.5).abs() < EPSILON);
    }

    #[test]
    fn cauchy_custom_parameters() {
        let cauchy = Cauchy::new(5.0, 2.0);
        let cdf = cauchy.cdf(5.0);
        assert!((cdf - 0.5).abs() < EPSILON);
    }

    #[test]
    fn laplace_custom_parameters() {
        let laplace = Laplace::new(3.0, 1.5);
        assert_eq!(laplace.mean(), 3.0);
        assert_eq!(laplace.variance(), 2.0 * 1.5 * 1.5);

        let cdf = laplace.cdf(3.0);
        assert!((cdf - 0.5).abs() < EPSILON);
    }

    #[test]
    fn erf_properties() {
        assert!((erf(0.0) - 0.0).abs() < EPSILON);
        assert!(erf(1.0) > 0.0);
        assert!(erf(-1.0) < 0.0);
        assert!((erf(1.0) + erf(-1.0)).abs() < EPSILON); // erf is odd
    }

    #[test]
    fn gamma_positive_integers() {
        // gamma(n) = (n-1)! for positive integers
        assert!((gamma(1.0) - 1.0).abs() < EPSILON); // 0! = 1
        assert!((gamma(2.0) - 1.0).abs() < EPSILON); // 1! = 1
        assert!((gamma(3.0) - 2.0).abs() < EPSILON); // 2! = 2
        assert!((gamma(4.0) - 6.0).abs() < EPSILON); // 3! = 6
    }
}
