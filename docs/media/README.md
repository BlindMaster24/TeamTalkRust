# Media APIs

TeamTalk provides APIs for desktop sharing, video capture, media file streaming, audio mixer control, and hotkey registration.

- [Desktop Sharing](desktop.md) — share and receive desktop window bitmaps, send input
- [Video Capture](video.md) — device discovery, capture, and frame acquisition
- [Media Streaming](streaming.md) — stream media files to channels, local playback
- [Mixer Control](mixer.md) — Windows audio mixer (volume, mute, selection)
- [Hotkeys](hotkeys.md) — register global hotkeys for push-to-talk and more

## Quick Example

```rust
use teamtalk::{Client, DesktopWindowView, VideoFrameView};

// Acquire a desktop frame with safe access
if let Some(guard) = client.acquire_user_desktop_window_guard(user_id) {
    let view = guard.view();
    println!("Desktop: {}x{}, format={:?}", view.width(), view.height(), view.bitmap_format());
    if let Some(pixels) = view.frame_buffer() {
        // Safe read-only slice of pixel data
    }
}

// Acquire a video frame with safe access
if let Some(guard) = client.acquire_video_frame_guard(user_id) {
    let view = guard.view();
    println!("Video: {}x{}, key_frame={}", view.width(), view.height(), view.key_frame());
}
```
