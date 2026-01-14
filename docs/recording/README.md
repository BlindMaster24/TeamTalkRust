# Recording

Recording workflows need uninterrupted sessions, pause/resume, and safe handling of codec
or channel changes. Use the managed recording APIs to keep one logical session
while rotating file segments underneath.

Why it helps:
- Pause/resume keeps a single logical session without losing context.
- Segments keep files small and allow clean rotation when codecs change.
- Switching channels without rebuilding state avoids recording gaps.

## Managed sessions

`RecordingSession` wraps the SDK recorder and provides:
- pause/resume (implemented as segment stop/start),
- manual segmentation,
- channel switching without rebuilding your own state machine.

```rust
use teamtalk::recording::{RecordingOptions, RecordingSession};
use teamtalk::ffi::AudioFileFormat;

let opts = RecordingOptions::new("recordings/session-{index}.ogg", AudioFileFormat::AFF_OPUS);
let mut session = RecordingSession::start_channel(&client, channel_id, opts)?;

session.pause();
session.resume()?;
session.segment()?;
let ok = session.stop();
```

## Channel or codec changes

When you move to a channel with a different codec, the SDK recording may restart.
Call `session.segment()` or `session.switch_channel(new_id)` to keep one logical
recording with multiple segments.

See:
- [Segment naming and rotation](segments.md)
