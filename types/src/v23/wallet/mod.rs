// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v23` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

mod error;
mod into;

use bitcoin::Transaction;
use serde::{Deserialize, Serialize};

pub use self::error::GetTransactionError;
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
#[serde(deny_unknown_fields)]
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

/// Result of the JSON-RPC method `getwalletinfo`.
///
/// > getwalletinfo
/// >
/// > Returns an object containing various wallet state info.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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

/// The `scanning` field of the `getwalletinfo` RPC in v23.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GetWalletInfoScanning {
    /// Scanning details.
    Details { duration: u64, progress: f64 },
    /// Not scanning (false).
    NotScanning(bool),
}
