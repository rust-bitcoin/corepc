// SPDX-License-Identifier: CC0-1.0

//! Auto-generated method wrappers for Bitcoin Core `31` - rawtransactions.
//!
//! Produced by `codegen`. Do not edit by hand, re-run
//! `just codegen` to regenerate. Defines the `*Options` request structs these methods
//! consume; the response types live in the `corepc-types` crate
//! (`types::v31::generated`).

#![allow(unused_imports, clippy::needless_pass_by_value, clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use serde_json::json;

use types::v31::generated::{
    AbortPrivateBroadcast,
    AnalyzePsbt,
    CombinePsbt,
    CombineRawTransaction,
    ConvertToPsbt,
    CreatePsbt,
    CreateRawTransaction,
    DecodePsbt,
    DecodeRawTransaction,
    DecodeScript,
    DescriptorProcessPsbt,
    FinalizePsbt,
    FundRawTransaction,
    GetPrivateBroadcastInfo,
    GetRawTransactionVerbose0,
    GetRawTransactionVerbose1,
    GetRawTransactionVerbose2,
    JoinPsbts,
    SendRawTransaction,
    SignRawTransactionWithKey,
    SubmitPackage,
    TestMempoolAccept,
    UtxoUpdatePsbt,
};

use crate::client_async::error::Result;
use crate::client_async::Client;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreatePsbtInputs {
    /// The sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i64>,
    /// The transaction id
    pub txid: String,
    /// The output number
    pub vout: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatePsbtOutputs {
    Object(std::collections::BTreeMap<String, CreatePsbtOutputsVariant0>),
    Object2(CreatePsbtOutputsVariant1),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreatePsbtOutputsVariant0 {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreatePsbtOutputsVariant1 {
    /// A key-value pair. The key must be "data", the value is hex-encoded data that becomes a part of an OP_RETURN output
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateRawTransactionInputs {
    /// The sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i64>,
    /// The transaction id
    pub txid: String,
    /// The output number
    pub vout: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreateRawTransactionOutputs {
    Object(std::collections::BTreeMap<String, CreateRawTransactionOutputsVariant0>),
    Object2(CreateRawTransactionOutputsVariant1),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CreateRawTransactionOutputsVariant0 {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateRawTransactionOutputsVariant1 {
    /// A key-value pair. The key must be "data", the value is hex-encoded data that becomes a part of an OP_RETURN output
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DescriptorProcessPsbtDescriptors {
    Text(String),
    Object(DescriptorProcessPsbtDescriptorsVariant1),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DescriptorProcessPsbtDescriptorsVariant1 {
    /// An output descriptor
    pub desc: String,
    /// Up to what index HD chains should be explored (either end or \[begin,end\])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<DescriptorProcessPsbtDescriptorsVariant1Range>,
}

/// Up to what index HD chains should be explored (either end or \[begin,end\])
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DescriptorProcessPsbtDescriptorsVariant1Range {
    Number(f64),
    List(Vec<serde_json::Value>),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FundRawTransactionOptionsArg {
    /// For a transaction with existing inputs, automatically include more if they are not enough.
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
    pub fee_rate: Option<FundRawTransactionOptionsArgFeeRate>,
    /// Specify a fee rate in sat/vB.
    #[serde(rename = "fee_rate", skip_serializing_if = "Option::is_none")]
    pub fee_rate2: Option<FundRawTransactionOptionsArgFeeRate>,
    /// (DEPRECATED) No longer used
    #[serde(rename = "includeWatching", skip_serializing_if = "Option::is_none")]
    pub include_watching: Option<bool>,
    /// Include inputs that are not safe to spend (unconfirmed transactions from outside keys and unconfirmed replacement transactions).
    /// Warning: the resulting transaction may become invalid if one of the unsafe inputs disappears.
    /// If that happens, you will need to fund the transaction with different inputs and republish it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_unsafe: Option<bool>,
    /// Inputs and their corresponding weights
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_weights: Option<Vec<FundRawTransactionOptionsArgInputWeightsItem>>,
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
    pub solving_data: Option<FundRawTransactionOptionsArgSolvingData>,
    /// The integers.
    /// The fee will be equally deducted from the amount of each specified output.
    /// Those recipients will receive less bitcoins than you enter in their corresponding amount field.
    /// If no outputs are specified here, the sender pays the fee.
    #[serde(rename = "subtractFeeFromOutputs", skip_serializing_if = "Option::is_none")]
    pub subtract_fee_from_outputs: Option<Vec<i64>>,
}

/// Specify a fee rate in BTC/kvB.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FundRawTransactionOptionsArgFeeRate {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FundRawTransactionOptionsArgInputWeightsItem {
    /// The transaction id
    pub txid: String,
    /// The output index
    pub vout: i64,
    /// The maximum weight for this input, including the weight of the outpoint and sequence number. Note that serialized signature sizes are not guaranteed to be consistent, so the maximum DER signatures size of 73 bytes should be used when considering ECDSA signatures.Remember to convert serialized sizes to weight units when necessary.
    pub weight: i64,
}

/// Keys and scripts needed for producing a final transaction with a dummy signature.
/// Used for fee estimation during coin selection.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FundRawTransactionOptionsArgSolvingData {
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
pub enum SendRawTransactionMaxBurnAmount {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SendRawTransactionMaxFeeRate {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SignRawTransactionWithKeyPrevTxs {
    /// (required for Segwit inputs) the amount spent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<SignRawTransactionWithKeyPrevTxsAmount>,
    /// (required for P2SH) redeem script
    #[serde(rename = "redeemScript", skip_serializing_if = "Option::is_none")]
    pub redeem_script: Option<String>,
    /// output script
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
pub enum SignRawTransactionWithKeyPrevTxsAmount {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SubmitPackageMaxBurnAmount {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SubmitPackageMaxFeeRate {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TestMempoolAcceptMaxFeeRate {
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UtxoUpdatePsbtDescriptors {
    Text(String),
    Object(UtxoUpdatePsbtDescriptorsVariant1),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UtxoUpdatePsbtDescriptorsVariant1 {
    /// An output descriptor
    pub desc: String,
    /// Up to what index HD chains should be explored (either end or \[begin,end\])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<UtxoUpdatePsbtDescriptorsVariant1Range>,
}

/// Up to what index HD chains should be explored (either end or \[begin,end\])
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UtxoUpdatePsbtDescriptorsVariant1Range {
    Number(f64),
    List(Vec<serde_json::Value>),
}

/// Optional parameters for the `converttopsbt` JSON-RPC method (consumed by `Client::convert_to_psbt_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertToPsbtOptions {
    /// If true, any signatures in the input will be discarded and conversion
    ///                               will continue. If false, RPC will fail if any signatures are present.
    pub permitsig_data: Option<bool>,
    /// Whether the transaction hex is a serialized witness transaction.
    /// If iswitness is not present, heuristic tests will be used in decoding.
    /// If true, only witness deserialization will be tried.
    /// If false, only non-witness deserialization will be tried.
    /// This boolean should reflect whether the transaction has inputs
    /// (e.g. fully valid, or on-chain transactions), if known by the caller.
    pub is_witness: Option<bool>,
}

/// Optional parameters for the `createpsbt` JSON-RPC method (consumed by `Client::create_psbt_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePsbtOptions {
    /// Raw locktime. Non-0 value also locktime-activates inputs
    pub locktime: Option<i64>,
    /// Marks this transaction as BIP125-replaceable.
    /// Allows this transaction to be replaced by a transaction with higher fees. If provided, it is an error if explicit sequence numbers are incompatible.
    pub replaceable: Option<bool>,
    /// Transaction version
    pub version: Option<i64>,
}

/// Optional parameters for the `createrawtransaction` JSON-RPC method (consumed by `Client::create_raw_transaction_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRawTransactionOptions {
    /// Raw locktime. Non-0 value also locktime-activates inputs
    pub locktime: Option<i64>,
    /// Marks this transaction as BIP125-replaceable.
    /// Allows this transaction to be replaced by a transaction with higher fees. If provided, it is an error if explicit sequence numbers are incompatible.
    pub replaceable: Option<bool>,
    /// Transaction version
    pub version: Option<i64>,
}

/// Optional parameters for the `decoderawtransaction` JSON-RPC method (consumed by `Client::decode_raw_transaction_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodeRawTransactionOptions {
    /// Whether the transaction hex is a serialized witness transaction.
    /// If iswitness is not present, heuristic tests will be used in decoding.
    /// If true, only witness deserialization will be tried.
    /// If false, only non-witness deserialization will be tried.
    /// This boolean should reflect whether the transaction has inputs
    /// (e.g. fully valid, or on-chain transactions), if known by the caller.
    pub is_witness: Option<bool>,
}

/// Optional parameters for the `descriptorprocesspsbt` JSON-RPC method (consumed by `Client::descriptor_process_psbt_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptorProcessPsbtOptions {
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

/// Optional parameters for the `finalizepsbt` JSON-RPC method (consumed by `Client::finalize_psbt_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalizePsbtOptions {
    /// If true and the transaction is complete,
    ///                              extract and return the complete transaction in normal network serialization instead of the PSBT.
    pub extract: Option<bool>,
}

/// Optional parameters for the `fundrawtransaction` JSON-RPC method (consumed by `Client::fund_raw_transaction_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FundRawTransactionOptions {
    pub options: Option<FundRawTransactionOptionsArg>,
    /// Whether the transaction hex is a serialized witness transaction.
    /// If iswitness is not present, heuristic tests will be used in decoding.
    /// If true, only witness deserialization will be tried.
    /// If false, only non-witness deserialization will be tried.
    /// This boolean should reflect whether the transaction has inputs
    /// (e.g. fully valid, or on-chain transactions), if known by the caller.
    pub is_witness: Option<bool>,
}

/// Optional parameters for the `getrawtransaction` JSON-RPC method (consumed by `Client::get_raw_transaction_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRawTransactionOptions {
    /// The block in which to look for the transaction
    pub block_hash: Option<String>,
}

/// Optional parameters for the `sendrawtransaction` JSON-RPC method (consumed by `Client::send_raw_transaction_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendRawTransactionOptions {
    /// Reject transactions whose fee rate is higher than the specified value, expressed in BTC/kvB.
    /// Fee rates larger than 1BTC/kvB are rejected.
    /// Set to 0 to accept any fee rate.
    pub max_fee_rate: Option<SendRawTransactionMaxFeeRate>,
    /// Reject transactions with provably unspendable outputs (e.g. 'datacarrier' outputs that use the OP_RETURN opcode) greater than the specified value, expressed in BTC.
    /// If burning funds through unspendable outputs is desired, increase this value.
    /// This check is based on heuristics and does not guarantee spendability of outputs.
    pub max_burn_amount: Option<SendRawTransactionMaxBurnAmount>,
}

/// Optional parameters for the `signrawtransactionwithkey` JSON-RPC method (consumed by `Client::sign_raw_transaction_with_key_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignRawTransactionWithKeyOptions {
    /// The previous dependent transaction outputs
    pub prev_txs: Option<Vec<SignRawTransactionWithKeyPrevTxs>>,
    /// The signature hash type. Must be one of:
    ///        "DEFAULT"
    ///        "ALL"
    ///        "NONE"
    ///        "SINGLE"
    ///        "ALL|ANYONECANPAY"
    ///        "NONE|ANYONECANPAY"
    ///        "SINGLE|ANYONECANPAY"
    pub sig_hash_type: Option<String>,
}

/// Optional parameters for the `submitpackage` JSON-RPC method (consumed by `Client::submit_package_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitPackageOptions {
    /// Reject transactions whose fee rate is higher than the specified value, expressed in BTC/kvB.
    /// Fee rates larger than 1BTC/kvB are rejected.
    /// Set to 0 to accept any fee rate.
    pub max_fee_rate: Option<SubmitPackageMaxFeeRate>,
    /// Reject transactions with provably unspendable outputs (e.g. 'datacarrier' outputs that use the OP_RETURN opcode) greater than the specified value, expressed in BTC.
    /// If burning funds through unspendable outputs is desired, increase this value.
    /// This check is based on heuristics and does not guarantee spendability of outputs.
    pub max_burn_amount: Option<SubmitPackageMaxBurnAmount>,
}

/// Optional parameters for the `testmempoolaccept` JSON-RPC method (consumed by `Client::test_mempool_accept_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestMempoolAcceptOptions {
    /// Reject transactions whose fee rate is higher than the specified value, expressed in BTC/kvB.
    /// Fee rates larger than 1BTC/kvB are rejected.
    /// Set to 0 to accept any fee rate.
    pub max_fee_rate: Option<TestMempoolAcceptMaxFeeRate>,
}

/// Optional parameters for the `utxoupdatepsbt` JSON-RPC method (consumed by `Client::utxo_update_psbt_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoUpdatePsbtOptions {
    /// An array of either strings or objects
    pub descriptors: Option<Vec<UtxoUpdatePsbtDescriptors>>,
}

impl Client {
    /// `abortprivatebroadcast` with required arguments only.
    ///
    /// Abort private broadcast attempts for a transaction currently being privately broadcast.
    /// The transaction will be removed from the private broadcast queue.
    pub async fn abort_private_broadcast(&self, id: String) -> Result<AbortPrivateBroadcast> {
        self.call_raw("abortprivatebroadcast", &[json!(id)]).await
    }

    /// `analyzepsbt` with required arguments only.
    ///
    /// Analyzes and provides information about the current status of a PSBT and its inputs
    pub async fn analyze_psbt(&self, psbt: String) -> Result<AnalyzePsbt> {
        self.call_raw("analyzepsbt", &[json!(psbt)]).await
    }

    /// `combinepsbt` with required arguments only.
    ///
    /// Combine multiple partially signed Bitcoin transactions into one transaction.
    /// Implements the Combiner role.
    pub async fn combine_psbt(&self, txs: Vec<String>) -> Result<CombinePsbt> {
        self.call_raw("combinepsbt", &[json!(txs)]).await
    }

    /// `combinerawtransaction` with required arguments only.
    ///
    /// Combine multiple partially signed transactions into one transaction.
    /// The combined transaction may be another partially signed transaction or a 
    /// fully signed transaction.
    pub async fn combine_raw_transaction(&self, txs: Vec<String>) -> Result<CombineRawTransaction> {
        self.call_raw("combinerawtransaction", &[json!(txs)]).await
    }

    /// `converttopsbt` with required arguments only.
    ///
    /// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
    /// createpsbt and walletcreatefundedpsbt should be used for new applications.
    pub async fn convert_to_psbt(&self, hexstring: String) -> Result<ConvertToPsbt> {
        self.call_raw("converttopsbt", &[json!(hexstring)]).await
    }

    /// `converttopsbt` with all optional arguments via [`ConvertToPsbtOptions`].
    ///
    /// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
    /// createpsbt and walletcreatefundedpsbt should be used for new applications.
    pub async fn convert_to_psbt_with(&self, hexstring: String, opts: ConvertToPsbtOptions) -> Result<ConvertToPsbt> {
        self.call_raw("converttopsbt", &[json!(hexstring), json!(opts.permitsig_data), json!(opts.is_witness)]).await
    }

    /// `createpsbt` with required arguments only.
    ///
    /// Creates a transaction in the Partially Signed Transaction format.
    /// Implements the Creator role.
    /// Note that the transaction's inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    pub async fn create_psbt(&self, inputs: Vec<CreatePsbtInputs>, outputs: Vec<CreatePsbtOutputs>) -> Result<CreatePsbt> {
        self.call_raw("createpsbt", &[json!(inputs), json!(outputs)]).await
    }

    /// `createpsbt` with all optional arguments via [`CreatePsbtOptions`].
    ///
    /// Creates a transaction in the Partially Signed Transaction format.
    /// Implements the Creator role.
    /// Note that the transaction's inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    pub async fn create_psbt_with(&self, inputs: Vec<CreatePsbtInputs>, outputs: Vec<CreatePsbtOutputs>, opts: CreatePsbtOptions) -> Result<CreatePsbt> {
        self.call_raw("createpsbt", &[json!(inputs), json!(outputs), json!(opts.locktime), json!(opts.replaceable), json!(opts.version)]).await
    }

    /// `createrawtransaction` with required arguments only.
    ///
    /// Create a transaction spending the given inputs and creating new outputs.
    /// Outputs can be addresses or data.
    /// Returns hex-encoded raw transaction.
    /// Note that the transaction's inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    pub async fn create_raw_transaction(&self, inputs: Vec<CreateRawTransactionInputs>, outputs: Vec<CreateRawTransactionOutputs>) -> Result<CreateRawTransaction> {
        self.call_raw("createrawtransaction", &[json!(inputs), json!(outputs)]).await
    }

    /// `createrawtransaction` with all optional arguments via [`CreateRawTransactionOptions`].
    ///
    /// Create a transaction spending the given inputs and creating new outputs.
    /// Outputs can be addresses or data.
    /// Returns hex-encoded raw transaction.
    /// Note that the transaction's inputs are not signed, and
    /// it is not stored in the wallet or transmitted to the network.
    pub async fn create_raw_transaction_with(&self, inputs: Vec<CreateRawTransactionInputs>, outputs: Vec<CreateRawTransactionOutputs>, opts: CreateRawTransactionOptions) -> Result<CreateRawTransaction> {
        self.call_raw("createrawtransaction", &[json!(inputs), json!(outputs), json!(opts.locktime), json!(opts.replaceable), json!(opts.version)]).await
    }

    /// `decodepsbt` with required arguments only.
    ///
    /// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
    pub async fn decode_psbt(&self, psbt: String) -> Result<DecodePsbt> {
        self.call_raw("decodepsbt", &[json!(psbt)]).await
    }

    /// `decoderawtransaction` with required arguments only.
    ///
    /// Return a JSON object representing the serialized, hex-encoded transaction.
    pub async fn decode_raw_transaction(&self, hexstring: String) -> Result<DecodeRawTransaction> {
        self.call_raw("decoderawtransaction", &[json!(hexstring)]).await
    }

    /// `decoderawtransaction` with all optional arguments via [`DecodeRawTransactionOptions`].
    ///
    /// Return a JSON object representing the serialized, hex-encoded transaction.
    pub async fn decode_raw_transaction_with(&self, hexstring: String, opts: DecodeRawTransactionOptions) -> Result<DecodeRawTransaction> {
        self.call_raw("decoderawtransaction", &[json!(hexstring), json!(opts.is_witness)]).await
    }

    /// `decodescript` with required arguments only.
    ///
    /// Decode a hex-encoded script.
    pub async fn decode_script(&self, hexstring: String) -> Result<DecodeScript> {
        self.call_raw("decodescript", &[json!(hexstring)]).await
    }

    /// `descriptorprocesspsbt` with required arguments only.
    ///
    /// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool. 
    /// Then, sign the inputs we are able to with information from the output descriptors.
    pub async fn descriptor_process_psbt(&self, psbt: String, descriptors: Vec<DescriptorProcessPsbtDescriptors>) -> Result<DescriptorProcessPsbt> {
        self.call_raw("descriptorprocesspsbt", &[json!(psbt), json!(descriptors)]).await
    }

    /// `descriptorprocesspsbt` with all optional arguments via [`DescriptorProcessPsbtOptions`].
    ///
    /// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool. 
    /// Then, sign the inputs we are able to with information from the output descriptors.
    pub async fn descriptor_process_psbt_with(&self, psbt: String, descriptors: Vec<DescriptorProcessPsbtDescriptors>, opts: DescriptorProcessPsbtOptions) -> Result<DescriptorProcessPsbt> {
        self.call_raw("descriptorprocesspsbt", &[json!(psbt), json!(descriptors), json!(opts.sig_hash_type), json!(opts.bip32derivs), json!(opts.finalize)]).await
    }

    /// `finalizepsbt` with required arguments only.
    ///
    /// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
    /// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
    /// created which has the final_scriptSig and final_scriptwitness fields filled for inputs that are complete.
    /// Implements the Finalizer and Extractor roles.
    pub async fn finalize_psbt(&self, psbt: String) -> Result<FinalizePsbt> {
        self.call_raw("finalizepsbt", &[json!(psbt)]).await
    }

    /// `finalizepsbt` with all optional arguments via [`FinalizePsbtOptions`].
    ///
    /// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
    /// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
    /// created which has the final_scriptSig and final_scriptwitness fields filled for inputs that are complete.
    /// Implements the Finalizer and Extractor roles.
    pub async fn finalize_psbt_with(&self, psbt: String, opts: FinalizePsbtOptions) -> Result<FinalizePsbt> {
        self.call_raw("finalizepsbt", &[json!(psbt), json!(opts.extract)]).await
    }

    /// `fundrawtransaction` with required arguments only.
    ///
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
    pub async fn fund_raw_transaction(&self, hexstring: String) -> Result<FundRawTransaction> {
        self.call_raw("fundrawtransaction", &[json!(hexstring)]).await
    }

    /// `fundrawtransaction` with all optional arguments via [`FundRawTransactionOptions`].
    ///
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
    pub async fn fund_raw_transaction_with(&self, hexstring: String, opts: FundRawTransactionOptions) -> Result<FundRawTransaction> {
        self.call_raw("fundrawtransaction", &[json!(hexstring), json!(opts.options), json!(opts.is_witness)]).await
    }

    /// `getprivatebroadcastinfo` with required arguments only.
    ///
    /// Returns information about transactions that are currently being privately broadcast.
    pub async fn get_private_broadcast_info(&self) -> Result<GetPrivateBroadcastInfo> {
        self.call_raw("getprivatebroadcastinfo", &[(); 0] as &[()]).await
    }

    /// `getrawtransaction` with the result selected for verbosity `0`.
    ///
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
    pub async fn get_raw_transaction_verbose_0(&self, txid: String) -> Result<GetRawTransactionVerbose0> {
        self.call_raw("getrawtransaction", &[json!(txid), json!(0), json!(null)]).await
    }

    /// `getrawtransaction` with the result selected for verbosity `0`. With all optional arguments via [`GetRawTransactionOptions`].
    ///
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
    pub async fn get_raw_transaction_verbose_0_with(&self, txid: String, opts: GetRawTransactionOptions) -> Result<GetRawTransactionVerbose0> {
        self.call_raw("getrawtransaction", &[json!(txid), json!(0), json!(opts.block_hash)]).await
    }

    /// `getrawtransaction` with the result selected for verbosity `1`.
    ///
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
    pub async fn get_raw_transaction_verbose_1(&self, txid: String) -> Result<GetRawTransactionVerbose1> {
        self.call_raw("getrawtransaction", &[json!(txid), json!(1), json!(null)]).await
    }

    /// `getrawtransaction` with the result selected for verbosity `1`. With all optional arguments via [`GetRawTransactionOptions`].
    ///
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
    pub async fn get_raw_transaction_verbose_1_with(&self, txid: String, opts: GetRawTransactionOptions) -> Result<GetRawTransactionVerbose1> {
        self.call_raw("getrawtransaction", &[json!(txid), json!(1), json!(opts.block_hash)]).await
    }

    /// `getrawtransaction` with the result selected for verbosity `2`.
    ///
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
    pub async fn get_raw_transaction_verbose_2(&self, txid: String) -> Result<GetRawTransactionVerbose2> {
        self.call_raw("getrawtransaction", &[json!(txid), json!(2), json!(null)]).await
    }

    /// `getrawtransaction` with the result selected for verbosity `2`. With all optional arguments via [`GetRawTransactionOptions`].
    ///
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
    pub async fn get_raw_transaction_verbose_2_with(&self, txid: String, opts: GetRawTransactionOptions) -> Result<GetRawTransactionVerbose2> {
        self.call_raw("getrawtransaction", &[json!(txid), json!(2), json!(opts.block_hash)]).await
    }


    /// `joinpsbts` with required arguments only.
    ///
    /// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
    /// No input in any of the PSBTs can be in more than one of the PSBTs.
    pub async fn join_psbts(&self, txs: Vec<String>) -> Result<JoinPsbts> {
        self.call_raw("joinpsbts", &[json!(txs)]).await
    }

    /// `sendrawtransaction` with required arguments only.
    ///
    /// Submit a raw transaction (serialized, hex-encoded) to the network.
    /// 
    /// If -privatebroadcast is disabled, then the transaction will be put into the
    /// local mempool of the node and will be sent unconditionally to all currently
    /// connected peers, so using sendrawtransaction for manual rebroadcast will degrade
    /// privacy by leaking the transaction's origin, as nodes will normally not
    /// rebroadcast non-wallet transactions already in their mempool.
    /// 
    /// If -privatebroadcast is enabled, then the transaction will be sent only via
    /// dedicated, short-lived connections to Tor or I2P peers or IPv4/IPv6 peers
    /// via the Tor network. This conceals the transaction's origin. The transaction
    /// will only enter the local mempool when it is received back from the network.
    /// 
    /// A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
    /// 
    /// Related RPCs: createrawtransaction, signrawtransactionwithkey
    pub async fn send_raw_transaction(&self, hexstring: String) -> Result<SendRawTransaction> {
        self.call_raw("sendrawtransaction", &[json!(hexstring)]).await
    }

    /// `sendrawtransaction` with all optional arguments via [`SendRawTransactionOptions`].
    ///
    /// Submit a raw transaction (serialized, hex-encoded) to the network.
    /// 
    /// If -privatebroadcast is disabled, then the transaction will be put into the
    /// local mempool of the node and will be sent unconditionally to all currently
    /// connected peers, so using sendrawtransaction for manual rebroadcast will degrade
    /// privacy by leaking the transaction's origin, as nodes will normally not
    /// rebroadcast non-wallet transactions already in their mempool.
    /// 
    /// If -privatebroadcast is enabled, then the transaction will be sent only via
    /// dedicated, short-lived connections to Tor or I2P peers or IPv4/IPv6 peers
    /// via the Tor network. This conceals the transaction's origin. The transaction
    /// will only enter the local mempool when it is received back from the network.
    /// 
    /// A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
    /// 
    /// Related RPCs: createrawtransaction, signrawtransactionwithkey
    pub async fn send_raw_transaction_with(&self, hexstring: String, opts: SendRawTransactionOptions) -> Result<SendRawTransaction> {
        self.call_raw("sendrawtransaction", &[json!(hexstring), json!(opts.max_fee_rate), json!(opts.max_burn_amount)]).await
    }

    /// `signrawtransactionwithkey` with required arguments only.
    ///
    /// Sign inputs for raw transaction (serialized, hex-encoded).
    /// The second argument is an array of base58-encoded private
    /// keys that will be the only keys used to sign the transaction.
    /// The third optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    pub async fn sign_raw_transaction_with_key(&self, hexstring: String, priv_keys: Vec<String>) -> Result<SignRawTransactionWithKey> {
        self.call_raw("signrawtransactionwithkey", &[json!(hexstring), json!(priv_keys)]).await
    }

    /// `signrawtransactionwithkey` with all optional arguments via [`SignRawTransactionWithKeyOptions`].
    ///
    /// Sign inputs for raw transaction (serialized, hex-encoded).
    /// The second argument is an array of base58-encoded private
    /// keys that will be the only keys used to sign the transaction.
    /// The third optional argument (may be null) is an array of previous transaction outputs that
    /// this transaction depends on but may not yet be in the block chain.
    pub async fn sign_raw_transaction_with_key_with(&self, hexstring: String, priv_keys: Vec<String>, opts: SignRawTransactionWithKeyOptions) -> Result<SignRawTransactionWithKey> {
        self.call_raw("signrawtransactionwithkey", &[json!(hexstring), json!(priv_keys), json!(opts.prev_txs), json!(opts.sig_hash_type)]).await
    }

    /// `submitpackage` with required arguments only.
    ///
    /// Submit a package of raw transactions (serialized, hex-encoded) to local node.
    /// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
    /// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
    /// Warning: successful submission does not mean the transactions will propagate throughout the network.
    pub async fn submit_package(&self, package: Vec<String>) -> Result<SubmitPackage> {
        self.call_raw("submitpackage", &[json!(package)]).await
    }

    /// `submitpackage` with all optional arguments via [`SubmitPackageOptions`].
    ///
    /// Submit a package of raw transactions (serialized, hex-encoded) to local node.
    /// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
    /// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
    /// Warning: successful submission does not mean the transactions will propagate throughout the network.
    pub async fn submit_package_with(&self, package: Vec<String>, opts: SubmitPackageOptions) -> Result<SubmitPackage> {
        self.call_raw("submitpackage", &[json!(package), json!(opts.max_fee_rate), json!(opts.max_burn_amount)]).await
    }

    /// `testmempoolaccept` with required arguments only.
    ///
    /// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
    /// 
    /// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
    /// 
    /// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
    /// 
    /// The maximum number of transactions allowed is 25.
    /// 
    /// This checks if transactions violate the consensus or policy rules.
    /// 
    /// See sendrawtransaction call.
    pub async fn test_mempool_accept(&self, raw_txs: Vec<String>) -> Result<TestMempoolAccept> {
        self.call_raw("testmempoolaccept", &[json!(raw_txs)]).await
    }

    /// `testmempoolaccept` with all optional arguments via [`TestMempoolAcceptOptions`].
    ///
    /// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
    /// 
    /// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
    /// 
    /// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
    /// 
    /// The maximum number of transactions allowed is 25.
    /// 
    /// This checks if transactions violate the consensus or policy rules.
    /// 
    /// See sendrawtransaction call.
    pub async fn test_mempool_accept_with(&self, raw_txs: Vec<String>, opts: TestMempoolAcceptOptions) -> Result<TestMempoolAccept> {
        self.call_raw("testmempoolaccept", &[json!(raw_txs), json!(opts.max_fee_rate)]).await
    }

    /// `utxoupdatepsbt` with required arguments only.
    ///
    /// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
    pub async fn utxo_update_psbt(&self, psbt: String) -> Result<UtxoUpdatePsbt> {
        self.call_raw("utxoupdatepsbt", &[json!(psbt)]).await
    }

    /// `utxoupdatepsbt` with all optional arguments via [`UtxoUpdatePsbtOptions`].
    ///
    /// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
    pub async fn utxo_update_psbt_with(&self, psbt: String, opts: UtxoUpdatePsbtOptions) -> Result<UtxoUpdatePsbt> {
        self.call_raw("utxoupdatepsbt", &[json!(psbt), json!(opts.descriptors)]).await
    }

}
