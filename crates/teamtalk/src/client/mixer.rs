use super::Client;
use crate::types::TT_STRLEN;
use crate::utils::strings::tt_buf;
use teamtalk_sys as ffi;

#[cfg(windows)]
impl Client {
    pub fn get_mixer_count(&self) -> i32 {
        unsafe { ffi::api().TT_Mixer_GetMixerCount() }
    }

    pub fn get_mixer_name(&self, index: i32) -> String {
        let mut buf = tt_buf::<TT_STRLEN>();
        unsafe {
            if ffi::api().TT_Mixer_GetMixerName(index, buf.as_mut_ptr()) == 1 {
                crate::utils::strings::to_string(&buf)
            } else {
                String::new()
            }
        }
    }

    pub fn set_mixer_output_mute(&self, wave_id: i32, control: ffi::MixerControl, mute: bool) -> bool {
        unsafe { ffi::api().TT_Mixer_SetWaveOutMute(wave_id, control, mute as i32) == 1 }
    }

    pub fn get_mixer_output_mute(&self, wave_id: i32, control: ffi::MixerControl) -> i32 {
        unsafe { ffi::api().TT_Mixer_GetWaveOutMute(wave_id, control) }
    }

    pub fn set_mixer_output_volume(&self, wave_id: i32, control: ffi::MixerControl, vol: i32) -> bool {
        unsafe { ffi::api().TT_Mixer_SetWaveOutVolume(wave_id, control, vol) == 1 }
    }

    pub fn get_mixer_output_volume(&self, wave_id: i32, control: ffi::MixerControl) -> i32 {
        unsafe { ffi::api().TT_Mixer_GetWaveOutVolume(wave_id, control) }
    }

    pub fn set_mixer_input_mute(&self, wave_id: i32, mute: bool) -> bool {
        unsafe { ffi::api().TT_Mixer_SetWaveInMute(wave_id, mute as i32) == 1 }
    }

    pub fn get_mixer_input_mute(&self, wave_id: i32) -> i32 {
        unsafe { ffi::api().TT_Mixer_GetWaveInMute(wave_id) }
    }

    pub fn get_mixer_wave_in_name(&self, wave_id: i32) -> String {
        let mut buf = tt_buf::<TT_STRLEN>();
        unsafe {
            if ffi::api().TT_Mixer_GetWaveInName(wave_id, buf.as_mut_ptr()) == 1 {
                crate::utils::strings::to_string(&buf)
            } else {
                String::new()
            }
        }
    }

    pub fn get_mixer_wave_out_name(&self, wave_id: i32) -> String {
        let mut buf = tt_buf::<TT_STRLEN>();
        unsafe {
            if ffi::api().TT_Mixer_GetWaveOutName(wave_id, buf.as_mut_ptr()) == 1 {
                crate::utils::strings::to_string(&buf)
            } else {
                String::new()
            }
        }
    }

    pub fn set_mixer_input_boost(&self, wave_id: i32, enable: bool) -> bool {
        unsafe { ffi::api().TT_Mixer_SetWaveInBoost(wave_id, enable as i32) == 1 }
    }

    pub fn get_mixer_input_boost(&self, wave_id: i32) -> i32 {
        unsafe { ffi::api().TT_Mixer_GetWaveInBoost(wave_id) }
    }

    pub fn get_mixer_input_control_count(&self, wave_id: i32) -> i32 {
        unsafe { ffi::api().TT_Mixer_GetWaveInControlCount(wave_id) }
    }

    pub fn get_mixer_input_control_name(&self, wave_id: i32, index: i32) -> String {
        let mut buf = tt_buf::<TT_STRLEN>();
        unsafe {
            if ffi::api().TT_Mixer_GetWaveInControlName(wave_id, index, buf.as_mut_ptr()) == 1 {
                crate::utils::strings::to_string(&buf)
            } else {
                String::new()
            }
        }
    }

    pub fn set_mixer_input_control_selected(&self, wave_id: i32, index: i32) -> bool {
        unsafe { ffi::api().TT_Mixer_SetWaveInControlSelected(wave_id, index) == 1 }
    }

    pub fn get_mixer_input_control_selected(&self, wave_id: i32, index: i32) -> bool {
        unsafe { ffi::api().TT_Mixer_GetWaveInControlSelected(wave_id, index) == 1 }
    }
}
