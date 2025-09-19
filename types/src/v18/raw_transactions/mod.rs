// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.18` - raw transactions.
//!
//! Types for methods found under the `== Rawtransactions ==` section of the API docs.

mod error;
mod into;

use serde::{Deserialize, Serialize};

pub use self::error::{AnalyzePsbtError, AnalyzePsbtInputMissingError};

/// Result of JSON-RPC method `analyzepsbt`.
///
/// analyzepsbt "psbt"
///
/// Analyzes and provides information about the current status of a PSBT and its inputs
///
/// Arguments:
/// 1. psbt    (string, required) A base64 string of a PSBT
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbt {
    /// Array of input objects.
    pub inputs: Vec<AnalyzePsbtInput>,
    /// Estimated vsize of the final signed transaction.
    pub estimated_vsize: Option<u32>,
    /// Estimated feerate of the final signed transaction in BTC/kB.
    ///
    /// Shown only if all UTXO slots in the PSBT have been filled.
    #[serde(rename = "estimated_feerate")]
    pub estimated_fee_rate: Option<f64>,
    /// The transaction fee paid. Shown only if all UTXO slots in the PSBT have been filled.
    pub fee: Option<f64>,
    /// Role of the next person that this psbt needs to go to.
    pub next: String,
}

/// Represents an input in a PSBT operation. Part of `analyzepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbtInput {
    /// Whether a UTXO is provided.
    pub has_utxo: bool,
    /// Whether the input is finalized.
    pub is_final: bool,
    /// Things that are missing that are required to complete this input.
    pub missing: Option<AnalyzePsbtInputMissing>,
    /// Role of the next person that this input needs to go to.
    pub next: Option<String>,
}

/// Represents missing elements required to complete an input. Part of `analyzepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbtInputMissing {
    /// Public key ID, hash160 of the public key, of a public key whose BIP 32 derivation path is missing.
    pub pubkeys: Option<Vec<String>>,
    /// Public key ID, hash160 of the public key, of a public key whose signature is missing.
    pub signatures: Option<Vec<String>>,
    /// Hash160 of the redeemScript that is missing.
    #[serde(rename = "redeemscript")]
    pub redeem_script: Option<String>,
    /// SHA256 of the witnessScript that is missing.
    #[serde(rename = "witnessscript")]
    pub witness_script: Option<String>,
}

/// Result of JSON-RPC method `joinpsbts`.
///
/// > joinpsbts ["psbt",...]
/// >
/// > Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
/// > No input in any of the PSBTs can be in more than one of the PSBTs.
/// >
/// > Arguments:
/// > 1. txs            (json array, required) A json array of base64 strings of partially signed transactions
/// >      [
/// >        "psbt",    (string, required) A base64 string of a PSBT
/// >        ...
/// >      ]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct JoinPsbts(
    /// The base64-encoded partially signed transaction.
    pub String,
);

/// Result of JSON-RPC method `utxoupdatepsbt`.
///
/// > utxoupdatepsbt "psbt"
/// >
/// > Updates a PSBT with witness UTXOs retrieved from the UTXO set or the mempool.
/// >
/// > Arguments:
/// > 1. psbt    (string, required) A base64 string of a PSBT
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct UtxoUpdatePsbt(
    /// The base64-encoded partially signed transaction with inputs updated.
    pub String,
);
