use super::{
    AudioDeviceProfile, AudioPreprocessor, Client, SoundDevice, SoundDeviceId, UserId, ffi,
};

impl Client {
    #[must_use]
    pub fn get_sound_devices(&self) -> Vec<SoundDevice> {
        let mut count: i32 = 0;
        unsafe {
            ffi::api().TT_GetSoundDevices(std::ptr::null_mut(), &raw mut count);
            let mut devices = vec![std::mem::zeroed::<ffi::SoundDevice>(); count as usize];
            if ffi::api().TT_GetSoundDevices(devices.as_mut_ptr(), &raw mut count) == 1 {
                devices.into_iter().map(SoundDevice::from).collect()
            } else {
                vec![]
            }
        }
    }

    /// Returns default input and output device ids.
    #[must_use]
    pub fn get_default_sound_devices(&self) -> (SoundDeviceId, SoundDeviceId) {
        let mut input: i32 = 0;
        let mut output: i32 = 0;
        unsafe {
            ffi::api().TT_GetDefaultSoundDevices(&raw mut input, &raw mut output);
        }
        (SoundDeviceId(input), SoundDeviceId(output))
    }

    /// Returns default input and output device ids for a sound system.
    #[must_use]
    pub fn get_default_sound_devices_ex(
        &self,
        system: ffi::SoundSystem,
    ) -> (SoundDeviceId, SoundDeviceId) {
        let mut input: i32 = 0;
        let mut output: i32 = 0;
        unsafe {
            ffi::api().TT_GetDefaultSoundDevicesEx(system, &raw mut input, &raw mut output);
        }
        (SoundDeviceId(input), SoundDeviceId(output))
    }

    /// Restarts the global sound system.
    #[must_use]
    pub fn restart_sound_system(&self) -> bool {
        unsafe { ffi::api().TT_RestartSoundSystem() == 1 }
    }

    /// Initializes the sound input device by id.
    #[must_use]
    pub fn init_sound_input_device(&self, device_id: SoundDeviceId) -> bool {
        unsafe { ffi::api().TT_InitSoundInputDevice(self.ptr.0, device_id.raw()) == 1 }
    }

    /// Initializes the sound output device by id.
    #[must_use]
    pub fn init_sound_output_device(&self, device_id: SoundDeviceId) -> bool {
        unsafe { ffi::api().TT_InitSoundOutputDevice(self.ptr.0, device_id.raw()) == 1 }
    }

    /// Initializes a shared input device.
    #[must_use]
    pub fn init_sound_input_shared_device(&self, rate: i32, chans: i32, frame: i32) -> bool {
        unsafe { ffi::api().TT_InitSoundInputSharedDevice(rate, chans, frame) == 1 }
    }

    /// Initializes a shared output device.
    #[must_use]
    pub fn init_sound_output_shared_device(&self, rate: i32, chans: i32, frame: i32) -> bool {
        unsafe { ffi::api().TT_InitSoundOutputSharedDevice(rate, chans, frame) == 1 }
    }

    /// Initializes duplex input/output devices.
    #[must_use]
    pub fn init_sound_duplex_devices(&self, in_id: SoundDeviceId, out_id: SoundDeviceId) -> bool {
        unsafe { ffi::api().TT_InitSoundDuplexDevices(self.ptr.0, in_id.raw(), out_id.raw()) == 1 }
    }

    /// Returns an audio profile using default input and output devices.
    #[must_use]
    pub fn default_audio_profile(&self) -> AudioDeviceProfile {
        let (input, output) = self.get_default_sound_devices();
        AudioDeviceProfile::split(input, output)
    }

    /// Applies an audio profile to the client.
    #[must_use]
    pub fn apply_audio_profile(&self, profile: AudioDeviceProfile) -> bool {
        if profile.duplex {
            self.init_sound_duplex_devices(profile.input_id, profile.output_id)
        } else {
            self.init_sound_input_device(profile.input_id)
                && self.init_sound_output_device(profile.output_id)
        }
    }

    /// Closes the sound input device.
    #[must_use]
    pub fn close_sound_input_device(&self) -> bool {
        unsafe { ffi::api().TT_CloseSoundInputDevice(self.ptr.0) == 1 }
    }

    /// Closes the sound output device.
    #[must_use]
    pub fn close_sound_output_device(&self) -> bool {
        unsafe { ffi::api().TT_CloseSoundOutputDevice(self.ptr.0) == 1 }
    }

    /// Closes duplex input/output devices.
    #[must_use]
    pub fn close_sound_duplex_devices(&self) -> bool {
        unsafe { ffi::api().TT_CloseSoundDuplexDevices(self.ptr.0) == 1 }
    }

    /// Returns current sound input level.
    #[must_use]
    pub fn get_sound_input_level(&self) -> i32 {
        unsafe { ffi::api().TT_GetSoundInputLevel(self.ptr.0) }
    }

    /// Sets the sound input gain level.
    #[must_use]
    pub fn set_sound_input_gain_level(&self, level: i32) -> bool {
        unsafe { ffi::api().TT_SetSoundInputGainLevel(self.ptr.0, level) == 1 }
    }

    /// Returns the sound input gain level.
    #[must_use]
    pub fn get_sound_input_gain_level(&self) -> i32 {
        unsafe { ffi::api().TT_GetSoundInputGainLevel(self.ptr.0) }
    }

    /// Sets output volume.
    #[must_use]
    pub fn set_sound_output_volume(&self, volume: i32) -> bool {
        unsafe { ffi::api().TT_SetSoundOutputVolume(self.ptr.0, volume) == 1 }
    }

    /// Returns output volume.
    #[must_use]
    pub fn get_sound_output_volume(&self) -> i32 {
        unsafe { ffi::api().TT_GetSoundOutputVolume(self.ptr.0) }
    }

