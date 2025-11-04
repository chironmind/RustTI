# Indicator Documentation Template Guide

This guide provides templates for creating indicator documentation for Zola Static Site Generator (SSG) websites. Use these templates when documenting RustTI indicators for web-based documentation.

---

## Overview

Each indicator documentation page consists of:
1. **TOML Front Matter** - Metadata and configuration
2. **Content Sections** - Markdown content describing the indicator (optional, added after the template)

The front matter includes:
- Basic metadata (title, type, category)
- Chart configuration
- Parameter specifications (default and optimized)
- Code usage examples (Rust, Python, JavaScript)
- Optimization code and results
- Trading simulation code and results

---

## Template Types

### Chart Overlay Indicators

Chart overlay indicators are plotted directly on the price chart (e.g., Donchian Channels, Keltner Channels, Bollinger Bands). They typically return multiple values that form bands or channels around price.

#### Template Structure

```toml
+++
title = "[Indicator Name]"

[extra]
indicator_type = "chart_overlay"
indicator_category = "candle"

[extra.chart]
enabled = true
name = "[Indicator Name]"

[extra.default_params]
# List default parameters here
# Example:
# period = 20

[extra.optimized_params]
# List optimized parameters here (from optimization section)
# Example:
# period = 15

[extra.usage]
rust = """
```rust
use rust_ti::candle_indicators::bulk::[indicator_function_name];

pub fn main() {
    // fetch the data in your preferred way
    // let high = vec![...];   // high prices
    // let low = vec![...];    // low prices
    // let close = vec![...];  // closing prices (if needed)

    let [indicator_variable] = [indicator_function_name](&high, &low, [params]);
    println!("{:?}", [indicator_variable]);
}
```
"""
python = """
```python
import pytechnicalindicators as pti

# fetch the data in your preferred way
# close = [...]  # closing prices
# high = [...]   # high prices
# low = [...]    # low prices

[indicator_variable] = pti.candle_indicators.bulk.[indicator_function_name](high, low, [params])
print([indicator_variable])
```
"""
javascript = """
```javascript
// WASM import
import init, { candle_bulk_[indicatorFunctionName] } from 'https://cdn.jsdelivr.net/npm/ti-engine@latest/dist/web/ti_engine.js';

await init();

// fetch the data in your preferred way
// const high = [...];   // high prices
// const low = [...];    // low prices
// const close = [...];  // closing prices (if needed)

const [indicatorVariable] = candle_bulk_[indicatorFunctionName](high, low, [params]);
console.log([indicatorVariable]);
```
"""

[extra.optimization]
rust = """
```rust
use rust_ti::chart_trends::{peaks, valleys};

fn proximity_rating(fuzzed_location: &usize, price_location: &usize) -> f64 {
    1.0 / (*fuzzed_location as f64 - *price_location as f64).abs()
}

