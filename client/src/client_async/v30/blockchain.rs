// SPDX-License-Identifier: CC0-1.0

//! Auto-generated method wrappers for Bitcoin Core `30` - blockchain.
//!
//! Produced by `codegen`. Do not edit by hand, re-run
//! `just codegen` to regenerate. Defines the `*Options` request structs these methods
//! consume; the response types live in the `corepc-types` crate
//! (`types::v30::generated`).

#![allow(unused_imports, clippy::needless_pass_by_value, clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use serde_json::json;
use types::v30::generated::{
    DumpTxOutSet, GetBestBlockHash, GetBlockCount, GetBlockFilter, GetBlockFromPeer, GetBlockHash,
    GetBlockHeaderVerbose0, GetBlockHeaderVerbose1, GetBlockStats, GetBlockVerbose0,
    GetBlockVerbose1, GetBlockVerbose2, GetBlockVerbose3, GetBlockchainInfo, GetChainStates,
    GetChainTips, GetChainTxStats, GetDeploymentInfo, GetDescriptorActivity, GetDifficulty,
    GetMempoolAncestorsVerbose0, GetMempoolAncestorsVerbose1, GetMempoolDescendantsVerbose0,
    GetMempoolDescendantsVerbose1, GetMempoolEntry, GetMempoolInfo, GetRawMempool, GetTxOut,
    GetTxOutProof, GetTxOutSetInfo, GetTxSpendingPrevout, ImportMempool, LoadTxOutSet,
    PruneBlockchain, SaveMempool, ScanBlocks, ScanTxOutSet, VerifyChain, VerifyTxOutProof,
    WaitForBlock, WaitForBlockHeight, WaitForNewBlock,
};

use crate::client_async::error::Result;
use crate::client_async::Client;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DumpTxOutSetOptionsArg {
    /// Height or hash of the block to roll back to before creating the snapshot. Note: The further this number is from the tip, the longer this process will take. Consider setting a higher -rpcclienttimeout value in this case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GetDescriptorActivityScanObjects {
    Text(String),
    Object(GetDescriptorActivityScanObjectsVariant1),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetDescriptorActivityScanObjectsVariant1 {
    /// An output descriptor
    pub desc: String,
    /// The range of HD chain indexes to explore (either end or \[begin,end\])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<GetDescriptorActivityScanObjectsVariant1Range>,
}

/// The range of HD chain indexes to explore (either end or \[begin,end\])
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GetDescriptorActivityScanObjectsVariant1Range {
    Number(f64),
    List(Vec<serde_json::Value>),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTxSpendingPrevoutOutputs {
    /// The transaction id
    pub txid: String,
    /// The output number
    pub vout: i64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanBlocksOptionsArg {
    /// Filter false positives (slower and may fail on pruned nodes). Otherwise they may occur at a rate of 1/M
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_false_positives: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ScanBlocksScanObjects {
    Text(String),
    Object(ScanBlocksScanObjectsVariant1),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanBlocksScanObjectsVariant1 {
    /// An output descriptor
    pub desc: String,
    /// The range of HD chain indexes to explore (either end or \[begin,end\])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<ScanBlocksScanObjectsVariant1Range>,
}

/// The range of HD chain indexes to explore (either end or \[begin,end\])
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ScanBlocksScanObjectsVariant1Range {
    Number(f64),
    List(Vec<serde_json::Value>),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ScanTxOutSetScanObjects {
    Text(String),
    Object(ScanTxOutSetScanObjectsVariant1),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScanTxOutSetScanObjectsVariant1 {
    /// An output descriptor
    pub desc: String,
    /// The range of HD chain indexes to explore (either end or \[begin,end\])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<ScanTxOutSetScanObjectsVariant1Range>,
}

/// The range of HD chain indexes to explore (either end or \[begin,end\])
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ScanTxOutSetScanObjectsVariant1Range {
    Number(f64),
    List(Vec<serde_json::Value>),
}

/// Optional parameters for the `dumptxoutset` JSON-RPC method (consumed by `Client::dump_tx_out_set_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DumpTxOutSetOptions {
    /// The type of snapshot to create. Can be "latest" to create a snapshot of the current UTXO set or "rollback" to temporarily roll back the state of the node to a historical block before creating the snapshot of a historical UTXO set. This parameter can be omitted if a separate "rollback" named parameter is specified indicating the height or hash of a specific historical block. If "rollback" is specified and separate "rollback" named parameter is not specified, this will roll back to the latest valid snapshot block that can currently be loaded with loadtxoutset.
    pub type_: Option<String>,
    pub options: Option<DumpTxOutSetOptionsArg>,
}

/// Optional parameters for the `getblockfilter` JSON-RPC method (consumed by `Client::get_block_filter_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockFilterOptions {
    /// The type name of the filter
    pub filter_type: Option<String>,
}

/// Optional parameters for the `getblockstats` JSON-RPC method (consumed by `Client::get_block_stats_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockStatsOptions {
    /// Values to plot (see result below)
    pub stats: Option<Vec<String>>,
}

/// Optional parameters for the `getchaintxstats` JSON-RPC method (consumed by `Client::get_chain_tx_stats_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChainTxStatsOptions {
    /// Size of the window in number of blocks
    pub n_blocks: Option<i64>,
    /// The hash of the block that ends the window.
    pub block_hash: Option<String>,
}

/// Optional parameters for the `getdeploymentinfo` JSON-RPC method (consumed by `Client::get_deployment_info_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeploymentInfoOptions {
    /// The block hash at which to query deployment state
    pub block_hash: Option<String>,
}

/// Optional parameters for the `getdescriptoractivity` JSON-RPC method (consumed by `Client::get_descriptor_activity_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDescriptorActivityOptions {
    /// Whether to include unconfirmed activity
    pub include_mempool: Option<bool>,
}

/// Optional parameters for the `getrawmempool` JSON-RPC method (consumed by `Client::get_raw_mempool_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRawMempoolOptions {
    /// True for a json object, false for array of transaction ids
    pub verbose: Option<bool>,
    /// If verbose=false, returns a json object with transaction list and mempool sequence number attached.
    pub mempool_sequence: Option<bool>,
}

