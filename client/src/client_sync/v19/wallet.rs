// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v0.19`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `getbalances`.
#[macro_export]
macro_rules! impl_client_v19__get_balances {
    () => {
        impl Client {
            pub fn get_balances(&self) -> Result<GetBalances> { self.call("getbalances", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `setwalletflag`.
#[macro_export]
macro_rules! impl_client_v19__set_wallet_flag {
    () => {
        impl Client {
            pub fn set_wallet_flag(&self, flag: &str) -> Result<SetWalletFlag> {
                self.call("setwalletflag", &[into_json(flag)?])
            }
        }
    };
}
