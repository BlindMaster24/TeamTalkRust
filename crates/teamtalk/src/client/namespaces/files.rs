//! File transfer namespace.
use super::define_namespace;
#[cfg(feature = "async")]
use crate::events::Result;
use crate::types::{ChannelId, FileId, FileTransfer, RemoteFile, TransferId};

define_namespace!(FilesNamespace);

impl FilesNamespace {
    /// Returns files available in a channel.
    pub fn list(&self, channel_id: ChannelId) -> Vec<RemoteFile> {
        self.client.get_channel_files(channel_id)
    }

    /// Sends a local file to a channel.
    pub fn upload(&self, channel_id: ChannelId, local_path: &str) -> i32 {
        self.client.send_file(channel_id, local_path)
    }

    /// Receives a remote file into a local directory.
    pub fn download(&self, channel_id: ChannelId, remote_file_id: FileId, local_dir: &str) -> i32 {
        self.client.recv_file(channel_id, remote_file_id, local_dir)
    }

    /// Deletes a remote file from a channel.
    pub fn delete(&self, channel_id: ChannelId, remote_file_id: FileId) -> i32 {
        self.client.delete_file(channel_id, remote_file_id)
    }

    /// Returns file transfer info by transfer id.
    pub fn transfer_info(&self, transfer_id: TransferId) -> Option<FileTransfer> {
        self.client.get_file_transfer_info(transfer_id)
    }

    /// Cancels an in-progress file transfer.
    pub fn cancel(&self, transfer_id: TransferId) -> bool {
        self.client.cancel_file_transfer(transfer_id)
    }
}

#[cfg(feature = "async")]
use super::define_async_namespace;
#[cfg(feature = "async")]
use crate::events::Event;

#[cfg(feature = "async")]
define_async_namespace!(AsyncFilesNamespace);

#[cfg(feature = "async")]
impl AsyncFilesNamespace {
    /// Returns files available in a channel (from cache).
    pub fn list(&self, channel_id: ChannelId) -> Vec<RemoteFile> {
        self.client.get_channel_files(channel_id)
    }

    /// Sends a local file to a channel and waits for the transfer to start.
    #[cfg(feature = "async")]
    pub async fn upload(&self, channel_id: ChannelId, local_path: &str) -> Result<FileTransfer> {
        self.client
            .execute_command(Event::FileTransfer, || {
                self.client.send_file(channel_id, local_path)
            })
            .await
    }

    /// Receives a remote file and waits for the transfer to start.
    #[cfg(feature = "async")]
    pub async fn download(
        &self,
        channel_id: ChannelId,
        remote_file_id: FileId,
        local_dir: &str,
    ) -> Result<FileTransfer> {
        self.client
            .execute_command(Event::FileTransfer, || {
                self.client.recv_file(channel_id, remote_file_id, local_dir)
            })
            .await
    }

    /// Deletes a remote file from a channel.
    pub fn delete(&self, channel_id: ChannelId, remote_file_id: FileId) -> i32 {
        self.client.delete_file(channel_id, remote_file_id)
    }

    /// Returns file transfer info.
    pub fn transfer_info(&self, transfer_id: TransferId) -> Option<FileTransfer> {
        self.client.get_file_transfer_info(transfer_id)
    }

    /// Cancels an in-progress file transfer.
    pub fn cancel(&self, transfer_id: TransferId) -> bool {
        self.client.cancel_file_transfer(transfer_id)
    }
}
