# Desktop Sharing

Share your desktop with other users in a channel, or receive and display their desktop bitmaps. Desktop sharing is built on top of the `DesktopWindow` FFI type; the safe Rust API wraps it in guard-and-view types that handle acquisition and release automatically.

## Sharing

Start sharing the local desktop with `send_desktop_window`. It accepts a `DesktopWindow` struct and a `BitmapFormat`, and returns the number of bytes sent (or a negative value on failure).

```rust
let sent = client.send_desktop_window(&desktop_window, bitmap_format);
```

On Windows, `send_desktop_window_from_hwnd` captures a window directly from an HWND handle. This avoids manual bitmap construction. The method is `unsafe` because the HWND must remain valid for the duration of the call.

```rust
#[cfg(windows)]
unsafe {
    let sent = client.send_desktop_window_from_hwnd(hwnd, bitmap_format, protocol);
}
```

Windows also exposes helpers to enumerate available desktop HWNDs:

- `get_desktop_active_hwnd()` — active window handle
- `get_desktop_hwnd()` — desktop handle
- `get_desktop_window_hwnd(index)` — window handle by index
- `get_share_window(hwnd)` — `ShareWindow` metadata for an HWND

Stop sharing with `close_desktop_window` (returns `bool`). Use `close_desktop_window_result` for a `Result<()>` variant.

## Desktop Input

Send keyboard and mouse events to a remote user's desktop session.

- `send_desktop_input(user_id, input)` — single `DesktopInput`
- `send_desktop_inputs(user_id, inputs)` — batch of up to `TT_DESKTOPINPUT_MAX` inputs
- `desktop_input_key_translate(translate, inputs)` — translate key codes using a `TTKeyTranslate` function; returns translated inputs or `None` on failure
- `execute_desktop_input(inputs)` — inject inputs into the local desktop; returns count of executed inputs or `-1` on error

```rust
use teamtalk::DesktopInput;

let inputs = vec![DesktopInput::key_press(vk_code)];
let ok = client.send_desktop_inputs(user_id, &inputs);
```

## Cursor Position

Send the local cursor position to remote desktop viewers:

```rust
client.send_desktop_cursor_position(x, y);
```

## Frame Acquisition

To read a remote user's desktop frame, acquire a guard that auto-releases on drop:

```rust
if let Some(guard) = client.acquire_user_desktop_window_guard(user_id) {
    let view = guard.view();
    // use view...
}
```

For format conversion at acquisition time, use the `_ex` variant:

```rust
if let Some(guard) = client.acquire_user_desktop_window_guard_ex(user_id, bitmap_format) {
    let view = guard.view();
    // frame is already in the requested format
}
```

Manual acquire/release is also available but requires `unsafe`:

```rust
if let Some(ptr) = client.acquire_user_desktop_window(user_id) {
    // use ptr...
    unsafe { client.release_user_desktop_window(ptr); }
}
```

## DesktopWindowView

The view provides read-only access to the acquired desktop frame:

| Method | Return type | Description |
|---|---|---|
| `width()` | `i32` | Frame width in pixels |
| `height()` | `i32` | Frame height in pixels |
| `bitmap_format()` | `BitmapFormat` | Pixel format of the frame buffer |
| `bytes_per_line()` | `i32` | Number of bytes per scan line |
| `session_id()` | `i32` | Desktop session identifier |
| `protocol()` | `DesktopProtocol` | Transmission protocol |
| `frame_buffer()` | `Option<&[u8]>` | Read-only pixel data, or `None` if empty |

`session_id()` changes when the remote desktop session restarts. Use it to detect session boundaries and discard stale frames.

## Windows Rendering

On Windows, desktop frames can be painted directly to a device context. Both methods are `unsafe` because the HDC must be valid for the entire call.

- `paint_desktop_window(user_id, hdc, x_dest, y_dest, dest_width, dest_height)` — stretch the full frame into the destination rectangle
- `paint_desktop_window_ex(user_id, hdc, x_dest, y_dest, dest_width, dest_height, x_src, y_src, src_width, src_height)` — paint a sub-rectangle of the source frame

## Authorization

Desktop control (sending input, cursor) requires the remote user to grant desktop controller rights. In the bot framework, check with `Permissions::desktop_controller()`.

## Result Variants

Several methods have `_result` suffixes that return `Result<()>` instead of `bool`:

- `close_desktop_window_result`
- `send_desktop_cursor_position_result`
- `send_desktop_input_result`