/// Optional parameters for the `gettxout` JSON-RPC method (consumed by `Client::get_tx_out_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTxOutOptions {
    /// Whether to include the mempool. Note that an unspent output that is spent in the mempool won't appear.
    pub include_mempool: Option<bool>,
}

/// Optional parameters for the `gettxoutproof` JSON-RPC method (consumed by `Client::get_tx_out_proof_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTxOutProofOptions {
    /// If specified, looks for txid in the block with this hash
    pub block_hash: Option<String>,
}

/// Optional parameters for the `gettxoutsetinfo` JSON-RPC method (consumed by `Client::get_tx_out_set_info_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTxOutSetInfoOptions {
    /// Which UTXO set hash should be calculated. Options: 'hash_serialized_3' (the legacy algorithm), 'muhash', 'none'.
    pub hash_type: Option<String>,
    /// The block hash or height of the target height (only available with coinstatsindex).
    pub hash_or_height: Option<f64>,
    /// Use coinstatsindex, if available.
    pub use_index: Option<bool>,
}

/// Optional parameters for the `importmempool` JSON-RPC method (consumed by `Client::import_mempool_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ImportMempoolOptions {
    /// Whether to apply the fee delta metadata from the mempool file.
    /// It will be added to any existing fee deltas.
    /// The fee delta can be set by the prioritisetransaction RPC.
    /// Warning: Importing untrusted metadata may lead to unexpected issues and undesirable behavior.
    /// Only set this bool if you understand what it does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_fee_delta_priority: Option<bool>,
    /// Whether to apply the unbroadcast set metadata from the mempool file.
    /// Warning: Importing untrusted metadata may lead to unexpected issues and undesirable behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_unbroadcast_set: Option<bool>,
    /// Whether to use the current system time or use the entry time metadata from the mempool file.
    /// Warning: Importing untrusted metadata may lead to unexpected issues and undesirable behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_current_time: Option<bool>,
}

/// Optional parameters for the `scanblocks` JSON-RPC method (consumed by `Client::scan_blocks_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanBlocksOptions {
    /// Array of scan objects. Required for "start" action
    /// Every scan object is either a string descriptor or an object:
    pub scan_objects: Option<Vec<ScanBlocksScanObjects>>,
    /// Height to start to scan from
    pub start_height: Option<i64>,
    /// Height to stop to scan
    pub stop_height: Option<i64>,
    /// The type name of the filter
    pub filter_type: Option<String>,
    pub options: Option<ScanBlocksOptionsArg>,
}

