//! Audio device and audio stream APIs.
use super::Client;
use crate::events::Event;
use crate::types::{AudioPreprocessor, SoundDevice, SoundDeviceId, UserId};
use std::io::Write;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use teamtalk_sys as ffi;

/// Audio device selection preset.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub struct AudioDeviceProfile {
    pub input_id: SoundDeviceId,
    pub output_id: SoundDeviceId,
    pub duplex: bool,
}

impl AudioDeviceProfile {
    /// Creates a split input/output profile.
    pub fn split(input_id: SoundDeviceId, output_id: SoundDeviceId) -> Self {
        Self {
            input_id,
            output_id,
            duplex: false,
        }
    }

    /// Creates a duplex input/output profile.
    pub fn duplex(input_id: SoundDeviceId, output_id: SoundDeviceId) -> Self {
        Self {
            input_id,
            output_id,
            duplex: true,
        }
    }
}

mod blocks;
mod devices;

/// View of raw audio block data.
#[non_exhaustive]
pub struct AudioBlockView<'a> {
    pub sample_rate: i32,
    pub channels: i32,
    pub stream_types: u32,
    pub samples: i32,
    /// Monotonic sample offset of this block in the current voice stream.
    ///
    /// Downstream struct literal construction must set this field explicitly.
    pub sample_index: u32,
    pub data: &'a [i16],
}

impl<'a> AudioBlockView<'a> {
    pub fn from_block(block: &'a ffi::AudioBlock) -> Option<Self> {
        if block.lpRawAudio.is_null() || block.nSamples <= 0 || block.nChannels <= 0 {
            return None;
        }
        let count = block.nSamples.saturating_mul(block.nChannels) as usize;
        let ptr = block.lpRawAudio as *const i16;
        let data = unsafe { std::slice::from_raw_parts(ptr, count) };
        Some(Self {
            sample_rate: block.nSampleRate,
            channels: block.nChannels,
            stream_types: block.uStreamTypes,
            samples: block.nSamples,
            sample_index: block.uSampleIndex,
            data,
        })
    }
}

/// Sink for streaming audio blocks.
pub trait AudioBlockSink {
    fn handle(&mut self, block: &AudioBlockView<'_>);
}

/// Sink backed by a callback.
#[non_exhaustive]
pub struct CallbackSink<F>(pub F);

impl<F> CallbackSink<F> {
    /// Creates a new callback sink.
    pub fn new(callback: F) -> Self {
        Self(callback)
    }
}

impl<F> AudioBlockSink for CallbackSink<F>
where
    F: FnMut(&AudioBlockView<'_>),
{
    fn handle(&mut self, block: &AudioBlockView<'_>) {
        (self.0)(block);
    }
}

/// Sink that writes raw PCM data into a writer.
pub struct WriterSink<W> {
    writer: W,
}

impl<W> WriterSink<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> AudioBlockSink for WriterSink<W>
where
    W: Write,
{
    fn handle(&mut self, block: &AudioBlockView<'_>) {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                block.data.as_ptr() as *const u8,
                std::mem::size_of_val(block.data),
            )
        };
        let _ = self.writer.write_all(bytes);
    }
}

/// Sink that sends raw PCM data over UDP.
pub struct UdpSink {
    socket: UdpSocket,
}

impl UdpSink {
    pub fn connect(addr: &str) -> std::io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(addr)?;
        Ok(Self { socket })
    }
}

impl AudioBlockSink for UdpSink {
    fn handle(&mut self, block: &AudioBlockView<'_>) {
        let bytes = unsafe {
            std::slice::from_raw_parts(
                block.data.as_ptr() as *const u8,
                std::mem::size_of_val(block.data),
            )
        };
        let _ = self.socket.send(bytes);
    }
}

/// Guard that cleans up an audio block subscription.
pub struct AudioBlockSubscription<'a> {
    client: &'a Client,
    subscription_id: crate::client::EventSubscriptionId,
    user_id: UserId,
    stream_types: u32,
}

impl Drop for AudioBlockSubscription<'_> {
    fn drop(&mut self) {
        let _ = self
            .client
            .enable_audio_block_event(self.user_id, self.stream_types, false);
        let _ = self.client.unsubscribe_event(self.subscription_id);
    }
}
