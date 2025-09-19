// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v24` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

mod error;
mod into;

use bitcoin::Transaction;
use serde::{Deserialize, Serialize};

pub use self::error::{
    GetTransactionError, ListSinceBlockError, SendAllError, TransactionItemError,
};
pub use super::{
    Bip125Replaceable, GetTransactionDetailError, ListUnspentItemError, TransactionCategory,
};

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
    /// The hash of serialized transaction, including witness data. v24 and later only.
    pub wtxid: Option<String>,
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
    /// Only if 'category' is 'received'. List of parent descriptors for the output script of this
    /// coin. v24 and later only.
    #[serde(rename = "parent_descs")]
    pub parent_descriptors: Option<Vec<String>>,
    /// Transaction details.
    pub details: Vec<GetTransactionDetail>,
    /// Raw data for transaction.
    pub hex: String,
    /// The decoded transaction (only present when `verbose` is passed). v19 and later only.
    pub decoded: Option<Transaction>,
}

/// Transaction detail. Part of the `gettransaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetTransactionDetail {
    /// Only returns true if imported addresses were involved in transaction. v20 and later only.
    #[serde(rename = "involvesWatchonly")]
    pub involves_watch_only: Option<bool>,
    /// DEPRECATED. The account name involved in the transaction, can be "" for the default account.
    pub account: Option<String>, // Docs are wrong, this is not documented as optional.
    /// The bitcoin address involved in the transaction.
    pub address: String,
    /// The category, either 'send' or 'receive'.
    pub category: TransactionCategory,
    ///  The amount in BTC.
    pub amount: f64,
    /// A comment for the address/transaction, if any.
    pub label: Option<String>,
    /// the vout value.
    pub vout: u32,
    /// The amount of the fee.
    ///
    /// This is negative and only available for the 'send' category of transactions.
    pub fee: Option<f64>,
    /// If the transaction has been abandoned (inputs are respendable).
    ///
    /// Only available for the 'send' category of transactions.
    pub abandoned: Option<bool>,
    /// Only if 'category' is 'received'. List of parent descriptors for the output script of this
    /// coin. v24 and later only.
    #[serde(rename = "parent_descs")]
    pub parent_descriptors: Option<Vec<String>>,
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
    pub address: Option<String>,
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
    /// The hash of serialized transaction, including witness data.
    pub wtxid: String,
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
    /// Only if 'category' is 'received'. List of parent descriptors for the scriptPubKey of this coin.
    #[serde(rename = "parent_descs")]
    pub parent_descriptors: Option<Vec<String>>,
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

/// Result of the JSON-RPC method `listunspent`.
///
/// > listunspent ( minconf maxconf  ["addresses",...] `[include_unsafe]` `[query_options]`)
/// >
/// > Returns array of unspent transaction outputs
/// > with between minconf and maxconf (inclusive) confirmations.
/// > Optionally filter to only include txouts paid to specified addresses.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListUnspent(pub Vec<ListUnspentItem>);

/// Unspent transaction output, part of `listunspent`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ListUnspentItem {
    /// The transaction id.
    pub txid: String,
    /// The vout value.
    pub vout: i64,
    /// The bitcoin address of the transaction.
    pub address: String,
    /// The associated label, or "" for the default label.
    pub label: String,
    /// The script key.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: String,
    /// The transaction amount in BTC.
    pub amount: f64,
    /// The number of confirmations.
    pub confirmations: i64,
    /// The redeemScript if scriptPubKey is P2SH.
    #[serde(rename = "redeemScript")]
    pub redeem_script: Option<String>,
    /// Whether we have the private keys to spend this output.
    pub spendable: bool,
    /// Whether we know how to spend this output, ignoring the lack of keys.
    pub solvable: bool,
    /// A descriptor for spending this output (only when solvable)
    #[serde(rename = "desc")]
    pub descriptor: Option<String>,
    /// Whether this output is considered safe to spend. Unconfirmed transactions from outside keys
    /// and unconfirmed replacement transactions are considered unsafe and are not eligible for
    /// spending by fundrawtransaction and sendtoaddress.
    pub safe: bool,
    /// List of parent descriptors for the scriptPubKey of this coin. v24 and later only.
    #[serde(rename = "parent_descs")]
    pub parent_descriptors: Option<Vec<String>>,
}

/// Result of JSON-RPC method `migratewallet`.
///
/// > migratewallet ( "wallet_name" "passphrase" )
/// >
/// > EXPERIMENTAL warning: This call may not work as expected and may be changed in future releases
/// >
/// > Migrate the wallet to a descriptor wallet.
/// > A new wallet backup will need to be made.
/// >
/// > The migration process will create a backup of the wallet before migrating. This backup
/// > file will be named {wallet name}-{timestamp}.legacy.bak and can be found in the directory
/// > for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
/// > Encrypted wallets must have the passphrase provided as an argument to this call.
/// >
/// > Arguments:
/// > 1. wallet_name    (string, optional, default=the wallet name from the RPC endpoint) The name of the wallet to migrate. If provided both here and in the RPC endpoint, the two must be identical.
/// > 2. passphrase     (string) The wallet passphrase
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct MigrateWallet {
    /// The name of the primary migrated wallet
    pub wallet_name: String,
    /// The name of the migrated wallet containing the watchonly scripts
    pub watchonly_name: Option<String>,
    /// The name of the migrated wallet containing solvable but not watched scripts
    pub solvables_name: Option<String>,
    /// The location of the backup of the original wallet
    pub backup_path: String,
}

/// Result of JSON-RPC method `sendall`.
///
/// > sendall ["address",{"address":amount,...},...] ( conf_target "estimate_mode" fee_rate options )
/// >
/// > EXPERIMENTAL warning: this call may be changed in future releases.
/// >
/// > Spend the value of all (or specific) confirmed UTXOs in the wallet to one or more recipients.
/// > Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
/// > If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
/// >
/// > Arguments:
/// > 1. recipients                       (json array, required) The sendall destinations. Each address may only appear once.
/// >                                     Optionally some recipients can be specified with an amount to perform payments, but at least one address must appear without a specified amount.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendAll {
    /// If the transaction has a complete set of signatures.
    pub complete: bool,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    pub txid: Option<String>,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s).
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction.
    pub psbt: Option<String>,
}

/// Result of JSON-RPC method `simulaterawtransaction`.
///
/// > simulaterawtransaction ( ["rawtx",...] {"include_watchonly":bool,...} )
/// >
/// > Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
/// >
/// > Arguments:
/// > 1. rawtxs                            (json array, optional) An array of hex strings of raw transactions.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SimulateRawTransaction {
    /// The wallet balance change (negative means decrease).
    pub balance_change: f64,
}
