# Configuration and Security

## SDK Binaries

By default, the SDK downloads TeamTalk runtime files into `TEAMTALK_DLL/` on first
use. This includes the shared library, import library, `TeamTalk.h`, and the
SDK `Documentation/` folder. The loader stores a documentation manifest and
verifies that every expected doc file is present on startup; if files are
missing, it re-downloads the SDK archive to repair the folder. In offline
environments, enable the `offline` feature and pre-populate that folder.

### SDK Version Override

By default, the loader uses the pinned version in [SDK_VERSION.txt](../crates/teamtalk/SDK_VERSION.txt).

Set `TEAMTALK_SDK_VERSION` to override the pinned version:

- `TEAMTALK_SDK_VERSION=v5.19` downloads that version.
- `TEAMTALK_SDK_VERSION=latest` forces the latest SDK from BearWare.

Priority order:

1. `TEAMTALK_SDK_VERSION`
2. SDK version file
3. Latest from BearWare (fallback)

If a requested version fails to download, the loader falls back to the latest
SDK.

## Networking

The client connects via TCP and UDP ports (default `10333`). Use explicit
ports in production and document them in your environment.

## TLS

For encrypted connections, configure the encryption context using certificate
and key files, then connect with `encrypted = true`.

For loader TLS options and build troubleshooting, see [docs/tls.md](tls.md).