pub fn main() {
    // fetch the data in your preferred way
    let indicator_loop = Instant::now();

    // Get buy and sell points
    // For chart overlay indicators, we compare price to the indicator bands
    let sell_points = peaks(&close, 20, 5).into_iter().map(|(_, i)| i).collect::<Vec<usize>>();
    let buy_points = valleys(&close, 20, 5).into_iter().map(|(_, i)| i).collect::<Vec<usize>>();

    // Define the ranges for optimization
    let max_period = 126;
    let min_period = 2;
    let fuzz_parameter = 5; // Allowable distance from buy/sell points

    // Store the best parameters found
    let mut best_rating = 0.0;
    let mut best_period = 0;
    let mut best_indicators = vec![];

    for period in min_period..=max_period {
        let indicators = [indicator_function_name](&high, &low, period);
        let mut rating = vec![];
        let mut matched_sell = vec![];
        let mut matched_buy = vec![];
        
        for i in 0..indicators.len() {
            let price_location = i + period + 1; // Adjust for indicator lag
            if i >= price_location { break; }
            if price_location >= close.len() { break; }
            
            // For chart overlays, typically:
            // indicators[i].0 = lower band (oversold)
            // indicators[i].1 = middle line
            // indicators[i].2 = upper band (overbought)
            let oversold = indicators[i].0;
            let overbought = indicators[i].2;
            
            if close[price_location] > overbought {
                // Price above upper band - potential sell signal
                if sell_points.contains(&price_location) {
                    rating.push(1.0);
                    matched_sell.push(price_location);
                } else if buy_points.contains(&price_location) {
                    rating.push(-1.0);
                } else {
                    let mut found_sell = false;
                    for fuzzed_location in (price_location - fuzz_parameter)..=(price_location + fuzz_parameter) {
                        if sell_points.contains(&fuzzed_location) {
                            rating.push(proximity_rating(&fuzzed_location, &price_location));
                            matched_sell.push(fuzzed_location);
                            found_sell = true;
                        }
                        if buy_points.contains(&fuzzed_location) {
                            if !matched_sell.contains(&fuzzed_location) {
                                rating.push(-proximity_rating(&fuzzed_location, &price_location));
                            }
                        }
                    }
                    if !found_sell {
                        rating.push(0.0);
                    }
                }
            } else if close[price_location] < oversold {
                // Price below lower band - potential buy signal
                if buy_points.contains(&price_location) {
                    rating.push(1.0);
                    matched_buy.push(price_location);
                } else if sell_points.contains(&price_location) {
                    rating.push(-1.0);
                } else {
                    let mut found_buy = false;
                    for fuzzed_location in (price_location - fuzz_parameter)..=(price_location + fuzz_parameter) {
                        if buy_points.contains(&fuzzed_location) {
                            rating.push(proximity_rating(&fuzzed_location, &price_location));
                            matched_buy.push(fuzzed_location);
                            found_buy = true;
                        }
                        if sell_points.contains(&fuzzed_location) {
                            if !matched_buy.contains(&fuzzed_location) {
                                rating.push(-proximity_rating(&fuzzed_location, &price_location));
                            }
                        }
                    }
                    if !found_buy {
                        rating.push(0.0);
                    }
                }
            }
        }
        
        // Penalize missed buy/sell points
        for missed_sell in sell_points.iter() {
            if !matched_sell.contains(missed_sell) {
                rating.push(-1.0);
            }
        }
        for missed_buy in buy_points.iter() {
            if !matched_buy.contains(missed_buy) {
                rating.push(-1.0);
            }
        }
        
        let total_rating: f64 = rating.iter().sum::<f64>() / (rating.len() as f64);
        if total_rating > best_rating {
            best_rating = total_rating;
            best_period = period;
            best_indicators = indicators.clone();
        }
    }

    println!(
        "Indicators optimization loop took {} ms to run",
        indicator_loop.elapsed().as_millis()
    );

    println!("\\nBest Indicator parameters found:");
    println!("period = {}", best_period);
    println!("Rating: {}", best_rating);
    println!("Best Indicator values: {:?}", best_indicators);
}
```
"""
output = """
```
Best Indicator parameters found:
period = [optimized_value]
Rating: [rating_value]
Best Indicator values: [(lower, middle, upper), ...]
```
"""

[extra.trading_simulation]
rust = """
```rust
fn simulate_trading(best_indicator: &[(f64, f64, f64)], best_period: usize, close: &[f64]) {
    println!("\\n--- Trading Simulation ---");

    let initial_capital = 1000.0;
    let mut capital = initial_capital;
    let investment_pct = 0.20;

    struct Position {
        entry_price: f64,
        shares: f64,
    }

    let mut open_long: Option<Position> = None;
    let mut open_short: Option<Position> = None;

    // Print table header
    println!("{:<5} | {:<19} | {:<10} | {:<10} | {:<12} | {:<15} | {:<10}",
             "Day", "Event", "Indicator", "Price", "Shares", "Capital", "P/L");
    println!("{}", "-".repeat(95));

    for i in 0..best_indicator.len() {
        let price_index = i + best_period + 1;
        if price_index >= close.len() { break; }

        let indicator_overbought = best_indicator[i].2;
        let indicator_oversold = best_indicator[i].0;
        let current_price = close[price_index];
        let day = price_index;

        // --- Handle Long Position ---
        if let Some(long_pos) = open_long.take() {
            if current_price > indicator_overbought as f64 {
                let sale_value = long_pos.shares * current_price;
                let profit = sale_value - (long_pos.shares * long_pos.entry_price);
                capital += sale_value;
                println!("{:<5} | {:<19} | {:<10.2} | ${:<9.2} | {:<12.4} | ${:<14.2} | ${:<9.2}",
                         day, "Sell (Close Long)", indicator_overbought, current_price, long_pos.shares, capital, profit);
            } else {
                open_long = Some(long_pos);
            }
        } else if current_price < indicator_oversold as f64 && open_short.is_none() {
            let investment = capital * investment_pct;
            let shares_bought = investment / current_price;
            open_long = Some(Position { entry_price: current_price, shares: shares_bought });
            capital -= investment;
            println!("{:<5} | {:<19} | {:<10.2} | ${:<9.2} | {:<12.4} | ${:<14.2} | {}",
                     day, "Buy (Open Long)", indicator_oversold, current_price, shares_bought, capital, "-");
        }

        // --- Handle Short Position ---
        if let Some(short_pos) = open_short.take() {
            if current_price < indicator_oversold as f64 {
                let cost_to_cover = short_pos.shares * current_price;
                let profit = (short_pos.shares * short_pos.entry_price) - cost_to_cover;
                capital += profit;
                println!("{:<5} | {:<19} | {:<10.2} | ${:<9.2} | {:<12.4} | ${:<14.2} | ${:<9.2}",
                         day, "Cover (Close Short)", indicator_oversold, current_price, short_pos.shares, capital, profit);
            } else {
                open_short = Some(short_pos);
            }
        } else if current_price > indicator_overbought as f64 && open_long.is_none() {
            let short_value = capital * investment_pct;
            let shares_shorted = short_value / current_price;
            open_short = Some(Position { entry_price: current_price, shares: shares_shorted });
            println!("{:<5} | {:<19} | {:<10.2} | ${:<9.2} | {:<12.4} | ${:<14.2} | {}",
                     day, "Short (Open Short)", indicator_overbought, current_price, shares_shorted, capital, "-");
        }
    }

    println!("\\n--- Final Results ---");
    if let Some(pos) = open_long {
        println!("Simulation ended with an OPEN LONG position:");
        println!("  - Shares: {:.4}", pos.shares);
        println!("  - Entry Price: ${:.2}", pos.entry_price);
        let last_price = close.last().unwrap_or(&0.0);
        let current_value = pos.shares * last_price;
        capital += current_value;
        println!("  - Position value at last price (${:.2}): ${:.2}", last_price, current_value);
    }
    if let Some(pos) = open_short {
        println!("Simulation ended with an OPEN SHORT position:");
        println!("  - Shares: {:.4}", pos.shares);
        println!("  - Entry Price: ${:.2}", pos.entry_price);
        let last_price = close.last().unwrap_or(&0.0);
        let cost_to_cover = pos.shares * last_price;
        let pnl = (pos.shares * pos.entry_price) - cost_to_cover;
        capital += pnl;
        println!("  - Unrealized P/L at last price (${:.2}): ${:.2}", last_price, pnl);
    }

    let final_pnl = capital - initial_capital;
    println!("\\nInitial Capital: ${:.2}", initial_capital);
    println!("Final Capital:   ${:.2}", capital);
    println!("Total P/L:       ${:.2}", final_pnl);
}

