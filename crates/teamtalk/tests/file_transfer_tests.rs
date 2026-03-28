use teamtalk::client::ffi;
use teamtalk::types::{ChannelId, FileTransfer, FileTransferStatus, TransferId};

#[test]
fn file_transfer_status_mapping() {
    assert!(matches!(
        FileTransferStatus::from(ffi::FileTransferStatus::FILETRANSFER_CLOSED),
        FileTransferStatus::Closed
    ));
    assert!(matches!(
        FileTransferStatus::from(ffi::FileTransferStatus::FILETRANSFER_ERROR),
        FileTransferStatus::Error
    ));
    assert!(matches!(
        FileTransferStatus::from(ffi::FileTransferStatus::FILETRANSFER_ACTIVE),
        FileTransferStatus::Active
    ));
    assert!(matches!(
        FileTransferStatus::from(ffi::FileTransferStatus::FILETRANSFER_FINISHED),
        FileTransferStatus::Finished
    ));
}

#[test]
fn file_transfer_progress_fraction() {
    let t = FileTransfer {
        status: FileTransferStatus::Active,
        id: TransferId(1),
        channel_id: ChannelId(2),
        local_path: String::new(),
        remote_name: String::new(),
        size: 200,
        transferred: 50,
        inbound: false,
    };
    assert_eq!(t.progress(), 0.25);
}

#[test]
fn file_transfer_status_helpers_match_terminal_states() {
    assert!(FileTransferStatus::Active.is_active());
    assert!(!FileTransferStatus::Active.is_terminal());
    assert!(FileTransferStatus::Finished.is_finished());
    assert!(FileTransferStatus::Finished.is_terminal());
    assert!(FileTransferStatus::Error.is_error());
    assert!(FileTransferStatus::Closed.is_terminal());
}

#[test]
fn file_transfer_helper_methods_report_remaining_and_terminal_state() {
    let active = FileTransfer {
        status: FileTransferStatus::Active,
        id: TransferId(1),
        channel_id: ChannelId(2),
        local_path: String::new(),
        remote_name: String::new(),
        size: 200,
        transferred: 50,
        inbound: false,
    };
    let finished = FileTransfer {
        status: FileTransferStatus::Finished,
        id: TransferId(2),
        channel_id: ChannelId(2),
        local_path: String::new(),
        remote_name: String::new(),
        size: 200,
        transferred: 200,
        inbound: true,
    };

    assert_eq!(active.remaining_bytes(), 150);
    assert!(!active.is_finished());
    assert!(!active.is_terminal());
    assert_eq!(finished.remaining_bytes(), 0);
    assert!(finished.is_finished());
    assert!(finished.is_terminal());
}

#[test]
fn file_transfer_direction_helpers_match_inbound_flag() {
    let upload = FileTransfer {
        status: FileTransferStatus::Active,
        id: TransferId(1),
        channel_id: ChannelId(2),
        local_path: String::new(),
        remote_name: String::new(),
        size: 1,
        transferred: 0,
        inbound: false,
    };
    let download = FileTransfer {
        status: FileTransferStatus::Active,
        id: TransferId(2),
        channel_id: ChannelId(2),
        local_path: String::new(),
        remote_name: String::new(),
        size: 1,
        transferred: 0,
        inbound: true,
    };

    assert!(!upload.inbound);
    assert!(download.inbound);
}
