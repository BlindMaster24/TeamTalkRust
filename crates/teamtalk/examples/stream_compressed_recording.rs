use teamtalk::client::ffi::AudioFileFormat;
use teamtalk::types::UserId;
use teamtalk::{UserRecordingOptions, UserRecordingSession};

fn main() -> teamtalk::Result<()> {
    let client = teamtalk::Client::new()?;

    // Example: record stream directly as compressed MP3 via TeamTalk SDK.
    // Replace UserId(1) with a real user ID from your channel.
    let options = UserRecordingOptions::new(
        "recordings/compressed",
        "user-%user_id%-%username%-%starttick%",
        AudioFileFormat::AFF_MP3_128KBIT_FORMAT,
    )
    .with_stop_delay(500);

    let _session = UserRecordingSession::start(&client, UserId(1), options);

    loop {
        if let Some((event, _message)) = client.poll(100)
            && matches!(
                event,
                teamtalk::Event::ConnectionLost | teamtalk::Event::ConnectFailed
            )
        {
            break;
        }
    }

    Ok(())
}
