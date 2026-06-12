// SPDX-License-Identifier: CC0-1.0

//! Support for connecting to Bitcoin Core via JSON-RPC.
//!
//! # Two clients
//!
//! This crate ships two distinct JSON-RPC clients:
//!
//! * `client_sync` (`client-sync` feature): a blocking client used for integration testing and
//!   covering every method on every supported version of Bitcoin Core.
//! * `client_async` (`client-async` feature): a production-oriented async client built on top of
//!   `jsonrpc` + `bitreq` (with `async` enabled). Its method surface is generated from Bitcoin
//!   Core's OpenRPC export and returns the generated response types (convertible to the strongly
//!   typed model layer via `into_model()`), plus a structured error model with retryability
//!   classification and a `call_raw` escape hatch for methods not yet wrapped.
//!
//! See the documentation of each module for details.

/// Re-export the `rust-bitcoin` crate.
pub extern crate bitcoin;

/// Re-export the `corepc-types` crate.
pub extern crate types;

#[cfg(feature = "client-sync")]
#[macro_use]
pub mod client_sync;

#[cfg(feature = "client-async")]
pub mod client_async;
