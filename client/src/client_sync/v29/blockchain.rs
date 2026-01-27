// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v29`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `dumptxoutset`.
#[macro_export]
macro_rules! impl_client_v29__dump_tx_out_set {
    () => {
        impl Client {
            pub fn dump_tx_out_set(&self, path: &str, snapshot_type: &str) -> Result<DumpTxOutSet> {
                self.call("dumptxoutset", &[path.into(), snapshot_type.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getdescriptoractivity`.
#[macro_export]
macro_rules! impl_client_v29__get_descriptor_activity {
    () => {
        impl Client {
            pub fn get_descriptor_activity(&self) -> Result<GetDescriptorActivity> {
                let block_hashes: &[BlockHash] = &[];
                let scan_objects: &[&str] = &[];
                // FIXME: Core errors if we don't pass empty arrays?
                let params = vec![json!(block_hashes), json!(scan_objects)];
                self.call("getdescriptoractivity", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblock`.
#[macro_export]
macro_rules! impl_client_v29__get_block {
    () => {
        impl Client {
            /// Gets a block by blockhash. Kept for compatibility; uses verbose set to 0.
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

            /// Gets a block by blockhash with verbose set to 2.
            pub fn get_block_verbose_two(&self, hash: BlockHash) -> Result<GetBlockVerboseTwo> {
                self.call("getblock", &[into_json(hash)?, 2.into()])
            }

            /// Gets a block by blockhash with verbose set to 3.
            pub fn get_block_verbose_three(&self, hash: BlockHash) -> Result<GetBlockVerboseThree> {
                self.call("getblock", &[into_json(hash)?, 3.into()])
            }
        }
    };
}
