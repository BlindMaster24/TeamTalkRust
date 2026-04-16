# Video Capture

Discover video capture devices, start capture sessions, and transmit video frames to other users in a channel.

## Device Discovery

List available capture devices with `get_video_capture_devices`. Each `VideoCaptureDevice` exposes:

- `id` — device identifier string
- `name` — human-readable device name
- `api` — capture API name
- `formats` — supported `VideoFormat` list

```rust
let devices = client.get_video_capture_devices();
for dev in &devices {
    println!("{} ({}): {} formats", dev.name, dev.api, dev.formats.len());
}
```

## Capture Lifecycle

1. Initialize a device with `init_video_capture_device(device_id, format)`.
2. Start transmission with `start_video_transmission(codec)`.
3. Stop transmission with `stop_video_transmission()`.
4. Close the device with `close_video_capture_device()`.

```rust
if client.init_video_capture_device(&device.id, &format) {
    client.start_video_transmission(&codec);
    // ... later ...
    client.stop_video_transmission();
    client.close_video_capture_device();
}
```

Each `bool`-returning method has a `_result` variant that returns `Result<()>`:

- `init_video_capture_device_result`
- `close_video_capture_device_result`
- `start_video_transmission_result`
- `stop_video_transmission_result`

## Frame Acquisition

Acquire a remote user's video frame with a guard that auto-releases on drop:

```rust
if let Some(guard) = client.acquire_video_frame_guard(user_id) {
    let view = guard.view();
    println!("Frame: {}x{}", view.width(), view.height());
}
```

Manual acquire/release is also available:

```rust
if let Some(ptr) = client.acquire_video_frame(user_id) {
    // use ptr...
    unsafe { client.release_video_frame(ptr); }
}
```

## VideoFrameView

The view provides read-only access to the acquired video frame:

| Method | Return type | Description |
|---|---|---|
| `width()` | `i32` | Frame width in pixels |
| `height()` | `i32` | Frame height in pixels |
| `stream_id()` | `i32` | Video stream identifier |
| `key_frame()` | `bool` | Whether this is a key (I) frame |
| `frame_buffer()` | `Option<&[u8]>` | Read-only pixel data, or `None` if empty |

`VideoFrameView` is reused for media video frames as well (see [streaming](streaming.md)).

## Windows Rendering

On Windows, video frames can be painted directly to a device context. Both methods are `unsafe` because the HDC and frame pointer must be valid for the entire call.

- `paint_video_frame(hdc, x_dest, y_dest, dest_width, dest_height, frame)` — stretch the full frame into the destination rectangle
- `paint_video_frame_ex(hdc, x_dest, y_dest, dest_width, dest_height, x_src, y_src, src_width, src_height, frame)` — paint a sub-rectangle of the source frame
