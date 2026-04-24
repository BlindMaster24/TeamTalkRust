//! Dialog status and timeout-policy enums plus their wire encodings.

/// Current lifecycle of a [`super::state::DialogState`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogStatus {
    /// Dialog is active and advancing on each matching event.
    Active,
    /// Dialog is paused and ignores events until explicitly resumed.
    Paused,
}

/// What [`super::machine::DialogMachine::current_live`] should do when
/// a dialog's deadline has elapsed.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogTimeoutPolicy {
    /// Drop the state entirely (default).
    Clear,
    /// Move to [`DialogStatus::Paused`] and clear the deadline.
    Pause,
}

impl DialogTimeoutPolicy {
    pub(super) fn encode(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Pause => "pause",
        }
    }

    pub(super) fn decode(raw: &str) -> Option<Self> {
        match raw {
            "clear" => Some(Self::Clear),
            "pause" => Some(Self::Pause),
            _ => None,
        }
    }
}

impl DialogStatus {
    pub(super) fn encode(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Paused => "paused",
        }
    }

    pub(super) fn decode(raw: &str) -> Option<Self> {
        match raw {
            "active" => Some(Self::Active),
            "paused" => Some(Self::Paused),
            _ => None,
        }
    }
}
