//! Recording APIs for channels and streams.

mod options;
mod raw;
mod session;

pub use options::{RecordingOptions, RecordingTarget};
pub use session::RecordingSession;

pub use raw::RecordSession;
