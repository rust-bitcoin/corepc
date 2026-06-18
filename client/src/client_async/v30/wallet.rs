// SPDX-License-Identifier: CC0-1.0

//! Auto-generated method wrappers for Bitcoin Core `30` - wallet.
//!
//! Produced by `codegen`. Do not edit by hand, re-run
//! `just codegen` to regenerate. Defines the `*Options` request structs these methods
//! consume; the response types live in the `corepc-types` crate
//! (`types::v30::generated`).

#![allow(unused_imports, clippy::needless_pass_by_value, clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use serde_json::json;
use types::v30::generated::{
    AbortRescan, BumpFee, CreateWallet, CreateWalletDescriptor, EncryptWallet, GetAddressInfo,
    GetAddressesByLabel, GetBalance, GetBalances, GetHdKeys, GetNewAddress, GetRawChangeAddress,
    GetReceivedByAddress, GetReceivedByLabel, GetTransaction, GetWalletInfo, ImportDescriptors,
    ListAddressGroupings, ListDescriptors, ListLabels, ListLockUnspent, ListReceivedByAddress,
    ListReceivedByLabel, ListSinceBlock, ListTransactions, ListUnspent, ListWalletDir, ListWallets,
    LoadWallet, LockUnspent, MigrateWallet, PsbtBumpFee, RescanBlockchain, RestoreWallet, SendAll,
    SendManyVerbose0, SendManyVerbose1, SendResult, SendToAddressVerbose0, SendToAddressVerbose1,
    SetTxFee, SetWalletFlag, SignMessage, SignRawTransactionWithWallet, SimulateRawTransaction,
    UnloadWallet, WalletCreateFundedPsbt, WalletDisplayAddress, WalletProcessPsbt,
};

use crate::client_async::error::Result;
use crate::client_async::Client;

