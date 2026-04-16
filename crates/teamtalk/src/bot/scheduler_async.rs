use super::scheduler::JobErrorPolicy;
use super::storage::StateStore;
use crate::client::Client;
use crate::events::Result;
use std::time::{Duration, Instant};

type AsyncJob = dyn FnMut(&Client, &mut dyn StateStore) -> Result<()> + Send;

#[cfg(feature = "async")]
struct AsyncScheduledJob {
    name: String,
    interval: Duration,
    next_run: Instant,
    enabled: bool,
    one_shot: bool,
    on_error: JobErrorPolicy,
    job: Box<AsyncJob>,
}

#[cfg(feature = "async")]
#[derive(Default)]
pub struct AsyncScheduler {
    jobs: Vec<AsyncScheduledJob>,
}

#[cfg(feature = "async")]
impl AsyncScheduler {
    #[allow(clippy::must_use_candidate)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn every<F>(&mut self, name: &str, interval: Duration, on_error: JobErrorPolicy, job: F)
    where
        F: FnMut(&Client, &mut dyn StateStore) -> Result<()> + Send + 'static,
    {
        let safe_interval = interval.max(Duration::from_millis(10));
        self.jobs.push(AsyncScheduledJob {
            name: name.to_owned(),
            interval: safe_interval,
            next_run: Instant::now() + safe_interval,
            enabled: true,
            one_shot: false,
            on_error,
            job: Box::new(job),
        });
    }

    pub fn after<F>(&mut self, name: &str, delay: Duration, job: F)
    where
        F: FnMut(&Client, &mut dyn StateStore) -> Result<()> + Send + 'static,
    {
        let safe_delay = delay.max(Duration::from_millis(10));
        self.jobs.push(AsyncScheduledJob {
            name: name.to_owned(),
            interval: safe_delay,
            next_run: Instant::now() + safe_delay,
            enabled: true,
            one_shot: true,
            on_error: JobErrorPolicy::Disable,
            job: Box::new(job),
        });
    }

    pub fn remove(&mut self, name: &str) -> bool {
        let before = self.jobs.len();
        self.jobs.retain(|j| j.name != name);
        self.jobs.len() < before
    }

    pub fn is_enabled(&self, name: &str) -> Option<bool> {
        self.jobs.iter().find(|j| j.name == name).map(|j| j.enabled)
    }

    pub fn set_enabled(&mut self, name: &str, enabled: bool) {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.name == name) {
            job.enabled = enabled;
        }
    }

    pub fn job_names(&self) -> Vec<&str> {
        self.jobs.iter().map(|j| j.name.as_str()).collect()
    }

    pub fn next_run_delay(&self) -> Duration {
        let now = Instant::now();
        self.jobs
            .iter()
            .filter(|j| j.enabled)
            .map(|j| j.next_run.saturating_duration_since(now))
            .min()
            .unwrap_or(Duration::from_secs(3600))
    }

    pub fn tick(&mut self, client: &Client, state: &mut dyn StateStore) -> Result<()> {
        let now = Instant::now();
        for job in &mut self.jobs {
            if !job.enabled || now < job.next_run {
                continue;
            }
            let result = (job.job)(client, state);
            if job.one_shot {
                job.enabled = false;
            } else {
                job.next_run = now + job.interval;
            }
            if let Err(err) = result {
                if matches!(job.on_error, JobErrorPolicy::Disable) {
                    job.enabled = false;
                }
                return Err(err);
            }
        }
        self.jobs.retain(|j| !j.one_shot || j.enabled);
        Ok(())
    }
}