fn main() {
    // Fetch data and perform optimization as shown in the optimization code above
    simulate_trading(&best_indicators, best_period, &close);

    println!("\\nDefault Indicator values for comparison:");
    let default_indicator = [indicator_function_name](&high, &low, [default_period]);
    println!("{:?}", default_indicator);
    simulate_trading(&default_indicator, [default_period], &close);
}
```
"""

[extra.trading_simulation_optimized]
table = """
Day   | Event               | Indicator  | Price      | Shares       | Capital         | P/L
-----------------------------------------------------------------------------------------------
[Add trading simulation results here]
"""
initial_capital = "$1000.00"
final_capital = "$[final_value]"
total_P_L = "$[profit_loss]"
open_position = { position = "[LONG/SHORT/NONE]", shares = 0.0000, entry_price = "$0.00", position_value_at_last_price = "$0.00" }

[extra.trading_simulation_default]
table = """
Day   | Event               | Indicator  | Price      | Shares       | Capital         | P/L
-----------------------------------------------------------------------------------------------
[Add trading simulation results here]
"""
initial_capital = "$1000.00"
final_capital = "$[final_value]"
total_P_L = "$[profit_loss]"
open_position = { position = "[LONG/SHORT/NONE]", shares = 0.0000, entry_price = "$0.00", position_value_at_last_price = "$0.00" }

+++

<!-- Add your indicator description and explanation here in Markdown -->
```

---

### Momentum Oscillator Indicators

Momentum oscillator indicators are plotted in a separate panel below the price chart (e.g., RSI, MACD, Stochastic). They typically oscillate between fixed boundaries and return a single value or a small set of values.

#### Template Structure

```toml
+++
title = "[Indicator Name]"

[extra]
indicator_type = "oscillator"
indicator_category = "momentum"

[extra.chart]
enabled = true
name = "[Indicator Name]"

[extra.default_params]
# List default parameters here
# Example:
# period = 14
# overbought = 70
# oversold = 30

[extra.optimized_params]
# List optimized parameters here
# Example:
# period = 12
# overbought = 75
# oversold = 25

[extra.usage]
rust = """
```rust
use rust_ti::momentum_indicators::bulk::[indicator_function_name];

pub fn main() {
    // fetch the data in your preferred way
    // let close = vec![...];  // closing prices

    let [indicator_variable] = [indicator_function_name](&close, [params]);
    println!("{:?}", [indicator_variable]);
}
```
"""
python = """
```python
import pytechnicalindicators as pti

# fetch the data in your preferred way
# close = [...]  # closing prices