///
/// Specify a fee rate in sat/vB instead of relying on the built-in fee estimator.
/// Must be at least 0.100 sat/vB higher than the current transaction fee rate.
/// WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BumpFeeOptionsFeeRate {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BumpFeeOptionsOutputs {
    Object(std::collections::BTreeMap<String, BumpFeeOptionsOutputsVariant0>),
    Object2(BumpFeeOptionsOutputsVariant1),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BumpFeeOptionsOutputsVariant0 {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BumpFeeOptionsOutputsVariant1 {
    /// A key-value pair. The key must be "data", the value is hex-encoded data that becomes a part of an OP_RETURN output
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ImportDescriptorsRequests {
    /// Set this descriptor to be the active descriptor for the corresponding output type/externality
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Descriptor to import.
    pub desc: String,
    /// Whether matching outputs should be treated as not incoming payments (e.g. change)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    /// Label to assign to the address, only allowed with internal=false. Disabled for ranged descriptors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// If a ranged descriptor is set to active, this specifies the next index to generate addresses from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_index: Option<i64>,
    /// If a ranged descriptor is used, this specifies the end or the range (in the form \[begin,end\]) to import
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<ImportDescriptorsRequestsRange>,
    /// Time from which to start rescanning the blockchain for this descriptor, in UNIX epoch time
    /// Use the string "now" to substitute the current synced blockchain time.
    /// "now" can be specified to bypass scanning, for outputs which are known to never have been used, and
    /// 0 can be specified to scan the entire blockchain. Blocks up to 2 hours before the earliest timestamp
    /// of all descriptors being imported will be scanned as well as the mempool.
    pub timestamp: i64,
}

/// If a ranged descriptor is used, this specifies the end or the range (in the form \[begin,end\]) to import
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ImportDescriptorsRequestsRange {
    Number(f64),
    List(Vec<serde_json::Value>),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListUnspentQueryOptions {
    /// Include immature coinbase UTXOs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_immature_coinbase: Option<bool>,
    /// Maximum value of each UTXO in BTC
    #[serde(rename = "maximumAmount", skip_serializing_if = "Option::is_none")]
    pub maximum_amount: Option<ListUnspentQueryOptionsMaximumAmount>,
    /// Maximum number of UTXOs
    #[serde(rename = "maximumCount", skip_serializing_if = "Option::is_none")]
    pub maximum_count: Option<i64>,
    /// Minimum value of each UTXO in BTC
    #[serde(rename = "minimumAmount", skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<ListUnspentQueryOptionsMinimumAmount>,
    /// Minimum sum value of all UTXOs in BTC
    #[serde(rename = "minimumSumAmount", skip_serializing_if = "Option::is_none")]
    pub minimum_sum_amount: Option<ListUnspentQueryOptionsMinimumSumAmount>,
}

/// Maximum value of each UTXO in BTC
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ListUnspentQueryOptionsMaximumAmount {
    Number(f64),
    Text(String),
}

/// Minimum value of each UTXO in BTC
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ListUnspentQueryOptionsMinimumAmount {
    Number(f64),
    Text(String),
}

/// Minimum sum value of all UTXOs in BTC
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ListUnspentQueryOptionsMinimumSumAmount {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LockUnspentTransactions {
    /// The transaction id
    pub txid: String,
    /// The output number
    pub vout: i64,
}

///
/// Specify a fee rate in sat/vB instead of relying on the built-in fee estimator.
/// Must be at least 0.100 sat/vB higher than the current transaction fee rate.
/// WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PsbtBumpFeeOptionsFeeRate {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PsbtBumpFeeOptionsOutputs {
    Object(std::collections::BTreeMap<String, PsbtBumpFeeOptionsOutputsVariant0>),
    Object2(PsbtBumpFeeOptionsOutputsVariant1),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PsbtBumpFeeOptionsOutputsVariant0 {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PsbtBumpFeeOptionsOutputsVariant1 {
    /// A key-value pair. The key must be "data", the value is hex-encoded data that becomes a part of an OP_RETURN output
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SendAllFeeRate {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendAllOptionsArg {
    /// When false, returns the serialized transaction without broadcasting or adding it to the wallet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_to_wallet: Option<bool>,
    /// Confirmation target in blocks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conf_target: Option<i64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_mode: Option<String>,
    /// Specify a fee rate in sat/vB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<SendAllOptionsArgFeeRate>,
    /// (DEPRECATED) No longer used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_watching: Option<bool>,
    /// Use exactly the specified inputs to build the transaction. Specifying inputs is incompatible with the send_max, minconf, and maxconf options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<SendAllOptionsArgInputsItem>>,
    /// Lock selected unspent outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_unspents: Option<bool>,
    /// Raw locktime. Non-0 value also locktime-activates inputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locktime: Option<i64>,
    /// Require inputs with at most this many confirmations.
    #[serde(rename = "maxconf", skip_serializing_if = "Option::is_none")]
    pub max_conf: Option<i64>,
    /// Require inputs with at least this many confirmations.
    #[serde(rename = "minconf", skip_serializing_if = "Option::is_none")]
    pub min_conf: Option<i64>,
    /// Always return a PSBT, implies add_to_wallet=false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<bool>,
    /// Marks this transaction as BIP125-replaceable.
    /// Allows this transaction to be replaced by a transaction with higher fees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaceable: Option<bool>,
    /// When true, only use UTXOs that can pay for their own fees to maximize the output amount. When 'false' (default), no UTXO is left behind. send_max is incompatible with providing specific inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_max: Option<bool>,
    /// Keys and scripts needed for producing a final transaction with a dummy signature.
    /// Used for fee estimation during coin selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solving_data: Option<SendAllOptionsArgSolvingData>,
    /// Transaction version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// Specify a fee rate in sat/vB.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SendAllOptionsArgFeeRate {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendAllOptionsArgInputsItem {
    /// The sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i64>,
    /// The transaction id
    pub txid: String,
    /// The output number
    pub vout: i64,
}

/// Keys and scripts needed for producing a final transaction with a dummy signature.
/// Used for fee estimation during coin selection.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendAllOptionsArgSolvingData {
    /// Descriptors that provide solving data for this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<Vec<String>>,
    /// Public keys involved in this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkeys: Option<Vec<String>>,
    /// Scripts involved in this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SendAllRecipients {
    Text(String),
    Object(std::collections::BTreeMap<String, SendAllRecipientsVariant1>),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SendAllRecipientsVariant1 {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SendManyAmounts {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SendManyFeeRate {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SendResultFeeRate {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendResultOptionsArg {
    /// Automatically include coins from the wallet to cover the target amount.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_inputs: Option<bool>,
    /// When false, returns a serialized transaction which will not be added to the wallet or broadcast
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_to_wallet: Option<bool>,
    /// The bitcoin address to receive the change
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_address: Option<String>,
    /// The index of the change output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_position: Option<i64>,
    /// The output type to use. Only valid if change_address is not specified. Options are "legacy", "p2sh-segwit", "bech32", "bech32m".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    /// Confirmation target in blocks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conf_target: Option<i64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_mode: Option<String>,
    /// Specify a fee rate in sat/vB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<SendResultOptionsArgFeeRate>,
    /// Include inputs that are not safe to spend (unconfirmed transactions from outside keys and unconfirmed replacement transactions).
    /// Warning: the resulting transaction may become invalid if one of the unsafe inputs disappears.
    /// If that happens, you will need to fund the transaction with different inputs and republish it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_unsafe: Option<bool>,
    /// (DEPRECATED) No longer used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_watching: Option<bool>,
    /// Specify inputs instead of adding them automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<SendResultOptionsArgInputsItem>>,
    /// Lock selected unspent outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_unspents: Option<bool>,
    /// Raw locktime. Non-0 value also locktime-activates inputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locktime: Option<i64>,
    /// The maximum acceptable transaction weight.
    /// Transaction building will fail if this can not be satisfied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tx_weight: Option<i64>,
    /// If add_inputs is specified, require inputs with at most this many confirmations.
    #[serde(rename = "maxconf", skip_serializing_if = "Option::is_none")]
    pub max_conf: Option<i64>,
    /// If add_inputs is specified, require inputs with at least this many confirmations.
    #[serde(rename = "minconf", skip_serializing_if = "Option::is_none")]
    pub min_conf: Option<i64>,
    /// Always return a PSBT, implies add_to_wallet=false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psbt: Option<bool>,
    /// Marks this transaction as BIP125-replaceable.
    /// Allows this transaction to be replaced by a transaction with higher fees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaceable: Option<bool>,
    /// Keys and scripts needed for producing a final transaction with a dummy signature.
    /// Used for fee estimation during coin selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solving_data: Option<SendResultOptionsArgSolvingData>,
    /// Outputs to subtract the fee from, specified as integer indices.
    /// The fee will be equally deducted from the amount of each specified output.
    /// Those recipients will receive less bitcoins than you enter in their corresponding amount field.
    /// If no outputs are specified here, the sender pays the fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtract_fee_from_outputs: Option<Vec<i64>>,
}

/// Specify a fee rate in sat/vB.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SendResultOptionsArgFeeRate {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendResultOptionsArgInputsItem {
    /// The sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i64>,
    /// The transaction id
    pub txid: String,
    /// The output number
    pub vout: i64,
    /// The maximum weight for this input, including the weight of the outpoint and sequence number. Note that signature sizes are not guaranteed to be consistent, so the maximum DER signatures size of 73 bytes should be used when considering ECDSA signatures.Remember to convert serialized sizes to weight units when necessary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// Keys and scripts needed for producing a final transaction with a dummy signature.
/// Used for fee estimation during coin selection.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendResultOptionsArgSolvingData {
    /// Descriptors that provide solving data for this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<Vec<String>>,
    /// Public keys involved in this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkeys: Option<Vec<String>>,
    /// Scripts involved in this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SendResultOutputs {
    Object(std::collections::BTreeMap<String, SendResultOutputsVariant0>),
    Object2(SendResultOutputsVariant1),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SendResultOutputsVariant0 {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendResultOutputsVariant1 {
    /// A key-value pair. The key must be "data", the value is hex-encoded data that becomes a part of an OP_RETURN output
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SendToAddressAmount {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SendToAddressFeeRate {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SetTxFeeAmount {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SignRawTransactionWithWalletPrevTxs {
    /// (required for Segwit inputs) the amount spent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<SignRawTransactionWithWalletPrevTxsAmount>,
    /// (required for P2SH) redeem script
    #[serde(rename = "redeemScript", skip_serializing_if = "Option::is_none")]
    pub redeem_script: Option<String>,
    /// The output script
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: String,
    /// The transaction id
    pub txid: String,
    /// The output number
    pub vout: i64,
    /// (required for P2WSH or P2SH-P2WSH) witness script
    #[serde(rename = "witnessScript", skip_serializing_if = "Option::is_none")]
    pub witness_script: Option<String>,
}

/// (required for Segwit inputs) the amount spent
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SignRawTransactionWithWalletPrevTxsAmount {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SimulateRawTransactionOptionsArg {
    /// (DEPRECATED) No longer used
    #[serde(rename = "include_watchonly", skip_serializing_if = "Option::is_none")]
    pub include_watch_only: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletCreateFundedPsbtInputs {
    /// The sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i64>,
    /// The transaction id
    pub txid: String,
    /// The output number
    pub vout: i64,
    /// The maximum weight for this input, including the weight of the outpoint and sequence number. Note that signature sizes are not guaranteed to be consistent, so the maximum DER signatures size of 73 bytes should be used when considering ECDSA signatures.Remember to convert serialized sizes to weight units when necessary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletCreateFundedPsbtOptionsArg {
    /// Automatically include coins from the wallet to cover the target amount.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_inputs: Option<bool>,
    /// The bitcoin address to receive the change
    #[serde(rename = "changeAddress", skip_serializing_if = "Option::is_none")]
    pub change_address: Option<String>,
    /// The index of the change output
    #[serde(rename = "changePosition", skip_serializing_if = "Option::is_none")]
    pub change_position: Option<i64>,
    /// The output type to use. Only valid if changeAddress is not specified. Options are "legacy", "p2sh-segwit", "bech32", "bech32m".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    /// Confirmation target in blocks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conf_target: Option<i64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_mode: Option<String>,
    /// Specify a fee rate in BTC/kvB.
    #[serde(rename = "feeRate", skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<WalletCreateFundedPsbtOptionsArgFeeRate>,
    /// Specify a fee rate in sat/vB.
    #[serde(rename = "fee_rate", skip_serializing_if = "Option::is_none")]
    pub fee_rate2: Option<WalletCreateFundedPsbtOptionsArgFeeRate>,
    /// (DEPRECATED) No longer used
    #[serde(rename = "includeWatching", skip_serializing_if = "Option::is_none")]
    pub include_watching: Option<bool>,
    /// Include inputs that are not safe to spend (unconfirmed transactions from outside keys and unconfirmed replacement transactions).
    /// Warning: the resulting transaction may become invalid if one of the unsafe inputs disappears.
    /// If that happens, you will need to fund the transaction with different inputs and republish it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_unsafe: Option<bool>,
    /// Lock selected unspent outputs
    #[serde(rename = "lockUnspents", skip_serializing_if = "Option::is_none")]
    pub lock_unspents: Option<bool>,
    /// The maximum acceptable transaction weight.
    /// Transaction building will fail if this can not be satisfied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tx_weight: Option<i64>,
    /// If add_inputs is specified, require inputs with at most this many confirmations.
    #[serde(rename = "maxconf", skip_serializing_if = "Option::is_none")]
    pub max_conf: Option<i64>,
    /// If add_inputs is specified, require inputs with at least this many confirmations.
    #[serde(rename = "minconf", skip_serializing_if = "Option::is_none")]
    pub min_conf: Option<i64>,
    /// Marks this transaction as BIP125-replaceable.
    /// Allows this transaction to be replaced by a transaction with higher fees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaceable: Option<bool>,
    /// Keys and scripts needed for producing a final transaction with a dummy signature.
    /// Used for fee estimation during coin selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solving_data: Option<WalletCreateFundedPsbtOptionsArgSolvingData>,
    /// The outputs to subtract the fee from.
    /// The fee will be equally deducted from the amount of each specified output.
    /// Those recipients will receive less bitcoins than you enter in their corresponding amount field.
    /// If no outputs are specified here, the sender pays the fee.
    #[serde(rename = "subtractFeeFromOutputs", skip_serializing_if = "Option::is_none")]
    pub subtract_fee_from_outputs: Option<Vec<i64>>,
}

/// Specify a fee rate in BTC/kvB.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WalletCreateFundedPsbtOptionsArgFeeRate {
    Number(f64),
    Text(String),
}

/// Keys and scripts needed for producing a final transaction with a dummy signature.
/// Used for fee estimation during coin selection.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletCreateFundedPsbtOptionsArgSolvingData {
    /// Descriptors that provide solving data for this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<Vec<String>>,
    /// Public keys involved in this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubkeys: Option<Vec<String>>,
    /// Scripts involved in this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WalletCreateFundedPsbtOutputs {
    Object(std::collections::BTreeMap<String, WalletCreateFundedPsbtOutputsVariant0>),
    Object2(WalletCreateFundedPsbtOutputsVariant1),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WalletCreateFundedPsbtOutputsVariant0 {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletCreateFundedPsbtOutputsVariant1 {
    /// A key-value pair. The key must be "data", the value is hex-encoded data that becomes a part of an OP_RETURN output
    pub data: String,
}

/// Optional parameters for the `bumpfee` JSON-RPC method (consumed by `Client::bump_fee_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BumpFeeOptions {
    /// Confirmation target in blocks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conf_target: Option<i64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_mode: Option<String>,
    /// Specify a fee rate in sat/vB instead of relying on the built-in fee estimator.
    /// Must be at least 0.100 sat/vB higher than the current transaction fee rate.
    /// WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<BumpFeeOptionsFeeRate>,
    /// The 0-based index of the change output on the original transaction. The indicated output will be recycled into the new change output on the bumped transaction. The remainder after paying the recipients and fees will be sent to the output script of the original change output. The change output’s amount can increase if bumping the transaction adds new inputs, otherwise it will decrease. Cannot be used in combination with the 'outputs' option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_change_index: Option<f64>,
    /// The outputs specified as key-value pairs.
    /// Each key may only appear once, i.e. there can only be one 'data' output, and no address may be duplicated.
    /// At least one output of either type must be specified.
    /// Cannot be provided if 'original_change_index' is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<BumpFeeOptionsOutputs>>,
    /// Whether the new transaction should be
    /// marked bip-125 replaceable. If true, the sequence numbers in the transaction will
    /// be set to 0xfffffffd. If false, any input sequence numbers in the
    /// transaction will be set to 0xfffffffe
    /// so the new transaction will not be explicitly bip-125 replaceable (though it may
    /// still be replaceable in practice, for example if it has unconfirmed ancestors which
    /// are replaceable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaceable: Option<bool>,
}

/// Optional parameters for the `createwallet` JSON-RPC method (consumed by `Client::create_wallet_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletOptions {
    /// Disable the possibility of private keys (only watchonlys are possible in this mode).
    pub disable_private_keys: Option<bool>,
    /// Create a blank wallet. A blank wallet has no keys.
    pub blank: Option<bool>,
    /// Encrypt the wallet with this passphrase.
    pub passphrase: Option<String>,
    /// Keep track of coin reuse, and treat dirty and clean coins differently with privacy considerations in mind.
    pub avoid_reuse: Option<bool>,
    /// If set, must be "true"
    pub descriptors: Option<bool>,
    /// Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
    pub load_on_startup: Option<bool>,
    /// Use an external signer such as a hardware wallet. Requires -signer to be configured. Wallet creation will fail if keys cannot be fetched. Requires disable_private_keys and descriptors set to true.
    pub external_signer: Option<bool>,
}

/// Optional parameters for the `createwalletdescriptor` JSON-RPC method (consumed by `Client::create_wallet_descriptor_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct CreateWalletDescriptorOptions {
    /// The HD key that the wallet knows the private key of, listed using 'gethdkeys', to use for this descriptor's key
    #[serde(skip_serializing_if = "Option::is_none", rename = "hdkey")]
    pub hd_key: Option<String>,
    /// Whether to only make one descriptor that is internal (if parameter is true) or external (if parameter is false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
}

/// Optional parameters for the `getbalance` JSON-RPC method (consumed by `Client::get_balance_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceOptions {
    /// Remains for backward compatibility. Must be excluded or set to "*".
    pub dummy: Option<String>,
    /// Only include transactions confirmed at least this many times.
    pub min_conf: Option<i64>,
    /// No longer used
    pub include_watch_only: Option<bool>,
    /// (only available if avoid_reuse wallet flag is set) Do not include balance in dirty outputs; addresses are considered dirty if they have previously been used in a transaction.
    pub avoid_reuse: Option<bool>,
}

/// Optional parameters for the `gethdkeys` JSON-RPC method (consumed by `Client::get_hd_keys_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GetHdKeysOptions {
    /// Show the keys for only active descriptors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_only: Option<bool>,
    /// Show private keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
}

/// Optional parameters for the `getnewaddress` JSON-RPC method (consumed by `Client::get_new_address_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNewAddressOptions {
    /// The label name for the address to be linked to. It can also be set to the empty string "" to represent the default label. The label does not need to exist, it will be created if there is no label by the given name.
    pub label: Option<String>,
    /// The address type to use. Options are "legacy", "p2sh-segwit", "bech32", "bech32m".
    pub address_type: Option<String>,
}

/// Optional parameters for the `getrawchangeaddress` JSON-RPC method (consumed by `Client::get_raw_change_address_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRawChangeAddressOptions {
    /// The address type to use. Options are "legacy", "p2sh-segwit", "bech32", "bech32m".
    pub address_type: Option<String>,
}

/// Optional parameters for the `getreceivedbyaddress` JSON-RPC method (consumed by `Client::get_received_by_address_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReceivedByAddressOptions {
    /// Only include transactions confirmed at least this many times.
    pub min_conf: Option<i64>,
    /// Include immature coinbase transactions.
    pub include_immature_coinbase: Option<bool>,
}

/// Optional parameters for the `getreceivedbylabel` JSON-RPC method (consumed by `Client::get_received_by_label_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReceivedByLabelOptions {
    /// Only include transactions confirmed at least this many times.
    pub min_conf: Option<i64>,
    /// Include immature coinbase transactions.
    pub include_immature_coinbase: Option<bool>,
}

/// Optional parameters for the `gettransaction` JSON-RPC method (consumed by `Client::get_transaction_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionOptions {
    /// (DEPRECATED) No longer used
    pub include_watch_only: Option<bool>,
    /// Whether to include a `decoded` field containing the decoded transaction (equivalent to RPC decoderawtransaction)
    pub verbose: Option<bool>,
}

/// Optional parameters for the `keypoolrefill` JSON-RPC method (consumed by `Client::keypool_refill_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeypoolRefillOptions {
    /// The new keypool size
    pub new_size: Option<f64>,
}

/// Optional parameters for the `listdescriptors` JSON-RPC method (consumed by `Client::list_descriptors_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDescriptorsOptions {
    /// Show private descriptors.
    pub private: Option<bool>,
}

/// Optional parameters for the `listlabels` JSON-RPC method (consumed by `Client::list_labels_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLabelsOptions {
    /// Address purpose to list labels for ('send','receive'). An empty string is the same as not providing this argument.
    pub purpose: Option<String>,
}

/// Optional parameters for the `listreceivedbyaddress` JSON-RPC method (consumed by `Client::list_received_by_address_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListReceivedByAddressOptions {
    /// The minimum number of confirmations before payments are included.
    pub min_conf: Option<i64>,
    /// Whether to include addresses that haven't received any payments.
    pub include_empty: Option<bool>,
    /// (DEPRECATED) No longer used
    pub include_watch_only: Option<bool>,
    /// If present and non-empty, only return information on this address.
    pub address_filter: Option<String>,
    /// Include immature coinbase transactions.
    pub include_immature_coinbase: Option<bool>,
}

/// Optional parameters for the `listreceivedbylabel` JSON-RPC method (consumed by `Client::list_received_by_label_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListReceivedByLabelOptions {
    /// The minimum number of confirmations before payments are included.
    pub min_conf: Option<i64>,
    /// Whether to include labels that haven't received any payments.
    pub include_empty: Option<bool>,
    /// (DEPRECATED) No longer used
    pub include_watch_only: Option<bool>,
    /// Include immature coinbase transactions.
    pub include_immature_coinbase: Option<bool>,
}

/// Optional parameters for the `listsinceblock` JSON-RPC method (consumed by `Client::list_since_block_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSinceBlockOptions {
    /// If set, the block hash to list transactions since, otherwise list all transactions.
    pub block_hash: Option<String>,
    /// Return the nth block hash from the main chain. e.g. 1 would mean the best block hash. Note: this is not used as a filter, but only affects \[lastblock\] in the return value
    pub target_confirmations: Option<f64>,
    /// (DEPRECATED) No longer used
    pub include_watch_only: Option<bool>,
    /// Show transactions that were removed due to a reorg in the "removed" array
    /// (not guaranteed to work on pruned nodes)
    pub include_removed: Option<bool>,
    /// Also add entries for change outputs.
    pub include_change: Option<bool>,
    /// Return only incoming transactions paying to addresses with the specified label.
    pub label: Option<String>,
}

/// Optional parameters for the `listtransactions` JSON-RPC method (consumed by `Client::list_transactions_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactionsOptions {
    /// If set, should be a valid label name to return only incoming transactions
    /// with the specified label, or "*" to disable filtering and return all transactions.
    pub label: Option<String>,
    /// The number of transactions to return
    pub count: Option<i64>,
    /// The number of transactions to skip
    pub skip: Option<i64>,
    /// (DEPRECATED) No longer used
    pub include_watch_only: Option<bool>,
}

/// Optional parameters for the `listunspent` JSON-RPC method (consumed by `Client::list_unspent_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUnspentOptions {
    /// The minimum confirmations to filter
    pub min_conf: Option<i64>,
    /// The maximum confirmations to filter
    pub max_conf: Option<i64>,
    /// The bitcoin addresses to filter
    pub addresses: Option<Vec<String>>,
    /// Include outputs that are not safe to spend
    /// See description of "safe" attribute below.
    pub include_unsafe: Option<bool>,
    pub query_options: Option<ListUnspentQueryOptions>,
}

