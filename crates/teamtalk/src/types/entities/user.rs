use crate::types::{ChannelId, Subscriptions, UserId, UserState, UserStatus};
use teamtalk_sys as ffi;

/// User state snapshot.
#[non_exhaustive]
#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub nickname: String,
    pub user_data: i32,
    pub user_type: u32,
    pub ip_address: String,
    pub version: u32,
    pub channel_id: ChannelId,
    pub status: UserStatus,
    pub status_msg: String,
    pub state: UserState,
    pub local_subscriptions: Subscriptions,
    pub peer_subscriptions: Subscriptions,
    pub media_storage_dir: String,
    pub volume_voice: i32,
    pub volume_media: i32,
    pub stopped_delay_voice: i32,
    pub stopped_delay_media: i32,
    pub sound_position_voice: [f32; 3],
    pub sound_position_media: [f32; 3],
    pub stereo_playback_voice: [bool; 2],
    pub stereo_playback_media: [bool; 2],
    pub buf_ms_voice: i32,
    pub buf_ms_media: i32,
    pub active_adaptive_delay: i32,
    pub client_name: String,
}

impl From<ffi::User> for User {
    fn from(u: ffi::User) -> Self {
        Self {
            id: UserId(u.nUserID),
            username: crate::utils::strings::to_string(&u.szUsername),
            nickname: crate::utils::strings::to_string(&u.szNickname),
            user_data: u.nUserData,
            user_type: u.uUserType,
            ip_address: crate::utils::strings::to_string(&u.szIPAddress),
            version: u.uVersion,
            channel_id: ChannelId(u.nChannelID),
            status: UserStatus::from_bits(u.nStatusMode as u32),
            status_msg: crate::utils::strings::to_string(&u.szStatusMsg),
            state: UserState::from_raw(u.uUserState),
            local_subscriptions: Subscriptions::from_raw(u.uLocalSubscriptions),
            peer_subscriptions: Subscriptions::from_raw(u.uPeerSubscriptions),
            media_storage_dir: crate::utils::strings::to_string(&u.szMediaStorageDir),
            volume_voice: u.nVolumeVoice,
            volume_media: u.nVolumeMediaFile,
            stopped_delay_voice: u.nStoppedDelayVoice,
            stopped_delay_media: u.nStoppedDelayMediaFile,
            sound_position_voice: u.soundPositionVoice,
            sound_position_media: u.soundPositionMediaFile,
            stereo_playback_voice: [u.stereoPlaybackVoice[0] != 0, u.stereoPlaybackVoice[1] != 0],
            stereo_playback_media: [
                u.stereoPlaybackMediaFile[0] != 0,
                u.stereoPlaybackMediaFile[1] != 0,
            ],
            buf_ms_voice: u.nBufferMSecVoice,
            buf_ms_media: u.nBufferMSecMediaFile,
            active_adaptive_delay: u.nActiveAdaptiveDelayMSec,
            client_name: crate::utils::strings::to_string(&u.szClientName),
        }
    }
}
