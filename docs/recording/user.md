# Per-user recording

Use per-user recording when you want each participant written separately.
This is useful for moderation, analytics, or mixing later.

```rust
use teamtalk::{UserRecordingOptions, UserRecordingSession};
use teamtalk::client::ffi::AudioFileFormat;

let options = UserRecordingOptions::new(
    "recordings/users",
    "user-%user_id%-%username%",
    AudioFileFormat::AFF_WAVE_FORMAT,
);

let _session = UserRecordingSession::start(&client, user_id, options);
```

Notes:
- Per-user recording is controlled by `TT_SetUserMediaStorageDir(Ex)` under the hood.
- Stop by dropping `UserRecordingSession` or calling `stop()`.
