// SPDX-License-Identifier: CC0-1.0

//! Auto-generated types for Bitcoin Core `31` - rawtransactions.
//!
//! Produced by `codegen`. Do not edit by hand, re-run
//! `just codegen` to regenerate. Holds the RPC return types for this section; the
//! `*Options` request structs live with the call surface in `corepc-client`.

#![allow(non_camel_case_types, non_snake_case, clippy::large_enum_variant)]

mod into;

use serde::{Deserialize, Serialize};

pub use self::into::{
    AbortPrivateBroadcastError, AnalyzePsbtError, AnalyzePsbtInputError,
    AnalyzePsbtInputMissingError, CombinePsbtError, CombineRawTransactionError, ConvertToPsbtError,
    CreatePsbtError, CreateRawTransactionError, DecodePsbtError, DecodeRawTransactionError,
    DecodeScriptError, DescriptorProcessPsbtError, FinalizePsbtError, FundRawTransactionError,
    GetPrivateBroadcastInfoError, GetRawTransactionError, GetRawTransactionVerboseError,
    JoinPsbtsError, MempoolAcceptanceError, MempoolAcceptanceFeesError, SendRawTransactionError,
    SignFailError, SignRawTransactionError, SubmitPackageError, SubmitPackageTxResultError,
    SubmitPackageTxResultFeesError, TestMempoolAcceptError, UtxoUpdatePsbtError,
};

/// Abort private broadcast attempts for a transaction currently being privately broadcast.
/// The transaction will be removed from the private broadcast queue.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AbortPrivateBroadcast {
    /// Transactions removed from the private broadcast queue
    pub removed_transactions: Vec<AbortPrivateBroadcastRemovedTransactionsItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AbortPrivateBroadcastRemovedTransactionsItem {
    /// The serialized, hex-encoded transaction data
    pub hex: String,
    /// The transaction hash in hex
    pub txid: String,
    /// The transaction witness hash in hex
    pub wtxid: String,
}

/// Analyzes and provides information about the current status of a PSBT and its inputs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbt {
    /// Error message (if there is one)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Estimated feerate of the final signed transaction in BTC/kvB. Shown only if all UTXO slots in the PSBT have been filled
    #[serde(rename = "estimated_feerate", skip_serializing_if = "Option::is_none")]
    pub estimated_fee_rate: Option<f64>,
    /// Estimated vsize of the final signed transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_vsize: Option<u64>,
    /// The transaction fee paid. Shown only if all UTXO slots in the PSBT have been filled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<AnalyzePsbtInputsItem>>,
    /// Role of the next person that this psbt needs to go to
    pub next: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbtInputsItem {
    /// Whether a UTXO is provided
    pub has_utxo: bool,
    /// Whether the input is finalized
    pub is_final: bool,
    /// Things that are missing that are required to complete this input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing: Option<AnalyzePsbtInputsItemMissing>,
    /// Role of the next person that this input needs to go to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

/// Things that are missing that are required to complete this input
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AnalyzePsbtInputsItemMissing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkeys: Option<Vec<String>>,
    /// Hash160 of the redeem script that is missing
    #[serde(rename = "redeemscript", skip_serializing_if = "Option::is_none")]
    pub redeem_script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatures: Option<Vec<String>>,
    /// SHA256 of the witness script that is missing
    #[serde(rename = "witnessscript", skip_serializing_if = "Option::is_none")]
    pub witness_script: Option<String>,
}

/// Result of the JSON-RPC method `combinepsbt`.
///
/// > combinepsbt
/// >
/// > Combine multiple partially signed Bitcoin transactions into one transaction.
/// > Implements the Combiner role.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CombinePsbt(pub String);

