// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v28`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `gethdkeys`.
#[macro_export]
macro_rules! impl_client_v28__get_hd_keys {
    () => {
        impl Client {
            pub fn get_hd_keys(&self) -> Result<GetHdKeys> { self.call("gethdkeys", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `createwalletdescriptor`.
#[macro_export]
macro_rules! impl_client_v28__create_wallet_descriptor {
    () => {
        impl Client {
            pub fn create_wallet_descriptor(&self, address_type: &str, hdkey: &str) -> Result<CreateWalletDescriptor> {
                let hdkey = serde_json::json!({ "hdkey": hdkey });
                self.call("createwalletdescriptor", &[address_type.into(), hdkey.into()])
            }
        }
    };
}
