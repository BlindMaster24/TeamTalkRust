//! System-level and licensing namespace.
use super::define_namespace;

define_namespace!(SystemNamespace);

impl SystemNamespace {
    /// Returns the TeamTalk SDK version string.
    pub fn version(&self) -> String {
        crate::client::Client::get_version()
    }

    /// Sets license information for the SDK.
    pub fn set_license(&self, name: &str, key: &str) -> bool {
        self.client.set_license(name, key)
    }

    #[cfg(windows)]
    /// Returns whether the Windows firewall is enabled.
    pub fn is_firewall_enabled(&self) -> bool {
        self.client.is_firewall_enabled()
    }

    #[cfg(windows)]
    /// Adds a firewall exception.
    pub fn add_firewall_exception(&self, name: &str, exe_path: &str) -> bool {
        self.client.add_firewall_exception(name, exe_path)
    }

    #[cfg(windows)]
    /// Removes a firewall exception.
    pub fn remove_firewall_exception(&self, exe_path: &str) -> bool {
        self.client.remove_firewall_exception(exe_path)
    }
}

#[cfg(feature = "async")]
use super::define_async_namespace;

#[cfg(feature = "async")]
define_async_namespace!(AsyncSystemNamespace);

#[cfg(feature = "async")]
impl AsyncSystemNamespace {
    /// Returns the TeamTalk SDK version string.
    pub fn version(&self) -> String {
        crate::client::Client::get_version()
    }

    /// Sets license information for the SDK.
    pub fn set_license(&self, name: &str, key: &str) -> bool {
        self.client.set_license(name, key)
    }
}
