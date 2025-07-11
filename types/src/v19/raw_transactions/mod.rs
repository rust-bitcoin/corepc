// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v19` - raw transactions.
//!
//! Types for methods found under the `== Rawtransactions ==` section of the API docs.

mod error;
mod into;

use bitcoin::address::{Address, NetworkUnchecked};
use bitcoin::ScriptBuf;
use serde::{Deserialize, Serialize};

pub use self::error::{DecodeScriptError, DecodeScriptSegwitError};

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
#[serde(deny_unknown_fields)]
pub struct DecodeScript {
    /// Script public key.
    pub asm: String,
    /// The output type
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
    /// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped).
    pub segwit: Option<DecodeScriptSegwit>,
}
/// `segwit` item returned as part of `decodescript`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DecodeScriptSegwit {
    /// Disassembly of the script.
    pub asm: String,
    /// The raw output script bytes, hex-encoded.
    pub hex: ScriptBuf,
    /// The output type (e.g. nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown).
    #[serde(rename = "type")]
    pub type_: String,
    /// Bitcoin address (only if a well-defined address exists). v22 and later only.
    pub address: Option<String>,
    /// The required signatures.
    #[serde(rename = "reqSigs")]
    pub required_signatures: Option<u64>,
    /// List of bitcoin addresses.
    pub addresses: Option<Vec<String>>,
    /// Address of the P2SH script wrapping this witness redeem script.
    #[serde(rename = "p2sh-segwit")]
    pub p2sh_segwit: Option<Address<NetworkUnchecked>>,
}
