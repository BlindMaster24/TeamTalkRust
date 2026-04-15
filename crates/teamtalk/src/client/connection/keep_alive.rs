use super::{Client, CommandId, ffi, validate_client_keep_alive};

impl Client {
    pub fn set_client_keep_alive(
        &self,
        keep_alive: &crate::types::ClientKeepAlive,
    ) -> Result<(), crate::events::Error> {
        validate_client_keep_alive(keep_alive)?;
        if unsafe { ffi::api().TT_SetClientKeepAlive(self.ptr.0, &keep_alive.to_ffi()) == 1 } {
            Ok(())
        } else {
            Err(crate::events::Error::CommandFailed {
                code: -1,
                message: "Set keep-alive failed".to_string(),
            })
        }
    }

    /// Sets keep-alive parameters and issues a ping to restart timers.
    pub fn set_client_keep_alive_and_ping(
        &self,
        keep_alive: &crate::types::ClientKeepAlive,
    ) -> Result<CommandId, crate::events::Error> {
        self.set_client_keep_alive(keep_alive)?;
        let cmd_id = self.ping();
        if cmd_id.is_ok() {
            Ok(cmd_id)
        } else {
            Err(crate::events::Error::CommandFailed {
                code: -1,
                message: "Ping failed".to_string(),
            })
        }
    }

    /// Returns client keep-alive parameters.
    pub fn get_client_keep_alive(&self) -> Option<crate::types::ClientKeepAlive> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::ClientKeepAlive>() };
        if unsafe { ffi::api().TT_GetClientKeepAlive(self.ptr.0, &raw mut raw) } == 1 {
            Some(crate::types::ClientKeepAlive::from(raw))
        } else {
            None
        }
    }
}
