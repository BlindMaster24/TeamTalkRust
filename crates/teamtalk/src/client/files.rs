use super::Client;
use crate::types::{ChannelId, FileId, RemoteFile, TransferId};
use crate::utils::ToTT;
use teamtalk_sys as ffi;

impl Client {
    pub fn get_channel_files(&self, channel_id: ChannelId) -> Vec<RemoteFile> {
        let mut count: i32 = 0;
        unsafe {
            ffi::api().TT_GetChannelFiles(self.ptr, channel_id.0, std::ptr::null_mut(), &mut count);
            let mut files = vec![std::mem::zeroed::<ffi::RemoteFile>(); count as usize];
            if ffi::api().TT_GetChannelFiles(self.ptr, channel_id.0, files.as_mut_ptr(), &mut count)
                == 1
            {
                files.into_iter().map(RemoteFile::from).collect()
            } else {
                vec![]
            }
        }
    }

    pub fn send_file(&self, channel_id: ChannelId, local_path: &str) -> i32 {
        unsafe { ffi::api().TT_DoSendFile(self.ptr, channel_id.0, local_path.tt().as_ptr()) }
    }

    pub fn recv_file(&self, channel_id: ChannelId, remote_file_id: FileId, local_dir: &str) -> i32 {
        unsafe {
            ffi::api().TT_DoRecvFile(
                self.ptr,
                channel_id.0,
                remote_file_id.0,
                local_dir.tt().as_ptr(),
            )
        }
    }

    pub fn delete_file(&self, channel_id: ChannelId, remote_file_id: FileId) -> i32 {
        unsafe { ffi::api().TT_DoDeleteFile(self.ptr, channel_id.0, remote_file_id.0) }
    }

    pub fn get_file_transfer_info(
        &self,
        transfer_id: TransferId,
    ) -> Option<crate::types::FileTransfer> {
        let mut raw = unsafe { std::mem::zeroed::<ffi::FileTransfer>() };
        if unsafe { ffi::api().TT_GetFileTransferInfo(self.ptr, transfer_id.0, &mut raw) } == 1 {
            Some(crate::types::FileTransfer::from(raw))
        } else {
            None
        }
    }

    pub fn cancel_file_transfer(&self, transfer_id: TransferId) -> bool {
        unsafe { ffi::api().TT_CancelFileTransfer(self.ptr, transfer_id.0) == 1 }
    }
}
