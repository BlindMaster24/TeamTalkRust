use super::router::HandlerResult;
use super::{AsyncScheduler, MemoryStateStore, Router, StateStore};
use crate::async_api::AsyncClient;
use crate::client::{EventData, Message};
use crate::events::{Event, Result};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Copy, Default)]
pub struct AsyncBotConfig;

pub struct AsyncBot {
    client: AsyncClient,
    router: Router,
    scheduler: AsyncScheduler,
    state: Box<dyn StateStore>,
    stop: Arc<AtomicBool>,
    _config: AsyncBotConfig,
}

pub struct AsyncBotBuilder {
    client: AsyncClient,
    router: Router,
    scheduler: AsyncScheduler,
    state: Box<dyn StateStore>,
    config: AsyncBotConfig,
}

impl AsyncBotBuilder {
    #[allow(clippy::must_use_candidate)]
    pub fn new(client: AsyncClient) -> Self {
        Self {
            client,
            router: Router::new(),
            scheduler: AsyncScheduler::new(),
            state: Box::new(MemoryStateStore::new()),
            config: AsyncBotConfig,
        }
    }

    #[must_use]
    pub fn with_router(mut self, router: Router) -> Self {
        self.router = router;
        self
    }

    #[must_use]
    pub fn with_scheduler(mut self, scheduler: AsyncScheduler) -> Self {
        self.scheduler = scheduler;
        self
    }

    #[must_use]
    pub fn with_state_store(mut self, store: impl StateStore + 'static) -> Self {
        self.state = Box::new(store);
        self
    }

    #[must_use]
    pub fn with_boxed_state_store(mut self, store: Box<dyn StateStore>) -> Self {
        self.state = store;
        self
    }

    #[must_use]
    pub fn with_config(mut self, config: AsyncBotConfig) -> Self {
        self.config = config;
        self
    }

    #[must_use]
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
    #[must_use]
    pub fn builder(client: AsyncClient) -> AsyncBotBuilder {
        AsyncBotBuilder::new(client)
    }

    #[must_use]
    pub fn stop_handle(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.stop)
    }

    pub fn request_stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    #[cfg(feature = "async-tokio")]
    pub async fn run(&mut self) -> Result<()> {
        while !self.stop.load(Ordering::Relaxed) {
            let next_job_delay = self.scheduler.next_run_delay();
            tokio::select! {
                next = self.client.next_event() => {
                    let Some((event, message)) = next else {
                        break;
                    };
                    let outcome = self
                        .client
                        .with_client(|client| {
                            self.router.dispatch(client, event, &message, self.state.as_mut())
                        })
                        .unwrap_or(Ok(HandlerResult::Continue))?;
                    if matches!(outcome, HandlerResult::Stop) {
                        self.request_stop();
                    }
                }
                _ = tokio::time::sleep(next_job_delay) => {
                    if let Some(result) = self
                        .client
                        .with_client(|client| self.scheduler.tick(client, self.state.as_mut()))
                    {
                        result?;
                    }
                }
            }
        }
        Ok(())
    }

    #[cfg(all(feature = "async", not(feature = "async-tokio")))]
    pub async fn run(&mut self) -> Result<()> {
        while !self.stop.load(Ordering::Relaxed) {
            let next_job_delay = self.scheduler.next_run_delay();
            let outcome = next_event_or_delay(&mut self.client, next_job_delay).await;
            match outcome {
                NextEventOrDelay::Event(next) => {
                    let Some((event, message)) = *next else {
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
                }
                NextEventOrDelay::Delay => {
                    if let Some(result) = self
                        .client
                        .with_client(|client| self.scheduler.tick(client, self.state.as_mut()))
                    {
                        result?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn client_mut(&mut self) -> &mut AsyncClient {
        &mut self.client
    }

    pub async fn wait_for_event(&mut self, event: Event) -> Option<Message> {
        self.client.wait_for_event(event).await
    }

    pub async fn wait_for_predicate<F>(&mut self, predicate: F) -> Option<(Event, Message)>
    where
        F: FnMut(Event, &Message) -> bool,
    {
        self.client.wait_for_predicate(predicate).await
    }

    pub async fn wait_for_data(&mut self) -> Option<(Event, Message, EventData)> {
        self.client.wait_for_data().await
    }

    #[cfg(feature = "async-tokio")]
    pub async fn wait_for_event_timeout(
        &mut self,
        event: Event,
        timeout: std::time::Duration,
    ) -> Option<Message> {
        self.client.wait_for_event_timeout(event, timeout).await
    }

    #[cfg(feature = "async-tokio")]
    pub async fn wait_for_data_timeout(
        &mut self,
        timeout: std::time::Duration,
    ) -> Option<(Event, Message, EventData)> {
        self.client.wait_for_data_timeout(timeout).await
    }
}

#[cfg(all(feature = "async", not(feature = "async-tokio")))]
enum NextEventOrDelay {
    Event(Box<Option<(Event, Message)>>),
    Delay,
}

#[cfg(all(feature = "async", not(feature = "async-tokio")))]
async fn next_event_or_delay(
    client: &mut AsyncClient,
    delay: std::time::Duration,
) -> NextEventOrDelay {
    use futures::future::{Either, FutureExt, select};

    let event_fut = client.next_event().boxed();
    let delay_fut = futures_timer::Delay::new(delay);
    match select(event_fut, delay_fut).await {
        Either::Left((next, _remaining_delay)) => NextEventOrDelay::Event(Box::new(next)),
        Either::Right(((), _remaining_event)) => NextEventOrDelay::Delay,
    }
}
