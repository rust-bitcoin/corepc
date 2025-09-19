// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v23` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

mod error;
mod into;

use bitcoin::Transaction;
use serde::{Deserialize, Serialize};

pub use self::error::{GetTransactionError, ListSinceBlockError, TransactionItemError};
pub use super::{
    AddMultisigAddressError, Bip125Replaceable, GetTransactionDetail, GetTransactionDetailError,
    GetWalletInfoError,
};

/// Result of the JSON-RPC method `addmultisigaddress`.
///
/// > addmultisigaddress nrequired ["key",...] ( "label" "address_type" )
/// >
/// > Add a nrequired-to-sign multisignature address to the wallet. Requires a new wallet backup.
/// > Each key is a Bitcoin address or hex-encoded public key.
/// > This functionality is only intended for use with non-watchonly addresses.
/// > See `importaddress` for watchonly p2sh address support.
/// > If 'label' is specified, assign address to that label.
///
/// > Arguments:
/// > 1. nrequired                      (numeric, required) The number of required signatures out of the n keys or addresses.
/// > 2. "keys"                         (string, required) A json array of bitcoin addresses or hex-encoded public keys
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct AddMultisigAddress {
    /// The value of the new multisig address.
    pub address: String,
    /// The string value of the hex-encoded redemption script.
    #[serde(rename = "redeemScript")]
    pub redeem_script: String,
    /// The descriptor for this multisig.
    pub descriptor: String,
    /// Any warnings resulting from the creation of this multisig.
    pub warnings: Option<Vec<String>>,
}

/// Result of the JSON-RPC method `gettransaction`.
///
/// > gettransaction "txid" ( include_watchonly )
/// >
/// > Get detailed information about in-wallet transaction `<txid>`
/// >
/// > Arguments:
/// > 1. txid                 (string, required) The transaction id
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransaction {
    /// The transaction amount in BTC.
    pub amount: f64,
    /// The amount of the fee in BTC.
    ///
    /// This is negative and only available for the 'send' category of transactions.
    pub fee: Option<f64>,
    /// The number of confirmations.
    pub confirmations: i64,
    /// Only present if the transaction's only input is a coinbase one. v20 and later only.
    pub generated: Option<bool>,
    /// Whether we consider the outputs of this unconfirmed transaction safe to spend.
    pub trusted: Option<bool>,
    /// The block hash.
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// The block height containing the transaction. v20 and later only.
    #[serde(rename = "blockheight")]
    pub block_height: Option<i64>,
    /// The index of the transaction in the block that includes it.
    #[serde(rename = "blockindex")]
    pub block_index: Option<i64>,
    /// The time in seconds since epoch (1 Jan 1970 GMT).
    #[serde(rename = "blocktime")]
    pub block_time: Option<u32>,
    /// The transaction id.
    pub txid: String,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    #[serde(rename = "walletconflicts")]
    pub wallet_conflicts: Vec<String>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced. v23 and later only.
    pub replaced_by_txid: Option<String>,
    /// Only if 'category' is 'send'. The txid if this tx replaces another. v23 and later only.
    pub replaces_txid: Option<String>,
    /// If a comment to is associated with the transaction. v23 and later only.
    pub to: Option<String>,
    /// The transaction time in seconds since epoch (1 Jan 1970 GMT).
    pub time: u32,
    /// The time received in seconds since epoch (1 Jan 1970 GMT).
    #[serde(rename = "timereceived")]
    pub time_received: u32,
    /// If a comment is associated with the transaction, only present if not empty. v20 to v24 only.
    pub comment: Option<String>,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee);
    /// may be unknown for unconfirmed transactions not in the mempool
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: Bip125Replaceable,
    /// Transaction details.
    pub details: Vec<GetTransactionDetail>,
    /// Raw data for transaction.
    pub hex: String,
    /// The decoded transaction (only present when `verbose` is passed). v19 and later only.
    pub decoded: Option<Transaction>,
}

/// Result of the JSON-RPC method `getwalletinfo`.
///
/// > getwalletinfo
/// >
/// > Returns an object containing various wallet state info.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetWalletInfo {
    /// the wallet name
    #[serde(rename = "walletname")]
    pub wallet_name: String,
    /// the wallet version
    #[serde(rename = "walletversion")]
    pub wallet_version: i64,
    /// the database format (bdb or sqlite)
    pub format: String,
    /// DEPRECATED. Identical to getbalances().mine.trusted
    pub balance: f64,
    /// DEPRECATED. Identical to getbalances().mine.untrusted_pending
    pub unconfirmed_balance: f64,
    /// DEPRECATED. Identical to getbalances().mine.immature
    pub immature_balance: f64,
    /// the total number of transactions in the wallet
    #[serde(rename = "txcount")]
    pub tx_count: i64,
    /// the UNIX epoch time of the oldest pre-generated key in the key pool. Legacy wallets only.
    #[serde(rename = "keypoololdest")]
    pub keypool_oldest: Option<i64>,
    /// how many new keys are pre-generated (only counts external keys)
    #[serde(rename = "keypoolsize")]
    pub keypool_size: i64,
    /// how many new keys are pre-generated for internal use (used for change outputs, only appears if the wallet is using this feature, otherwise external keys are used)
    #[serde(rename = "keypoolsize_hd_internal")]
    pub keypool_size_hd_internal: Option<i64>,
    /// the UNIX epoch time until which the wallet is unlocked for transfers, or 0 if the wallet is locked (only present for passphrase-encrypted wallets)
    pub unlocked_until: Option<u32>,
    /// the transaction fee configuration, set in BTC/kvB
    #[serde(rename = "paytxfee")]
    pub pay_tx_fee: f64,
    /// the Hash160 of the HD seed (only present when HD is enabled)
    #[serde(rename = "hdseedid")]
    pub hd_seed_id: Option<String>,
    /// false if privatekeys are disabled for this wallet (enforced watch-only wallet)
    pub private_keys_enabled: bool,
    /// whether this wallet tracks clean/dirty coins in terms of reuse
    pub avoid_reuse: bool,
    /// current scanning details, or false if no scan is in progress
    pub scanning: GetWalletInfoScanning,
    /// whether this wallet uses descriptors for scriptPubKey management
    pub descriptors: bool,
    /// whether this wallet is configured to use an external signer such as a hardware wallet
    pub external_signer: bool,
}

