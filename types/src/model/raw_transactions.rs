// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the `== Rawtransactions ==` section of the API docs.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use alloc::collections::BTreeMap;

use bitcoin::address::{Address, NetworkUnchecked};
use bitcoin::hashes::{hash160, sha256};
use bitcoin::{Amount, BlockHash, FeeRate, Psbt, ScriptBuf, Sequence, Transaction, Txid, Wtxid};
use serde::{Deserialize, Serialize};

/// Models the result of JSON-RPC method `analyzepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AnalyzePsbt {
    /// Array of input objects.
    pub inputs: Vec<AnalyzePsbtInput>,
    /// Estimated vsize of the final signed transaction.
    pub estimated_vsize: Option<u32>,
    /// Estimated feerate of the final signed transaction in BTC/kB.
    ///
    /// Shown only if all UTXO slots in the PSBT have been filled.
    pub estimated_fee_rate: Option<FeeRate>,
    /// The transaction fee paid. Shown only if all UTXO slots in the PSBT have been filled.
    pub fee: Option<Amount>,
    /// Role of the next person that this psbt needs to go to.
    pub next: String,
}

/// Represents an input in a PSBT operation.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
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

/// Represents missing elements required to complete an input.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AnalyzePsbtInputMissing {
    /// Public key IDs of public keys whose BIP 32 derivation paths are missing.
    pub pubkeys: Vec<hash160::Hash>,
    /// Public key IDs of public keys whose signatures are missing.
    pub signatures: Vec<hash160::Hash>,
    /// Hash160 of the redeem script that is missing.
    pub redeem_script: Option<hash160::Hash>,
    /// SHA256 of the witness script that is missing.
    pub witness_script: Option<sha256::Hash>,
}

/// Models the result of JSON-RPC method `combinepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CombinePsbt(pub Psbt);

/// Models the result of JSON-RPC method `combinerawtransaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CombineRawTransaction(pub Transaction);

/// Models the result of JSON-RPC method `converttopsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ConvertToPsbt(pub Psbt);

/// Models the result of JSON-RPC method `createpsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreatePsbt(pub Psbt);

/// Models the result of JSON-RPC method `createrawtransaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateRawTransaction(pub Transaction);

/// Models the result of JSON-RPC method `decodepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DecodePsbt {
    /// The decoded PSBT.
    pub psbt: Psbt,
    /// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
    pub fee: Option<Amount>,
}

/// Models the result of JSON-RPC method `decoderawtransaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DecodeRawTransaction(pub Transaction);

/// Models the result of JSON-RPC method `decodescript`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DecodeScript {
    /// The `scriptPubkey`.
    pub script_pubkey: Option<ScriptBuf>,
    /// The output type.
    pub type_: String,
    /// The required signatures.
    pub required_signatures: Option<u64>,
    /// List of bitcoin addresses.
    pub addresses: Vec<Address<NetworkUnchecked>>,
    /// Address of P2SH script wrapping this redeem script (not returned if the script is already a P2SH).
    pub p2sh: Option<Address<NetworkUnchecked>>,
    /// Address of the P2SH script wrapping this witness redeem script
    pub p2sh_segwit: Option<String>,
}

/// Models the result of JSON-RPC method `descriptorprocesspsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DescriptorProcessPsbt {
    /// The decoded PSBT.
    pub psbt: Psbt,
    /// If the transaction has a complete set of signatures.
    pub complete: bool,
    /// The transaction if complete.
    pub tx: Option<Transaction>,
}

/// Models the result of JSON-RPC method `finalizepsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FinalizePsbt {
    /// The partially signed transaction if not extracted.
    pub psbt: Psbt,
    /// The transaction if extracted.
    pub tx: Option<Transaction>,
    /// If the transaction has a complete set of signatures.
    pub complete: bool,
}

/// Models the result of JSON-RPC method `fundrawtransaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FundRawTransaction {
    /// The resulting raw transaction.
    pub tx: Transaction,
    /// Fee the resulting transaction pays.
    pub fee: Amount,
    /// The position of the added change output, or -1.
    pub change_position: i64,
}

/// Models the result of JSON-RPC method `getrawtransaction` with verbose set to `false`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetRawTransaction(pub Transaction);

