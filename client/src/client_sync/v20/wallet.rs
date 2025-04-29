// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v21`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `abortrescan`
#[macro_export]
macro_rules! impl_client_v20__abortrescan {
    () => {
        impl Client {
            pub fn abort_rescan(&self) -> Result<AbortRescan> { self.call("abortrescan", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `encryptwallet`
#[macro_export]
macro_rules! impl_client_v20__encryptwallet {
    () => {
        impl Client {
            pub fn encrypt_wallet(&self, passphrase: &str) -> Result<EncryptWallet> {
                self.call("encryptwallet", &[passphrase.into()])
            }
        }
    };
}
