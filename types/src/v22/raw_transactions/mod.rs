// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v22` - raw transactions.
//!
//! Types for methods found under the `== Rawtransactions ==` section of the API docs.

mod error;
mod into;

use serde::{Deserialize, Serialize};

pub use self::error::{DecodeScriptError, MempoolAcceptanceError, TestMempoolAcceptError};

/// Result of JSON-RPC method `decodescript`.
///
/// > decodescript "hexstring"
/// >
/// > Decode a hex-encoded script.
/// >
/// > Arguments:
/// > 1. "hexstring"     (string) the hex encoded script
// The docs on Core v0.17 appear to be way off what is actually returned.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeScript {
    /// Script public key.
    pub asm: String,
    /// The output type.
    #[serde(rename = "type")]
    pub type_: String,
    /// Bitcoin address (only if a well-defined address exists). v22 and later only.
    pub address: Option<String>,
    /// The required signatures.
    #[serde(rename = "reqSigs")]
    pub required_signatures: Option<u64>,
    /// List of bitcoin addresses.
    pub addresses: Option<Vec<String>>,
    /// Address of P2SH script wrapping this redeem script (not returned if the script is already a P2SH).
    pub p2sh: Option<String>,
    /// Segwit data (see `DecodeScriptSegwit` for explanation).
    pub segwit: Option<DecodeScriptSegwit>,
    /// Address of the P2SH script wrapping this witness redeem script
    #[serde(rename = "p2sh-segwit")]
    pub p2sh_segwit: Option<String>,
}

/// Segwit data. Part of `decodescript`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeScriptSegwit {
    /// Script public key.
    pub asm: String,
    /// Hex encoded public key.
    pub hex: String,
    /// The output type.
    #[serde(rename = "type")]
    pub type_: String,
    /// Bitcoin address (only if a well-defined address exists). v22 and later only.
    pub address: Option<String>,
    /// The required signatures.
    #[serde(rename = "reqSigs")]
    pub required_signatures: Option<u64>,
    /// List of bitcoin addresses.
    pub addresses: Option<Vec<String>>,
    /// Address of P2SH script wrapping this redeem script (not returned if the script is already a P2SH).
    #[serde(rename = "p2sh-segwit")]
    pub p2sh_segwit: Option<String>,
}

/// Result of JSON-RPC method `testmempoolaccept`.
///
/// > testmempoolaccept ["rawtxs"] ( allowhighfees )
/// >
/// > Returns if raw transaction (serialized, hex-encoded) would be accepted by mempool.
/// >
/// > This checks if the transaction violates the consensus or policy rules.
/// >
/// > See sendrawtransaction call.
/// >
/// > Arguments:
/// > 1. ["rawtxs"]       (array, required) An array of hex strings of raw transactions.
/// >                                         Length must be one for now.
/// > 2. allowhighfees    (boolean, optional, default=false) Allow high fees
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TestMempoolAccept(pub Vec<MempoolAcceptance>);

/// A single mempool acceptance test result. Part of `testmempoolaccept`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MempoolAcceptance {
    /// The transaction hash in hex.
    pub txid: String,
    /// The transaction witness hash in hex.
    pub wtxid: String,
    /// If the mempool allows this tx to be inserted.
    pub allowed: bool,
    /// Virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted (only present when 'allowed' is true).
    pub vsize: Option<i64>,
    /// Transaction fees (only present if 'allowed' is true).
    pub fees: Option<MempoolAcceptanceFees>,
    /// Rejection string (only present when 'allowed' is false).
    #[serde(rename = "reject-reason")]
    pub reject_reason: Option<String>,
}

/// Wrapper for the fees field. Part of `testmempoolaccept`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MempoolAcceptanceFees {
    /// Transaction fee in BTC.
    pub base: f64,
}
