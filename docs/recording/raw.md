# Raw recording API

If the SDK exposes a new argument or function before the wrapper is updated,
you can call the raw API directly.

## Raw SDK access

- Use `Client::raw_ptr()` to get the `TTInstance`.
- Use `teamtalk::client::ffi::api()` to call the C API.