/// Optional parameters for the `scantxoutset` JSON-RPC method (consumed by `Client::scan_tx_out_set_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanTxOutSetOptions {
    /// Array of scan objects. Required for "start" action
    /// Every scan object is either a string descriptor or an object:
    pub scan_objects: Option<Vec<ScanTxOutSetScanObjects>>,
}

/// Optional parameters for the `verifychain` JSON-RPC method (consumed by `Client::verify_chain_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyChainOptions {
    /// How thorough the block verification is:
    /// - level 0 reads the blocks from disk
    /// - level 1 verifies block validity
    /// - level 2 verifies undo data
    /// - level 3 checks disconnection of tip blocks
    /// - level 4 tries to reconnect the blocks
    /// - each level includes the checks of the previous levels
    pub checklevel: Option<f64>,
    /// The number of blocks to check.
    pub n_blocks: Option<i64>,
}

/// Optional parameters for the `waitforblock` JSON-RPC method (consumed by `Client::wait_for_block_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitForBlockOptions {
    /// Time in milliseconds to wait for a response. 0 indicates no timeout.
    pub time_out: Option<i64>,
}

/// Optional parameters for the `waitforblockheight` JSON-RPC method (consumed by `Client::wait_for_block_height_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitForBlockHeightOptions {
    /// Time in milliseconds to wait for a response. 0 indicates no timeout.
    pub time_out: Option<i64>,
}

/// Optional parameters for the `waitfornewblock` JSON-RPC method (consumed by `Client::wait_for_new_block_with`).
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitForNewBlockOptions {
    /// Time in milliseconds to wait for a response. 0 indicates no timeout.
    pub time_out: Option<i64>,
    /// Method waits for the chain tip to differ from this.
    pub current_tip: Option<String>,
}

impl Client {
    /// `dumptxoutset` with required arguments only.
    ///
    /// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
    ///
    /// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
    ///
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn dump_tx_out_set(&self, path: String) -> Result<DumpTxOutSet> {
        self.call_raw("dumptxoutset", &[json!(path)]).await
    }

    /// `dumptxoutset` with all optional arguments via [`DumpTxOutSetOptions`].
    ///
    /// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
    ///
    /// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
    ///
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn dump_tx_out_set_with(
        &self,
        path: String,
        opts: DumpTxOutSetOptions,
    ) -> Result<DumpTxOutSet> {
        self.call_raw("dumptxoutset", &[json!(path), json!(opts.type_), json!(opts.options)]).await
    }

    /// `getbestblockhash` with required arguments only.
    ///
    /// Returns the hash of the best (tip) block in the most-work fully-validated chain.
    pub async fn get_best_block_hash(&self) -> Result<GetBestBlockHash> {
        self.call_raw("getbestblockhash", &[(); 0] as &[()]).await
    }

    /// `getblock` with the result selected for verbosity `0`.
    ///
    /// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
    /// If verbosity is 1, returns an Object with information about block \<hash\>.
    /// If verbosity is 2, returns an Object with information about block \<hash\> and information about each transaction.
    /// If verbosity is 3, returns an Object with information about block \<hash\> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
    pub async fn get_block_verbose_0(&self, block_hash: String) -> Result<GetBlockVerbose0> {
        self.call_raw("getblock", &[json!(block_hash), json!(0)]).await
    }

    /// `getblock` with the result selected for verbosity `1`.
    ///
    /// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
    /// If verbosity is 1, returns an Object with information about block \<hash\>.
    /// If verbosity is 2, returns an Object with information about block \<hash\> and information about each transaction.
    /// If verbosity is 3, returns an Object with information about block \<hash\> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
    pub async fn get_block_verbose_1(&self, block_hash: String) -> Result<GetBlockVerbose1> {
        self.call_raw("getblock", &[json!(block_hash), json!(1)]).await
    }

    /// `getblock` with the result selected for verbosity `2`.
    ///
    /// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
    /// If verbosity is 1, returns an Object with information about block \<hash\>.
    /// If verbosity is 2, returns an Object with information about block \<hash\> and information about each transaction.
    /// If verbosity is 3, returns an Object with information about block \<hash\> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
    pub async fn get_block_verbose_2(&self, block_hash: String) -> Result<GetBlockVerbose2> {
        self.call_raw("getblock", &[json!(block_hash), json!(2)]).await
    }

