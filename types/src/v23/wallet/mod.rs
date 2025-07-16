// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v23` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

mod error;
mod into;

use bitcoin::Transaction;
use serde::{Deserialize, Serialize};

pub use self::error::GetTransactionError;
pub use super::{Bip125Replaceable, GetTransactionDetail, GetTransactionDetailError};

/// Result of the JSON-RPC method `gettransaction`.
///
/// > gettransaction "txid" ( include_watchonly )
/// >
/// > Get detailed information about in-wallet transaction `<txid>`
/// >
/// > Arguments:
/// > 1. txid                 (string, required) The transaction id
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct RestoreWallet {
    /// The wallet name if restored successfully.
    pub name: String,
    /// Warning message if wallet was not loaded cleanly.
    pub warning: Option<String>,
}