/// Optional parameters for the `loadwallet` JSON-RPC method (consumed by `Client::load_wallet_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadWalletOptions {
    /// Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
    pub load_on_startup: Option<bool>,
}

/// Optional parameters for the `lockunspent` JSON-RPC method (consumed by `Client::lock_unspent_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LockUnspentOptions {
    /// The transaction outputs and within each, the txid (string) vout (numeric).
    pub transactions: Option<Vec<LockUnspentTransactions>>,
    /// Whether to write/erase this lock in the wallet database, or keep the change in memory only. Ignored for unlocking.
    pub persistent: Option<bool>,
}

/// Optional parameters for the `migratewallet` JSON-RPC method (consumed by `Client::migrate_wallet_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrateWalletOptions {
    /// The name of the wallet to migrate. If provided both here and in the RPC endpoint, the two must be identical.
    pub wallet_name: Option<String>,
    /// The wallet passphrase
    pub passphrase: Option<String>,
}

/// Optional parameters for the `psbtbumpfee` JSON-RPC method (consumed by `Client::psbt_bump_fee_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PsbtBumpFeeOptions {
    /// Confirmation target in blocks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conf_target: Option<i64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimate_mode: Option<String>,
    /// Specify a fee rate in sat/vB instead of relying on the built-in fee estimator.
    /// Must be at least 0.100 sat/vB higher than the current transaction fee rate.
    /// WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<PsbtBumpFeeOptionsFeeRate>,
    /// The 0-based index of the change output on the original transaction. The indicated output will be recycled into the new change output on the bumped transaction. The remainder after paying the recipients and fees will be sent to the output script of the original change output. The change output’s amount can increase if bumping the transaction adds new inputs, otherwise it will decrease. Cannot be used in combination with the 'outputs' option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_change_index: Option<f64>,
    /// The outputs specified as key-value pairs.
    /// Each key may only appear once, i.e. there can only be one 'data' output, and no address may be duplicated.
    /// At least one output of either type must be specified.
    /// Cannot be provided if 'original_change_index' is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<PsbtBumpFeeOptionsOutputs>>,
    /// Whether the new transaction should be
    /// marked bip-125 replaceable. If true, the sequence numbers in the transaction will
    /// be set to 0xfffffffd. If false, any input sequence numbers in the
    /// transaction will be set to 0xfffffffe
    /// so the new transaction will not be explicitly bip-125 replaceable (though it may
    /// still be replaceable in practice, for example if it has unconfirmed ancestors which
    /// are replaceable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaceable: Option<bool>,
}

/// Optional parameters for the `rescanblockchain` JSON-RPC method (consumed by `Client::rescan_blockchain_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RescanBlockchainOptions {
    /// block height where the rescan should start
    pub start_height: Option<i64>,
    /// the last block height that should be scanned. If none is provided it will rescan up to the tip at return time of this call.
    pub stop_height: Option<i64>,
}

/// Optional parameters for the `restorewallet` JSON-RPC method (consumed by `Client::restore_wallet_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreWalletOptions {
    /// Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
    pub load_on_startup: Option<bool>,
}

