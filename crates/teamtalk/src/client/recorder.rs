use crate::events::Event;
use std::collections::VecDeque;
use teamtalk_sys as ffi;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecordedEvent {
    pub event: String,
    pub source: i32,
}

pub struct EventRecorder {
    events: Vec<RecordedEvent>,
}

impl EventRecorder {
    #[allow(clippy::must_use_candidate)]
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn record(&mut self, event: Event, source: i32) {
        self.events.push(RecordedEvent {
            event: format!("{event:?}"),
            source,
        });
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.events)
    }

    pub fn from_json(data: &str) -> Result<Self, serde_json::Error> {
        let events = serde_json::from_str(data)?;
        Ok(Self { events })
    }

    #[must_use]
    pub fn events(&self) -> &[RecordedEvent] {
        &self.events
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl Default for EventRecorder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EventReplayer {
    events: VecDeque<RecordedEvent>,
}

impl EventReplayer {
    #[allow(clippy::must_use_candidate)]
    pub fn from_recorder(recorder: EventRecorder) -> Self {
        Self {
            events: recorder.events.into_iter().collect(),
        }
    }

    pub fn from_json(data: &str) -> Result<Self, serde_json::Error> {
        let recorder = EventRecorder::from_json(data)?;
        Ok(Self::from_recorder(recorder))
    }

    pub fn replay_next(&mut self, backend: &super::backend::MockBackend) -> bool {
        let recorded = match self.events.pop_front() {
            Some(e) => e,
            None => return false,
        };
        let event = parse_event(&recorded.event);
        let mut msg = unsafe { std::mem::zeroed::<ffi::TTMessage>() };
        msg.nSource = recorded.source;
        msg.nClientEvent = event_to_client_event(event);
        backend.push_raw_message(msg);
        true
    }

    pub fn replay_all(&mut self, backend: &super::backend::MockBackend) {
        while self.replay_next(backend) {}
    }

    #[must_use]
    pub fn remaining(&self) -> usize {
        self.events.len()
    }
}

fn parse_event(s: &str) -> Event {
    match s {
        "None" => Event::None,
        "ConnectSuccess" => Event::ConnectSuccess,
        "ConnectCryptError" => Event::ConnectCryptError,
        "ConnectFailed" => Event::ConnectFailed,
        "ConnectionLost" => Event::ConnectionLost,
        "ConnectMaxPayloadUpdated" => Event::ConnectMaxPayloadUpdated,
        "CmdProcessing" => Event::CmdProcessing,
        "CmdError" => Event::CmdError,
        "CmdSuccess" => Event::CmdSuccess,
        "MySelfLoggedIn" => Event::MySelfLoggedIn,
        "MySelfLoggedOut" => Event::MySelfLoggedOut,
        "MySelfKicked" => Event::MySelfKicked,
        "UserLoggedIn" => Event::UserLoggedIn,
        "UserLoggedOut" => Event::UserLoggedOut,
        "UserUpdate" => Event::UserUpdate,
        "UserJoined" => Event::UserJoined,
        "UserLeft" => Event::UserLeft,
        "TextMessage" => Event::TextMessage,
        "ChannelCreated" => Event::ChannelCreated,
        "ChannelUpdated" => Event::ChannelUpdated,
        "ChannelRemoved" => Event::ChannelRemoved,
        "ServerUpdate" => Event::ServerUpdate,
        "ServerStatistics" => Event::ServerStatistics,
        "FileNew" => Event::FileNew,
        "FileRemove" => Event::FileRemove,
        "UserAccount" => Event::UserAccount,
        "BannedUser" => Event::BannedUser,
        "UserAccountCreated" => Event::UserAccountCreated,
        "UserAccountRemoved" => Event::UserAccountRemoved,
        "UserStateChange" => Event::UserStateChange,
        "VideoCaptureFrame" => Event::VideoCaptureFrame,
        "MediaFileVideo" => Event::MediaFileVideo,
        "DesktopWindow" => Event::DesktopWindow,
        "DesktopCursor" => Event::DesktopCursor,
        "DesktopInput" => Event::DesktopInput,
        "UserRecordMediaFile" => Event::UserRecordMediaFile,
        "AudioBlock" => Event::AudioBlock,
        "InternalError" => Event::InternalError,
        "VoiceActivation" => Event::VoiceActivation,
        "Hotkey" => Event::Hotkey,
        "HotkeyTest" => Event::HotkeyTest,
        "FileTransfer" => Event::FileTransfer,
        "DesktopWindowTransfer" => Event::DesktopWindowTransfer,
        "StreamMediaFile" => Event::StreamMediaFile,
        "LocalMediaFile" => Event::LocalMediaFile,
        "AudioInput" => Event::AudioInput,
        "UserFirstVoiceStreamPacket" => Event::UserFirstVoiceStreamPacket,
        "SoundDeviceAdded" => Event::SoundDeviceAdded,
        "SoundDeviceRemoved" => Event::SoundDeviceRemoved,
        "SoundDeviceUnplugged" => Event::SoundDeviceUnplugged,
        "SoundDeviceNewDefaultInput" => Event::SoundDeviceNewDefaultInput,
        "SoundDeviceNewDefaultOutput" => Event::SoundDeviceNewDefaultOutput,
        "SoundDeviceNewDefaultInputComDevice" => Event::SoundDeviceNewDefaultInputComDevice,
        "SoundDeviceNewDefaultOutputComDevice" => Event::SoundDeviceNewDefaultOutputComDevice,
        _ => {
            if s.starts_with("BeforeReconnect") {
                Event::BeforeReconnect {
                    attempt: 0,
                    delay: std::time::Duration::ZERO,
                }
            } else if s.starts_with("Reconnecting") {
                Event::Reconnecting {
                    attempt: 0,
                    delay: std::time::Duration::ZERO,
                }
            } else if s.starts_with("AfterReconnect") {
                Event::AfterReconnect { attempt: 0 }
            } else if s.starts_with("ReconnectFailed") {
                Event::ReconnectFailed { attempts: 0 }
            } else if s.starts_with("BeforeAutoLogin") {
                Event::BeforeAutoLogin {
                    attempt: 0,
                    delay: std::time::Duration::ZERO,
                }
            } else if s.starts_with("AutoLoginFailed") {
                Event::AutoLoginFailed { attempts: 0 }
            } else if s.starts_with("BeforeAutoJoin") {
                Event::BeforeAutoJoin {
                    attempt: 0,
                    delay: std::time::Duration::ZERO,
                }
            } else if s.starts_with("AutoJoinFailed") {
                Event::AutoJoinFailed { attempts: 0 }
            } else if s.starts_with("AutoRecoverCompleted") {
                Event::AutoRecoverCompleted {
                    reconnect_attempts: 0,
                    login_attempts: 0,
                    join_attempts: 0,
                }
            } else {
                Event::None
            }
        }
    }
}

fn event_to_client_event(event: Event) -> ffi::ClientEvent {
    match event {
        Event::None => ffi::ClientEvent::CLIENTEVENT_NONE,
        Event::ConnectSuccess => ffi::ClientEvent::CLIENTEVENT_CON_SUCCESS,
        Event::ConnectCryptError => ffi::ClientEvent::CLIENTEVENT_CON_CRYPT_ERROR,
        Event::ConnectFailed => ffi::ClientEvent::CLIENTEVENT_CON_FAILED,
        Event::ConnectionLost => ffi::ClientEvent::CLIENTEVENT_CON_LOST,
        Event::ConnectMaxPayloadUpdated => ffi::ClientEvent::CLIENTEVENT_CON_MAX_PAYLOAD_UPDATED,
        Event::CmdProcessing => ffi::ClientEvent::CLIENTEVENT_CMD_PROCESSING,
        Event::CmdError => ffi::ClientEvent::CLIENTEVENT_CMD_ERROR,
        Event::CmdSuccess => ffi::ClientEvent::CLIENTEVENT_CMD_SUCCESS,
        Event::MySelfLoggedIn => ffi::ClientEvent::CLIENTEVENT_CMD_MYSELF_LOGGEDIN,
        Event::MySelfLoggedOut => ffi::ClientEvent::CLIENTEVENT_CMD_MYSELF_LOGGEDOUT,
        Event::MySelfKicked => ffi::ClientEvent::CLIENTEVENT_CMD_MYSELF_KICKED,
        Event::UserLoggedIn => ffi::ClientEvent::CLIENTEVENT_CMD_USER_LOGGEDIN,
        Event::UserLoggedOut => ffi::ClientEvent::CLIENTEVENT_CMD_USER_LOGGEDOUT,
        Event::UserUpdate => ffi::ClientEvent::CLIENTEVENT_CMD_USER_UPDATE,
        Event::UserJoined => ffi::ClientEvent::CLIENTEVENT_CMD_USER_JOINED,
        Event::UserLeft => ffi::ClientEvent::CLIENTEVENT_CMD_USER_LEFT,
        Event::TextMessage => ffi::ClientEvent::CLIENTEVENT_CMD_USER_TEXTMSG,
        Event::ChannelCreated => ffi::ClientEvent::CLIENTEVENT_CMD_CHANNEL_NEW,
        Event::ChannelUpdated => ffi::ClientEvent::CLIENTEVENT_CMD_CHANNEL_UPDATE,
        Event::ChannelRemoved => ffi::ClientEvent::CLIENTEVENT_CMD_CHANNEL_REMOVE,
        Event::ServerUpdate => ffi::ClientEvent::CLIENTEVENT_CMD_SERVER_UPDATE,
        Event::ServerStatistics => ffi::ClientEvent::CLIENTEVENT_CMD_SERVERSTATISTICS,
        Event::FileNew => ffi::ClientEvent::CLIENTEVENT_CMD_FILE_NEW,
        Event::FileRemove => ffi::ClientEvent::CLIENTEVENT_CMD_FILE_REMOVE,
        Event::UserAccount => ffi::ClientEvent::CLIENTEVENT_CMD_USERACCOUNT,
        Event::BannedUser => ffi::ClientEvent::CLIENTEVENT_CMD_BANNEDUSER,
        Event::UserAccountCreated => ffi::ClientEvent::CLIENTEVENT_CMD_USERACCOUNT_NEW,
        Event::UserAccountRemoved => ffi::ClientEvent::CLIENTEVENT_CMD_USERACCOUNT_REMOVE,
        Event::UserStateChange => ffi::ClientEvent::CLIENTEVENT_USER_STATECHANGE,
        Event::VideoCaptureFrame => ffi::ClientEvent::CLIENTEVENT_USER_VIDEOCAPTURE,
        Event::MediaFileVideo => ffi::ClientEvent::CLIENTEVENT_USER_MEDIAFILE_VIDEO,
        Event::DesktopWindow => ffi::ClientEvent::CLIENTEVENT_USER_DESKTOPWINDOW,
        Event::DesktopCursor => ffi::ClientEvent::CLIENTEVENT_USER_DESKTOPCURSOR,
        Event::DesktopInput => ffi::ClientEvent::CLIENTEVENT_USER_DESKTOPINPUT,
        Event::UserRecordMediaFile => ffi::ClientEvent::CLIENTEVENT_USER_RECORD_MEDIAFILE,
        Event::AudioBlock => ffi::ClientEvent::CLIENTEVENT_USER_AUDIOBLOCK,
        Event::InternalError => ffi::ClientEvent::CLIENTEVENT_INTERNAL_ERROR,
        Event::VoiceActivation => ffi::ClientEvent::CLIENTEVENT_VOICE_ACTIVATION,
        Event::Hotkey => ffi::ClientEvent::CLIENTEVENT_HOTKEY,
        Event::HotkeyTest => ffi::ClientEvent::CLIENTEVENT_HOTKEY_TEST,
        Event::FileTransfer => ffi::ClientEvent::CLIENTEVENT_FILETRANSFER,
        Event::DesktopWindowTransfer => ffi::ClientEvent::CLIENTEVENT_DESKTOPWINDOW_TRANSFER,
        Event::StreamMediaFile => ffi::ClientEvent::CLIENTEVENT_STREAM_MEDIAFILE,
        Event::LocalMediaFile => ffi::ClientEvent::CLIENTEVENT_LOCAL_MEDIAFILE,
        Event::AudioInput => ffi::ClientEvent::CLIENTEVENT_AUDIOINPUT,
        Event::UserFirstVoiceStreamPacket => {
            ffi::ClientEvent::CLIENTEVENT_USER_FIRSTVOICESTREAMPACKET
        }
        Event::SoundDeviceAdded => ffi::ClientEvent::CLIENTEVENT_SOUNDDEVICE_ADDED,
        Event::SoundDeviceRemoved => ffi::ClientEvent::CLIENTEVENT_SOUNDDEVICE_REMOVED,
        Event::SoundDeviceUnplugged => ffi::ClientEvent::CLIENTEVENT_SOUNDDEVICE_UNPLUGGED,
        Event::SoundDeviceNewDefaultInput => {
            ffi::ClientEvent::CLIENTEVENT_SOUNDDEVICE_NEW_DEFAULT_INPUT
        }
        Event::SoundDeviceNewDefaultOutput => {
            ffi::ClientEvent::CLIENTEVENT_SOUNDDEVICE_NEW_DEFAULT_OUTPUT
        }
        Event::SoundDeviceNewDefaultInputComDevice => {
            ffi::ClientEvent::CLIENTEVENT_SOUNDDEVICE_NEW_DEFAULT_INPUT_COMDEVICE
        }
        Event::SoundDeviceNewDefaultOutputComDevice => {
            ffi::ClientEvent::CLIENTEVENT_SOUNDDEVICE_NEW_DEFAULT_OUTPUT_COMDEVICE
        }
        Event::BeforeReconnect { .. }
        | Event::Reconnecting { .. }
        | Event::AfterReconnect { .. }
        | Event::ReconnectFailed { .. }
        | Event::BeforeAutoLogin { .. }
        | Event::AutoLoginFailed { .. }
        | Event::BeforeAutoJoin { .. }
        | Event::AutoJoinFailed { .. }
        | Event::AutoRecoverCompleted { .. } => ffi::ClientEvent::CLIENTEVENT_NONE,
        Event::Unknown(c) => c,
    }
}
