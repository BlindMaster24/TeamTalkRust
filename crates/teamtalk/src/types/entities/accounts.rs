use crate::types::{AbusePrevention, ChannelId};
use teamtalk_sys as ffi;

/// User account definition.
#[derive(Debug, Clone, Default)]
pub struct UserAccount {
    pub username: String,
    pub password: String,
    pub user_type: u32,
    pub user_rights: u32,
    pub note: String,
    pub init_channel: String,
    pub user_data: i32,
    pub auto_operator_channels: Vec<ChannelId>,
    pub audio_codec_bps_limit: i32,
    pub abuse_prevent: AbusePrevention,
}

impl UserAccount {
    /// Creates a user account builder.
    pub fn builder(username: &str) -> UserAccountBuilder {
        UserAccountBuilder::new(username)
    }
}

/// Builder for user account configuration.
pub struct UserAccountBuilder {
    inner: UserAccount,
}

impl UserAccountBuilder {
    /// Creates a new builder with the username.
    pub fn new(username: &str) -> Self {
        Self {
            inner: UserAccount {
                username: username.to_string(),
                password: String::new(),
                user_type: 0,
                user_rights: 0,
                note: String::new(),
                init_channel: String::new(),
                user_data: 0,
                auto_operator_channels: Vec::new(),
                audio_codec_bps_limit: 0,
                abuse_prevent: AbusePrevention::default(),
            },
        }
    }

    /// Sets the account password.
    pub fn password(mut self, pass: &str) -> Self {
        self.inner.password = pass.to_string();
        self
    }

    /// Sets the user type.
    pub fn user_type(mut self, t: u32) -> Self {
        self.inner.user_type = t;
        self
    }

    /// Sets the user rights.
    pub fn rights(mut self, r: u32) -> Self {
        self.inner.user_rights = r;
        self
    }

    /// Builds the user account.
    pub fn build(self) -> UserAccount {
        self.inner
    }
}

impl UserAccount {
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::UserAccount {
        let mut raw = ffi::UserAccount::default();
        let u = crate::utils::ToTT::tt(&self.username);
        let p = crate::utils::ToTT::tt(&self.password);
        let n = crate::utils::ToTT::tt(&self.note);
        let c = crate::utils::ToTT::tt(&self.init_channel);
        unsafe {
            let u_len = u.len().min(511);
            std::ptr::copy_nonoverlapping(u.as_ptr(), raw.szUsername.as_mut_ptr(), u_len);
            let p_len = p.len().min(511);
            std::ptr::copy_nonoverlapping(p.as_ptr(), raw.szPassword.as_mut_ptr(), p_len);
            let n_len = n.len().min(511);
            std::ptr::copy_nonoverlapping(n.as_ptr(), raw.szNote.as_mut_ptr(), n_len);
            let c_len = c.len().min(511);
            std::ptr::copy_nonoverlapping(c.as_ptr(), raw.szInitChannel.as_mut_ptr(), c_len);
        }
        raw.uUserType = self.user_type;
        raw.uUserRights = self.user_rights;
        raw.nUserData = self.user_data;
        raw.nAudioCodecBpsLimit = self.audio_codec_bps_limit;
        raw.abusePrevent = self.abuse_prevent.to_ffi();
        for (i, cid) in self.auto_operator_channels.iter().take(16).enumerate() {
            raw.autoOperatorChannels[i] = cid.0;
        }
        raw
    }
}

impl From<ffi::UserAccount> for UserAccount {
    fn from(a: ffi::UserAccount) -> Self {
        Self {
            username: crate::utils::strings::to_string(&a.szUsername),
            password: String::new(),
            user_type: a.uUserType,
            user_rights: a.uUserRights,
            note: crate::utils::strings::to_string(&a.szNote),
            init_channel: crate::utils::strings::to_string(&a.szInitChannel),
            user_data: a.nUserData,
            auto_operator_channels: a
                .autoOperatorChannels
                .iter()
                .take_while(|&&c| c != 0)
                .map(|&c| ChannelId(c))
                .collect(),
            audio_codec_bps_limit: a.nAudioCodecBpsLimit,
            abuse_prevent: AbusePrevention::from(a.abusePrevent),
        }
    }
}

#[derive(Debug, Clone, Default)]
/// Banned user entry.
pub struct BannedUser {
    pub ip: String,
    pub channel_path: String,
    pub nickname: String,
    pub username: String,
    pub ban_time: String,
    pub ban_types: u32,
    pub owner: String,
}

impl From<ffi::BannedUser> for BannedUser {
    fn from(b: ffi::BannedUser) -> Self {
        Self {
            ip: crate::utils::strings::to_string(&b.szIPAddress),
            channel_path: crate::utils::strings::to_string(&b.szChannelPath),
            nickname: crate::utils::strings::to_string(&b.szNickname),
            username: crate::utils::strings::to_string(&b.szUsername),
            ban_time: crate::utils::strings::to_string(&b.szBanTime),
            ban_types: b.uBanTypes,
            owner: crate::utils::strings::to_string(&b.szOwner),
        }
    }
}

impl BannedUser {
    /// Converts to the raw TeamTalk struct.
    pub fn to_ffi(&self) -> ffi::BannedUser {
        let mut raw = ffi::BannedUser::default();
        let ip = crate::utils::ToTT::tt(&self.ip);
        let chan = crate::utils::ToTT::tt(&self.channel_path);
        let nick = crate::utils::ToTT::tt(&self.nickname);
        let user = crate::utils::ToTT::tt(&self.username);
        let owner = crate::utils::ToTT::tt(&self.owner);
        unsafe {
            let ip_len = ip.len().min(511);
            std::ptr::copy_nonoverlapping(ip.as_ptr(), raw.szIPAddress.as_mut_ptr(), ip_len);
            let chan_len = chan.len().min(511);
            std::ptr::copy_nonoverlapping(chan.as_ptr(), raw.szChannelPath.as_mut_ptr(), chan_len);
            let nick_len = nick.len().min(511);
            std::ptr::copy_nonoverlapping(nick.as_ptr(), raw.szNickname.as_mut_ptr(), nick_len);
            let user_len = user.len().min(511);
            std::ptr::copy_nonoverlapping(user.as_ptr(), raw.szUsername.as_mut_ptr(), user_len);
            let owner_len = owner.len().min(511);
            std::ptr::copy_nonoverlapping(owner.as_ptr(), raw.szOwner.as_mut_ptr(), owner_len);
        }
        raw.uBanTypes = self.ban_types;
        raw
    }
}
