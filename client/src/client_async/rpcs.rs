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
use crate::client_async::{into_json, Client, Result};
use crate::types::model::{
    GetBlockFilter, GetBlockHeaderVerbose, GetBlockVerboseOne, GetBlockchainInfo, GetTxOut,
};

/// Template trait for implementing custom async RPC method sets.
///
/// This intentionally includes one method so downstream users can copy this pattern
/// and define their own trait with just the RPCs they need.
pub trait RpcApi: Sized {
    /// Call an RPC `method` with given `args` list.
    fn call<T: for<'a> serde::de::Deserialize<'a>>(
        &self,
        method: &str,
        args: &[serde_json::Value],
    ) -> impl std::future::Future<Output = Result<T>> + Send;

    /// Gets the block count.
    fn get_block_count(&self) -> impl std::future::Future<Output = Result<u64>> + Send
    where
        Self: Sync,
    {
        async move {
            let json: crate::types::v25::GetBlockCount = self.call("getblockcount", &[]).await?;
            Ok(json.into_model().0)
        }
    }
}

impl RpcApi for Client {
    fn call<T: for<'a> serde::de::Deserialize<'a>>(
        &self,
        method: &str,
        args: &[serde_json::Value],
    ) -> impl std::future::Future<Output = Result<T>> + Send {
        Client::call(self, method, args)
    }
}

impl Client {
    /// Gets a block by blockhash.
    pub async fn get_block(&self, hash: &BlockHash) -> std::result::Result<Block, GetBlockError> {
        let json: crate::types::v25::GetBlockVerboseZero =
            self.call("getblock", &[into_json(hash)?, into_json(0)?]).await?;
        Ok(json.into_model().map_err(GetBlockError::Model)?.0)
    }

    /// Gets the block count.
    pub async fn get_block_count(&self) -> std::result::Result<u64, GetBlockCountError> {
        let json: crate::types::v25::GetBlockCount = self.call("getblockcount", &[]).await?;
        Ok(json.into_model().0)
    }

    /// Gets the block hash for a height.
    pub async fn get_block_hash(
        &self,
        height: u32,
    ) -> std::result::Result<BlockHash, GetBlockHashError> {
        let json: crate::types::v25::GetBlockHash =
            self.call("getblockhash", &[into_json(height)?]).await?;
        Ok(json.into_model().map_err(GetBlockHashError::Model)?.0)
    }

    /// Gets the hash of the chain tip.
    pub async fn get_best_block_hash(
        &self,
    ) -> std::result::Result<BlockHash, GetBestBlockHashError> {
        let json: crate::types::v25::GetBestBlockHash = self.call("getbestblockhash", &[]).await?;
        Ok(json.into_model().map_err(GetBestBlockHashError::Model)?.0)
    }

    /// Gets the block header by blockhash.
    pub async fn get_block_header(
        &self,
        hash: &BlockHash,
    ) -> std::result::Result<block::Header, GetBlockHeaderError> {
        let json: crate::types::v25::GetBlockHeader =
            self.call("getblockheader", &[into_json(hash)?, into_json(false)?]).await?;
        Ok(json.into_model().map_err(GetBlockHeaderError::Model)?.0)
    }

    /// Gets the block header with verbose output.
    ///
    /// The version specific type is detected from the response shape.
    pub async fn get_block_header_verbose(
        &self,
        hash: &BlockHash,
    ) -> std::result::Result<GetBlockHeaderVerbose, GetBlockHeaderVerboseError> {
        let raw: Box<RawValue> =
            self.call("getblockheader", &[into_json(hash)?, into_json(true)?]).await?;
        if let Ok(json) =
            serde_json::from_str::<crate::types::v29::GetBlockHeaderVerbose>(raw.get())
        {
            Ok(json.into_model().map_err(|e| GetBlockHeaderVerboseError::Model(Box::new(e)))?)
        } else {
            let json: crate::types::v25::GetBlockHeaderVerbose = serde_json::from_str(raw.get())?;
            Ok(json.into_model().map_err(|e| GetBlockHeaderVerboseError::Model(Box::new(e)))?)
        }
    }

