// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v0.18`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.
/// Implements Bitcoin Core JSON-RPC API method `getreceivedbylabel`.
#[macro_export]
macro_rules! impl_client_v18__get_received_by_label {
    () => {
        impl Client {
            pub fn get_received_by_label(&self, label: &str) -> Result<GetReceivedByLabel> {
                self.call("getreceivedbylabel", &[label.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listreceivedbylabel`.
#[macro_export]
macro_rules! impl_client_v18__list_received_by_label {
    () => {
        impl Client {
            pub fn list_received_by_label(&self) -> Result<ListReceivedByLabel> {
                self.call("listreceivedbylabel", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listwalletdir`.
#[macro_export]
macro_rules! impl_client_v18__list_wallet_dir {
    () => {
        impl Client {
            pub fn list_wallet_dir(&self) -> Result<ListWalletDir> {
                self.call("listwalletdir", &[])
            }
        }
    };
}