[indicator_variable] = pti.momentum_indicators.bulk.[indicator_function_name](close, [params])
print([indicator_variable])
```
"""
javascript = """
```javascript
// WASM import
import init, { momentum_bulk_[indicatorFunctionName] } from 'https://cdn.jsdelivr.net/npm/ti-engine@latest/dist/web/ti_engine.js';

await init();

// fetch the data in your preferred way
// const close = [...];  // closing prices

const [indicatorVariable] = momentum_bulk_[indicatorFunctionName](close, [params]);
console.log([indicatorVariable]);
```
"""

[extra.optimization]
rust = """
```rust
use rust_ti::chart_trends::{peaks, valleys};

fn proximity_rating(fuzzed_location: &usize, price_location: &usize) -> f64 {
    1.0 / (*fuzzed_location as f64 - *price_location as f64).abs()
}

pub fn main() {
    // fetch the data in your preferred way
    let indicator_loop = Instant::now();

    // Get buy and sell points
    let sell_points = peaks(&close, 20, 5).into_iter().map(|(_, i)| i).collect::<Vec<usize>>();
    let buy_points = valleys(&close, 20, 5).into_iter().map(|(_, i)| i).collect::<Vec<usize>>();

    // Define optimization ranges
    let max_period = 50;
    let min_period = 2;
    let fuzz_parameter = 5;

    // For oscillators, also optimize threshold values
    let overbought_range = 60..=90;
    let oversold_range = 10..=40;

    let mut best_rating = 0.0;
    let mut best_params = (0, 0, 0); // (period, overbought, oversold)
    let mut best_indicators = vec![];

    for period in min_period..=max_period {
        for overbought in overbought_range.clone() {
            for oversold in oversold_range.clone() {
                let indicators = [indicator_function_name](&close, period);
                let mut rating = vec![];
                let mut matched_sell = vec![];
                let mut matched_buy = vec![];
                
                for i in 0..indicators.len() {
                    let price_location = i + period;
                    if price_location >= close.len() { break; }
                    
                    let indicator_value = indicators[i];
                    
                    // Check for overbought (sell signal)
                    if indicator_value > overbought as f64 {
                        if sell_points.contains(&price_location) {
                            rating.push(1.0);
                            matched_sell.push(price_location);
                        } else if buy_points.contains(&price_location) {
                            rating.push(-1.0);
                        } else {
                            // Fuzzy matching logic
                            let mut found_sell = false;
                            for fuzzed_location in (price_location.saturating_sub(fuzz_parameter))..=(price_location + fuzz_parameter) {
                                if sell_points.contains(&fuzzed_location) {
                                    rating.push(proximity_rating(&fuzzed_location, &price_location));
                                    matched_sell.push(fuzzed_location);
                                    found_sell = true;
                                }
                            }
                            if !found_sell {
                                rating.push(0.0);
                            }
                        }
                    }
                    // Check for oversold (buy signal)
                    else if indicator_value < oversold as f64 {
                        if buy_points.contains(&price_location) {
                            rating.push(1.0);
                            matched_buy.push(price_location);
                        } else if sell_points.contains(&price_location) {
                            rating.push(-1.0);
                        } else {
                            // Fuzzy matching logic
                            let mut found_buy = false;
                            for fuzzed_location in (price_location.saturating_sub(fuzz_parameter))..=(price_location + fuzz_parameter) {
                                if buy_points.contains(&fuzzed_location) {
                                    rating.push(proximity_rating(&fuzzed_location, &price_location));
                                    matched_buy.push(fuzzed_location);
                                    found_buy = true;
                                }
                            }
                            if !found_buy {
                                rating.push(0.0);
                            }
                        }
                    }
                }
                
                // Penalize missed points
                for missed_sell in sell_points.iter() {
                    if !matched_sell.contains(missed_sell) {
                        rating.push(-1.0);
                    }
                }
                for missed_buy in buy_points.iter() {
                    if !matched_buy.contains(missed_buy) {
                        rating.push(-1.0);
                    }
                }
                
                let total_rating: f64 = rating.iter().sum::<f64>() / rating.len().max(1) as f64;
                if total_rating > best_rating {
                    best_rating = total_rating;
                    best_params = (period, overbought, oversold);
                    best_indicators = indicators.clone();
                }
            }
        }
    }

    println!("Optimization took {} ms", indicator_loop.elapsed().as_millis());
    println!("\\nBest parameters:");
    println!("period = {}, overbought = {}, oversold = {}", best_params.0, best_params.1, best_params.2);
    println!("Rating: {}", best_rating);
}
```
"""
output = """
```
Best parameters:
period = [value], overbought = [value], oversold = [value]
Rating: [rating]
```
"""

# Trading simulation sections similar to chart overlay template
# ... (omitted for brevity, follow same structure as chart overlay)

+++

<!-- Add your indicator description here -->
```

---

## Complete Example: Donchian Channels

