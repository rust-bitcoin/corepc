// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.19` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

mod error;
mod into;

use bitcoin::Transaction;
use serde::{Deserialize, Serialize};

pub use self::error::GetBalancesError;
use super::{Bip125Replaceable, GetTransactionDetail, GetTransactionError, GetWalletInfoError};

/// Result of the JSON-RPC method `getbalances`.
///
/// > getbalances
/// >
/// > Returns an object with all balances in BTC.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBalances {
    /// Balances from outputs that the wallet can sign.
    pub mine: GetBalancesMine,
    #[serde(rename = "watchonly")]
    pub watch_only: Option<GetBalancesWatchOnly>,
}

/// Balances from outputs that the wallet can sign. Part of `getbalances`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBalancesMine {
    /// Trusted balance (outputs created by the wallet or confirmed outputs).
    pub trusted: f64,
    /// Untrusted pending balance (outputs created by others that are in the mempool).
    pub untrusted_pending: f64,
    /// Balance from immature coinbase outputs.
    pub immature: f64,
    /// Balance from coins sent to addresses that were previously spent from (potentially privacy violating).
    ///
    /// Only present if `avoid_reuse` is set.
    pub used: Option<f64>,
}

/// Hash and height of the block this information was generated on. Part of `getbalances`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetBalancesWatchOnly {
    /// Trusted balance (outputs created by the wallet or confirmed outputs).
    pub trusted: f64,
    /// Untrusted pending balance (outputs created by others that are in the mempool).
    pub untrusted_pending: f64,
    /// Balance from immature coinbase outputs.
    pub immature: f64,
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
    /// Whether we consider the outputs of this unconfirmed transaction safe to spend.
    pub trusted: Option<bool>,
    /// The block hash.
    #[serde(rename = "blockhash")]
    pub block_hash: Option<String>,
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
    /// The transaction time in seconds since epoch (1 Jan 1970 GMT).
    pub time: u32,
    /// The time received in seconds since epoch (1 Jan 1970 GMT).
    #[serde(rename = "timereceived")]
    pub time_received: u32,
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
/// > Returns an object containing various wallet state info.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetWalletInfo {
    /// The wallet name.
    #[serde(rename = "walletname")]
    pub wallet_name: String,
    /// The wallet version.
    #[serde(rename = "walletversion")]
    pub wallet_version: i64,
    /// The total confirmed balance of the wallet in BTC. (DEPRECATED)
    pub balance: f64,
    /// The total unconfirmed balance of the wallet in BTC. (DEPRECATED)
    pub unconfirmed_balance: f64,
    /// The total immature balance of the wallet in BTC. (DEPRECATED)
    pub immature_balance: f64,
    /// The total number of transactions in the wallet
    #[serde(rename = "txcount")]
    pub tx_count: i64,
    /// The timestamp (seconds since Unix epoch) of the oldest pre-generated key in the key pool.
    #[serde(rename = "keypoololdest")]
    pub keypool_oldest: i64,
    /// How many new keys are pre-generated (only counts external keys).
    #[serde(rename = "keypoolsize")]
    pub keypool_size: i64,
    /// How many new keys are pre-generated for internal use (used for change outputs, only appears
    /// if the wallet is using this feature, otherwise external keys are used).
    #[serde(rename = "keypoolsize_hd_internal")]
    pub keypool_size_hd_internal: i64,
    /// The timestamp in seconds since epoch (midnight Jan 1 1970 GMT) that the wallet is unlocked
    /// for transfers, or 0 if the wallet is locked.
    pub unlocked_until: Option<u32>,
    /// The transaction fee configuration, set in BTC/kB.
    #[serde(rename = "paytxfee")]
    pub pay_tx_fee: f64,
    /// The Hash160 of the HD seed (only present when HD is enabled).
    #[serde(rename = "hdseedid")]
    pub hd_seed_id: Option<String>,
    /// If privatekeys are disabled for this wallet (enforced watch-only wallet).
    pub private_keys_enabled: bool,
    /// Whether this wallet tracks clean/dirty coins in terms of reuse.
    pub avoid_reuse: bool,
    /// Current scanning details, or false if no scan is in progress.
    pub scanning: GetWalletInfoScanning,
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

/// Result of the JSON-RPC method `setwalletflag`.
///
/// > setwalletflag "flag" ( value )
/// >
/// > Change the state of the given wallet flag for a wallet.
/// >
/// > Arguments:
/// > 1. flag     (string, required) The name of the flag to change. Current available flags: avoid_reuse
/// > 2. value    (boolean, optional, default=true) The new state.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SetWalletFlag {
    /// The name of the flag that was modified.
    pub flag_name: String,
    /// The new state of the flag.
    pub flag_state: bool,
    /// Any warnings associated with the change. (Always optional, but docs only state this from v24).
    pub warnings: Option<String>,
}
