// SPDX-License-Identifier: CC0-1.0

//! Async production JSON-RPC client for Bitcoin Core.
//!
//! The version modules are code generated from the Bitcoin Core OpenRPC spec by the
//! `btc-codegen` tool in the repository's `codegen/` directory.

mod auth;
mod client;

#[cfg(feature = "blocking")]
pub mod blocking;

pub mod error;
pub mod v30;

pub use self::auth::Auth;
pub use self::client::{Builder, Client, CONFIGURED_VERSION};
pub use self::error::{ConfigError, Error, Result, Retryability};