Here's a complete, filled-out example for the Donchian Channels indicator:

```toml
+++
title = "Donchian Channels"

[extra]
indicator_type = "chart_overlay"
indicator_category = "candle"

[extra.chart]
enabled = true
name = "Donchian Channels"

[extra.default_params]
period = 20

[extra.optimized_params]
period = 15

[extra.usage]
rust = """
```rust
use rust_ti::candle_indicators::bulk::donchian_channels;

pub fn main() {
    // fetch the data in your preferred way
    // let high = vec![...];   // high prices
    // let low = vec![...];    // low prices

    let donchian_channels = donchian_channels(&high, &low, 20);
    println!("{:?}", donchian_channels);
}
```
"""
python = """
```python
import pytechnicalindicators as pti

# fetch the data in your preferred way
# close = [...]  # closing prices
# high = [...]   # high prices
# low = [...]    # low prices

donchian_channels = pti.candle_indicators.bulk.donchian_channels(high, low, period=20)
print(donchian_channels)
```
"""
javascript = """
```javascript
// WASM import
import init, { candle_bulk_donchianChannels } from 'https://cdn.jsdelivr.net/npm/ti-engine@latest/dist/web/ti_engine.js';

await init();

// fetch the data in your preferred way
// const high = [...];   // high prices
// const low = [...];    // low prices

const donchian_channels = candle_bulk_donchianChannels(high, low, 20);
console.log(donchian_channels);
```
"""

[extra.optimization]
rust = """
```rust
use rust_ti::chart_trends::{peaks, valleys};

fn proximity_rating(fuzzed_location: &usize, price_location: &usize) -> f64 {
    1.0 / (*fuzzed_location as f64 - *price_location as f64).abs()
}


pub fn main() {

    // fetch the data in your preferred way
    let indicator_loop = Instant::now();

        // get buy and sell points, in an ideal we would buy at the lowest point in the dip and sell at the highest point in the peak
        // In the course of a 20-day period (1 month of trading days), we want to find the highest peak and lowest valley within 5 days of each other
        let sell_points = peaks(&close, 20, 5).into_iter().map(|(_, i)| i).collect::<Vec<usize>>();
        let buy_points = valleys(&close, 20, 5).into_iter().map(|(_, i)| i).collect::<Vec<usize>>();

        // Define the ranges for optimization
        let max_period = 126;
        let min_period = 2;

        let fuzz_parameter = 5; // Allowable distance from buy/sell points

        // Store the best parameters found
        let mut best_rating = 0.0;
        let mut best_period = 0;
        let mut best_indicators = vec![];

        for period in min_period..=max_period {

                                let indicators = donchian_channels(&high, &low, period);
                                let mut rating = vec![];
                                let mut matched_sell = vec![];
                                let mut matched_buy = vec![];
                                for i in 0..indicators.len() {
                                    let price_location = i + period + 1; // Adjust for indicator lag
                                    if i >= price_location { break; }
                                    if price_location >= close.len() { break; }
                                    let oversold = indicators[i].0;
                                    let overbought = indicators[i].2; // Placeholder for overbought threshold
                                    if close[price_location] > overbought {
                                        if sell_points.contains(&price_location) {
                                            // If sell point == rsi, rate positively
                                            rating.push(1.0);
                                            matched_sell.push(price_location);
                                        } else if buy_points.contains(&price_location) {
                                            // If buy point == rsi, rate negatively
                                            rating.push(-1.0);
                                        } else {
                                            let mut found_sell = false;
                                            for fuzzed_location in (price_location - fuzz_parameter)..=(price_location + fuzz_parameter) {
                                                // It's ok if we count multiple times for fuzzed locations as we reduce the rating
                                                // based off of distance from the actual sell point which will impact the final rating
                                                if sell_points.contains(&fuzzed_location) {
                                                    rating.push(proximity_rating(&fuzzed_location, &price_location));
                                                    matched_sell.push(fuzzed_location);
                                                    found_sell = true;
                                                }
                                                if buy_points.contains(&fuzzed_location) {
                                                    // Note the `-` here to penalize for selling instead of buying
                                                    if !matched_sell.contains(&fuzzed_location) {
                                                        rating.push(-proximity_rating(&fuzzed_location, &price_location));
                                                    }
                                                }
                                            }
                                            if !found_sell {
                                                rating.push(0.0);
                                            }
                                        }
                                    } else if close[price_location] < oversold {
                                        if buy_points.contains(&price_location) {
                                            // If buy point == rsi, rate positively
                                            rating.push(1.0);
                                            matched_buy.push(price_location);
                                        } else if sell_points.contains(&price_location) {
                                            rating.push(-1.0);
                                        } else {
                                            let mut found_buy = false;
                                            for fuzzed_location in (price_location - fuzz_parameter)..=(price_location + fuzz_parameter) {
                                                // It's ok if we count multiple times for fuzzed locations as we reduce the rating
                                                // based off of distance from the actual sell point which will impact the final rating
                                                if buy_points.contains(&fuzzed_location) {
                                                    rating.push(proximity_rating(&fuzzed_location, &price_location));
                                                    matched_buy.push(fuzzed_location);
                                                    found_buy = true;
                                                }
                                                if sell_points.contains(&fuzzed_location) {
                                                    // Note the `-` here to penalize for buying instead of selling
                                                    if !matched_buy.contains(&fuzzed_location) {
                                                        rating.push(-proximity_rating(&fuzzed_location, &price_location));
                                                    }
                                                }
                                            }
                                            if !found_buy {
                                                rating.push(0.0);
                                            }
                                        }
                                    }
                                }
                                // Look for any missed buy/sell points and penalize
                                for missed_sell in sell_points.iter() {
                                    if !matched_sell.contains(missed_sell) {
                                        rating.push(-1.0);
                                    }
                                }
                                for missed_buy in buy_points.iter() {
                                    if !matched_buy.contains(missed_buy) {
                                        rating.push(-1.0);
                                    }
                                }
                                let total_rating: f64 = rating.iter().sum::<f64>() / (rating.len() as f64);
                                if total_rating > best_rating {
                                    best_rating = total_rating;
                                    best_period = period;
                                    best_indicators = indicators.clone();

            }
        }

        println!(
            "Indicators optimization loop took {} ms to run",
            indicator_loop.elapsed().as_millis()
        );

        println!("\\nBest Indicator parameters found:");
        println!("period = {}", best_period);
        println!("Rating: {}", best_rating);
        println!("Best Indicator values: {:?}", best_indicators);

}
```
"""
output = """
```
Best Indicator parameters found:
period = 15
Rating: 0.2584229390681003
Best Indicator values: [(5104.35, 5184.6, 5264.85), (5104.35, 5184.6, 5264.85), (5131.59, 5198.22, 5264.85), ...]
```
"""

[extra.trading_simulation]
rust = """
```rust

