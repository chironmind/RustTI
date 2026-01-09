# 🆕 We have rebranded from RustTI to Centaur Technical Indicators!
# The functionality remains the same, only the name and branding have changed.
# Please update any references to RustTI in your projects to Centaur Technical Indicators.
# This change aligns us with the Centaur Labs ecosystem (exciting work coming soon!).
# All packages now follow a consistent naming scheme.

[![Crates.io Version](https://img.shields.io/crates/v/centaur_technical_indicators.svg)](https://crates.io/crates/centaur_technical_indicators)
[![Docs.rs](https://docs.rs/centaur_technical_indicators/badge.svg)](https://docs.rs/centaur_technical_indicators/)
[![CI](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/actions/workflows/rust.yml/badge.svg)](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/actions)
[![License](https://img.shields.io/github/license/ChironMind/CentaurTechnicalIndicators-Rust)](LICENSE-MIT)

[![Crates.io Downloads](https://img.shields.io/crates/d/centaur_technical_indicators.svg)](https://crates.io/crates/centaur_technical_indicators)
[![Tutorials](https://img.shields.io/badge/Tutorials-Available-brightgreen?style=flat&logo=book)](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-tutorials)
[![Benchmarks](https://img.shields.io/badge/Performance-Microsecond-blue?logo=zap)](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-benchmarks)

# 🦀 Meet the Centaur Technical Indicators Mascot

Say hello to our clawed crusader of candlesticks, the battle-hardened mascot of Centaur Technical Indicators! 🦀📈

Forged from rusted metal and born in the depths of the financial abyss, this crustacean doesn't just ride sideways markets — he lives for them. With a stack of notebooks, a thousand-yard stare, and more indicators on his screen than legs on his body, he's the ultimate trading companion. He reads charts, calculates MACD in his sleep, and isn’t afraid to pinch your code into shape.

Welcome to Centaur Technical Indicators — powered by precision, performance, and one extremely serious crustacean from Centaur Labs.

# Centaur Technical Indicators

A highly configurable and high-performance technical indicators library written in pure Rust. 

Designed for flexibility, speed, and advanced use cases in quantitative and algorithmic trading.

Part of the Centaur Labs ecosystem — visit [https://centaurlabs.pages.dev/](https://centaurlabs.pages.dev/) for more.

Looking for the Python bindings? See: [CentaurTechnicalIndicators-Python](https://github.com/chironmind/CentaurTechnicalIndicators-Python)

Looking for the WASM bindings? See: [CentaurTechnicalIndicators-JS](https://github.com/chironmind/CentaurTechnicalIndicators-JS)

---

## 🚀 Getting Started (Tutorial)

> The fastest way to get up and running with Centaur Technical Indicators.

**1. Add Centaur Technical Indicators to your project:**

```shell
cargo add centaur_technical_indicators
```
Or, manually in your `Cargo.toml`:
```toml
centaur_technical_indicators = "1.0.0"
```

**2. Calculate your first indicator:**

```rust
use centaur_technical_indicators;

let prices = vec![100.2, 100.46, 100.53, 100.38, 100.19];

let ma = centaur_technical_indicators::moving_average::single::moving_average(
    &prices,
    centaur_technical_indicators::MovingAverageType::Simple
);
println!("Simple Moving Average: {}", ma);
```
Expected output:
```
Simple Moving Average: 100.352
```

**3. Explore more tutorials**

- [Getting started tutorial](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-tutorials/blob/main/getting_started.md)
- [Choosing the right model](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-tutorials/blob/main/choose_right_model.md)
- [Building your first strategy](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-tutorials/blob/main/first_strategy.md)
- [Backtesting tutorial](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-tutorials/blob/main/backtest.md)
- [Visualization tutorial](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-tutorials/blob/main/visualization.md)

---

## 🛠️ How-To Guides

> Task-oriented guides for common problems and advanced scenarios.

- [Load CSV Price Data:](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-how-to-guides/blob/main/load_csv.md) Parse OHLC prices from a file and calculate RSI 
- [When to use a bulk vs single module:](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-how-to-guides/blob/main/bulk_vs_single.md) Understand when to use bulk functions or single functions
- [Choosing the right constant model type:](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-how-to-guides/blob/main/choose_constant_model_type.md) Programatically determine the best `ConstantModelType` 
- [Choosing the right deviation model:](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-how-to-guides/blob/main/choose_deviation_model.md) Programatically determine the best deviation model 
- [Choosing the right period:](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-how-to-guides/blob/main/choose_period.md) Programatically determine the best period
- [How to use the Personalised Moving Average:](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-how-to-guides/blob/main/personliased_moving_average.md) Programatically determine the alpha of the moving average
- [How to use the McGinley dynamic variation of functions:](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-how-to-guides/blob/main/mcginley_dynamic.md) Quick guide on how to use the McGinley Dynamic functions 


*(Contributions welcome! Submit your favorite how-to guide as a PR.)*

---


## 📚 Reference

> For complete API details, see [docs.rs/centaur_technical_indicators](https://docs.rs/centaur_technical_indicators/).

### Example

A reference of how to call each function can be found 

- [Reference Example](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/blob/main/examples/reference.rs)

Clone and run:
```shell
cargo build
cargo run --example reference
```


### Library Structure

- Modules based on their analysis areas (**`moving_average`**, **`momentum_indicators`**, **`strength_indicators`**...)
- **`bulk` & `single` submodules**  
  - `bulk`: Compute indicator over rolling periods, returns a vector.
  - `single`: Compute indicator for the entire vector, returns a single value.
- Types used to personalise the technical indicators (**`MovingAverageType`**, **`DeviationModel`**, **`Position`**...)

---

## 🧠 Explanation & Design

### Why Centaur Technical Indicators?

- **Performance:** Pure Rust implementation for maximal speed, safety, and zero dependencies.
- **Configurability:** Most indicators are highly customizable—tweak calculation methods, periods, or even use medians instead of means.
- **Breadth:** Covers a wide range of technical indicators out of the box.
- **Advanced Use:** Designed for users who understand technical analysis and want deep control.

**Note:** Some features may require background in technical analysis. See [Investopedia: Technical Analysis](https://www.investopedia.com/terms/t/technicalanalysis.asp) for a primer.

---

## 📈 Available Indicators

All indicators are grouped and split into modules based on their analysis area.  
Each module has `bulk` (vector output) and `single` (scalar output) submodules.

### Standard Indicators
- Simple, Smoothed, Exponential Moving Average, Bollinger Bands, MACD, RSI

### Basic Indicators
- Absolute Deviation, Log, Mean, Median, Mode, Std. Deviation, Variance, Max/Min

### Candle Indicators
- Ichimoku Cloud, Moving Constant Bands/Envelopes, Donchian Channels, Keltner, Supertrend

### Chart Trends
- Trend break down, overall trends, peak/valley trends

### Correlation Indicators
- Correlate asset prices

### Momentum Indicators
- Chaikin Oscillator, CCI, MACD, Money Flow Index, On Balance Volume, ROC, RSI, Williams %R

### Moving Averages
- McGinley Dynamic, Moving Average

### Other Indicators
- ROI, True Range, ATR, Internal Bar Strength

### Strength Indicators
- Accumulation/Distribution, PVI, NVI, RVI

### Trend Indicators
- Aroon (Up/Down/Oscillator), Parabolic, DM, Volume-Price Trend, TSI

### Volatility Indicators
- Ulcer Index, Volatility System

---

## 📊 Performance Benchmarks

Want to know how fast Centaur Technical Indicators runs in real-world scenarios?  
We provide detailed, reproducible benchmarks using realistic OHLCV data and a variety of indicators.

### Momentum Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `relative_strength_index`                     | 573.86 µs          |
| `stochastic_oscillator`                       | 784.13 µs          |
| `slow_stochastic`                             | 28.866 µs          |
| `slowest_stochastic`                          | 28.866 µs          |
| `williams_percent_r`                          | 76.256 µs          |
| `money_flow_index`                            | 150.69 µs          |
| `rate_of_change`                              | 5.3984 µs          |
| `on_balance_volume`                           | 17.405 µs          |
| `commodity_channel_index`                     | 103.19 µs          |
| `mcginley_dynamic_commodity_channel_index`    | 66.044 µs          |
| `macd_line`                                   | 51.482 µs          |
| `mcginley_dynamic_macd_line`                  | 44.461 µs          |
| `chaikin_oscillator`                          | 258.33 µs          |
| `percentage_price_oscillator`                 | 58.060 µs          |
| `chande_momentum_oscillator`                  | 370.14 µs          |

### Candle Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `moving_constant_envelopes`                   | 37.572 µs          |
| `mcginley_dynamic_envelopes`                  | 39.264 µs          |
| `moving_constant_bands`                       | 119.70 µs          |
| `mcginley_dynamic_bands`                      | 43.219 µs          |
| `ichimoku_cloud`                              | 192.93 µs          |
| `donchian_channel`                            | 28.481 µs          |
| `keltner_channel`                             | 318.05 µs          |
| `supertrend`                                  | 148.80 µs          |

### Trend Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `aroon_up`                                    | 16.531 µs          |
| `aroon_down`                                  | 16.592 µs          |
| `aroon_indicator`                             | 66.468 µs          |
| `parabolic_time_price_system`                 | 43.939 µs          |
| `directional_movement_system`                 | 88.965 µs          |
| `volume_price_trend`                          | 6.2801 µs          |
| `true_strength_indx`                          | 705.25 µs          |

### Strength Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `accumulation_distribution`                   | 8.2935 µs          |
| `positive_volume_index`                       | 7.6977 µs          |
| `negative_volume_index`                       | 7.6167 µs          |
| `relative_vigor_index`                        | 505.34 µs          |

### Other Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `return_on_investment`                        | 40.962 µs          |
| `true_range`                                  | 3.4663 µs          |
| `average_true_range`                          | 122.08 µs          |
| `internal_bar_strength`                       | 5.3943 µs          |
| `positivity_indicator`                        | 20.683 µs          |

### Basic Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `mean`                                        | 5.7432 µs          |
| `median`                                      | 333.68 µs          |
| `mode`                                        | 931.09 µs          |
| `log`                                         | 20.335 µs          |
| `log_difference`                              | 42.223 µs          |
| `variance`                                    | 20.921 µs          |
| `standard_deviation`                          | 24.095 µs          |
| `absolute_deviation(Mean)`                    | 26.991 µs          |
| `absolute_deviation(Median)`                  | 345.14 µs          |
| `absoluite_deviation(Mode)`                   | 956.83 µs          |

### Chart Trends

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `peaks`                                       | 93.094 µs          |
| `valleys`                                     | 92.119 µs          |
| `peak_trend`                                  | 188.14 µs          |
| `valley_trend`                                | 188.81 µs          |
| `overall_trend`                               | 10.337 µs          |
| `break_down_trends`                           | 14.655 ms          |

### Correlation Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `correlate_asset_prices`                      | 231.14 µs          |

### Moving Average

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `moving_average(Simple)`                      | 17.575 µs          |
| `moving_average(Smoothed)`                    | 76.601 µs          |
| `moving_average(Exponential)`                 | 78.505 µs          |
| `mcginley_dynamic`                            | 39.653 µs          |

### Volatility Indicators

| Function                                      | Time per Operation |
|-----------------------------------------------|--------------------|
| `ulcer_index`                                 | 65.959 µs          |
| `volatility_system`                           | 137.25 µs          |


*These results are from a Raspberry Pi 5 8GB, your machine will likely be faster!*

👉 [See all benchmarks and how to run your own](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-benchmarks)

---

## 🤝 Contributing

Contributions, bug reports, and feature requests are welcome!
- [Open an issue](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/issues)
- [Submit a pull request](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/pulls)
- See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines

---

## 💬 Community & Support

- Start a [discussion](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/discussions)
- File [issues](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/issues)
- Add your project to the [Showcase](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/discussions/categories/show-and-tell)

---

## 📰 Release Notes

**Latest (v1.0.0):**
- Rebranded from RustTI to Centaur Technical Indicators
- Part of the Centaur Labs ecosystem
- All functionality remains the same

[Human friendly changelog →](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/blob/main/CHANGELOG.md)

[Full changelog →](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/releases)

---

## 📄 License

MIT License. See [LICENSE](LICENSE-MIT).

