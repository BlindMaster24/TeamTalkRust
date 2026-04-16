use super::{Bot, BotBuilder, BotConfig, MemoryStateStore, Router, Scheduler, StateStore};
use crate::client::Client;
use crate::events::Result;

#[cfg(feature = "async")]
use super::AsyncScheduler;

pub struct BotApp {
    router: Router,
    scheduler: Scheduler,
    #[cfg(feature = "async")]
    async_scheduler: AsyncScheduler,
    state: Box<dyn StateStore>,
    config: BotConfig,
}

impl Default for BotApp {
    fn default() -> Self {
        Self {
            router: Router::new(),
            scheduler: Scheduler::new(),
            #[cfg(feature = "async")]
            async_scheduler: AsyncScheduler::new(),
            state: Box::new(MemoryStateStore::new()),
            config: BotConfig::default(),
        }
    }
}

impl BotApp {
    #[allow(clippy::must_use_candidate)]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_router(mut self, router: Router) -> Self {
        self.router = router;
        self
    }

    #[must_use]
    pub fn with_scheduler(mut self, scheduler: Scheduler) -> Self {
        self.scheduler = scheduler;
        self
    }

    #[cfg(feature = "async")]
    #[must_use]
    pub fn with_async_scheduler(mut self, scheduler: AsyncScheduler) -> Self {
        self.async_scheduler = scheduler;
        self
    }

    #[must_use]
    pub fn with_state_store(mut self, store: impl StateStore + 'static) -> Self {
        self.state = Box::new(store);
        self
    }

    #[must_use]
    pub fn with_config(mut self, config: BotConfig) -> Self {
        self.config = config;
        self
    }

    pub fn into_bot(self, client: Client) -> Bot {
        BotBuilder::new(client)
            .with_router(self.router)
            .with_scheduler(self.scheduler)
            .with_boxed_state_store(self.state)
            .with_config(self.config)
            .build()
    }

    pub fn run_sync(self, client: Client) -> Result<()> {
        let mut bot = self.into_bot(client);
        bot.run()
    }

    #[cfg(feature = "async")]
    #[must_use]
    pub fn into_async_bot(self, client: crate::async_api::AsyncClient) -> super::AsyncBot {
        super::AsyncBotBuilder::new(client)
            .with_router(self.router)
            .with_scheduler(self.async_scheduler)
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
