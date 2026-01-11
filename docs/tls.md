# TLS Guide

This guide covers TLS options for the SDK loader and common build/runtime issues.

## Which TLS Feature to Use

The loader downloads SDK binaries over HTTPS. Choose one TLS backend:

- `tls-native` (default): Uses the OS TLS stack (Schannel on Windows, Secure Transport on macOS, OpenSSL on Linux).
- `tls-rustls`: Uses Rustls (pure Rust TLS), useful when OpenSSL is unavailable.

If you are on Linux without OpenSSL installed, prefer `tls-rustls`.

## Feature Selection

Default (native TLS):

```bash
cargo build
```

Rustls only:

```bash
cargo build --no-default-features --features tls-rustls
```

## Typical Errors and Fixes

### OpenSSL not found (Linux)

Symptoms:
- `openssl-sys` build failure
- missing `libssl` / `libcrypto`

Fix:
- Install OpenSSL development packages, or
- Switch to Rustls:

```bash
cargo build --no-default-features --features tls-rustls
```

### Corporate proxy or custom CA

Symptoms:
- TLS handshake failures during SDK download
- `certificate verify failed`

Fix:
- Use system TLS (`tls-native`) so OS trust store is honored.
- If you must use Rustls, configure system-wide or custom CA trust for your environment.

### Offline environments

Symptoms:
- Loader cannot download SDK binaries

Fix:
- Enable `offline` feature and pre-populate `TEAMTALK_DLL/` with SDK files.
- The TLS backend does not matter when `offline` is enabled.

## Notes

- TLS selection only affects the SDK download step.
- Encrypted TeamTalk connections are configured via the encryption context API and are independent of the loader TLS backend.
