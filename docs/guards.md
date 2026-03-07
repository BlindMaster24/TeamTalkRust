# Guards and Rate Limits

Guards are reusable middleware blocks that enforce access rules before a handler runs.
Use them to keep routing logic clean and consistent.

## Built-in guards

- `CommandOnly` - allow only command messages.
- `RequirePrivateMessage` - allow private messages only.
- `RequireChannelMessage` - allow channel messages only.
- `RequireCommand` - allow a specific command name.
- `RequireCommandPrefix` - allow a specific prefix (`/`, `!`, etc.).
- `RequireUserIds` - allow a specific list of sender ids.
- `RequireUserType` - allow a set of `user_type` values.

`RequireUserType` relies on `Client::get_user`, so it needs the sender to be
available in the local cache.

### Example

```rust
use teamtalk::{
    CommandOnly, RequireCommand, RequireCommandPrefix, RequirePrivateMessage, RequireUserIds,
    RequireUserType, Router, UserId,
};

let router = Router::new()
    .use_middleware(CommandOnly)
    .use_middleware(RequireCommandPrefix::new('/'))
    .use_middleware(RequirePrivateMessage)
    .use_middleware(RequireCommand::new("admin"))
    .use_middleware(RequireUserIds::new(vec![UserId(1), UserId(7)]))
    .use_middleware(RequireUserType::new(vec![2, 3]));
```

## Rate limiting

Rate limiting is separate from guards, but it uses the same middleware pipeline.

```rust
use std::time::Duration;
use teamtalk::{RateLimitBySource, Router};

let router = Router::new().use_middleware(RateLimitBySource::new(Duration::from_secs(2)));
```
