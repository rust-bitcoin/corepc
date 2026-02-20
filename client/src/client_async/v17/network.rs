// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing async JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Network ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_async_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getnetworkinfo`.
#[macro_export]
macro_rules! impl_async_client_v17__get_network_info {
    () => {
        impl Client {
            /// Returns the server version field of `GetNetworkInfo`.
            pub async fn server_version(&self) -> Result<usize> {
                let info = self.get_network_info().await?;
                Ok(info.version)
            }

            pub async fn get_network_info(&self) -> Result<GetNetworkInfo> {
                self.call("getnetworkinfo", &[]).await
            }
        }
    };
}
