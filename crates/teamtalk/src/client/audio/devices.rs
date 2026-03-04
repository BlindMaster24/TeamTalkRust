use super::*;

impl Client {
    pub fn get_sound_devices(&self) -> Vec<SoundDevice> {
        let mut count: i32 = 0;
        unsafe {
            ffi::api().TT_GetSoundDevices(std::ptr::null_mut(), &mut count);
            let mut devices = vec![std::mem::zeroed::<ffi::SoundDevice>(); count as usize];
            if ffi::api().TT_GetSoundDevices(devices.as_mut_ptr(), &mut count) == 1 {
                devices.into_iter().map(SoundDevice::from).collect()
            } else {
                vec![]
            }
        }
    }

    /// Returns default input and output device ids.
    pub fn get_default_sound_devices(&self) -> (i32, i32) {
        let mut input: i32 = 0;
        let mut output: i32 = 0;
        unsafe {
            ffi::api().TT_GetDefaultSoundDevices(&mut input, &mut output);
        }
        (input, output)
    }

    /// Returns default input and output device ids for a sound system.
    pub fn get_default_sound_devices_ex(&self, system: ffi::SoundSystem) -> (i32, i32) {
        let mut input: i32 = 0;
        let mut output: i32 = 0;
        unsafe {
            ffi::api().TT_GetDefaultSoundDevicesEx(system, &mut input, &mut output);
        }
        (input, output)
    }

    /// Restarts the global sound system.
    pub fn restart_sound_system(&self) -> bool {
        unsafe { ffi::api().TT_RestartSoundSystem() == 1 }
    }

    /// Initializes the sound input device by id.
    pub fn init_sound_input_device(&self, device_id: i32) -> bool {
        unsafe { ffi::api().TT_InitSoundInputDevice(self.ptr.0, device_id) == 1 }
    }

    /// Initializes the sound output device by id.
    pub fn init_sound_output_device(&self, device_id: i32) -> bool {
        unsafe { ffi::api().TT_InitSoundOutputDevice(self.ptr.0, device_id) == 1 }
    }

    /// Initializes a shared input device.
    pub fn init_sound_input_shared_device(&self, rate: i32, chans: i32, frame: i32) -> bool {
        unsafe { ffi::api().TT_InitSoundInputSharedDevice(rate, chans, frame) == 1 }
    }

    /// Initializes a shared output device.
    pub fn init_sound_output_shared_device(&self, rate: i32, chans: i32, frame: i32) -> bool {
        unsafe { ffi::api().TT_InitSoundOutputSharedDevice(rate, chans, frame) == 1 }
    }

    /// Initializes duplex input/output devices.
    pub fn init_sound_duplex_devices(&self, in_id: i32, out_id: i32) -> bool {
        unsafe { ffi::api().TT_InitSoundDuplexDevices(self.ptr.0, in_id, out_id) == 1 }
    }

    /// Returns an audio profile using default input and output devices.
    pub fn default_audio_profile(&self) -> AudioDeviceProfile {
        let (input, output) = self.get_default_sound_devices();
        AudioDeviceProfile::split(input, output)
    }

    /// Applies an audio profile to the client.
    pub fn apply_audio_profile(&self, profile: AudioDeviceProfile) -> bool {
        if profile.duplex {
            self.init_sound_duplex_devices(profile.input_id, profile.output_id)
        } else {
            self.init_sound_input_device(profile.input_id)
                && self.init_sound_output_device(profile.output_id)
        }
    }

    /// Closes the sound input device.
    pub fn close_sound_input_device(&self) -> bool {
        unsafe { ffi::api().TT_CloseSoundInputDevice(self.ptr.0) == 1 }
    }

    /// Closes the sound output device.
    pub fn close_sound_output_device(&self) -> bool {
        unsafe { ffi::api().TT_CloseSoundOutputDevice(self.ptr.0) == 1 }
    }

    /// Closes duplex input/output devices.
    pub fn close_sound_duplex_devices(&self) -> bool {
        unsafe { ffi::api().TT_CloseSoundDuplexDevices(self.ptr.0) == 1 }
    }

    /// Returns current sound input level.
    pub fn get_sound_input_level(&self) -> i32 {
        unsafe { ffi::api().TT_GetSoundInputLevel(self.ptr.0) }
    }

    /// Sets the sound input gain level.
    pub fn set_sound_input_gain_level(&self, level: i32) -> bool {
        unsafe { ffi::api().TT_SetSoundInputGainLevel(self.ptr.0, level) == 1 }
    }

    /// Returns the sound input gain level.
    pub fn get_sound_input_gain_level(&self) -> i32 {
        unsafe { ffi::api().TT_GetSoundInputGainLevel(self.ptr.0) }
    }

    /// Sets output volume.
    pub fn set_sound_output_volume(&self, volume: i32) -> bool {
        unsafe { ffi::api().TT_SetSoundOutputVolume(self.ptr.0, volume) == 1 }
    }

