//! Unified logging interface for the TeamTalk SDK.

#[cfg(feature = "logging")]
pub use tracing::{Span, debug, error, info, info_span, trace, warn};

#[cfg(not(feature = "logging"))]
mod stubs {
    #[macro_export]
    macro_rules! sdk_info {
        ($($arg:tt)*) => {};
    }
    #[macro_export]
    macro_rules! sdk_warn {
        ($($arg:tt)*) => {};
    }
    #[macro_export]
    macro_rules! sdk_error {
        ($($arg:tt)*) => {};
    }
    #[macro_export]
    macro_rules! sdk_debug {
        ($($arg:tt)*) => {};
    }
    #[macro_export]
    macro_rules! sdk_trace {
        ($($arg:tt)*) => {};
    }
    #[macro_export]
    macro_rules! sdk_info_span {
        ($($arg:tt)*) => {
            $crate::logging::span_stub::SpanStub
        };
    }
}

#[cfg(not(feature = "logging"))]
pub use crate::{
    sdk_debug as debug, sdk_error as error, sdk_info as info, sdk_info_span as info_span,
    sdk_trace as trace, sdk_warn as warn,
};

#[cfg(not(feature = "logging"))]
pub mod span_stub {
    pub struct SpanStub;
    impl SpanStub {
        pub fn entered(&self) -> () {}
    }
}

// Re-export event helper
#[cfg(feature = "logging")]
pub fn event(event: &crate::events::Event, message: &crate::client::Message) {
    tracing::debug!(?event, source = message.source());
}

#[cfg(not(feature = "logging"))]
pub fn event(_event: &crate::events::Event, _message: &crate::client::Message) {}
