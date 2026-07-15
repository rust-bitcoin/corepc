// SPDX-License-Identifier: CC0-1.0

//! RPC set used by BDK.
//!
//! All functions return the version nonspecific, strongly typed types.
//!
//! Some methods return a different JSON shape depending on the Core version (e.g. `getblock` at
//! verbosity 1 gained a required `target` field in v29 and `coinbase_tx` in v31). These methods fetch
//! the raw JSON and try to deserialize it into each version's type, newest first, falling back to
//! older versions.

use bitcoin::{block, Block, BlockHash, Transaction, Txid};
use serde_json::value::RawValue;

use crate::client_async::error::{
    GetBestBlockHashError, GetBlockCountError, GetBlockError, GetBlockFilterError,
    GetBlockHashError, GetBlockHeaderError, GetBlockHeaderVerboseError, GetBlockVerboseError,
    GetBlockchainInfoError, GetRawMempoolError, GetRawTransactionError, GetTxOutError,
    ServerVersionError,
};
use crate::client_async::Client;
use crate::types::{self, model};
use crate::into_json;

impl Client {
    /// Gets a block by blockhash.
    pub async fn get_block(&self, hash: &BlockHash) -> Result<Block, GetBlockError> {
        // This type hasnt' changed between Core v17 and v31.
        let json: types::v25::GetBlockVerboseZero =
            self.call("getblock", &[into_json(hash)?, into_json(0)?]).await?;
        Ok(json.into_model().map_err(GetBlockError::Model)?.0)
    }

    /// Gets the block count.
    pub async fn get_block_count(&self) -> Result<u64, GetBlockCountError> {
        // This type hasnt' changed between Core v17 and v31.
        let json: types::v25::GetBlockCount = self.call("getblockcount", &[]).await?;
        Ok(json.into_model().0)
    }

    /// Gets the block hash for a height.
    pub async fn get_block_hash(
        &self,
        height: u32,
    ) -> Result<BlockHash, GetBlockHashError> {
        // This type hasnt' changed between Core v17 and v31.
        let json: types::v25::GetBlockHash =
            self.call("getblockhash", &[into_json(height)?]).await?;
        Ok(json.into_model().map_err(GetBlockHashError::Model)?.0)
    }

    /// Gets the hash of the chain tip.
    pub async fn get_best_block_hash(
        &self,
    ) -> Result<BlockHash, GetBestBlockHashError> {
        // This type hasnt' changed between Core v17 and v31.
        let json: types::v25::GetBestBlockHash = self.call("getbestblockhash", &[]).await?;
        Ok(json.into_model().map_err(GetBestBlockHashError::Model)?.0)
    }

    /// Gets the block header by blockhash.
    pub async fn get_block_header(
        &self,
        hash: &BlockHash,
    ) -> Result<block::Header, GetBlockHeaderError> {
        // This type hasnt' changed between Core v17 and v31.
        let json: types::v25::GetBlockHeader =
            self.call("getblockheader", &[into_json(hash)?, into_json(false)?]).await?;
        Ok(json.into_model().map_err(GetBlockHeaderError::Model)?.0)
    }

    /// Gets the block header with verbose output.
    ///
    /// The version specific type is detected from the response shape.
    pub async fn get_block_header_verbose(
        &self,
        hash: &BlockHash,
    ) -> Result<model::GetBlockHeaderVerbose, GetBlockHeaderVerboseError> {
        let raw: Box<RawValue> =
            self.call("getblockheader", &[into_json(hash)?, into_json(true)?]).await?;
        if let Ok(json) =
            serde_json::from_str::<types::v29::GetBlockHeaderVerbose>(raw.get())
        {
            Ok(json.into_model().map_err(|e| GetBlockHeaderVerboseError::Model(Box::new(e)))?)
        } else {
            let json: types::v25::GetBlockHeaderVerbose = serde_json::from_str(raw.get())?;
            Ok(json.into_model().map_err(|e| GetBlockHeaderVerboseError::Model(Box::new(e)))?)
        }
    }

