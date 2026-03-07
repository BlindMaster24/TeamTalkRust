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
- `RequireClientRightsAny` - allow handlers when the current client account has any of the required TeamTalk `UserRights`.
- `RequireClientRightsAll` - allow handlers when the current client account has all required TeamTalk `UserRights`.

`RequireUserType` relies on `Client::get_user`, so it needs the sender to be
available in the local cache.

`RequireClientRightsAny` and `RequireClientRightsAll` use the current logged-in
account via `Client::my_user_rights()`. That matches the TeamTalk server/account
model more closely than checking sender cache state.

### Example

```rust
use teamtalk::{
    CommandOnly, Permissions, RequireClientRightsAll, RequireClientRightsAny, RequireCommand,
    RequireCommandPrefix, RequirePrivateMessage, RequireUserIds, RequireUserType, Router,
    UserId, UserRights,
};

let router = Router::new()
    .use_middleware(CommandOnly)
    .use_middleware(RequireCommandPrefix::new('/'))
    .use_middleware(RequirePrivateMessage)
    .use_middleware(RequireCommand::new("admin"))
    .use_middleware(RequireUserIds::new(vec![UserId(1), UserId(7)]))
    .use_middleware(RequireUserType::new(vec![2, 3]))
    .use_middleware(RequireClientRightsAny::new(
        UserRights::KICK_USERS | UserRights::BAN_USERS,
    ))
    .use_middleware(RequireClientRightsAll::new(Permissions::moderator().rights()));
```

Use `RequireUserType` only when sender cache state is the right signal. For
server-authorized moderation or admin actions, prefer rights-based guards.

## Rate limiting

Rate limiting is separate from guards, but it uses the same middleware pipeline.

```rust
use std::time::Duration;
use teamtalk::{RateLimitBySource, Router};

let router = Router::new().use_middleware(RateLimitBySource::new(Duration::from_secs(2)));
```
