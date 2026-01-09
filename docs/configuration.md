# Configuration and Security

## SDK Binaries

By default, the SDK downloads TeamTalk binaries into `TEAMTALK_DLL/` on first
use. In offline environments, enable the `offline` feature and pre-populate
that folder with the correct DLL and headers.

## Networking

The client connects via TCP and UDP ports (default `10333`). Use explicit
ports in production and document them in your environment.

## TLS

For encrypted connections, configure the encryption context using certificate
and key files, then connect with `encrypted = true`.
