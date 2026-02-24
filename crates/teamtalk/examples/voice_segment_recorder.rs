use std::collections::HashMap;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use teamtalk::Event;
use teamtalk::types::{Subscriptions, UserId};

struct SegmentWriter {
    file: File,
    data_bytes: u32,
    sample_rate: i32,
    channels: i32,
}

impl SegmentWriter {
    fn create(path: PathBuf, sample_rate: i32, channels: i32) -> std::io::Result<Self> {
        let mut writer = Self {
            file: File::create(path)?,
            data_bytes: 0,
            sample_rate,
            channels,
        };
        writer.write_header(0)?;
        Ok(writer)
    }

    fn write_pcm_i16(&mut self, pcm: &[i16]) -> std::io::Result<()> {
        let bytes = unsafe {
            std::slice::from_raw_parts(pcm.as_ptr() as *const u8, std::mem::size_of_val(pcm))
        };
        self.file.write_all(bytes)?;
        self.data_bytes = self.data_bytes.saturating_add(bytes.len() as u32);
        Ok(())
    }

    fn finalize(mut self) -> std::io::Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        self.write_header(self.data_bytes)?;
        self.file.flush()
    }

    fn write_header(&mut self, data_bytes: u32) -> std::io::Result<()> {
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
        self.file.write_all(&header)
    }
}

#[derive(Default)]
struct UserSegmentState {
    active: bool,
    stream_id: i32,
    last_block_at: Option<Instant>,
    sample_rate: Option<i32>,
    channels: Option<i32>,
    next_segment: u32,
    writer: Option<SegmentWriter>,
}

fn main() -> teamtalk::Result<()> {
    let client = teamtalk::Client::new()?;
    let mut states: HashMap<UserId, UserSegmentState> = HashMap::new();
    let out_dir = PathBuf::from("recordings/segments");
    std::fs::create_dir_all(&out_dir).map_err(|e| teamtalk::Error::IoError {
        message: e.to_string(),
    })?;
    let silence_timeout = Duration::from_millis(800);

    loop {
        if let Some((event, message)) = client.poll(100) {
            match event {
                Event::UserJoined => {
                    if let Some(user) = message.user() {
                        let _ = client.subscribe(
                            user.id,
                            Subscriptions::from_raw(
                                Subscriptions::VOICE | Subscriptions::MEDIAFILE,
                            ),
                        );
                        let _ =
                            client.enable_audio_block_event(user.id, Subscriptions::VOICE, true);
                        states.entry(user.id).or_default();
                    }
                }
                Event::UserLeft => {
                    if let Some(user) = message.user() {
                        let _ =
                            client.enable_audio_block_event(user.id, Subscriptions::VOICE, false);
                        if let Some(state) = states.remove(&user.id) {
                            close_segment(user.id, state);
                        }
                    }
                }
                Event::UserFirstVoiceStreamPacket => {
                    if let Some(user) = message.user() {
                        let state = states.entry(user.id).or_default();
                        state.stream_id = message.source();
                    }
                }
                Event::AudioBlock => {
                    if let Some(user) = message.user()
                        && let Some(ptr) =
                            client.acquire_user_audio_block(Subscriptions::VOICE, user.id)
                    {
                        let block = unsafe { &*ptr };
                        let sample_rate = block.nSampleRate;
                        let channels = block.nChannels;
                        let sample_index = block.uSampleIndex;
                        let count = block.nSamples.saturating_mul(block.nChannels) as usize;
                        let pcm_ptr = block.lpRawAudio as *const i16;
                        let pcm = unsafe { std::slice::from_raw_parts(pcm_ptr, count) };

                        let state = states.entry(user.id).or_default();
                        state.sample_rate = Some(sample_rate);
                        state.channels = Some(channels);
                        state.last_block_at = Some(Instant::now());

                        if !state.active || sample_index == 0 {
                            if state.active {
                                let old = std::mem::take(&mut state.writer);
                                if let Some(writer) = old {
                                    let _ = writer.finalize();
                                }
                            }
                            state.active = true;
                            state.next_segment = state.next_segment.saturating_add(1);

                            let mut path = out_dir.clone();
                            path.push(format!(
                                "user-{}-segment-{:04}.wav",
                                user.id.0, state.next_segment
                            ));

                            if let Ok(writer) =
                                SegmentWriter::create(path.clone(), sample_rate, channels)
                            {
                                println!(
                                    "segment start: user_id={}, stream_id={}, file={}",
                                    user.id.0,
                                    state.stream_id,
                                    path.to_string_lossy()
                                );
                                state.writer = Some(writer);
                            }
                        }

                        if let Some(writer) = state.writer.as_mut() {
                            let _ = writer.write_pcm_i16(pcm);
                        }

                        unsafe {
                            let _ = client.release_user_audio_block(ptr);
                        }
                    }
                }
                Event::ConnectionLost | Event::ConnectFailed => break,
                _ => {}
            }
        }

        let now = Instant::now();
        for (user_id, state) in &mut states {
            if state.active
                && let Some(last) = state.last_block_at
                && now.duration_since(last) >= silence_timeout
            {
                println!("segment end: user_id={}", user_id.0);
                state.active = false;
                if let Some(writer) = state.writer.take() {
                    let _ = writer.finalize();
                }
            }
        }
    }

    for (user_id, state) in states {
        let _ = client.enable_audio_block_event(user_id, Subscriptions::VOICE, false);
        close_segment(user_id, state);
    }

    Ok(())
}

fn close_segment(user_id: UserId, mut state: UserSegmentState) {
    if state.active {
        println!("segment end: user_id={}", user_id.0);
    }
    if let Some(writer) = state.writer.take() {
        let _ = writer.finalize();
    }
}
