// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing async JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_async_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getblock`.
#[macro_export]
macro_rules! impl_async_client_v17__get_block {
    () => {
        impl Client {
            /// Gets a block by blockhash.
            pub async fn get_block(&self, hash: BlockHash) -> Result<Block> {
                let json = self.get_block_verbose_zero(hash).await?;
                Ok(json.block()?)
            }

            /// Gets a block by blockhash with verbose set to 0.
            pub async fn get_block_verbose_zero(
                &self,
                hash: BlockHash,
            ) -> Result<GetBlockVerboseZero> {
                self.call("getblock", &[into_json(hash)?, 0.into()]).await
            }

            /// Gets a block by blockhash with verbose set to 1.
            pub async fn get_block_verbose_one(
                &self,
                hash: BlockHash,
            ) -> Result<GetBlockVerboseOne> {
                self.call("getblock", &[into_json(hash)?, 1.into()]).await
            }

            /// Alias for getblock verbosity 1, matching bitcoincore-rpc naming.
            pub async fn get_block_info(&self, hash: BlockHash) -> Result<GetBlockVerboseOne> {
                self.get_block_verbose_one(hash).await
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockcount`.
#[macro_export]
macro_rules! impl_async_client_v17__get_block_count {
    () => {
        impl Client {
            pub async fn get_block_count(&self) -> Result<GetBlockCount> {
                self.call("getblockcount", &[]).await
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockhash`.
#[macro_export]
macro_rules! impl_async_client_v17__get_block_hash {
    () => {
        impl Client {
            pub async fn get_block_hash(&self, height: u64) -> Result<GetBlockHash> {
                self.call("getblockhash", &[into_json(height)?]).await
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockheader`.
#[macro_export]
macro_rules! impl_async_client_v17__get_block_header {
    () => {
        impl Client {
            pub async fn get_block_header(&self, hash: &BlockHash) -> Result<GetBlockHeader> {
                self.call("getblockheader", &[into_json(hash)?, into_json(false)?]).await
            }

            // This is the same as calling getblockheader with verbose==true.
            pub async fn get_block_header_verbose(
                &self,
                hash: &BlockHash,
            ) -> Result<GetBlockHeaderVerbose> {
                self.call("getblockheader", &[into_json(hash)?]).await
            }

            /// Alias for getblockheader with verbose true.
            pub async fn get_block_header_info(
                &self,
                hash: &BlockHash,
            ) -> Result<GetBlockHeaderVerbose> {
                self.get_block_header_verbose(hash).await
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getrawmempool`.
#[macro_export]
macro_rules! impl_async_client_v17__get_raw_mempool {
    () => {
        impl Client {
            pub async fn get_raw_mempool(&self) -> Result<GetRawMempool> {
                // Equivalent to self.call("getrawmempool", &[into_json(false)?])
                self.call("getrawmempool", &[]).await
            }

            pub async fn get_raw_mempool_verbose(&self) -> Result<GetRawMempoolVerbose> {
                self.call("getrawmempool", &[into_json(true)?]).await
            }
        }
    };
}
