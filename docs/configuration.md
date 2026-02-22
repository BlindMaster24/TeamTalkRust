# Configuration and Security

## SDK Binaries

By default, the SDK downloads TeamTalk runtime files into `TEAMTALK_DLL/` on first
use. This includes the shared library, import library, `TeamTalk.h`, and the
SDK `Documentation/C-API/` folder. The loader stores a documentation manifest
and verifies that every expected C-API doc file is present on startup; if files
are missing, it re-downloads the SDK archive to repair that docs subtree. In
offline environments, enable the `offline` feature and pre-populate that
folder.

### SDK Version Override

By default, the loader uses the pinned version in [SDK_VERSION.txt](../crates/teamtalk/SDK_VERSION.txt).
You can set this file to either a concrete version (for example `v5.18a`) or
`latest`.

Set `TEAMTALK_SDK_VERSION` to override the pinned version:

- `TEAMTALK_SDK_VERSION=v5.19` downloads that version.
- `TEAMTALK_SDK_VERSION=latest` forces the latest SDK from BearWare.

Set `TEAMTALK_SDK_VERSION_URL` to override the remote URL used when
`SDK_VERSION.txt` contains `latest`:

- Default:
  `https://raw.githubusercontent.com/BlindMaster24/TeamTalkRust/main/crates/teamtalk/SDK_VERSION.txt`
- Example custom mirror:
  `TEAMTALK_SDK_VERSION_URL=https://example.com/SDK_VERSION.txt`

Priority order:

1. `TEAMTALK_SDK_VERSION`
2. `SDK_VERSION.txt`
3. Installed `TEAMTALK_DLL/TEAMTALK_SDK_VERSION.txt`
4. Latest from BearWare (fallback)

When `SDK_VERSION.txt` is set to `latest`, the loader resolves the target
version from the repository copy in real time and updates local SDK files when
that remote value changes. If the remote lookup fails, the loader keeps the
already installed SDK when available; if no SDK is installed yet, startup fails
with an error.

If a requested version download fails, the loader falls back to the latest SDK
(except in `offline` mode).

## Networking

The client connects via TCP and UDP ports (default `10333`). Use explicit
ports in production and document them in your environment.

## License Information

If you use a TeamTalk license key, set it during startup before creating
`Client`:

```rust
teamtalk::set_license("Company Name", "license-key")?;
let client = Client::new()?;
client.connect("127.0.0.1", 10333, 10333, false)?;
```

This matches TeamTalk C-API requirements (`TT_SetLicenseInformation` before
`TT_InitTeamTalk`) and ensures the first client instance uses the license.

The `connect_login` example also supports environment-based setup:

- `TT_LICENSE_NAME`
- `TT_LICENSE_KEY`

## TLS

For encrypted connections, configure the encryption context using certificate
and key files, then connect with `encrypted = true`.

For loader TLS options and build troubleshooting, see [docs/tls.md](tls.md).
