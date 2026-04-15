//! File transfer APIs.
use super::Client;
use crate::events::{Error, Event, Result};
use crate::types::{ChannelId, CommandId, FileId, RemoteFile, TransferId};
use std::time::{Duration, Instant};
use teamtalk_sys as ffi;

fn can_issue_logged_in_command(state: crate::events::ConnectionState) -> bool {
    matches!(
        state,
        crate::events::ConnectionState::LoggedIn
            | crate::events::ConnectionState::Joining(_)
            | crate::events::ConnectionState::Joined(_)
    )
}

fn wait_slice(deadline: Instant) -> i32 {
    deadline
        .saturating_duration_since(Instant::now())
        .min(Duration::from_millis(50))
        .as_millis()
        .min(i32::MAX as u128) as i32
}

/// Handle for tracking an active file transfer.
pub struct FileTransferHandle<'a> {
    client: &'a Client,
    transfer_id: TransferId,
}

impl FileTransferHandle<'_> {
    #[must_use]
    pub fn transfer_id(&self) -> TransferId {
        self.transfer_id
    }

    #[must_use]
    pub fn refresh(&self) -> Option<crate::types::FileTransfer> {
        self.client.get_file_transfer_info(self.transfer_id)
    }

    #[must_use]
    pub fn cancel(&self) -> bool {
        self.client.cancel_file_transfer(self.transfer_id)
    }

    pub fn wait_until_terminal(&self, timeout_ms: i32) -> Result<crate::types::FileTransfer> {
        self.client
            .wait_for_file_transfer_terminal(self.transfer_id, timeout_ms)
    }
}

impl Client {
    /// Returns files available in a channel.
    pub fn get_channel_files(&self, channel_id: ChannelId) -> Vec<RemoteFile> {
        let mut count: i32 = 0;
        unsafe {
            ffi::api().TT_GetChannelFiles(
                self.ptr.0,
                channel_id.raw(),
                std::ptr::null_mut(),
                &raw mut count,
            );
            let mut files = vec![std::mem::zeroed::<ffi::RemoteFile>(); count as usize];
            if ffi::api().TT_GetChannelFiles(
                self.ptr.0,
                channel_id.raw(),
                files.as_mut_ptr(),
                &raw mut count,
            ) == 1
            {
                files.into_iter().map(RemoteFile::from).collect()
            } else {
                vec![]
            }
        }
    }

    /// Returns one file by channel id and file id.
    pub fn get_channel_file(&self, channel_id: ChannelId, file_id: FileId) -> Option<RemoteFile> {
        self.backend()
            .get_channel_file(self.ptr.0, channel_id, file_id)
    }

    /// Sends a local file to a channel.
    pub fn send_file(&self, channel_id: ChannelId, local_path: &str) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(
            self.backend()
                .do_send_file(self.ptr.0, channel_id, local_path),
        )
    }

    /// Receives a remote file into a local directory.
    pub fn recv_file(
        &self,
        channel_id: ChannelId,
        remote_file_id: FileId,
        local_dir: &str,
    ) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(
            self.backend()
                .do_recv_file(self.ptr.0, channel_id, remote_file_id, local_dir),
        )
    }

    /// Deletes a remote file from a channel.
    pub fn delete_file(&self, channel_id: ChannelId, remote_file_id: FileId) -> CommandId {
        if !can_issue_logged_in_command(self.connection_state()) {
            return CommandId::ZERO;
        }
        CommandId(
            self.backend()
                .do_delete_file(self.ptr.0, channel_id, remote_file_id),
        )
    }

    /// Returns file transfer info by transfer id.
    pub fn get_file_transfer_info(
        &self,
        transfer_id: TransferId,
    ) -> Option<crate::types::FileTransfer> {
        self.backend()
            .get_file_transfer_info(self.ptr.0, transfer_id)
    }

    /// Cancels an in-progress file transfer.
    pub fn cancel_file_transfer(&self, transfer_id: TransferId) -> bool {
        self.backend().cancel_file_transfer(self.ptr.0, transfer_id)
    }

    /// Returns a tracking handle for an active transfer id.
    pub fn watch_file_transfer(&self, transfer_id: TransferId) -> FileTransferHandle<'_> {
        FileTransferHandle {
            client: self,
            transfer_id,
        }
    }

    /// Waits for the next matching file transfer event.
    pub fn wait_for_file_transfer(
        &self,
        transfer_id: TransferId,
        timeout_ms: i32,
    ) -> Result<crate::types::FileTransfer> {
        if timeout_ms < 0 {
            loop {
                if let Some((event, message)) = self.poll(50)
                    && matches!(event, Event::FileTransfer)
                    && let Some(transfer) = message.file_transfer()
                    && transfer.id == transfer_id
                {
                    return Ok(transfer);
                }
            }
        }

        let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64);
        loop {
            let wait_ms = wait_slice(deadline);
            if wait_ms <= 0 {
                return Err(Error::Timeout);
            }
            if let Some((event, message)) = self.poll(wait_ms)
                && matches!(event, Event::FileTransfer)
                && let Some(transfer) = message.file_transfer()
                && transfer.id == transfer_id
            {
                return Ok(transfer);
            }
        }
    }

    /// Waits until a file transfer reaches a terminal state.
    pub fn wait_for_file_transfer_terminal(
        &self,
        transfer_id: TransferId,
        timeout_ms: i32,
    ) -> Result<crate::types::FileTransfer> {
        if timeout_ms < 0 {
            loop {
                let transfer = self.wait_for_file_transfer(transfer_id, 50)?;
                if transfer.is_terminal() {
                    return Ok(transfer);
                }
            }
        }

        let deadline = Instant::now() + Duration::from_millis(timeout_ms as u64);
        loop {
            let wait_ms = wait_slice(deadline);
            if wait_ms <= 0 {
                return Err(Error::Timeout);
            }
            let transfer = self.wait_for_file_transfer(transfer_id, wait_ms)?;
            if transfer.is_terminal() {
                return Ok(transfer);
            }
        }
    }
}