    /// Gets a block by blockhash with verbose set to 1.
    ///
    /// The version specific type is detected from the response shape.
    pub async fn get_block_verbose(
        &self,
        hash: &BlockHash,
    ) -> Result<model::GetBlockVerboseOne, GetBlockVerboseError> {
        let raw: Box<RawValue> = self.call("getblock", &[into_json(hash)?, into_json(1)?]).await?;
        if let Ok(json) = serde_json::from_str::<types::v31::GetBlockVerboseOne>(raw.get()) {
            Ok(json.into_model().map_err(|e| GetBlockVerboseError::Model(Box::new(e)))?)
        } else if let Ok(json) =
            serde_json::from_str::<types::v29::GetBlockVerboseOne>(raw.get())
        {
            Ok(json.into_model().map_err(|e| GetBlockVerboseError::Model(Box::new(e)))?)
        } else {
            let json: types::v25::GetBlockVerboseOne = serde_json::from_str(raw.get())?;
            Ok(json.into_model().map_err(|e| GetBlockVerboseError::Model(Box::new(e)))?)
        }
    }

    /// Gets the block filter for a blockhash.
    pub async fn get_block_filter(
        &self,
        hash: &BlockHash,
    ) -> Result<model::GetBlockFilter, GetBlockFilterError> {
        // This type hasnt' changed between Core v19 and v31.
        let json: types::v25::GetBlockFilter =
            self.call("getblockfilter", &[into_json(hash)?]).await?;
        json.into_model().map_err(GetBlockFilterError::Model)
    }

    /// Gets various state info regarding blockchain processing.
    ///
    /// The version specific type is detected from the response shape.
    pub async fn get_blockchain_info(
        &self,
    ) -> Result<model::GetBlockchainInfo, GetBlockchainInfoError> {
        let raw: Box<RawValue> = self.call("getblockchaininfo", &[]).await?;
        if let Ok(json) = serde_json::from_str::<types::v29::GetBlockchainInfo>(raw.get()) {
            Ok(json.into_model().map_err(|e| GetBlockchainInfoError::Model(Box::new(e)))?)
        } else if let Ok(json) =
            serde_json::from_str::<types::v28::GetBlockchainInfo>(raw.get())
        {
            Ok(json.into_model().map_err(|e| GetBlockchainInfoError::Model(Box::new(e)))?)
        } else {
            let json: types::v25::GetBlockchainInfo = serde_json::from_str(raw.get())?;
            Ok(json.into_model().map_err(|e| GetBlockchainInfoError::Model(Box::new(e)))?)
        }
    }

    /// Gets the transaction IDs currently in the mempool.
    pub async fn get_raw_mempool(&self) -> Result<Vec<Txid>, GetRawMempoolError> {
        // This type hasnt' changed between Core v23 and v31.
        let json: types::v25::GetRawMempool = self.call("getrawmempool", &[]).await?;
        Ok(json.into_model().map_err(GetRawMempoolError::Model)?.0)
    }

    /// Gets the raw transaction by txid.
    pub async fn get_raw_transaction(
        &self,
        txid: &Txid,
    ) -> Result<Transaction, GetRawTransactionError> {
        // This type hasnt' changed between Core v17 and v31.
        let json: types::v25::GetRawTransaction =
            self.call("getrawtransaction", &[into_json(txid)?]).await?;
        Ok(json.into_model().map_err(GetRawTransactionError::Model)?.0)
    }

    /// Gets details about an unspent transaction output.
    pub async fn get_tx_out(
        &self,
        txid: &Txid,
        vout: u64,
    ) -> Result<model::GetTxOut, GetTxOutError> {
        // This type hasnt' changed between Core v17 and v31.
        let json: types::v25::GetTxOut =
            self.call("gettxout", &[into_json(txid)?, into_json(vout)?]).await?;
        json.into_model().map_err(GetTxOutError::Model)
    }

    /// Returns the version integer reported by the server (e.g. `250200` for v25.2.0).
    pub async fn server_version(&self) -> Result<usize, ServerVersionError> {
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
