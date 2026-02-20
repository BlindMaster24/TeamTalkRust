#![doc = include_str!("../README.md")]
use std::path::Path;
use utils::ToTT;

#[cfg(feature = "bot")]
pub mod bot;
pub mod client;
pub mod events;
pub mod extensions;
pub mod loader;
pub mod types;
pub mod utils;

#[cfg(feature = "async")]
pub mod async_api;
#[cfg(feature = "dispatch")]
pub mod dispatch;
#[cfg(feature = "logging")]
pub mod logging;
#[cfg(feature = "mock")]
pub mod mock;

#[cfg(feature = "async")]
pub use async_api::{AsyncClient, AsyncConfig};
#[cfg(all(feature = "bot", feature = "bot-redis"))]
pub use bot::RedisStateStore;
#[cfg(all(feature = "bot", feature = "bot-sqlite"))]
pub use bot::SqliteStateStore;
#[cfg(feature = "bot")]
pub use bot::{
    Args, Bot, BotApp, BotBuilder, BotConfig, Command, CommandOnly, Context, DialogFlow,
    DialogMachine, DialogState, HandlerResult, JobErrorPolicy, MemoryStateStore, Middleware,
    RateLimitBySource, RouteMatcher, Router, Scheduler, StateStore, UnknownCommandPolicy,
    parse_command,
};
#[cfg(all(feature = "bot", feature = "async"))]
pub use bot::{AsyncBot, AsyncBotBuilder, AsyncBotConfig};
pub use client::audio::AudioDeviceProfile;
pub use client::audio::{
    AudioBlockSink, AudioBlockSubscription, AudioBlockView, CallbackSink, UdpSink, WriterSink,
};
pub use client::recording::{
    RecordSession, RecordingOptions, RecordingSampleFormat, RecordingSession, RecordingTarget,
    SilencePolicy, SyncedUserRecording, SyncedUserRecordingBus, SyncedUserRecordingOptions,
    SyncedUserRecordingSession, UserRecordingOptions, UserRecordingSession,
};
pub use client::users::{LoginParams, SendTextOptions};
pub use client::{
    Client, ClientCommands, ClientEvent, ClientEvents, ClientHealth, ClientHooks, ClientInfo,
    ClientManager, ClientRegistry, EventContext, EventData, EventSubscriptionId, Message,
    ReconnectConfig,
};
#[cfg(feature = "state")]
pub use client::{ServerInfo, StoreSnapshot};
#[cfg(feature = "dispatch")]
pub use dispatch::{
    ClientConfig, ConnectParamsOwned, DispatchFlow, Dispatcher,
    EventContext as DispatchEventContext, ReconnectSettings,
};
pub use events::{ConnectionState, Error, Event, Result};
#[cfg(feature = "mock")]
pub use mock::{MockClient, MockMessage, MockUserBuilder};
#[cfg(feature = "bot-macros")]
pub use teamtalk_macros::{teamtalk_command, teamtalk_event};
pub use types::{ClientId, MessageBuilder};

/// Initializes the TeamTalk SDK by loading the runtime DLL from the default location.
pub fn init() -> Result<()> {
    let dll_path = loader::find_or_download_dll().map_err(|_| Error::InitFailed)?;
    let dll_path = dll_path.to_str().ok_or(Error::InitFailed)?;
    teamtalk_sys::load(dll_path).map_err(|_| Error::InitFailed)?;
    Ok(())
}

/// Sets TeamTalk license information before creating any `Client`.
///
/// TeamTalk C-API requires license information to be configured before
/// `TT_InitTeamTalk`/`TT_InitTeamTalkPoll`, which in this crate happens inside
/// `Client::new()` and `Client::with_hwnd()`.
pub fn set_license(name: &str, key: &str) -> Result<bool> {
    init()?;
    Ok(unsafe {
        teamtalk_sys::api().TT_SetLicenseInformation(name.tt().as_ptr(), key.tt().as_ptr()) == 1
    })
}

/// Initializes the TeamTalk SDK using a custom DLL path.
pub fn init_with_path<P: AsRef<Path>>(path: P) -> Result<()> {
    teamtalk_sys::load(path.as_ref().to_str().ok_or(Error::InitFailed)?)
        .map_err(|_| Error::InitFailed)?;
    Ok(())
}
