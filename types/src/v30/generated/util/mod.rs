// SPDX-License-Identifier: CC0-1.0

//! Auto-generated types for Bitcoin Core `30` - util.
//!
//! Produced by `codegen`. Do not edit by hand, re-run
//! `just codegen` to regenerate. Holds the RPC return types for this section; the
//! `*Options` request structs live with the call surface in `corepc-client`.

#![allow(non_camel_case_types, non_snake_case, clippy::large_enum_variant)]

mod into;

use serde::{Deserialize, Serialize};

pub use self::into::{
    CreateMultisigError, EstimateSmartFeeError, SignMessageWithPrivKeyError, ValidateAddressError,
};

/// Creates a multi-signature address with n signatures of m keys required.
/// It returns a json object with the address and redeemScript.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateMultisig {
    /// The value of the new multisig address.
    pub address: String,
    /// The descriptor for this multisig
    pub descriptor: String,
    /// The string value of the hex-encoded redemption script.
    #[serde(rename = "redeemScript")]
    pub redeem_script: String,
    /// Any warnings resulting from the creation of this multisig
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// Result of the JSON-RPC method `deriveaddresses`.
///
/// > deriveaddresses
/// >
/// > Derives one or more addresses corresponding to an output descriptor.
/// > Examples of output descriptors are:
/// >     pkh(\<pubkey\>)                                     P2PKH outputs for the given pubkey
/// >     wpkh(\<pubkey\>)                                    Native segwit P2PKH outputs for the given pubkey
/// >     sh(multi(\<n\>,\<pubkey\>,\<pubkey\>,...))              P2SH-multisig outputs for the given threshold and pubkeys
/// >     raw(\<hex script\>)                                 Outputs whose output script equals the specified hex-encoded bytes
/// >     tr(\<pubkey\>,multi_a(\<n\>,\<pubkey\>,\<pubkey\>,...))   P2TR-multisig outputs for the given threshold and pubkeys
/// >
/// > In the above, \<pubkey\> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// > or more path elements separated by "/", where "h" represents a hardened child key.
/// > For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DeriveAddresses {
    List(Vec<String>),
    List2(Vec<Vec<String>>),
}

/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct EstimateSmartFee {
    /// block number where estimate was found
    /// The request target will be clamped between 2 and the highest target
    /// fee estimation is able to return based on how long it has been running.
    /// An error is returned if not enough transactions and blocks
    /// have been observed to make an estimate for any number of blocks.
    pub blocks: i64,
    /// Errors encountered during processing (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// estimate fee rate in BTC/kvB (only present if no errors were encountered)
    #[serde(rename = "feerate", skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<f64>,
}

/// Analyses a descriptor.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorInfo {
    /// The checksum for the input descriptor
    pub checksum: String,
    /// The descriptor in canonical form, without private keys. For a multipath descriptor, only the first will be returned.
    pub descriptor: String,
    /// Whether the input descriptor contained at least one private key
    #[serde(rename = "hasprivatekeys")]
    pub has_private_keys: bool,
    /// Whether the descriptor is ranged
    pub isrange: bool,
    /// Whether the descriptor is solvable
    pub issolvable: bool,
    /// All descriptors produced by expanding multipath derivation elements. Only if the provided descriptor specifies multipath derivation elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipath_expansion: Option<Vec<String>>,
}

/// Result of the JSON-RPC method `getindexinfo`.
///
/// > getindexinfo
/// >
/// > Returns the status of one or all available indices currently running in the node.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetIndexInfo(
    /// Map entries
    pub std::collections::BTreeMap<String, GetIndexInfoEntry>,
);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetIndexInfoEntry {
    /// The block height to which the index is synced
    pub best_block_height: i64,
    /// Whether the index is synced or not
    pub synced: bool,
}

/// Result of the JSON-RPC method `signmessagewithprivkey`.
///
/// > signmessagewithprivkey
/// >
/// > Sign a message with the private key of an address
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignMessageWithPrivKey(pub String);

impl std::ops::Deref for SignMessageWithPrivKey {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Return information about the given bitcoin address.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ValidateAddress {
    /// The bitcoin address validated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Error message, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Indices of likely error locations in address, if known (e.g. Bech32 errors)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_locations: Option<Vec<i64>>,
    /// If the key is a script
    #[serde(rename = "isscript", skip_serializing_if = "Option::is_none")]
    pub is_script: Option<bool>,
    /// If the address is valid or not
    pub isvalid: bool,
    /// If the address is a witness address
    #[serde(rename = "iswitness", skip_serializing_if = "Option::is_none")]
    pub is_witness: Option<bool>,
    /// The hex-encoded output script generated by the address
    #[serde(rename = "scriptPubKey", skip_serializing_if = "Option::is_none")]
    pub script_pub_key: Option<String>,
    /// The hex value of the witness program
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_program: Option<String>,
    /// The version number of the witness program
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_version: Option<i64>,
}

/// Result of the JSON-RPC method `verifymessage`.
///
/// > verifymessage
/// >
/// > Verify a signed message.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyMessage(pub bool);

impl std::ops::Deref for VerifyMessage {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.0 }
}
