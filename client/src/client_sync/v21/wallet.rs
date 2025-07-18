// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v0.21`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `createwallet` with descriptors=true (descriptor wallet).
#[macro_export]
macro_rules! impl_client_v21__create_wallet_with_descriptors {
    () => {
        impl Client {
            pub fn create_wallet_with_descriptors(&self, wallet: &str) -> Result<CreateWallet> {
                let args = [
                    wallet.into(),
                    false.into(),            // disable_private_keys.
                    false.into(),            // blank.
                    serde_json::Value::Null, // passphrase.
                    false.into(),            // avoid_reuse.
                    true.into(),             // descriptors=true.
                    serde_json::Value::Null, // load_on_startup.
                ];
                self.call("createwallet", &args)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importdescriptors`.
#[macro_export]
macro_rules! impl_client_v21__import_descriptors {
    () => {
        impl Client {
            pub fn import_descriptors(
                &self,
                requests: &[ImportDescriptorsRequest],
            ) -> Result<ImportDescriptors> {
                self.call("importdescriptors", &[into_json(requests)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `psbtbumpfee`.
#[macro_export]
macro_rules! impl_client_v21__psbt_bump_fee {
    () => {
        impl Client {
            pub fn psbt_bump_fee(&self, txid: &bitcoin::Txid) -> Result<PsbtBumpFee> {
                self.call("psbtbumpfee", &[into_json(txid)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `send`.
#[macro_export]
macro_rules! impl_client_v21__send {
    () => {
        impl Client {
            pub fn send(&self, outputs: &BTreeMap<String, f64>) -> Result<Send> {
                self.call("send", &[into_json(outputs)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `unloadwallet`.
#[macro_export]
macro_rules! impl_client_v21__unload_wallet {
    () => {
        impl Client {
            pub fn unload_wallet(&self, wallet: &str) -> Result<UnloadWallet> {
                self.call("unloadwallet", &[wallet.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `upgradewallet`.
#[macro_export]
macro_rules! impl_client_v21__upgrade_wallet {
    () => {
        impl Client {
            pub fn upgrade_wallet(&self) -> Result<UpgradeWallet> {
                self.call("upgradewallet", &[])
            }
        }
    };
}
