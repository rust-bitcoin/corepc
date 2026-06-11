// SPDX-License-Identifier: CC0-1.0

//! RPC methods for the async Bitcoin Core client.
//! All functions return the version nonspecific, strongly typed types.

use bitcoin::{block, Block, BlockHash, Transaction, Txid};
use serde_json::value::RawValue;

use super::{into_json, Client, IntoModelError, Result};
use crate::types::model::{GetBlockFilter, GetBlockHeaderVerbose, GetBlockVerboseOne};

/// Bitcoin Core RPC methods (v25 to v30).
///
/// This trait exposes the Bitcoin Core RPC methods available on [`Client`]. Downstream users
/// can define their own extension traits with additional methods without risk of
/// name collision with the methods defined here.
// `async fn` in traits produces non-`Send` futures by default; we suppress this lint because
// `BitcoinRpcs` is intended only for use with concrete types (never `dyn BitcoinRpcs`).
#[allow(async_fn_in_trait)]
pub trait BitcoinRpcs {
    /// Gets a block by blockhash.
    async fn get_block(&self, hash: &BlockHash) -> Result<Block>;

    /// Gets the block count.
    async fn get_block_count(&self) -> Result<u64>;

    /// Gets the block hash for a height.
    async fn get_block_hash(&self, height: u32) -> Result<BlockHash>;

    /// Gets the hash of the chain tip.
    async fn get_best_block_hash(&self) -> Result<BlockHash>;

    /// Gets the block header by blockhash.
    async fn get_block_header(&self, hash: &BlockHash) -> Result<block::Header>;

    /// Gets the block header with verbose output.
    async fn get_block_header_verbose(&self, hash: &BlockHash) -> Result<GetBlockHeaderVerbose>;

    /// Gets a block by blockhash with verbose set to 1.
    async fn get_block_verbose(&self, hash: &BlockHash) -> Result<GetBlockVerboseOne>;

    /// Gets the block filter for a blockhash.
    async fn get_block_filter(&self, hash: &BlockHash) -> Result<GetBlockFilter>;

    /// Gets the transaction IDs currently in the mempool.
    async fn get_raw_mempool(&self) -> Result<Vec<Txid>>;

    /// Gets the raw transaction by txid.
    async fn get_raw_transaction(&self, txid: &Txid) -> Result<Transaction>;

    /// Returns the version integer reported by the server (e.g. `250200` for v25.2.0).
    async fn server_version(&self) -> Result<usize>;
}

impl BitcoinRpcs for Client {
    async fn get_block(&self, hash: &BlockHash) -> Result<Block> {
        let json: crate::types::v25::GetBlockVerboseZero =
            self.call("getblock", &[into_json(hash)?, into_json(0)?]).await?;
        Ok(json.into_model().map_err(|e| IntoModelError::new("`getblock`", e))?.0)
    }

    async fn get_block_count(&self) -> Result<u64> {
        let json: crate::types::v25::GetBlockCount = self.call("getblockcount", &[]).await?;
        Ok(json.into_model().0)
    }

    async fn get_block_hash(&self, height: u32) -> Result<BlockHash> {
        let json: crate::types::v25::GetBlockHash =
            self.call("getblockhash", &[into_json(height)?]).await?;
        Ok(json.into_model().map_err(|e| IntoModelError::new("`getblockhash`", e))?.0)
    }

    async fn get_best_block_hash(&self) -> Result<BlockHash> {
        let json: crate::types::v25::GetBestBlockHash = self.call("getbestblockhash", &[]).await?;
        Ok(json.into_model().map_err(|e| IntoModelError::new("`getbestblockhash`", e))?.0)
    }

    async fn get_block_header(&self, hash: &BlockHash) -> Result<block::Header> {
        let json: crate::types::v25::GetBlockHeader =
            self.call("getblockheader", &[into_json(hash)?, into_json(false)?]).await?;
        Ok(json.into_model().map_err(|e| IntoModelError::new("`getblockheader`", e))?.0)
    }

    async fn get_block_header_verbose(&self, hash: &BlockHash) -> Result<GetBlockHeaderVerbose> {
        let raw: Box<RawValue> =
            self.call("getblockheader", &[into_json(hash)?, into_json(true)?]).await?;

        if let Ok(json) =
            serde_json::from_str::<crate::types::v29::GetBlockHeaderVerbose>(raw.get())
        {
            Ok(json.into_model().map_err(|e| IntoModelError::new("`getblockheader` verbose", e))?)
        } else {
            let json: crate::types::v25::GetBlockHeaderVerbose = serde_json::from_str(raw.get())?;
            Ok(json.into_model().map_err(|e| IntoModelError::new("`getblockheader` verbose", e))?)
        }
    }

    async fn get_block_verbose(&self, hash: &BlockHash) -> Result<GetBlockVerboseOne> {
        let raw: Box<RawValue> = self.call("getblock", &[into_json(hash)?, into_json(1)?]).await?;

        if let Ok(json) = serde_json::from_str::<crate::types::v29::GetBlockVerboseOne>(raw.get()) {
            Ok(json.into_model().map_err(|e| IntoModelError::new("`getblock` verbose=1", e))?)
        } else {
            let json: crate::types::v25::GetBlockVerboseOne = serde_json::from_str(raw.get())?;
            Ok(json.into_model().map_err(|e| IntoModelError::new("`getblock` verbose=1", e))?)
        }
    }

    async fn get_block_filter(&self, hash: &BlockHash) -> Result<GetBlockFilter> {
        let json: crate::types::v25::GetBlockFilter =
            self.call("getblockfilter", &[into_json(hash)?]).await?;
        Ok(json.into_model().map_err(|e| IntoModelError::new("`getblockfilter`", e))?)
    }

    async fn get_raw_mempool(&self) -> Result<Vec<Txid>> {
        let json: crate::types::v25::GetRawMempool = self.call("getrawmempool", &[]).await?;
        Ok(json.into_model().map_err(|e| IntoModelError::new("`getrawmempool`", e))?.0)
    }

    async fn get_raw_transaction(&self, txid: &Txid) -> Result<Transaction> {
        let json: crate::types::v25::GetRawTransaction =
            self.call("getrawtransaction", &[into_json(txid)?]).await?;
        Ok(json.into_model().map_err(|e| IntoModelError::new("`getrawtransaction`", e))?.0)
    }

    async fn server_version(&self) -> Result<usize> {
        // Use a minimal type to read only the `version` field; the shape of other fields
        // (e.g. `warnings` changed from String to Vec<String> at v28) differs across the
        // supported version range.
        #[derive(serde::Deserialize)]
        struct NetworkVersion {
            version: usize,
        }
        let json: NetworkVersion = self.call("getnetworkinfo", &[]).await?;
        Ok(json.version)
    }
}
