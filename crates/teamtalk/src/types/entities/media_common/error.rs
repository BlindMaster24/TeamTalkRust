//! SDK error message payload.

use teamtalk_sys as ffi;

/// SDK error message payload.
#[non_exhaustive]
#[derive(Debug, Clone, Default)]
pub struct ErrorMessage {
    /// Numeric SDK error code.
    pub code: i32,
    /// Human-readable SDK error message.
    pub message: String,
}

impl From<ffi::ClientErrorMsg> for ErrorMessage {
    fn from(e: ffi::ClientErrorMsg) -> Self {
        Self {
            code: e.nErrorNo,
            message: crate::utils::strings::to_string(&e.szErrorMsg),
        }
    }
}
