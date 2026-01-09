# Contributing to Centaur Technical Indicators

Thank you for considering contributing—every improvement is appreciated, big or small!

---

## 🙌 How to Contribute

1. **Fork and clone the repository**  
2. **Make your changes**  
3. **Open a Pull Request (PR)** on [GitHub](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/pulls)
4. **Tag @ChironMind** in your PR for a review
5. **Run benchmarks** from [CentaurTechnicalIndicators-Rust-benchmarks](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-benchmarks)

See [open issues](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust/issues) if you want to start with something small.

---

## 🛠️ What to Work On?

- Refactor parabolic T/P system: remove single functions; let bulk function determine trend (like `volatility` system)
- No more panics, use `Result` types instead, and centralize validation and error messages
  There are lots of repeated checks: empty slices, mismatched lengths, period > len, etc. Also error strings vary (“Prices is empty”, “Prices cannot be empty”, etc.).
  You currently rely on panics by design; that’s okay for a low-level math crate, but consistency matters.
  Refactor suggestion:
  Add internal helpers (private module) like:
  assert_non_empty(name, slice)
  assert_same_len([("high", high), ("low", low), ...])
  assert_period(period, len)
  Use them everywhere so error messages and edge-case behavior are uniform.

### New Indicator Ideas

- McGinley dynamic versions of indicators using MA (Donchian, Keltner, Supertrend, PPO, RVI, etc.)?

---

## ➕ Adding a New Indicator

1. **Open an Issue**  
   - Describe the indicator, what it’s used for, and how it’s calculated (with source/reference)
2. **Implement the indicator**  
   - Add documentation and unit tests
3. **Verify results**  
   - Add a tab to `assets/centaur_ti_hand_calcs.ods` with hand calculations to ensure test accuracy
4. **Add to benchmarks and run**
   - Add a bench in [CentaurTechnicalIndicators-Rust-benchmarks](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-benchmarks), run it, and document the runtime

---

## 🧪 Code Style & Testing

- Format code with `rustfmt` or `cargo fmt`
- Run tests with `cargo test` before submitting your PR

---

Thanks again for your interest and contributions!

