// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v26` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

mod error;
mod into;

use bitcoin::Transaction;
use serde::{Deserialize, Serialize};

pub use self::error::{GetBalancesError, GetTransactionError, LastProcessedBlockError};
pub use super::{
    Bip125Replaceable, GetBalancesMine, GetBalancesWatchOnly, GetTransactionDetail,
    GetTransactionDetailError,
};

/// Result of the JSON-RPC method `createwallet`.
///
/// > createwallet "wallet_name" ( disable_private_keys blank "passphrase" avoid_reuse descriptors load_on_startup external_signer )
///
/// > Creates and loads a new wallet.
///
/// > Arguments:
/// > 1. wallet_name             (string, required) The name for the new wallet. If this is a path, the wallet will be created at the path location.
/// > 2. disable_private_keys    (boolean, optional, default=false) Disable the possibility of private keys (only watchonlys are possible in this mode).
/// > 3. blank                   (boolean, optional, default=false) Create a blank wallet. A blank wallet has no keys or HD seed. One can be set using sethdseed.
/// > 4. passphrase              (string, optional) Encrypt the wallet with this passphrase.
/// > 5. avoid_reuse             (boolean, optional, default=false) Keep track of coin reuse, and treat dirty and clean coins differently with privacy considerations in mind.
/// > 6. descriptors             (boolean, optional, default=true) Create a native descriptor wallet. The wallet will use descriptors internally to handle address creation. Setting to "false" will create a legacy wallet; however, the legacy wallet type is being deprecated and support for creating and opening legacy wallets will be removed in the future.
/// > 7. load_on_startup         (boolean, optional) Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
/// > 8. external_signer         (boolean, optional, default=false) Use an external signer such as a hardware wallet. Requires -signer to be configured. Wallet creation will fail if keys cannot be fetched. Requires disable_private_keys and descriptors set to true.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateWallet {
    /// The wallet name if created successfully.
    ///
    /// If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating and loading the wallet.
    pub warnings: Option<Vec<String>>,
}

/// Result of the JSON-RPC method `getbalances`.
///
/// > getbalances
/// >
/// > Returns an object with all balances in BTC.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GetBalances {
    /// Balances from outputs that the wallet can sign.
    pub mine: GetBalancesMine,
    #[serde(rename = "watchonly")]
    pub watch_only: Option<GetBalancesWatchOnly>,
    /// Hash and height of the block this information was generated on. v26 and later only.
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: Option<LastProcessedBlock>,
}

/// Result of the JSON-RPC method `gettransaction`.
///
/// > gettransaction "txid" ( include_watchonly )
/// >
/// > Get detailed information about in-wallet transaction `<txid>`.
/// >
/// > Arguments:
/// > 1. txid                 (string, required) The transaction id.
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
    /// Hash and height of the block this information was generated on. v26 and later only.
    #[serde(rename = "lastprocessedblock")]
    pub last_processed_block: Option<LastProcessedBlock>,
}

/// Item returned as part of of `gettransaction`. v26 and later only.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LastProcessedBlock {
    /// Hash of the block this information was generated on.
    pub hash: String,
    /// Height of the block this information was generated on.
    pub height: i64,
}

/// Result of the JSON-RPC method `loadwallet`.
///
/// > loadwallet "filename" ( load_on_startup )
///
/// > Loads a wallet from a wallet file or directory.
/// > Note that all wallet command-line options used when starting bitcoind will be
/// > applied to the new wallet.
///
/// > Arguments:
/// > 1. filename           (string, required) The wallet directory or .dat file.
/// > 2. load_on_startup    (boolean, optional) Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LoadWallet {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    pub warnings: Option<Vec<String>>,
}

/// Result of the JSON-RPC method `unloadwallet`.
///
/// > unloadwallet ( "wallet_name" load_on_startup )
/// >
/// > Unloads the wallet referenced by the request endpoint, otherwise unloads the wallet specified in the argument.
/// > Specifying the wallet name on a wallet endpoint is invalid.
/// >
/// > Arguments:
/// > 1. wallet_name        (string, optional, default=the wallet name from the RPC endpoint) The name of the wallet to unload. If provided both here and in the RPC endpoint, the two must be identical.
/// > 2. load_on_startup    (boolean, optional) Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UnloadWallet {
    /// Warning messages, if any, related to loading the wallet.
    pub warnings: Option<Vec<String>>,
}
