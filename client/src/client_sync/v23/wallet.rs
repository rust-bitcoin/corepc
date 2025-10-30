// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v23`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `createwallet`.
#[macro_export]
macro_rules! impl_client_v23__create_wallet {
    () => {
        impl Client {
            /// Calls `createwallet` with `wallet` as the only argument.
            ///
            /// In v23 and later this creates a descriptor wallet. Use `create_legacy_wallet` to create
            /// a legacy wallet.
            pub fn create_wallet(&self, wallet: &str) -> Result<CreateWallet> {
                self.call("createwallet", &[wallet.into()])
            }

            /// Creates a legacy wallet (i.e not a native descriptor wallet).
            ///
            /// > createwallet "wallet_name" ( disable_private_keys blank "passphrase" avoid_reuse descriptors load_on_startup external_signer )
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
            /// > 8. external_signer         (boolean, optional, default=false) Use an external signer such as a hardware wallet. Requires -signer to be configured. Wallet creation will fail if keys cannot be fetched. Requires disable_private_keys and descriptors set to true.
            pub fn create_legacy_wallet(&self, wallet: &str) -> Result<CreateWallet> {
                let disable_private_keys = false;
                let blank = false;
                let passphrase = String::new();
                let avoid_reuse = false;
                let descriptors = false;

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

/// Implements Bitcoin Core JSON-RPC API method `newkeypool`.
#[macro_export]
macro_rules! impl_client_v23__new_keypool {
    () => {
        impl Client {
            /// Calls `newkeypool` for the loaded wallet.
            ///
            /// > newkeypool
            /// >
            /// > Entirely clears and refills the keypool.
            /// > Requires wallet passphrase to be set if wallet is encrypted.
            pub fn new_keypool(&self) -> Result<()> {
                match self.call("newkeypool", &[]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `restorewallet`.
#[macro_export]
macro_rules! impl_client_v23__restore_wallet {
    () => {
        impl Client {
            /// Calls `restorewallet` with required and optional arguments.
            ///
            /// > restorewallet "wallet_name" "backup_file" ( load_on_startup )
            pub fn restore_wallet(
                &self,
                wallet_name: &str,
                backup_file: &Path,
            ) -> Result<RestoreWallet> {
                self.call("restorewallet", &[wallet_name.into(), into_json(backup_file)?])
            }
        }
    };
}
