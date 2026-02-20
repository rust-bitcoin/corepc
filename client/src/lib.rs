// SPDX-License-Identifier: CC0-1.0

//! Support for connecting to Bitcoin Core via JSON-RPC.

/// Re-export the `rust-bitcoin` crate.
pub extern crate bitcoin;

/// Re-export the `corepc-types` crate.
pub extern crate types;

#[cfg(feature = "client-sync")]
#[macro_use]
pub mod client_sync;

#[cfg(feature = "client-async")]
pub mod client_async;
