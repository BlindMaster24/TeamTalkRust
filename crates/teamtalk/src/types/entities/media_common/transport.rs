//! Transport-layer and security configuration types (TLS, keep-alive,
//! abuse prevention).

use teamtalk_sys as ffi;

/// TLS encryption context settings.
#[non_exhaustive]
#[derive(Debug, Clone, Default)]
pub struct EncryptionContext {
    /// Path to the certificate file.
    pub cert_file: String,
    /// Path to the private key file.
    pub key_file: String,
    /// Path to the CA certificate bundle file.
    pub ca_file: String,
    /// Path to the CA certificate directory.
    pub ca_dir: String,
    /// Whether to verify the peer certificate.
    pub verify_peer: bool,
    /// Whether to verify the client certificate once per session.
    pub verify_client_once: bool,
    /// Maximum certificate chain depth to verify.
    pub verify_depth: i32,
}

impl EncryptionContext {
    /// Creates a new encryption context.
    pub fn new(
        cert_file: impl Into<String>,
        key_file: impl Into<String>,
        ca_file: impl Into<String>,
        ca_dir: impl Into<String>,
        verify_peer: bool,
        verify_client_once: bool,
        verify_depth: i32,
    ) -> Self {
        Self {
            cert_file: cert_file.into(),
            key_file: key_file.into(),
            ca_file: ca_file.into(),
            ca_dir: ca_dir.into(),
            verify_peer,
            verify_client_once,
            verify_depth,
        }
    }
    /// Converts to the raw `TeamTalk` struct.
    #[must_use]
    pub fn to_ffi(&self) -> ffi::EncryptionContext {
        let mut raw = ffi::EncryptionContext::default();
        let cert = crate::utils::ToTT::tt(&self.cert_file);
        let key = crate::utils::ToTT::tt(&self.key_file);
        let ca = crate::utils::ToTT::tt(&self.ca_file);
        let cadir = crate::utils::ToTT::tt(&self.ca_dir);
        unsafe {
            let n_len = cert.len().min(511);
            std::ptr::copy_nonoverlapping(cert.as_ptr(), raw.szCertificateFile.as_mut_ptr(), n_len);
            let k_len = key.len().min(511);
            std::ptr::copy_nonoverlapping(key.as_ptr(), raw.szPrivateKeyFile.as_mut_ptr(), k_len);
            let ca_len = ca.len().min(511);
            std::ptr::copy_nonoverlapping(ca.as_ptr(), raw.szCAFile.as_mut_ptr(), ca_len);
            let cadir_len = cadir.len().min(511);
            std::ptr::copy_nonoverlapping(cadir.as_ptr(), raw.szCADir.as_mut_ptr(), cadir_len);
        }
        raw.bVerifyPeer = i32::from(self.verify_peer);
        raw.bVerifyClientOnce = i32::from(self.verify_client_once);
        raw.nVerifyDepth = self.verify_depth;
        raw
    }
}

/// Keep-alive configuration for client connections.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct ClientKeepAlive {
    /// Connection-lost threshold in milliseconds.
    pub lost_ms: i32,
    /// TCP keep-alive interval in milliseconds.
    pub tcp_interval_ms: i32,
    /// UDP keep-alive interval in milliseconds.
    pub udp_interval_ms: i32,
    /// UDP keep-alive retransmit interval in milliseconds.
    pub udp_rtx_ms: i32,
    /// UDP connect retransmit interval in milliseconds.
    pub udp_connect_rtx_ms: i32,
    /// UDP connect timeout in milliseconds.
    pub udp_timeout_ms: i32,
}

impl From<ffi::ClientKeepAlive> for ClientKeepAlive {
    fn from(c: ffi::ClientKeepAlive) -> Self {
        Self {
            lost_ms: c.nConnectionLostMSec,
            tcp_interval_ms: c.nTcpKeepAliveIntervalMSec,
            udp_interval_ms: c.nUdpKeepAliveIntervalMSec,
            udp_rtx_ms: c.nUdpKeepAliveRTXMSec,
            udp_connect_rtx_ms: c.nUdpConnectRTXMSec,
            udp_timeout_ms: c.nUdpConnectTimeoutMSec,
        }
    }
}

impl ClientKeepAlive {
    /// Creates a new keep-alive configuration.
    #[must_use]
    pub fn new(
        lost_ms: i32,
        tcp_interval_ms: i32,
        udp_interval_ms: i32,
        udp_rtx_ms: i32,
        udp_connect_rtx_ms: i32,
        udp_timeout_ms: i32,
    ) -> Self {
        Self {
            lost_ms,
            tcp_interval_ms,
            udp_interval_ms,
            udp_rtx_ms,
            udp_connect_rtx_ms,
            udp_timeout_ms,
        }
    }
    /// Converts to the raw `TeamTalk` struct.
    #[must_use]
    pub fn to_ffi(&self) -> ffi::ClientKeepAlive {
        ffi::ClientKeepAlive {
            nConnectionLostMSec: self.lost_ms,
            nTcpKeepAliveIntervalMSec: self.tcp_interval_ms,
            nUdpKeepAliveIntervalMSec: self.udp_interval_ms,
            nUdpKeepAliveRTXMSec: self.udp_rtx_ms,
            nUdpConnectRTXMSec: self.udp_connect_rtx_ms,
            nUdpConnectTimeoutMSec: self.udp_timeout_ms,
        }
    }
}

/// Abuse prevention configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub struct AbusePrevention {
    /// Maximum number of commands permitted within the interval.
    pub commands_limit: i32,
    /// Sliding window for the command rate limit, in milliseconds.
    pub commands_interval_ms: i32,
}

impl From<ffi::AbusePrevention> for AbusePrevention {
    fn from(a: ffi::AbusePrevention) -> Self {
        Self {
            commands_limit: a.nCommandsLimit,
            commands_interval_ms: a.nCommandsIntervalMSec,
        }
    }
}

impl AbusePrevention {
    /// Creates a new abuse prevention configuration.
    #[must_use]
    pub fn new(commands_limit: i32, commands_interval_ms: i32) -> Self {
        Self {
            commands_limit,
            commands_interval_ms,
        }
    }
    /// Converts to the raw `TeamTalk` struct.
    #[must_use]
    pub fn to_ffi(&self) -> ffi::AbusePrevention {
        ffi::AbusePrevention {
            nCommandsLimit: self.commands_limit,
            nCommandsIntervalMSec: self.commands_interval_ms,
        }
    }
}