fn simulate_trading(best_indicator: &[(f64, f64, f64)], best_period: usize, close: &[f64]) {
    println!("\\n--- Trading Simulation ---");

    let initial_capital = 1000.0;
    let mut capital = initial_capital;
    let investment_pct = 0.20;

    struct Position {
        entry_price: f64,
        shares: f64,
    }

    let mut open_long: Option<Position> = None;
    let mut open_short: Option<Position> = None;

    // Print table header
    println!("{:<5} | {:<19} | {:<10} | {:<10} | {:<12} | {:<15} | {:<10}",
             "Day", "Event", "Indicator", "Price", "Shares", "Capital", "P/L");
    println!("{}", "-".repeat(95));

    for i in 0..best_indicator.len() {
        let price_index = i + best_period + 1;
        if price_index >= close.len() { break; }

        let indicator_overbought = best_indicator[i].2;
        let indicator_oversold = best_indicator[i].0;
        let current_price = close[price_index];
        let day = price_index;

        // --- Handle Long Position ---
        if let Some(long_pos) = open_long.take() {
            if current_price > indicator_overbought as f64 {
                let sale_value = long_pos.shares * current_price;
                let profit = sale_value - (long_pos.shares * long_pos.entry_price);
                capital += sale_value;
                println!("{:<5} | {:<19} | {:<10.2} | ${:<9.2} | {:<12.4} | ${:<14.2} | ${:<9.2}",
                         day, "Sell (Close Long)", indicator_overbought, current_price, long_pos.shares, capital, profit);
            } else {
                open_long = Some(long_pos); // Put it back if not selling
            }
        } else if current_price < indicator_oversold as f64 && open_short.is_none() { // Don't buy if short is open
            let investment = capital * investment_pct;
            let shares_bought = investment / current_price;
            open_long = Some(Position { entry_price: current_price, shares: shares_bought });
            capital -= investment;
            println!("{:<5} | {:<19} | {:<10.2} | ${:<9.2} | {:<12.4} | ${:<14.2} | {}",
                     day, "Buy (Open Long)", indicator_oversold, current_price, shares_bought, capital, "-");
        }

        // --- Handle Short Position ---
        if let Some(short_pos) = open_short.take() {
            if current_price < indicator_oversold as f64 {
                let cost_to_cover = short_pos.shares * current_price;
                let profit = (short_pos.shares * short_pos.entry_price) - cost_to_cover;
                capital += profit; // Add profit to capital
                println!("{:<5} | {:<19} | {:<10.2} | ${:<9.2} | {:<12.4} | ${:<14.2} | ${:<9.2}",
                         day, "Cover (Close Short)", indicator_oversold, current_price, short_pos.shares, capital, profit);
            } else {
                open_short = Some(short_pos); // Put it back if not covering
            }
        } else if current_price > indicator_overbought as f64 && open_long.is_none() { // Don't short if long is open
            let short_value = capital * investment_pct;
            let shares_shorted = short_value / current_price;
            open_short = Some(Position { entry_price: current_price, shares: shares_shorted });
            // Capital doesn't change when opening a short, it's held as collateral
            println!("{:<5} | {:<19} | {:<10.2} | ${:<9.2} | {:<12.4} | ${:<14.2} | {}",
                     day, "Short (Open Short)", indicator_overbought, current_price, shares_shorted, capital, "-");
        }
    }

    println!("\\n--- Final Results ---");
    if let Some(pos) = open_long {
        println!("Simulation ended with an OPEN LONG position:");
        println!("  - Shares: {:.4}", pos.shares);
        println!("  - Entry Price: ${:.2}", pos.entry_price);
        let last_price = close.last().unwrap_or(&0.0);
        let current_value = pos.shares * last_price;
        capital += current_value;
        println!("  - Position value at last price (${:.2}): ${:.2}", last_price, current_value);
    }
    if let Some(pos) = open_short {
        println!("Simulation ended with an OPEN SHORT position:");
        println!("  - Shares: {:.4}", pos.shares);
        println!("  - Entry Price: ${:.2}", pos.entry_price);
        let last_price = close.last().unwrap_or(&0.0);
        let cost_to_cover = pos.shares * last_price;
        let pnl = (pos.shares * pos.entry_price) - cost_to_cover;
        capital += pnl;
        println!("  - Unrealized P/L at last price (${:.2}): ${:.2}", last_price, pnl);
    }

    let final_pnl = capital - initial_capital;
    println!("\\nInitial Capital: ${:.2}", initial_capital);
    println!("Final Capital:   ${:.2}", capital);
    println!("Total P/L:       ${:.2}", final_pnl);

}

