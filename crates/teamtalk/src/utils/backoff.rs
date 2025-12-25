use rand::{Rng, thread_rng};
use std::time::Duration;

pub struct ExponentialBackoff {
    current_delay: Duration,
    initial_delay: Duration,
    max_delay: Duration,
    factor: f32,
    jitter: f32,
    attempts: u32,
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self {
            current_delay: Duration::from_secs(1),
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
            factor: 2.0,
            jitter: 0.1,
            attempts: 0,
        }
    }
}

impl ExponentialBackoff {
    pub fn new(initial: Duration, max: Duration, factor: f32, jitter: f32) -> Self {
        Self {
            current_delay: initial,
            initial_delay: initial,
            max_delay: max,
            factor,
            jitter,
            attempts: 0,
        }
    }

    pub fn next_delay(&mut self) -> Duration {
        self.attempts += 1;

        let jitter_range = self.current_delay.as_secs_f32() * self.jitter;
        let jitter = if jitter_range > 0.0 {
            thread_rng().gen_range(-jitter_range..jitter_range)
        } else {
            0.0
        };

        let delay_with_jitter =
            Duration::from_secs_f32((self.current_delay.as_secs_f32() + jitter).max(0.0));

        let next_raw = self.current_delay.as_secs_f32() * self.factor;
        self.current_delay = Duration::from_secs_f32(next_raw).min(self.max_delay);

        delay_with_jitter
    }

    pub fn reset(&mut self) {
        self.attempts = 0;
        self.current_delay = self.initial_delay;
    }

    pub fn attempts(&self) -> u32 {
        self.attempts
    }
}
