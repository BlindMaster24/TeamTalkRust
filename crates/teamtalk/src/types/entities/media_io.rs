use crate::types::{ChannelId, FileId, TransferId};
use teamtalk_sys as ffi;

/// Sound device description.
pub struct SoundDevice {
    pub id: i32,
    pub name: String,
    pub system: ffi::SoundSystem,
    pub device_uid: String,
    pub max_input_channels: i32,
    pub max_output_channels: i32,
    pub input_sample_rates: Vec<i32>,
    pub output_sample_rates: Vec<i32>,
    pub default_sample_rate: i32,
    pub features: u32,
}

impl From<ffi::SoundDevice> for SoundDevice {
    fn from(d: ffi::SoundDevice) -> Self {
        Self {
            id: d.nDeviceID,
            name: crate::utils::strings::to_string(&d.szDeviceName),
            system: d.nSoundSystem,
            device_uid: crate::utils::strings::to_string(&d.szDeviceID),
            max_input_channels: d.nMaxInputChannels,
            max_output_channels: d.nMaxOutputChannels,
            input_sample_rates: d
                .inputSampleRates
                .iter()
                .take_while(|&&r| r != 0)
                .cloned()
                .collect(),
            output_sample_rates: d
                .outputSampleRates
                .iter()
                .take_while(|&&r| r != 0)
                .cloned()
                .collect(),
            default_sample_rate: d.nDefaultSampleRate,
            features: d.uSoundDeviceFeatures,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// File transfer status.
pub enum FileTransferStatus {
    Closed,
    Error,
    Active,
    Finished,
}

impl From<ffi::FileTransferStatus> for FileTransferStatus {
    fn from(s: ffi::FileTransferStatus) -> Self {
        match s {
            ffi::FileTransferStatus::FILETRANSFER_CLOSED => Self::Closed,
            ffi::FileTransferStatus::FILETRANSFER_ERROR => Self::Error,
            ffi::FileTransferStatus::FILETRANSFER_ACTIVE => Self::Active,
            ffi::FileTransferStatus::FILETRANSFER_FINISHED => Self::Finished,
        }
    }
}

/// File transfer information.
pub struct FileTransfer {
    pub status: FileTransferStatus,
    pub id: TransferId,
    pub channel_id: ChannelId,
    pub local_path: String,
    pub remote_name: String,
    pub size: i64,
    pub transferred: i64,
    pub inbound: bool,
}

impl FileTransfer {
    /// Returns transfer progress as a 0.0-1.0 fraction.
    pub fn progress(&self) -> f32 {
        if self.size == 0 {
            0.0
        } else {
            self.transferred as f32 / self.size as f32
        }
    }
}

impl From<ffi::FileTransfer> for FileTransfer {
    fn from(t: ffi::FileTransfer) -> Self {
        Self {
            status: FileTransferStatus::from(t.nStatus),
            id: TransferId(t.nTransferID),
            channel_id: ChannelId(t.nChannelID),
            local_path: crate::utils::strings::to_string(&t.szLocalFilePath),
            remote_name: crate::utils::strings::to_string(&t.szRemoteFileName),
            size: t.nFileSize,
            transferred: t.nTransferred,
            inbound: t.bInbound != 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
/// Remote file metadata.
pub struct RemoteFile {
    pub channel_id: ChannelId,
    pub id: FileId,
    pub name: String,
    pub size: i64,
    pub owner: String,
    pub upload_time: String,
}

impl From<ffi::RemoteFile> for RemoteFile {
    fn from(f: ffi::RemoteFile) -> Self {
        Self {
            channel_id: ChannelId(f.nChannelID),
            id: FileId(f.nFileID),
            name: crate::utils::strings::to_string(&f.szFileName),
            size: f.nFileSize,
            owner: crate::utils::strings::to_string(&f.szUsername),
            upload_time: crate::utils::strings::to_string(&f.szUploadTime),
        }
    }
}

#[derive(Debug, Clone)]
/// Media file information.
pub struct MediaFileInfo {
    pub status: ffi::MediaFileStatus,
    pub name: String,
    pub audio_fmt: ffi::AudioFormat,
    pub video_fmt: ffi::VideoFormat,
    pub duration_ms: u32,
    pub elapsed_ms: u32,
}

impl From<ffi::MediaFileInfo> for MediaFileInfo {
    fn from(i: ffi::MediaFileInfo) -> Self {
        Self {
            status: i.nStatus,
            name: crate::utils::strings::to_string(&i.szFileName),
            audio_fmt: i.audioFmt,
            video_fmt: i.videoFmt,
            duration_ms: i.uDurationMSec,
            elapsed_ms: i.uElapsedMSec,
        }
    }
}
