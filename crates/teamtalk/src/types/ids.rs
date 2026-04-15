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

/// Reserved local user id.
pub const LOCAL_USER_ID: UserId = UserId(0);
/// Reserved local transmit user id.
pub const LOCAL_TX_USER_ID: UserId = UserId(4098);
/// Reserved muxed audio user id.
pub const MUXED_USER_ID: UserId = UserId(4097);

impl CommandId {
    /// Command ID indicating the command was rejected.
    pub const ZERO: Self = Self(0);

    pub fn raw(self) -> i32 {
        self.0
    }

    pub fn is_ok(self) -> bool {
        self.0 > 0
    }

    pub fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl From<i32> for CommandId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl PartialEq<i32> for CommandId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}
