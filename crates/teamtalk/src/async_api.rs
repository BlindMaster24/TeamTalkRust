//! Async wrapper around the polling client.
use crate::client::{Client, Message};
use crate::events::Event;
use futures::SinkExt;
use futures::channel::mpsc::{Receiver, Sender, channel};
use futures::executor::block_on;
use futures::stream::Stream;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;

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
pub struct AsyncClient {
    client: Option<Arc<Mutex<Client>>>,
    receiver: Receiver<(Event, Message)>,
    stop: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl AsyncClient {
    /// Creates an async client with default configuration.
    pub fn new(client: Client) -> Self {
        Self::with_config(client, AsyncConfig::default())
    }

    /// Creates an async client with custom configuration.
    pub fn with_config(client: Client, config: AsyncConfig) -> Self {
        let buffer = config.buffer.max(1);
        let (sender, receiver) = channel(buffer);
        let client = Arc::new(Mutex::new(client));
        let stop = Arc::new(AtomicBool::new(false));
        let thread_client = Arc::clone(&client);
        let thread_stop = Arc::clone(&stop);
        let handle = thread::spawn(move || {
            run_event_loop(thread_client, thread_stop, sender, config.poll_timeout_ms);
        });
        Self {
            client: Some(client),
            receiver,
            stop,
            handle: Some(handle),
        }
    }

    /// Runs a closure with a shared client reference.
    pub fn with_client<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Client) -> R,
    {
        let client = self.client.as_ref()?;
        let guard = client.lock().ok()?;
        Some(f(&guard))
    }

    /// Runs a closure with a mutable client reference.
    pub fn with_client_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Client) -> R,
    {
        let client = self.client.as_ref()?;
        let mut guard = client.lock().ok()?;
        Some(f(&mut guard))
    }

    /// Stops the async polling loop.
    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        self.receiver.close();
    }

    /// Stops the loop and returns the underlying client.
    pub fn into_client(mut self) -> Option<Client> {
        self.stop();
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        let client = self.client.take()?;
        Arc::try_unwrap(client)
            .ok()
            .and_then(|mutex| mutex.into_inner().ok())
    }
}

impl Stream for AsyncClient {
    type Item = (Event, Message);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        Pin::new(&mut this.receiver).poll_next(cx)
    }
}

impl Drop for AsyncClient {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        self.receiver.close();
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn run_event_loop(
    client: Arc<Mutex<Client>>,
    stop: Arc<AtomicBool>,
    mut sender: Sender<(Event, Message)>,
    poll_timeout_ms: i32,
) {
    while !stop.load(Ordering::Relaxed) {
        let event = {
            let guard = match client.lock() {
                Ok(guard) => guard,
                Err(_) => break,
            };
            guard.poll(poll_timeout_ms)
        };
        if let Some((event, message)) = event
            && block_on(sender.send((event, message))).is_err()
        {
            break;
        }
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
