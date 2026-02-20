use super::storage::StateStore;
use crate::client::Client;
use crate::events::Result;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobErrorPolicy {
    KeepRunning,
    Disable,
}

type Job = dyn FnMut(&Client, &mut dyn StateStore) -> Result<()> + Send;

struct ScheduledJob {
    interval: Duration,
    next_run: Instant,
    enabled: bool,
    on_error: JobErrorPolicy,
    job: Box<Job>,
}

#[derive(Default)]
pub struct Scheduler {
    jobs: Vec<ScheduledJob>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn every<F>(&mut self, interval: Duration, on_error: JobErrorPolicy, job: F)
    where
        F: FnMut(&Client, &mut dyn StateStore) -> Result<()> + Send + 'static,
    {
        let safe_interval = interval.max(Duration::from_millis(10));
        self.jobs.push(ScheduledJob {
            interval: safe_interval,
            next_run: Instant::now() + safe_interval,
            enabled: true,
            on_error,
            job: Box::new(job),
        });
    }

    pub fn tick(&mut self, client: &Client, state: &mut dyn StateStore) -> Result<()> {
        let now = Instant::now();
        for job in &mut self.jobs {
            if !job.enabled || now < job.next_run {
                continue;
            }

            let result = (job.job)(client, state);
            job.next_run = now + job.interval;
            if let Err(err) = result {
                if matches!(job.on_error, JobErrorPolicy::Disable) {
                    job.enabled = false;
                }
                return Err(err);
            }
        }
        Ok(())
    }
}
