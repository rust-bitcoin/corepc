// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v0.21`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `createwallet`.
#[macro_export]
macro_rules! impl_client_v21__create_wallet {
    () => {
        impl Client {
            /// Calls `createwallet` with `wallet` as the only argument.
            ///
            /// In v21 and v22 this creates a legacy wallet. Use `create_descriptor_wallet` to create
            /// a descriptor wallet.
            pub fn create_wallet(&self, wallet: &str) -> Result<CreateWallet> {
                self.call("createwallet", &[wallet.into()])
            }

            /// Creates a wallet with descriptors=true (descriptor wallet).
            ///
            /// > createwallet "wallet_name" ( disable_private_keys blank "passphrase" avoid_reuse descriptors load_on_startup )
            /// >
            /// > Creates and loads a new wallet.
            /// >
            /// > Arguments:
            /// > 1. wallet_name             (string, required) The name for the new wallet. If this is a path, the wallet will be created at the path location.
            /// > 2. disable_private_keys    (boolean, optional, default=false) Disable the possibility of private keys (only watchonlys are possible in this mode).
            /// > 3. blank                   (boolean, optional, default=false) Create a blank wallet. A blank wallet has no keys or HD seed. One can be set using sethdseed.
            /// > 4. passphrase              (string, optional) Encrypt the wallet with this passphrase.
            /// > 5. avoid_reuse             (boolean, optional, default=false) Keep track of coin reuse, and treat dirty and clean coins differently with privacy considerations in mind.
            /// > 6. descriptors             (boolean, optional, default=true) Create a native descriptor wallet. The wallet will use descriptors internally to handle address creation
            /// > 7. load_on_startup         (boolean, optional) Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
            pub fn create_descriptor_wallet(&self, wallet: &str) -> Result<CreateWallet> {
                let disable_private_keys = false;
                let blank = false;
                let passphrase = String::new();
                let avoid_reuse = false;
                let descriptors = true;

                self.call(
                    "createwallet",
                    &[
                        wallet.into(),
                        disable_private_keys.into(),
                        blank.into(),
                        passphrase.into(),
                        avoid_reuse.into(),
                        descriptors.into(),
                    ],
                )
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

/// Implements Bitcoin Core JSON-RPC API method `sendmany` with `verbose=true` (v21+).
#[macro_export]
macro_rules! impl_client_v21__send_many_verbose {
    () => {
        impl Client {
            pub fn send_many_verbose(
                &self,
                amounts: BTreeMap<Address, Amount>,
            ) -> Result<SendManyVerbose> {
                let dummy = ""; // Backwards compatibility dummy.
                let amount_btc: BTreeMap<String, f64> = amounts
                    .into_iter()
                    .map(|(addr, amount)| (addr.to_string(), amount.to_btc()))
                    .collect();
                let minconf = 1u64;
                let comment = "";
                let subtract_fee_from: Vec<String> = Vec::new();
                let replaceable = true;
                let conf_target = 1u64;
                let estimate_mode = "unset";
                let fee_rate = serde_json::Value::Null;
                let verbose = true;
                self.call(
                    "sendmany",
                    &[
                        into_json(dummy)?,
                        into_json(amount_btc)?,
                        minconf.into(),
                        comment.into(),
                        into_json(subtract_fee_from)?,
                        replaceable.into(),
                        conf_target.into(),
                        estimate_mode.into(),
                        fee_rate,
                        verbose.into(),
                    ],
                )
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
