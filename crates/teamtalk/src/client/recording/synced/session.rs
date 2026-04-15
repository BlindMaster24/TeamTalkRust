use super::writer::{AudioBlockGuard, UserTrack};
use super::*;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum SilencePolicy {
    Always,
    OnlyWhileConnected,
    OnlyWhileTalking,
}

#[derive(Clone, Debug)]
pub struct SyncedUserRecordingOptions {
    pub folder: String,
    pub file_vars: String,
    pub format: RecordingSampleFormat,
    pub stream_types: u32,
    pub tick_interval: Duration,
    pub subscribe_audio: bool,
    pub default_sample_rate: Option<i32>,
    pub default_channels: Option<i32>,
    pub silence_policy: SilencePolicy,
}

impl SyncedUserRecordingOptions {
    pub fn new(folder: impl Into<String>) -> Self {
        Self {
            folder: folder.into(),
            file_vars: "user-%user_id%-%username%".to_string(),
            format: RecordingSampleFormat::PcmS16Le,
            stream_types: ffi::StreamType::STREAMTYPE_VOICE as u32,
            tick_interval: Duration::from_millis(250),
            subscribe_audio: true,
            default_sample_rate: None,
            default_channels: None,
            silence_policy: SilencePolicy::Always,
        }
    }

    pub fn with_format(mut self, format: RecordingSampleFormat) -> Self {
        self.format = format;
        self
    }

    pub fn with_file_vars(mut self, vars: impl Into<String>) -> Self {
        self.file_vars = vars.into();
        self
    }

    pub fn with_stream_types(mut self, types: u32) -> Self {
        self.stream_types = types;
        self
    }

    pub fn with_tick_interval(mut self, interval: Duration) -> Self {
        self.tick_interval = interval;
        self
    }

    pub fn with_default_audio_format(mut self, sample_rate: i32, channels: i32) -> Self {
        self.default_sample_rate = Some(sample_rate);
        self.default_channels = Some(channels);
        self
    }

    pub fn with_subscribe_audio(mut self, enabled: bool) -> Self {
        self.subscribe_audio = enabled;
        self
    }

    pub fn with_silence_policy(mut self, policy: SilencePolicy) -> Self {
        self.silence_policy = policy;
        self
    }
}

pub struct SyncedUserRecordingSession {
    options: SyncedUserRecordingOptions,
    start: Instant,
    users: HashMap<UserId, UserTrack>,
    last_tick: Instant,
    connected: bool,
}

impl SyncedUserRecordingSession {
    pub fn start(client: &Client, options: SyncedUserRecordingOptions) -> Result<Self> {
        fs::create_dir_all(&options.folder).map_err(|e| Error::IoError {
            message: e.to_string(),
        })?;

        let mut session = Self {
            options,
            start: Instant::now(),
            users: HashMap::new(),
            last_tick: Instant::now(),
            connected: client.is_connected(),
        };

        session.attach_existing_users(client)?;
        Ok(session)
    }

    pub fn tick(&mut self) -> Result<()> {
        if self.options.tick_interval > Duration::ZERO
            && self.last_tick.elapsed() < self.options.tick_interval
        {
            return Ok(());
        }
        self.last_tick = Instant::now();
        if matches!(self.options.silence_policy, SilencePolicy::OnlyWhileTalking) {
            return Ok(());
        }
        if matches!(
            self.options.silence_policy,
            SilencePolicy::OnlyWhileConnected
        ) && !self.connected
        {
            return Ok(());
        }
        let elapsed = self.start.elapsed();
        for track in self.users.values_mut() {
            track.pad_to(elapsed)?;
        }
        Ok(())
    }

    pub fn handle_event(&mut self, client: &Client, event: Event, message: &Message) -> Result<()> {
        match event {
            Event::UserJoined => {
                if let Some(user) = message.user() {
                    self.connected = true;
                    self.start_user(client, user.id, Some(user))?;
                }
            }
            Event::UserLeft => {
                let user_id = message
                    .user()
                    .map(|u| u.id)
                    .unwrap_or(UserId(message.source()));
                if user_id.0 > 0 {
                    self.stop_user(client, user_id);
                }
            }
            Event::AudioBlock => {
                let user_id = UserId(message.source());
                if user_id.0 > 0 {
                    self.connected = true;
                    self.on_audio_block(client, user_id)?;
                }
            }
            Event::ConnectSuccess => {
                self.connected = true;
            }
            Event::ConnectionLost | Event::ConnectFailed | Event::ConnectCryptError => {
                self.connected = false;
                self.stop_all(client);
            }
            _ => {}
        }
        Ok(())
    }

    pub fn attach_existing_users(&mut self, client: &Client) -> Result<()> {
        let channel_id = client.my_channel_id();
        let users = client.get_channel_users(channel_id);
        for user in users {
            self.start_user(client, user.id, Some(user))?;
        }
        Ok(())
    }

