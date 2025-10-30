// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v24`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `migratewallet`.
#[macro_export]
macro_rules! impl_client_v24__migrate_wallet {
    () => {
        impl Client {
            pub fn migrate_wallet(&self, wallet_name: &str) -> Result<MigrateWallet> {
                self.call("migratewallet", &[wallet_name.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `sendall`.
#[macro_export]
macro_rules! impl_client_v24__send_all {
    () => {
        impl Client {
            pub fn send_all(&self, recipients: &[Address]) -> Result<SendAll> {
                self.call("sendall", &[into_json(recipients)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `simulaterawtransaction`.
#[macro_export]
macro_rules! impl_client_v24__simulate_raw_transaction {
    () => {
        impl Client {
            pub fn simulate_raw_transaction(
                &self,
                rawtxs: &[String],
            ) -> Result<SimulateRawTransaction> {
                self.call("simulaterawtransaction", &[into_json(rawtxs)?])
            }
        }
    };
}
