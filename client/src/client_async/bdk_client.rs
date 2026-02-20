// SPDX-License-Identifier: CC0-1.0

//! Async JSON-RPC client with the RPC set used by BDK for Core versions 25 to 30.

use bitcoin::{BlockHash, Txid};

use crate::client_async::{into_json, Client, Error, Result};
use crate::types::model::{
    GetBestBlockHash, GetBlockCount, GetBlockFilter, GetBlockHash, GetBlockHeader,
    GetBlockHeaderVerbose, GetBlockVerboseOne, GetBlockVerboseZero, GetRawMempool,
    GetRawTransaction,
};

const VERSION_WITH_TARGET_FIELD: usize = 290000;

impl Client {
    async fn bdk_server_version(&self) -> Result<usize> {
        let info: serde_json::Value = self.call("getnetworkinfo", &[]).await?;
        let version = info
            .get("version")
            .and_then(serde_json::Value::as_u64)
            .ok_or(Error::UnexpectedStructure)?;
        usize::try_from(version).map_err(|_| Error::UnexpectedStructure)
    }

    /// Gets a block by blockhash.
    pub async fn get_block(&self, hash: &BlockHash) -> Result<GetBlockVerboseZero> {
        let json: crate::types::v25::GetBlockVerboseZero =
            self.call("getblock", &[into_json(hash)?, into_json(0)?]).await?;
        json.into_model().map_err(|e| Error::Returned(e.to_string()))
    }

    /// Gets block count.
    pub async fn get_block_count(&self) -> Result<GetBlockCount> {
        let json: crate::types::v25::GetBlockCount = self.call("getblockcount", &[]).await?;
        Ok(json.into_model())
    }

    /// Gets block hash for a height.
    pub async fn get_block_hash(&self, height: u32) -> Result<GetBlockHash> {
        let json: crate::types::v25::GetBlockHash =
            self.call("getblockhash", &[into_json(height)?]).await?;
        json.into_model().map_err(|e| Error::Returned(e.to_string()))
    }

    /// Gets the hash of the chain tip.
    pub async fn get_best_block_hash(&self) -> Result<GetBestBlockHash> {
        let json: crate::types::v25::GetBestBlockHash = self.call("getbestblockhash", &[]).await?;
        json.into_model().map_err(|e| Error::Returned(e.to_string()))
    }

    /// Gets block header by blockhash.
    pub async fn get_block_header(&self, hash: &BlockHash) -> Result<GetBlockHeader> {
        let json: crate::types::v25::GetBlockHeader =
            self.call("getblockheader", &[into_json(hash)?, into_json(false)?]).await?;
        json.into_model().map_err(|e| Error::Returned(e.to_string()))
    }

    /// Gets block header with verbose output.
    pub async fn get_block_header_verbose(
        &self,
        hash: &BlockHash,
    ) -> Result<GetBlockHeaderVerbose> {
        if self.bdk_server_version().await? >= VERSION_WITH_TARGET_FIELD {
            let json: crate::types::v29::GetBlockHeaderVerbose =
                self.call("getblockheader", &[into_json(hash)?]).await?;
            json.into_model().map_err(|e| Error::Returned(e.to_string()))
        } else {
            let json: crate::types::v25::GetBlockHeaderVerbose =
                self.call("getblockheader", &[into_json(hash)?]).await?;
            json.into_model().map_err(|e| Error::Returned(e.to_string()))
        }
    }

    /// Gets a block by blockhash with verbose set to 1.
    pub async fn get_block_verbose(&self, hash: &BlockHash) -> Result<GetBlockVerboseOne> {
        if self.bdk_server_version().await? >= VERSION_WITH_TARGET_FIELD {
            let json: crate::types::v29::GetBlockVerboseOne =
                self.call("getblock", &[into_json(hash)?, into_json(1)?]).await?;
            json.into_model().map_err(|e| Error::Returned(e.to_string()))
        } else {
            let json: crate::types::v25::GetBlockVerboseOne =
                self.call("getblock", &[into_json(hash)?, into_json(1)?]).await?;
            json.into_model().map_err(|e| Error::Returned(e.to_string()))
        }
    }

    /// Gets block filter for a blockhash.
    pub async fn get_block_filter(&self, hash: &BlockHash) -> Result<GetBlockFilter> {
        let json: crate::types::v25::GetBlockFilter =
            self.call("getblockfilter", &[into_json(hash)?]).await?;
        json.into_model().map_err(|e| Error::Returned(e.to_string()))
    }

    /// Gets transaction IDs currently in the mempool.
    pub async fn get_raw_mempool(&self) -> Result<GetRawMempool> {
        let json: crate::types::v25::GetRawMempool = self.call("getrawmempool", &[]).await?;
        json.into_model().map_err(|e| Error::Returned(e.to_string()))
    }

    /// Gets raw transaction by txid.
    pub async fn get_raw_transaction(&self, txid: &Txid) -> Result<GetRawTransaction> {
        let json: crate::types::v25::GetRawTransaction =
            self.call("getrawtransaction", &[into_json(txid)?]).await?;
        json.into_model().map_err(|e| Error::Returned(e.to_string()))
    }
}
