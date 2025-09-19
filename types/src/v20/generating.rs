// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.20` - generating.
//!
//! Types for methods found under the `== Generating ==` section of the API docs.

use bitcoin::hex;
use serde::{Deserialize, Serialize};

use crate::model;

/// Result of JSON-RPC method `generatetodescriptor`.
///
/// > generatetodescriptor num_blocks "descriptor" ( maxtries )
/// >
/// > Mine blocks immediately to a specified descriptor (before the RPC call returns)
/// >
/// > Arguments:
/// > 1. num_blocks    (numeric, required) How many blocks are generated immediately.
/// > 2. descriptor    (string, required) The descriptor to send the newly generated bitcoin to.
/// > 3. maxtries      (numeric, optional, default=1000000) How many iterations to try.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GenerateToDescriptor(
    /// Hashes of blocks generated.
    pub Vec<String>,
);

impl GenerateToDescriptor {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GenerateToDescriptor, hex::HexToArrayError> {
        let v = self.0.iter().map(|s| s.parse()).collect::<Result<Vec<_>, _>>()?;
        Ok(model::GenerateToDescriptor(v))
    }
}
