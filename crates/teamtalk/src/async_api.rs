//! Async wrapper around the polling client.
use crate::client::{Client, Message};
use crate::events::Event;
use futures::stream::Stream;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};
use std::thread;

#[cfg(feature = "async-tokio")]
use tokio::sync::mpsc;

#[cfg(not(feature = "async-tokio"))]
use futures::{SinkExt, channel::mpsc};

/// Configuration for the async polling loop.
#[derive(Clone, Copy)]
pub struct AsyncConfig {
    pub poll_timeout_ms: i32,
    pub buffer: usize,
}

impl Default for AsyncConfig {
    fn default() -> Self {
        Self {
            poll_timeout_ms: 10,
            buffer: 1024,
        }
    }
}

impl AsyncConfig {
    /// Creates a configuration with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the polling timeout in milliseconds for the background thread.
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

/// Async stream of client events backed by a dedicated worker thread.
pub struct AsyncClient {
    client: Arc<Client>,
    stop: Arc<AtomicBool>,
    #[cfg(feature = "async-tokio")]
    receiver: mpsc::Receiver<(Event, Message)>,
    #[cfg(not(feature = "async-tokio"))]
    receiver: mpsc::Receiver<(Event, Message)>,
    worker: Option<thread::JoinHandle<()>>,
}

impl AsyncClient {
    /// Creates an async client with default configuration.
    pub fn new(client: Client) -> Self {
        Self::with_config(client, AsyncConfig::default())
    }

    /// Creates an async client with custom configuration.
    pub fn with_config(client: Client, config: AsyncConfig) -> Self {
        let client = Arc::new(client);
        let stop = Arc::new(AtomicBool::new(false));

        #[cfg(feature = "async-tokio")]
        let (sender, receiver) = mpsc::channel(config.buffer);

        #[cfg(not(feature = "async-tokio"))]
        let (mut sender, receiver) = mpsc::channel(config.buffer);

        let worker_client = Arc::clone(&client);
        let worker_stop = Arc::clone(&stop);
        let poll_timeout = config.poll_timeout_ms;

        let worker = thread::spawn(move || {
            while !worker_stop.load(Ordering::Relaxed) {
                if let Some((event, message)) = worker_client.poll(poll_timeout) {
                    #[cfg(feature = "async-tokio")]
                    {
                        // blocking_send handles backpressure by waiting for space in the channel.
                        // It only fails if the receiver is dropped.
                        if sender.blocking_send((event, message)).is_err() {
                            break;
                        }
                    }
                    #[cfg(not(feature = "async-tokio"))]
                    {
                        // Use block_on to wait for space in the futures mpsc channel.
                        if futures::executor::block_on(sender.send((event, message))).is_err() {
                            break;
                        }
                    }
                }
            }
            // Ensure the worker knows it's finished
            worker_stop.store(true, Ordering::Relaxed);
        });

        Self {
            client,
            stop,
            receiver,
            worker: Some(worker),
        }
    }

    /// Returns a reference to the underlying client.
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Accessor for user management operations.
    pub fn users(&self) -> crate::client::namespaces::AsyncUsersNamespace {
        crate::client::namespaces::AsyncUsersNamespace::new(self.client.clone())
    }

    /// Accessor for channel management operations.
    pub fn channels(&self) -> crate::client::namespaces::AsyncChannelsNamespace {
        crate::client::namespaces::AsyncChannelsNamespace::new(self.client.clone())
    }

    /// Accessor for audio operations.
    pub fn audio(&self) -> crate::client::namespaces::AsyncAudioNamespace {
        crate::client::namespaces::AsyncAudioNamespace::new(self.client.clone())
    }

    /// Accessor for server operations.
    pub fn server(&self) -> crate::client::namespaces::AsyncServerNamespace {
        crate::client::namespaces::AsyncServerNamespace::new(self.client.clone())
    }

    /// Accessor for file transfer operations.
    pub fn files(&self) -> crate::client::namespaces::AsyncFilesNamespace {
        crate::client::namespaces::AsyncFilesNamespace::new(self.client.clone())
    }

    /// Accessor for desktop sharing operations.
    pub fn desktop(&self) -> crate::client::namespaces::AsyncDesktopNamespace {
        crate::client::namespaces::AsyncDesktopNamespace::new(self.client.clone())
    }

    /// Accessor for video operations.
    pub fn video(&self) -> crate::client::namespaces::AsyncVideoNamespace {
        crate::client::namespaces::AsyncVideoNamespace::new(self.client.clone())
    }

    /// Accessor for system and licensing operations.
    pub fn system(&self) -> crate::client::namespaces::AsyncSystemNamespace {
        crate::client::namespaces::AsyncSystemNamespace::new(self.client.clone())
    }

    /// Returns true if the background worker thread is still running.
    pub fn is_running(&self) -> bool {
        !self.stop.load(Ordering::Relaxed)
    }

    /// Stops the async polling loop and worker thread.
    ///
    /// This method is idempotent. It will block until the worker thread has finished.
    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }

        // Clear the receiver to drop any pending messages and signals
        #[cfg(feature = "async-tokio")]
        self.receiver.close();
        #[cfg(not(feature = "async-tokio"))]
        self.receiver.close();
    }

    /// Registers a one-time notification for a specific event.
    ///
    /// Returns a Future that resolves when the event occurs. This is useful for
    /// "wait for X after I do Y" patterns where you need to subscribe *before*
    /// performing the action to avoid race conditions.
    pub fn notify_on(
        &self,
        event: Event,
    ) -> impl std::future::Future<Output = crate::events::Result<Message>> {
        self.client.notify_on(event)
    }

    /// Waits for a specific event to occur.
    pub async fn wait_for(&self, event: Event) -> crate::events::Result<Message> {
        self.client.wait_for_async(event).await
    }

    /// Waits for a specific event and extracts its typed data.
    pub async fn wait_for_data<T: crate::events::FromMessage>(
        &self,
        event: Event,
    ) -> crate::events::Result<T> {
        self.client.wait_for_data_async(event).await
    }

    /// Executes a command and waits for a success event OR a command error with matching ID.
    pub async fn execute_command<F, T>(
        &self,
        success_event: Event,
        f: F,
    ) -> crate::events::Result<T>
    where
        F: FnOnce() -> i32,
        T: crate::events::FromMessage + 'static,
    {
        self.client.execute_command(success_event, f).await
    }
}

impl Stream for AsyncClient {
    type Item = (Event, Message);

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        #[cfg(feature = "async-tokio")]
        {
            self.receiver.poll_recv(cx)
        }

        #[cfg(not(feature = "async-tokio"))]
        {
            use futures::stream::StreamExt;
            self.receiver.poll_next_unpin(cx)
        }
    }
}

impl Drop for AsyncClient {
    fn drop(&mut self) {
        self.stop();
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
