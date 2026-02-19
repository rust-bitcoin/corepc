// SPDX-License-Identifier: CC0-1.0

//! Async JSON-RPC client with the RPC set used by BDK for Core versions 25 to 30.

use bitcoin::{block, Block, BlockHash, Transaction, Txid};

use crate::client_async::{into_json, Client, Result};
use crate::types::model::{GetBlockFilter, GetBlockHeaderVerbose, GetBlockVerboseOne};

impl Client {
    /// Gets a block by blockhash.
    pub async fn get_block(&self, hash: &BlockHash) -> Result<Block> {
        let json: crate::types::v25::GetBlockVerboseZero =
            self.call("getblock", &[into_json(hash)?, into_json(0)?]).await?;
        Ok(json.into_model()?.0)
    }

    /// Gets block count.
    pub async fn get_block_count(&self) -> Result<u64> {
        let json: crate::types::v25::GetBlockCount = self.call("getblockcount", &[]).await?;
        Ok(json.into_model().0)
    }

    /// Gets block hash for a height.
    pub async fn get_block_hash(&self, height: u32) -> Result<BlockHash> {
        let json: crate::types::v25::GetBlockHash =
            self.call("getblockhash", &[into_json(height)?]).await?;
        Ok(json.into_model()?.0)
    }

    /// Gets the hash of the chain tip.
    pub async fn get_best_block_hash(&self) -> Result<BlockHash> {
        let json: crate::types::v25::GetBestBlockHash = self.call("getbestblockhash", &[]).await?;
        Ok(json.into_model()?.0)
    }

    /// Gets block header by blockhash.
    pub async fn get_block_header(&self, hash: &BlockHash) -> Result<block::Header> {
        let json: crate::types::v25::GetBlockHeader =
            self.call("getblockheader", &[into_json(hash)?, into_json(false)?]).await?;
        Ok(json.into_model()?.0)
    }

    /// Gets block header with verbose output.
    pub async fn get_block_header_verbose(
        &self,
        hash: &BlockHash,
    ) -> Result<GetBlockHeaderVerbose> {
        let response: serde_json::Value =
            self.call("getblockheader", &[into_json(hash)?, into_json(true)?]).await?;

        if let Ok(json) =
            serde_json::from_value::<crate::types::v29::GetBlockHeaderVerbose>(response.clone())
        {
            Ok(json.into_model()?)
        } else {
            let json: crate::types::v25::GetBlockHeaderVerbose = serde_json::from_value(response)?;
            Ok(json.into_model()?)
        }
    }

    /// Gets a block by blockhash with verbose set to 1.
    pub async fn get_block_verbose(&self, hash: &BlockHash) -> Result<GetBlockVerboseOne> {
        let response: serde_json::Value =
            self.call("getblock", &[into_json(hash)?, into_json(1)?]).await?;

        if let Ok(json) =
            serde_json::from_value::<crate::types::v29::GetBlockVerboseOne>(response.clone())
        {
            Ok(json.into_model()?)
        } else {
            let json: crate::types::v25::GetBlockVerboseOne = serde_json::from_value(response)?;
            Ok(json.into_model()?)
        }
    }

    /// Gets block filter for a blockhash.
    pub async fn get_block_filter(&self, hash: &BlockHash) -> Result<GetBlockFilter> {
        let json: crate::types::v25::GetBlockFilter =
            self.call("getblockfilter", &[into_json(hash)?]).await?;
        Ok(json.into_model()?)
    }

    /// Gets transaction IDs currently in the mempool.
    pub async fn get_raw_mempool(&self) -> Result<Vec<Txid>> {
        let json: crate::types::v25::GetRawMempool = self.call("getrawmempool", &[]).await?;
        Ok(json.into_model()?.0)
    }

    /// Gets raw transaction by txid.
    pub async fn get_raw_transaction(&self, txid: &Txid) -> Result<Transaction> {
        let json: crate::types::v25::GetRawTransaction =
            self.call("getrawtransaction", &[into_json(txid)?]).await?;
        Ok(json.into_model()?.0)
    }
}
