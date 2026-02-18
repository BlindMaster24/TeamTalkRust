//! Type-safe state machine for TeamTalk client operations.
#![cfg(feature = "async")]

use crate::async_api::AsyncClient;
use crate::client::Client;
use crate::events::Result;
use crate::types::User;
use std::marker::PhantomData;

/// State markers for the TeamTalk state machine.
pub mod state {
    /// The client is initialized but not yet connected to a server.
    pub struct Idle;
    /// The client is connected to a server but not yet logged in.
    pub struct Connected;
    /// The client is logged in and ready to issue commands.
    pub struct LoggedIn;
}

/// A type-safe facade for the TeamTalk client using the Typestate pattern.
///
/// This structure ensures that commands can only be issued in valid connection states.
pub struct TeamTalk<S = state::Idle> {
    inner: AsyncClient,
    _state: PhantomData<S>,
}

impl TeamTalk<state::Idle> {
    /// Creates a new TeamTalk client in the Idle state.
    pub fn new(client: Client) -> Self {
        Self {
            inner: client.into_async(),
            _state: PhantomData,
        }
    }

    /// Connects to a TeamTalk server.
    pub async fn connect(
        self,
        host: &str,
        tcp_port: i32,
        udp_port: i32,
        encrypted: bool,
    ) -> Result<TeamTalk<state::Connected>> {
        self.inner
            .connect(host, tcp_port, udp_port, encrypted)
            .await?;
        Ok(TeamTalk {
            inner: self.inner,
            _state: PhantomData,
        })
    }
}

impl TeamTalk<state::Connected> {
    /// Logs in to the server.
    pub async fn login(
        self,
        nickname: &str,
        username: &str,
        password: &str,
        client_name: &str,
    ) -> Result<(TeamTalk<state::LoggedIn>, User)> {
        let user = self
            .inner
            .users()
            .login(nickname, username, password, client_name)
            .await?;
        Ok((
            TeamTalk {
                inner: self.inner,
                _state: PhantomData,
            },
            user,
        ))
    }
}

impl TeamTalk<state::LoggedIn> {
    /// Provides access to user management APIs.
    pub fn users(&self) -> crate::client::namespaces::AsyncUsersNamespace {
        self.inner.users()
    }

    /// Provides access to channel management APIs.
    pub fn channels(&self) -> crate::client::namespaces::AsyncChannelsNamespace {
        self.inner.channels()
    }

    /// Provides access to audio APIs.
    pub fn audio(&self) -> crate::client::namespaces::AsyncAudioNamespace {
        self.inner.audio()
    }

    /// Provides access to server management APIs.
    pub fn server(&self) -> crate::client::namespaces::AsyncServerNamespace {
        self.inner.server()
    }

    /// Accesses the underlying async client.
    pub fn async_client(&self) -> &AsyncClient {
        &self.inner
    }
}

impl<S> TeamTalk<S> {
    /// Downgrades the type-safe client to a regular dynamic AsyncClient.
    pub fn into_inner(self) -> AsyncClient {
        self.inner
    }
}