/// Models the result of JSON-RPC method `getrawtransaction` with verbose set to `true`.
/// Result of JSON-RPC method `getrawtransaction`
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetRawTransactionVerbose {
    /// Whether specified block is in the active chain or not (only present with explicit "blockhash" argument).
    pub in_active_chain: Option<bool>,
    /// The transaction (encapsulates the other data returned by original RPC call).
    pub transaction: Transaction,
    /// The block hash (`None` for mempool transactions).
    pub block_hash: Option<BlockHash>,
    /// The confirmations (`None` for mempool transactions).
    pub confirmations: Option<u64>,
    /// The transaction time in seconds since epoch (Jan 1 1970 GMT).
    pub transaction_time: Option<u64>,
    /// The block time in seconds since epoch (Jan 1 1970 GMT).
    pub block_time: Option<u64>,
}

/// Models the result of JSON-RPC method `joinpsbts`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct JoinPsbts(pub Psbt);

/// Models the result of JSON-RPC method `sendrawtransaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendRawTransaction(pub Txid);

/// Models the result of JSON-RPC method `signrawtransaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SignRawTransaction {
    /// The raw transaction with signature(s).
    pub tx: Transaction,
    /// If the transaction has a complete set of signatures.
    pub complete: bool,
    /// Script verification errors (if there are any).
    pub errors: Vec<SignFail>,
}

/// Represents a script verification error.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SignFail {
    /// The referenced, previous transaction.
    pub txid: Txid,
    /// The index of the output to spent and used as input.
    pub vout: u64,
    /// The signature script.
    pub script_sig: ScriptBuf,
    /// Script sequence number.
    pub sequence: Sequence,
    /// Verification or signing error related to the input.
    pub error: String,
}

/// Models the result of JSON-RPC method `submitpackage`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackage {
    /// The transaction package result message. "success" indicates all transactions were accepted into or are already in the mempool.
    pub package_msg: String,
    /// Transaction results keyed by [`Wtxid`].
    pub tx_results: BTreeMap<Wtxid, SubmitPackageTxResult>,
    /// List of txids of replaced transactions.
    pub replaced_transactions: Vec<Txid>,
}

/// Models the per-transaction result included in the JSON-RPC method `submitpackage`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackageTxResult {
    /// The transaction id.
    pub txid: Txid,
    /// The [`Wtxid`] of a different transaction with the same [`Txid`] but different witness found in the mempool.
    ///
    /// If set, this means the submitted transaction was ignored.
    pub other_wtxid: Option<Wtxid>,
    /// Sigops-adjusted virtual transaction size.
    pub vsize: Option<u32>,
    /// Transaction fees.
    pub fees: Option<SubmitPackageTxResultFees>,
    /// The transaction error string, if it was rejected by the mempool
    pub error: Option<String>,
}

/// Models the fees included in the per-transaction result of the JSON-RPC method `submitpackage`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackageTxResultFees {
    /// Transaction fee.
    pub base_fee: Amount,
    /// The effective feerate.
    ///
    /// Will be `None` if the transaction was already in the mempool. For example, the package
    /// feerate and/or feerate with modified fees from the `prioritisetransaction` JSON-RPC method.
    pub effective_fee_rate: Option<FeeRate>,
    /// If [`Self::effective_fee_rate`] is provided, this holds the [`Wtxid`]s of the transactions
    /// whose fees and vsizes are included in effective-feerate.
    pub effective_includes: Vec<Wtxid>,
}

/// Models the result of JSON-RPC method `testmempoolaccept`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TestMempoolAccept {
    /// Test results for each raw transaction in the input array.
    pub results: Vec<MempoolAcceptance>,
}

/// Represents a single mempool acceptance test result.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MempoolAcceptance {
    /// The transaction ID.
    pub txid: Txid,
    /// If the mempool allows this transaction to be inserted.
    pub allowed: bool,
    /// Rejection string (only present when 'allowed' is false).
    pub reject_reason: Option<String>,
}

/// Models the result of JSON-RPC method `utxoupdatepsbt;`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UtxoUpdatePsbt(pub Psbt);
