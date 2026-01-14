use teamtalk::client::ffi::AudioFileFormat;
use teamtalk::{RecordingOptions, RecordingSession};

fn main() -> teamtalk::Result<()> {
    let client = teamtalk::Client::new()?;
    let channel_id = client.get_root_channel_id();

    let options = RecordingOptions::new(
        "recordings/session-{index}.wav",
        AudioFileFormat::AFF_WAVE_FORMAT,
    );
    let mut session = RecordingSession::start_channel(&client, channel_id, options)?;

    session.pause();
    session.resume()?;
    session.segment()?;
    let _ = session.stop();
    Ok(())
}
