//! Low-level media/transport configuration and payload types split by
//! domain.
//!
//! The module was previously a single 458-line file bundling jitter
//! control, audio/video formats, TLS encryption context, keep-alive,
//! abuse prevention, desktop input, and SDK error payloads in one
//! place. It is now directory-first and split by domain per the
//! AGENTS.md guideline:
//!
//! * `audio.rs` - [`JitterConfig`], [`AudioFormat`],
//!   [`AudioInputProgress`].
//! * `video.rs` - [`VideoFormat`], [`VideoCodec`], [`VideoFrame`].
//! * `transport.rs` - [`EncryptionContext`], [`ClientKeepAlive`],
//!   [`AbusePrevention`].
//! * `desktop.rs` - [`DesktopInput`].
//! * `error.rs` - [`ErrorMessage`].
//!
//! All items are re-exported so existing call sites that use the
//! glob re-export from `types::entities` (see
//! `types/entities/mod.rs`, `pub use media_common::*;`) keep
//! working unchanged.

mod audio;
mod desktop;
mod error;
mod transport;
mod video;

pub use audio::{AudioFormat, AudioInputProgress, JitterConfig};
pub use desktop::DesktopInput;
pub use error::ErrorMessage;
pub use transport::{AbusePrevention, ClientKeepAlive, EncryptionContext};
pub use video::{VideoCodec, VideoFormat, VideoFrame};
