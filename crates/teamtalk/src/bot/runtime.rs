use super::router::HandlerResult;
use super::{MemoryStateStore, Router, Scheduler, StateStore};
use crate::client::Client;
use crate::events::Result;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct BotConfig {
    pub poll_timeout_ms: i32,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            poll_timeout_ms: 100,
        }
    }
}

impl BotConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn poll_timeout_ms(mut self, timeout_ms: i32) -> Self {
        self.poll_timeout_ms = timeout_ms;
        self
    }
}

pub struct Bot {
    client: Client,
    router: Router,
    scheduler: Scheduler,
    state: Box<dyn StateStore>,
    stop: Arc<AtomicBool>,
    config: BotConfig,
}

pub struct BotBuilder {
    client: Client,
    router: Router,
    scheduler: Scheduler,
    state: Box<dyn StateStore>,
    config: BotConfig,
}

impl BotBuilder {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            router: Router::new(),
            scheduler: Scheduler::new(),
            state: Box::new(MemoryStateStore::new()),
            config: BotConfig::default(),
        }
    }

    pub fn with_router(mut self, router: Router) -> Self {
        self.router = router;
        self
    }

    pub fn with_scheduler(mut self, scheduler: Scheduler) -> Self {
        self.scheduler = scheduler;
        self
    }

    pub fn with_state_store(mut self, store: impl StateStore + 'static) -> Self {
        self.state = Box::new(store);
        self
    }

    pub fn with_boxed_state_store(mut self, store: Box<dyn StateStore>) -> Self {
        self.state = store;
        self
    }

    pub fn with_config(mut self, config: BotConfig) -> Self {
        self.config = config;
        self
    }

    pub fn build(self) -> Bot {
        Bot {
            client: self.client,
            router: self.router,
            scheduler: self.scheduler,
            state: self.state,
            stop: Arc::new(AtomicBool::new(false)),
            config: self.config,
        }
    }
}

impl Bot {
    pub fn builder(client: Client) -> BotBuilder {
        BotBuilder::new(client)
    }

    pub fn stop_handle(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.stop)
    }

    pub fn request_stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.stop.load(Ordering::Relaxed) {
            if let Some((event, message)) = self.client.poll(self.config.poll_timeout_ms) {
                let outcome =
                    self.router
                        .dispatch(&self.client, event, &message, self.state.as_mut())?;
                if matches!(outcome, HandlerResult::Stop) {
                    self.request_stop();
                }
            }
            self.scheduler.tick(&self.client, self.state.as_mut())?;
        }
        Ok(())
    }

    pub fn router_mut(&mut self) -> &mut Router {
        &mut self.router
    }

    pub fn scheduler_mut(&mut self) -> &mut Scheduler {
        &mut self.scheduler
    }

    pub fn state_mut(&mut self) -> &mut dyn StateStore {
        self.state.as_mut()
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