/// Current scanning details. Part of `getwalletinfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GetWalletInfoScanning {
    /// Scanning details.
    Details { duration: u64, progress: f64 },
    /// Not scanning (false).
    NotScanning(bool),
}

/// Result of the JSON-RPC method `listsinceblock`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListSinceBlock {
    /// All the transactions.
    pub transactions: Vec<TransactionItem>,
    /// Only present if `include_removed=true`.
    ///
    /// Note: transactions that were re-added in the active chain will appear as-is in this array,
    /// and may thus have a positive confirmation count.
    pub removed: Vec<TransactionItem>,
    /// The hash of the block (target_confirmations-1) from the best block on the main chain.
    ///
    /// This is typically used to feed back into listsinceblock the next time you call it. So you
    /// would generally use a target_confirmations of say 6, so you will be continually
    /// re-notified of transactions until they've reached 6 confirmations plus any new ones.
    #[serde(rename = "lastblock")]
    pub last_block: String,
}

/// Transaction item. Part of `listsinceblock`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct TransactionItem {
    /// Only returns true if imported addresses were involved in transaction.
    #[serde(rename = "involvesWatchonly")]
    pub involves_watch_only: Option<bool>,
    /// The bitcoin address of the transaction.
    pub address: String,
    /// The transaction category.
    pub category: super::TransactionCategory,
    /// The amount in BTC.
    ///
    /// This is negative for the 'send' category, and is positive for all other categories.
    pub amount: f64,
    /// The vout value.
    pub vout: i64,
    /// The amount of the fee in BTC.
    ///
    /// This is negative and only available for the 'send' category of transactions.
    pub fee: Option<f64>,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// Only present if transaction only input is a coinbase one.
    pub generated: Option<bool>,
    /// Only present if we consider transaction to be trusted and so safe to spend from.
    pub trusted: Option<bool>,
    /// The block hash containing the transaction.
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
    /// The block height containing the transaction.
    #[serde(rename = "blockheight")]
    pub block_height: Option<i64>,
    /// The index of the transaction in the block that includes it.
    #[serde(rename = "blockindex")]
    pub block_index: Option<i64>,
    /// The block time expressed in UNIX epoch time.
    #[serde(rename = "blocktime")]
    pub block_time: Option<u32>,
    /// The transaction id.
    pub txid: String,
    /// Conflicting transaction ids.
    #[serde(rename = "walletconflicts")]
    pub wallet_conflicts: Vec<String>,
    /// The txid if this tx was replaced.
    pub replaced_by_txid: Option<String>,
    /// The txid if this tx replaces one.
    pub replaces_txid: Option<String>,
    /// If a comment is associated with the transaction, only present if not empty.
    pub comment: Option<String>,
    /// If a comment to is associated with the transaction.
    pub to: Option<String>,
    /// The transaction time expressed in UNIX epoch time.
    pub time: u32,
    /// The time received expressed in UNIX epoch time.
    #[serde(rename = "timereceived")]
    pub time_received: u32,
    /// ("yes|no|unknown") Whether this transaction could be replaced due to BIP125 (replace-by-fee);
    /// may be unknown for unconfirmed transactions not in the mempool
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: Bip125Replaceable,
    /// 'true' if the transaction has been abandoned (inputs are respendable). Only available for the
    /// 'send' category of transactions.
    pub abandoned: Option<bool>,
    /// A comment for the address/transaction, if any.
    pub label: Option<String>,
}

/// Result of the JSON-RPC method `listtransactions`.
///
/// > listtransactions (label count skip include_watchonly)
/// >
/// > If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
/// >
/// > Returns up to 'count' most recent transactions skipping the first 'from' transactions.
/// > Note that the "account" argument and "otheraccount" return value have been removed in V0.17. To use this RPC with an "account" argument, restart
/// > bitcoind with -deprecatedrpc=accounts
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListTransactions(pub Vec<TransactionItem>);

/// Result of the JSON-RPC method `restorewallet`.
///
/// > restorewallet "wallet_name" "backup_file" ( load_on_startup )
/// >
/// > Restore and loads a wallet from backup.
/// >
/// > Arguments:
/// > 1. wallet_name        (string, required) The name that will be applied to the restored wallet
/// > 2. backup_file        (string, required) The backup file that will be used to restore the wallet.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct RestoreWallet {
    /// The wallet name if restored successfully.
    pub name: String,
    /// Warning message if wallet was not loaded cleanly.
    pub warning: Option<String>,
}
