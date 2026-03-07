use super::middleware::{RequireClientRightsAll, RequireClientRightsAny};
use crate::types::UserRights;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permissions {
    rights: UserRights,
}

impl Permissions {
    pub fn new(rights: UserRights) -> Self {
        Self { rights }
    }

    pub fn rights(self) -> UserRights {
        self.rights
    }

    pub fn any(self) -> RequireClientRightsAny {
        RequireClientRightsAny::new(self.rights)
    }

    pub fn all(self) -> RequireClientRightsAll {
        RequireClientRightsAll::new(self.rights)
    }

    pub fn moderator() -> Self {
        Self::new(UserRights::KICK_USERS | UserRights::BAN_USERS | UserRights::MOVE_USERS)
    }

    pub fn admin() -> Self {
        Self::new(
            Self::moderator().rights()
                | UserRights::MODIFY_CHANNELS
                | UserRights::UPDATE_SERVERPROPERTIES
                | UserRights::OPERATOR_ENABLE,
        )
    }
}
