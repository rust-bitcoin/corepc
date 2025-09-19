// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the `== Generating ==` section of the API docs.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use bitcoin::{Block, BlockHash};
use serde::{Deserialize, Serialize};

/// Models the result of JSON-RPC method `generate`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Generate(pub Vec<BlockHash>);

impl Generate {
    /// Returns the number of blocks generated.
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns true if 0 blocks were generated.
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

/// Models the result of JSON-RPC method `generateblock`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateBlock {
    /// Hash of generated block.
    pub hash: BlockHash,
    /// Hex of generated block, only present when submit=false.
    pub hex: Option<Block>,
}

/// Models the result of JSON-RPC method `generatetoaddress`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateToAddress(pub Vec<BlockHash>);

impl GenerateToAddress {
    /// Returns the number of blocks generated.
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns true if 0 blocks were generated.
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

/// Models the result of JSON-RPC method `generatetodescriptor`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateToDescriptor(pub Vec<BlockHash>);

impl GenerateToDescriptor {
    /// Returns the number of blocks generated.
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns true if 0 blocks were generated.
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}