/// Optional parameters for the `send` JSON-RPC method (consumed by `Client::send_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendResultOptions {
    /// Confirmation target in blocks
    pub conf_target: Option<i64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    pub estimate_mode: Option<String>,
    /// Specify a fee rate in sat/vB.
    pub fee_rate: Option<SendResultFeeRate>,
    pub options: Option<SendResultOptionsArg>,
    /// Transaction version
    pub version: Option<i64>,
}

/// Optional parameters for the `sendall` JSON-RPC method (consumed by `Client::send_all_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendAllOptions {
    /// Confirmation target in blocks
    pub conf_target: Option<i64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    pub estimate_mode: Option<String>,
    /// Specify a fee rate in sat/vB.
    pub fee_rate: Option<SendAllFeeRate>,
    pub options: Option<SendAllOptionsArg>,
}

/// Optional parameters for the `sendmany` JSON-RPC method (consumed by `Client::send_many_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendManyOptions {
    /// Must be set to "" for backwards compatibility.
    pub dummy: Option<String>,
    /// Ignored dummy value
    pub min_conf: Option<i64>,
    /// A comment
    pub comment: Option<String>,
    /// The addresses.
    /// The fee will be equally deducted from the amount of each selected address.
    /// Those recipients will receive less bitcoins than you enter in their corresponding amount field.
    /// If no addresses are specified here, the sender pays the fee.
    pub subtract_fee_from: Option<Vec<String>>,
    /// Signal that this transaction can be replaced by a transaction (BIP 125)
    pub replaceable: Option<bool>,
    /// Confirmation target in blocks
    pub conf_target: Option<i64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    pub estimate_mode: Option<String>,
    /// Specify a fee rate in sat/vB.
    pub fee_rate: Option<SendManyFeeRate>,
}

/// Optional parameters for the `sendtoaddress` JSON-RPC method (consumed by `Client::send_to_address_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendToAddressOptions {
    /// A comment used to store what the transaction is for.
    /// This is not part of the transaction, just kept in your wallet.
    pub comment: Option<String>,
    /// A comment to store the name of the person or organization
    /// to which you're sending the transaction. This is not part of the
    /// transaction, just kept in your wallet.
    pub comment_to: Option<String>,
    /// The fee will be deducted from the amount being sent.
    /// The recipient will receive less bitcoins than you enter in the amount field.
    pub subtract_fee_from_amount: Option<bool>,
    /// Signal that this transaction can be replaced by a transaction (BIP 125)
    pub replaceable: Option<bool>,
    /// Confirmation target in blocks
    pub conf_target: Option<i64>,
    /// The fee estimate mode, must be one of (case insensitive):
    /// unset, economical, conservative
    /// unset means no mode set (economical mode is used if the transaction is replaceable;
    /// otherwise, conservative mode is used).
    /// economical estimates use a shorter time horizon, making them more
    /// responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a lower fee rate estimate.
    /// conservative estimates use a longer time horizon, making them
    /// less responsive to short-term drops in the prevailing fee market. This mode
    /// potentially returns a higher fee rate estimate.
    pub estimate_mode: Option<String>,
    /// (only available if avoid_reuse wallet flag is set) Avoid spending from dirty addresses; addresses are considered
    /// dirty if they have previously been used in a transaction. If true, this also activates avoidpartialspends, grouping outputs by their addresses.
    pub avoid_reuse: Option<bool>,
    /// Specify a fee rate in sat/vB.
    pub fee_rate: Option<SendToAddressFeeRate>,
}

/// Optional parameters for the `setwalletflag` JSON-RPC method (consumed by `Client::set_wallet_flag_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetWalletFlagOptions {
    /// The new state.
    pub value: Option<bool>,
}

/// Optional parameters for the `signrawtransactionwithwallet` JSON-RPC method (consumed by `Client::sign_raw_transaction_with_wallet_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignRawTransactionWithWalletOptions {
    /// The previous dependent transaction outputs
    pub prev_txs: Option<Vec<SignRawTransactionWithWalletPrevTxs>>,
    /// The signature hash type. Must be one of
    ///        "DEFAULT"
    ///        "ALL"
    ///        "NONE"
    ///        "SINGLE"
    ///        "ALL|ANYONECANPAY"
    ///        "NONE|ANYONECANPAY"
    ///        "SINGLE|ANYONECANPAY"
    pub sig_hash_type: Option<String>,
}

/// Optional parameters for the `simulaterawtransaction` JSON-RPC method (consumed by `Client::simulate_raw_transaction_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateRawTransactionOptions {
    /// An array of hex strings of raw transactions.
    pub raw_txs: Option<Vec<String>>,
    pub options: Option<SimulateRawTransactionOptionsArg>,
}

/// Optional parameters for the `unloadwallet` JSON-RPC method (consumed by `Client::unload_wallet_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnloadWalletOptions {
    /// The name of the wallet to unload. If provided both here and in the RPC endpoint, the two must be identical.
    pub wallet_name: Option<String>,
    /// Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
    pub load_on_startup: Option<bool>,
}

/// Optional parameters for the `walletcreatefundedpsbt` JSON-RPC method (consumed by `Client::wallet_create_funded_psbt_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletCreateFundedPsbtOptions {
    /// Leave empty to add inputs automatically. See add_inputs option.
    pub inputs: Option<Vec<WalletCreateFundedPsbtInputs>>,
    /// Raw locktime. Non-0 value also locktime-activates inputs
    pub locktime: Option<i64>,
    pub options: Option<WalletCreateFundedPsbtOptionsArg>,
    /// Include BIP 32 derivation paths for public keys if we know them
    pub bip32derivs: Option<bool>,
    /// Transaction version
    pub version: Option<i64>,
}

/// Optional parameters for the `walletprocesspsbt` JSON-RPC method (consumed by `Client::wallet_process_psbt_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletProcessPsbtOptions {
    /// Also sign the transaction when updating (requires wallet to be unlocked)
    pub sign: Option<bool>,
    /// The signature hash type to sign with if not specified by the PSBT. Must be one of
    ///        "DEFAULT"
    ///        "ALL"
    ///        "NONE"
    ///        "SINGLE"
    ///        "ALL|ANYONECANPAY"
    ///        "NONE|ANYONECANPAY"
    ///        "SINGLE|ANYONECANPAY"
    pub sig_hash_type: Option<String>,
    /// Include BIP 32 derivation paths for public keys if we know them
    pub bip32derivs: Option<bool>,
    /// Also finalize inputs if possible
    pub finalize: Option<bool>,
}

impl Client {
    /// `abandontransaction` with required arguments only.
    ///
    /// Mark in-wallet transaction \<txid\> as abandoned
    /// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
    /// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
    /// It only works on transactions which are not included in a block and are not currently in the mempool.
    /// It has no effect on transactions which are already abandoned.
    pub async fn abandon_transaction(&self, txid: String) -> Result<()> {
        self.call_raw("abandontransaction", &[json!(txid)]).await
    }

    /// `abortrescan` with required arguments only.
    ///
    /// Stops current wallet rescan triggered by an RPC call, e.g. by a rescanblockchain call.
    /// Note: Use "getwalletinfo" to query the scanning progress.
    pub async fn abort_rescan(&self) -> Result<AbortRescan> {
        self.call_raw("abortrescan", &[(); 0] as &[()]).await
    }

    /// `backupwallet` with required arguments only.
    ///
    /// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
    pub async fn backup_wallet(&self, destination: String) -> Result<()> {
        self.call_raw("backupwallet", &[json!(destination)]).await
    }

    /// `bumpfee` with required arguments only.
    ///
    /// Bumps the fee of a transaction T, replacing it with a new transaction B.
    /// A transaction with the given txid must be in the wallet.
    /// The command will pay the additional fee by reducing change outputs or adding inputs when necessary.
    /// It may add a new change output if one does not already exist.
    /// All inputs in the original transaction will be included in the replacement transaction.
    /// The command will fail if the wallet or mempool contains a transaction that spends one of T's outputs.
    /// By default, the new fee will be calculated automatically using the estimatesmartfee RPC.
    /// The user can specify a confirmation target for estimatesmartfee.
    /// Alternatively, the user can specify a fee rate in sat/vB for the new transaction.
    /// At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
    /// returned by getnetworkinfo) to enter the node's mempool.
    /// * WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB. *
    pub async fn bump_fee(&self, txid: String) -> Result<BumpFee> {
        self.call_raw("bumpfee", &[json!(txid)]).await
    }

    /// `bumpfee` with all optional arguments via [`BumpFeeOptions`].
    ///
    /// Bumps the fee of a transaction T, replacing it with a new transaction B.
    /// A transaction with the given txid must be in the wallet.
    /// The command will pay the additional fee by reducing change outputs or adding inputs when necessary.
    /// It may add a new change output if one does not already exist.
    /// All inputs in the original transaction will be included in the replacement transaction.
    /// The command will fail if the wallet or mempool contains a transaction that spends one of T's outputs.
    /// By default, the new fee will be calculated automatically using the estimatesmartfee RPC.
    /// The user can specify a confirmation target for estimatesmartfee.
    /// Alternatively, the user can specify a fee rate in sat/vB for the new transaction.
    /// At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
    /// returned by getnetworkinfo) to enter the node's mempool.
    /// * WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB. *
    pub async fn bump_fee_with(&self, txid: String, opts: BumpFeeOptions) -> Result<BumpFee> {
        self.call_raw("bumpfee", &[json!(txid), json!(opts)]).await
    }

    /// `createwallet` with required arguments only.
    ///
    /// Creates and loads a new wallet.
    pub async fn create_wallet(&self, wallet_name: String) -> Result<CreateWallet> {
        self.call_raw("createwallet", &[json!(wallet_name)]).await
    }

    /// `createwallet` with all optional arguments via [`CreateWalletOptions`].
    ///
    /// Creates and loads a new wallet.
    pub async fn create_wallet_with(
        &self,
        wallet_name: String,
        opts: CreateWalletOptions,
    ) -> Result<CreateWallet> {
        self.call_raw(
            "createwallet",
            &[
                json!(wallet_name),
                json!(opts.disable_private_keys),
                json!(opts.blank),
                json!(opts.passphrase),
                json!(opts.avoid_reuse),
                json!(opts.descriptors),
                json!(opts.load_on_startup),
                json!(opts.external_signer),
            ],
        )
        .await
    }

