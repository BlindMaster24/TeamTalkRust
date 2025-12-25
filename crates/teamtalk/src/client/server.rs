use super::Client;
use crate::types::{ChannelId, ClientStatistics, ServerProperties, User, UserId};
use crate::utils::ToTT;
use teamtalk_sys as ffi;

impl Client {
    pub fn get_server_properties(&self) -> Option<ServerProperties> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::ServerProperties>() };
        if unsafe { ffi::api().TT_GetServerProperties(self.ptr, &mut raw) } == 1 {
            Some(ServerProperties::from(raw))
        } else {
            None
        }
    }

    pub fn get_server_users(&self) -> Vec<User> {
        let mut count: i32 = 0;
        unsafe {
            ffi::api().TT_GetServerUsers(self.ptr, std::ptr::null_mut(), &mut count);
            let mut users = vec![std::mem::zeroed::<ffi::User>(); count as usize];
            if ffi::api().TT_GetServerUsers(self.ptr, users.as_mut_ptr(), &mut count) == 1 {
                users.into_iter().map(User::from).collect()
            } else {
                vec![]
            }
        }
    }

    pub fn ban_ip_address(&self, ip: &str, ban_type: i32) -> i32 {
        unsafe { ffi::api().TT_DoBanIPAddress(self.ptr, ip.tt().as_ptr(), ban_type) }
    }

    pub fn get_client_statistics(&self) -> Option<ClientStatistics> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::ClientStatistics>() };
        if unsafe { ffi::api().TT_GetClientStatistics(self.ptr, &mut raw) } == 1 {
            Some(ClientStatistics::from(raw))
        } else {
            None
        }
    }

    pub fn list_bans(&self, channel_id: ChannelId, index: i32, count: i32) -> i32 {
        unsafe { ffi::api().TT_DoListBans(self.ptr, channel_id.0, index, count) }
    }

    pub fn update_server_properties(&self, props: &ServerProperties) -> i32 {
        unsafe { ffi::api().TT_DoUpdateServer(self.ptr, &props.to_ffi()) }
    }

    pub fn save_config(&self) -> i32 {
        unsafe { ffi::api().TT_DoSaveConfig(self.ptr) }
    }

    pub fn root_channel_id(&self) -> ChannelId {
        ChannelId(unsafe { ffi::api().TT_GetRootChannelID(self.ptr) })
    }

    pub fn query_server_stats(&self) -> i32 {
        unsafe { ffi::api().TT_DoQueryServerStats(self.ptr) }
    }

    pub fn query_max_payload(&self, user_id: UserId) -> bool {
        unsafe { ffi::api().TT_QueryMaxPayload(self.ptr, user_id.0) == 1 }
    }

    pub fn pump_message(&self, event: ffi::ClientEvent, id: i32) -> bool {
        unsafe { ffi::api().TT_PumpMessage(self.ptr, event, id) == 1 }
    }

    pub fn quit(&self) -> i32 {
        unsafe { ffi::api().TT_DoQuit(self.ptr) }
    }
}