fn main() {
    // Fetch data and perform optimization as shown in the optimization code above
    simulate_trading(&best_indicators, best_period, &close);

    println!("\\nDefault Indicator values for comparison:");
    let default_dc = donchian_channels(&high, &low, 20);
    println!("{:?}", default_dc);
    simulate_trading(&default_dc, 20, &close);
}

```
"""

[extra.trading_simulation_optimized]
table = """
Day   | Event               | Indicator  | Price      | Shares       | Capital         | P/L
-----------------------------------------------------------------------------------------------
20    | Buy (Open Long)     | 5138.70    | $5123.41   | 0.0390       | $800.00         | -
36    | Sell (Close Long)   | 5175.03    | $5180.74   | 0.0390       | $1002.24        | $2.24
36    | Short (Open Short)  | 5175.03    | $5180.74   | 0.0387       | $1002.24        | -
90    | Cover (Close Short) | 5446.53    | $5427.13   | 0.0387       | $992.70         | $-9.53
91    | Buy (Open Long)     | 5458.43    | $5399.22   | 0.0368       | $794.16         | -
108   | Sell (Close Long)   | 5566.16    | $5608.25   | 0.0368       | $1000.39        | $7.69
108   | Short (Open Short)  | 5566.16    | $5608.25   | 0.0357       | $1000.39        | -
121   | Cover (Close Short) | 5415.91    | $5408.42   | 0.0357       | $1007.52        | $7.13
122   | Buy (Open Long)     | 5480.54    | $5471.05   | 0.0368       | $806.02         | -
130   | Sell (Close Long)   | 5670.81    | $5713.64   | 0.0368       | $1016.46        | $8.93
130   | Short (Open Short)  | 5670.81    | $5713.64   | 0.0356       | $1016.46        | -
160   | Cover (Close Short) | 5745.02    | $5705.45   | 0.0356       | $1016.75        | $0.29
161   | Buy (Open Long)     | 5762.41    | $5728.80   | 0.0355       | $813.40         | -
164   | Sell (Close Long)   | 5878.46    | $5929.04   | 0.0355       | $1023.85        | $7.11
164   | Short (Open Short)  | 5878.46    | $5929.04   | 0.0345       | $1023.85        | -
193   | Cover (Close Short) | 5963.91    | $5872.16   | 0.0345       | $1025.82        | $1.96
194   | Buy (Open Long)     | 5984.87    | $5867.08   | 0.0350       | $820.66         | -
214   | Sell (Close Long)   | 6049.75    | $6086.37   | 0.0350       | $1033.49        | $7.67
214   | Short (Open Short)  | 6049.75    | $6086.37   | 0.0340       | $1033.49        | -
239   | Cover (Close Short) | 5908.49    | $5861.57   | 0.0340       | $1041.12        | $7.63
241   | Buy (Open Long)     | 5858.78    | $5849.72   | 0.0356       | $832.90         | -
"""
initial_capital = "$1000.00"
final_capital = "$1033.62"
total_P_L = "$33.62"
open_position = { position = "LONG", shares = 0.0356, entry_price = "$5849.72", position_value_at_last_price = "$200.72" }

[extra.trading_simulation_default]
table = """
Day   | Event               | Indicator  | Price      | Shares       | Capital         | P/L
-----------------------------------------------------------------------------------------------
21    | Buy (Open Long)     | 5104.35    | $5061.82   | 0.0395       | $800.00         | -
39    | Sell (Close Long)   | 5211.78    | $5214.08   | 0.0395       | $1006.02        | $6.02
39    | Short (Open Short)  | 5211.78    | $5214.08   | 0.0386       | $1006.02        | -
90    | Cover (Close Short) | 5446.53    | $5427.13   | 0.0386       | $997.79         | $-8.22
91    | Buy (Open Long)     | 5446.53    | $5399.22   | 0.0370       | $798.24         | -
108   | Sell (Close Long)   | 5585.34    | $5608.25   | 0.0370       | $1005.52        | $7.73
108   | Short (Open Short)  | 5585.34    | $5608.25   | 0.0359       | $1005.52        | -
207   | Cover (Close Short) | 5829.53    | $5827.04   | 0.0359       | $997.68         | $-7.85
214   | Short (Open Short)  | 6070.67    | $6086.37   | 0.0328       | $997.68         | -
239   | Cover (Close Short) | 5908.49    | $5861.57   | 0.0328       | $1005.04        | $7.37
241   | Buy (Open Long)     | 5858.78    | $5849.72   | 0.0344       | $804.04         | -
"""
initial_capital = "$1000.00"
final_capital = "$997.80"
total_P_L = "$-2.20"
open_position = { position = "LONG", shares = 0.0344, entry_price = "$5849.72", position_value_at_last_price = "$193.77" }

+++

<!-- Indicator description goes here -->

## What are Donchian Channels?

Donchian Channels are a technical analysis indicator used to identify potential breakouts and measure market volatility. Created by Richard Donchian, they consist of three lines:

- **Upper Band**: The highest high over the specified period
- **Middle Line**: The average of the upper and lower bands
- **Lower Band**: The lowest low over the specified period

When price breaks above the upper band, it may signal an uptrend. When price breaks below the lower band, it may signal a downtrend.

## How to Use

- **Breakout Strategy**: Buy when price breaks above the upper band, sell when it breaks below the lower band
- **Mean Reversion**: Buy when price touches the lower band, sell when it touches the upper band
- **Volatility Measurement**: Wider channels indicate higher volatility, narrower channels indicate lower volatility

## Parameters

- **period**: The number of periods to use for calculating the highest high and lowest low (default: 20)
```

