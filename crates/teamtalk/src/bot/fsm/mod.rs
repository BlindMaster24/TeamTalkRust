//! Finite-state-machine layer for per-user bot dialogs.
//!
//! Public surface:
//!
//! * [`DialogStatus`] / [`DialogTimeoutPolicy`] - lifecycle enums.
//! * [`DialogState`] - per-user persistent state record with
//!   netstring-based encoding.
//! * [`DialogFlow`] - declarative linear step sequence.
//! * [`DialogMachine`] - mutating façade over a [`crate::bot::StateStore`].
//!
//! The module is split so that each concern lives in its own file:
//!
//! * `status.rs` - [`DialogStatus`] and [`DialogTimeoutPolicy`] enums
//!   with their wire encode/decode helpers.
//! * `state.rs` - [`DialogState`] + [`Self::encode`][1] /
//!   [`Self::decode`][2].
//! * `flow.rs` - [`DialogFlow`] (self-contained).
//! * `machine.rs` - [`DialogMachine`] lifecycle/timeout/metadata
//!   operations.
//! * `encoding.rs` - shared constants, netstring codec, and time
//!   helpers used by `state.rs` and `machine.rs`.
//!
//! [1]: DialogState::encode
//! [2]: DialogState::decode

mod encoding;
mod flow;
mod machine;
mod state;
mod status;

pub use flow::DialogFlow;
pub use machine::DialogMachine;
pub use state::DialogState;
pub use status::{DialogStatus, DialogTimeoutPolicy};