    fn start_user(&mut self, client: &Client, user_id: UserId, user: Option<User>) -> Result<()> {
        if self.users.contains_key(&user_id) {
            return Ok(());
        }

        if should_warn_missing_audio_subscriptions(self.options.subscribe_audio, user.as_ref()) {
            #[cfg(feature = "logging")]
            tracing::warn!(
                user_id = user_id.0,
                "synced recording started with subscribe_audio=false and no local audio subscriptions"
            );
        }

        let track = UserTrack::new(
            &self.options.folder,
            &self.options.file_vars,
            self.options.format.clone(),
            user_id,
            user,
            self.options.default_sample_rate,
            self.options.default_channels,
        )?;
        self.users.insert(user_id, track);

        if self.options.subscribe_audio {
            let _ = client.subscribe(user_id, synced_audio_subscription_mask());
        }
        client.enable_audio_block_event(user_id, self.options.stream_types, true);

        if let Err(err) = self.drain_pending_blocks(client, user_id) {
            self.stop_user(client, user_id);
            return Err(err);
        }
        Ok(())
    }

    fn stop_user(&mut self, client: &Client, user_id: UserId) {
        client.enable_audio_block_event(user_id, self.options.stream_types, false);
        if self.options.subscribe_audio {
            let _ = client.unsubscribe(user_id, synced_audio_subscription_mask());
        }
        self.users.remove(&user_id);
    }

    pub fn stop_all(&mut self, client: &Client) {
        let user_ids: Vec<UserId> = self.users.keys().copied().collect();
        for user_id in user_ids {
            self.stop_user(client, user_id);
        }
    }

    fn on_audio_block(&mut self, client: &Client, user_id: UserId) -> Result<()> {
        let Some(ptr) = client.acquire_user_audio_block(self.options.stream_types, user_id) else {
            return Ok(());
        };
        let guard = AudioBlockGuard::new(client, ptr);
        let block = unsafe { &*guard.ptr() };
        let Some(view) = AudioBlockView::from_block(block) else {
            return Ok(());
        };

        if !self.users.contains_key(&user_id) {
            self.start_user(client, user_id, None)?;
        }

        let elapsed = self.start.elapsed();
        if let Some(track) = self.users.get_mut(&user_id) {
            track.ensure_format(view.sample_rate, view.channels)?;
            track.pad_to(elapsed)?;
            track.write_block(&view)?;
        }
        Ok(())
    }

    fn drain_pending_blocks(&mut self, client: &Client, user_id: UserId) -> Result<()> {
        loop {
            let Some(ptr) = client.acquire_user_audio_block(self.options.stream_types, user_id)
            else {
                break;
            };
            let guard = AudioBlockGuard::new(client, ptr);
            let block = unsafe { &*guard.ptr() };
            let Some(view) = AudioBlockView::from_block(block) else {
                continue;
            };

            let elapsed = self.start.elapsed();
            if let Some(track) = self.users.get_mut(&user_id) {
                track.ensure_format(view.sample_rate, view.channels)?;
                track.pad_to(elapsed)?;
                track.write_block(&view)?;
            }
        }
        Ok(())
    }
}

pub struct SyncedUserRecording {
    session: SyncedUserRecordingSession,
}

impl SyncedUserRecording {
    pub fn start(client: &Client, options: SyncedUserRecordingOptions) -> Result<Self> {
        Ok(Self {
            session: SyncedUserRecordingSession::start(client, options)?,
        })
    }

    pub fn tick(&mut self) -> Result<()> {
        self.session.tick()
    }

    pub fn handle_event(&mut self, client: &Client, event: Event, message: &Message) -> Result<()> {
        self.session.handle_event(client, event, message)
    }

    pub fn stop_all(&mut self, client: &Client) {
        self.session.stop_all(client);
    }
}

pub struct SyncedUserRecordingBus<'a> {
    client: &'a Client,
    group: String,
    session: Arc<UnpoisonedMutex<SyncedUserRecordingSession>>,
    stop_on_drop: bool,
}

impl<'a> SyncedUserRecordingBus<'a> {
    pub fn attach(
        session: Arc<UnpoisonedMutex<SyncedUserRecordingSession>>,
        client: &'a Client,
        group: impl Into<String>,
    ) -> Self {
        let group_name = group.into();
        let group_filter = group_name.clone();
        let handler_session = Arc::clone(&session);
        let _id = client
            .on_any()
            .group(group_filter)
            .filter(|ctx| is_synced_bus_event(ctx.event()))
            .subscribe(move |ctx| {
                let mut session = handler_session.lock();
                let _ = session.handle_event(ctx.client(), ctx.event(), ctx.message());
            });
        Self {
            client,
            group: group_name,
            session,
            stop_on_drop: false,
        }
    }

    pub fn stop_on_drop(mut self, stop: bool) -> Self {
        self.stop_on_drop = stop;
        self
    }
}

impl Drop for SyncedUserRecordingBus<'_> {
    fn drop(&mut self) {
        if self.stop_on_drop {
            self.session.lock().stop_all(self.client);
        }
        let _ = self.client.unsubscribe_event_group(&self.group);
    }
}