    /// `getblock` with the result selected for verbosity `3`.
    ///
    /// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
    /// If verbosity is 1, returns an Object with information about block \<hash\>.
    /// If verbosity is 2, returns an Object with information about block \<hash\> and information about each transaction.
    /// If verbosity is 3, returns an Object with information about block \<hash\> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
    pub async fn get_block_verbose_3(&self, block_hash: String) -> Result<GetBlockVerbose3> {
        self.call_raw("getblock", &[json!(block_hash), json!(3)]).await
    }

    /// `getblockchaininfo` with required arguments only.
    ///
    /// Returns an object containing various state info regarding blockchain processing.
    pub async fn get_blockchain_info(&self) -> Result<GetBlockchainInfo> {
        self.call_raw("getblockchaininfo", &[(); 0] as &[()]).await
    }

    /// `getblockcount` with required arguments only.
    ///
    /// Returns the height of the most-work fully-validated chain.
    /// The genesis block has height 0.
    pub async fn get_block_count(&self) -> Result<GetBlockCount> {
        self.call_raw("getblockcount", &[(); 0] as &[()]).await
    }

    /// `getblockfilter` with required arguments only.
    ///
    /// Retrieve a BIP 157 content filter for a particular block.
    pub async fn get_block_filter(&self, block_hash: String) -> Result<GetBlockFilter> {
        self.call_raw("getblockfilter", &[json!(block_hash)]).await
    }

    /// `getblockfilter` with all optional arguments via [`GetBlockFilterOptions`].
    ///
    /// Retrieve a BIP 157 content filter for a particular block.
    pub async fn get_block_filter_with(
        &self,
        block_hash: String,
        opts: GetBlockFilterOptions,
    ) -> Result<GetBlockFilter> {
        self.call_raw("getblockfilter", &[json!(block_hash), json!(opts.filter_type)]).await
    }

    /// `getblockfrompeer` with required arguments only.
    ///
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
    pub async fn get_block_from_peer(
        &self,
        block_hash: String,
        peer_id: f64,
    ) -> Result<GetBlockFromPeer> {
        self.call_raw("getblockfrompeer", &[json!(block_hash), json!(peer_id)]).await
    }

    /// `getblockhash` with required arguments only.
    ///
    /// Returns hash of block in best-block-chain at height provided.
    pub async fn get_block_hash(&self, height: i64) -> Result<GetBlockHash> {
        self.call_raw("getblockhash", &[json!(height)]).await
    }

    /// `getblockheader` with the result selected for verbosity `true`.
    ///
    /// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
    /// If verbose is true, returns an Object with information about blockheader \<hash\>.
    pub async fn get_block_header_verbose_1(
        &self,
        block_hash: String,
    ) -> Result<GetBlockHeaderVerbose1> {
        self.call_raw("getblockheader", &[json!(block_hash), json!(true)]).await
    }

    /// `getblockheader` with the result selected for verbosity `false`.
    ///
    /// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
    /// If verbose is true, returns an Object with information about blockheader \<hash\>.
    pub async fn get_block_header_verbose_0(
        &self,
        block_hash: String,
    ) -> Result<GetBlockHeaderVerbose0> {
        self.call_raw("getblockheader", &[json!(block_hash), json!(false)]).await
    }

    /// `getblockstats` with required arguments only.
    ///
    /// Compute per block statistics for a given window. All amounts are in satoshis.
    /// It won't work for some heights with pruning.
    pub async fn get_block_stats(&self, hash_or_height: f64) -> Result<GetBlockStats> {
        self.call_raw("getblockstats", &[json!(hash_or_height)]).await
    }

    /// `getblockstats` with all optional arguments via [`GetBlockStatsOptions`].
    ///
    /// Compute per block statistics for a given window. All amounts are in satoshis.
    /// It won't work for some heights with pruning.
    pub async fn get_block_stats_with(
        &self,
        hash_or_height: f64,
        opts: GetBlockStatsOptions,
    ) -> Result<GetBlockStats> {
        self.call_raw("getblockstats", &[json!(hash_or_height), json!(opts.stats)]).await
    }

    /// `getchainstates` with required arguments only.
    ///
    /// Return information about chainstates.
    pub async fn get_chain_states(&self) -> Result<GetChainStates> {
        self.call_raw("getchainstates", &[(); 0] as &[()]).await
    }

    /// `getchaintips` with required arguments only.
    ///
    /// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
    pub async fn get_chain_tips(&self) -> Result<GetChainTips> {
        self.call_raw("getchaintips", &[(); 0] as &[()]).await
    }