    /// Returns output volume.
    pub fn get_sound_output_volume(&self) -> i32 {
        unsafe { ffi::api().TT_GetSoundOutputVolume(self.ptr.0) }
    }

    /// Mutes or unmutes output audio.
    pub fn set_sound_output_mute(&self, mute: bool) -> bool {
        unsafe { ffi::api().TT_SetSoundOutputMute(self.ptr.0, if mute { 1 } else { 0 }) == 1 }
    }

    /// Mutes or unmutes a user stream.
    pub fn set_user_mute(&self, user_id: UserId, stream_type: ffi::StreamType, mute: bool) -> bool {
        unsafe {
            ffi::api().TT_SetUserMute(self.ptr.0, user_id.0, stream_type, if mute { 1 } else { 0 })
                == 1
        }
    }

    /// Sets the user audio stream buffer size.
    pub fn set_user_audio_stream_buffer_size(
        &self,
        user_id: UserId,
        stream_type: ffi::StreamType,
        msec: i32,
    ) -> bool {
        unsafe {
            let st = stream_type as u32;
            ffi::api().TT_SetUserAudioStreamBufferSize(self.ptr.0, user_id.0, st, msec) == 1
        }
    }

    /// Sets stopped playback delay for a user stream.
    pub fn set_user_stopped_playback_delay(
        &self,
        user_id: UserId,
        stream_type: ffi::StreamType,
        msec: i32,
    ) -> bool {
        unsafe {
            ffi::api().TT_SetUserStoppedPlaybackDelay(self.ptr.0, user_id.0, stream_type, msec) == 1
        }
    }

    /// Enables or disables voice transmission.
    pub fn enable_voice_transmission(&self, enable: bool) -> bool {
        unsafe {
            ffi::api().TT_EnableVoiceTransmission(self.ptr.0, if enable { 1 } else { 0 }) == 1
        }
    }

    /// Enables or disables voice activation.
    pub fn enable_voice_activation(&self, enable: bool) -> bool {
        unsafe { ffi::api().TT_EnableVoiceActivation(self.ptr.0, if enable { 1 } else { 0 }) == 1 }
    }

    /// Sets the voice activation level.
    pub fn set_voice_activation_level(&self, level: i32) -> bool {
        unsafe { ffi::api().TT_SetVoiceActivationLevel(self.ptr.0, level) == 1 }
    }

    /// Returns the voice activation level.
    pub fn get_voice_activation_level(&self) -> i32 {
        unsafe { ffi::api().TT_GetVoiceActivationLevel(self.ptr.0) }
    }

    /// Sets the voice activation stop delay.
    pub fn set_voice_activation_stop_delay(&self, delay: i32) -> bool {
        unsafe { ffi::api().TT_SetVoiceActivationStopDelay(self.ptr.0, delay) == 1 }
    }

    /// Returns the voice activation stop delay.
    pub fn get_voice_activation_stop_delay(&self) -> i32 {
        unsafe { ffi::api().TT_GetVoiceActivationStopDelay(self.ptr.0) }
    }

    /// Sets the audio preprocessor configuration.
    pub fn set_audio_preprocessor(&self, preprocessor: &AudioPreprocessor) -> bool {
        unsafe { ffi::api().TT_SetSoundInputPreprocessEx(self.ptr.0, &preprocessor.to_ffi()) == 1 }
    }

    /// Returns the audio preprocessor configuration.
    pub fn get_audio_preprocessor(&self) -> Option<AudioPreprocessor> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::AudioPreprocessor>() };
        if unsafe { ffi::api().TT_GetSoundInputPreprocessEx(self.ptr.0, &mut raw) } == 1 {
            Some(AudioPreprocessor::from(raw))
        } else {
            None
        }
    }

    /// Sets sound device effects.
    pub fn set_device_effects(&self, effects: &ffi::SoundDeviceEffects) -> bool {
        unsafe { ffi::api().TT_SetSoundDeviceEffects(self.ptr.0, effects) == 1 }
    }

    /// Returns sound device effects.
    pub fn get_device_effects(&self) -> Option<ffi::SoundDeviceEffects> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::SoundDeviceEffects>() };
        if unsafe { ffi::api().TT_GetSoundDeviceEffects(self.ptr.0, &mut raw) } == 1 {
            Some(raw)
        } else {
            None
        }
    }

    /// Enables or disables 3D sound positioning.
    pub fn enable_3d_sound(&self, enable: bool) -> bool {
        unsafe {
            ffi::api().TT_Enable3DSoundPositioning(self.ptr.0, if enable { 1 } else { 0 }) == 1
        }
    }

    /// Automatically positions users in 3D space.
    pub fn auto_position_users(&self) -> bool {
        unsafe { ffi::api().TT_AutoPositionUsers(self.ptr.0) == 1 }
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
        unsafe { ffi::api().TT_SetUserPosition(self.ptr.0, user_id.0, stream_type, x, y, z) == 1 }
    }

    /// Sets a user's stereo playback.
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
                user_id.0,
                stream_type,
                left as i32,
                right as i32,
            ) == 1
        }
    }
}
