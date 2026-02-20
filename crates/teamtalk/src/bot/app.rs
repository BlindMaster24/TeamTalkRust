use super::{Bot, BotBuilder, BotConfig, MemoryStateStore, Router, Scheduler, StateStore};
use crate::client::Client;
use crate::events::Result;

/// High-level bot entrypoint that wires router/scheduler/state into a runtime.
///
/// This struct helps build one bot definition and run it with a sync or async
/// runtime without repeating the same wiring code.
pub struct BotApp {
    router: Router,
    scheduler: Scheduler,
    state: Box<dyn StateStore>,
    config: BotConfig,
}

impl Default for BotApp {
    fn default() -> Self {
        Self {
            router: Router::new(),
            scheduler: Scheduler::new(),
            state: Box::new(MemoryStateStore::new()),
            config: BotConfig::default(),
        }
    }
}

impl BotApp {
    pub fn new() -> Self {
        Self::default()
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

    pub fn with_config(mut self, config: BotConfig) -> Self {
        self.config = config;
        self
    }

    /// Builds a sync bot runtime.
    pub fn into_bot(self, client: Client) -> Bot {
        BotBuilder::new(client)
            .with_router(self.router)
            .with_scheduler(self.scheduler)
            .with_boxed_state_store(self.state)
            .with_config(self.config)
            .build()
    }

    /// Builds and runs a sync bot runtime.
    pub fn run_sync(self, client: Client) -> Result<()> {
        let mut bot = self.into_bot(client);
        bot.run()
    }

    #[cfg(feature = "async")]
    pub fn into_async_bot(self, client: crate::async_api::AsyncClient) -> super::AsyncBot {
        super::AsyncBotBuilder::new(client)
            .with_router(self.router)
            .with_scheduler(self.scheduler)
            .with_boxed_state_store(self.state)
            .with_config(super::AsyncBotConfig)
            .build()
    }

    #[cfg(feature = "async")]
    pub async fn run_async(self, client: crate::async_api::AsyncClient) -> Result<()> {
        let mut bot = self.into_async_bot(client);
        bot.run().await
    }
}
