//! Async audio streaming implementations.

use crate::types::UserId;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::sync::mpsc;

use super::Client;
use super::audio::{AudioBlockSink, AudioBlockSubscription, AudioBlockView};
use teamtalk_sys as ffi;

/// Configuration for audio streaming.
#[derive(Debug, Clone, Copy)]
pub struct AudioStreamConfig {
    /// Sample rate in Hz (e.g., 48000).
    pub sample_rate: i32,
    /// Number of channels (1 or 2).
    pub channels: i32,
    /// Frame size in milliseconds (e.g., 20).
    pub frame_ms: i32,
}

impl Default for AudioStreamConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48000,
            channels: 1,
            frame_ms: 20,
        }
    }
}

/// Async reader for a user's audio stream.
pub struct AudioStreamReader<'a> {
    _subscription: AudioBlockSubscription<'a>,
    receiver: mpsc::Receiver<Vec<i16>>,
    buffer: Vec<u8>,
}

impl<'a> AudioStreamReader<'a> {
    pub(crate) fn new(client: &'a Client, user_id: UserId, stream_types: u32) -> Self {
        let (tx, rx) = mpsc::channel(128);
        let subscription =
            client.stream_audio_blocks(user_id, stream_types, AudioReceiverSink { tx });
        Self {
            _subscription: subscription,
            receiver: rx,
            buffer: Vec::new(),
        }
    }
}

struct AudioReceiverSink {
    tx: mpsc::Sender<Vec<i16>>,
}

impl AudioBlockSink for AudioReceiverSink {
    fn handle(&mut self, block: &AudioBlockView<'_>) {
        let _ = self.tx.try_send(block.data.to_vec());
    }
}

impl<'a> AsyncRead for AudioStreamReader<'a> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        if !self.buffer.is_empty() {
            let n = std::cmp::min(self.buffer.len(), buf.remaining());
            let data: Vec<u8> = self.buffer.drain(..n).collect();
            buf.put_slice(&data);
            return Poll::Ready(Ok(()));
        }

        match self.receiver.poll_recv(cx) {
            Poll::Ready(Some(pcm_data)) => {
                let byte_data: &[u8] = unsafe {
                    std::slice::from_raw_parts(pcm_data.as_ptr() as *const u8, pcm_data.len() * 2)
                };
                let n = std::cmp::min(byte_data.len(), buf.remaining());
                buf.put_slice(&byte_data[..n]);
                if n < byte_data.len() {
                    self.buffer.extend_from_slice(&byte_data[n..]);
                }
                Poll::Ready(Ok(()))
            }
            Poll::Ready(None) => Poll::Ready(Ok(())),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Async writer for inserting audio into the TeamTalk mixer.
pub struct AudioStreamWriter {
    client: Arc<Client>,
    config: AudioStreamConfig,
    buffer: Vec<u8>,
    frame_size_bytes: usize,
}

impl AudioStreamWriter {
    pub(crate) fn new(client: Arc<Client>, config: AudioStreamConfig) -> Self {
        let frame_size_samples = (config.sample_rate * config.channels * config.frame_ms) / 1000;
        let frame_size_bytes = (frame_size_samples * 2) as usize;
        Self {
            client,
            config,
            buffer: Vec::with_capacity(frame_size_bytes),
            frame_size_bytes,
        }
    }

    fn flush_block(&mut self) -> std::io::Result<()> {
        if self.buffer.len() < self.frame_size_bytes {
            return Ok(());
        }

        let pcm_data: &[i16] = unsafe {
            std::slice::from_raw_parts(self.buffer.as_ptr() as *const i16, self.buffer.len() / 2)
        };

        let mut block = unsafe { std::mem::zeroed::<ffi::AudioBlock>() };
        block.nSampleRate = self.config.sample_rate;
        block.nChannels = self.config.channels;
        block.uStreamTypes = ffi::StreamType::STREAMTYPE_VOICE as u32;
        block.nSamples = (pcm_data.len() / self.config.channels as usize) as i32;
        block.lpRawAudio = pcm_data.as_ptr() as *mut std::ffi::c_void;

        if !self.client.insert_audio_block(&block) {
            return Err(std::io::Error::other("Failed to insert audio block"));
        }

        self.buffer.clear();
        Ok(())
    }
}

impl AsyncWrite for AudioStreamWriter {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let remaining = self.frame_size_bytes - self.buffer.len();
        let n = std::cmp::min(remaining, buf.len());
        self.buffer.extend_from_slice(&buf[..n]);

        if self.buffer.len() >= self.frame_size_bytes {
            self.flush_block()?;
        }

        Poll::Ready(Ok(n))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}