    /// `getchaintxstats` with required arguments only.
    ///
    /// Compute statistics about the total number and rate of transactions in the chain.
    pub async fn get_chain_tx_stats(&self) -> Result<GetChainTxStats> {
        self.call_raw("getchaintxstats", &[(); 0] as &[()]).await
    }

    /// `getchaintxstats` with all optional arguments via [`GetChainTxStatsOptions`].
    ///
    /// Compute statistics about the total number and rate of transactions in the chain.
    pub async fn get_chain_tx_stats_with(
        &self,
        opts: GetChainTxStatsOptions,
    ) -> Result<GetChainTxStats> {
        self.call_raw("getchaintxstats", &[json!(opts.n_blocks), json!(opts.block_hash)]).await
    }

    /// `getdeploymentinfo` with required arguments only.
    ///
    /// Returns an object containing various state info regarding deployments of consensus changes.
    pub async fn get_deployment_info(&self) -> Result<GetDeploymentInfo> {
        self.call_raw("getdeploymentinfo", &[(); 0] as &[()]).await
    }

    /// `getdeploymentinfo` with all optional arguments via [`GetDeploymentInfoOptions`].
    ///
    /// Returns an object containing various state info regarding deployments of consensus changes.
    pub async fn get_deployment_info_with(
        &self,
        opts: GetDeploymentInfoOptions,
    ) -> Result<GetDeploymentInfo> {
        self.call_raw("getdeploymentinfo", &[json!(opts.block_hash)]).await
    }

