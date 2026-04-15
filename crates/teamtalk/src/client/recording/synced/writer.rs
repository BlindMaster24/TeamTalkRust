use super::{
    AudioBlockView, Client, Duration, Error, File, OpenOptions, Path, RecordingSampleFormat,
    Result, Seek, SeekFrom, User, UserId, Write, ffi, render_vars, sanitized_filename,
    unique_recording_path,
};

pub(super) struct AudioBlockGuard<'a> {
    client: &'a Client,
    ptr: *mut ffi::AudioBlock,
}

impl<'a> AudioBlockGuard<'a> {
    pub(super) fn new(client: &'a Client, ptr: *mut ffi::AudioBlock) -> Self {
        Self { client, ptr }
    }

    pub(super) fn ptr(&self) -> *mut ffi::AudioBlock {
        self.ptr
    }
}

impl Drop for AudioBlockGuard<'_> {
    fn drop(&mut self) {
        unsafe {
            let _ = self.client.release_user_audio_block(self.ptr);
        }
    }
}

pub(super) struct UserTrack {
    writer: TrackWriter,
    samples_written: u64,
    sample_rate: Option<i32>,
    channels: Option<i32>,
}

impl UserTrack {
    pub(super) fn new(
        folder: &str,
        file_vars: &str,
        format: RecordingSampleFormat,
        user_id: UserId,
        user: Option<User>,
        default_sample_rate: Option<i32>,
        default_channels: Option<i32>,
    ) -> Result<Self> {
        let username = user.map_or_else(|| "unknown".to_string(), |u| u.username);
        let filename = sanitized_filename(&render_vars(file_vars, user_id, &username));
        let path = Path::new(folder).join(filename);
        let mut writer = TrackWriter::new(&path, format)?;
        let sample_rate = default_sample_rate;
        let channels = default_channels;
        if let (Some(rate), Some(ch)) = (sample_rate, channels) {
            writer.init(rate, ch)?;
        }
        Ok(Self {
            writer,
            samples_written: 0,
            sample_rate,
            channels,
        })
    }

    pub(super) fn ensure_format(&mut self, sample_rate: i32, channels: i32) -> Result<()> {
        if self.sample_rate.is_none() {
            self.sample_rate = Some(sample_rate);
            self.channels = Some(channels);
            self.writer.init(sample_rate, channels)?;
        }
        Ok(())
    }

    pub(super) fn pad_to(&mut self, elapsed: Duration) -> Result<()> {
        let Some(sample_rate) = self.sample_rate else {
            return Ok(());
        };
        let channels = self.channels.unwrap_or(1) as u64;
        let target_samples = (elapsed.as_secs_f64() * f64::from(sample_rate)) as u64;
        if target_samples > self.samples_written {
            let missing = target_samples - self.samples_written;
            self.writer.write_silence(missing, channels)?;
            self.samples_written = target_samples;
        }
        Ok(())
    }

    pub(super) fn write_block(&mut self, block: &AudioBlockView<'_>) -> Result<()> {
        self.writer.write_pcm(block.data)?;
        self.samples_written = self.samples_written.saturating_add(block.samples as u64);
        Ok(())
    }
}

enum TrackWriter {
    Pcm(File),
    Wav(WavWriter),
}

impl TrackWriter {
    fn new(path: &Path, format: RecordingSampleFormat) -> Result<Self> {
        let path = unique_recording_path(path);
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
            .map_err(|e| Error::IoError {
                message: e.to_string(),
            })?;
        Ok(match format {
            RecordingSampleFormat::PcmS16Le => TrackWriter::Pcm(file),
            RecordingSampleFormat::WavS16Le => TrackWriter::Wav(WavWriter::new(file)),
        })
    }

    fn init(&mut self, sample_rate: i32, channels: i32) -> Result<()> {
        match self {
            TrackWriter::Pcm(_) => Ok(()),
            TrackWriter::Wav(writer) => writer.init(sample_rate, channels),
        }
    }

    fn write_pcm(&mut self, data: &[i16]) -> Result<()> {
        let bytes = unsafe {
            std::slice::from_raw_parts(data.as_ptr().cast::<u8>(), std::mem::size_of_val(data))
        };
        match self {
            TrackWriter::Pcm(file) => file.write_all(bytes).map_err(|e| Error::IoError {
                message: e.to_string(),
            }),
            TrackWriter::Wav(writer) => writer.write(bytes),
        }
    }

    fn write_silence(&mut self, samples: u64, channels: u64) -> Result<()> {
        const SILENCE_CHUNK_SAMPLES: usize = 48_000;
        let mut remaining = samples.saturating_mul(channels);
        while remaining > 0 {
            let chunk = remaining.min(SILENCE_CHUNK_SAMPLES as u64) as usize;
            let buf = vec![0i16; chunk];
            self.write_pcm(&buf)?;
            remaining -= chunk as u64;
        }
        Ok(())
    }
}

struct WavWriter {
    file: File,
    data_bytes: u32,
    sample_rate: i32,
    channels: i32,
}

impl WavWriter {
    fn new(file: File) -> Self {
        Self {
            file,
            data_bytes: 0,
            sample_rate: 0,
            channels: 0,
        }
    }

    fn init(&mut self, sample_rate: i32, channels: i32) -> Result<()> {
        self.sample_rate = sample_rate;
        self.channels = channels;
        self.write_header(0)
    }

    fn write(&mut self, data: &[u8]) -> Result<()> {
        self.file.write_all(data).map_err(|e| Error::IoError {
            message: e.to_string(),
        })?;
        self.data_bytes = self.data_bytes.saturating_add(data.len() as u32);
        Ok(())
    }

    fn write_header(&mut self, data_bytes: u32) -> Result<()> {
        let byte_rate = self.sample_rate as u32 * self.channels as u32 * 2;
        let block_align = self.channels as u16 * 2;
        let mut header = Vec::with_capacity(44);
        header.extend_from_slice(b"RIFF");
        header.extend_from_slice(&(36 + data_bytes).to_le_bytes());
        header.extend_from_slice(b"WAVEfmt ");
        header.extend_from_slice(&(16u32).to_le_bytes());
        header.extend_from_slice(&(1u16).to_le_bytes());
        header.extend_from_slice(&(self.channels as u16).to_le_bytes());
        header.extend_from_slice(&(self.sample_rate as u32).to_le_bytes());
        header.extend_from_slice(&byte_rate.to_le_bytes());
        header.extend_from_slice(&block_align.to_le_bytes());
        header.extend_from_slice(&(16u16).to_le_bytes());
        header.extend_from_slice(b"data");
        header.extend_from_slice(&data_bytes.to_le_bytes());
        self.file.write_all(&header).map_err(|e| Error::IoError {
            message: e.to_string(),
        })
    }
}

impl Drop for WavWriter {
    fn drop(&mut self) {
        let _ = self.file.seek(SeekFrom::Start(0));
        let _ = self.write_header(self.data_bytes);
    }
}
