//! Exponential backoff helper.
//!
//! Implements an AWS-style exponential-backoff schedule with a
//! parameterised jitter factor (see "Exponential Backoff And Jitter",
//! AWS Architecture Blog, 2015). Given an attempt count `n`, a base
//! delay `b`, a growth factor `f`, a cap `m`, and a jitter knob
//! `j in [0, 1]`, the next delay is:
//!
//! ```text
//! cap_n = min(b * f^n, m)
//! delay = cap_n * (1 - j) + rand(0, cap_n * j)
//! ```
//!
//! The extremes are:
//!
//! * `j = 1.0` — full jitter: `delay = rand(0, cap_n)`.
//! * `j = 0.0` — no jitter:   `delay = cap_n`.
//! * `j = 0.5` — equal jitter: `delay in [0.5*cap_n, cap_n]`.
//!
//! Full jitter is the default and is usually the safest choice for
//! thundering-herd avoidance on reconnect storms.

use rand::{RngExt, rng};
use std::time::Duration;

/// Default jitter factor used by [`Default`].
const DEFAULT_JITTER: f32 = 1.0;

/// Exponential backoff with jitter and a maximum cap.
#[derive(Debug, Clone)]
pub struct ExponentialBackoff {
    initial_delay: Duration,
    max_delay: Duration,
    factor: f32,
    jitter: f32,
    attempts: u32,
    current_val: Duration,
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(120),
            factor: 1.6,
            jitter: DEFAULT_JITTER,
            attempts: 0,
            current_val: Duration::ZERO,
        }
    }
}

impl ExponentialBackoff {
    /// Creates a new backoff schedule.
    ///
    /// * `initial` — base delay for attempt 0.
    /// * `max` — upper cap on any single delay (pre-jitter).
    /// * `factor` — growth factor per attempt; the cap at attempt `n`
    ///   is `initial * factor^n`, then clamped to `max`.
    /// * `jitter` — jitter factor, clamped to `[0.0, 1.0]`. A value
    ///   of `1.0` picks uniformly in `[0, cap]`; `0.0` returns the
    ///   deterministic cap; `0.5` picks uniformly in
    ///   `[cap/2, cap]`. Values outside `[0, 1]` are silently
    ///   clamped.
    #[must_use]
    pub fn new(initial: Duration, max: Duration, factor: f32, jitter: f32) -> Self {
        Self {
            initial_delay: initial,
            max_delay: max,
            factor,
            jitter: jitter.clamp(0.0, 1.0),
            attempts: 0,
            current_val: Duration::ZERO,
        }
    }

    /// Returns the configured jitter factor, always in `[0.0, 1.0]`.
    #[must_use]
    pub fn jitter(&self) -> f32 {
        self.jitter
    }

    /// Returns the next delay in the schedule.
    #[must_use]
    pub fn next_delay(&mut self) -> Duration {
        if self.attempts == 0 && self.initial_delay.is_zero() {
            self.attempts += 1;
            self.current_val = Duration::ZERO;
            return Duration::ZERO;
        }

        let base = if self.initial_delay.is_zero() {
            Duration::from_millis(100)
        } else {
            self.initial_delay
        };

        // Compute the cap in integer milliseconds to avoid panics
        // from `Duration::from_secs_f32` on overflow / NaN and to
        // keep exact integer growth for typical millisecond-scale
        // backoffs (no floating-point drift for bases like 10 ms).
        let max_millis_u128 = self.max_delay.as_millis();
        let mut cap_millis_u128 = base.as_millis().min(max_millis_u128);
        for _ in 0..self.attempts {
            if cap_millis_u128 >= max_millis_u128 {
                cap_millis_u128 = max_millis_u128;
                break;
            }
            let factor = f64::from(self.factor);
            if !factor.is_finite() || factor <= 1.0 {
                break;
            }
            let next = ((cap_millis_u128 as f64) * factor) as u128;
            if next <= cap_millis_u128 {
                break;
            }
            cap_millis_u128 = next.min(max_millis_u128);
        }
        let cap_millis = cap_millis_u128.min(u128::from(u64::MAX)) as u64;

        self.attempts += 1;

        if cap_millis == 0 {
            self.current_val = Duration::ZERO;
            return Duration::ZERO;
        }

        // delay = cap * (1 - j) + rand(0, cap * j)
        //
        // Compute the random span in integer milliseconds so that we
        // stay deterministic in ordering with the pre-existing tests
        // and avoid floating-point drift on the cap boundary.
        let random_span_millis = ((cap_millis as f64) * f64::from(self.jitter)) as u64;
        let fixed_millis = cap_millis.saturating_sub(random_span_millis);
        let jittered = if random_span_millis == 0 {
            fixed_millis
        } else {
            fixed_millis + rng().random_range(0..=random_span_millis)
        };
        self.current_val = Duration::from_millis(jittered);
        self.current_val
    }

    /// Returns the current delay without advancing.
    #[must_use]
    pub fn current_delay(&self) -> Duration {
        self.current_val
    }

    /// Resets the schedule to its initial state.
    pub fn reset(&mut self) {
        self.attempts = 0;
        self.current_val = Duration::ZERO;
    }

    /// Returns the number of attempts.
    #[must_use]
    pub fn attempts(&self) -> u32 {
        self.attempts
    }
}
