//! Example demonstrating the distributions module
//!
//! This example shows how to use the various probability distributions
//! provided by the RustTI library.

use rust_ti::distributions::{Cauchy, Distribution, Laplace, LogNormal, Normal, StudentT};

fn main() {
    println!("=== RustTI Distributions Example ===\n");

    // Normal Distribution
    println!("1. Normal Distribution (Gaussian)");
    let normal = Normal::new(0.0, 1.0); // Standard normal
    println!("   Mean: {:?}", normal.mean());
    println!("   Variance: {:?}", normal.variance());
    println!("   Std Dev: {:?}", normal.std_dev());
    println!("   PDF at x=0.0: {:.6}", normal.pdf(0.0));
    println!("   CDF at x=0.0: {:.6}", normal.cdf(0.0));
    println!("   CDF at x=1.0: {:.6}", normal.cdf(1.0));
    println!();

    // Custom normal distribution
    let custom_normal = Normal::new(100.0, 15.0);
    println!("2. Custom Normal Distribution (μ=100, σ=15)");
    println!("   Mean: {:?}", custom_normal.mean());
    println!("   PDF at x=100.0: {:.6}", custom_normal.pdf(100.0));
    println!("   CDF at x=100.0: {:.6}", custom_normal.cdf(100.0));
    println!();

    // Cauchy Distribution
    println!("3. Cauchy Distribution (Heavy-tailed)");
    let cauchy = Cauchy::standard();
    println!("   Mean: {:?} (undefined)", cauchy.mean());
    println!("   Variance: {:?} (undefined)", cauchy.variance());
    println!("   PDF at x=0.0: {:.6}", cauchy.pdf(0.0));
    println!("   CDF at x=0.0: {:.6}", cauchy.cdf(0.0));
    println!();

    // Student's t-Distribution
    println!("4. Student's t-Distribution (df=5)");
    let student_t = StudentT::new(5.0);
    println!("   Mean: {:?}", student_t.mean());
    println!("   Variance: {:?}", student_t.variance());
    println!("   PDF at x=0.0: {:.6}", student_t.pdf(0.0));
    println!("   CDF at x=0.0: {:.6}", student_t.cdf(0.0));
    println!();

    // Laplace Distribution
    println!("5. Laplace Distribution (Double Exponential)");
    let laplace = Laplace::standard();
    println!("   Mean: {:?}", laplace.mean());
    println!("   Variance: {:?}", laplace.variance());
    println!("   PDF at x=0.0: {:.6}", laplace.pdf(0.0));
    println!("   CDF at x=0.0: {:.6}", laplace.cdf(0.0));
    println!();

    // Log-Normal Distribution
    println!("6. Log-Normal Distribution");
    let lognormal = LogNormal::standard();
    println!("   Mean: {:?}", lognormal.mean());
    println!("   Variance: {:?}", lognormal.variance());
    println!("   PDF at x=1.0: {:.6}", lognormal.pdf(1.0));
    println!("   CDF at x=1.0: {:.6}", lognormal.cdf(1.0));
    println!("   PDF at x=0.0: {:.6} (always 0 for x≤0)", lognormal.pdf(0.0));
    println!();

    // Comparison of tail behavior
    println!("7. Comparison of tail probabilities (P(X > 3))");
    let normal_tail = 1.0 - Normal::standard().cdf(3.0);
    let cauchy_tail = 1.0 - Cauchy::standard().cdf(3.0);
    let student_t_tail = 1.0 - StudentT::new(5.0).cdf(3.0);
    let laplace_tail = 1.0 - Laplace::standard().cdf(3.0);

    println!("   Normal: {:.6}", normal_tail);
    println!("   Cauchy: {:.6} (heaviest tails)", cauchy_tail);
    println!("   Student-t (df=5): {:.6}", student_t_tail);
    println!("   Laplace: {:.6}", laplace_tail);
    println!();

    println!("=== Example Complete ===");
}
