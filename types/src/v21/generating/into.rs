// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.21` - generating.
//!
//! Types for methods found under the `== Generating ==` section of the API docs.

use bitcoin::{hex, BlockHash};

use super::GenerateBlock;
use crate::model;

impl GenerateBlock {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GenerateBlock, hex::DecodeFixedLengthBytesError> {
        let hash = self.hash.parse::<BlockHash>()?;
        Ok(model::GenerateBlock {
            hash,
            hex: None, // v25 and later only.
        })
    }
}