---

## Usage Guidelines

1. **Choose the appropriate template** based on your indicator type (chart overlay or momentum oscillator)

2. **Fill in the metadata**:
   - Set the correct `title`
   - Set `indicator_type` to either `"chart_overlay"` or `"oscillator"`
   - Set `indicator_category` appropriately (`"candle"`, `"momentum"`, etc.)

3. **Configure parameters**:
   - Add default parameters in `[extra.default_params]`
   - Run optimization code to find best parameters
   - Add optimized parameters in `[extra.optimized_params]`

4. **Add code examples**:
   - Provide Rust, Python, and JavaScript usage examples
   - Use actual function names from the RustTI library
   - Include comments to guide users

5. **Include optimization code**:
   - Use the template optimization code as a starting point
   - Customize for your specific indicator's return type
   - Run the code and capture the output

6. **Add trading simulation**:
   - Run the trading simulation code
   - Capture the table output and metrics
   - Include both optimized and default parameter results

7. **Write description** (optional):
   - Add Markdown content after the `+++` closing delimiter
   - Explain what the indicator measures
   - Provide usage tips and strategies

---

## Notes

- Replace placeholder text in `[brackets]` with actual values
- Ensure function names match those in RustTI, PyTechnicalIndicators, and ti-engine
- JavaScript function names use camelCase (e.g., `donchianChannels`)
- Rust/Python function names use snake_case (e.g., `donchian_channels`)
- Module prefixes in JavaScript: `candle_bulk_`, `momentum_bulk_`, etc.
- Test all code examples before publishing

---

## Related Resources

- [RustTI Documentation](https://docs.rs/rust_ti/)
- [PyTechnicalIndicators Repository](https://github.com/chironmind/PyTechnicalIndicators)
- [ti-engine Repository](https://github.com/chironmind/ti-engine)
- [CentaurLabs Repository](https://github.com/chironmind/CentaurLabs)
