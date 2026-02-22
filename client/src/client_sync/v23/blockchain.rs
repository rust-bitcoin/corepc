// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v23`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getblockfrompeer`.
#[macro_export]
macro_rules! impl_client_v23__get_block_from_peer {
    () => {
        impl Client {
            pub fn get_block_from_peer(&self, blockhash: BlockHash, peer_id: u32) -> Result<()> {
                match self.call("getblockfrompeer", &[into_json(blockhash)?, into_json(peer_id)?]) {
                    Ok(serde_json::Value::Object(ref map)) if map.is_empty() => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getdeploymentinfo`.
#[macro_export]
macro_rules! impl_client_v23__get_deployment_info {
    () => {
        impl Client {
            /// Query deployment info at the current chain tip.
            pub fn get_deployment_info_tip(&self) -> Result<GetDeploymentInfo> {
                self.call("getdeploymentinfo", &[])
            }

            /// Query deployment info at the given block hash.
            pub fn get_deployment_info(&self, blockhash: &BlockHash) -> Result<GetDeploymentInfo> {
                self.call("getdeploymentinfo", &[into_json(blockhash)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `savemempool`.
#[macro_export]
macro_rules! impl_client_v23__save_mempool {
    () => {
        impl Client {
            pub fn save_mempool(&self) -> Result<SaveMempool> { self.call("savemempool", &[]) }
        }
    };
}
