// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v23` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `savemempool`.
///
/// Method call: `savemempool`
///
/// > Returns a json object containing "filename": "str" (string) which is the directory and file where mempool was saved.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SaveMempool {
    /// The directory and file where the mempool was saved.
    pub filename: String,
}
