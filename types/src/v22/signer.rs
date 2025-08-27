// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v22` - signer.
//!
//! Types for methods found under the `== Signer ==` section of the API docs.

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `enumeratesigners`.
///
/// > Returns a list of external signers from -signer.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EnumerateSigners {
    /// List of external signers.
    pub signers: Vec<Signers>,
}

/// An item from the list returned by the JSON-RPC method `enumeratesigners`
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Signers {
    /// Master key fingerprint.
    pub fingerprint: String,
    /// Device name.
    pub name: String,
}
