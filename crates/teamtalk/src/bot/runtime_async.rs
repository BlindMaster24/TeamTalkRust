use super::router::HandlerResult;
use super::{MemoryStateStore, Router, Scheduler, StateStore};
use crate::async_api::AsyncClient;
use crate::events::Result;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Copy, Default)]
pub struct AsyncBotConfig;

pub struct AsyncBot {
    client: AsyncClient,
    router: Router,
    scheduler: Scheduler,
    state: Box<dyn StateStore>,
    stop: Arc<AtomicBool>,
    _config: AsyncBotConfig,
}

pub struct AsyncBotBuilder {
    client: AsyncClient,
    router: Router,
    scheduler: Scheduler,
    state: Box<dyn StateStore>,
    config: AsyncBotConfig,
}

impl AsyncBotBuilder {
    pub fn new(client: AsyncClient) -> Self {
        Self {
            client,
            router: Router::new(),
            scheduler: Scheduler::new(),
            state: Box::new(MemoryStateStore::new()),
            config: AsyncBotConfig,
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

    pub fn with_config(mut self, config: AsyncBotConfig) -> Self {
        self.config = config;
        self
    }

    pub fn build(self) -> AsyncBot {
        AsyncBot {
            client: self.client,
            router: self.router,
            scheduler: self.scheduler,
            state: self.state,
            stop: Arc::new(AtomicBool::new(false)),
            _config: self.config,
        }
    }
}

impl AsyncBot {
    pub fn builder(client: AsyncClient) -> AsyncBotBuilder {
        AsyncBotBuilder::new(client)
    }

    pub fn stop_handle(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.stop)
    }

    pub fn request_stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    pub async fn run(&mut self) -> Result<()> {
        while !self.stop.load(Ordering::Relaxed) {
            let next = self.client.next_event().await;
            let Some((event, message)) = next else {
                break;
            };

            let outcome = self
                .client
                .with_client(|client| {
                    self.router
                        .dispatch(client, event, &message, self.state.as_mut())
                })
                .unwrap_or(Ok(HandlerResult::Continue))?;

            if matches!(outcome, HandlerResult::Stop) {
                self.request_stop();
            }

            if let Some(result) = self
                .client
                .with_client(|client| self.scheduler.tick(client, self.state.as_mut()))
            {
                result?;
            }
        }

        Ok(())
    }

    pub fn client_mut(&mut self) -> &mut AsyncClient {
        &mut self.client
    }
}
