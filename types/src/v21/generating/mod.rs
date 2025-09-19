// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.21` - generating.
//!
//! Types for methods found under the `== Generating ==` section of the API docs.

mod into;

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `generateblock`.
///
/// > Mine a block with a set of ordered transactions immediately to a specified address or descriptor (before the RPC call returns)
/// >
/// > Arguments:
/// > 1. output               (string, required) The address or descriptor to send the newly generated bitcoin to.
/// > 2. transactions         (json array, required) An array of hex strings which are either txids or raw transactions.
/// >                        Txids must reference transactions currently in the mempool.
/// >                        All transactions must be valid and in valid order, otherwise the block will be rejected.
/// >      [
/// >        "rawtx/txid",    (string)
/// >        ...
/// >      ]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateBlock {
    /// Hash of generated block
    pub hash: String,
}
