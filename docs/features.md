# Feature Flags

Feature flags enable optional capabilities. Defaults are empty.

Enable features in `Cargo.toml`:

```toml
[dependencies]
teamtalk = { version = "1.0.0", features = ["dispatch", "async"] }
```

## Available Features

- `dispatch`: event dispatcher with handler routing and reconnect support.
- `async`: async wrapper with a worker thread.
- `logging`: event logging integration.
- `mock`: in-memory event source for tests.
- `offline`: disables SDK downloads; requires `TEAMTALK_DLL/` to be present.
