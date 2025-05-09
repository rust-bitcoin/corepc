// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Mining ==` section of the
//! API docs of Bitcoin Core `v29`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getmininginfo`
#[macro_export]
macro_rules! impl_client_v29__getmininginfo {
    () => {
        impl Client {
            pub fn get_mining_info(&self) -> Result<GetMiningInfo> {
                self.call("getmininginfo", &[])
            }
        }
    };
}

#[macro_export]
macro_rules! impl_client_v29__getblocktemplate {
    () => {
        impl Client {
            pub fn get_block_template(
                &self,
                request: &$crate::client_sync::v29::TemplateRequest,
            ) -> Result<GetBlockTemplate> {
                self.call("getblocktemplate", &[into_json(request)?])
            }
        }
    };
}
