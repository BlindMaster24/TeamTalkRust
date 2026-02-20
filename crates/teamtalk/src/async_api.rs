//! Async wrapper around the polling client.
use crate::client::{Client, EventData, Message};
use crate::events::Event;
use futures::stream::Stream;
use futures::task::AtomicWaker;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::task::{Context, Poll};
use std::time::Duration;

use futures::StreamExt;
use std::thread::{self, JoinHandle};

/// Configuration for the async polling loop.
#[derive(Clone, Copy)]
pub struct AsyncConfig {
    pub poll_timeout_ms: i32,
    pub buffer: usize,
}

impl Default for AsyncConfig {
    fn default() -> Self {
        Self {
            poll_timeout_ms: 100,
            buffer: 512,
        }
    }
}

impl AsyncConfig {
    /// Creates a configuration with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the polling timeout in milliseconds.
    pub fn poll_timeout_ms(mut self, timeout_ms: i32) -> Self {
        self.poll_timeout_ms = timeout_ms;
        self
    }

    /// Sets the channel buffer size for events.
    pub fn buffer(mut self, buffer: usize) -> Self {
        self.buffer = buffer;
        self
    }
}

/// Async stream of client events backed by a worker thread.
///
/// # Threading
/// `AsyncClient` is `Send` but intentionally not `Sync`.
/// Keep it owned by one async task/runtime and use its wait/shutdown helpers
/// for coordination.
///
/// Migration guide: <https://github.com/BlindMaster24/TeamTalkRust/blob/main/docs/migrations/2-to-3.md>.
pub struct AsyncClient {
    client: Option<Arc<Client>>,
    stop: Arc<AtomicBool>,
    receiver: Option<Receiver<(Event, Message)>>,
    waker: Arc<AtomicWaker>,
    worker: Option<JoinHandle<()>>,
}

impl AsyncClient {
    /// Creates an async client with default configuration.
    pub fn new(client: Client) -> Self {
        Self::with_config(client, AsyncConfig::default())
    }

    /// Creates an async client with custom configuration.
    pub fn with_config(client: Client, config: AsyncConfig) -> Self {
        let buffer_cap = config.buffer.max(1);
        let poll_timeout_ms = config.poll_timeout_ms;
        let client = Arc::new(client);
        let stop = Arc::new(AtomicBool::new(false));
        let waker = Arc::new(AtomicWaker::new());
        let (sender, receiver) = mpsc::sync_channel(buffer_cap);
        let worker_stop = Arc::clone(&stop);
        let worker_client = Arc::clone(&client);
        let worker_waker = Arc::clone(&waker);
        let worker = Some(thread::spawn(move || {
            while !worker_stop.load(Ordering::Relaxed) {
                if let Some(item) = worker_client.poll(poll_timeout_ms) {
                    if sender.send(item).is_err() {
                        break;
                    }
                    worker_waker.wake();
                }
            }
        }));
        Self {
            client: Some(client),
            stop,
            receiver: Some(receiver),
            waker,
            worker,
        }
    }

    /// Runs a closure with a shared client reference.
    pub fn with_client<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Client) -> R,
    {
        let client = self.client.as_deref()?;
        Some(f(client))
    }

    /// Runs a closure with a mutable client reference.
    pub fn with_client_mut<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Client) -> R,
    {
        self.shutdown();
        let client = Arc::get_mut(self.client.as_mut()?)?;
        Some(f(client))
    }

    /// Stops the async polling loop.
    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    /// Stops the worker and waits until it exits.
    pub fn shutdown(&mut self) {
        self.stop();
        let _ = self.receiver.take();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }

    /// Stops the loop and returns the underlying client.
    pub fn into_client(mut self) -> Option<Client> {
        self.shutdown();
        Arc::try_unwrap(self.client.take()?).ok()
    }

    /// Waits for the next event from the stream.
    pub async fn next_event(&mut self) -> Option<(Event, Message)> {
        self.next().await
    }

    /// Waits until a specific event is received.
    pub async fn wait_for_event(&mut self, expected: Event) -> Option<Message> {
        while let Some((event, msg)) = self.next().await {
            if event == expected {
                return Some(msg);
            }
        }
        None
    }

    /// Waits until the provided predicate matches an event.
    pub async fn wait_for_predicate<F>(&mut self, mut predicate: F) -> Option<(Event, Message)>
    where
        F: FnMut(Event, &Message) -> bool,
    {
        while let Some((event, msg)) = self.next().await {
            if predicate(event, &msg) {
                return Some((event, msg));
            }
        }
        None
    }

    /// Waits for the next event with a typed payload.
    pub async fn wait_for_data(&mut self) -> Option<(Event, Message, EventData)> {
        while let Some((event, msg)) = self.next().await {
            if let Some(data) = msg.data() {
                return Some((event, msg, data));
            }
        }
        None
    }

    /// Waits for a specific event until timeout is reached.
    #[cfg(feature = "async-tokio")]
    pub async fn wait_for_event_timeout(
        &mut self,
        expected: Event,
        timeout: Duration,
    ) -> Option<Message> {
        tokio::time::timeout(timeout, self.wait_for_event(expected))
            .await
            .ok()
            .flatten()
    }

    /// Waits until the provided predicate matches an event or timeout is reached.
    #[cfg(feature = "async-tokio")]
    pub async fn wait_for_predicate_timeout<F>(
        &mut self,
        predicate: F,
        timeout: Duration,
    ) -> Option<(Event, Message)>
    where
        F: FnMut(Event, &Message) -> bool,
    {
        tokio::time::timeout(timeout, self.wait_for_predicate(predicate))
            .await
            .ok()
            .flatten()
    }

    /// Waits for the next typed payload event or timeout is reached.
    #[cfg(feature = "async-tokio")]
    pub async fn wait_for_data_timeout(
        &mut self,
        timeout: Duration,
    ) -> Option<(Event, Message, EventData)> {
        tokio::time::timeout(timeout, self.wait_for_data())
            .await
            .ok()
            .flatten()
    }
}

impl Stream for AsyncClient {
    type Item = (Event, Message);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if this.stop.load(Ordering::Relaxed) {
            return Poll::Ready(None);
        }
        let Some(receiver) = this.receiver.as_ref() else {
            return Poll::Ready(None);
        };
        match receiver.try_recv() {
            Ok(item) => Poll::Ready(Some(item)),
            Err(mpsc::TryRecvError::Disconnected) => Poll::Ready(None),
            Err(mpsc::TryRecvError::Empty) => {
                this.waker.register(cx.waker());
                match receiver.try_recv() {
                    Ok(item) => Poll::Ready(Some(item)),
                    Err(mpsc::TryRecvError::Disconnected) => Poll::Ready(None),
                    Err(mpsc::TryRecvError::Empty) => Poll::Pending,
                }
            }
        }
    }
}

impl Drop for AsyncClient {
    fn drop(&mut self) {
        self.shutdown();
    }
}

impl Client {
    /// Converts the client into an async event stream.
    pub fn into_async(self) -> AsyncClient {
        AsyncClient::new(self)
    }

    /// Converts the client into an async event stream with configuration.
    pub fn into_async_with_config(self, config: AsyncConfig) -> AsyncClient {
        AsyncClient::with_config(self, config)
    }
}
