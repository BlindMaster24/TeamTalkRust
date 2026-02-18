//! Audio management namespace.
use super::define_namespace;
use crate::client::audio::{AudioBlockSink, AudioBlockSubscription, AudioDeviceProfile};
#[cfg(feature = "async-tokio")]
use crate::client::audio::{AudioStreamConfig, AudioStreamReader, AudioStreamWriter};
use crate::types::{AudioPreprocessor, SoundDevice, UserId};
use teamtalk_sys as ffi;

define_namespace!(AudioNamespace);

impl AudioNamespace {
    /// Returns available sound devices.
    pub fn devices(&self) -> Vec<SoundDevice> {
        self.client.get_sound_devices()
    }

    /// Returns default input and output device ids.
    pub fn default_devices(&self) -> (i32, i32) {
        self.client.get_default_sound_devices()
    }

    /// Returns default input and output device ids for a sound system.
    pub fn default_devices_ex(&self, system: ffi::SoundSystem) -> (i32, i32) {
        self.client.get_default_sound_devices_ex(system)
    }

    /// Restarts the global sound system.
    pub fn restart_system(&self) -> bool {
        self.client.restart_sound_system()
    }

    /// Initializes the sound input device by id.
    pub fn init_input(&self, device_id: i32) -> bool {
        self.client.init_sound_input_device(device_id)
    }

    /// Initializes the sound output device by id.
    pub fn init_output(&self, device_id: i32) -> bool {
        self.client.init_sound_output_device(device_id)
    }

    /// Initializes duplex input/output devices.
    pub fn init_duplex(&self, in_id: i32, out_id: i32) -> bool {
        self.client.init_sound_duplex_devices(in_id, out_id)
    }

    /// Applies an audio profile to the client.
    pub fn apply_profile(&self, profile: AudioDeviceProfile) -> bool {
        self.client.apply_audio_profile(profile)
    }

    /// Closes all sound devices.
    pub fn close_all(&self) -> bool {
        self.client.close_sound_input_device() && self.client.close_sound_output_device()
    }

    /// Returns current sound input level.
    pub fn input_level(&self) -> i32 {
        self.client.get_sound_input_level()
    }

    /// Sets the sound input gain level.
    pub fn set_input_gain(&self, level: i32) -> bool {
        self.client.set_sound_input_gain_level(level)
    }

    /// Returns the sound input gain level.
    pub fn input_gain(&self) -> i32 {
        self.client.get_sound_input_gain_level()
    }

    /// Sets output volume.
    pub fn set_output_volume(&self, volume: i32) -> bool {
        self.client.set_sound_output_volume(volume)
    }

    /// Returns output volume.
    pub fn output_volume(&self) -> i32 {
        self.client.get_sound_output_volume()
    }

    /// Mutes or unmutes output audio.
    pub fn set_output_mute(&self, mute: bool) -> bool {
        self.client.set_sound_output_mute(mute)
    }

    /// Mutes or unmutes a user stream.
    pub fn set_user_mute(&self, user_id: UserId, stream_type: ffi::StreamType, mute: bool) -> bool {
        self.client.set_user_mute(user_id, stream_type, mute)
    }

    /// Enables or disables voice transmission.
    pub fn enable_voice_tx(&self, enable: bool) -> bool {
        self.client.enable_voice_transmission(enable)
    }

    /// Enables or disables voice activation.
    pub fn enable_voice_activation(&self, enable: bool) -> bool {
        self.client.enable_voice_activation(enable)
    }

    /// Sets the voice activation level.
    pub fn set_voice_activation_level(&self, level: i32) -> bool {
        self.client.set_voice_activation_level(level)
    }

    /// Sets the audio preprocessor configuration.
    pub fn set_preprocessor(&self, preprocessor: &AudioPreprocessor) -> bool {
        self.client.set_audio_preprocessor(preprocessor)
    }

    /// Returns the audio preprocessor configuration.
    pub fn preprocessor(&self) -> Option<AudioPreprocessor> {
        self.client.get_audio_preprocessor()
    }

    /// Enables or disables 3D sound positioning.
    pub fn enable_3d(&self, enable: bool) -> bool {
        self.client.enable_3d_sound(enable)
    }

    /// Automatically positions users in 3D space.
    pub fn auto_position_users(&self) -> bool {
        self.client.auto_position_users()
    }

    /// Sets a user's 3D position.
    pub fn set_user_position(
        &self,
        user_id: UserId,
        stream_type: ffi::StreamType,
        x: f32,
        y: f32,
        z: f32,
    ) -> bool {
        self.client.set_user_position(user_id, stream_type, x, y, z)
    }

    /// Subscribes to audio blocks for a user.
    pub fn stream<S>(&self, user_id: UserId, types: u32, sink: S) -> AudioBlockSubscription<'_>
    where
        S: AudioBlockSink + Send + 'static,
    {
        self.client.stream_audio_blocks(user_id, types, sink)
    }
}

#[cfg(feature = "async")]
use super::define_async_namespace;

#[cfg(feature = "async")]
define_async_namespace!(AsyncAudioNamespace);

#[cfg(feature = "async")]
impl AsyncAudioNamespace {
    // TODO: Implement proper async commands with success confirmation where applicable

    /// Returns an async reader for a user's audio stream.
    #[cfg(feature = "async-tokio")]
    pub fn reader(&self, user_id: UserId, stream_types: u32) -> AudioStreamReader<'_> {
        AudioStreamReader::new(&self.client, user_id, stream_types)
    }

    /// Returns an async writer for inserting audio into the mixer.
    #[cfg(feature = "async-tokio")]
    pub fn writer(&self, config: AudioStreamConfig) -> AudioStreamWriter {
        AudioStreamWriter::new(self.client.clone(), config)
    }

    /// Returns available sound devices.
    pub fn devices(&self) -> Vec<SoundDevice> {
        self.client.get_sound_devices()
    }

    /// Sets output volume.
    pub fn set_output_volume(&self, volume: i32) -> bool {
        self.client.set_sound_output_volume(volume)
    }

    /// Enables or disables voice transmission.
    pub fn enable_voice_tx(&self, enable: bool) -> bool {
        self.client.enable_voice_transmission(enable)
    }
}