    /// `createwalletdescriptor` with required arguments only.
    ///
    /// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn create_wallet_descriptor(&self, type_: String) -> Result<CreateWalletDescriptor> {
        self.call_raw("createwalletdescriptor", &[json!(type_)]).await
    }

    /// `createwalletdescriptor` with all optional arguments via [`CreateWalletDescriptorOptions`].
    ///
    /// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn create_wallet_descriptor_with(
        &self,
        type_: String,
        opts: CreateWalletDescriptorOptions,
    ) -> Result<CreateWalletDescriptor> {
        self.call_raw("createwalletdescriptor", &[json!(type_), json!(opts)]).await
    }

    /// `encryptwallet` with required arguments only.
    ///
    /// Encrypts the wallet with 'passphrase'. This is for first time encryption.
    /// After this, any calls that interact with private keys such as sending or signing
    /// will require the passphrase to be set prior to making these calls.
    /// Use the walletpassphrase call for this, and then walletlock call.
    /// If the wallet is already encrypted, use the walletpassphrasechange call.
    /// ** IMPORTANT **
    /// For security reasons, the encryption process will generate a new HD seed, resulting
    /// in the creation of a fresh set of active descriptors. Therefore, it is crucial to
    /// securely back up the newly generated wallet file using the backupwallet RPC.
    pub async fn encrypt_wallet(&self, passphrase: String) -> Result<EncryptWallet> {
        self.call_raw("encryptwallet", &[json!(passphrase)]).await
    }

    /// `getaddressesbylabel` with required arguments only.
    ///
    /// Returns the list of addresses assigned the specified label.
    pub async fn get_addresses_by_label(&self, label: String) -> Result<GetAddressesByLabel> {
        self.call_raw("getaddressesbylabel", &[json!(label)]).await
    }

    /// `getaddressinfo` with required arguments only.
    ///
    /// Return information about the given bitcoin address.
    /// Some of the information will only be present if the address is in the active wallet.
    pub async fn get_address_info(&self, address: String) -> Result<GetAddressInfo> {
        self.call_raw("getaddressinfo", &[json!(address)]).await
    }

    /// `getbalance` with required arguments only.
    ///
    /// Returns the total available balance.
    /// The available balance is what the wallet considers currently spendable, and is
    /// thus affected by options which limit spendability such as -spendzeroconfchange.
    pub async fn get_balance(&self) -> Result<GetBalance> {
        self.call_raw("getbalance", &[(); 0] as &[()]).await
    }

    /// `getbalance` with all optional arguments via [`GetBalanceOptions`].
    ///
    /// Returns the total available balance.
    /// The available balance is what the wallet considers currently spendable, and is
    /// thus affected by options which limit spendability such as -spendzeroconfchange.
    pub async fn get_balance_with(&self, opts: GetBalanceOptions) -> Result<GetBalance> {
        self.call_raw(
            "getbalance",
            &[
                json!(opts.dummy),
                json!(opts.min_conf),
                json!(opts.include_watch_only),
                json!(opts.avoid_reuse),
            ],
        )
        .await
    }

    /// `getbalances` with required arguments only.
    ///
    /// Returns an object with all balances in BTC.
    pub async fn get_balances(&self) -> Result<GetBalances> {
        self.call_raw("getbalances", &[(); 0] as &[()]).await
    }

    /// `gethdkeys` with required arguments only.
    ///
    /// List all BIP 32 HD keys in the wallet and which descriptors use them.
    pub async fn get_hd_keys(&self) -> Result<GetHdKeys> {
        self.call_raw("gethdkeys", &[(); 0] as &[()]).await
    }

    /// `gethdkeys` with all optional arguments via [`GetHdKeysOptions`].
    ///
    /// List all BIP 32 HD keys in the wallet and which descriptors use them.
    pub async fn get_hd_keys_with(&self, opts: GetHdKeysOptions) -> Result<GetHdKeys> {
        self.call_raw("gethdkeys", &[json!(opts)]).await
    }

    /// `getnewaddress` with required arguments only.
    ///
    /// Returns a new Bitcoin address for receiving payments.
    /// If 'label' is specified, it is added to the address book
    /// so payments received with the address will be associated with 'label'.
    pub async fn get_new_address(&self) -> Result<GetNewAddress> {
        self.call_raw("getnewaddress", &[(); 0] as &[()]).await
    }

    /// `getnewaddress` with all optional arguments via [`GetNewAddressOptions`].
    ///
    /// Returns a new Bitcoin address for receiving payments.
    /// If 'label' is specified, it is added to the address book
    /// so payments received with the address will be associated with 'label'.
    pub async fn get_new_address_with(&self, opts: GetNewAddressOptions) -> Result<GetNewAddress> {
        self.call_raw("getnewaddress", &[json!(opts.label), json!(opts.address_type)]).await
    }

    /// `getrawchangeaddress` with required arguments only.
    ///
    /// Returns a new Bitcoin address, for receiving change.
    /// This is for use with raw transactions, NOT normal use.
    pub async fn get_raw_change_address(&self) -> Result<GetRawChangeAddress> {
        self.call_raw("getrawchangeaddress", &[(); 0] as &[()]).await
    }

    /// `getrawchangeaddress` with all optional arguments via [`GetRawChangeAddressOptions`].
    ///
    /// Returns a new Bitcoin address, for receiving change.
    /// This is for use with raw transactions, NOT normal use.
    pub async fn get_raw_change_address_with(
        &self,
        opts: GetRawChangeAddressOptions,
    ) -> Result<GetRawChangeAddress> {
        self.call_raw("getrawchangeaddress", &[json!(opts.address_type)]).await
    }

    /// `getreceivedbyaddress` with required arguments only.
    ///
    /// Returns the total amount received by the given address in transactions with at least minconf confirmations.
    pub async fn get_received_by_address(&self, address: String) -> Result<GetReceivedByAddress> {
        self.call_raw("getreceivedbyaddress", &[json!(address)]).await
    }

    /// `getreceivedbyaddress` with all optional arguments via [`GetReceivedByAddressOptions`].
    ///
    /// Returns the total amount received by the given address in transactions with at least minconf confirmations.
    pub async fn get_received_by_address_with(
        &self,
        address: String,
        opts: GetReceivedByAddressOptions,
    ) -> Result<GetReceivedByAddress> {
        self.call_raw(
            "getreceivedbyaddress",
            &[json!(address), json!(opts.min_conf), json!(opts.include_immature_coinbase)],
        )
        .await
    }

    /// `getreceivedbylabel` with required arguments only.
    ///
    /// Returns the total amount received by addresses with \<label\> in transactions with at least \[minconf\] confirmations.
    pub async fn get_received_by_label(&self, label: String) -> Result<GetReceivedByLabel> {
        self.call_raw("getreceivedbylabel", &[json!(label)]).await
    }

    /// `getreceivedbylabel` with all optional arguments via [`GetReceivedByLabelOptions`].
    ///
    /// Returns the total amount received by addresses with \<label\> in transactions with at least \[minconf\] confirmations.
    pub async fn get_received_by_label_with(
        &self,
        label: String,
        opts: GetReceivedByLabelOptions,
    ) -> Result<GetReceivedByLabel> {
        self.call_raw(
            "getreceivedbylabel",
            &[json!(label), json!(opts.min_conf), json!(opts.include_immature_coinbase)],
        )
        .await
    }

    /// `gettransaction` with required arguments only.
    ///
    /// Get detailed information about in-wallet transaction \<txid\>
    pub async fn get_transaction(&self, txid: String) -> Result<GetTransaction> {
        self.call_raw("gettransaction", &[json!(txid)]).await
    }

    /// `gettransaction` with all optional arguments via [`GetTransactionOptions`].
    ///
    /// Get detailed information about in-wallet transaction \<txid\>
    pub async fn get_transaction_with(
        &self,
        txid: String,
        opts: GetTransactionOptions,
    ) -> Result<GetTransaction> {
        self.call_raw(
            "gettransaction",
            &[json!(txid), json!(opts.include_watch_only), json!(opts.verbose)],
        )
        .await
    }

    /// `getwalletinfo` with required arguments only.
    ///
    /// Returns an object containing various wallet state info.
    pub async fn get_wallet_info(&self) -> Result<GetWalletInfo> {
        self.call_raw("getwalletinfo", &[(); 0] as &[()]).await
    }

    /// `importdescriptors` with required arguments only.
    ///
    /// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
    /// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second element will be imported as an internal descriptor.
    ///
    /// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
    /// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
    /// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
    pub async fn import_descriptors(
        &self,
        requests: Vec<ImportDescriptorsRequests>,
    ) -> Result<ImportDescriptors> {
        self.call_raw("importdescriptors", &[json!(requests)]).await
    }

    /// `importprunedfunds` with required arguments only.
    ///
    /// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
    pub async fn import_pruned_funds(
        &self,
        raw_transaction: String,
        tx_out_proof: String,
    ) -> Result<()> {
        self.call_raw("importprunedfunds", &[json!(raw_transaction), json!(tx_out_proof)]).await
    }

    /// `keypoolrefill` with required arguments only.
    ///
    /// Refills each descriptor keypool in the wallet up to the specified number of new keys.
    /// By default, descriptor wallets have 4 active ranged descriptors ("legacy", "p2sh-segwit", "bech32", "bech32m"), each with 1000 entries.
    ///
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn keypool_refill(&self) -> Result<()> {
        self.call_raw("keypoolrefill", &[(); 0] as &[()]).await
    }

    /// `keypoolrefill` with all optional arguments via [`KeypoolRefillOptions`].
    ///
    /// Refills each descriptor keypool in the wallet up to the specified number of new keys.
    /// By default, descriptor wallets have 4 active ranged descriptors ("legacy", "p2sh-segwit", "bech32", "bech32m"), each with 1000 entries.
    ///
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn keypool_refill_with(&self, opts: KeypoolRefillOptions) -> Result<()> {
        self.call_raw("keypoolrefill", &[json!(opts.new_size)]).await
    }

    /// `listaddressgroupings` with required arguments only.
    ///
    /// Lists groups of addresses which have had their common ownership
    /// made public by common use as inputs or as the resulting change
    /// in past transactions
    pub async fn list_address_groupings(&self) -> Result<ListAddressGroupings> {
        self.call_raw("listaddressgroupings", &[(); 0] as &[()]).await
    }

    /// `listdescriptors` with required arguments only.
    ///
    /// List all descriptors present in a wallet.
    pub async fn list_descriptors(&self) -> Result<ListDescriptors> {
        self.call_raw("listdescriptors", &[(); 0] as &[()]).await
    }

    /// `listdescriptors` with all optional arguments via [`ListDescriptorsOptions`].
    ///
    /// List all descriptors present in a wallet.
    pub async fn list_descriptors_with(
        &self,
        opts: ListDescriptorsOptions,
    ) -> Result<ListDescriptors> {
        self.call_raw("listdescriptors", &[json!(opts.private)]).await
    }

    /// `listlabels` with required arguments only.
    ///
    /// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
    pub async fn list_labels(&self) -> Result<ListLabels> {
        self.call_raw("listlabels", &[(); 0] as &[()]).await
    }

    /// `listlabels` with all optional arguments via [`ListLabelsOptions`].
    ///
    /// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
    pub async fn list_labels_with(&self, opts: ListLabelsOptions) -> Result<ListLabels> {
        self.call_raw("listlabels", &[json!(opts.purpose)]).await
    }

    /// `listlockunspent` with required arguments only.
    ///
    /// Returns list of temporarily unspendable outputs.
    /// See the lockunspent call to lock and unlock transactions for spending.
    pub async fn list_lock_unspent(&self) -> Result<ListLockUnspent> {
        self.call_raw("listlockunspent", &[(); 0] as &[()]).await
    }

    /// `listreceivedbyaddress` with required arguments only.
    ///
    /// List balances by receiving address.
    pub async fn list_received_by_address(&self) -> Result<ListReceivedByAddress> {
        self.call_raw("listreceivedbyaddress", &[(); 0] as &[()]).await
    }

    /// `listreceivedbyaddress` with all optional arguments via [`ListReceivedByAddressOptions`].
    ///
    /// List balances by receiving address.
    pub async fn list_received_by_address_with(
        &self,
        opts: ListReceivedByAddressOptions,
    ) -> Result<ListReceivedByAddress> {
        self.call_raw(
            "listreceivedbyaddress",
            &[
                json!(opts.min_conf),
                json!(opts.include_empty),
                json!(opts.include_watch_only),
                json!(opts.address_filter),
                json!(opts.include_immature_coinbase),
            ],
        )
        .await
    }

    /// `listreceivedbylabel` with required arguments only.
    ///
    /// List received transactions by label.
    pub async fn list_received_by_label(&self) -> Result<ListReceivedByLabel> {
        self.call_raw("listreceivedbylabel", &[(); 0] as &[()]).await
    }

    /// `listreceivedbylabel` with all optional arguments via [`ListReceivedByLabelOptions`].
    ///
    /// List received transactions by label.
    pub async fn list_received_by_label_with(
        &self,
        opts: ListReceivedByLabelOptions,
    ) -> Result<ListReceivedByLabel> {
        self.call_raw(
            "listreceivedbylabel",
            &[
                json!(opts.min_conf),
                json!(opts.include_empty),
                json!(opts.include_watch_only),
                json!(opts.include_immature_coinbase),
            ],
        )
        .await
    }

    /// `listsinceblock` with required arguments only.
    ///
    /// Get all transactions in blocks since block \[blockhash\], or all transactions if omitted.
    /// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
    /// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
    pub async fn list_since_block(&self) -> Result<ListSinceBlock> {
        self.call_raw("listsinceblock", &[(); 0] as &[()]).await
    }

    /// `listsinceblock` with all optional arguments via [`ListSinceBlockOptions`].
    ///
    /// Get all transactions in blocks since block \[blockhash\], or all transactions if omitted.
    /// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
    /// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
    pub async fn list_since_block_with(
        &self,
        opts: ListSinceBlockOptions,
    ) -> Result<ListSinceBlock> {
        self.call_raw(
            "listsinceblock",
            &[
                json!(opts.block_hash),
                json!(opts.target_confirmations),
                json!(opts.include_watch_only),
                json!(opts.include_removed),
                json!(opts.include_change),
                json!(opts.label),
            ],
        )
        .await
    }

    /// `listtransactions` with required arguments only.
    ///
    /// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
    ///
    /// Returns up to 'count' most recent transactions skipping the first 'from' transactions.
    pub async fn list_transactions(&self) -> Result<ListTransactions> {
        self.call_raw("listtransactions", &[(); 0] as &[()]).await
    }

    /// `listtransactions` with all optional arguments via [`ListTransactionsOptions`].
    ///
    /// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
    ///
    /// Returns up to 'count' most recent transactions skipping the first 'from' transactions.
    pub async fn list_transactions_with(
        &self,
        opts: ListTransactionsOptions,
    ) -> Result<ListTransactions> {
        self.call_raw(
            "listtransactions",
            &[
                json!(opts.label),
                json!(opts.count),
                json!(opts.skip),
                json!(opts.include_watch_only),
            ],
        )
        .await
    }

    /// `listunspent` with required arguments only.
    ///
    /// Returns array of unspent transaction outputs
    /// with between minconf and maxconf (inclusive) confirmations.
    /// Optionally filter to only include txouts paid to specified addresses.
    pub async fn list_unspent(&self) -> Result<ListUnspent> {
        self.call_raw("listunspent", &[(); 0] as &[()]).await
    }

    /// `listunspent` with all optional arguments via [`ListUnspentOptions`].
    ///
    /// Returns array of unspent transaction outputs
    /// with between minconf and maxconf (inclusive) confirmations.
    /// Optionally filter to only include txouts paid to specified addresses.
    pub async fn list_unspent_with(&self, opts: ListUnspentOptions) -> Result<ListUnspent> {
        self.call_raw(
            "listunspent",
            &[
                json!(opts.min_conf),
                json!(opts.max_conf),
                json!(opts.addresses),
                json!(opts.include_unsafe),
                json!(opts.query_options),
            ],
        )
        .await
    }

    /// `listwalletdir` with required arguments only.
    ///
    /// Returns a list of wallets in the wallet directory.
    pub async fn list_wallet_dir(&self) -> Result<ListWalletDir> {
        self.call_raw("listwalletdir", &[(); 0] as &[()]).await
    }

    /// `listwallets` with required arguments only.
    ///
    /// Returns a list of currently loaded wallets.
    /// For full information on the wallet, use "getwalletinfo"
    pub async fn list_wallets(&self) -> Result<ListWallets> {
        self.call_raw("listwallets", &[(); 0] as &[()]).await
    }

    /// `loadwallet` with required arguments only.
    ///
    /// Loads a wallet from a wallet file or directory.
    /// Note that all wallet command-line options used when starting bitcoind will be
    /// applied to the new wallet.
    pub async fn load_wallet(&self, file_name: String) -> Result<LoadWallet> {
        self.call_raw("loadwallet", &[json!(file_name)]).await
    }

    /// `loadwallet` with all optional arguments via [`LoadWalletOptions`].
    ///
    /// Loads a wallet from a wallet file or directory.
    /// Note that all wallet command-line options used when starting bitcoind will be
    /// applied to the new wallet.
    pub async fn load_wallet_with(
        &self,
        file_name: String,
        opts: LoadWalletOptions,
    ) -> Result<LoadWallet> {
        self.call_raw("loadwallet", &[json!(file_name), json!(opts.load_on_startup)]).await
    }

    /// `lockunspent` with required arguments only.
    ///
    /// Updates list of temporarily unspendable outputs.
    /// Temporarily lock (unlock=false) or unlock (unlock=true) specified transaction outputs.
    /// If no transaction outputs are specified when unlocking then all current locked transaction outputs are unlocked.
    /// A locked transaction output will not be chosen by automatic coin selection, when spending bitcoins.
    /// Manually selected coins are automatically unlocked.
    /// Locks are stored in memory only, unless persistent=true, in which case they will be written to the
    /// wallet database and loaded on node start. Unwritten (persistent=false) locks are always cleared
    /// (by virtue of process exit) when a node stops or fails. Unlocking will clear both persistent and not.
    /// Also see the listunspent call
    pub async fn lock_unspent(&self, unlock: bool) -> Result<LockUnspent> {
        self.call_raw("lockunspent", &[json!(unlock)]).await
    }

    /// `lockunspent` with all optional arguments via [`LockUnspentOptions`].
    ///
    /// Updates list of temporarily unspendable outputs.
    /// Temporarily lock (unlock=false) or unlock (unlock=true) specified transaction outputs.
    /// If no transaction outputs are specified when unlocking then all current locked transaction outputs are unlocked.
    /// A locked transaction output will not be chosen by automatic coin selection, when spending bitcoins.
    /// Manually selected coins are automatically unlocked.
    /// Locks are stored in memory only, unless persistent=true, in which case they will be written to the
    /// wallet database and loaded on node start. Unwritten (persistent=false) locks are always cleared
    /// (by virtue of process exit) when a node stops or fails. Unlocking will clear both persistent and not.
    /// Also see the listunspent call
    pub async fn lock_unspent_with(
        &self,
        unlock: bool,
        opts: LockUnspentOptions,
    ) -> Result<LockUnspent> {
        self.call_raw(
            "lockunspent",
            &[json!(unlock), json!(opts.transactions), json!(opts.persistent)],
        )
        .await
    }

    /// `migratewallet` with required arguments only.
    ///
    /// Migrate the wallet to a descriptor wallet.
    /// A new wallet backup will need to be made.
    ///
    /// The migration process will create a backup of the wallet before migrating. This backup
    /// file will be named \<wallet name\>-\<timestamp\>.legacy.bak and can be found in the directory
    /// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
    /// Encrypted wallets must have the passphrase provided as an argument to this call.
    ///
    /// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
    pub async fn migrate_wallet(&self) -> Result<MigrateWallet> {
        self.call_raw("migratewallet", &[(); 0] as &[()]).await
    }

    /// `migratewallet` with all optional arguments via [`MigrateWalletOptions`].
    ///
    /// Migrate the wallet to a descriptor wallet.
    /// A new wallet backup will need to be made.
    ///
    /// The migration process will create a backup of the wallet before migrating. This backup
    /// file will be named \<wallet name\>-\<timestamp\>.legacy.bak and can be found in the directory
    /// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
    /// Encrypted wallets must have the passphrase provided as an argument to this call.
    ///
    /// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
    pub async fn migrate_wallet_with(&self, opts: MigrateWalletOptions) -> Result<MigrateWallet> {
        self.call_raw("migratewallet", &[json!(opts.wallet_name), json!(opts.passphrase)]).await
    }

    /// `psbtbumpfee` with required arguments only.
    ///
    /// Bumps the fee of a transaction T, replacing it with a new transaction B.
    /// Returns a PSBT instead of creating and signing a new transaction.
    /// A transaction with the given txid must be in the wallet.
    /// The command will pay the additional fee by reducing change outputs or adding inputs when necessary.
    /// It may add a new change output if one does not already exist.
    /// All inputs in the original transaction will be included in the replacement transaction.
    /// The command will fail if the wallet or mempool contains a transaction that spends one of T's outputs.
    /// By default, the new fee will be calculated automatically using the estimatesmartfee RPC.
    /// The user can specify a confirmation target for estimatesmartfee.
    /// Alternatively, the user can specify a fee rate in sat/vB for the new transaction.
    /// At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
    /// returned by getnetworkinfo) to enter the node's mempool.
    /// * WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB. *
    pub async fn psbt_bump_fee(&self, txid: String) -> Result<PsbtBumpFee> {
        self.call_raw("psbtbumpfee", &[json!(txid)]).await
    }

    /// `psbtbumpfee` with all optional arguments via [`PsbtBumpFeeOptions`].
    ///
    /// Bumps the fee of a transaction T, replacing it with a new transaction B.
    /// Returns a PSBT instead of creating and signing a new transaction.
    /// A transaction with the given txid must be in the wallet.
    /// The command will pay the additional fee by reducing change outputs or adding inputs when necessary.
    /// It may add a new change output if one does not already exist.
    /// All inputs in the original transaction will be included in the replacement transaction.
    /// The command will fail if the wallet or mempool contains a transaction that spends one of T's outputs.
    /// By default, the new fee will be calculated automatically using the estimatesmartfee RPC.
    /// The user can specify a confirmation target for estimatesmartfee.
    /// Alternatively, the user can specify a fee rate in sat/vB for the new transaction.
    /// At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
    /// returned by getnetworkinfo) to enter the node's mempool.
    /// * WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB. *
    pub async fn psbt_bump_fee_with(
        &self,
        txid: String,
        opts: PsbtBumpFeeOptions,
    ) -> Result<PsbtBumpFee> {
        self.call_raw("psbtbumpfee", &[json!(txid), json!(opts)]).await
    }

    /// `removeprunedfunds` with required arguments only.
    ///
    /// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
    pub async fn remove_pruned_funds(&self, txid: String) -> Result<()> {
        self.call_raw("removeprunedfunds", &[json!(txid)]).await
    }

    /// `rescanblockchain` with required arguments only.
    ///
    /// Rescan the local blockchain for wallet related transactions.
    /// Note: Use "getwalletinfo" to query the scanning progress.
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    pub async fn rescan_blockchain(&self) -> Result<RescanBlockchain> {
        self.call_raw("rescanblockchain", &[(); 0] as &[()]).await
    }

    /// `rescanblockchain` with all optional arguments via [`RescanBlockchainOptions`].
    ///
    /// Rescan the local blockchain for wallet related transactions.
    /// Note: Use "getwalletinfo" to query the scanning progress.
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    pub async fn rescan_blockchain_with(
        &self,
        opts: RescanBlockchainOptions,
    ) -> Result<RescanBlockchain> {
        self.call_raw("rescanblockchain", &[json!(opts.start_height), json!(opts.stop_height)])
            .await
    }

    /// `restorewallet` with required arguments only.
    ///
    /// Restores and loads a wallet from backup.
    ///
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    pub async fn restore_wallet(
        &self,
        wallet_name: String,
        backup_file: String,
    ) -> Result<RestoreWallet> {
        self.call_raw("restorewallet", &[json!(wallet_name), json!(backup_file)]).await
    }

    /// `restorewallet` with all optional arguments via [`RestoreWalletOptions`].
    ///
    /// Restores and loads a wallet from backup.
    ///
    /// The rescan is significantly faster if block filters are available
    /// (using startup option "-blockfilterindex=1").
    pub async fn restore_wallet_with(
        &self,
        wallet_name: String,
        backup_file: String,
        opts: RestoreWalletOptions,
    ) -> Result<RestoreWallet> {
        self.call_raw(
            "restorewallet",
            &[json!(wallet_name), json!(backup_file), json!(opts.load_on_startup)],
        )
        .await
    }

    /// `send` with required arguments only.
    ///
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Send a transaction.
    pub async fn send(&self, outputs: Vec<SendResultOutputs>) -> Result<SendResult> {
        self.call_raw("send", &[json!(outputs)]).await
    }

    /// `send` with all optional arguments via [`SendResultOptions`].
    ///
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Send a transaction.
    pub async fn send_with(
        &self,
        outputs: Vec<SendResultOutputs>,
        opts: SendResultOptions,
    ) -> Result<SendResult> {
        self.call_raw(
            "send",
            &[
                json!(outputs),
                json!(opts.conf_target),
                json!(opts.estimate_mode),
                json!(opts.fee_rate),
                json!(opts.options),
                json!(opts.version),
            ],
        )
        .await
    }

    /// `sendall` with required arguments only.
    ///
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
    /// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
    /// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
    pub async fn send_all(&self, recipients: Vec<SendAllRecipients>) -> Result<SendAll> {
        self.call_raw("sendall", &[json!(recipients)]).await
    }

    /// `sendall` with all optional arguments via [`SendAllOptions`].
    ///
    /// EXPERIMENTAL warning: this call may be changed in future releases.
    ///
    /// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
    /// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
    /// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
    pub async fn send_all_with(
        &self,
        recipients: Vec<SendAllRecipients>,
        opts: SendAllOptions,
    ) -> Result<SendAll> {
        self.call_raw(
            "sendall",
            &[
                json!(recipients),
                json!(opts.conf_target),
                json!(opts.estimate_mode),
                json!(opts.fee_rate),
                json!(opts.options),
            ],
        )
        .await
    }

    /// `sendmany` with the result selected for verbosity `false`.
    ///
    /// Send multiple times. Amounts are double-precision floating point numbers.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn send_many_verbose_0(
        &self,
        amounts: std::collections::BTreeMap<String, SendManyAmounts>,
    ) -> Result<SendManyVerbose0> {
        self.call_raw(
            "sendmany",
            &[
                json!(null),
                json!(amounts),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(false),
            ],
        )
        .await
    }

    /// `sendmany` with the result selected for verbosity `false`. With all optional arguments via [`SendManyOptions`].
    ///
    /// Send multiple times. Amounts are double-precision floating point numbers.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn send_many_verbose_0_with(
        &self,
        amounts: std::collections::BTreeMap<String, SendManyAmounts>,
        opts: SendManyOptions,
    ) -> Result<SendManyVerbose0> {
        self.call_raw(
            "sendmany",
            &[
                json!(opts.dummy),
                json!(amounts),
                json!(opts.min_conf),
                json!(opts.comment),
                json!(opts.subtract_fee_from),
                json!(opts.replaceable),
                json!(opts.conf_target),
                json!(opts.estimate_mode),
                json!(opts.fee_rate),
                json!(false),
            ],
        )
        .await
    }

    /// `sendmany` with the result selected for verbosity `true`.
    ///
    /// Send multiple times. Amounts are double-precision floating point numbers.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn send_many_verbose_1(
        &self,
        amounts: std::collections::BTreeMap<String, SendManyAmounts>,
    ) -> Result<SendManyVerbose1> {
        self.call_raw(
            "sendmany",
            &[
                json!(null),
                json!(amounts),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(true),
            ],
        )
        .await
    }

    /// `sendmany` with the result selected for verbosity `true`. With all optional arguments via [`SendManyOptions`].
    ///
    /// Send multiple times. Amounts are double-precision floating point numbers.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn send_many_verbose_1_with(
        &self,
        amounts: std::collections::BTreeMap<String, SendManyAmounts>,
        opts: SendManyOptions,
    ) -> Result<SendManyVerbose1> {
        self.call_raw(
            "sendmany",
            &[
                json!(opts.dummy),
                json!(amounts),
                json!(opts.min_conf),
                json!(opts.comment),
                json!(opts.subtract_fee_from),
                json!(opts.replaceable),
                json!(opts.conf_target),
                json!(opts.estimate_mode),
                json!(opts.fee_rate),
                json!(true),
            ],
        )
        .await
    }

    /// `sendtoaddress` with the result selected for verbosity `false`.
    ///
    /// Send an amount to a given address.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn send_to_address_verbose_0(
        &self,
        address: String,
        amount: SendToAddressAmount,
    ) -> Result<SendToAddressVerbose0> {
        self.call_raw(
            "sendtoaddress",
            &[
                json!(address),
                json!(amount),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(false),
            ],
        )
        .await
    }

    /// `sendtoaddress` with the result selected for verbosity `false`. With all optional arguments via [`SendToAddressOptions`].
    ///
    /// Send an amount to a given address.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn send_to_address_verbose_0_with(
        &self,
        address: String,
        amount: SendToAddressAmount,
        opts: SendToAddressOptions,
    ) -> Result<SendToAddressVerbose0> {
        self.call_raw(
            "sendtoaddress",
            &[
                json!(address),
                json!(amount),
                json!(opts.comment),
                json!(opts.comment_to),
                json!(opts.subtract_fee_from_amount),
                json!(opts.replaceable),
                json!(opts.conf_target),
                json!(opts.estimate_mode),
                json!(opts.avoid_reuse),
                json!(opts.fee_rate),
                json!(false),
            ],
        )
        .await
    }

    /// `sendtoaddress` with the result selected for verbosity `true`.
    ///
    /// Send an amount to a given address.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn send_to_address_verbose_1(
        &self,
        address: String,
        amount: SendToAddressAmount,
    ) -> Result<SendToAddressVerbose1> {
        self.call_raw(
            "sendtoaddress",
            &[
                json!(address),
                json!(amount),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(null),
                json!(true),
            ],
        )
        .await
    }

    /// `sendtoaddress` with the result selected for verbosity `true`. With all optional arguments via [`SendToAddressOptions`].
    ///
    /// Send an amount to a given address.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn send_to_address_verbose_1_with(
        &self,
        address: String,
        amount: SendToAddressAmount,
        opts: SendToAddressOptions,
    ) -> Result<SendToAddressVerbose1> {
        self.call_raw(
            "sendtoaddress",
            &[
                json!(address),
                json!(amount),
                json!(opts.comment),
                json!(opts.comment_to),
                json!(opts.subtract_fee_from_amount),
                json!(opts.replaceable),
                json!(opts.conf_target),
                json!(opts.estimate_mode),
                json!(opts.avoid_reuse),
                json!(opts.fee_rate),
                json!(true),
            ],
        )
        .await
    }

    /// `setlabel` with required arguments only.
    ///
    /// Sets the label associated with the given address.
    pub async fn set_label(&self, address: String, label: String) -> Result<()> {
        self.call_raw("setlabel", &[json!(address), json!(label)]).await
    }

    /// `settxfee` with required arguments only.
    ///
    /// (DEPRECATED) Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
    /// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.
    pub async fn set_tx_fee(&self, amount: SetTxFeeAmount) -> Result<SetTxFee> {
        self.call_raw("settxfee", &[json!(amount)]).await
    }

    /// `setwalletflag` with required arguments only.
    ///
    /// Change the state of the given wallet flag for a wallet.
    pub async fn set_wallet_flag(&self, flag: String) -> Result<SetWalletFlag> {
        self.call_raw("setwalletflag", &[json!(flag)]).await
    }

    /// `setwalletflag` with all optional arguments via [`SetWalletFlagOptions`].
    ///
    /// Change the state of the given wallet flag for a wallet.
    pub async fn set_wallet_flag_with(
        &self,
        flag: String,
        opts: SetWalletFlagOptions,
    ) -> Result<SetWalletFlag> {
        self.call_raw("setwalletflag", &[json!(flag), json!(opts.value)]).await
    }

    /// `signmessage` with required arguments only.
    ///
    /// Sign a message with the private key of an address
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn sign_message(&self, address: String, message: String) -> Result<SignMessage> {
        self.call_raw("signmessage", &[json!(address), json!(message)]).await
    }

    /// `signrawtransactionwithwallet` with required arguments only.
    ///
    /// Sign inputs for raw transaction (serialized, hex-encoded).
    /// The second optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn sign_raw_transaction_with_wallet(
        &self,
        hexstring: String,
    ) -> Result<SignRawTransactionWithWallet> {
        self.call_raw("signrawtransactionwithwallet", &[json!(hexstring)]).await
    }

    /// `signrawtransactionwithwallet` with all optional arguments via [`SignRawTransactionWithWalletOptions`].
    ///
    /// Sign inputs for raw transaction (serialized, hex-encoded).
    /// The second optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn sign_raw_transaction_with_wallet_with(
        &self,
        hexstring: String,
        opts: SignRawTransactionWithWalletOptions,
    ) -> Result<SignRawTransactionWithWallet> {
        self.call_raw(
            "signrawtransactionwithwallet",
            &[json!(hexstring), json!(opts.prev_txs), json!(opts.sig_hash_type)],
        )
        .await
    }

    /// `simulaterawtransaction` with required arguments only.
    ///
    /// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
    pub async fn simulate_raw_transaction(&self) -> Result<SimulateRawTransaction> {
        self.call_raw("simulaterawtransaction", &[(); 0] as &[()]).await
    }

    /// `simulaterawtransaction` with all optional arguments via [`SimulateRawTransactionOptions`].
    ///
    /// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
    pub async fn simulate_raw_transaction_with(
        &self,
        opts: SimulateRawTransactionOptions,
    ) -> Result<SimulateRawTransaction> {
        self.call_raw("simulaterawtransaction", &[json!(opts.raw_txs), json!(opts.options)]).await
    }

    /// `unloadwallet` with required arguments only.
    ///
    /// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
    /// If both are specified, they must be identical.
    pub async fn unload_wallet(&self) -> Result<UnloadWallet> {
        self.call_raw("unloadwallet", &[(); 0] as &[()]).await
    }

    /// `unloadwallet` with all optional arguments via [`UnloadWalletOptions`].
    ///
    /// Unloads the wallet referenced by the request endpoint or the wallet_name argument.
    /// If both are specified, they must be identical.
    pub async fn unload_wallet_with(&self, opts: UnloadWalletOptions) -> Result<UnloadWallet> {
        self.call_raw("unloadwallet", &[json!(opts.wallet_name), json!(opts.load_on_startup)]).await
    }

    /// `walletcreatefundedpsbt` with required arguments only.
    ///
    /// Creates and funds a transaction in the Partially Signed Transaction format.
    /// Implements the Creator and Updater roles.
    /// All existing inputs must either have their previous output transaction be in the wallet
    /// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
    pub async fn wallet_create_funded_psbt(
        &self,
        outputs: Vec<WalletCreateFundedPsbtOutputs>,
    ) -> Result<WalletCreateFundedPsbt> {
        self.call_raw("walletcreatefundedpsbt", &[json!(null), json!(outputs)]).await
    }

    /// `walletcreatefundedpsbt` with all optional arguments via [`WalletCreateFundedPsbtOptions`].
    ///
    /// Creates and funds a transaction in the Partially Signed Transaction format.
    /// Implements the Creator and Updater roles.
    /// All existing inputs must either have their previous output transaction be in the wallet
    /// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
    pub async fn wallet_create_funded_psbt_with(
        &self,
        outputs: Vec<WalletCreateFundedPsbtOutputs>,
        opts: WalletCreateFundedPsbtOptions,
    ) -> Result<WalletCreateFundedPsbt> {
        self.call_raw(
            "walletcreatefundedpsbt",
            &[
                json!(opts.inputs),
                json!(outputs),
                json!(opts.locktime),
                json!(opts.options),
                json!(opts.bip32derivs),
                json!(opts.version),
            ],
        )
        .await
    }

    /// `walletdisplayaddress` with required arguments only.
    ///
    /// Display address on an external signer for verification.
    pub async fn wallet_display_address(&self, address: String) -> Result<WalletDisplayAddress> {
        self.call_raw("walletdisplayaddress", &[json!(address)]).await
    }

    /// `walletlock` with required arguments only.
    ///
    /// Removes the wallet encryption key from memory, locking the wallet.
    /// After calling this method, you will need to call walletpassphrase again
    /// before being able to call any methods which require the wallet to be unlocked.
    pub async fn wallet_lock(&self) -> Result<()> {
        self.call_raw("walletlock", &[(); 0] as &[()]).await
    }

    /// `walletpassphrase` with required arguments only.
    ///
    /// Stores the wallet decryption key in memory for 'timeout' seconds.
    /// This is needed prior to performing transactions related to private keys such as sending bitcoins
    ///
    /// Note:
    /// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
    /// time that overrides the old one.
    pub async fn wallet_passphrase(&self, passphrase: String, time_out: i64) -> Result<()> {
        self.call_raw("walletpassphrase", &[json!(passphrase), json!(time_out)]).await
    }

    /// `walletpassphrasechange` with required arguments only.
    ///
    /// Changes the wallet passphrase from 'oldpassphrase' to 'newpassphrase'.
    pub async fn wallet_passphrase_change(
        &self,
        old_passphrase: String,
        new_passphrase: String,
    ) -> Result<()> {
        self.call_raw("walletpassphrasechange", &[json!(old_passphrase), json!(new_passphrase)])
            .await
    }

    /// `walletprocesspsbt` with required arguments only.
    ///
    /// Update a PSBT with input information from our wallet and then sign inputs
    /// that we can sign for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn wallet_process_psbt(&self, psbt: String) -> Result<WalletProcessPsbt> {
        self.call_raw("walletprocesspsbt", &[json!(psbt)]).await
    }

    /// `walletprocesspsbt` with all optional arguments via [`WalletProcessPsbtOptions`].
    ///
    /// Update a PSBT with input information from our wallet and then sign inputs
    /// that we can sign for.
    /// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
    pub async fn wallet_process_psbt_with(
        &self,
        psbt: String,
        opts: WalletProcessPsbtOptions,
    ) -> Result<WalletProcessPsbt> {
        self.call_raw(
            "walletprocesspsbt",
            &[
                json!(psbt),
                json!(opts.sign),
                json!(opts.sig_hash_type),
                json!(opts.bip32derivs),
                json!(opts.finalize),
            ],
        )
        .await
    }
}
