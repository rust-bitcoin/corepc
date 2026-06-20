// SPDX-License-Identifier: CC0-1.0

//! Async production JSON-RPC client for Bitcoin Core.
//!
//! The version modules are code generated from the Bitcoin Core OpenRPC spec by the
//! `btc-codegen` tool in the repository's `codegen/` directory.

mod auth;
mod client;

#[cfg(feature = "blocking")]
pub mod blocking;

// Defines `impl_async_bridges!`, the facade's hand-written isolation bridges (real Rust, not codegen
// string templates). Only the blocking facade uses it.
#[cfg(feature = "blocking")]
#[macro_use]
mod blocking_bridges;

pub mod error;

// Exactly one version module is compiled (its methods hang off the single `Client`). When several
// version features are enabled (e.g. docs.rs `all-features`), the highest version wins.
#[cfg(all(feature = "30_0", not(feature = "31_0")))]
pub mod v30;
#[cfg(feature = "31_0")]
pub mod v31;

pub use self::auth::Auth;
pub use self::client::{Builder, Client, CONFIGURED_VERSION};
pub use self::error::{ConfigError, Error, Result, Retryability};
