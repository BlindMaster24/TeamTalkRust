# TeamTalkRust: Enterprise-Grade Rust SDK for TeamTalk 5

This project is a high-level, safety-first, and fully autonomous idiomatic Rust wrapper for the BearWare.dk TeamTalk 5 SDK. It features a "Zero-Config" architecture and enforces a strict **Pure Event-Driven** paradigm for maximum performance and reliability.

## 🌟 Key Features

- **Pure Event-Driven Architecture:** The SDK operates entirely on a reactive model via `client.poll()`. No arbitrary sleeps or polling loops; every action (login, join, message) is a direct response to a server event.
- **Dynamic Runtime Loading:** Zero-Configuration architecture. No DLLs required at compile-time; all FFI is resolved at runtime via `libloading`.
- **Autonomous Lifecycle:** `loader.rs` automatically scans official BearWare servers, downloads the latest SDK, extracts DLL/LIB/H, and manages versioning.
- **Resilient Connectivity:** 
    - **Smart Reconnection:** Automatic handling of `ConnectionLost` and `ConnectFailed` events.
    - **Flood Protection Handling:** Intelligent management of `CmdError` events to respect server rate limits without crashing.
- **100% API Coverage:**
    - **Events:** Comprehensive mapping of all 50+ TeamTalk events with a professional unified event bus.
    - **Admin Mastery:** Full suite for user moderation (kick/ban/move), account orchestration, and server property management.
    - **File Orchestration:** Integrated file transfer system with granular progress tracking.
    - **User Management:** Detailed control over user states (Voice, Video, Desktop), Gender, and Presence.
- **Strictly Pure Codebase:** The `src` directory is maintained 100% comment-free, adhering to the highest self-documenting code standards of modern Rust.

## 🛠 Technologies

- **Rust 2024 Edition:** Leveraging the latest language features, including strict `unsafe` blocks.
- **Dynamic Bindgen:** Auto-generated dynamic function pointers for seamless TeamTalk FFI.
- **OnceCell & Arc:** High-performance, thread-safe global access to the TeamTalk API instance.
- **Reactive Polling:** Non-blocking event loop architecture.

## 📂 Project Structure

- `crates/teamtalk-sys/`: Low-level bindings, dynamic loading, and embedded headers.
- `crates/teamtalk/`: High-level idiomatic wrapper.
  - `src/loader.rs`: Automated SDK provisioning and updates.
  - `src/client/`: Modularized architecture (Connection, Core, Users, Audio, Video, Files).
  - `src/events.rs`: Comprehensive event bus covering every single SDK callback.
  - `src/types.rs`: Clean Rust abstractions for complex C-structures.
- `TEAMTALK_DLL/`: Local isolated storage for active binaries.

## 🤖 Advanced Examples

- **`mic_bot`**: A fully reactive bot that connects, authenticates, and streams audio based solely on server state changes. Demonstrates the "No Sleep" philosophy.
- **`mass_spammer`**: **Infinite Cycle Stress Tester.** A high-performance tool that:
    - Cycles through server lists indefinitely.
    - Detects Bans/Kicks instantly via event bus.
    - Ignores Flood Protection (`CmdError`) to maximize throughput.
    - Zero delays: operates at the speed of the network stack.
- **`account_manager`**: Admin capabilities demo (create/delete accounts).
- **`server_scanner`**: Automated crawler for discovering active TeamTalk servers.
- **`setup_sdk`**: Utility to force-prepare the environment.

## 🚀 Getting Started

The SDK provisions itself on the first run.

```powershell
# Prepare environment
cargo run --example setup_sdk

# Run the Infinite Stress Tester
cargo run --example mass_spammer
```

## 📚 Developer Guide: Internals

### 1. The Logic Map (Header File)
The **`TeamTalk.h`** file is the source of truth for all documentation.
- **Location:** `crates/teamtalk-sys/TeamTalk.h`

### 2. The Implementation Map (Bindings)
The **`bindings.rs`** file contains the raw Rust representation of the C library.
- **Location:** `target/debug/build/teamtalk-sys-*/out/bindings.rs`

## ⚖️ Development Standards

- **Event-Driven Only:** Never use `thread::sleep` for logic flow. Always wait for the specific `Event`.
- **Zero-Comment Policy:** Logic must be self-evident. No comments allowed in `src`.
- **Safety First:** All `unsafe` FFI calls are encapsulated within safe Rust methods.
- **Linter Driven:** Zero warnings policy (`cargo clippy` must pass with `-D warnings`).
- **Standardized Style:** Strict adherence to `cargo fmt`.

## 🛠 Developer Workflow: Precision Editing

All code modifications in this project MUST be performed using the **Precision Replace Tool**. This ensures:
- **Zero Regression:** Exact matching of context blocks (at least 3 lines before/after) prevents accidental breakage.
- **Context Preservation:** Edits are made within the flow of existing logic, maintaining the "Zero-Comment" and "Event-Driven" standards.
- **Auditability:** Every change is targeted and documented through the tool's instruction set.

Do not use bulk overwrites unless scaffolding a new file. Always prefer atomic, precise replacements.