    /// `getdescriptoractivity` with required arguments only.
    ///
    /// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
    /// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn get_descriptor_activity(
        &self,
        block_hashes: Vec<String>,
        scan_objects: Vec<GetDescriptorActivityScanObjects>,
    ) -> Result<GetDescriptorActivity> {
        self.call_raw("getdescriptoractivity", &[json!(block_hashes), json!(scan_objects)]).await
    }

    /// `getdescriptoractivity` with all optional arguments via [`GetDescriptorActivityOptions`].
    ///
    /// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
    /// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn get_descriptor_activity_with(
        &self,
        block_hashes: Vec<String>,
        scan_objects: Vec<GetDescriptorActivityScanObjects>,
        opts: GetDescriptorActivityOptions,
    ) -> Result<GetDescriptorActivity> {
        self.call_raw(
            "getdescriptoractivity",
            &[json!(block_hashes), json!(scan_objects), json!(opts.include_mempool)],
        )
        .await
    }

    /// `getdifficulty` with required arguments only.
    ///
    /// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
    pub async fn get_difficulty(&self) -> Result<GetDifficulty> {
        self.call_raw("getdifficulty", &[(); 0] as &[()]).await
    }

    /// `getmempoolancestors` with the result selected for verbosity `false`.
    ///
    /// If txid is in the mempool, returns all in-mempool ancestors.
    pub async fn get_mempool_ancestors_verbose_0(
        &self,
        txid: String,
    ) -> Result<GetMempoolAncestorsVerbose0> {
        self.call_raw("getmempoolancestors", &[json!(txid), json!(false)]).await
    }

    /// `getmempoolancestors` with the result selected for verbosity `true`.
    ///
    /// If txid is in the mempool, returns all in-mempool ancestors.
    pub async fn get_mempool_ancestors_verbose_1(
        &self,
        txid: String,
    ) -> Result<GetMempoolAncestorsVerbose1> {
        self.call_raw("getmempoolancestors", &[json!(txid), json!(true)]).await
    }

    /// `getmempooldescendants` with the result selected for verbosity `false`.
    ///
    /// If txid is in the mempool, returns all in-mempool descendants.
    pub async fn get_mempool_descendants_verbose_0(
        &self,
        txid: String,
    ) -> Result<GetMempoolDescendantsVerbose0> {
        self.call_raw("getmempooldescendants", &[json!(txid), json!(false)]).await
    }

    /// `getmempooldescendants` with the result selected for verbosity `true`.
    ///
    /// If txid is in the mempool, returns all in-mempool descendants.
    pub async fn get_mempool_descendants_verbose_1(
        &self,
        txid: String,
    ) -> Result<GetMempoolDescendantsVerbose1> {
        self.call_raw("getmempooldescendants", &[json!(txid), json!(true)]).await
    }

    /// `getmempoolentry` with required arguments only.
    ///
    /// Returns mempool data for given transaction
    pub async fn get_mempool_entry(&self, txid: String) -> Result<GetMempoolEntry> {
        self.call_raw("getmempoolentry", &[json!(txid)]).await
    }

    /// `getmempoolinfo` with required arguments only.
    ///
    /// Returns details on the active state of the TX memory pool.
    pub async fn get_mempool_info(&self) -> Result<GetMempoolInfo> {
        self.call_raw("getmempoolinfo", &[(); 0] as &[()]).await
    }

    /// `getrawmempool` with required arguments only.
    ///
    /// Returns all transaction ids in memory pool as a json array of string transaction ids.
    ///
    /// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
    pub async fn get_raw_mempool(&self) -> Result<GetRawMempool> {
        self.call_raw("getrawmempool", &[(); 0] as &[()]).await
    }

    /// `getrawmempool` with all optional arguments via [`GetRawMempoolOptions`].
    ///
    /// Returns all transaction ids in memory pool as a json array of string transaction ids.
    ///
    /// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
    pub async fn get_raw_mempool_with(&self, opts: GetRawMempoolOptions) -> Result<GetRawMempool> {
        self.call_raw("getrawmempool", &[json!(opts.verbose), json!(opts.mempool_sequence)]).await
    }

    /// `gettxout` with required arguments only.
    ///
    /// Returns details about an unspent transaction output.
    pub async fn get_tx_out(&self, txid: String, n: i64) -> Result<GetTxOut> {
        self.call_raw("gettxout", &[json!(txid), json!(n)]).await
    }

    /// `gettxout` with all optional arguments via [`GetTxOutOptions`].
    ///
    /// Returns details about an unspent transaction output.
    pub async fn get_tx_out_with(
        &self,
        txid: String,
        n: i64,
        opts: GetTxOutOptions,
    ) -> Result<GetTxOut> {
        self.call_raw("gettxout", &[json!(txid), json!(n), json!(opts.include_mempool)]).await
    }

    /// `gettxoutproof` with required arguments only.
    ///
    /// Returns a hex-encoded proof that "txid" was included in a block.
    ///
    /// NOTE: By default this function only works sometimes. This is when there is an
    /// unspent output in the utxo for this transaction. To make it always work,
    /// you need to maintain a transaction index, using the -txindex command line option or
    /// specify the block in which the transaction is included manually (by blockhash).
    pub async fn get_tx_out_proof(&self, txids: Vec<String>) -> Result<GetTxOutProof> {
        self.call_raw("gettxoutproof", &[json!(txids)]).await
    }

    /// `gettxoutproof` with all optional arguments via [`GetTxOutProofOptions`].
    ///
    /// Returns a hex-encoded proof that "txid" was included in a block.
    ///
    /// NOTE: By default this function only works sometimes. This is when there is an
    /// unspent output in the utxo for this transaction. To make it always work,
    /// you need to maintain a transaction index, using the -txindex command line option or
    /// specify the block in which the transaction is included manually (by blockhash).
    pub async fn get_tx_out_proof_with(
        &self,
        txids: Vec<String>,
        opts: GetTxOutProofOptions,
    ) -> Result<GetTxOutProof> {
        self.call_raw("gettxoutproof", &[json!(txids), json!(opts.block_hash)]).await
    }

    /// `gettxoutsetinfo` with required arguments only.
    ///
    /// Returns statistics about the unspent transaction output set.
    /// Note this call may take some time if you are not using coinstatsindex.
    pub async fn get_tx_out_set_info(&self) -> Result<GetTxOutSetInfo> {
        self.call_raw("gettxoutsetinfo", &[(); 0] as &[()]).await
    }

    /// `gettxoutsetinfo` with all optional arguments via [`GetTxOutSetInfoOptions`].
    ///
    /// Returns statistics about the unspent transaction output set.
    /// Note this call may take some time if you are not using coinstatsindex.
    pub async fn get_tx_out_set_info_with(
        &self,
        opts: GetTxOutSetInfoOptions,
    ) -> Result<GetTxOutSetInfo> {
        self.call_raw(
            "gettxoutsetinfo",
            &[json!(opts.hash_type), json!(opts.hash_or_height), json!(opts.use_index)],
        )
        .await
    }

    /// `gettxspendingprevout` with required arguments only.
    ///
    /// Scans the mempool to find transactions spending any of the given outputs
    pub async fn get_tx_spending_prevout(
        &self,
        outputs: Vec<GetTxSpendingPrevoutOutputs>,
    ) -> Result<GetTxSpendingPrevout> {
        self.call_raw("gettxspendingprevout", &[json!(outputs)]).await
    }

    /// `importmempool` with required arguments only.
    ///
    /// Import a mempool.dat file and attempt to add its contents to the mempool.
    /// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
    pub async fn import_mempool(&self, file_path: String) -> Result<ImportMempool> {
        self.call_raw("importmempool", &[json!(file_path)]).await
    }

    /// `importmempool` with all optional arguments via [`ImportMempoolOptions`].
    ///
    /// Import a mempool.dat file and attempt to add its contents to the mempool.
    /// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
    pub async fn import_mempool_with(
        &self,
        file_path: String,
        opts: ImportMempoolOptions,
    ) -> Result<ImportMempool> {
        self.call_raw("importmempool", &[json!(file_path), json!(opts)]).await
    }

    /// `loadtxoutset` with required arguments only.
    ///
    /// Load the serialized UTXO set from a file.
    /// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
    ///
    /// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
    ///
    /// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
    pub async fn load_tx_out_set(&self, path: String) -> Result<LoadTxOutSet> {
        self.call_raw("loadtxoutset", &[json!(path)]).await
    }

    /// `preciousblock` with required arguments only.
    ///
    /// Treats a block as if it were received before others with the same work.
    ///
    /// A later preciousblock call can override the effect of an earlier one.
    ///
    /// The effects of preciousblock are not retained across restarts.
    pub async fn precious_block(&self, block_hash: String) -> Result<()> {
        self.call_raw("preciousblock", &[json!(block_hash)]).await
    }

    /// `pruneblockchain` with required arguments only.
    ///
    /// Attempts to delete block and undo data up to a specified height or timestamp, if eligible for pruning.
    /// Requires `-prune` to be enabled at startup. While pruned data may be re-fetched in some cases (e.g., via `getblockfrompeer`), local deletion is irreversible.
    pub async fn prune_blockchain(&self, height: i64) -> Result<PruneBlockchain> {
        self.call_raw("pruneblockchain", &[json!(height)]).await
    }

    /// `savemempool` with required arguments only.
    ///
    /// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
    pub async fn save_mempool(&self) -> Result<SaveMempool> {
        self.call_raw("savemempool", &[(); 0] as &[()]).await
    }

    /// `scanblocks` with required arguments only.
    ///
    /// Return relevant blockhashes for given descriptors (requires blockfilterindex).
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn scan_blocks(&self, action: String) -> Result<ScanBlocks> {
        self.call_raw("scanblocks", &[json!(action)]).await
    }

    /// `scanblocks` with all optional arguments via [`ScanBlocksOptions`].
    ///
    /// Return relevant blockhashes for given descriptors (requires blockfilterindex).
    /// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn scan_blocks_with(
        &self,
        action: String,
        opts: ScanBlocksOptions,
    ) -> Result<ScanBlocks> {
        self.call_raw(
            "scanblocks",
            &[
                json!(action),
                json!(opts.scan_objects),
                json!(opts.start_height),
                json!(opts.stop_height),
                json!(opts.filter_type),
                json!(opts.options),
            ],
        )
        .await
    }

    /// `scantxoutset` with required arguments only.
    ///
    /// Scans the unspent transaction output set for entries that match certain output descriptors.
    /// Examples of output descriptors are:
    ///     addr(\<address\>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
    ///     raw(\<hex script\>)                    Outputs whose output script equals the specified hex-encoded bytes
    ///     combo(\<pubkey\>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
    ///     pkh(\<pubkey\>)                        P2PKH outputs for the given pubkey
    ///     sh(multi(\<n\>,\<pubkey\>,\<pubkey\>,...)) P2SH-multisig outputs for the given threshold and pubkeys
    ///     tr(\<pubkey\>)                         P2TR
    ///     tr(\<pubkey\>,{pk(\<pubkey\>)})          P2TR with single fallback pubkey in tapscript
    ///     rawtr(\<pubkey\>)                      P2TR with the specified key as output key rather than inner
    ///     wsh(and_v(v:pk(\<pubkey\>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
    ///
    /// In the above, \<pubkey\> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
    /// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
    /// unhardened or hardened child keys.
    /// In the latter case, a range needs to be specified by below if different from 1000.
    /// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
    pub async fn scan_tx_out_set(&self, action: String) -> Result<ScanTxOutSet> {
        self.call_raw("scantxoutset", &[json!(action)]).await
    }

    /// `scantxoutset` with all optional arguments via [`ScanTxOutSetOptions`].
    ///
    /// Scans the unspent transaction output set for entries that match certain output descriptors.
    /// Examples of output descriptors are:
    ///     addr(\<address\>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
    ///     raw(\<hex script\>)                    Outputs whose output script equals the specified hex-encoded bytes
    ///     combo(\<pubkey\>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
    ///     pkh(\<pubkey\>)                        P2PKH outputs for the given pubkey
    ///     sh(multi(\<n\>,\<pubkey\>,\<pubkey\>,...)) P2SH-multisig outputs for the given threshold and pubkeys
    ///     tr(\<pubkey\>)                         P2TR
    ///     tr(\<pubkey\>,{pk(\<pubkey\>)})          P2TR with single fallback pubkey in tapscript
    ///     rawtr(\<pubkey\>)                      P2TR with the specified key as output key rather than inner
    ///     wsh(and_v(v:pk(\<pubkey\>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
    ///
    /// In the above, \<pubkey\> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
    /// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
    /// unhardened or hardened child keys.
    /// In the latter case, a range needs to be specified by below if different from 1000.
    /// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
    pub async fn scan_tx_out_set_with(
        &self,
        action: String,
        opts: ScanTxOutSetOptions,
    ) -> Result<ScanTxOutSet> {
        self.call_raw("scantxoutset", &[json!(action), json!(opts.scan_objects)]).await
    }

    /// `verifychain` with required arguments only.
    ///
    /// Verifies blockchain database.
    pub async fn verify_chain(&self) -> Result<VerifyChain> {
        self.call_raw("verifychain", &[(); 0] as &[()]).await
    }

    /// `verifychain` with all optional arguments via [`VerifyChainOptions`].
    ///
    /// Verifies blockchain database.
    pub async fn verify_chain_with(&self, opts: VerifyChainOptions) -> Result<VerifyChain> {
        self.call_raw("verifychain", &[json!(opts.checklevel), json!(opts.n_blocks)]).await
    }

    /// `verifytxoutproof` with required arguments only.
    ///
    /// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
    /// and throwing an RPC error if the block is not in our best chain
    pub async fn verify_tx_out_proof(&self, proof: String) -> Result<VerifyTxOutProof> {
        self.call_raw("verifytxoutproof", &[json!(proof)]).await
    }

    /// `waitforblock` with required arguments only.
    ///
    /// Waits for a specific new block and returns useful info about it.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn wait_for_block(&self, block_hash: String) -> Result<WaitForBlock> {
        self.call_raw("waitforblock", &[json!(block_hash)]).await
    }

    /// `waitforblock` with all optional arguments via [`WaitForBlockOptions`].
    ///
    /// Waits for a specific new block and returns useful info about it.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn wait_for_block_with(
        &self,
        block_hash: String,
        opts: WaitForBlockOptions,
    ) -> Result<WaitForBlock> {
        self.call_raw("waitforblock", &[json!(block_hash), json!(opts.time_out)]).await
    }

    /// `waitforblockheight` with required arguments only.
    ///
    /// Waits for (at least) block height and returns the height and hash
    /// of the current tip.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn wait_for_block_height(&self, height: i64) -> Result<WaitForBlockHeight> {
        self.call_raw("waitforblockheight", &[json!(height)]).await
    }

    /// `waitforblockheight` with all optional arguments via [`WaitForBlockHeightOptions`].
    ///
    /// Waits for (at least) block height and returns the height and hash
    /// of the current tip.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn wait_for_block_height_with(
        &self,
        height: i64,
        opts: WaitForBlockHeightOptions,
    ) -> Result<WaitForBlockHeight> {
        self.call_raw("waitforblockheight", &[json!(height), json!(opts.time_out)]).await
    }

    /// `waitfornewblock` with required arguments only.
    ///
    /// Waits for any new block and returns useful info about it.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn wait_for_new_block(&self) -> Result<WaitForNewBlock> {
        self.call_raw("waitfornewblock", &[(); 0] as &[()]).await
    }

    /// `waitfornewblock` with all optional arguments via [`WaitForNewBlockOptions`].
    ///
    /// Waits for any new block and returns useful info about it.
    ///
    /// Returns the current block on timeout or exit.
    ///
    /// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
    pub async fn wait_for_new_block_with(
        &self,
        opts: WaitForNewBlockOptions,
    ) -> Result<WaitForNewBlock> {
        self.call_raw("waitfornewblock", &[json!(opts.time_out), json!(opts.current_tip)]).await
    }
}
