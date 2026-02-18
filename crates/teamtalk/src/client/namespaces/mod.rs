//! Namespaced API for organized client operations.

pub mod audio;
pub mod channels;
pub mod desktop;
pub mod files;
pub mod server;
pub mod system;
pub mod users;
pub mod video;

#[cfg(feature = "async")]
pub use audio::AsyncAudioNamespace;
pub use audio::AudioNamespace;

#[cfg(feature = "async")]
pub use channels::AsyncChannelsNamespace;
pub use channels::ChannelsNamespace;

#[cfg(feature = "async")]
pub use desktop::AsyncDesktopNamespace;
pub use desktop::DesktopNamespace;

#[cfg(feature = "async")]
pub use files::AsyncFilesNamespace;
pub use files::FilesNamespace;

#[cfg(feature = "async")]
pub use server::AsyncServerNamespace;
pub use server::ServerNamespace;

#[cfg(feature = "async")]
pub use system::AsyncSystemNamespace;
pub use system::SystemNamespace;

#[cfg(feature = "async")]
pub use users::AsyncUsersNamespace;
pub use users::UsersNamespace;

#[cfg(feature = "async")]
pub use video::AsyncVideoNamespace;
pub use video::VideoNamespace;

macro_rules! define_namespace {
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name {
            pub(crate) client: std::sync::Arc<crate::client::Client>,
        }

        impl $name {
            pub(crate) fn new(client: std::sync::Arc<crate::client::Client>) -> Self {
                Self { client }
            }
        }
    };
}

#[cfg(feature = "async")]
macro_rules! define_async_namespace {
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name {
            pub(crate) client: std::sync::Arc<crate::client::Client>,
        }

        impl $name {
            pub(crate) fn new(client: std::sync::Arc<crate::client::Client>) -> Self {
                Self { client }
            }
        }
    };
}

#[cfg(feature = "async")]
pub(crate) use define_async_namespace;
pub(crate) use define_namespace;
