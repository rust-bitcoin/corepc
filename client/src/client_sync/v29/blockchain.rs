// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v29`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getblock`
#[macro_export]
macro_rules! impl_client_v29__getblock {
    () => {
        impl Client {
            /// Gets a block by blockhash.
            pub fn get_block(&self, hash: BlockHash) -> Result<Block> {
                let json = self.get_block_verbose_zero(hash)?;
                Ok(json.block()?)
            }

            /// Gets a block by blockhash with verbose set to 0.
            pub fn get_block_verbose_zero(&self, hash: BlockHash) -> Result<GetBlockVerboseZero> {
                self.call("getblock", &[into_json(hash)?, 0.into()])
            }

            /// Gets a block by blockhash with verbose set to 1.
            pub fn get_block_verbose_one(&self, hash: BlockHash) -> Result<GetBlockVerboseOne> {
                self.call("getblock", &[into_json(hash)?, 1.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockheader`
#[macro_export]
macro_rules! impl_client_v29__getblockheader {
    () => {
        impl Client {
            pub fn get_block_header(&self, hash: &BlockHash) -> Result<GetBlockHeader> {
                self.call("getblockheader", &[into_json(hash)?, into_json(false)?])
            }

            // This is the same as calling getblockheader with verbose==true.
            pub fn get_block_header_verbose(
                &self,
                hash: &BlockHash,
            ) -> Result<GetBlockHeaderVerbose> {
                self.call("getblockheader", &[into_json(hash)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockchaininfo`
#[macro_export]
macro_rules! impl_client_v29__getblockchaininfo {
    () => {
        impl Client {
            pub fn get_blockchain_info(&self) -> Result<GetBlockchainInfo> {
                self.call("getblockchaininfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getdescriptoractivity`
#[macro_export]
macro_rules! impl_client_v29__getdescriptoractivity {
    () => {
        impl Client {
            pub fn get_descriptor_activity(
                &self,
                blockhashes: &[BlockHash],
                scan_objects: &[&str],
                include_mempool: bool,
            ) -> Result<GetDescriptorActivity> {
                let blockhashes_val = json!(blockhashes);
                let scan_objects_val = json!(scan_objects);
                let include_mempool_val = json!(include_mempool);

                let params = vec![blockhashes_val, scan_objects_val, include_mempool_val];

                self.call("getdescriptoractivity", &params)
            }
        }
    };
}