    /// Gets a block by blockhash with verbose set to 1.
    ///
    /// The version specific type is detected from the response shape.
    pub async fn get_block_verbose(
        &self,
        hash: &BlockHash,
    ) -> std::result::Result<GetBlockVerboseOne, GetBlockVerboseError> {
        let raw: Box<RawValue> = self.call("getblock", &[into_json(hash)?, into_json(1)?]).await?;
        if let Ok(json) = serde_json::from_str::<crate::types::v31::GetBlockVerboseOne>(raw.get()) {
            Ok(json.into_model().map_err(|e| GetBlockVerboseError::Model(Box::new(e)))?)
        } else if let Ok(json) =
            serde_json::from_str::<crate::types::v29::GetBlockVerboseOne>(raw.get())
        {
            Ok(json.into_model().map_err(|e| GetBlockVerboseError::Model(Box::new(e)))?)
        } else {
            let json: crate::types::v25::GetBlockVerboseOne = serde_json::from_str(raw.get())?;
            Ok(json.into_model().map_err(|e| GetBlockVerboseError::Model(Box::new(e)))?)
        }
    }

    /// Gets the block filter for a blockhash.
    pub async fn get_block_filter(
        &self,
        hash: &BlockHash,
    ) -> std::result::Result<GetBlockFilter, GetBlockFilterError> {
        let json: crate::types::v25::GetBlockFilter =
            self.call("getblockfilter", &[into_json(hash)?]).await?;
        json.into_model().map_err(GetBlockFilterError::Model)
    }

    /// Gets various state info regarding blockchain processing.
    ///
    /// The version specific type is detected from the response shape.
    pub async fn get_blockchain_info(
        &self,
    ) -> std::result::Result<GetBlockchainInfo, GetBlockchainInfoError> {
        let raw: Box<RawValue> = self.call("getblockchaininfo", &[]).await?;
        if let Ok(json) = serde_json::from_str::<crate::types::v29::GetBlockchainInfo>(raw.get()) {
            Ok(json.into_model().map_err(|e| GetBlockchainInfoError::Model(Box::new(e)))?)
        } else if let Ok(json) =
            serde_json::from_str::<crate::types::v28::GetBlockchainInfo>(raw.get())
        {
            Ok(json.into_model().map_err(|e| GetBlockchainInfoError::Model(Box::new(e)))?)
        } else {
            let json: crate::types::v25::GetBlockchainInfo = serde_json::from_str(raw.get())?;
            Ok(json.into_model().map_err(|e| GetBlockchainInfoError::Model(Box::new(e)))?)
        }
    }

    /// Gets the transaction IDs currently in the mempool.
    pub async fn get_raw_mempool(&self) -> std::result::Result<Vec<Txid>, GetRawMempoolError> {
        let json: crate::types::v25::GetRawMempool = self.call("getrawmempool", &[]).await?;
        Ok(json.into_model().map_err(GetRawMempoolError::Model)?.0)
    }

    /// Gets the raw transaction by txid.
    pub async fn get_raw_transaction(
        &self,
        txid: &Txid,
    ) -> std::result::Result<Transaction, GetRawTransactionError> {
        let json: crate::types::v25::GetRawTransaction =
            self.call("getrawtransaction", &[into_json(txid)?]).await?;
        Ok(json.into_model().map_err(GetRawTransactionError::Model)?.0)
    }

    /// Gets details about an unspent transaction output.
    pub async fn get_tx_out(
        &self,
        txid: &Txid,
        vout: u64,
    ) -> std::result::Result<GetTxOut, GetTxOutError> {
        let json: crate::types::v25::GetTxOut =
            self.call("gettxout", &[into_json(txid)?, into_json(vout)?]).await?;
        json.into_model().map_err(GetTxOutError::Model)
    }

    /// Returns the version integer reported by the server (e.g. `250200` for v25.2.0).
    pub async fn server_version(&self) -> std::result::Result<usize, ServerVersionError> {
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
