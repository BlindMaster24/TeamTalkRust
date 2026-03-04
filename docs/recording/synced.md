# Synced user recording

This mode writes one file per user while keeping all tracks aligned to a shared
timeline. Late joiners are padded with silence, and voice activation toggles no
longer split files.

```rust
use teamtalk::{RecordingSampleFormat, SilencePolicy, SyncedUserRecordingOptions, SyncedUserRecordingSession};

let options = SyncedUserRecordingOptions::new("recordings/users")
    .with_format(RecordingSampleFormat::PcmS16Le)
    .with_silence_policy(SilencePolicy::Always);

let mut recorder = SyncedUserRecordingSession::start(&client, options)?;

loop {
    if let Some((event, message)) = client.poll(50) {
        recorder.handle_event(&client, event, &message)?;
    }
    recorder.tick()?;
}
```

## Event bus integration

```rust
use std::sync::{Arc, Mutex};
use teamtalk::{SyncedUserRecordingBus, SyncedUserRecordingOptions, SyncedUserRecordingSession};

let session = Arc::new(Mutex::new(SyncedUserRecordingSession::start(&client, SyncedUserRecordingOptions::new("recordings/users"))?));
let _bus = SyncedUserRecordingBus::attach(session.clone(), &client, "synced-recorder");
```

Notes:
- Default output is PCM; use `WavS16Le` for a ready-to-use file.
- Silence policy controls how padding is applied.
- Call `tick()` regularly to pad silence when users are connected but silent.
- Timeline is anchored to session start time. Users who start speaking later are padded
  from the session start, so their track can begin with intentional leading silence.
