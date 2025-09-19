// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.17` - generating.
//!
//! Types for methods found under the `== Generating ==` section of the API docs.

use bitcoin::{hex, BlockHash};
use serde::{Deserialize, Serialize};

use crate::model;

/// Result of JSON-RPC method `generate`.
///
/// > generate nblocks ( maxtries )
/// >
/// > Mine up to nblocks blocks immediately (before the RPC call returns) to an address in the wallet.
/// >
/// > Arguments:
/// > 1. nblocks      (numeric, required) How many blocks are generated immediately.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Generate(
    /// Hashes of blocks generated.
    pub Vec<String>,
);

impl Generate {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::Generate, hex::HexToArrayError> {
        let v = self.0.iter().map(|s| s.parse::<BlockHash>()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::Generate(v))
    }
}

/// Result of JSON-RPC method `generatetoaddress`.
///
/// > generatetoaddress nblocks "address" ( maxtries )
/// >
/// > Mine blocks immediately to a specified address (before the RPC call returns)
/// >
/// > Arguments:
/// > 1. nblocks     (numeric, required) How many blocks are generated immediately.
/// > 2. address     (string, required) The address to send the newly generated bitcoin to.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateToAddress(
    /// Hashes of blocks generated.
    pub Vec<String>,
);

impl GenerateToAddress {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GenerateToAddress, hex::HexToArrayError> {
        let v = self.0.iter().map(|s| s.parse::<BlockHash>()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::GenerateToAddress(v))
    }
}
