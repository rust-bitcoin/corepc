// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Wallet ==` section of the
//! API docs of Bitcoin Core `v0.17`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_bitreq_client!` macro to define a `Client`.

/// Implements Bitcoin Core JSON-RPC API method `abandontransaction`.
#[macro_export]
macro_rules! impl_client_v17__abandon_transaction {
    () => {
        impl Client {
            pub fn abandon_transaction(&self, txid: Txid) -> Result<()> {
                match self.call("abandontransaction", &[into_json(txid)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `abortrescan`.
#[macro_export]
macro_rules! impl_client_v17__abort_rescan {
    () => {
        impl Client {
            pub fn abort_rescan(&self) -> Result<AbortRescan> { self.call("abortrescan", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `addmultisigaddress`.
#[macro_export]
macro_rules! impl_client_v17__add_multisig_address {
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
macro_rules! impl_client_v17__backup_wallet {
    () => {
        impl Client {
            pub fn backup_wallet(&self, destination: &Path) -> Result<()> {
                match self.call("backupwallet", &[into_json(destination)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `bumpfee`.
#[macro_export]
macro_rules! impl_client_v17__bump_fee {
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
macro_rules! impl_client_v17__create_wallet {
    () => {
        impl Client {
            pub fn create_wallet(&self, wallet: &str) -> Result<CreateWallet> {
                self.call("createwallet", &[wallet.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `dumpprivkey`.
#[macro_export]
macro_rules! impl_client_v17__dump_priv_key {
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
macro_rules! impl_client_v17__dump_wallet {
    () => {
        impl Client {
            // filename is either absolute or relative to bitcoind.
            pub fn dump_wallet(&self, filename: &Path) -> Result<DumpWallet> {
                self.call("dumpwallet", &[into_json(filename)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `encryptwallet`.
#[macro_export]
macro_rules! impl_client_v17__encrypt_wallet {
    () => {
        impl Client {
            // filename is either absolute or relative to bitcoind.
            pub fn encrypt_wallet(&self, passphrase: &str) -> Result<EncryptWallet> {
                self.call("encryptwallet", &[into_json(passphrase)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getaddressesbylabel`.
#[macro_export]
macro_rules! impl_client_v17__get_addresses_by_label {
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
macro_rules! impl_client_v17__get_address_info {
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
macro_rules! impl_client_v17__get_balance {
    () => {
        impl Client {
            pub fn get_balance(&self) -> Result<GetBalance> { self.call("getbalance", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getnewaddress`.
#[macro_export]
macro_rules! impl_client_v17__get_new_address {
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
macro_rules! impl_client_v17__get_raw_change_address {
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
macro_rules! impl_client_v17__get_received_by_address {
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
macro_rules! impl_client_v17__get_transaction {
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
macro_rules! impl_client_v17__get_unconfirmed_balance {
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
macro_rules! impl_client_v17__get_wallet_info {
    () => {
        impl Client {
            pub fn get_wallet_info(&self) -> Result<GetWalletInfo> {
                self.call("getwalletinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importaddress`.
#[macro_export]
macro_rules! impl_client_v17__import_address {
    () => {
        impl Client {
            pub fn import_address(&self, address: &Address) -> Result<()> {
                match self.call("importaddress", &[into_json(address)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importmulti`.
#[macro_export]
macro_rules! impl_client_v17__import_multi {
    () => {
        impl Client {
            pub fn import_multi(&self, requests: &[ImportMultiRequest]) -> Result<ImportMulti> {
                self.call("importmulti", &[into_json(requests)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importprivkey`.
#[macro_export]
macro_rules! impl_client_v17__import_privkey {
    () => {
        impl Client {
            pub fn import_privkey(&self, privkey: &bitcoin::PrivateKey) -> Result<()> {
                match self.call("importprivkey", &[into_json(privkey)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importprunedfunds`.
#[macro_export]
macro_rules! impl_client_v17__import_pruned_funds {
    () => {
        impl Client {
            pub fn import_pruned_funds(
                &self,
                raw_transaction: &str,
                tx_out_proof: &str,
            ) -> Result<()> {
                match self.call(
                    "importprunedfunds",
                    &[into_json(raw_transaction)?, into_json(tx_out_proof)?],
                ) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importpubkey`.
#[macro_export]
macro_rules! impl_client_v17__import_pubkey {
    () => {
        impl Client {
            pub fn import_pubkey(&self, pubkey: &bitcoin::PublicKey) -> Result<()> {
                match self.call("importpubkey", &[into_json(pubkey)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importwallet`.
#[macro_export]
macro_rules! impl_client_v17__import_wallet {
    () => {
        impl Client {
            pub fn import_wallet(&self, filename: &Path) -> Result<()> {
                match self.call("importwallet", &[into_json(filename)?]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `keypoolrefill`.
#[macro_export]
macro_rules! impl_client_v17__key_pool_refill {
    () => {
        impl Client {
            pub fn key_pool_refill(&self) -> Result<()> {
                match self.call("keypoolrefill", &[]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listaddressgroupings`.
#[macro_export]
macro_rules! impl_client_v17__list_address_groupings {
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
macro_rules! impl_client_v17__list_labels {
    () => {
        impl Client {
            pub fn list_labels(&self) -> Result<ListLabels> { self.call("listlabels", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listlockunspent`.
#[macro_export]
macro_rules! impl_client_v17__list_lock_unspent {
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
macro_rules! impl_client_v17__list_received_by_address {
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
macro_rules! impl_client_v17__list_since_block {
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
macro_rules! impl_client_v17__list_transactions {
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
macro_rules! impl_client_v17__list_unspent {
    () => {
        impl Client {
            pub fn list_unspent(&self) -> Result<ListUnspent> { self.call("listunspent", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listwallets`.
#[macro_export]
macro_rules! impl_client_v17__list_wallets {
    () => {
        impl Client {
            pub fn list_wallets(&self) -> Result<ListWallets> { self.call("listwallets", &[]) }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `loadwallet`.
#[macro_export]
macro_rules! impl_client_v17__load_wallet {
    () => {
        impl Client {
            pub fn load_wallet(&self, filename: &str) -> Result<LoadWallet> {
                self.call("loadwallet", &[into_json(filename)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `lockunspent`.
#[macro_export]
macro_rules! impl_client_v17__lock_unspent {
    () => {
        impl Client {
            /// Lock the given list of transaction outputs. Returns true on success.
            ///
            /// This wraps Core RPC: `lockunspent false [{"txid":"..","vout":n},...]`.
            pub fn lock_unspent(&self, outputs: &[(Txid, u32)]) -> Result<LockUnspent> {
                let outs: Vec<_> = outputs
                    .iter()
                    .map(|(txid, vout)| serde_json::json!({"txid": txid, "vout": vout}))
                    .collect();
                self.call("lockunspent", &[into_json(false)?, outs.into()])
            }

            /// Unlock the given list of transaction outputs. Returns true on success.
            ///
            /// This wraps Core RPC: `lockunspent true [{"txid":"..","vout":n},...]`.
            pub fn unlock_unspent(&self, outputs: &[(Txid, u32)]) -> Result<LockUnspent> {
                let outs: Vec<_> = outputs
                    .iter()
                    .map(|(txid, vout)| serde_json::json!({"txid": txid, "vout": vout}))
                    .collect();
                self.call("lockunspent", &[into_json(true)?, outs.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `removeprunedfunds`.
#[macro_export]
macro_rules! impl_client_v17__remove_pruned_funds {
    () => {
        impl Client {
            pub fn remove_pruned_funds(&self, txid: Txid) -> Result<()> {
                self.call("removeprunedfunds", &[into_json(txid)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `rescanblockchain`.
#[macro_export]
macro_rules! impl_client_v17__rescan_blockchain {
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
macro_rules! impl_client_v17__send_many {
    () => {
        impl Client {
            pub fn send_many(&self, amounts: BTreeMap<Address, Amount>) -> Result<SendMany> {
                let dummy = ""; // Must be set to "" for backwards compatibility.
                let amount_btc: BTreeMap<String, f64> = amounts
                    .into_iter()
                    .map(|(addr, amount)| (addr.to_string(), amount.to_btc()))
                    .collect();
                self.call("sendmany", &[into_json(dummy)?, into_json(amount_btc)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `sendtoaddress`.
#[macro_export]
macro_rules! impl_client_v17__send_to_address {
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

/// Implements Bitcoin Core JSON-RPC API method `sethdseed`.
#[macro_export]
macro_rules! impl_client_v17__set_hd_seed {
    () => {
        impl Client {
            pub fn set_hd_seed(&self) -> Result<()> {
                match self.call("sethdseed", &[]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `settxfee`.
#[macro_export]
macro_rules! impl_client_v17__set_tx_fee {
    () => {
        impl Client {
            pub fn set_tx_fee(&self, fee_rate: bitcoin::FeeRate) -> Result<SetTxFee> {
                let fee_rate_btc_kvb = fee_rate.to_sat_per_vb_floor() as f64 / 100_000.0;
                self.call("settxfee", &[fee_rate_btc_kvb.into()])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `signmessage`.
#[macro_export]
macro_rules! impl_client_v17__sign_message {
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
macro_rules! impl_client_v17__sign_raw_transaction_with_wallet {
    () => {
        impl Client {
            // `hexstring`: The transaction hex string.
            pub fn sign_raw_transaction_with_wallet(
                &self,
                tx: &bitcoin::Transaction,
            ) -> Result<SignRawTransactionWithWallet> {
                let hex = bitcoin::consensus::encode::serialize_hex(tx);
                self.call("signrawtransactionwithwallet", &[into_json(hex)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `unloadwallet`.
#[macro_export]
macro_rules! impl_client_v17__unload_wallet {
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

/// Implements Bitcoin Core JSON-RPC API method `walletpassphrase`.
#[macro_export]
macro_rules! impl_client_v17__wallet_passphrase {
    () => {
        impl Client {
            pub fn wallet_passphrase(&self, passphrase: &str, timeout: u64) -> Result<()> {
                match self.call("walletpassphrase", &[passphrase.into(), timeout.into()]) {
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
macro_rules! impl_client_v17__wallet_create_funded_psbt {
    () => {
        impl Client {
            pub fn wallet_create_funded_psbt(
                &self,
                inputs: Vec<WalletCreateFundedPsbtInput>,
                outputs: Vec<BTreeMap<Address, Amount>>,
            ) -> Result<WalletCreateFundedPsbt> {
                // Convert outputs: Vec<BTreeMap<Address, Amount>> to Vec<BTreeMap<String, f64>>
                let outputs_json: Vec<_> = outputs
                    .into_iter()
                    .map(|map| {
                        map.into_iter()
                            .map(|(addr, amt)| (addr.to_string(), amt.to_btc()))
                            .collect::<BTreeMap<_, _>>()
                    })
                    .collect();
                self.call("walletcreatefundedpsbt", &[into_json(inputs)?, into_json(outputs_json)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletlock`.
#[macro_export]
macro_rules! impl_client_v17__wallet_lock {
    () => {
        impl Client {
            pub fn wallet_lock(&self) -> Result<()> {
                match self.call("walletlock", &[]) {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletpassphrasechange`.
#[macro_export]
macro_rules! impl_client_v17__wallet_passphrase_change {
    () => {
        impl Client {
            pub fn wallet_passphrase_change(
                &self,
                old_passphrase: &str,
                new_passphrase: &str,
            ) -> Result<()> {
                match self
                    .call("walletpassphrasechange", &[old_passphrase.into(), new_passphrase.into()])
                {
                    Ok(serde_json::Value::Null) => Ok(()),
                    Ok(res) => Err(Error::Returned(res.to_string())),
                    Err(err) => Err(err.into()),
                }
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletprocesspsbt`.
#[macro_export]
macro_rules! impl_client_v17__wallet_process_psbt {
    () => {
        impl Client {
            pub fn wallet_process_psbt(&self, psbt: &bitcoin::Psbt) -> Result<WalletProcessPsbt> {
                // Core expects the PSBT as a base64 string argument (same representation
                // used by `finalizepsbt`). Serializing the struct with `into_json` produced
                // an object which Core rejected ("Expected type string, got object").
                let psbt = format!("{}", psbt);
                self.call("walletprocesspsbt", &[psbt.into()])
            }
        }
    };
}
