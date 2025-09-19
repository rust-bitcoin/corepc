// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v22` - signer.
//!
//! Types for methods found under the `== Signer ==` section of the API docs.

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `enumeratesigners`.
///
/// > Returns a list of external signers from -signer.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EnumerateSigners {
    /// List of external signers.
    pub signers: Vec<Signers>,
}

/// An signer item. Part of `enumeratesigners`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Signers {
    /// Master key fingerprint.
    pub fingerprint: String,
    /// Device name.
    pub name: String,
}