    /// Mutes or unmutes output audio.
    #[must_use]
    pub fn set_sound_output_mute(&self, mute: bool) -> bool {
        unsafe { ffi::api().TT_SetSoundOutputMute(self.ptr.0, i32::from(mute)) == 1 }
    }

    /// Mutes or unmutes a user stream.
    #[must_use]
    pub fn set_user_mute(&self, user_id: UserId, stream_type: ffi::StreamType, mute: bool) -> bool {
        unsafe {
            ffi::api().TT_SetUserMute(self.ptr.0, user_id.raw(), stream_type, i32::from(mute)) == 1
        }
    }

    /// Sets the user audio stream buffer size.
    #[must_use]
    pub fn set_user_audio_stream_buffer_size(
        &self,
        user_id: UserId,
        stream_type: ffi::StreamType,
        msec: i32,
    ) -> bool {
        unsafe {
            let st = stream_type as u32;
            ffi::api().TT_SetUserAudioStreamBufferSize(self.ptr.0, user_id.raw(), st, msec) == 1
        }
    }

    /// Sets stopped playback delay for a user stream.
    #[must_use]
    pub fn set_user_stopped_playback_delay(
        &self,
        user_id: UserId,
        stream_type: ffi::StreamType,
        msec: i32,
    ) -> bool {
        unsafe {
            ffi::api().TT_SetUserStoppedPlaybackDelay(self.ptr.0, user_id.raw(), stream_type, msec)
                == 1
        }
    }

    /// Enables or disables voice transmission.
    #[must_use]
    pub fn enable_voice_transmission(&self, enable: bool) -> bool {
        unsafe { ffi::api().TT_EnableVoiceTransmission(self.ptr.0, i32::from(enable)) == 1 }
    }

    /// Enables or disables voice activation.
    #[must_use]
    pub fn enable_voice_activation(&self, enable: bool) -> bool {
        unsafe { ffi::api().TT_EnableVoiceActivation(self.ptr.0, i32::from(enable)) == 1 }
    }

    /// Sets the voice activation level.
    #[must_use]
    pub fn set_voice_activation_level(&self, level: i32) -> bool {
        unsafe { ffi::api().TT_SetVoiceActivationLevel(self.ptr.0, level) == 1 }
    }

    /// Returns the voice activation level.
    #[must_use]
    pub fn get_voice_activation_level(&self) -> i32 {
        unsafe { ffi::api().TT_GetVoiceActivationLevel(self.ptr.0) }
    }

    /// Sets the voice activation stop delay.
    #[must_use]
    pub fn set_voice_activation_stop_delay(&self, delay: i32) -> bool {
        unsafe { ffi::api().TT_SetVoiceActivationStopDelay(self.ptr.0, delay) == 1 }
    }

    /// Returns the voice activation stop delay.
    #[must_use]
    pub fn get_voice_activation_stop_delay(&self) -> i32 {
        unsafe { ffi::api().TT_GetVoiceActivationStopDelay(self.ptr.0) }
    }

    /// Sets the audio preprocessor configuration.
    #[must_use]
    pub fn set_audio_preprocessor(&self, preprocessor: &AudioPreprocessor) -> bool {
        unsafe { ffi::api().TT_SetSoundInputPreprocessEx(self.ptr.0, &preprocessor.to_ffi()) == 1 }
    }

    /// Returns the audio preprocessor configuration.
    pub fn get_audio_preprocessor(&self) -> Option<AudioPreprocessor> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::AudioPreprocessor>() };
        if unsafe { ffi::api().TT_GetSoundInputPreprocessEx(self.ptr.0, &raw mut raw) } == 1 {
            Some(AudioPreprocessor::from(raw))
        } else {
            None
        }
    }

    /// Sets sound device effects.
    #[must_use]
    pub fn set_device_effects(&self, effects: &ffi::SoundDeviceEffects) -> bool {
        unsafe { ffi::api().TT_SetSoundDeviceEffects(self.ptr.0, effects) == 1 }
    }

    /// Returns sound device effects.
    pub fn get_device_effects(&self) -> Option<ffi::SoundDeviceEffects> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::SoundDeviceEffects>() };
        if unsafe { ffi::api().TT_GetSoundDeviceEffects(self.ptr.0, &raw mut raw) } == 1 {
            Some(raw)
        } else {
            None
        }
    }

    /// Enables or disables 3D sound positioning.
    #[must_use]
    pub fn enable_3d_sound(&self, enable: bool) -> bool {
        unsafe { ffi::api().TT_Enable3DSoundPositioning(self.ptr.0, i32::from(enable)) == 1 }
    }

    /// Automatically positions users in 3D space.
    #[must_use]
    pub fn auto_position_users(&self) -> bool {
        unsafe { ffi::api().TT_AutoPositionUsers(self.ptr.0) == 1 }
    }

    /// Sets a user's 3D position.
    #[must_use]
    pub fn set_user_position(
        &self,
        user_id: UserId,
        stream_type: ffi::StreamType,
        x: f32,
        y: f32,
        z: f32,
    ) -> bool {
        unsafe {
            ffi::api().TT_SetUserPosition(self.ptr.0, user_id.raw(), stream_type, x, y, z) == 1
        }
    }

    /// Sets a user's stereo playback.
    #[must_use]
    pub fn set_user_stereo(
        &self,
        user_id: UserId,
        stream_type: ffi::StreamType,
        left: bool,
        right: bool,
    ) -> bool {
        unsafe {
            ffi::api().TT_SetUserStereo(
                self.ptr.0,
                user_id.raw(),
                stream_type,
                i32::from(left),
                i32::from(right),
            ) == 1
        }
    }
}
