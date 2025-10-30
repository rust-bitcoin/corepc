// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v22`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `listdescriptors`.
#[macro_export]
macro_rules! impl_client_v22__list_descriptors {
    () => {
        impl Client {
            pub fn list_descriptors(&self) -> Result<ListDescriptors> {
                self.call("listdescriptors", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `loadwallet`.
#[macro_export]
macro_rules! impl_client_v22__load_wallet {
    () => {
        impl Client {
            pub fn load_wallet(&self, wallet: &str) -> Result<LoadWallet> {
                self.call("loadwallet", &[wallet.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletdisplayaddress`.
#[macro_export]
macro_rules! impl_client_v22__wallet_display_address {
    () => {
        impl Client {
            pub fn wallet_display_address(&self, address: &str) -> Result<WalletDisplayAddress> {
                self.call("walletdisplayaddress", &[address.into()])
            }
        }
    };
}
