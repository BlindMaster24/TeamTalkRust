use super::Client;
use crate::types::{AudioPreprocessor, SoundDevice, UserId};
use teamtalk_sys as ffi;

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

    pub fn get_default_sound_devices(&self) -> (i32, i32) {
        let mut input: i32 = 0;
        let mut output: i32 = 0;
        unsafe {
            ffi::api().TT_GetDefaultSoundDevices(&mut input, &mut output);
        }
        (input, output)
    }

    pub fn get_default_sound_devices_ex(&self, system: ffi::SoundSystem) -> (i32, i32) {
        let mut input: i32 = 0;
        let mut output: i32 = 0;
        unsafe {
            ffi::api().TT_GetDefaultSoundDevicesEx(system, &mut input, &mut output);
        }
        (input, output)
    }

    pub fn restart_sound_system(&self) -> bool {
        unsafe { ffi::api().TT_RestartSoundSystem() == 1 }
    }

    pub fn init_sound_input_device(&self, device_id: i32) -> bool {
        unsafe { ffi::api().TT_InitSoundInputDevice(self.ptr, device_id) == 1 }
    }

    pub fn init_sound_output_device(&self, device_id: i32) -> bool {
        unsafe { ffi::api().TT_InitSoundOutputDevice(self.ptr, device_id) == 1 }
    }

    pub fn init_sound_input_shared_device(&self, rate: i32, chans: i32, frame: i32) -> bool {
        unsafe { ffi::api().TT_InitSoundInputSharedDevice(rate, chans, frame) == 1 }
    }

    pub fn init_sound_output_shared_device(&self, rate: i32, chans: i32, frame: i32) -> bool {
        unsafe { ffi::api().TT_InitSoundOutputSharedDevice(rate, chans, frame) == 1 }
    }

    pub fn init_sound_duplex_devices(&self, in_id: i32, out_id: i32) -> bool {
        unsafe { ffi::api().TT_InitSoundDuplexDevices(self.ptr, in_id, out_id) == 1 }
    }

    pub fn close_sound_input_device(&self) -> bool {
        unsafe { ffi::api().TT_CloseSoundInputDevice(self.ptr) == 1 }
    }

    pub fn close_sound_output_device(&self) -> bool {
        unsafe { ffi::api().TT_CloseSoundOutputDevice(self.ptr) == 1 }
    }

    pub fn close_sound_duplex_devices(&self) -> bool {
        unsafe { ffi::api().TT_CloseSoundDuplexDevices(self.ptr) == 1 }
    }

    pub fn get_sound_input_level(&self) -> i32 {
        unsafe { ffi::api().TT_GetSoundInputLevel(self.ptr) }
    }

    pub fn set_sound_input_gain_level(&self, level: i32) -> bool {
        unsafe { ffi::api().TT_SetSoundInputGainLevel(self.ptr, level) == 1 }
    }

    pub fn get_sound_input_gain_level(&self) -> i32 {
        unsafe { ffi::api().TT_GetSoundInputGainLevel(self.ptr) }
    }

    pub fn set_sound_output_volume(&self, volume: i32) -> bool {
        unsafe { ffi::api().TT_SetSoundOutputVolume(self.ptr, volume) == 1 }
    }

    pub fn get_sound_output_volume(&self) -> i32 {
        unsafe { ffi::api().TT_GetSoundOutputVolume(self.ptr) }
    }

    pub fn set_sound_output_mute(&self, mute: bool) -> bool {
        unsafe { ffi::api().TT_SetSoundOutputMute(self.ptr, if mute { 1 } else { 0 }) == 1 }
    }

    pub fn set_user_mute(&self, user_id: UserId, stream_type: ffi::StreamType, mute: bool) -> bool {
        unsafe {
            ffi::api().TT_SetUserMute(self.ptr, user_id.0, stream_type, if mute { 1 } else { 0 })
                == 1
        }
    }

    pub fn set_user_audio_stream_buffer_size(
        &self,
        user_id: UserId,
        stream_type: ffi::StreamType,
        msec: i32,
    ) -> bool {
        unsafe {
            let st = stream_type as u32;
            ffi::api().TT_SetUserAudioStreamBufferSize(self.ptr, user_id.0, st, msec) == 1
        }
    }

    pub fn set_user_stopped_playback_delay(
        &self,
        user_id: UserId,
        stream_type: ffi::StreamType,
        msec: i32,
    ) -> bool {
        unsafe {
            ffi::api().TT_SetUserStoppedPlaybackDelay(self.ptr, user_id.0, stream_type, msec) == 1
        }
    }

    pub fn enable_voice_transmission(&self, enable: bool) -> bool {
        unsafe { ffi::api().TT_EnableVoiceTransmission(self.ptr, if enable { 1 } else { 0 }) == 1 }
    }

    pub fn enable_voice_activation(&self, enable: bool) -> bool {
        unsafe { ffi::api().TT_EnableVoiceActivation(self.ptr, if enable { 1 } else { 0 }) == 1 }
    }

    pub fn set_voice_activation_level(&self, level: i32) -> bool {
        unsafe { ffi::api().TT_SetVoiceActivationLevel(self.ptr, level) == 1 }
    }

    pub fn get_voice_activation_level(&self) -> i32 {
        unsafe { ffi::api().TT_GetVoiceActivationLevel(self.ptr) }
    }

    pub fn set_voice_activation_stop_delay(&self, delay: i32) -> bool {
        unsafe { ffi::api().TT_SetVoiceActivationStopDelay(self.ptr, delay) == 1 }
    }

    pub fn get_voice_activation_stop_delay(&self) -> i32 {
        unsafe { ffi::api().TT_GetVoiceActivationStopDelay(self.ptr) }
    }

    pub fn set_audio_preprocessor(&self, preprocessor: &AudioPreprocessor) -> bool {
        unsafe { ffi::api().TT_SetSoundInputPreprocessEx(self.ptr, &preprocessor.to_ffi()) == 1 }
    }

    pub fn get_audio_preprocessor(&self) -> Option<AudioPreprocessor> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::AudioPreprocessor>() };
        if unsafe { ffi::api().TT_GetSoundInputPreprocessEx(self.ptr, &mut raw) } == 1 {
            Some(AudioPreprocessor::from(raw))
        } else {
            None
        }
    }

    pub fn set_device_effects(&self, effects: &ffi::SoundDeviceEffects) -> bool {
        unsafe { ffi::api().TT_SetSoundDeviceEffects(self.ptr, effects) == 1 }
    }

    pub fn get_device_effects(&self) -> Option<ffi::SoundDeviceEffects> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::SoundDeviceEffects>() };
        if unsafe { ffi::api().TT_GetSoundDeviceEffects(self.ptr, &mut raw) } == 1 {
            Some(raw)
        } else {
            None
        }
    }

    pub fn enable_3d_sound(&self, enable: bool) -> bool {
        unsafe { ffi::api().TT_Enable3DSoundPositioning(self.ptr, if enable { 1 } else { 0 }) == 1 }
    }

    pub fn auto_position_users(&self) -> bool {
        unsafe { ffi::api().TT_AutoPositionUsers(self.ptr) == 1 }
    }

    pub fn set_user_position(
        &self,
        user_id: UserId,
        stream_type: ffi::StreamType,
        x: f32,
        y: f32,
        z: f32,
    ) -> bool {
        unsafe { ffi::api().TT_SetUserPosition(self.ptr, user_id.0, stream_type, x, y, z) == 1 }
    }

    pub fn set_user_stereo(
        &self,
        user_id: UserId,
        stream_type: ffi::StreamType,
        left: bool,
        right: bool,
    ) -> bool {
        unsafe {
            ffi::api().TT_SetUserStereo(self.ptr, user_id.0, stream_type, left as i32, right as i32)
                == 1
        }
    }

    pub fn enable_audio_block_event(&self, user_id: UserId, types: u32, enable: bool) -> bool {
        unsafe {
            ffi::api().TT_EnableAudioBlockEvent(
                self.ptr,
                user_id.0,
                types,
                if enable { 1 } else { 0 },
            ) == 1
        }
    }

    pub fn set_user_jitter_control(&self, user_id: UserId, delay: i32) -> bool {
        let mut cfg = unsafe { std::mem::zeroed::<ffi::JitterConfig>() };
        cfg.nFixedDelayMSec = delay;
        unsafe {
            ffi::api().TT_SetUserJitterControl(
                self.ptr,
                user_id.0,
                ffi::StreamType::STREAMTYPE_VOICE,
                &cfg,
            ) == 1
        }
    }

    pub fn enable_audio_block_event_ex(
        &self,
        user_id: UserId,
        types: u32,
        format: Option<&ffi::AudioFormat>,
        enable: bool,
    ) -> bool {
        let fmt_ptr = format.map_or(std::ptr::null(), |f| f);
        unsafe {
            ffi::api().TT_EnableAudioBlockEventEx(
                self.ptr,
                user_id.0,
                types,
                fmt_ptr,
                if enable { 1 } else { 0 },
            ) == 1
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn start_sound_loopback_test_ex(
        &self,
        in_id: i32,
        out_id: i32,
        rate: i32,
        chans: i32,
        duplex: bool,
        preprocessor: Option<&AudioPreprocessor>,
        effects: Option<&ffi::SoundDeviceEffects>,
    ) -> *mut ffi::TTSoundLoop {
        let prep_raw = preprocessor.map(|p| p.to_ffi());
        let prep_ptr = prep_raw.as_ref().map_or(std::ptr::null(), |p| p);
        let eff_ptr = effects.map_or(std::ptr::null(), |e| e);
        unsafe {
            ffi::api().TT_StartSoundLoopbackTestEx(
                in_id,
                out_id,
                rate,
                chans,
                if duplex { 1 } else { 0 },
                prep_ptr,
                eff_ptr,
            )
        }
    }

    pub fn get_user_jitter_control(&self, user_id: UserId) -> Option<crate::types::JitterConfig> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::JitterConfig>() };
        if unsafe {
            ffi::api().TT_GetUserJitterControl(
                self.ptr,
                user_id.0,
                ffi::StreamType::STREAMTYPE_VOICE,
                &mut raw,
            ) == 1
        } {
            Some(crate::types::JitterConfig::from(raw))
        } else {
            None
        }
    }

    pub fn acquire_user_audio_block(
        &self,
        types: u32,
        user_id: UserId,
    ) -> Option<*mut ffi::AudioBlock> {
        unsafe {
            let ptr = ffi::api().TT_AcquireUserAudioBlock(self.ptr, types, user_id.0);
            if ptr.is_null() { None } else { Some(ptr) }
        }
    }

    pub fn insert_audio_block(&self, block: &ffi::AudioBlock) -> bool {
        unsafe { ffi::api().TT_InsertAudioBlock(self.ptr, block) == 1 }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn release_user_audio_block(&self, block: *mut ffi::AudioBlock) -> bool {
        if block.is_null() {
            return false;
        }
        unsafe { ffi::api().TT_ReleaseUserAudioBlock(self.ptr, block) == 1 }
    }

    pub fn set_user_volume(
        &self,
        user_id: UserId,
        stream_type: ffi::StreamType,
        volume: i32,
    ) -> bool {
        unsafe { ffi::api().TT_SetUserVolume(self.ptr, user_id.0, stream_type, volume) == 1 }
    }

    pub fn start_sound_loopback_test(
        &self,
        in_id: i32,
        out_id: i32,
        rate: i32,
        chans: i32,
        duplex: bool,
    ) -> *mut ffi::TTSoundLoop {
        unsafe {
            ffi::api().TT_StartSoundLoopbackTest(
                in_id,
                out_id,
                rate,
                chans,
                if duplex { 1 } else { 0 },
                std::ptr::null(),
            )
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn close_sound_loopback_test(&self, loopback: *mut ffi::TTSoundLoop) -> bool {
        if loopback.is_null() {
            return false;
        }
        unsafe { ffi::api().TT_CloseSoundLoopbackTest(loopback) == 1 }
    }
}