impl std::ops::Deref for CombinePsbt {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Result of the JSON-RPC method `combinerawtransaction`.
///
/// > combinerawtransaction
/// >
/// > Combine multiple partially signed transactions into one transaction.
/// > The combined transaction may be another partially signed transaction or a
/// > fully signed transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CombineRawTransaction(pub String);

impl std::ops::Deref for CombineRawTransaction {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Result of the JSON-RPC method `converttopsbt`.
///
/// > converttopsbt
/// >
/// > Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
/// > createpsbt and walletcreatefundedpsbt should be used for new applications.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ConvertToPsbt(pub String);

impl std::ops::Deref for ConvertToPsbt {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Result of the JSON-RPC method `createpsbt`.
///
/// > createpsbt
/// >
/// > Creates a transaction in the Partially Signed Transaction format.
/// > Implements the Creator role.
/// > Note that the transaction's inputs are not signed, and
/// > it is not stored in the wallet or transmitted to the network.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreatePsbt(pub String);

impl std::ops::Deref for CreatePsbt {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Result of the JSON-RPC method `createrawtransaction`.
///
/// > createrawtransaction
/// >
/// > Create a transaction spending the given inputs and creating new outputs.
/// > Outputs can be addresses or data.
/// > Returns hex-encoded raw transaction.
/// > Note that the transaction's inputs are not signed, and
/// > it is not stored in the wallet or transmitted to the network.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct CreateRawTransaction(pub String);

impl std::ops::Deref for CreateRawTransaction {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbt {
    /// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    pub global_xpubs: Vec<DecodePsbtGlobalXpubsItem>,
    pub inputs: Vec<DecodePsbtInputsItem>,
    pub outputs: Vec<DecodePsbtOutputsItem>,
    /// The global proprietary map
    pub proprietary: Vec<DecodePsbtProprietaryItem>,
    /// The PSBT version number. Not to be confused with the unsigned transaction version
    pub psbt_version: i64,
    /// The decoded network-serialized unsigned transaction.
    pub tx: DecodePsbtTx,
    /// The unknown global fields
    pub unknown: std::collections::BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtGlobalXpubsItem {
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The extended public key this path corresponds to
    pub xpub: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip32_derivs: Option<Vec<DecodePsbtInputsItemBip32DerivsItem>>,
    #[serde(rename = "final_scriptSig", skip_serializing_if = "Option::is_none")]
    pub final_script_sig: Option<DecodePsbtInputsItemFinalScriptSig>,
    #[serde(rename = "final_scriptwitness", skip_serializing_if = "Option::is_none")]
    pub final_script_witness: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash160_preimages: Option<std::collections::BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash256_preimages: Option<std::collections::BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub musig2_partial_sigs: Option<Vec<DecodePsbtInputsItemMusig2PartialSigsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub musig2_participant_pubkeys: Option<Vec<DecodePsbtInputsItemMusig2ParticipantPubkeysItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub musig2_pubnonces: Option<Vec<DecodePsbtInputsItemMusig2PubnoncesItem>>,
    /// Decoded network transaction for non-witness UTXOs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_witness_utxo: Option<DecodePsbtInputsItemNonWitnessUtxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_signatures: Option<std::collections::BTreeMap<String, String>>,
    /// The input proprietary map
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proprietary: Option<Vec<DecodePsbtInputsItemProprietaryItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_script: Option<DecodePsbtInputsItemRedeemScript>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ripemd160_preimages: Option<std::collections::BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256_preimages: Option<std::collections::BTreeMap<String, String>>,
    /// The sighash type to be used
    #[serde(rename = "sighash", skip_serializing_if = "Option::is_none")]
    pub sig_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_bip32_derivs: Option<Vec<DecodePsbtInputsItemTaprootBip32DerivsItem>>,
    /// The hex-encoded Taproot x-only internal key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_internal_key: Option<String>,
    /// hex-encoded signature for the Taproot key path spend
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_key_path_sig: Option<String>,
    /// The hex-encoded Taproot merkle root
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_merkle_root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_script_path_sigs: Option<Vec<DecodePsbtInputsItemTaprootScriptPathSigsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_scripts: Option<Vec<DecodePsbtInputsItemTaprootScriptsItem>>,
    /// The unknown input fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<std::collections::BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_script: Option<DecodePsbtInputsItemWitnessScript>,
    /// Transaction output for witness UTXOs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_utxo: Option<DecodePsbtInputsItemWitnessUtxo>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemBip32DerivsItem {
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The public key with the derivation path as the value.
    pub pubkey: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemFinalScriptSig {
    /// Disassembly of the final signature script
    pub asm: String,
    /// The raw final signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemMusig2PartialSigsItem {
    /// The compressed aggregate public key for which this partial signature is for.
    pub aggregate_pubkey: String,
    /// The hash of the leaf script that contains the aggregate pubkey being signed for. Omitted when signing for the internal key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_hash: Option<String>,
    /// The partial signature itself.
    pub partial_sig: String,
    /// The compressed public key of the participant that created this partial signature.
    pub participant_pubkey: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemMusig2ParticipantPubkeysItem {
    /// The compressed aggregate public key for which the participants create.
    pub aggregate_pubkey: String,
    pub participant_pubkeys: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemMusig2PubnoncesItem {
    /// The compressed aggregate public key for which this pubnonce is for.
    pub aggregate_pubkey: String,
    /// The hash of the leaf script that contains the aggregate pubkey being signed for. Omitted when signing for the internal key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_hash: Option<String>,
    /// The compressed public key of the participant that created this pubnonce.
    pub participant_pubkey: String,
    /// The public nonce itself.
    pub pubnonce: String,
}

/// Decoded network transaction for non-witness UTXOs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemNonWitnessUtxo {
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The lock time
    pub locktime: i64,
    /// The serialized transaction size
    pub size: i64,
    /// The transaction id
    pub txid: String,
    /// The version
    pub version: i64,
    pub vin: Vec<DecodePsbtInputsItemNonWitnessUtxoVinItem>,
    pub vout: Vec<DecodePsbtInputsItemNonWitnessUtxoVoutItem>,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: i64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemNonWitnessUtxoVinItem {
    /// The coinbase value (only if coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,
    /// The script (if not coinbase transaction)
    #[serde(rename = "scriptSig", skip_serializing_if = "Option::is_none")]
    pub script_sig: Option<DecodePsbtInputsItemNonWitnessUtxoVinItemScriptSig>,
    /// The script sequence number
    pub sequence: i64,
    /// The transaction id (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    #[serde(rename = "txinwitness", skip_serializing_if = "Option::is_none")]
    pub txin_witness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vout: Option<i64>,
}

/// The script (if not coinbase transaction)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemNonWitnessUtxoVinItemScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemNonWitnessUtxoVoutItem {
    /// index
    pub n: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: DecodePsbtInputsItemNonWitnessUtxoVoutItemScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemNonWitnessUtxoVoutItemScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemProprietaryItem {
    /// The hex string for the proprietary identifier
    pub identifier: String,
    /// The hex for the key
    pub key: String,
    /// The number for the subtype
    #[serde(rename = "subtype")]
    pub sub_type: u64,
    /// The hex for the value
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemRedeemScript {
    /// Disassembly of the redeem script
    pub asm: String,
    /// The raw redeem script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemTaprootBip32DerivsItem {
    /// The hashes of the leaves this pubkey appears in
    pub leaf_hashes: Vec<String>,
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The x-only public key this path corresponds to
    pub pubkey: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemTaprootScriptPathSigsItem {
    /// The leaf hash for this signature
    pub leaf_hash: String,
    /// The x-only pubkey for this signature
    pub pubkey: String,
    /// The signature itself
    pub sig: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemTaprootScriptsItem {
    /// The control blocks for this script
    pub control_blocks: Vec<String>,
    /// The version number for the leaf script
    pub leaf_ver: i64,
    /// A leaf script
    pub script: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemWitnessScript {
    /// Disassembly of the witness script
    pub asm: String,
    /// The raw witness script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

/// Transaction output for witness UTXOs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemWitnessUtxo {
    /// The value in BTC
    pub amount: f64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: DecodePsbtInputsItemWitnessUtxoScriptPubKey,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtInputsItemWitnessUtxoScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip32_derivs: Option<Vec<DecodePsbtOutputsItemBip32DerivsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub musig2_participant_pubkeys: Option<Vec<DecodePsbtOutputsItemMusig2ParticipantPubkeysItem>>,
    /// The output proprietary map
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proprietary: Option<Vec<DecodePsbtOutputsItemProprietaryItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_script: Option<DecodePsbtOutputsItemRedeemScript>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_bip32_derivs: Option<Vec<DecodePsbtOutputsItemTaprootBip32DerivsItem>>,
    /// The hex-encoded Taproot x-only internal key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_internal_key: Option<String>,
    /// The tuples that make up the Taproot tree, in depth first search order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taproot_tree: Option<Vec<DecodePsbtOutputsItemTaprootTreeItem>>,
    /// The unknown output fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<std::collections::BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness_script: Option<DecodePsbtOutputsItemWitnessScript>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItemBip32DerivsItem {
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The public key this path corresponds to
    pub pubkey: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItemMusig2ParticipantPubkeysItem {
    /// The compressed aggregate public key for which the participants create.
    pub aggregate_pubkey: String,
    pub participant_pubkeys: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItemProprietaryItem {
    /// The hex string for the proprietary identifier
    pub identifier: String,
    /// The hex for the key
    pub key: String,
    /// The number for the subtype
    #[serde(rename = "subtype")]
    pub sub_type: u64,
    /// The hex for the value
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItemRedeemScript {
    /// Disassembly of the redeem script
    pub asm: String,
    /// The raw redeem script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItemTaprootBip32DerivsItem {
    /// The hashes of the leaves this pubkey appears in
    pub leaf_hashes: Vec<String>,
    /// The fingerprint of the master key
    pub master_fingerprint: String,
    /// The path
    pub path: String,
    /// The x-only public key this path corresponds to
    pub pubkey: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItemTaprootTreeItem {
    /// The depth of this element in the tree
    pub depth: i64,
    /// The version of this leaf
    pub leaf_ver: i64,
    /// The hex-encoded script itself
    pub script: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtOutputsItemWitnessScript {
    /// Disassembly of the witness script
    pub asm: String,
    /// The raw witness script bytes, hex-encoded
    pub hex: String,
    /// The type, eg 'pubkeyhash'
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtProprietaryItem {
    /// The hex string for the proprietary identifier
    pub identifier: String,
    /// The hex for the key
    pub key: String,
    /// The number for the subtype
    #[serde(rename = "subtype")]
    pub sub_type: u64,
    /// The hex for the value
    pub value: String,
}

/// The decoded network-serialized unsigned transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtTx {
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The lock time
    pub locktime: i64,
    /// The serialized transaction size
    pub size: i64,
    /// The transaction id
    pub txid: String,
    /// The version
    pub version: i64,
    pub vin: Vec<DecodePsbtTxVinItem>,
    pub vout: Vec<DecodePsbtTxVoutItem>,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: i64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtTxVinItem {
    /// The coinbase value (only if coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,
    /// The script (if not coinbase transaction)
    #[serde(rename = "scriptSig", skip_serializing_if = "Option::is_none")]
    pub script_sig: Option<DecodePsbtTxVinItemScriptSig>,
    /// The script sequence number
    pub sequence: i64,
    /// The transaction id (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    #[serde(rename = "txinwitness", skip_serializing_if = "Option::is_none")]
    pub txin_witness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vout: Option<i64>,
}

/// The script (if not coinbase transaction)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtTxVinItemScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtTxVoutItem {
    /// index
    pub n: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: DecodePsbtTxVoutItemScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodePsbtTxVoutItemScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub type_: String,
}

/// Return a JSON object representing the serialized, hex-encoded transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransaction {
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The lock time
    pub locktime: i64,
    /// The serialized transaction size
    pub size: i64,
    /// The transaction id
    pub txid: String,
    /// The version
    pub version: i64,
    pub vin: Vec<DecodeRawTransactionVinItem>,
    pub vout: Vec<DecodeRawTransactionVoutItem>,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: i64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionVinItem {
    /// The coinbase value (only if coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,
    /// The script (if not coinbase transaction)
    #[serde(rename = "scriptSig", skip_serializing_if = "Option::is_none")]
    pub script_sig: Option<DecodeRawTransactionVinItemScriptSig>,
    /// The script sequence number
    pub sequence: i64,
    /// The transaction id (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    #[serde(rename = "txinwitness", skip_serializing_if = "Option::is_none")]
    pub txin_witness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vout: Option<i64>,
}

/// The script (if not coinbase transaction)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionVinItemScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionVoutItem {
    /// index
    pub n: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: DecodeRawTransactionVoutItemScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeRawTransactionVoutItemScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub type_: String,
}

/// Decode a hex-encoded script.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeScript {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the script
    pub asm: String,
    /// Inferred descriptor for the script
    pub desc: String,
    /// address of P2SH script wrapping this redeem script (not returned for types that should not be wrapped)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2sh: Option<String>,
    /// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segwit: Option<DecodeScriptSegwit>,
    /// The output type (e.g. nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub type_: String,
}

/// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DecodeScriptSegwit {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the script
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// address of the P2SH script wrapping this witness redeem script
    #[serde(rename = "p2sh-segwit")]
    pub p2sh_segwit: String,
    /// The type of the output script (e.g. witness_v0_keyhash or witness_v0_scripthash)
    #[serde(rename = "type")]
    pub type_: String,
}

/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DescriptorProcessPsbt {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction
    pub psbt: String,
}

/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptwitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FinalizePsbt {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if extracted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    /// The base64-encoded partially signed transaction if not extracted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<String>,
}

/// If the transaction has no inputs, they will be automatically selected to meet its out value.
/// It will add at most one change output to the outputs.
/// No existing outputs will be modified unless "subtractFeeFromOutputs" is specified.
/// Note that inputs which were signed may need to be resigned after completion since in/outputs have been added.
/// The inputs added will not be signed, use signrawtransactionwithkey
/// or signrawtransactionwithwallet for that.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
/// Note that all inputs selected must be of standard form and P2SH scripts must be
/// in the wallet using importdescriptors (to calculate fees).
/// You can see whether this is the case by checking the "solvable" field in the listunspent output.
/// Note that if specifying an exact fee rate, the resulting transaction may have a higher fee rate
/// if the transaction has unconfirmed inputs. This is because the wallet will attempt to make the
/// entire package have the given fee rate, not the resulting transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct FundRawTransaction {
    /// The position of the added change output, or -1
    pub changepos: i64,
    /// Fee in BTC the resulting transaction pays
    pub fee: f64,
    /// The resulting raw transaction (hex-encoded string)
    pub hex: String,
}

/// Returns information about transactions that are currently being privately broadcast.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPrivateBroadcastInfo {
    pub transactions: Vec<GetPrivateBroadcastInfoTransactionsItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPrivateBroadcastInfoTransactionsItem {
    /// The serialized, hex-encoded transaction data
    pub hex: String,
    /// Per-peer send and acknowledgment information for this transaction
    pub peers: Vec<GetPrivateBroadcastInfoTransactionsItemPeersItem>,
    /// The transaction hash in hex
    pub txid: String,
    /// The transaction witness hash in hex
    pub wtxid: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPrivateBroadcastInfoTransactionsItemPeersItem {
    /// The address of the peer to which the transaction was sent
    pub address: String,
    /// The time this peer acknowledged reception of the transaction (seconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received: Option<i64>,
    /// The time this transaction was picked for sending to this peer via private broadcast (seconds since epoch)
    pub sent: i64,
}

/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
///
/// Hint: Use gettransaction for wallet transactions.
///
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerbose0(pub String);

impl std::ops::Deref for GetRawTransactionVerbose0 {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
///
/// Hint: Use gettransaction for wallet transactions.
///
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerbose1 {
    /// the block hash
    #[serde(rename = "blockhash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    /// The block time expressed in UNIX epoch time
    #[serde(rename = "blocktime", skip_serializing_if = "Option::is_none")]
    pub block_time: Option<i64>,
    /// The confirmations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmations: Option<i64>,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The serialized, hex-encoded data for 'txid'
    pub hex: String,
    /// Whether specified block is in the active chain or not (only present with explicit "blockhash" argument)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_active_chain: Option<bool>,
    /// The lock time
    pub locktime: i64,
    /// The serialized transaction size
    pub size: i64,
    /// Same as "blocktime"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// The transaction id (same as provided)
    pub txid: String,
    /// The version
    pub version: i64,
    pub vin: Vec<GetRawTransactionVerbose1VinItem>,
    pub vout: Vec<GetRawTransactionVerbose1VoutItem>,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: i64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerbose1VinItem {
    /// The coinbase value (only if coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,
    /// The script (if not coinbase transaction)
    #[serde(rename = "scriptSig", skip_serializing_if = "Option::is_none")]
    pub script_sig: Option<GetRawTransactionVerbose1VinItemScriptSig>,
    /// The script sequence number
    pub sequence: i64,
    /// The transaction id (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    #[serde(rename = "txinwitness", skip_serializing_if = "Option::is_none")]
    pub txin_witness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vout: Option<i64>,
}

/// The script (if not coinbase transaction)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerbose1VinItemScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerbose1VoutItem {
    /// index
    pub n: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetRawTransactionVerbose1VoutItemScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerbose1VoutItemScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub type_: String,
}

/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
///
/// Hint: Use gettransaction for wallet transactions.
///
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerbose2 {
    /// the block hash
    #[serde(rename = "blockhash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    /// The block time expressed in UNIX epoch time
    #[serde(rename = "blocktime", skip_serializing_if = "Option::is_none")]
    pub block_time: Option<i64>,
    /// The confirmations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmations: Option<i64>,
    /// transaction fee in BTC, omitted if block undo data is not available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The serialized, hex-encoded data for 'txid'
    pub hex: String,
    /// Whether specified block is in the active chain or not (only present with explicit "blockhash" argument)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_active_chain: Option<bool>,
    /// The lock time
    pub locktime: i64,
    /// The serialized transaction size
    pub size: i64,
    /// Same as "blocktime"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// The transaction id
    pub txid: String,
    /// The version
    pub version: i64,
    pub vin: Vec<GetRawTransactionVerbose2VinItem>,
    pub vout: Vec<GetRawTransactionVerbose2VoutItem>,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: i64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerbose2VinItem {
    /// The coinbase value (only if coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,
    /// The previous output, omitted if block undo data is not available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevout: Option<GetRawTransactionVerbose2VinItemPrevout>,
    /// The script (if not coinbase transaction)
    #[serde(rename = "scriptSig", skip_serializing_if = "Option::is_none")]
    pub script_sig: Option<GetRawTransactionVerbose2VinItemScriptSig>,
    /// The script sequence number
    pub sequence: i64,
    /// The transaction id (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    #[serde(rename = "txinwitness", skip_serializing_if = "Option::is_none")]
    pub txin_witness: Option<Vec<String>>,
    /// The output number (if not coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vout: Option<i64>,
}

/// The previous output, omitted if block undo data is not available
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerbose2VinItemPrevout {
    /// Coinbase or not
    pub generated: bool,
    /// The height of the prevout
    pub height: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetRawTransactionVerbose2VinItemPrevoutScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerbose2VinItemPrevoutScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub type_: String,
}

/// The script (if not coinbase transaction)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerbose2VinItemScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerbose2VoutItem {
    /// index
    pub n: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetRawTransactionVerbose2VoutItemScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawTransactionVerbose2VoutItemScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type (one of: nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    #[serde(rename = "type")]
    pub type_: String,
}

/// Result of the JSON-RPC method `joinpsbts`.
///
/// > joinpsbts
/// >
/// > Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
/// > No input in any of the PSBTs can be in more than one of the PSBTs.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct JoinPsbts(pub String);

impl std::ops::Deref for JoinPsbts {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Result of the JSON-RPC method `sendrawtransaction`.
///
/// > sendrawtransaction
/// >
/// > Submit a raw transaction (serialized, hex-encoded) to the network.
/// >
/// > If -privatebroadcast is disabled, then the transaction will be put into the
/// > local mempool of the node and will be sent unconditionally to all currently
/// > connected peers, so using sendrawtransaction for manual rebroadcast will degrade
/// > privacy by leaking the transaction's origin, as nodes will normally not
/// > rebroadcast non-wallet transactions already in their mempool.
/// >
/// > If -privatebroadcast is enabled, then the transaction will be sent only via
/// > dedicated, short-lived connections to Tor or I2P peers or IPv4/IPv6 peers
/// > via the Tor network. This conceals the transaction's origin. The transaction
/// > will only enter the local mempool when it is received back from the network.
/// >
/// > A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
/// >
/// > Related RPCs: createrawtransaction, signrawtransactionwithkey
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendRawTransaction(pub String);

impl std::ops::Deref for SendRawTransaction {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionWithKey {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<SignRawTransactionWithKeyErrorsItem>>,
    /// The hex-encoded raw transaction with signature(s)
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SignRawTransactionWithKeyErrorsItem {
    /// Verification or signing error related to the input
    pub error: String,
    /// The hex-encoded signature script
    #[serde(rename = "scriptSig")]
    pub script_sig: String,
    /// Script sequence number
    pub sequence: i64,
    /// The hash of the referenced, previous transaction
    pub txid: String,
    /// The index of the output to spent and used as input
    pub vout: i64,
    pub witness: Vec<String>,
}

/// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackage {
    /// The transaction package result message. "success" indicates all transactions were accepted into or are already in the mempool.
    pub package_msg: String,
    /// List of txids of replaced transactions
    #[serde(rename = "replaced-transactions", skip_serializing_if = "Option::is_none")]
    pub replaced_transactions: Option<Vec<String>>,
    /// The transaction results keyed by wtxid. An entry is returned for every submitted wtxid.
    #[serde(rename = "tx-results")]
    pub tx_results: std::collections::BTreeMap<String, SubmitPackageTxResults>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackageTxResults {
    /// Error string if rejected from mempool, or "package-not-validated" when the package aborts before any per-tx processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Transaction fees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<SubmitPackageTxResultsFees>,
    /// The wtxid of a different transaction with the same txid but different witness found in the mempool. This means the submitted transaction was ignored.
    #[serde(rename = "other-wtxid", skip_serializing_if = "Option::is_none")]
    pub other_wtxid: Option<String>,
    /// The transaction hash in hex
    pub txid: String,
    /// Sigops-adjusted virtual transaction size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsize: Option<i64>,
}

/// Transaction fees
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SubmitPackageTxResultsFees {
    /// transaction fee in BTC
    pub base: f64,
    /// if the transaction was not already in the mempool, the effective feerate in BTC per KvB. For example, the package feerate and/or feerate with modified fees from prioritisetransaction.
    #[serde(rename = "effective-feerate", skip_serializing_if = "Option::is_none")]
    pub effective_fee_rate: Option<f64>,
    /// if effective-feerate is provided, the wtxids of the transactions whose fees and vsizes are included in effective-feerate.
    #[serde(rename = "effective-includes", skip_serializing_if = "Option::is_none")]
    pub effective_includes: Option<Vec<String>>,
}

/// Result of the JSON-RPC method `testmempoolaccept`.
///
/// > testmempoolaccept
/// >
/// > Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
/// >
/// > If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
/// >
/// > If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
/// >
/// > The maximum number of transactions allowed is 25.
/// >
/// > This checks if transactions violate the consensus or policy rules.
/// >
/// > See sendrawtransaction call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TestMempoolAccept(pub Vec<TestMempoolAcceptItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TestMempoolAcceptItem {
    /// Whether this tx would be accepted to the mempool and pass client-specified maxfeerate. If not present, the tx was not fully validated due to a failure in another tx in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed: Option<bool>,
    /// Transaction fees (only present if 'allowed' is true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<TestMempoolAcceptItemFees>,
    /// Package validation error, if any (only possible if rawtxs had more than 1 transaction).
    #[serde(rename = "package-error", skip_serializing_if = "Option::is_none")]
    pub package_error: Option<String>,
    /// Rejection details (only present when 'allowed' is false and rejection details exist)
    #[serde(rename = "reject-details", skip_serializing_if = "Option::is_none")]
    pub reject_details: Option<String>,
    /// Rejection reason (only present when 'allowed' is false)
    #[serde(rename = "reject-reason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    /// The transaction hash in hex
    pub txid: String,
    /// Virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted (only present when 'allowed' is true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsize: Option<i64>,
    /// The transaction witness hash in hex
    pub wtxid: String,
}

/// Transaction fees (only present if 'allowed' is true)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TestMempoolAcceptItemFees {
    /// transaction fee in BTC
    pub base: f64,
    /// the effective feerate in BTC per KvB. May differ from the base feerate if, for example, there are modified fees from prioritisetransaction or a package feerate was used.
    #[serde(rename = "effective-feerate")]
    pub effective_fee_rate: f64,
    /// transactions whose fees and vsizes are included in effective-feerate.
    #[serde(rename = "effective-includes")]
    pub effective_includes: Vec<String>,
}

/// Result of the JSON-RPC method `utxoupdatepsbt`.
///
/// > utxoupdatepsbt
/// >
/// > Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct UtxoUpdatePsbt(pub String);

impl std::ops::Deref for UtxoUpdatePsbt {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}
