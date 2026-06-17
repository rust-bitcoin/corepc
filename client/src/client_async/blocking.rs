// SPDX-License-Identifier: CC0-1.0

//! Blocking, sync-API facade over the async production client.
//!
//! This is a thin version dispatcher. The actual facade is generated per version by `btc-codegen`
//! into `client_async/v{N}/blocking.rs`: it reuses that version's sync-client method macros
//! (`impl_client_*`) over the async transport, so there is no second method surface to
//! hand-maintain. Exactly one version is active (the highest enabled wins), matching the rest of
//! the async client. `bitcoind` swaps `node.client` to this `Client` under its `client-async`
//! feature, so the unchanged integration tests run against the async transport.

#[cfg(feature = "31_0")]
pub use crate::client_async::v31::blocking::*;

#[cfg(all(feature = "30_0", not(feature = "31_0")))]
pub use crate::client_async::v30::blocking::*;
