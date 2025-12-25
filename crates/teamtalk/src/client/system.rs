use super::Client;
use crate::utils::ToTT;
use teamtalk_sys as ffi;

impl Client {
    pub fn get_version() -> String {
        unsafe {
            let ptr = ffi::api().TT_GetVersion();
            crate::utils::strings::from_tt(ptr)
        }
    }

    pub fn set_license(&self, name: &str, key: &str) -> bool {
        unsafe { ffi::api().TT_SetLicenseInformation(name.tt().as_ptr(), key.tt().as_ptr()) == 1 }
    }

    #[cfg(windows)]
    pub fn is_firewall_enabled(&self) -> bool {
        unsafe { ffi::api().TT_Firewall_IsEnabled() == 1 }
    }

    #[cfg(windows)]
    pub fn firewall_app_exception_exists(&self, exe_path: &str) -> bool {
        unsafe { ffi::api().TT_Firewall_AppExceptionExists(exe_path.tt().as_ptr()) == 1 }
    }

    #[cfg(windows)]
    pub fn enable_firewall(&self, enable: bool) -> bool {
        unsafe { ffi::api().TT_Firewall_Enable(if enable { 1 } else { 0 }) == 1 }
    }

    #[cfg(windows)]
    pub fn add_firewall_exception(&self, name: &str, exe_path: &str) -> bool {
        unsafe {
            ffi::api().TT_Firewall_AddAppException(name.tt().as_ptr(), exe_path.tt().as_ptr()) == 1
        }
    }

    #[cfg(windows)]
    pub fn remove_firewall_exception(&self, exe_path: &str) -> bool {
        unsafe { ffi::api().TT_Firewall_RemoveAppException(exe_path.tt().as_ptr()) == 1 }
    }
}
