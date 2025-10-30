// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Rawtransactions ==` section of the
//! API docs of Bitcoin Core `v0.18`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `analyzepsbt`.
#[macro_export]
macro_rules! impl_client_v18__analyze_psbt {
    () => {
        impl Client {
            pub fn analyze_psbt(&self, psbt: &bitcoin::Psbt) -> Result<AnalyzePsbt> {
                let psbt = format!("{}", psbt);
                self.call("analyzepsbt", &[psbt.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `joinpsbts`.
#[macro_export]
macro_rules! impl_client_v18__join_psbts {
    () => {
        impl Client {
            pub fn join_psbts(&self, psbts: &[bitcoin::Psbt]) -> Result<JoinPsbts> {
                let psbts = psbts.iter().map(|psbt| format!("{}", psbt)).collect::<Vec<String>>();
                self.call("joinpsbts", &[psbts.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `utxoupdatepsbt`.
#[macro_export]
macro_rules! impl_client_v18__utxo_update_psbt {
    () => {
        impl Client {
            pub fn utxo_update_psbt(&self, psbt: &bitcoin::Psbt) -> Result<UtxoUpdatePsbt> {
                let psbt = format!("{}", psbt);
                self.call("utxoupdatepsbt", &[psbt.into()])
            }
        }
    };
}
