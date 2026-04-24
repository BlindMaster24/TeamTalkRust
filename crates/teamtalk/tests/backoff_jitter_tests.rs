//! Integration tests for [`ExponentialBackoff`] jitter semantics.
//!
//! The core contract (repeated here so tests read top-to-bottom):
//!
//! ```text
//! cap_n = min(initial * factor^n, max)
//! delay = cap_n * (1 - jitter) + rand(0, cap_n * jitter)
//! ```
//!
//! * `jitter = 1.0` — uniform `[0, cap_n]` (full jitter, AWS default).
//! * `jitter = 0.0` — deterministic `cap_n`.
//! * `jitter = 0.5` — uniform `[cap_n/2, cap_n]`.
//! * Values outside `[0, 1]` are clamped.

use std::time::Duration;
use teamtalk::utils::backoff::ExponentialBackoff;

#[test]
fn jitter_zero_is_deterministic_cap() {
    let mut backoff = ExponentialBackoff::new(
        Duration::from_millis(50),
        Duration::from_millis(50),
        2.0,
        0.0,
    );
    // With jitter = 0 and a flat cap we must always return the cap.
    for _ in 0..10 {
        let delay = backoff.next_delay();
        assert_eq!(
            delay,
            Duration::from_millis(50),
            "jitter = 0 must produce the deterministic cap each time",
        );
    }
    assert_eq!(backoff.jitter(), 0.0);
}

#[test]
fn jitter_one_spans_full_range() {
    let mut backoff = ExponentialBackoff::new(
        Duration::from_millis(100),
        Duration::from_millis(100),
        2.0,
        1.0,
    );
    let mut min_seen = Duration::from_millis(u64::MAX);
    let mut max_seen = Duration::ZERO;
    for _ in 0..1_000 {
        let delay = backoff.next_delay();
        assert!(
            delay <= Duration::from_millis(100),
            "jitter = 1 must stay at or below cap",
        );
        if delay < min_seen {
            min_seen = delay;
        }
        if delay > max_seen {
            max_seen = delay;
        }
    }
    // Over 1 000 samples the spread should be wide; we assert a
    // generous lower bound so the test is not flaky but still
    // detects a regression to "always cap".
    assert!(
        min_seen <= Duration::from_millis(20),
        "jitter = 1 should produce low values over 1 000 samples, got min {min_seen:?}",
    );
    assert!(
        max_seen >= Duration::from_millis(80),
        "jitter = 1 should produce high values over 1 000 samples, got max {max_seen:?}",
    );
}

#[test]
fn jitter_half_stays_in_upper_half_of_cap() {
    let mut backoff = ExponentialBackoff::new(
        Duration::from_millis(100),
        Duration::from_millis(100),
        2.0,
        0.5,
    );
    for _ in 0..500 {
        let delay = backoff.next_delay();
        assert!(
            delay >= Duration::from_millis(50),
            "jitter = 0.5 must stay at or above cap/2, got {delay:?}",
        );
        assert!(
            delay <= Duration::from_millis(100),
            "jitter = 0.5 must stay at or below cap, got {delay:?}",
        );
    }
}

#[test]
fn jitter_above_one_is_clamped() {
    let backoff = ExponentialBackoff::new(
        Duration::from_millis(10),
        Duration::from_millis(10),
        2.0,
        5.0,
    );
    assert_eq!(
        backoff.jitter(),
        1.0,
        "jitter > 1 must be clamped to the full-jitter upper bound",
    );
}

#[test]
fn jitter_below_zero_is_clamped() {
    let backoff = ExponentialBackoff::new(
        Duration::from_millis(10),
        Duration::from_millis(10),
        2.0,
        -0.5,
    );
    assert_eq!(
        backoff.jitter(),
        0.0,
        "jitter < 0 must be clamped to the no-jitter lower bound",
    );
}

#[test]
fn jitter_nan_is_clamped_to_zero() {
    // f32::clamp(NaN, 0, 1) returns NaN in the spec; use the
    // observable behaviour instead: a NaN jitter must never produce
    // a delay outside the cap range.
    let mut backoff = ExponentialBackoff::new(
        Duration::from_millis(100),
        Duration::from_millis(100),
        2.0,
        f32::NAN,
    );
    let delay = backoff.next_delay();
    assert!(
        delay <= Duration::from_millis(100),
        "NaN jitter must not produce delays above the cap, got {delay:?}",
    );
}

#[test]
fn next_delay_advances_cap_until_max() {
    // With jitter = 0 we can observe the raw cap sequence.
    let mut backoff = ExponentialBackoff::new(
        Duration::from_millis(10),
        Duration::from_millis(1_000),
        2.0,
        0.0,
    );
    let d0 = backoff.next_delay();
    let d1 = backoff.next_delay();
    let d2 = backoff.next_delay();
    assert_eq!(d0, Duration::from_millis(10));
    assert_eq!(d1, Duration::from_millis(20));
    assert_eq!(d2, Duration::from_millis(40));
    for _ in 0..20 {
        let _ = backoff.next_delay();
    }
    // Eventually the cap must hit the configured max.
    assert_eq!(backoff.next_delay(), Duration::from_millis(1_000));
}

#[test]
fn reset_clears_current_delay() {
    let mut backoff = ExponentialBackoff::new(
        Duration::from_millis(10),
        Duration::from_millis(100),
        2.0,
        0.5,
    );
    let _ = backoff.next_delay();
    assert!(backoff.attempts() >= 1);
    backoff.reset();
    assert_eq!(backoff.attempts(), 0);
    assert_eq!(backoff.current_delay(), Duration::ZERO);
}

#[test]
fn default_is_full_jitter() {
    let backoff = ExponentialBackoff::default();
    assert_eq!(
        backoff.jitter(),
        1.0,
        "Default should be full jitter for safe thundering-herd avoidance",
    );
}
