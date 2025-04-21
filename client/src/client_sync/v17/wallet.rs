// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `addmultisigaddress`.
#[macro_export]
macro_rules! impl_client_v17__addmultisigaddress {
    () => {
        impl Client {
            pub fn add_multisig_address_with_keys(
                &self,
                nrequired: u32,
                keys: Vec<PublicKey>,
            ) -> Result<AddMultisigAddress> {
                self.call("addmultisigaddress", &[nrequired.into(), into_json(keys)?])
            }

            pub fn add_multisig_address_with_addresses(
                &self,
                nrequired: u32,
                keys: Vec<Address>,
            ) -> Result<AddMultisigAddress> {
                self.call("addmultisigaddress", &[nrequired.into(), into_json(keys)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `bumpfee`.
#[macro_export]
macro_rules! impl_client_v17__bumpfee {
    () => {
        impl Client {
            pub fn bump_fee(&self, txid: Txid) -> Result<BumpFee> {
                self.call("bumpfee", &[into_json(txid)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `createwallet`.
#[macro_export]
macro_rules! impl_client_v17__createwallet {
    () => {
        impl Client {
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

/// Implements Bitcoin Core JSON-RPC API method `dumpprivkey`.
#[macro_export]
macro_rules! impl_client_v17__dumpprivkey {
    () => {
        impl Client {
            pub fn dump_priv_key(&self, address: &Address) -> Result<DumpPrivKey> {
                self.call("dumpprivkey", &[into_json(address)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `dumpwallet`.
#[macro_export]
macro_rules! impl_client_v17__dumpwallet {
    () => {
        impl Client {
            // filename is either absolute or relative to bitcoind.
            pub fn dump_wallet(&self, filename: &Path) -> Result<DumpWallet> {
                self.call("dumpwallet", &[into_json(filename)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getaddressesbylabel`.
#[macro_export]
macro_rules! impl_client_v17__getaddressesbylabel {
    () => {
        impl Client {
            pub fn get_addresses_by_label(&self, label: &str) -> Result<GetAddressesByLabel> {
                self.call("getaddressesbylabel", &[label.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getaddressinfo`.
#[macro_export]
macro_rules! impl_client_v17__getaddressinfo {
    () => {
        impl Client {
            pub fn get_address_info(&self, address: &Address) -> Result<GetAddressInfo> {
                self.call("getaddressinfo", &[into_json(address)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getbalance`.
#[macro_export]
macro_rules! impl_client_v17__getbalance {
    () => {
        impl Client {
            pub fn get_balance(&self) -> Result<GetBalance> { self.call("getbalance", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getnewaddress`.
#[macro_export]
macro_rules! impl_client_v17__getnewaddress {
    () => {
        impl Client {
            /// Gets a new address from `bitcoind` and parses it assuming its correct.
            pub fn new_address(&self) -> Result<bitcoin::Address> {
                let json = self.get_new_address(None, None)?;
                let model = json.into_model().unwrap();
                Ok(model.0.assume_checked())
            }

            /// Gets a new address from `bitcoind` and parses it assuming its correct.
            pub fn new_address_with_type(&self, ty: AddressType) -> Result<bitcoin::Address> {
                let json = self.get_new_address(None, Some(ty))?;
                let model = json.into_model().unwrap();
                Ok(model.0.assume_checked())
            }

            /// Gets a new address with label from `bitcoind` and parses it assuming its correct.
            // FIXME: unchecked network here is ugly and not uniform with other functions.
            pub fn new_address_with_label(
                &self,
                label: &str,
            ) -> Result<bitcoin::Address<bitcoin::address::NetworkUnchecked>> {
                let json = self.get_new_address(Some(label), None)?;
                let model = json.into_model().unwrap();
                Ok(model.0)
            }

            /// Gets a new address - low level RPC call.
            pub fn get_new_address(
                &self,
                label: Option<&str>,
                ty: Option<AddressType>,
            ) -> Result<GetNewAddress> {
                match (label, ty) {
                    (Some(label), Some(ty)) =>
                        self.call("getnewaddress", &[into_json(label)?, into_json(ty)?]),
                    (Some(label), None) => self.call("getnewaddress", &[into_json(label)?]),
                    (None, Some(ty)) => self.call("getnewaddress", &["".into(), into_json(ty)?]),
                    (None, None) => self.call("getnewaddress", &[]),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getrawchangeaddress`.
#[macro_export]
macro_rules! impl_client_v17__getrawchangeaddress {
    () => {
        impl Client {
            pub fn get_raw_change_address(&self) -> Result<GetRawChangeAddress> {
                self.call("getrawchangeaddress", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getreceivedbyaddress`.
#[macro_export]
macro_rules! impl_client_v17__getreceivedbyaddress {
    () => {
        impl Client {
            pub fn get_received_by_address(
                &self,
                address: &Address<NetworkChecked>,
            ) -> Result<GetReceivedByAddress> {
                self.call("getreceivedbyaddress", &[address.to_string().into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettransaction`.
#[macro_export]
macro_rules! impl_client_v17__gettransaction {
    () => {
        impl Client {
            pub fn get_transaction(&self, txid: Txid) -> Result<GetTransaction> {
                self.call("gettransaction", &[into_json(txid)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getunconfirmedbalance`.
#[macro_export]
macro_rules! impl_client_v17__getunconfirmedbalance {
    () => {
        impl Client {
            pub fn get_unconfirmed_balance(&self) -> Result<GetUnconfirmedBalance> {
                self.call("getunconfirmedbalance", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getwalletinfo`.
#[macro_export]
macro_rules! impl_client_v17__getwalletinfo {
    () => {
        impl Client {
            pub fn get_wallet_info(&self) -> Result<GetWalletInfo> {
                self.call("getwalletinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listaddressgroupings`.
#[macro_export]
macro_rules! impl_client_v17__listaddressgroupings {
    () => {
        impl Client {
            pub fn list_address_groupings(&self) -> Result<ListAddressGroupings> {
                self.call("listaddressgroupings", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listlabels`.
#[macro_export]
macro_rules! impl_client_v17__listlabels {
    () => {
        impl Client {
            pub fn list_labels(&self) -> Result<ListLabels> { self.call("listlabels", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listlockunspent`.
#[macro_export]
macro_rules! impl_client_v17__listlockunspent {
    () => {
        impl Client {
            pub fn list_lock_unspent(&self) -> Result<ListLockUnspent> {
                self.call("listlockunspent", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listreceivedbyaddress`.
#[macro_export]
macro_rules! impl_client_v17__listreceivedbyaddress {
    () => {
        impl Client {
            pub fn list_received_by_address(&self) -> Result<ListReceivedByAddress> {
                self.call("listreceivedbyaddress", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listsinceblock`.
#[macro_export]
macro_rules! impl_client_v17__listsinceblock {
    () => {
        impl Client {
            pub fn list_since_block(&self) -> Result<ListSinceBlock> {
                self.call("listsinceblock", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listtransactions`.
#[macro_export]
macro_rules! impl_client_v17__listtransactions {
    () => {
        impl Client {
            pub fn list_transactions(&self) -> Result<ListTransactions> {
                self.call("listtransactions", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listunspent`.
#[macro_export]
macro_rules! impl_client_v17__listunspent {
    () => {
        impl Client {
            pub fn list_unspent(&self) -> Result<ListUnspent> { self.call("listunspent", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listwallets`.
#[macro_export]
macro_rules! impl_client_v17__listwallets {
    () => {
        impl Client {
            pub fn list_wallets(&self) -> Result<ListWallets> { self.call("listwallets", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `loadwallet`.
#[macro_export]
macro_rules! impl_client_v17__loadwallet {
    () => {
        impl Client {
            pub fn load_wallet(&self, filename: &str) -> Result<LoadWallet> {
                self.call("loadwallet", &[into_json(filename)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `rescanblockchain`.
#[macro_export]
macro_rules! impl_client_v17__rescanblockchain {
    () => {
        impl Client {
            pub fn rescan_blockchain(&self) -> Result<RescanBlockchain> {
                self.call("rescanblockchain", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `sendmany`.
#[macro_export]
macro_rules! impl_client_v17__sendmany {
    () => {
        impl Client {
            pub fn send_many(&self, amounts: BTreeMap<Address, Amount>) -> Result<SendMany> {
                let dummy = ""; // Must be set to "" for backwards compatibility.
                self.call("sendmany", &[into_json(dummy)?, into_json(amounts)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `sendtoaddress`.
#[macro_export]
macro_rules! impl_client_v17__sendtoaddress {
    () => {
        impl Client {
            // Send to address - no RBF.
            pub fn send_to_address(
                &self,
                address: &Address<NetworkChecked>,
                amount: Amount,
            ) -> Result<SendToAddress> {
                let args = [address.to_string().into(), into_json(amount.to_btc())?];
                self.call("sendtoaddress", &args)
            }

            // Send to address - with RBF.
            pub fn send_to_address_rbf(
                &self,
                address: &Address<NetworkChecked>,
                amount: Amount,
            ) -> Result<SendToAddress> {
                let comment = "";
                let comment_to = "";
                let subtract_fee_from_amount = false;
                let replaceable = true;

                let args = [
                    address.to_string().into(),
                    into_json(amount.to_btc())?,
                    comment.into(),
                    comment_to.into(),
                    subtract_fee_from_amount.into(),
                    replaceable.into(),
                ];
                self.call("sendtoaddress", &args)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `signmessage`.
#[macro_export]
macro_rules! impl_client_v17__signmessage {
    () => {
        impl Client {
            pub fn sign_message(&self, address: &Address, message: &str) -> Result<SignMessage> {
                self.call("signmessage", &[into_json(address)?, into_json(message)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `signrawtransactionwithwallet`.
#[macro_export]
macro_rules! impl_client_v17__signrawtransactionwithwallet {
    () => {
        impl Client {
            // `hexstring`: The transaction hex string.
            pub fn sign_raw_transaction_with_wallet(
                &self,
                hex: &str,
            ) -> Result<SignRawTransactionWithWallet> {
                self.call("signrawtransactionwithwallet", &[into_json(hex)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `unloadwallet`.
#[macro_export]
macro_rules! impl_client_v17__unloadwallet {
    () => {
        impl Client {
            pub fn unload_wallet(&self, wallet_name: &str) -> Result<()> {
                match self.call("unloadwallet", &[into_json(wallet_name)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletcreatefundedpsbt`.
#[macro_export]
macro_rules! impl_client_v17__walletcreatefundedpsbt {
    () => {
        impl Client {
            pub fn wallet_create_funded_psbt(
                &self,
                inputs: Vec<$crate::client_sync::WalletCreateFundedPsbtInput>,
                outputs: Vec<BTreeMap<Address, Amount>>,
            ) -> Result<WalletCreateFundedPsbt> {
                self.call("walletcreatefundedpsbt", &[into_json(inputs)?, into_json(outputs)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletprocesspsbt`.
#[macro_export]
macro_rules! impl_client_v17__walletprocesspsbt {
    () => {
        impl Client {
            pub fn wallet_process_psbt(&self, psbt: &bitcoin::Psbt) -> Result<WalletProcessPsbt> {
                self.call("walletprocesspsbt", &[into_json(psbt)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `abandontransaction`
#[macro_export]
macro_rules! impl_client_v17__abandontransaction {
    () => {
        impl Client {
            pub fn abandon_transaction(&self, txid: Txid) -> Result<AbandonTransaction> {
                match self.call("abandontransaction", &[into_json(txid)?]) {
                    Ok(serde_json::Value::Null) => Ok(AbandonTransaction),
                    Ok(ref val) if val.is_null() => Ok(AbandonTransaction),
                    Ok(other) => Err(Error::Returned(format!(
                        "abandontransaction expected null, got: {}",
                        other
                    ))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `abortrescan`
#[macro_export]
macro_rules! impl_client_v17__abortrescan {
    () => {
        impl Client {
            pub fn abort_rescan(&self) -> Result<AbortRescan> {
                match self.call("abortrescan", &[]) {
                    Ok(serde_json::Value::Null) => Ok(AbortRescan),
                    Ok(ref val) if val.is_null() => Ok(AbortRescan),
                    Ok(other) =>
                        Err(Error::Returned(format!("abortrescan expected null, got: {}", other))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `backupwallet`
#[macro_export]
macro_rules! impl_client_v17__backupwallet {
    () => {
        impl Client {
            pub fn backup_wallet(&self, destination: &Path) -> Result<BackupWallet> {
                let dest_str = destination.to_string_lossy();
                match self.call("backupwallet", &[dest_str.as_ref().into()]) {
                    Ok(serde_json::Value::Null) => Ok(BackupWallet),
                    Ok(ref val) if val.is_null() => Ok(BackupWallet),
                    Ok(other) =>
                        Err(Error::Returned(format!("backupwallet expected null, got: {}", other))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `encryptwallet`
#[macro_export]
macro_rules! impl_client_v17__encryptwallet {
    () => {
        impl Client {
            pub fn encrypt_wallet(&self, passphrase: &str) -> Result<EncryptWallet> {
                match self.call("encryptwallet", &[passphrase.into()]) {
                    Ok(serde_json::Value::Null) => Ok(EncryptWallet),
                    Ok(ref val) if val.is_null() => Ok(EncryptWallet),
                    Ok(other) => Err(Error::Returned(format!(
                        "encryptwallet v17-v19 expected null, got: {}",
                        other
                    ))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importaddress`
#[macro_export]
macro_rules! impl_client_v17__importaddress {
    () => {
        impl Client {
            pub fn import_address(
                &self,
                address_or_script: &str,
                label: Option<&str>,
                rescan: Option<bool>,
                p2sh: Option<bool>,
            ) -> Result<ImportAddress> {
                let mut params = vec![address_or_script.into()];

                if label.is_some() || rescan.is_some() || p2sh.is_some() {
                    params.push(label.map_or(serde_json::Value::String("".into()), |l| l.into()));
                }

                if rescan.is_some() || p2sh.is_some() {
                    params.push(rescan.map_or(true.into(), |r| r.into()));
                }

                if let Some(p) = p2sh {
                    params.push(p.into());
                }

                match self.call("importaddress", &params) {
                    Ok(serde_json::Value::Null) => Ok(ImportAddress),
                    Ok(ref val) if val.is_null() => Ok(ImportAddress),
                    Ok(other) => Err(Error::Returned(format!(
                        "importaddress expected null, got: {}", other
                    ))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importprunedfunds`
#[macro_export]
macro_rules! impl_client_v17__importprunedfunds {
    () => {
        impl Client {
            pub fn import_pruned_funds(
                &self,
                raw_transaction: &str,
                txout_proof: &str,
            ) -> Result<ImportPrunedFunds> {
                match self.call("importprunedfunds", &[raw_transaction.into(), txout_proof.into()])
                {
                    Ok(serde_json::Value::Null) => Ok(ImportPrunedFunds),
                    Ok(ref val) if val.is_null() => Ok(ImportPrunedFunds),
                    Ok(other) => Err(Error::Returned(format!(
                        "importprunedfunds expected null, got: {}",
                        other
                    ))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importpubkey`
#[macro_export]
macro_rules! impl_client_v17__importpubkey {
    () => {
        impl Client {
            pub fn import_pubkey(
                &self,
                pubkey: &PublicKey,
                label: Option<&str>,
                rescan: Option<bool>,
            ) -> Result<ImportPubKey> {
                let pubkey_hex = pubkey.to_string();
                let mut params = vec![pubkey_hex.into()];

                if label.is_some() || rescan.is_some() {
                    params.push(label.map_or(serde_json::Value::String("".into()), |l| l.into()));
                }

                if let Some(r) = rescan {
                    params.push(r.into());
                }

                match self.call("importpubkey", &params) {
                    Ok(serde_json::Value::Null) => Ok(ImportPubKey),
                    Ok(ref val) if val.is_null() => Ok(ImportPubKey),
                    Ok(other) =>
                        Err(Error::Returned(format!("importpubkey expected null, got: {}", other))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importwallet`
#[macro_export]
macro_rules! impl_client_v17__importwallet {
    () => {
        impl Client {
            pub fn import_wallet(&self, filename: &Path) -> Result<ImportWallet> {
                let filename_str = filename.to_string_lossy();
                match self.call("importwallet", &[filename_str.as_ref().into()]) {
                    Ok(serde_json::Value::Null) => Ok(ImportWallet),
                    Ok(ref val) if val.is_null() => Ok(ImportWallet),
                    Ok(other) =>
                        Err(Error::Returned(format!("importwallet expected null, got: {}", other))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `keypoolrefill`
#[macro_export]
macro_rules! impl_client_v17__keypoolrefill {
    () => {
        impl Client {
            pub fn keypool_refill(&self, new_size: Option<usize>) -> Result<KeypoolRefill> {
                let params = match new_size {
                    Some(size) => vec![size.into()],
                    None => vec![],
                };

                match self.call("keypoolrefill", &params) {
                    Ok(serde_json::Value::Null) => Ok(KeypoolRefill),
                    Ok(ref val) if val.is_null() => Ok(KeypoolRefill),
                    Ok(other) => Err(Error::Returned(format!("keypoolrefill expected null, got: {}", other))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `lockunspent`
#[macro_export]
macro_rules! impl_client_v17__lockunspent {
    () => {
        use $crate::client_sync::LockUnspentOutput;
        impl Client {
            pub fn lock_unspent(
                &self,
                unlock: bool,
                outputs: Option<&[LockUnspentOutput]>,
                persistent: Option<bool>,
            ) -> Result<LockUnspent> {
                let mut params = vec![unlock.into()];

                match outputs {
                    Some(outs) => params.push(serde_json::to_value(outs)?),
                    None =>
                        if unlock {
                            params.push(serde_json::Value::Array(vec![]));
                        } else {
                            return Err(Error::Returned(
                                "lockunspent requires specific outputs when locking (unlock=false)"
                                    .to_string(),
                            ));
                        },
                }

                if !unlock {
                    if let Some(p) = persistent {
                        if params.len() == 1 {
                            params.push(serde_json::Value::Array(vec![]));
                        }
                        params.push(p.into());
                    }
                }
                self.call("lockunspent", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `removeprunedfunds`
#[macro_export]
macro_rules! impl_client_v17__removeprunedfunds {
    () => {
        impl Client {
            pub fn remove_pruned_funds(&self, txid: Txid) -> Result<RemovePrunedFunds> {
                match self.call("removeprunedfunds", &[into_json(txid)?]) {
                    Ok(serde_json::Value::Null) => Ok(RemovePrunedFunds),
                    Ok(ref val) if val.is_null() => Ok(RemovePrunedFunds),
                    Ok(other) => Err(Error::Returned(format!(
                        "removeprunedfunds expected null, got: {}",
                        other
                    ))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `sethdseed`
#[macro_export]
macro_rules! impl_client_v17__sethdseed {
    () => {
        impl Client {
            pub fn set_hd_seed(
                &self,
                new_keypool: Option<bool>,
                seed: Option<&PrivateKey>,
            ) -> Result<SetHdSeed> {
                let mut params = vec![];

                if new_keypool.is_some() || seed.is_some() {
                    params.push(new_keypool.map_or(true.into(), |k| k.into()));
                }

                if let Some(s) = seed {
                    params.push(s.to_wif().into());
                }

                match self.call("sethdseed", &params) {
                    Ok(serde_json::Value::Null) => Ok(SetHdSeed),
                    Ok(ref val) if val.is_null() => Ok(SetHdSeed),
                    Ok(other) =>
                        Err(Error::Returned(format!("sethdseed expected null, got: {}", other))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `settxfee`
#[macro_export]
macro_rules! impl_client_v17__settxfee {
    () => {
        const SATS_PER_BTC_F64_SETTXFEE: f64 = 100_000_000.0;
        fn fee_rate_to_rpc_arg_settxfee(fee_rate: bitcoin::FeeRate) -> f64 {
            let sat_per_kwu = fee_rate.to_sat_per_kwu();
            let sat_per_kvb = (sat_per_kwu as f64) * 4.0;
            sat_per_kvb / SATS_PER_BTC_F64_SETTXFEE
        }

        impl Client {
            pub fn set_tx_fee(&self, fee_rate: bitcoin::FeeRate) -> Result<SetTxFee> {
                let amount_rpc_arg = fee_rate_to_rpc_arg_settxfee(fee_rate);

                self.call("settxfee", &[amount_rpc_arg.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletlock`
#[macro_export]
macro_rules! impl_client_v17__walletlock {
    () => {
        impl Client {
            pub fn wallet_lock(&self) -> Result<WalletLock> {
                match self.call("walletlock", &[]) {
                    Ok(serde_json::Value::Null) => Ok(WalletLock),
                    Ok(ref val) if val.is_null() => Ok(WalletLock),
                    Ok(other) =>
                        Err(Error::Returned(format!("walletlock expected null, got: {}", other))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletpassphrase`
#[macro_export]
macro_rules! impl_client_v17__walletpassphrase {
    () => {
        impl Client {
            pub fn wallet_passphrase(
                &self,
                passphrase: &str,
                timeout: u64,
            ) -> Result<WalletPassPhrase> {
                match self.call("walletpassphrase", &[passphrase.into(), timeout.into()]) {
                    Ok(serde_json::Value::Null) => Ok(WalletPassPhrase),
                    Ok(ref val) if val.is_null() => Ok(WalletPassPhrase),
                    Ok(other) => Err(Error::Returned(format!(
                        "walletpassphrase expected null, got: {}",
                        other
                    ))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletpassphrasechange`
#[macro_export]
macro_rules! impl_client_v17__walletpassphrasechange {
    () => {
        impl Client {
            pub fn wallet_passphrase_change(
                &self,
                old_passphrase: &str,
                new_passphrase: &str,
            ) -> Result<WalletPassPhraseChange> {
                match self
                    .call("walletpassphrasechange", &[old_passphrase.into(), new_passphrase.into()])
                {
                    Ok(serde_json::Value::Null) => Ok(WalletPassPhraseChange),
                    Ok(ref val) if val.is_null() => Ok(WalletPassPhraseChange),
                    Ok(other) => Err(Error::Returned(format!(
                        "walletpassphrasechange expected null, got: {}",
                        other
                    ))),
                    Err(e) => Err(e.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importmulti`
#[macro_export]
macro_rules! impl_client_v17__importmulti {
    () => {
        impl Client {
            pub fn import_multi(
                &self,
                requests: &[ImportMultiRequest],
                options: Option<&ImportMultiOptions>,
            ) -> Result<ImportMulti> {
                let mut params = vec![serde_json::to_value(requests)?];

                if let Some(opts) = options {
                    if opts != &ImportMultiOptions::default() {
                        params.push(serde_json::to_value(opts)?);
                    }
                }
                self.call("importmulti", &params)
            }
        }
    };
}
