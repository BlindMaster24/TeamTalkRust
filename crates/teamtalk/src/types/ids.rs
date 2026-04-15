#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Strongly typed user id.
pub struct UserId(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Strongly typed channel id.
pub struct ChannelId(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Strongly typed remote file id.
pub struct FileId(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Strongly typed transfer id.
pub struct TransferId(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Strongly typed command id.
pub struct CommandId(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Strongly typed client id.
pub struct ClientId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Strongly typed sound device id.
pub struct SoundDeviceId(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Strongly typed hotkey id.
pub struct HotkeyId(pub i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Strongly typed playback session id.
pub struct PlaybackSessionId(pub i32);

/// Reserved local user id.
pub const LOCAL_USER_ID: UserId = UserId(0);
/// Reserved local transmit user id.
pub const LOCAL_TX_USER_ID: UserId = UserId(4098);
/// Reserved muxed audio user id.
pub const MUXED_USER_ID: UserId = UserId(4097);

macro_rules! impl_id_type {
    ($ty:ident, $inner:ty) => {
        impl $ty {
            #[must_use]
            pub fn raw(self) -> $inner {
                self.0
            }
        }

        impl From<$inner> for $ty {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl PartialEq<$inner> for $ty {
            fn eq(&self, other: &$inner) -> bool {
                self.0 == *other
            }
        }

        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

impl_id_type!(UserId, i32);
impl_id_type!(ChannelId, i32);
impl_id_type!(FileId, i32);
impl_id_type!(TransferId, i32);
impl_id_type!(CommandId, i32);
impl_id_type!(ClientId, u64);
impl_id_type!(SoundDeviceId, i32);
impl_id_type!(HotkeyId, i32);
impl_id_type!(PlaybackSessionId, i32);

impl CommandId {
    /// Command ID indicating the command was rejected.
    pub const ZERO: Self = Self(0);

    #[must_use]
    pub fn is_ok(self) -> bool {
        self.0 > 0
    }

    #[must_use]
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }
}
