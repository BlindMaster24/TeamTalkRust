use super::*;

impl Client {
    fn disconnect_connect_barrier_if_needed(&self) {
        if matches!(self.connection_state(), ConnectionState::Disconnected) {
            let _ = self.disconnect();
        }
    }

    fn ensure_connect_allowed(&self) -> Result<(), Error> {
        ensure_connect_not_busy(self.has_connection_flags())
    }

    fn disconnect_reconnect_barrier(&self) -> Result<(), Error> {
        let _ = self.disconnect();
        if self.has_connection_flags() {
            return Err(Error::CommandFailed {
                code: -1,
                message: "Reconnect barrier failed: client still has connection flags".to_string(),
            });
        }
        Ok(())
    }

    pub fn connect_remember(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
    ) -> Result<(), crate::events::Error> {
        self.set_reconnect_params(ConnectParamsOwned::new(host, tcp, udp, encrypted));
        self.connect(host, tcp, udp, encrypted)
    }

    /// Connects using the provided parameters.
    pub fn connect_with_params(
        &self,
        params: &ConnectParamsOwned,
    ) -> Result<(), crate::events::Error> {
        self.connect(&params.host, params.tcp, params.udp, params.encrypted)
    }

    pub fn connect_from_env(&self) -> Result<(), crate::events::Error> {
        let params = ConnectParamsOwned::from_env();
        self.connect_remember(&params.host, params.tcp, params.udp, params.encrypted)
    }

    /// Connects to a TeamTalk server.
    pub fn connect(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
    ) -> Result<(), crate::events::Error> {
        self.ensure_connect_allowed()?;
        self.disconnect_connect_barrier_if_needed();
        let ok = self
            .backend()
            .connect(self.ptr.0, host, tcp, udp, encrypted);
        if ok {
            self.set_connection_state(ConnectionState::Connecting);
            Ok(())
        } else {
            Err(crate::events::Error::ConnectFailed)
        }
    }

    /// Connects without encryption.
    pub fn connect_auto(&self, host: &str, tcp: i32, udp: i32) -> Result<(), crate::events::Error> {
        self.connect(host, tcp, udp, false)
    }

    /// Returns true when the client is connected.
    pub fn is_connected(&self) -> bool {
        let flags = self.backend().get_flags(self.ptr.0);
        (flags & ffi::ClientFlag::CLIENT_CONNECTED as u32) != 0
    }

    /// Returns true when the client is attempting to connect.
    pub fn is_connecting(&self) -> bool {
        let flags = self.backend().get_flags(self.ptr.0);
        (flags & ffi::ClientFlag::CLIENT_CONNECTING as u32) != 0
    }

    pub(crate) fn has_connection_flags(&self) -> bool {
        self.get_flags().has(crate::types::ClientFlags::CONNECTION)
    }

    /// Handles reconnect logic using provided parameters.
    pub fn handle_reconnect(&self, params: &ConnectParams, handler: &mut ReconnectHandler) -> bool {
        if !handler.can_attempt() {
            return true;
        }

        if self.disconnect_reconnect_barrier().is_err() {
            return true;
        }

        handler.record_attempt();
        let _ = self.connect(params.host, params.tcp, params.udp, params.encrypted);
        true
    }

    /// Reconnects to a TeamTalk server using a disconnect barrier first.
    pub fn reconnect(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
    ) -> Result<(), crate::events::Error> {
        self.disconnect_reconnect_barrier()?;
        self.connect(host, tcp, udp, encrypted)
    }

    /// Reconnects using the provided parameters.
    pub fn reconnect_with_params(
        &self,
        params: &ConnectParamsOwned,
    ) -> Result<(), crate::events::Error> {
        self.reconnect(&params.host, params.tcp, params.udp, params.encrypted)
    }

    /// Connects with a custom system id string.
    pub fn connect_sys_id(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
        sys_id: &str,
    ) -> Result<(), crate::events::Error> {
        self.ensure_connect_allowed()?;
        self.disconnect_connect_barrier_if_needed();
        let ok = self
            .backend()
            .connect_sys_id(self.ptr.0, host, tcp, udp, encrypted, sys_id);
        if ok {
            self.set_connection_state(ConnectionState::Connecting);
            Ok(())
        } else {
            Err(crate::events::Error::ConnectFailed)
        }
    }

    /// Reconnects with a custom system id string.
    pub fn reconnect_sys_id(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        encrypted: bool,
        sys_id: &str,
    ) -> Result<(), crate::events::Error> {
        self.disconnect_reconnect_barrier()?;
        self.connect_sys_id(host, tcp, udp, encrypted, sys_id)
    }

    /// Connects with a custom bind IP.
    pub fn connect_ex(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        bind_ip: &str,
        encrypted: bool,
    ) -> Result<(), crate::events::Error> {
        self.ensure_connect_allowed()?;
        self.disconnect_connect_barrier_if_needed();
        let ok = self
            .backend()
            .connect_ex(self.ptr.0, host, tcp, udp, bind_ip, encrypted);
        if ok {
            self.set_connection_state(ConnectionState::Connecting);
            Ok(())
        } else {
            Err(crate::events::Error::ConnectFailed)
        }
    }

    /// Reconnects with a custom bind IP.
    pub fn reconnect_ex(
        &self,
        host: &str,
        tcp: i32,
        udp: i32,
        bind_ip: &str,
        encrypted: bool,
    ) -> Result<(), crate::events::Error> {
        self.disconnect_reconnect_barrier()?;
        self.connect_ex(host, tcp, udp, bind_ip, encrypted)
    }

    /// Disconnects from the server.
    pub fn disconnect(&self) -> Result<(), crate::events::Error> {
        if self.backend().disconnect(self.ptr.0) {
            self.set_connection_state(ConnectionState::Disconnected);
            Ok(())
        } else {
            Err(crate::events::Error::CommandFailed {
                code: -1,
                message: "Disconnect failed".to_string(),
            })
        }
    }
}
