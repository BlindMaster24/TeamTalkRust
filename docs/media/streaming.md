# Media Streaming

Stream media files to a channel for all participants, or play back media files locally.

## File Information

Query metadata for a media file before streaming:

```rust
if let Some(info) = client.get_media_file_info("audio.ogg") {
    println!("Duration: {} ms", info.duration_ms);
}
```

`get_media_file_info` returns `Option<MediaFileInfo>`. It returns `None` when the file cannot be read or the format is unsupported.

## Channel Streaming

Stream a media file to the currently joined channel:

```rust
client.start_streaming_media_file_to_channel("audio.ogg", Some(&codec));
```

Use `start_streaming_media_file_to_channel_ex` to include playback settings (offset, pause state):

```rust
use teamtalk::client::media::MediaFilePlayback;

let playback = MediaFilePlayback { offset_ms: 0, paused: false };
client.start_streaming_media_file_to_channel_ex("audio.ogg", &playback, Some(&codec));
```

Update a running stream (seek, pause/resume):

```rust
let playback = MediaFilePlayback { offset_ms: 5000, paused: false };
client.update_streaming_media_file_to_channel(&playback, Some(&codec));
```

Stop streaming:

```rust
client.stop_streaming_media_file_to_channel();
```

Result variants (`_result`) are available for `start_streaming_media_file_to_channel` and `stop_streaming_media_file_to_channel`.

## Local Playback

Play a media file locally without transmitting to a channel:

```rust
let playback = MediaFilePlayback { offset_ms: 0, paused: false };
let session_id = client.init_local_playback("audio.ogg", &playback);

// Seek or pause
let updated = MediaFilePlayback { offset_ms: 10000, paused: false };
client.update_local_playback(session_id, &updated);

// Stop
client.stop_local_playback(session_id);
```

`init_local_playback` returns a `PlaybackSessionId`. `stop_local_playback_result` provides a `Result<()>` variant.

## Media Video Frames

When a media file contains video, acquire the current video frame for a user:

```rust
if let Some(guard) = client.acquire_user_media_video_frame_guard(user_id) {
    let view = guard.view(); // VideoFrameView
    println!("Media video: {}x{}", view.width(), view.height());
}
```

The guard auto-releases on drop. Manual acquire/release is available via `acquire_user_media_video_frame` and `release_user_media_video_frame` (unsafe).

`VideoFrameView` is the same type used for video capture frames (see [video](video.md)).

## Palette Colors

For 8-bit bitmap formats, look up palette entries:

```rust
if let Some(rgb) = client.get_palette_color(bitmap_format, index) {
    println!("Color: #{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]);
}
```

`get_palette_color` returns `Option<[u8; 3]>` (R, G, B) or `None` if the index is out of range.
