use super::middleware::{RequireClientRightsAll, RequireClientRightsAny};
use crate::types::UserRights;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permissions {
    rights: UserRights,
}

impl Permissions {
    #[allow(clippy::must_use_candidate)]
    pub fn new(rights: UserRights) -> Self {
        Self { rights }
    }

    #[must_use]
    pub fn rights(self) -> UserRights {
        self.rights
    }

    #[must_use]
    pub fn any(self) -> RequireClientRightsAny {
        RequireClientRightsAny::new(self.rights)
    }

    #[must_use]
    pub fn all(self) -> RequireClientRightsAll {
        RequireClientRightsAll::new(self.rights)
    }

    #[must_use]
    pub fn moderator() -> Self {
        Self::new(UserRights::KICK_USERS | UserRights::BAN_USERS | UserRights::MOVE_USERS)
    }

    #[must_use]
    pub fn file_manager() -> Self {
        Self::new(UserRights::UPLOAD_FILES | UserRights::DOWNLOAD_FILES)
    }

    #[must_use]
    pub fn channel_admin() -> Self {
        Self::new(
            UserRights::MODIFY_CHANNELS | UserRights::MOVE_USERS | UserRights::OPERATOR_ENABLE,
        )
    }

    #[must_use]
    pub fn media_sender() -> Self {
        Self::new(
            UserRights::TRANSMIT_VOICE
                | UserRights::TRANSMIT_VIDEOCAPTURE
                | UserRights::TRANSMIT_MEDIAFILE,
        )
    }

    #[must_use]
    pub fn desktop_controller() -> Self {
        Self::new(UserRights::TRANSMIT_DESKTOP | UserRights::TRANSMIT_DESKTOPINPUT)
    }

    #[must_use]
    pub fn server_admin() -> Self {
        Self::new(
            Self::moderator().rights()
                | UserRights::MODIFY_CHANNELS
                | UserRights::UPDATE_SERVERPROPERTIES
                | UserRights::OPERATOR_ENABLE,
        )
    }

    #[must_use]
    pub fn admin() -> Self {
        Self::server_admin()
    }
}
