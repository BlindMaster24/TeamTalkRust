use teamtalk_sys as ffi;

/// User statistics snapshot.
#[non_exhaustive]
pub struct UserStatistics {
    pub voice_recv: i64,
    pub voice_lost: i64,
    pub video_recv: i64,
    pub video_frames_recv: i64,
    pub video_frames_lost: i64,
    pub video_frames_dropped: i64,
    pub media_audio_recv: i64,
    pub media_audio_lost: i64,
    pub media_video_recv: i64,
    pub media_video_frames_recv: i64,
    pub media_video_frames_lost: i64,
    pub media_video_frames_dropped: i64,
}

impl From<ffi::UserStatistics> for UserStatistics {
    fn from(s: ffi::UserStatistics) -> Self {
        Self {
            voice_recv: s.nVoicePacketsRecv,
            voice_lost: s.nVoicePacketsLost,
            video_recv: s.nVideoCapturePacketsRecv,
            video_frames_recv: s.nVideoCaptureFramesRecv,
            video_frames_lost: s.nVideoCaptureFramesLost,
            video_frames_dropped: s.nVideoCaptureFramesDropped,
            media_audio_recv: s.nMediaFileAudioPacketsRecv,
            media_audio_lost: s.nMediaFileAudioPacketsLost,
            media_video_recv: s.nMediaFileVideoPacketsRecv,
            media_video_frames_recv: s.nMediaFileVideoFramesRecv,
            media_video_frames_lost: s.nMediaFileVideoFramesLost,
            media_video_frames_dropped: s.nMediaFileVideoFramesDropped,
        }
    }
}

/// Server statistics snapshot.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct ServerStatistics {
    pub total_tx: i64,
    pub total_rx: i64,
    pub voice_tx: i64,
    pub voice_rx: i64,
    pub video_tx: i64,
    pub video_rx: i64,
    pub media_tx: i64,
    pub media_rx: i64,
    pub desktop_tx: i64,
    pub desktop_rx: i64,
    pub users_served: i32,
    pub users_peak: i32,
    pub files_tx: i64,
    pub files_rx: i64,
    pub uptime_ms: i64,
}

impl From<ffi::ServerStatistics> for ServerStatistics {
    fn from(s: ffi::ServerStatistics) -> Self {
        Self {
            total_tx: s.nTotalBytesTX,
            total_rx: s.nTotalBytesRX,
            voice_tx: s.nVoiceBytesTX,
            voice_rx: s.nVoiceBytesRX,
            video_tx: s.nVideoCaptureBytesTX,
            video_rx: s.nVideoCaptureBytesRX,
            media_tx: s.nMediaFileBytesTX,
            media_rx: s.nMediaFileBytesRX,
            desktop_tx: s.nDesktopBytesTX,
            desktop_rx: s.nDesktopBytesRX,
            users_served: s.nUsersServed,
            users_peak: s.nUsersPeak,
            files_tx: s.nFilesTx,
            files_rx: s.nFilesRx,
            uptime_ms: s.nUptimeMSec,
        }
    }
}
