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
pub use self::error::{DecodePsbtError, GlobalXpubError, PsbtInputError, PsbtOutputError, TaprootScriptPathSigError, TaprootScriptError, TaprootBip32DerivsError, ControlBlocksError, TaprootLeafError};
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
    /// Hex-encoded signature for the Taproot key path spend.
    pub taproot_key_path_sig: Option<String>,
    /// The signature for the pubkey and leaf hash combination.
    pub taproot_script_path_sigs: Option<Vec<TaprootScriptPathSig>>,
    /// Scripts and control blocks for script path spends.
    pub taproot_scripts: Option<Vec<TaprootScript>>,
    /// BIP-32 derivation paths for keys.
    pub taproot_bip32_derivs: Option<Vec<TaprootBip32Deriv>>,
    /// The hex-encoded Taproot x-only internal key.
    pub taproot_internal_key: Option<String>,
    /// The hex-encoded Taproot merkle root.
    pub taproot_merkle_root: Option<String>,
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
    /// The hex-encoded Taproot x-only internal key.
    pub taproot_internal_key: Option<String>,
    /// The tuples that make up the Taproot tree, in depth first search order.
    pub taproot_tree: Option<Vec<TaprootLeaf>>,
    /// BIP32 derivation paths for keys.
    pub taproot_bip32_derivs: Option<Vec<TaprootBip32Deriv>>,
    /// The output proprietary map.
    pub proprietary: Option<Vec<Proprietary>>,
    /// The unknown global fields.
    pub unknown: Option<HashMap<String, String>>,
}

/// An item from the `taproot_script_path_sigs` list. Part of `decodepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TaprootScriptPathSig {
    /// The x-only pubkey for this signature.
    pub pubkey: String,
    /// The leaf hash for this signature.
    pub leaf_hash: String,
    /// The signature itself.
    pub sig: String,
}

/// An item from the `taproot_scripts` list. Part of `decodepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TaprootScript {
    /// A leaf script.
    pub script: String,
    /// The version number for the leaf script.
    #[serde(rename = "leaf_ver")]
    pub leaf_version: u32,
    /// The control blocks for this script.
    pub control_blocks: Vec<String>,
}

/// An item from the `taproot_bip32_derivs` list. Part of `decodepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TaprootBip32Deriv {
    /// The x-only public key this path corresponds to.
    pub pubkey: String,
    /// The fingerprint of the master key.
    pub master_fingerprint: String,
    /// The path.
    pub path: String,
    /// The hashes of the leaves this pubkey appears in.
    pub leaf_hashes: Vec<String>,
}

/// A Taproot leaf script at depth with version. Part of `decodepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TaprootLeaf {
    /// The depth of this element in the tree.
    pub depth: u32,
    /// The version of this leaf.
    #[serde(rename = "leaf_ver")]
    pub leaf_version: u32,
    /// The hex-encoded script itself.
    pub script: String,
}

// TODO: Remove all this code once it is implemented and backported to 0.32.x
// https://github.com/rust-bitcoin/rust-bitcoin/issues/3285
pub mod taproot {
    use core::fmt;

    use bitcoin::hex::{self, FromHex as _};
    use bitcoin::sighash::InvalidSighashTypeError;
    // Re-export this because this module is named the same as the one from `bitcoin`.
    pub use bitcoin::taproot::Signature;
    use bitcoin::{secp256k1, taproot, TapSighashType};

    use crate::error::write_err;

    /// Parses a Taproot signature from a hex string.
    pub fn signature_from_str(sig: &str) -> Result<taproot::Signature, Error> {
        use Error as E;

        let bytes = Vec::from_hex(sig).map_err(E::Hex)?;
        let (sighash_byte, signature) = bytes.split_last().ok_or(E::EmptySignature)?;
        Ok(Signature {
            signature: secp256k1::schnorr::Signature::from_slice(signature)
                .map_err(E::Secp256k1)?,
            sighash_type: TapSighashType::from_consensus_u8(*sighash_byte)
                .map_err(E::SighashType)?,
        })
    }

    /// A Taproot signature-related error.
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[non_exhaustive]
    pub enum Error {
        /// Hex decoding error.
        Hex(hex::HexToBytesError),
        /// Non-standard sighash type.
        SighashType(InvalidSighashTypeError),
        /// Signature was empty.
        EmptySignature,
        /// A secp256k1 error while creating signature from a slice.
        Secp256k1(secp256k1::Error),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use Error::*;

            match *self {
                Hex(ref e) => write_err!(f, "signature hex decoding error"; e),
                SighashType(ref e) => write_err!(f, "non-standard signature hash type"; e),
                EmptySignature => write!(f, "empty Taproot signature"),
                Secp256k1(ref e) => write_err!(f, "secp256k1"; e),
            }
        }
    }

    #[cfg(feature = "std")]
    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            use Error::*;

            match *self {
                Hex(ref e) => Some(e),
                Secp256k1(ref e) => Some(e),
                SighashType(ref e) => Some(e),
                EmptySignature => None,
            }
        }
    }
}
