// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.21` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

mod error;
mod into;

use serde::{Deserialize, Serialize};

pub use self::error::{PsbtBumpFeeError, SendError};
pub use super::GetWalletInfoError;

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
    /// The database format (bdb or sqlite).
    pub format: String,
    /// The total confirmed balance of the wallet in BTC. (DEPRECATED)
    pub balance: f64,
    /// The total unconfirmed balance of the wallet in BTC. (DEPRECATED)
    pub unconfirmed_balance: f64,
    /// The total immature balance of the wallet in BTC. (DEPRECATED)
    pub immature_balance: f64,
    /// The total number of transactions in the wallet
    #[serde(rename = "txcount")]
    pub tx_count: i64,
    /// The UNIX epoch time of the oldest pre-generated key in the key pool. Legacy wallets only.
    #[serde(rename = "keypoololdest")]
    pub keypool_oldest: i64,
    /// How many new keys are pre-generated (only counts external keys).
    #[serde(rename = "keypoolsize")]
    pub keypool_size: i64,
    /// How many new keys are pre-generated for internal use (used for change outputs, only appears
    /// if the wallet is using this feature, otherwise external keys are used).
    #[serde(rename = "keypoolsize_hd_internal")]
    pub keypool_size_hd_internal: i64,
    /// The UNIX epoch time until which the wallet is unlocked for transfers, or 0 if the wallet is locked.
    /// Only present for passphrase-encrypted wallets.
    pub unlocked_until: Option<u32>,
    /// The transaction fee configuration, set in BTC/kvB.
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
    /// Whether this wallet uses descriptors for scriptPubKey management.
    pub descriptors: bool,
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

/// Result of JSON-RPC method `importdescriptors`.
///
/// > Import descriptors. This will trigger a rescan of the blockchain based on the earliest
/// > timestamp of all descriptors being imported. Requires a new wallet backup.
/// >
/// > Note: This call can take over an hour to complete if using an early timestamp; during that
/// > time, other rpc calls may report that the imported keys, addresses or scripts exist but
/// > related transactions are still missing.
/// >
/// > Arguments:
/// > 1. requests (json array, required) Data to be imported
/// >    [
/// >      { (json object)
/// >        "desc": "str", (string, required) Descriptor to import.
/// >        "active": bool, (boolean, optional, default=false) Set this descriptor to be the
/// >            active descriptor for the corresponding output type/externality.
/// >        "range": n or \[n,n\], (numeric or array) If a ranged descriptor is used, this
/// >            specifies the end or the range (in the form \[begin,end\]) to import.
/// >        "next_index": n, (numeric) If a ranged descriptor is set to active, this specifies
/// >            the next index to generate addresses from.
/// >        "timestamp": timestamp | "now", (integer / string, required) Time from which to
/// >            start rescanning the blockchain for this descriptor, in UNIX epoch time.
/// >            Use the string "now" to substitute the current synced blockchain time.
/// >            "now" can be specified to bypass scanning, for outputs which are known to never
/// >            have been used, and 0 can be specified to scan the entire blockchain. Blocks up
/// >            to 2 hours before the earliest timestamp of all descriptors being imported will
/// >            be scanned.
/// >        "internal": bool, (boolean, optional, default=false) Whether matching outputs should
/// >            be treated as not incoming payments (e.g. change).
/// >        "label": "str", (string, optional, default='') Label to assign to the address, only
/// >            allowed with internal=false.
/// >      },
/// >      ...
/// >    ]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ImportDescriptors(
    /// Response is an array with the same size as the input that has the execution result.
    pub Vec<ImportDescriptorsResult>,
);

/// Result object for each descriptor import. Part of `importdescriptors`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ImportDescriptorsResult {
    /// Whether the import was successful.
    pub success: bool,
    /// Warnings, if any.
    pub warnings: Option<Vec<String>>,
    /// Error object, if any.
    pub error: Option<serde_json::Value>,
}

/// Result of JSON-RPC method `psbtbumpfee`.
///
/// > Bumps the fee of an opt-in-RBF transaction T, replacing it with a new transaction B.
/// > Returns a PSBT instead of creating and signing a new transaction.
/// > See Bitcoin Core RPC documentation for full details.
///
/// Arguments:
/// 1. txid    (string, required) The txid to be bumped
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PsbtBumpFee {
    /// The base64-encoded unsigned PSBT of the new transaction.
    pub psbt: String,
    /// The fee of the replaced transaction.
    #[serde(rename = "origfee")]
    pub original_fee: f64,
    /// The fee of the new transaction.
    pub fee: f64,
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,
}

/// Result of JSON-RPC method `send`.
///
/// > EXPERIMENTAL warning: this call may be changed in future releases.
/// >
/// > Send a transaction.
/// >
/// > Arguments:
/// > 1. outputs (json array, required) The outputs (key-value pairs), where none of the keys are duplicated.
/// >    That is, each address can only appear once and there can only be one 'data' object.
/// >    For convenience, a dictionary, which holds the key-value pairs directly, is also accepted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct Send {
    /// If the transaction has a complete set of signatures.
    pub complete: bool,
    /// The transaction id for the send.
    pub txid: Option<String>,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s).
    pub hex: Option<String>,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction.
    pub psbt: Option<String>,
}

/// Result of JSON-RPC method `sendmany` when `verbose=false`.
///
/// > sendmany "" {"address":amount}
/// >
/// > Send multiple times. Amounts are double-precision floating point numbers.
/// > Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
/// >
/// > Arguments:
/// > 1. dummy   (string, required) Must be set to "" for backwards compatibility.
/// > 2. amounts (json object, required) The addresses and amounts
/// >    { "address": amount, (numeric or string, required) The bitcoin address is the key, the numeric amount (can be string) in BTC is the value }
/// > ...
/// > 10. verbose (boolean, optional, default=false) If true, return extra infomration about the transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendMany(
    /// The transaction id for the send.
    pub String,
);

/// Result of JSON-RPC method `sendmany` when `verbose=true`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct SendManyVerbose {
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    pub txid: String,
    /// The transaction fee reason.
    pub fee_reason: String,
}

/// Result of the JSON-RPC method `unloadwallet`.
///
/// > unloadwallet ( "wallet_name" load_on_startup )
/// >
/// > Unloads the wallet referenced by the request endpoint, otherwise unloads the wallet specified in the argument.
/// > Specifying the wallet name on a wallet endpoint is invalid.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct UnloadWallet {
    /// Warning messages, if any, related to unloading the wallet.
    pub warning: String,
}

/// Result of JSON-RPC method `upgradewallet`.
///
/// > Upgrade the wallet. Upgrades to the latest version if no version number is specified.
/// > New keys may be generated and a new wallet backup will need to be made.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct UpgradeWallet {
    /// Name of wallet this operation was performed on
    pub wallet_name: String,
    /// Version of wallet before this operation
    pub previous_version: u32,
    /// Version of wallet after this operation
    pub current_version: u32,
    /// Description of result, if no error
    pub result: Option<String>,
    /// Error message (if there is one)
    pub error: Option<String>,
}
