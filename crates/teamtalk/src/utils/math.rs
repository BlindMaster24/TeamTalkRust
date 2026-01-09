//! Math helpers used by TeamTalk types.

/// Converts a percentage (0.0-100.0) into a gain value.
pub fn ref_gain(percent: f64) -> i32 {
    if percent <= 0.0 {
        return 0;
    }
    let gain = 82.832 * (0.0508 * percent).exp() - 50.0;
    gain as i32
}
