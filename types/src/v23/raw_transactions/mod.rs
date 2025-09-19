// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v23` - raw transactions.
//!
//! Types for methods found under the `== Rawtransactions ==` section of the API docs.

mod error;
mod into;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ScriptSig;

#[rustfmt::skip]                // Keep public re-exports separate.
pub use self::error::{DecodePsbtError, DecodeScriptError, GlobalXpubError, PsbtInputError, PsbtOutputError};
// Re-export types that appear in the public API of this module.
pub use crate::psbt::{Bip32Deriv, PsbtScript, RawTransaction, WitnessUtxo};

/// Result of JSON-RPC method `decodepsbt`.
///
/// > decodepsbt "psbt"
/// >
/// > Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
/// >
/// > Arguments:
/// > 1. "psbt"            (string, required) The PSBT base64 string
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbt {
    /// The decoded network-serialized unsigned transaction.
    pub tx: RawTransaction,
    /// The global xpubs.
    pub global_xpubs: Vec<GlobalXpub>,
    /// The PSBT version number. Not to be confused with the unsigned transaction version.
    pub psbt_version: u32,
    /// The global proprietary map.
    pub proprietary: Option<Vec<Proprietary>>,
    /// The unknown global fields.
    pub unknown: Option<HashMap<String, String>>,
    /// Array of transaction inputs.
    pub inputs: Vec<PsbtInput>,
    /// Array of transaction outputs.
    pub outputs: Vec<PsbtOutput>,
    /// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
    pub fee: Option<u64>,
}

/// An item from the global xpubs list. Part of `decodepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GlobalXpub {
    /// The extended public key this path corresponds to.
    pub xpub: String,
    /// The fingerprint of the master key.
    pub master_fingerprint: String,
    /// The path.
    pub path: String,
}

/// An item from the global proprietary list. Part of `decodepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Proprietary {
    /// The hex string for the proprietary identifier.
    identifier: String,
    /// The number for the subtype.
    subtype: i64,
    /// The hex for the key.
    key: String,
    /// The hex for the value.
    value: String,
}

/// An input in a partially signed Bitcoin transaction. Part of `decodepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PsbtInput {
    /// Decoded network transaction for non-witness UTXOs.
    pub non_witness_utxo: Option<RawTransaction>,
    /// Transaction output for witness UTXOs.
    pub witness_utxo: Option<WitnessUtxo>,
    /// The public key and signature that corresponds to it.
    pub partial_signatures: Option<HashMap<String, String>>,
    /// The sighash type to be used.
    pub sighash: Option<String>,
    /// The redeem script.
    pub redeem_script: Option<PsbtScript>,
    /// The witness script.
    pub witness_script: Option<PsbtScript>,
    /// The public key with the derivation path as the value.
    pub bip32_derivs: Option<Vec<Bip32Deriv>>,
    /// The final scriptsig.
    #[serde(rename = "final_scriptsig")]
    pub final_script_sig: Option<ScriptSig>,
    /// Hex-encoded witness data (if any).
    #[serde(rename = "final_scriptwitness")]
    pub final_script_witness: Option<Vec<String>>,
    /// The hash and preimage that corresponds to it.
    pub ripemd160_preimages: Option<HashMap<String, String>>,
    /// The hash and preimage that corresponds to it.
    pub sha256_preimages: Option<HashMap<String, String>>,
    /// The hash and preimage that corresponds to it.
    pub hash160_preimages: Option<HashMap<String, String>>,
    /// The hash and preimage that corresponds to it.
    pub hash256_preimages: Option<HashMap<String, String>>,
    /// The input proprietary map.
    pub proprietary: Option<Vec<Proprietary>>,
    /// The unknown input fields.
    pub unknown: Option<HashMap<String, String>>,
}

/// An output in a partially signed Bitcoin transaction. Part of `decodepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PsbtOutput {
    /// The redeem script.
    pub redeem_script: Option<PsbtScript>,
    /// The witness script.
    pub witness_script: Option<PsbtScript>,
    /// The public key with the derivation path as the value.
    pub bip32_derivs: Option<Vec<Bip32Deriv>>,
    /// The output proprietary map.
    pub proprietary: Option<Vec<Proprietary>>,
    /// The unknown global fields.
    pub unknown: Option<HashMap<String, String>>,
}

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
    /// Inferred descriptor for the script. v23 and later only.
    #[serde(rename = "desc")]
    pub descriptor: Option<String>,
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
    /// Inferred descriptor for the script. v23 and later only.
    #[serde(rename = "desc")]
    pub descriptor: Option<String>,
    /// Address of P2SH script wrapping this redeem script (not returned if the script is already a P2SH).
    #[serde(rename = "p2sh-segwit")]
    pub p2sh_segwit: Option<String>,
}
