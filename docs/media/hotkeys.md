# Hotkeys

Register global hotkeys for push-to-talk and other actions. Hotkey support requires a Windows message loop and an HWND-backed client.

## HWND Requirement

Hotkeys only work with `Client::with_hwnd(hwnd, msg)`. A client created with `Client::new()` (polling mode) will fail on all hotkey registration calls — methods return `false` without calling the SDK.

```rust
use teamtalk::Client;

// Required for hotkey support
let client = unsafe { Client::with_hwnd(hwnd, msg)? };

// Will return false — no HWND backing
let polling_client = Client::new()?;
assert!(!polling_client.register_hotkey(hotkey_id, &vk_codes));
```

## Registration

Register a global hotkey by ID and virtual key codes:

```rust
use teamtalk::HotkeyId;

let id = HotkeyId::new(1);
let vk_codes = [0x41]; // 'A' key
let ok = client.register_hotkey(id, &vk_codes);
```

`vk_codes` is a slice of Windows virtual key codes. Multiple codes create a combined hotkey (e.g. Ctrl+Shift+P).

Unregister with:

```rust
client.unregister_hotkey(id);
```

Check whether a hotkey is currently active:

```rust
if client.is_hotkey_active(id) {
    // hotkey is pressed
}
```

## Key String Helper

Get a human-readable name for a virtual key code:

```rust
let name = client.get_key_string(0x41); // e.g. "A"
```

`get_key_string` returns an empty string when the key code has no display name.

## Hotkey Test Hook

On Windows, install a test hook to intercept hotkey events in the message loop:

```rust
#[cfg(windows)]
unsafe {
    client.install_hotkey_test_hook(hwnd, msg);
}
```

Remove the hook when done:

```rust
client.remove_hotkey_test_hook();
```

`install_hotkey_test_hook` is `unsafe` because the HWND must be valid and the message loop must remain active while the hook is installed.

## Event Handling

Hotkey events are delivered through the normal event pipeline. Register handlers with `ClientHooks::on_hotkey` and `ClientHooks::on_hotkey_test`:

```rust
use teamtalk::ClientHooks;

let hooks = ClientHooks::builder()
    .on_hotkey(|client, msg| {
        if let Some(id) = msg.hotkey_id() {
            println!("Hotkey activated: {:?}", id);
        }
    })
    .build();
```

## Limitations

- Hotkeys require Windows and an HWND-backed client.
- Polling clients (`Client::new()`) cannot register hotkeys.
- The application must run a Windows message loop for hotkey events to fire.
