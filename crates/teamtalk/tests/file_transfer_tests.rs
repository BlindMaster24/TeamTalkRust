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
    let t = FileTransfer::new(
        FileTransferStatus::Active,
        TransferId(1),
        ChannelId(2),
        "",
        "",
        200,
        50,
        false,
    );
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
    let active = FileTransfer::new(
        FileTransferStatus::Active,
        TransferId(1),
        ChannelId(2),
        "",
        "",
        200,
        50,
        false,
    );
    let finished = FileTransfer::new(
        FileTransferStatus::Finished,
        TransferId(2),
        ChannelId(2),
        "",
        "",
        200,
        200,
        true,
    );

    assert_eq!(active.remaining_bytes(), 150);
    assert!(!active.is_finished());
    assert!(!active.is_terminal());
    assert_eq!(finished.remaining_bytes(), 0);
    assert!(finished.is_finished());
    assert!(finished.is_terminal());
}

#[test]
fn file_transfer_direction_helpers_match_inbound_flag() {
    let upload = FileTransfer::new(
        FileTransferStatus::Active,
        TransferId(1),
        ChannelId(2),
        "",
        "",
        1,
        0,
        false,
    );
    let download = FileTransfer::new(
        FileTransferStatus::Active,
        TransferId(2),
        ChannelId(2),
        "",
        "",
        1,
        0,
        true,
    );

    assert!(!upload.inbound);
    assert!(download.inbound);
}
