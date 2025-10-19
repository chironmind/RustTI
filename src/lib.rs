//! # RustTI
//!
//! **RustTI** is an comprehensive, highly configurable Technical Indicators library for Rust.
//! It empowers you to design, compute, and experiment with a wide variety of
//! technical indicators for financial data analysis.
//!
//! ## Why RustTI?
//! - **Configurable**: Nearly every parameter (from periods to models) is customizable.
//! - **Modern**: Suitable for stocks, crypto, and any asset with arbitrary trading calendars.
//! - **Powerful**: Use industry standards or create your own quant-style indicators.
//! - **Powered by Rust**: Written in pure Rust through and through
//!
//! ## Philosophy
//! Prefer customizing your indicators to fit your market and strategy, just like the best quants do.
//! RustTI gives you the flexibility to do just that.
//!
//! ## Library Structure
//!
//! - Indicators are grouped into modules by type (e.g., `momentum_indicators`, `trend_indicators`).
//! - Each module is split into:
//!   - **single**: Calculate the indicator for a single period or the whole slice.
//!   - **bulk**: Compute the indicator value over a rolling window or for each element in a series.
//!
//! ## Quick Start
//!
//! ```rust
//! use rust_ti::standard_indicators::bulk::rsi;
//! let prices = vec![100.0, 102.0, 103.0, 102.5, 102.8, 103.1, 103.8, 103.9, 104.4, 103.6, 103.1,
//! 102.9, 103.3, 103.7];
//! let my_rsi = rsi(&prices);
//! println!("Your RSI: {:?}", my_rsi);
//! ```
//!
//! ## Modules
//! - [`standard_indicators`] - Industry-standard indicators (RSI, MACD, Bollinger, etc.)
//! - [`basic_indicators`] - Fundamental stats (mean, median, std, etc.)
//! - [`candle_indicators`] - Candle chart tools (Ichimoku, bands, envelopes, etc.)
//! - [`chart_trends`] - Trend and peak/valley analysis
//! - [`correlation_indicators`] - Asset correlation metrics
//! - [`distributions`] - Probability distributions (Normal, Cauchy, Student-t, Laplace, Log-normal)
//! - [`momentum_indicators`] - Momentum and oscillator indicators
//! - [`moving_average`] - Moving averages: simple, smoothed, exponential, McGinley, etc.
//! - [`other_indicators`] - ROI, true range, internal bar strength, etc.
//! - [`strength_indicators`] - Volume and vigor metrics
//! - [`trend_indicators`] - Trend direction and strength
//! - [`volatility_indicators`] - Volatility measures
//!
//! ## API Reference
//!
//! See each module for detailed function docs and examples.
//!
//! ## Types
//! All shared enums and types are re-exported at the crate root for convenience.
//!
//! ## More docs
//!
//! This repository is part of a structured documentation suite:
//!
//! - **Tutorials:** [See here](https://github.com/ChironMind/RustTI-tutorials)
//! - **How-To Guides:** [See here](https://github.com/ChironMind/RustTI-how-to-guides)
//! - **Benchmarks:** [See here](github.com/ChironMind/RustTI-benchmarks)
//! - **Explanations:** Coming soon
//! - **Reference:** You're here!
//!
//! ---

#![allow(unreachable_patterns)]

pub mod basic_indicators;
pub mod candle_indicators;
pub mod chart_trends;
pub mod correlation_indicators;
pub mod distributions;
pub mod momentum_indicators;
pub mod moving_average;
pub mod other_indicators;
pub mod standard_indicators;
pub mod strength_indicators;
pub mod trend_indicators;
pub mod volatility_indicators;

mod types;
pub use types::*;
