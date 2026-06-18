// SPDX-License-Identifier: CC0-1.0

//! Auto-generated types for Bitcoin Core `31` - blockchain.
//!
//! Produced by `codegen`. Do not edit by hand, re-run
//! `just codegen` to regenerate. Holds the RPC return types for this section; the
//! `*Options` request structs live with the call surface in `corepc-client`.

#![allow(non_camel_case_types, non_snake_case, clippy::large_enum_variant)]

mod into;

use serde::{Deserialize, Serialize};

pub use self::into::{
    ActivityEntryError, Bip9InfoError, Bip9StatisticsError, ChainStateError, ChainTipsError,
    ChunkError, CoinbaseTransactionError, DeploymentInfoError, DumpTxOutSetError,
    GetBestBlockHashError, GetBlockCountError, GetBlockFilterError, GetBlockHashError,
    GetBlockHeaderError, GetBlockHeaderVerboseError, GetBlockStatsError, GetBlockVerboseOneError,
    GetBlockVerboseThreeError, GetBlockVerboseThreePrevoutError,
    GetBlockVerboseThreeTransactionError, GetBlockVerboseTwoError,
    GetBlockVerboseTwoTransactionError, GetBlockVerboseZeroError, GetBlockchainInfoError,
    GetChainStatesError, GetChainTipsError, GetChainTxStatsError, GetDeploymentInfoError,
    GetDescriptorActivityError, GetDifficultyError, GetMempoolAncestorsError,
    GetMempoolAncestorsVerboseError, GetMempoolClusterError, GetMempoolDescendantsError,
    GetMempoolDescendantsVerboseError, GetMempoolEntryError, GetMempoolInfoError,
    GetRawMempoolResultError, GetRawMempoolSequenceError, GetTxOutError,
    GetTxOutSetInfoBlockInfoError, GetTxOutSetInfoError, GetTxOutSetInfoUnspendablesError,
    GetTxSpendingPrevoutError, GetTxSpendingPrevoutItemError, LoadTxOutSetError, MempoolEntryError,
    MempoolEntryFeesError, ReceiveActivityError, ScanTxOutSetStartError, ScanTxOutSetUnspentError,
    ScriptPubKeyError, SpendActivityError, VerifyTxOutProofError, WaitForBlockError,
    WaitForBlockHeightError, WaitForNewBlockError,
};

/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
///
/// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
///
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct DumpTxOutSet {
    /// the hash of the base of the snapshot
    pub base_hash: String,
    /// the height of the base of the snapshot
    pub base_height: i64,
    /// the number of coins written in the snapshot
    pub coins_written: u64,
    /// the number of transactions in the chain up to and including the base block
    #[serde(rename = "nchaintx")]
    pub n_chain_tx: u64,
    /// the absolute path that the snapshot was written to
    pub path: String,
    /// the hash of the UTXO set contents
    #[serde(rename = "txoutset_hash")]
    pub tx_out_set_hash: String,
}

/// Result of the JSON-RPC method `getbestblockhash`.
///
/// > getbestblockhash
/// >
/// > Returns the hash of the best (tip) block in the most-work fully-validated chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBestBlockHash(pub String);

