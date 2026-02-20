// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing async JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of Bitcoin Core `v0.21`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_async_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getrawmempool`.
#[macro_export]
macro_rules! impl_async_client_v21__get_raw_mempool {
    () => {
        impl Client {
            pub async fn get_raw_mempool(&self) -> Result<GetRawMempool> {
                // Equivalent to self.call("getrawmempool", &[into_json(false)?])
                self.call("getrawmempool", &[]).await
            }

            pub async fn get_raw_mempool_verbose(&self) -> Result<GetRawMempoolVerbose> {
                self.call("getrawmempool", &[into_json(true)?]).await
            }

            pub async fn get_raw_mempool_sequence(&self) -> Result<GetRawMempoolSequence> {
                self.call("getrawmempool", &[into_json(false)?, into_json(true)?]).await
            }
        }
    };
}
