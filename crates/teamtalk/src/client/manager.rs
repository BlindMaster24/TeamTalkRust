use crate::events::Event;
use crate::types::ClientId;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant, SystemTime};

#[derive(Debug, Clone)]
pub struct ClientEvent {
    pub client_id: ClientId,
    pub label: Option<String>,
    pub event: Event,
    pub at: SystemTime,
}

#[derive(Debug, Clone, Default)]
pub struct ClientHealth {
    pub last_event: Option<Event>,
    pub last_event_at: Option<SystemTime>,
    pub last_poll_at: Option<SystemTime>,
}

pub struct ClientManager {
    clients: Vec<crate::client::Client>,
    health: HashMap<ClientId, ClientHealth>,
    queue: VecDeque<usize>,
    poll_timeout_ms: i32,
    tick_sleep: Duration,
    tx: Sender<ClientEvent>,
    rx: Receiver<ClientEvent>,
}

impl Default for ClientManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            clients: Vec::new(),
            health: std::collections::HashMap::new(),
            queue: VecDeque::new(),
            poll_timeout_ms: 0,
            tick_sleep: Duration::from_millis(1),
            tx,
            rx,
        }
    }

    pub fn add_client(&mut self, client: crate::client::Client) {
        let id = client.id();
        self.health.insert(id, ClientHealth::default());
        self.queue.push_back(self.clients.len());
        self.clients.push(client);
    }

    pub fn remove_client(&mut self, id: ClientId) {
        if let Some(pos) = self.clients.iter().position(|c| c.id() == id) {
            self.clients.remove(pos);
            self.health.remove(&id);
            self.queue = self
                .queue
                .iter()
                .filter_map(|&idx| {
                    if idx == pos {
                        None
                    } else if idx > pos {
                        Some(idx - 1)
                    } else {
                        Some(idx)
                    }
                })
                .collect();
        }
    }

    pub fn set_poll_timeout(&mut self, timeout_ms: i32) {
        self.poll_timeout_ms = timeout_ms;
    }

    pub fn set_tick_sleep(&mut self, sleep: Duration) {
        self.tick_sleep = sleep;
    }

    pub fn events(&self) -> &Receiver<ClientEvent> {
        &self.rx
    }

    pub fn health_snapshot(&self, id: ClientId) -> Option<ClientHealth> {
        self.health.get(&id).cloned()
    }

    pub fn run_once(&mut self) {
        let now = SystemTime::now();
        let mut processed = 0;
        let queue_len = self.queue.len();
        for _ in 0..queue_len {
            if let Some(idx) = self.queue.pop_front() {
                if let Some(client) = self.clients.get(idx) {
                    if let Some((event, _)) = client.poll(self.poll_timeout_ms) {
                        let evt = ClientEvent {
                            client_id: client.id(),
                            label: client.label(),
                            event,
                            at: now,
                        };
                        let _ = self.tx.send(evt.clone());
                        let entry = self.health.entry(client.id()).or_default();
                        entry.last_event = Some(event);
                        entry.last_event_at = Some(now);
                    }
                    let entry = self.health.entry(client.id()).or_default();
                    entry.last_poll_at = Some(now);
                }
                self.queue.push_back(idx);
                processed += 1;
            }
        }
        if processed == 0 {
            std::thread::sleep(self.tick_sleep);
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            let start = Instant::now();
            self.run_once();
            if self.tick_sleep > Duration::ZERO {
                let elapsed = start.elapsed();
                if elapsed < self.tick_sleep {
                    std::thread::sleep(self.tick_sleep - elapsed);
                }
            }
        }
    }
}
