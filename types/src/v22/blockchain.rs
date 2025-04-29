// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.22` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

use core::fmt;

use bitcoin::{address, amount, hex, Address, Amount, BlockHash, ScriptBuf, TxOut};
use serde::{Deserialize, Serialize};

use crate::error::write_err;
use crate::v19::ScanTxOutSet as ScanTxOutSetStart;
use crate::{model, NumericError};

/// Result of JSON-RPC method `gettxout`.
///
/// > gettxout "txid" n ( include_mempool )
/// >
/// > Returns details about an unspent transaction output.
/// >
/// > Arguments:
/// > 1. txid               (string, required) The transaction id
/// > 2. n                  (numeric, required) vout number
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTxOut {
    /// The hash of the block at the tip of the chain.
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// The number of confirmations.
    pub confirmations: u32, // TODO: Change this to an i64.
    /// The transaction value in BTC.
    pub value: f64,
    /// The script pubkey.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: ScriptPubkey,
    /// Coinbase or not.
    pub coinbase: bool,
}

impl GetTxOut {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTxOut, GetTxOutError> {
        use GetTxOutError as E;

        let best_block = self.best_block.parse::<BlockHash>().map_err(E::BestBlock)?;
        let tx_out = TxOut {
            value: Amount::from_btc(self.value).map_err(E::Value)?,
            script_pubkey: ScriptBuf::from_hex(&self.script_pubkey.hex).map_err(E::ScriptPubkey)?,
        };

        let addresses = self
            .script_pubkey
            .address
            .into_iter()
            .map(|address| address.parse::<Address<_>>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(E::Addresses)?;

        Ok(model::GetTxOut {
            best_block,
            confirmations: self.confirmations,
            tx_out,
            addresses,
            coinbase: self.coinbase,
        })
    }
}

/// A script pubkey.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ScriptPubkey {
    /// Script assembly.
    pub asm: String,
    /// Script hex.
    pub hex: String,
    /// The type, eg pubkeyhash.
    #[serde(rename = "type")]
    pub type_: String,
    /// bitcoin address (only if a well-defined address exists).
    pub address: Option<String>,
}

/// Error when converting a `GetTxOut` type into the model type.
#[derive(Debug)]
pub enum GetTxOutError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `best_block` field failed.
    BestBlock(hex::HexToArrayError),
    /// Conversion of the transaction `value` field failed.
    Value(amount::ParseAmountError),
    /// Conversion of the transaction `script_pubkey` field failed.
    ScriptPubkey(hex::HexToBytesError),
    /// Conversion of the transaction `addresses` field failed.
    Addresses(address::ParseError),
}

impl fmt::Display for GetTxOutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetTxOutError::*;

        match *self {
            Numeric(ref e) => write_err!(f, "numeric"; e),
            BestBlock(ref e) => write_err!(f, "conversion of the `beast_block` field failed"; e),
            Value(ref e) => write_err!(f, "conversion of the `value` field failed"; e),
            ScriptPubkey(ref e) =>
                write_err!(f, "conversion of the `script_pubkey` field failed"; e),
            Addresses(ref e) => write_err!(f, "conversion of the `addresses` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetTxOutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetTxOutError::*;

        match *self {
            Numeric(ref e) => Some(e),
            BestBlock(ref e) => Some(e),
            Value(ref e) => Some(e),
            ScriptPubkey(ref e) => Some(e),
            Addresses(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetTxOutError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ScanTxOutSetStatus {
    /// Approximate percent complete
    pub progress: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ScanTxOutSet {
    /// Returns after scan completes
    Start(ScanTxOutSetStart),
    /// True (scan will be aborted), False (no scan to abort)
    Abort(bool),
    /// Scan in progress or Completed
    Status(Option<ScanTxOutSetStatus>),
}