impl std::ops::Deref for GetBestBlockHash {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Result of the JSON-RPC method `getblockcount`.
///
/// > getblockcount
/// >
/// > Returns the height of the most-work fully-validated chain.
/// > The genesis block has height 0.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockCount(pub i64);

impl std::ops::Deref for GetBlockCount {
    type Target = i64;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Retrieve a BIP 157 content filter for a particular block.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockFilter {
    /// the hex-encoded filter data
    pub filter: String,
    /// the hex-encoded filter header
    pub header: String,
}

/// Attempt to fetch block from a given peer.
///
/// We must have the header for this block, e.g. using submitheader.
/// The block will not have any undo data which can limit the usage of the block data in a context where the undo data is needed.
/// Subsequent calls for the same block may cause the response from the previous peer to be ignored.
/// Peers generally ignore requests for a stale block that they never fully verified, or one that is more than a month old.
/// When a peer does not respond with a block, we will disconnect.
/// Note: The block could be re-pruned as soon as it is received.
///
/// Returns an empty JSON object if the request was successfully scheduled.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockFromPeer {}

/// Result of the JSON-RPC method `getblockhash`.
///
/// > getblockhash
/// >
/// > Returns hash of block in best-block-chain at height provided.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHash(pub String);

impl std::ops::Deref for GetBlockHash {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader \<hash\>.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHeaderVerbose0(pub String);

impl std::ops::Deref for GetBlockHeaderVerbose0 {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader \<hash\>.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockHeaderVerbose1 {
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// Expected number of hashes required to produce the current chain
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The difficulty
    pub difficulty: f64,
    /// the block hash (same as provided)
    pub hash: String,
    /// The block height or index
    pub height: i64,
    /// The median block time expressed in UNIX epoch time
    #[serde(rename = "mediantime")]
    pub median_time: i64,
    /// The merkle root
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    /// The number of transactions in the block
    #[serde(rename = "nTx")]
    pub n_tx: i64,
    /// The hash of the next block (if available)
    #[serde(rename = "nextblockhash", skip_serializing_if = "Option::is_none")]
    pub next_block_hash: Option<String>,
    /// The nonce
    pub nonce: i64,
    /// The hash of the previous block (if available)
    #[serde(rename = "previousblockhash", skip_serializing_if = "Option::is_none")]
    pub previous_block_hash: Option<String>,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: i64,
    /// The block version
    pub version: i64,
    /// The block version formatted in hexadecimal
    #[serde(rename = "versionHex")]
    pub version_hex: String,
}

/// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won't work for some heights with pruning.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockStats {
    /// Average fee in the block
    #[serde(rename = "avgfee", skip_serializing_if = "Option::is_none")]
    pub avg_fee: Option<i64>,
    /// Average feerate (in satoshis per virtual byte)
    #[serde(rename = "avgfeerate", skip_serializing_if = "Option::is_none")]
    pub avg_fee_rate: Option<i64>,
    /// Average transaction size
    #[serde(rename = "avgtxsize", skip_serializing_if = "Option::is_none")]
    pub avg_tx_size: Option<i64>,
    /// The block hash (to check for potential reorgs)
    #[serde(rename = "blockhash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    /// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte)
    #[serde(rename = "feerate_percentiles", skip_serializing_if = "Option::is_none")]
    pub fee_rate_percentiles: Option<Vec<serde_json::Value>>,
    /// The height of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The number of inputs (excluding coinbase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins: Option<i64>,
    /// Maximum fee in the block
    #[serde(rename = "maxfee", skip_serializing_if = "Option::is_none")]
    pub max_fee: Option<i64>,
    /// Maximum feerate (in satoshis per virtual byte)
    #[serde(rename = "maxfeerate", skip_serializing_if = "Option::is_none")]
    pub max_fee_rate: Option<i64>,
    /// Maximum transaction size
    #[serde(rename = "maxtxsize", skip_serializing_if = "Option::is_none")]
    pub max_tx_size: Option<i64>,
    /// Truncated median fee in the block
    #[serde(rename = "medianfee", skip_serializing_if = "Option::is_none")]
    pub median_fee: Option<i64>,
    /// The block median time past
    #[serde(rename = "mediantime", skip_serializing_if = "Option::is_none")]
    pub median_time: Option<i64>,
    /// Truncated median transaction size
    #[serde(rename = "mediantxsize", skip_serializing_if = "Option::is_none")]
    pub median_tx_size: Option<i64>,
    /// Minimum fee in the block
    #[serde(rename = "minfee", skip_serializing_if = "Option::is_none")]
    pub min_fee: Option<i64>,
    /// Minimum feerate (in satoshis per virtual byte)
    #[serde(rename = "minfeerate", skip_serializing_if = "Option::is_none")]
    pub min_fee_rate: Option<i64>,
    /// Minimum transaction size
    #[serde(rename = "mintxsize", skip_serializing_if = "Option::is_none")]
    pub min_tx_size: Option<i64>,
    /// The number of outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outs: Option<i64>,
    /// The block subsidy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsidy: Option<i64>,
    /// Total size of all segwit transactions
    #[serde(rename = "swtotal_size", skip_serializing_if = "Option::is_none")]
    pub sw_total_size: Option<i64>,
    /// Total weight of all segwit transactions
    #[serde(rename = "swtotal_weight", skip_serializing_if = "Option::is_none")]
    pub sw_total_weight: Option<i64>,
    /// The number of segwit transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swtxs: Option<i64>,
    /// The block time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Total amount in all outputs (excluding coinbase and thus reward \[ie subsidy + totalfee\])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_out: Option<i64>,
    /// Total size of all non-coinbase transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i64>,
    /// Total weight of all non-coinbase transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_weight: Option<i64>,
    /// The fee total
    #[serde(rename = "totalfee", skip_serializing_if = "Option::is_none")]
    pub total_fee: Option<i64>,
    /// The number of transactions (including coinbase)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs: Option<u64>,
    /// The increase/decrease in the number of unspent outputs (not discounting op_return and similar)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase: Option<i64>,
    /// The increase/decrease in the number of unspent outputs, not counting unspendables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_increase_actual: Option<i64>,
    /// The increase/decrease in size for the utxo index (not discounting op_return and similar)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc: Option<i64>,
    /// The increase/decrease in size for the utxo index, not counting unspendables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utxo_size_inc_actual: Option<i64>,
}

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block \<hash\>.
/// If verbosity is 2, returns an Object with information about block \<hash\> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block \<hash\> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose0(pub String);

impl std::ops::Deref for GetBlockVerbose0 {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block \<hash\>.
/// If verbosity is 2, returns an Object with information about block \<hash\> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block \<hash\> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose1 {
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// Expected number of hashes required to produce the chain up to this block (in hex)
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// Coinbase transaction metadata
    pub coinbase_tx: GetBlockVerbose1CoinbaseTx,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The difficulty
    pub difficulty: f64,
    /// the block hash (same as provided)
    pub hash: String,
    /// The block height or index
    pub height: i64,
    /// The median block time expressed in UNIX epoch time
    #[serde(rename = "mediantime")]
    pub median_time: i64,
    /// The merkle root
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    /// The number of transactions in the block
    #[serde(rename = "nTx")]
    pub n_tx: i64,
    /// The hash of the next block (if available)
    #[serde(rename = "nextblockhash", skip_serializing_if = "Option::is_none")]
    pub next_block_hash: Option<String>,
    /// The nonce
    pub nonce: i64,
    /// The hash of the previous block (if available)
    #[serde(rename = "previousblockhash", skip_serializing_if = "Option::is_none")]
    pub previous_block_hash: Option<String>,
    /// The block size
    pub size: u64,
    /// The block size excluding witness data
    #[serde(rename = "strippedsize")]
    pub stripped_size: u64,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: i64,
    /// The transaction ids
    pub tx: Vec<String>,
    /// The block version
    pub version: i64,
    /// The block version formatted in hexadecimal
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The block weight as defined in BIP 141
    pub weight: i64,
}

/// Coinbase transaction metadata
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose1CoinbaseTx {
    /// The coinbase input's script
    pub coinbase: String,
    /// The coinbase transaction's locktime (nLockTime)
    pub locktime: i64,
    /// The coinbase input's sequence number (nSequence)
    pub sequence: i64,
    /// The coinbase transaction version
    pub version: i64,
    /// The coinbase input's first (and only) witness stack element, if present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness: Option<String>,
}

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block \<hash\>.
/// If verbosity is 2, returns an Object with information about block \<hash\> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block \<hash\> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose2 {
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// Expected number of hashes required to produce the chain up to this block (in hex)
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// Coinbase transaction metadata
    pub coinbase_tx: GetBlockVerbose2CoinbaseTx,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The difficulty
    pub difficulty: f64,
    /// the block hash (same as provided)
    pub hash: String,
    /// The block height or index
    pub height: i64,
    /// The median block time expressed in UNIX epoch time
    #[serde(rename = "mediantime")]
    pub median_time: i64,
    /// The merkle root
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    /// The number of transactions in the block
    #[serde(rename = "nTx")]
    pub n_tx: i64,
    /// The hash of the next block (if available)
    #[serde(rename = "nextblockhash", skip_serializing_if = "Option::is_none")]
    pub next_block_hash: Option<String>,
    /// The nonce
    pub nonce: i64,
    /// The hash of the previous block (if available)
    #[serde(rename = "previousblockhash", skip_serializing_if = "Option::is_none")]
    pub previous_block_hash: Option<String>,
    /// The block size
    pub size: u64,
    /// The block size excluding witness data
    #[serde(rename = "strippedsize")]
    pub stripped_size: u64,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: i64,
    pub tx: Vec<GetBlockVerbose2TxItem>,
    /// The block version
    pub version: i64,
    /// The block version formatted in hexadecimal
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The block weight as defined in BIP 141
    pub weight: i64,
}

/// Coinbase transaction metadata
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose2CoinbaseTx {
    /// The coinbase input's script
    pub coinbase: String,
    /// The coinbase transaction's locktime (nLockTime)
    pub locktime: i64,
    /// The coinbase input's sequence number (nSequence)
    pub sequence: i64,
    /// The coinbase transaction version
    pub version: i64,
    /// The coinbase input's first (and only) witness stack element, if present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockVerbose2TxItem {
    /// The transaction fee in BTC, omitted if block undo data is not available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The hex-encoded transaction data
    pub hex: String,
    /// The lock time
    pub locktime: i64,
    /// The serialized transaction size
    pub size: i64,
    /// The transaction id
    pub txid: String,
    /// The version
    pub version: i64,
    pub vin: Vec<GetBlockVerbose2TxItemVinItem>,
    pub vout: Vec<GetBlockVerbose2TxItemVoutItem>,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: i64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: i64,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose2TxItemVinItem {
    /// The coinbase value (only if coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,
    /// The script (if not coinbase transaction)
    #[serde(rename = "scriptSig", skip_serializing_if = "Option::is_none")]
    pub script_sig: Option<GetBlockVerbose2TxItemVinItemScriptSig>,
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
pub struct GetBlockVerbose2TxItemVinItemScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose2TxItemVoutItem {
    /// index
    pub n: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetBlockVerbose2TxItemVoutItemScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose2TxItemVoutItemScriptPubKey {
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

/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block \<hash\>.
/// If verbosity is 2, returns an Object with information about block \<hash\> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block \<hash\> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose3 {
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// Expected number of hashes required to produce the chain up to this block (in hex)
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// Coinbase transaction metadata
    pub coinbase_tx: GetBlockVerbose3CoinbaseTx,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: i64,
    /// The difficulty
    pub difficulty: f64,
    /// the block hash (same as provided)
    pub hash: String,
    /// The block height or index
    pub height: i64,
    /// The median block time expressed in UNIX epoch time
    #[serde(rename = "mediantime")]
    pub median_time: i64,
    /// The merkle root
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
    /// The number of transactions in the block
    #[serde(rename = "nTx")]
    pub n_tx: i64,
    /// The hash of the next block (if available)
    #[serde(rename = "nextblockhash", skip_serializing_if = "Option::is_none")]
    pub next_block_hash: Option<String>,
    /// The nonce
    pub nonce: i64,
    /// The hash of the previous block (if available)
    #[serde(rename = "previousblockhash", skip_serializing_if = "Option::is_none")]
    pub previous_block_hash: Option<String>,
    /// The block size
    pub size: u64,
    /// The block size excluding witness data
    #[serde(rename = "strippedsize")]
    pub stripped_size: u64,
    /// The difficulty target
    pub target: String,
    /// The block time expressed in UNIX epoch time
    pub time: i64,
    pub tx: Vec<GetBlockVerbose3TxItem>,
    /// The block version
    pub version: i64,
    /// The block version formatted in hexadecimal
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The block weight as defined in BIP 141
    pub weight: i64,
}

/// Coinbase transaction metadata
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose3CoinbaseTx {
    /// The coinbase input's script
    pub coinbase: String,
    /// The coinbase transaction's locktime (nLockTime)
    pub locktime: i64,
    /// The coinbase input's sequence number (nSequence)
    pub sequence: i64,
    /// The coinbase transaction version
    pub version: i64,
    /// The coinbase input's first (and only) witness stack element, if present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witness: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockVerbose3TxItem {
    /// transaction fee in BTC, omitted if block undo data is not available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: String,
    /// The hex-encoded transaction data
    pub hex: String,
    /// The lock time
    pub locktime: i64,
    /// The serialized transaction size
    pub size: i64,
    /// The transaction id
    pub txid: String,
    /// The version
    pub version: i64,
    pub vin: Vec<GetBlockVerbose3TxItemVinItem>,
    pub vout: Vec<GetBlockVerbose3TxItemVoutItem>,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: i64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: i64,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose3TxItemVinItem {
    /// The coinbase value (only if coinbase transaction)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,
    /// (Only if undo information is available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevout: Option<GetBlockVerbose3TxItemVinItemPrevout>,
    /// The script (if not coinbase transaction)
    #[serde(rename = "scriptSig", skip_serializing_if = "Option::is_none")]
    pub script_sig: Option<GetBlockVerbose3TxItemVinItemScriptSig>,
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

/// (Only if undo information is available)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose3TxItemVinItemPrevout {
    /// Coinbase or not
    pub generated: bool,
    /// The height of the prevout
    pub height: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetBlockVerbose3TxItemVinItemPrevoutScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose3TxItemVinItemPrevoutScriptPubKey {
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
pub struct GetBlockVerbose3TxItemVinItemScriptSig {
    /// Disassembly of the signature script
    pub asm: String,
    /// The raw signature script bytes, hex-encoded
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose3TxItemVoutItem {
    /// index
    pub n: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetBlockVerbose3TxItemVoutItemScriptPubKey,
    /// The value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockVerbose3TxItemVoutItemScriptPubKey {
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

/// Returns an object containing various state info regarding blockchain processing.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBlockchainInfo {
    /// whether automatic pruning is enabled (only present if pruning is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_pruning: Option<bool>,
    /// the hash of the currently best block
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// the height of the most-work fully-validated chain. The genesis block has height 0
    pub blocks: i64,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// total amount of work in active chain, in hexadecimal
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// the current difficulty
    pub difficulty: f64,
    /// the current number of headers we have validated
    pub headers: i64,
    /// (debug information) estimate of whether this node is in Initial Block Download mode
    #[serde(rename = "initialblockdownload")]
    pub initial_block_download: bool,
    /// the median block time expressed in UNIX epoch time
    #[serde(rename = "mediantime")]
    pub median_time: i64,
    /// the target size used by pruning (only present if automatic pruning is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune_target_size: Option<u64>,
    /// if the blocks are subject to pruning
    pub pruned: bool,
    /// the first block unpruned, all previous blocks were pruned (only present if pruning is enabled)
    #[serde(rename = "pruneheight", skip_serializing_if = "Option::is_none")]
    pub prune_height: Option<i64>,
    /// the block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signet_challenge: Option<String>,
    /// the estimated size of the block and undo files on disk
    pub size_on_disk: u64,
    /// the difficulty target
    pub target: String,
    /// the block time expressed in UNIX epoch time
    pub time: i64,
    /// estimate of verification progress \[0..1\]
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,
}

/// Return information about chainstates.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainStates {
    /// list of the chainstates ordered by work, with the most-work (active) chainstate last
    #[serde(rename = "chainstates")]
    pub chain_states: Vec<GetChainStatesChainStatesItem>,
    /// the number of headers seen so far
    pub headers: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainStatesChainStatesItem {
    /// blockhash of the tip
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// nBits: compact representation of the block difficulty target
    pub bits: String,
    /// number of blocks in this chainstate
    pub blocks: i64,
    /// size of the coinsdb cache
    pub coins_db_cache_bytes: u64,
    /// size of the coinstip cache
    pub coins_tip_cache_bytes: u64,
    /// difficulty of the tip
    pub difficulty: f64,
    /// the base block of the snapshot this chainstate is based on, if any
    #[serde(rename = "snapshot_blockhash", skip_serializing_if = "Option::is_none")]
    pub snapshot_block_hash: Option<String>,
    /// The difficulty target
    pub target: String,
    /// whether the chainstate is fully validated. True if all blocks in the chainstate were validated, false if the chain is based on a snapshot and the snapshot has not yet been validated.
    pub validated: bool,
    /// progress towards the network tip
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
}

/// Result of the JSON-RPC method `getchaintips`.
///
/// > getchaintips
/// >
/// > Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTips(pub Vec<GetChainTipsItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTipsItem {
    /// zero for main chain, otherwise length of branch connecting the tip to the main chain
    pub branchlen: i64,
    /// block hash of the tip
    pub hash: String,
    /// height of the chain tip
    pub height: i64,
    /// status of the chain, "active" for the main chain
    /// Possible values for status:
    /// 1.  "invalid"               This branch contains at least one invalid block
    /// 2.  "headers-only"          Not all blocks for this branch are available, but the headers are valid
    /// 3.  "valid-headers"         All blocks are available for this branch, but they were never fully validated
    /// 4.  "valid-fork"            This branch is not part of the active chain, but is fully validated
    /// 5.  "active"                This is the tip of the active main chain, which is certainly valid
    pub status: String,
}

/// Compute statistics about the total number and rate of transactions in the chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetChainTxStats {
    /// The timestamp for the final block in the window, expressed in UNIX epoch time
    pub time: i64,
    /// The total number of transactions in the chain up to that point, if known. It may be unknown when using assumeutxo.
    #[serde(rename = "txcount", skip_serializing_if = "Option::is_none")]
    pub tx_count: Option<u64>,
    /// The average rate of transactions per second in the window. Only returned if "window_interval" is > 0 and if window_tx_count exists.
    #[serde(rename = "txrate", skip_serializing_if = "Option::is_none")]
    pub tx_rate: Option<f64>,
    /// Size of the window in number of blocks
    pub window_block_count: i64,
    /// The hash of the final block in the window
    pub window_final_block_hash: String,
    /// The height of the final block in the window.
    pub window_final_block_height: i64,
    /// The elapsed time in the window in seconds. Only returned if "window_block_count" is > 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_interval: Option<i64>,
    /// The number of transactions in the window. Only returned if "window_block_count" is > 0 and if txcount exists for the start and end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_tx_count: Option<u64>,
}

/// Returns an object containing various state info regarding deployments of consensus changes.
/// Consensus changes for which the new rules are enforced from genesis are not listed in "deployments".
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfo {
    pub deployments: std::collections::BTreeMap<String, GetDeploymentInfoDeployments>,
    /// requested block hash (or tip)
    pub hash: String,
    /// requested block height (or tip)
    pub height: i64,
    /// script verify flags for the block
    pub script_flags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfoDeployments {
    /// true if the rules are enforced for the mempool and the next block
    pub active: bool,
    /// status of bip9 softforks (only for "bip9" type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bip9: Option<GetDeploymentInfoDeploymentsBip9>,
    /// height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// one of "buried", "bip9"
    #[serde(rename = "type")]
    pub type_: String,
}

/// status of bip9 softforks (only for "bip9" type)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfoDeploymentsBip9 {
    /// the bit (0-28) in the block version field used to signal this softfork (only for "started" and "locked_in" status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit: Option<i64>,
    /// minimum height of blocks for which the rules may be enforced
    pub min_activation_height: i64,
    /// indicates blocks that signalled with a # and blocks that did not with a -
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signalling: Option<String>,
    /// height of the first block to which the status applies
    pub since: i64,
    /// the minimum median time past of a block at which the bit gains its meaning
    pub start_time: i64,
    /// numeric statistics about signalling for a softfork (only for "started" and "locked_in" status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<GetDeploymentInfoDeploymentsBip9Statistics>,
    /// status of deployment at specified block (one of "defined", "started", "locked_in", "active", "failed")
    pub status: String,
    /// status of deployment at the next block
    pub status_next: String,
    /// the median time past of a block at which the deployment is considered failed if not yet locked in
    #[serde(rename = "timeout")]
    pub time_out: i64,
}

/// numeric statistics about signalling for a softfork (only for "started" and "locked_in" status)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDeploymentInfoDeploymentsBip9Statistics {
    /// the number of blocks with the version bit set in the current period
    pub count: i64,
    /// the number of blocks elapsed since the beginning of the current period
    pub elapsed: i64,
    /// the length in blocks of the signalling period
    pub period: i64,
    /// returns false if there are not enough blocks left in this period to pass activation threshold (only for "started" status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible: Option<bool>,
    /// the number of blocks with the version bit set required to activate the feature (only for "started" status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i64>,
}

/// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
/// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivity {
    /// events
    pub activity: Vec<GetDescriptorActivityActivity>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GetDescriptorActivityActivity {
    Object(GetDescriptorActivityActivityVariant0),
    Object2(GetDescriptorActivityActivityVariant1),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivityActivityVariant0 {
    /// The total amount in BTC of the spent output
    pub amount: f64,
    /// The blockhash this spend appears in (omitted if unconfirmed)
    #[serde(rename = "blockhash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    /// Height of the spend (omitted if unconfirmed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    pub prevout_spk: GetDescriptorActivityActivityVariant0PrevoutSpk,
    /// The txid of the prevout
    pub prevout_txid: String,
    /// The vout of the prevout
    pub prevout_vout: i64,
    /// The txid of the spending transaction
    pub spend_txid: String,
    /// The input index of the spend
    pub spend_vin: i64,
    /// always 'spend'
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivityActivityVariant0PrevoutSpk {
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
pub struct GetDescriptorActivityActivityVariant1 {
    /// The total amount in BTC of the new output
    pub amount: f64,
    /// The block that this receive is in (omitted if unconfirmed)
    #[serde(rename = "blockhash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    /// The height of the receive (omitted if unconfirmed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    pub output_spk: GetDescriptorActivityActivityVariant1OutputSpk,
    /// The txid of the receiving transaction
    pub txid: String,
    /// always 'receive'
    #[serde(rename = "type")]
    pub type_: String,
    /// The vout of the receiving output
    pub vout: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDescriptorActivityActivityVariant1OutputSpk {
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

/// Result of the JSON-RPC method `getdifficulty`.
///
/// > getdifficulty
/// >
/// > Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetDifficulty(pub f64);

impl std::ops::Deref for GetDifficulty {
    type Target = f64;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// If txid is in the mempool, returns all in-mempool ancestors.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsVerbose0(pub Vec<String>);

/// If txid is in the mempool, returns all in-mempool ancestors.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsVerbose1(
    /// for verbose = true
    pub std::collections::BTreeMap<String, GetMempoolAncestorsVerbose1Entry>,
);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsVerbose1Entry {
    /// number of in-mempool ancestor transactions (including this one)
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: u64,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
    ///
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    #[serde(rename = "chunkweight")]
    pub chunk_weight: i64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendant_size: u64,
    pub fees: GetMempoolAncestorsVerbose1EntryFees,
    /// block height when transaction entered pool
    pub height: i64,
    /// unconfirmed transactions spending outputs from this transaction
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: i64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: i64,
    /// transaction weight as defined in BIP 141.
    pub weight: i64,
    /// hash of serialized transaction, including witness data
    pub wtxid: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolAncestorsVerbose1EntryFees {
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    pub ancestor: f64,
    /// transaction fee, denominated in BTC
    pub base: f64,
    /// transaction fees of chunk, denominated in BTC
    pub chunk: f64,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    pub descendant: f64,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    pub modified: f64,
}

/// Returns mempool data for given cluster
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolCluster {
    /// chunks in this cluster (in mining order)
    pub chunks: Vec<GetMempoolClusterChunksItem>,
    /// total sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop')
    #[serde(rename = "clusterweight")]
    pub cluster_weight: i64,
    /// number of transactions
    #[serde(rename = "txcount")]
    pub tx_count: u64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolClusterChunksItem {
    /// fees of the transactions in this chunk
    #[serde(rename = "chunkfee")]
    pub chunk_fee: f64,
    /// sigops-adjusted weight of all transactions in this chunk
    #[serde(rename = "chunkweight")]
    pub chunk_weight: i64,
    /// transactions in this chunk in mining order
    pub txs: Vec<String>,
}

/// If txid is in the mempool, returns all in-mempool descendants.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsVerbose0(pub Vec<String>);

/// If txid is in the mempool, returns all in-mempool descendants.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsVerbose1(
    /// for verbose = true
    pub std::collections::BTreeMap<String, GetMempoolDescendantsVerbose1Entry>,
);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsVerbose1Entry {
    /// number of in-mempool ancestor transactions (including this one)
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: u64,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
    ///
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    #[serde(rename = "chunkweight")]
    pub chunk_weight: i64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendant_size: u64,
    pub fees: GetMempoolDescendantsVerbose1EntryFees,
    /// block height when transaction entered pool
    pub height: i64,
    /// unconfirmed transactions spending outputs from this transaction
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: i64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: i64,
    /// transaction weight as defined in BIP 141.
    pub weight: i64,
    /// hash of serialized transaction, including witness data
    pub wtxid: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolDescendantsVerbose1EntryFees {
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    pub ancestor: f64,
    /// transaction fee, denominated in BTC
    pub base: f64,
    /// transaction fees of chunk, denominated in BTC
    pub chunk: f64,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    pub descendant: f64,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    pub modified: f64,
}

/// Returns mempool data for given transaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolEntry {
    /// number of in-mempool ancestor transactions (including this one)
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: u64,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
    ///
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    #[serde(rename = "chunkweight")]
    pub chunk_weight: i64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendant_size: u64,
    pub fees: GetMempoolEntryFees,
    /// block height when transaction entered pool
    pub height: i64,
    /// unconfirmed transactions spending outputs from this transaction
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: i64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: i64,
    /// transaction weight as defined in BIP 141.
    pub weight: i64,
    /// hash of serialized transaction, including witness data
    pub wtxid: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolEntryFees {
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    pub ancestor: f64,
    /// transaction fee, denominated in BTC
    pub base: f64,
    /// transaction fees of chunk, denominated in BTC
    pub chunk: f64,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    pub descendant: f64,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    pub modified: f64,
}

/// Returns details on the active state of the TX memory pool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetMempoolInfo {
    /// Sum of all virtual transaction sizes as defined in BIP 141. Differs from actual serialized size because witness data is discounted
    pub bytes: u64,
    /// True if the mempool accepts RBF without replaceability signaling inspection (DEPRECATED)
    #[serde(rename = "fullrbf")]
    pub full_rbf: bool,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    #[serde(rename = "incrementalrelayfee")]
    pub incremental_relay_fee: f64,
    /// Maximum number of transactions that can be in a cluster (configured by -limitclustercount)
    #[serde(rename = "limitclustercount")]
    pub limit_cluster_count: i64,
    /// Maximum size of a cluster in virtual bytes (configured by -limitclustersize)
    #[serde(rename = "limitclustersize")]
    pub limit_cluster_size: i64,
    /// True if the initial load attempt of the persisted mempool finished
    pub loaded: bool,
    /// Maximum number of bytes that can be used by OP_RETURN outputs in the mempool
    #[serde(rename = "maxdatacarriersize")]
    pub max_data_carrier_size: i64,
    /// Maximum memory usage for the mempool
    #[serde(rename = "maxmempool")]
    pub max_mempool: i64,
    /// Minimum fee rate in BTC/kvB for tx to be accepted. Is the maximum of minrelaytxfee and minimum mempool fee
    #[serde(rename = "mempoolminfee")]
    pub mempool_min_fee: f64,
    /// Current minimum relay fee for transactions
    #[serde(rename = "minrelaytxfee")]
    pub min_relay_tx_fee: f64,
    /// If the mempool is in a known-optimal transaction ordering
    pub optimal: bool,
    /// True if the mempool accepts transactions with bare multisig outputs
    #[serde(rename = "permitbaremultisig")]
    pub permit_bare_multisig: bool,
    /// Current tx count
    pub size: u64,
    /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction
    pub total_fee: f64,
    /// Current number of transactions that haven't passed initial broadcast yet
    #[serde(rename = "unbroadcastcount")]
    pub unbroadcast_count: u64,
    /// Total memory usage for the mempool
    pub usage: u64,
}

/// Result of the JSON-RPC method `getrawmempool`.
///
/// > getrawmempool
/// >
/// > Returns all transaction ids in memory pool as a json array of string transaction ids.
/// >
/// > Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GetRawMempool {
    List(Vec<String>),
    Object(std::collections::BTreeMap<String, GetRawMempoolVariant1>),
    Object2(GetRawMempoolVariant2),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolVariant1 {
    /// number of in-mempool ancestor transactions (including this one)
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: u64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: u64,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
    ///
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// sigops-adjusted weight (as defined in BIP 141 and modified by '-bytespersigop') of this transaction's chunk
    #[serde(rename = "chunkweight")]
    pub chunk_weight: i64,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<String>,
    /// number of in-mempool descendant transactions (including this one)
    #[serde(rename = "descendantcount")]
    pub descendant_count: u64,
    /// virtual transaction size of in-mempool descendants (including this one)
    #[serde(rename = "descendantsize")]
    pub descendant_size: u64,
    pub fees: GetRawMempoolVariant1Fees,
    /// block height when transaction entered pool
    pub height: i64,
    /// unconfirmed transactions spending outputs from this transaction
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: i64,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: i64,
    /// transaction weight as defined in BIP 141.
    pub weight: i64,
    /// hash of serialized transaction, including witness data
    pub wtxid: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolVariant1Fees {
    /// transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining priority, denominated in BTC
    pub ancestor: f64,
    /// transaction fee, denominated in BTC
    pub base: f64,
    /// transaction fees of chunk, denominated in BTC
    pub chunk: f64,
    /// transaction fees of in-mempool descendants (including this one) with fee deltas used for mining priority, denominated in BTC
    pub descendant: f64,
    /// transaction fee with fee deltas used for mining priority, denominated in BTC
    pub modified: f64,
}

/// for verbose = false and mempool_sequence = true
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetRawMempoolVariant2 {
    /// The mempool sequence value.
    pub mempool_sequence: u64,
    pub txids: Vec<String>,
}

/// Result of the JSON-RPC method `gettxout`.
///
/// > gettxout
/// >
/// > Returns details about an unspent transaction output.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GetTxOut {
    Null(()),
    Object(GetTxOutVariant1),
}

/// Result of the JSON-RPC method `gettxoutproof`.
///
/// > gettxoutproof
/// >
/// > Returns a hex-encoded proof that "txid" was included in a block.
/// >
/// > NOTE: By default this function only works sometimes. This is when there is an
/// > unspent output in the utxo for this transaction. To make it always work,
/// > you need to maintain a transaction index, using the -txindex command line option or
/// > specify the block in which the transaction is included manually (by blockhash).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutProof(pub String);

impl std::ops::Deref for GetTxOutProof {
    type Target = String;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutSetInfo {
    /// The hash of the block at which these statistics are calculated
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// Info on amounts in the block at this block height (only available if coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_info: Option<GetTxOutSetInfoBlockInfo>,
    /// Database-independent, meaningless metric indicating the UTXO set size
    #[serde(rename = "bogosize")]
    pub bogo_size: u64,
    /// The estimated size of the chainstate on disk (not available when coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<u64>,
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_serialized_3: Option<String>,
    /// The block height (index) of the returned statistics
    pub height: i64,
    /// The serialized hash (only present if 'muhash' hash_type is chosen)
    #[serde(rename = "muhash", skip_serializing_if = "Option::is_none")]
    pub mu_hash: Option<String>,
    /// The total amount of coins in the UTXO set
    pub total_amount: f64,
    /// The total amount of coins permanently excluded from the UTXO set (only available if coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_unspendable_amount: Option<f64>,
    /// The number of transactions with unspent outputs (not available when coinstatsindex is used)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<u64>,
    /// The number of unspent transaction outputs
    #[serde(rename = "txouts")]
    pub tx_outs: u64,
}

/// Info on amounts in the block at this block height (only available if coinstatsindex is used)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutSetInfoBlockInfo {
    /// Coinbase subsidy amount of this block
    pub coinbase: f64,
    /// Total amount of new outputs created by this block
    pub new_outputs_ex_coinbase: f64,
    /// Total amount of all prevouts spent in this block
    pub prevout_spent: f64,
    /// Total amount of unspendable outputs created in this block
    pub unspendable: f64,
    /// Detailed view of the unspendable categories
    pub unspendables: GetTxOutSetInfoBlockInfoUnspendables,
}

/// Detailed view of the unspendable categories
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutSetInfoBlockInfoUnspendables {
    /// Transactions overridden by duplicates (no longer possible with BIP30)
    pub bip30: f64,
    /// The unspendable amount of the Genesis block subsidy
    pub genesis_block: f64,
    /// Amounts sent to scripts that are unspendable (for example OP_RETURN outputs)
    pub scripts: f64,
    /// Fee rewards that miners did not claim in their coinbase transaction
    pub unclaimed_rewards: f64,
}

/// Otherwise
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutVariant1 {
    /// The hash of the block at the tip of the chain
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// Coinbase or not
    pub coinbase: bool,
    /// The number of confirmations
    pub confirmations: i64,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: GetTxOutVariant1ScriptPubKey,
    /// The transaction value in BTC
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxOutVariant1ScriptPubKey {
    /// The Bitcoin address (only if a well-defined address exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Disassembly of the output script
    pub asm: String,
    /// Inferred descriptor for the output
    pub desc: String,
    /// The raw output script bytes, hex-encoded
    pub hex: String,
    /// The type, eg pubkeyhash
    #[serde(rename = "type")]
    pub type_: String,
}

/// Result of the JSON-RPC method `gettxspendingprevout`.
///
/// > gettxspendingprevout
/// >
/// > Scans the mempool (and the txospenderindex, if available) to find transactions spending any of the given outputs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxSpendingPrevout(pub Vec<GetTxSpendingPrevoutItem>);

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTxSpendingPrevoutItem {
    /// the hash of the spending block (omitted if unspent or the spending tx is not confirmed)
    #[serde(rename = "blockhash", skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    /// the transaction spending this output (only if return_spending_tx is set, omitted if unspent)
    #[serde(rename = "spendingtx", skip_serializing_if = "Option::is_none")]
    pub spending_tx: Option<String>,
    /// the transaction id of the mempool transaction spending this output (omitted if unspent)
    #[serde(rename = "spendingtxid", skip_serializing_if = "Option::is_none")]
    pub spending_txid: Option<String>,
    /// the transaction id of the checked output
    pub txid: String,
    /// the vout value of the checked output
    pub vout: i64,
}

/// Import a mempool.dat file and attempt to add its contents to the mempool.
/// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ImportMempool {}

/// Load the serialized UTXO set from a file.
/// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
///
/// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
///
/// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct LoadTxOutSet {
    /// the height of the base of the snapshot
    pub base_height: i64,
    /// the number of coins loaded from the snapshot
    pub coins_loaded: u64,
    /// the absolute path that the snapshot was loaded from
    pub path: String,
    /// the hash of the base of the snapshot
    pub tip_hash: String,
}

/// Result of the JSON-RPC method `pruneblockchain`.
///
/// > pruneblockchain
/// >
/// > Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
/// > Requires `-prune` to be enabled at startup. While pruned data may be re-fetched in some cases (e.g., via `getblockfrompeer`), local deletion is irreversible.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PruneBlockchain(pub i64);

impl std::ops::Deref for PruneBlockchain {
    type Target = i64;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SaveMempool {
    /// the directory and file where the mempool was saved
    #[serde(rename = "filename")]
    pub file_name: String,
}

/// Result of the JSON-RPC method `scanblocks`.
///
/// > scanblocks
/// >
/// > Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// > This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ScanBlocks {
    Null(()),
    Object(ScanBlocksVariant1),
    Object2(ScanBlocksVariant2),
    Bool(bool),
}

/// When action=='start'; only returns after scan completes
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlocksVariant1 {
    /// true if the scan process was not aborted
    pub completed: bool,
    /// The height we started the scan from
    pub from_height: i64,
    /// Blocks that may have matched a scanobject.
    pub relevant_blocks: Vec<String>,
    /// The height we ended the scan at
    pub to_height: i64,
}

/// when action=='status' and a scan is currently in progress
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanBlocksVariant2 {
    /// Height of the block currently being scanned
    pub current_height: i64,
    /// Approximate percent complete
    pub progress: i64,
}

/// Result of the JSON-RPC method `scantxoutset`.
///
/// > scantxoutset
/// >
/// > Scans the unspent transaction output set for entries that match certain output descriptors.
/// > Examples of output descriptors are:
/// >     addr(\<address\>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
/// >     raw(\<hex script\>)                    Outputs whose output script equals the specified hex-encoded bytes
/// >     combo(\<pubkey\>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
/// >     pkh(\<pubkey\>)                        P2PKH outputs for the given pubkey
/// >     sh(multi(\<n\>,\<pubkey\>,\<pubkey\>,...)) P2SH-multisig outputs for the given threshold and pubkeys
/// >     tr(\<pubkey\>)                         P2TR
/// >     tr(\<pubkey\>,{pk(\<pubkey\>)})          P2TR with single fallback pubkey in tapscript
/// >     rawtr(\<pubkey\>)                      P2TR with the specified key as output key rather than inner
/// >     wsh(and_v(v:pk(\<pubkey\>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
/// >
/// > In the above, \<pubkey\> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// > or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
/// > unhardened or hardened child keys.
/// > In the latter case, a range needs to be specified by below if different from 1000.
/// > For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ScanTxOutSet {
    Object(ScanTxOutSetVariant0),
    Bool(bool),
    Object2(ScanTxOutSetVariant2),
    Null(()),
}

/// when action=='start'; only returns after scan completes
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxOutSetVariant0 {
    /// The hash of the block at the tip of the chain
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// The block height at which the scan was done
    pub height: i64,
    /// Whether the scan was completed
    pub success: bool,
    /// The total amount of all found unspent outputs in BTC
    pub total_amount: f64,
    /// The number of unspent transaction outputs scanned
    #[serde(rename = "txouts")]
    pub tx_outs: i64,
    pub unspents: Vec<ScanTxOutSetVariant0UnspentsItem>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxOutSetVariant0UnspentsItem {
    /// The total amount in BTC of the unspent output
    pub amount: f64,
    /// Blockhash of the unspent transaction output
    #[serde(rename = "blockhash")]
    pub block_hash: String,
    /// Whether this is a coinbase output
    pub coinbase: bool,
    /// Number of confirmations of the unspent transaction output when the scan was done
    pub confirmations: i64,
    /// A specialized descriptor for the matched output script
    pub desc: String,
    /// Height of the unspent transaction output
    pub height: i64,
    /// The output script
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: String,
    /// The transaction id
    pub txid: String,
    /// The vout value
    pub vout: i64,
}

/// when action=='status' and a scan is currently in progress
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScanTxOutSetVariant2 {
    /// Approximate percent complete
    pub progress: i64,
}

/// Result of the JSON-RPC method `verifychain`.
///
/// > verifychain
/// >
/// > Verifies blockchain database.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyChain(pub bool);

impl std::ops::Deref for VerifyChain {
    type Target = bool;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// Result of the JSON-RPC method `verifytxoutproof`.
///
/// > verifytxoutproof
/// >
/// > Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// > and throwing an RPC error if the block is not in our best chain
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct VerifyTxOutProof(pub Vec<String>);

/// Waits for a specific new block and returns useful info about it.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForBlock {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: i64,
}

/// Waits for (at least) block height and returns the height and hash
/// of the current tip.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForBlockHeight {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: i64,
}

/// Waits for any new block and returns useful info about it.
///
/// Returns the current block on timeout or exit.
///
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct WaitForNewBlock {
    /// The blockhash
    pub hash: String,
    /// Block height
    pub height: i64,
}
