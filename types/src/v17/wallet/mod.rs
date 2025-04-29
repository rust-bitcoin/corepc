// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v0.17` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

mod error;
mod into;

use alloc::collections::BTreeMap;

use bitcoin::amount::ParseAmountError;
use bitcoin::key::{self, PrivateKey};
use bitcoin::{hex, Amount, Txid};
use serde::{Deserialize, Serialize};

// TODO: Remove wildcard, use explicit types.
pub use self::error::*;

// # Notes
//
// The following structs are very similar but have slightly different fields and docs.
// - GetTransaction
// - ListSinceBlockTransaction
// - ListTransactionsItem

/// Returned as part of `getaddressesbylabel` and `getaddressinfo`
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AddressPurpose {
    /// A send-to address.
    Send,
    /// A receive-from address.
    Receive,
}

/// The category of a transaction.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionCategory {
    /// Transactions sent.
    Send,
    /// Non-coinbase transactions received.
    Receive,
    /// Coinbase transactions received with more than 100 confirmations.
    Generate,
    /// Coinbase transactions received with 100 or fewer confirmations.
    Immature,
    /// Orphaned coinbase transactions received.
    Orphan,
}

/// Whether this transaction can be RBF'ed.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Bip125Replaceable {
    /// Yes, can be replaced due to BIP-125 (RBF).
    Yes,
    /// No, cannot be replaced due to BIP-125 (RBF).
    No,
    /// RBF unknown.
    Unknown,
}

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
pub struct AddMultisigAddress {
    /// The value of the new multisig address.
    pub address: String,
    /// The string value of the hex-encoded redemption script.
    #[serde(rename = "redeemScript")]
    pub redeem_script: String,
}

/// Result of the JSON-RPC method `bumpfee`.
///
/// > bumpfee "txid" ( options )
/// >
/// > Bumps the fee of an opt-in-RBF transaction T, replacing it with a new transaction B.
/// > An opt-in RBF transaction with the given txid must be in the wallet.
/// > The command will pay the additional fee by decreasing (or perhaps removing) its change output.
/// > If the change output is not big enough to cover the increased fee, the command will currently fail
/// > instead of adding new inputs to compensate. (A future implementation could improve this.)
/// > The command will fail if the wallet or mempool contains a transaction that spends one of T's outputs.
/// > By default, the new fee will be calculated automatically using estimatesmartfee.
/// > The user can specify a confirmation target for estimatesmartfee.
/// > Alternatively, the user can specify totalFee, or use RPC settxfee to set a higher fee rate.
/// > At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
/// > returned by getnetworkinfo) to enter the node's mempool.
/// >
/// > Arguments:
/// > 1. txid                  (string, required) The txid to be bumped
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BumpFee {
    /// The id of the new transaction.
    pub txid: String,
    /// Fee of the replaced transaction.
    #[serde(rename = "origfee")]
    pub original_fee: f64,
    /// Fee of the new transaction.
    pub fee: f64,
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,
}

/// Result of the JSON-RPC method `createwallet`.
///
/// > createwallet "wallet_name" ( disable_private_keys )
/// >
/// > Creates and loads a new wallet.
/// >
/// > Arguments:
/// > 1. "wallet_name"          (string, required) The name for the new wallet. If this is a path, the wallet will be created at the path location.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CreateWallet {
    /// The wallet name if created successfully.
    ///
    /// If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating and loading the wallet.
    pub warning: String,
}

impl CreateWallet {
    /// Returns the created wallet name.
    pub fn name(self) -> String { self.into_model().name }
}

/// Result of the JSON-RPC method `dumpprivkey`.
///
/// > dumpprivkey "address"
/// >
/// > Reveals the private key corresponding to 'address'.
/// > Then the importprivkey can be used with this output
/// >
/// > Arguments:
/// > 1. "address"   (string, required) The bitcoin address for the private key
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DumpPrivKey(pub String); // The private key.

impl DumpPrivKey {
    /// Returns the dumped key.
    pub fn key(self) -> Result<PrivateKey, key::FromWifError> { Ok(self.into_model()?.0) }
}

/// Result of the JSON-RPC method `dumpwallet`.
///
/// > dumpwallet "filename"
/// >
/// > Dumps all wallet keys in a human-readable format to a server-side file. This does not allow overwriting existing files.
/// > Imported scripts are included in the dumpfile, but corresponding BIP173 addresses, etc. may not be added automatically by importwallet.
/// > Note that if your wallet contains keys which are not derived from your HD seed (e.g. imported keys), these are not covered by
/// > only backing up the seed itself, and must be backed up too (e.g. ensure you back up the whole dumpfile).
/// >
/// > Arguments:
/// > 1. "filename"    (string, required) The filename with path (either absolute or relative to bitcoind)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DumpWallet {
    /// The filename with full absolute path.
    #[serde(rename = "filename")]
    pub file_name: String,
}

/// Result of the JSON-RPC method `getaddressesbylabel`.
///
/// > getaddressesbylabel "label"
/// >
/// > Returns the list of addresses assigned the specified label.
/// >
/// > Arguments:
/// > 1. "label"  (string, required) The label.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetAddressesByLabel(pub BTreeMap<String, AddressInformation>);

/// Returned as part of `getaddressesbylabel`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddressInformation {
    /// Purpose of address.
    pub purpose: AddressPurpose,
}

/// Result of the JSON-RPC method `getaddressinfo`.
///
/// > getaddressinfo "address"
/// >
/// > Return information about the given bitcoin address. Some information requires the address
/// > to be in the wallet.
/// >
/// > Arguments:
/// > 1. "address"                    (string, required) The bitcoin address to get the information of.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetAddressInfo {
    /// The bitcoin address validated.
    pub address: String,
    /// The hex encoded scriptPubKey generated by the address.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: String,
    /// If the address is yours or not.
    #[serde(rename = "ismine")]
    pub is_mine: bool,
    /// If the address is watchonly.
    #[serde(rename = "iswatchonly")]
    pub is_watch_only: bool,
    /// If the key is a script.
    #[serde(rename = "isscript")]
    pub is_script: bool,
    /// If the address is a witness address.
    #[serde(rename = "iswitness")]
    pub is_witness: bool,
    /// The version number of the witness program.
    pub witness_version: Option<i64>,
    /// The hex value of the witness program.
    pub witness_program: Option<String>,
    /// The output script type.
    ///
    /// Only if "is_script" is true and the redeemscript is known.
    pub script: Option<ScriptType>,
    /// The redeemscript for the p2sh address.
    pub hex: Option<String>,
    /// Array of pubkeys associated with the known redeemscript (only if "script" is "multisig").
    pub pubkeys: Option<Vec<String>>,
    /// Number of signatures required to spend multisig output (only if "script" is "multisig").
    #[serde(rename = "sigsrequired")]
    pub sigs_required: Option<i64>,
    /// The hex value of the raw public key, for single-key addresses (possibly embedded in P2SH or P2WSH).
    pub pubkey: Option<String>,
    /// Information about the address embedded in P2SH or P2WSH, if relevant and known.
    pub embedded: Option<GetAddressInfoEmbedded>,
    /// If the address is compressed.
    #[serde(rename = "iscompressed")]
    pub is_compressed: bool,
    /// The label associated with the address, "" is the default account.
    pub label: String,
    /// DEPRECATED. The account associated with the address, "" is the default account.
    pub account: String,
    /// The creation time of the key if available in seconds since epoch (Jan 1 1970 GMT).
    pub timestamp: Option<u32>,
    /// The HD keypath if the key is HD and available.
    #[serde(rename = "hdkeypath")]
    pub hd_key_path: Option<String>,
    /// The Hash160 of the HD seed.
    #[serde(rename = "hdseedid")]
    pub hd_seed_id: Option<String>,
    /// Alias for hdseedid maintained for backwards compatibility.
    ///
    /// Will be removed in V0.18.
    #[serde(rename = "hdmasterkeyid")]
    pub hd_master_key_id: Option<String>,
    /// Array of labels associated with the address.
    pub labels: Vec<GetAddressInfoLabel>,
}

/// The `script` field of `GetAddressInfo` (and `GetAddressInfoEmbedded`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ScriptType {
    /// Non-standard output script type.
    #[serde(rename = "nonstandard")]
    NonStandard,
    /// Pubkey output script.
    #[serde(rename = "pubkey")]
    Pubkey,
    /// Pubkey hash output script.
    #[serde(rename = "pubkeyhash")]
    PubkeyHash,
    /// Script hash output script.
    #[serde(rename = "scripthash")]
    ScriptHash,
    /// Multisig output script.
    #[serde(rename = "multisig")]
    Multisig,
    /// Null data for output script.
    #[serde(rename = "nulldata")]
    NullData,
    /// Witness version 0 key hash output script.
    #[serde(rename = "witness_v0_keyhash")]
    WitnessV0KeyHash,
    /// Witness version 0 script hash output script.
    #[serde(rename = "witness_v0_scripthash")]
    WitnessV0ScriptHash,
    /// Witness unknown for output script.
    #[serde(rename = "witness_unknown")]
    WitnessUnknown,
}

/// The `embedded` field of `GetAddressInfo`.
///
/// It includes all getaddressinfo output fields for the embedded address, excluding metadata
/// ("timestamp", "hdkeypath", "hdseedid") and relation to the wallet ("ismine", "iswatchonly",
/// "account").
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetAddressInfoEmbedded {
    /// The bitcoin address validated.
    pub address: String,
    /// The hex encoded scriptPubKey generated by the address.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: String,
    /// If the key is a script.
    #[serde(rename = "isscript")]
    pub is_script: bool,
    /// If the address is a witness address.
    #[serde(rename = "iswitness")]
    pub is_witness: bool,
    /// The version number of the witness program.
    pub witness_version: Option<i64>,
    /// The hex value of the witness program.
    pub witness_program: Option<String>,
    /// The output script type.
    ///
    /// Only if "is_script" is true and the redeemscript is known.
    pub script: Option<ScriptType>,
    /// The redeemscript for the p2sh address.
    pub hex: Option<String>,
    /// Array of pubkeys associated with the known redeemscript (only if "script" is "multisig").
    pub pubkeys: Vec<String>,
    /// Number of signatures required to spend multisig output (only if "script" is "multisig").
    #[serde(rename = "sigsrequired")]
    pub sigs_required: Option<i64>,
    /// The hex value of the raw public key, for single-key addresses (possibly embedded in P2SH or P2WSH).
    pub pubkey: Option<String>,
    /// If the address is compressed.
    #[serde(rename = "iscompressed")]
    pub is_compressed: bool,
    /// The label associated with the address, "" is the default account.
    pub label: String,
    /// Array of labels associated with the address.
    pub labels: Vec<GetAddressInfoLabel>,
}

/// The `label` field of `GetAddressInfo` (and `GetAddressInfoEmbedded`).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetAddressInfoLabel {
    /// The label.
    pub name: String,
    /// Purpose of address ("send" for sending address, "receive" for receiving address).
    pub purpose: AddressPurpose,
}

/// Result of the JSON-RPC method `getbalance`.
///
/// > getbalance ( "(dummy)" minconf include_watchonly )
/// >
/// > Returns the total available balance.
/// > The available balance is what the wallet considers currently spendable, and is
/// > thus affected by options which limit spendability such as -spendzeroconfchange.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalance(pub f64);

impl GetBalance {
    /// Converts json straight to a `bitcoin::Amount`.
    pub fn balance(self) -> Result<Amount, ParseAmountError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

/// Result of the JSON-RPC method `getnewaddress`.
///
/// > getnewaddress ( "label" "address_type" )
/// >
/// > Returns a new Bitcoin address for receiving payments.
/// > If 'label' is specified, it is added to the address book
/// > so payments received with the address will be associated with 'label'.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetNewAddress(pub String);

/// Result of the JSON-RPC method `getrawchangeaddress`.
///
/// > getrawchangeaddress ( "address_type" )
/// >
/// > Returns a new Bitcoin address, for receiving change.
/// > This is for use with raw transactions, NOT normal use.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetRawChangeAddress(pub String);

/// Result of the JSON-RPC method `getreceivedbyaddress`.
///
/// > getreceivedbyaddress "address" ( minconf )
/// >
/// > Returns the total amount received by the given address in transactions with at least minconf confirmations.
/// >
/// > Arguments:
/// > 1. "address"         (string, required) The bitcoin address for transactions.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetReceivedByAddress(pub f64); // Amount in BTC.

/// Result of the JSON-RPC method `gettransaction`.
///
/// > gettransaction "txid" ( include_watchonly )
/// >
/// > Get detailed information about in-wallet transaction `<txid>`
/// >
/// > Arguments:
/// > 1. txid                 (string, required) The transaction id
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTransaction {
    /// The transaction amount in BTC.
    pub amount: f64,
    /// The amount of the fee in BTC.
    ///
    /// This is negative and only available for the 'send' category of transactions.
    pub fee: Option<f64>,
    /// The number of confirmations.
    pub confirmations: i64,
    /// The block hash.
    #[serde(rename = "blockhash")]
    // The docs say this field should exist but integration test fail without `Option`.
    pub block_hash: Option<String>,
    /// The index of the transaction in the block that includes it.
    #[serde(rename = "blockindex")]
    // The docs say this field should exist but integration test fail without `Option`.
    pub block_index: Option<i64>,
    /// The time in seconds since epoch (1 Jan 1970 GMT).
    #[serde(rename = "blocktime")]
    pub block_time: Option<u32>, // Docs are wrong, this is not documented as optional.
    /// The transaction id.
    pub txid: String,
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
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionDetail {
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
}

/// Result of the JSON-RPC method `getunconfirmedbalance`.
///
/// > getunconfirmedbalance
/// > Returns the server's total unconfirmed balance
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetUnconfirmedBalance(pub f64); // Core docs are missing so this is just a guess.

/// Result of the JSON-RPC method `getwalletinfo`.
///
/// > getwalletinfo
/// > Returns an object containing various wallet state info.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetWalletInfo {
    /// The wallet name.
    #[serde(rename = "walletname")]
    pub wallet_name: String,
    /// The wallet version.
    #[serde(rename = "walletversion")]
    pub wallet_version: i64,
    /// The total confirmed balance of the wallet in BTC.
    pub balance: f64,
    /// The total unconfirmed balance of the wallet in BTC.
    pub unconfirmed_balance: f64,
    /// The total immature balance of the wallet in BTC.
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
    pub unlocked_until: u32,
    /// The transaction fee configuration, set in BTC/kB.
    #[serde(rename = "paytxfee")]
    pub pay_tx_fee: f64,
    /// The Hash160 of the HD seed (only present when HD is enabled).
    #[serde(rename = "hdseedid")]
    pub hd_seed_id: Option<String>,
    /// DEPRECATED. Alias for hdseedid retained for backwards-compatibility.
    #[serde(rename = "hdmasterkeyid")]
    pub hd_master_key_id: Option<String>,
    /// If privatekeys are disabled for this wallet (enforced watch-only wallet).
    pub private_keys_enabled: bool,
}

/// Result of the JSON-RPC method `listaddressgroupings`.
///
/// > listaddressgroupings
/// >
/// > Lists groups of addresses which have had their common ownership
/// > made public by common use as inputs or as the resulting change
/// > in past transactions
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListAddressGroupings(pub Vec<Vec<ListAddressGroupingsItem>>);

/// List item type returned as part of `listaddressgroupings`.
// FIXME: The Core docs seem wrong, not sure what shape this should be?
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListAddressGroupingsItem {
    /// The bitcoin address.
    pub address: String,
    /// The amount in BTC.
    pub amount: f64,
    /// The label.
    pub label: Option<String>,
}

/// Result of the JSON-RPC method `listlabels`.
///
/// > listlabels ( "purpose" )
/// >
/// > Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListLabels(pub Vec<String>);

/// Result of the JSON-RPC method `listlockunspent`.
///
/// > listlockunspent
/// >
/// > Returns list of temporarily unspendable outputs.
/// > See the lockunspent call to lock and unlock transactions for spending.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListLockUnspent(pub Vec<ListLockUnspentItem>);

/// List item returned as part of of `listlockunspent`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListLockUnspentItem {
    /// The transaction id locked.
    pub txid: String,
    /// The vout value.
    pub vout: i64,
}

/// Result of the JSON-RPC method `listreceivedbyaddress`.
///
/// > listreceivedbyaddress ( minconf include_empty include_watchonly address_filter )
/// >
/// > List balances by receiving address.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListReceivedByAddress(pub Vec<ListReceivedByAddressItem>);

/// List item returned as part of of `listreceivedByaddress`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListReceivedByAddressItem {
    /// Only returned if imported addresses were involved in transaction.
    #[serde(rename = "involvesWatchonly")]
    pub involves_watch_only: bool,
    /// The receiving address.
    pub address: String,
    /// DEPRECATED. Backwards compatible alias for label.
    pub account: String,
    /// The total amount in BTC received by the address.
    pub amount: f64,
    /// The number of confirmations of the most recent transaction included.
    pub confirmations: i64,
    /// The label of the receiving address. The default label is "".
    pub label: String,
    /// The ids of transactions received with the address.
    pub txids: Vec<String>,
}

/// Result of the JSON-RPC method `listsinceblock`.
///
/// > listsinceblock ( "blockhash" target_confirmations include_watchonly include_removed )
/// >
/// > Get all transactions in blocks since block `blockhash`, or all transactions if omitted.
/// > If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
/// > Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListSinceBlock {
    /// All the transactions.
    pub transactions: Vec<ListSinceBlockTransaction>,
    /// Only present if `include_removed=true`.
    ///
    /// Note: transactions that were re-added in the active chain will appear as-is in this array,
    /// and may thus have a positive confirmation count.
    pub removed: Vec<ListSinceBlockTransaction>,
    /// The hash of the block (target_confirmations-1) from the best block on the main chain.
    ///
    /// This is typically used to feed back into listsinceblock the next time you call it. So you
    /// would generally use a target_confirmations of say 6, so you will be continually
    /// re-notified of transactions until they've reached 6 confirmations plus any new ones.
    #[serde(rename = "lastblock")]
    pub last_block: String,
}

/// Transaction item returned as part of `listsinceblock`.
// FIXME: These docs from Core seem to buggy, there is only partial mention of 'move' category?
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListSinceBlockTransaction {
    /// DEPRECATED. The account name associated with the transaction. Will be "" for the default account.
    pub account: String,
    /// The bitcoin address of the transaction.
    ///
    /// Not present for move transactions (category = move).
    pub address: String,
    // FIXME: Maybe there is a 'move' category too?
    /// The transaction category. 'send' has negative amounts, 'receive' has positive amounts.
    pub category: TransactionCategory,
    /// The amount in BTC.
    ///
    /// This is negative for the 'send' category, and for the 'move' category for moves outbound. It
    /// is positive for the 'receive' category, and for the 'move' category for inbound funds.
    pub amount: f64,
    /// The vout value.
    pub vout: i64,
    /// The amount of the fee in BTC.
    ///
    /// This is negative and only available for the 'send' category of transactions.
    pub fee: f64,
    /// The number of confirmations for the transaction.
    ///
    /// Available for 'send' and 'receive' category of transactions. When it's < 0, it means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: i64,
    /// The block hash containing the transaction.
    ///
    /// Available for 'send' and 'receive' category of transactions.
    #[serde(rename = "blockhash")]
    pub block_hash: String,
    /// The index of the transaction in the block that includes it.
    ///
    /// Available for 'send' and 'receive' category of transactions.
    #[serde(rename = "blockindex")]
    pub block_index: i64,
    /// The block time in seconds since epoch (1 Jan 1970 GMT).
    #[serde(rename = "blocktime")]
    pub block_time: u32,
    /// The transaction id.
    ///
    /// Available for 'send' and 'receive' category of transactions.
    pub txid: Option<String>,
    /// The transaction time in seconds since epoch (Jan 1 1970 GMT).
    pub time: u32,
    /// The time received in seconds since epoch (Jan 1 1970 GMT).
    ///
    /// Available for 'send' and 'receive' category of transactions.
    #[serde(rename = "timereceived")]
    pub time_received: u32,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee);
    /// may be unknown for unconfirmed transactions not in the mempool
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: Bip125Replaceable,
    /// If the transaction has been abandoned (inputs are respendable).
    ///
    /// Only available for the 'send' category of transactions.
    pub abandoned: Option<bool>,
    /// If a comment is associated with the transaction.
    pub comment: Option<String>,
    /// A comment for the address/transaction, if any.
    pub label: Option<String>,
    /// If a comment to is associated with the transaction.
    pub to: Option<String>,
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
pub struct ListTransactions(pub Vec<ListTransactionsItem>);

/// Transaction item returned as part of `listtransactions`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListTransactionsItem {
    /// The bitcoin address of the transaction.
    pub address: String,
    /// The transaction category.
    pub category: TransactionCategory, // FIXME: It appears ok to reuse this?
    /// The amount in BTC.
    ///
    /// This is negative for the 'send' category, and is positive for the 'receive' category.
    pub amount: f64,
    /// A comment for the address/transaction, if any.
    pub label: Option<String>,
    /// The vout value.
    pub vout: i64,
    /// The amount of the fee in BTC.
    ///
    /// This is negative and only available for the 'send' category of transactions.
    pub fee: f64,
    /// The number of confirmations for the transaction.
    ///
    /// Negative confirmations indicate the transaction conflicts with the block chain.
    pub confirmations: i64,
    /// Whether we consider the outputs of this unconfirmed transaction safe to spend.
    pub trusted: bool,
    /// The block hash containing the transaction.
    #[serde(rename = "blockhash")]
    pub block_hash: String,
    /// The index of the transaction in the block that includes it.
    #[serde(rename = "blockindex")]
    pub block_index: i64,
    /// The block time in seconds since epoch (1 Jan 1970 GMT).
    #[serde(rename = "blocktime")]
    pub block_time: u32,
    /// The transaction id.
    pub txid: String,
    /// The transaction time in seconds since epoch (Jan 1 1970 GMT).
    pub time: u32,
    /// The time received in seconds since epoch (Jan 1 1970 GMT).
    #[serde(rename = "timereceived")]
    pub time_received: u32,
    /// If a comment is associated with the transaction.
    pub comment: Option<String>,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee);
    /// may be unknown for unconfirmed transactions not in the mempool
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: Bip125Replaceable,
    /// If the transaction has been abandoned (inputs are respendable).
    ///
    /// Only available for the 'send' category of transactions.
    pub abandoned: Option<bool>,
}

/// Result of the JSON-RPC method `listunspent`.
///
/// > listunspent ( minconf maxconf  ["addresses",...] `[include_unsafe]` `[query_options]`)
/// >
/// > Returns array of unspent transaction outputs
/// > with between minconf and maxconf (inclusive) confirmations.
/// > Optionally filter to only include txouts paid to specified addresses.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListUnspent(pub Vec<ListUnspentItem>);

/// Unspent transaction output, returned as part of `listunspent`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListUnspentItem {
    /// The transaction id.
    pub txid: String,
    /// The vout value.
    pub vout: i64,
    /// The bitcoin address of the transaction.
    pub address: String,
    /// The associated label, or "" for the default label.
    pub label: String,
    /// DEPRECATED. The account name associated with the transaction. Will be "" for the default account.
    pub account: String,
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
    /// Whether this output is considered safe to spend. Unconfirmed transactions from outside keys
    /// and unconfirmed replacement transactions are considered unsafe and are not eligible for
    /// spending by fundrawtransaction and sendtoaddress.
    pub safe: bool,
}

/// Result of the JSON-RPC method `listwallets`.
///
/// > listwallets
/// > Returns a list of currently loaded wallets.
/// > For full information on the wallet, use "getwalletinfo"
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListWallets(pub Vec<String>);

/// Result of the JSON-RPC method `loadwallet`.
///
/// > loadwallet "filename"
/// >
/// > Loads a wallet from a wallet file or directory.
/// > Note that all wallet command-line options used when starting bitcoind will be
/// > applied to the new wallet (eg -zapwallettxes, upgradewallet, rescan, etc).
/// >
/// > Arguments:
/// > 1. "filename"    (string, required) The wallet directory or .dat file.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoadWallet {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    pub warning: String,
}

impl LoadWallet {
    /// Returns the loaded wallet name.
    pub fn name(self) -> String { self.into_model().name }
}

/// Result of the JSON-RPC method `rescanblockchain`.
///
/// > rescanblockchain ("start_height") ("stop_height")
/// >
/// > Rescan the local blockchain for wallet related transactions.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RescanBlockchain {
    /// The block height where the rescan has started.
    pub start_height: i64,
    /// The height of the last rescanned block.
    pub stop_height: i64,
}

/// Result of the JSON-RPC method `sendmany`.
///
/// > sendmany "" {"address":amount,...} ( minconf "comment" ["address",...] replaceable conf_target "estimate_mode")
/// >
/// > Send multiple times. Amounts are double-precision floating point numbers.
/// > Note that the "fromaccount" argument has been removed in V0.17. To use this RPC with a "fromaccount" argument, restart
/// > bitcoind with -deprecatedrpc=accounts
/// >
/// >
/// > Arguments:
/// > 1. "dummy"               (string, required) Must be set to "" for backwards compatibility.
/// > 2. "amounts"             (string, required) A json object with addresses and amounts
/// >     {
/// >       "address":amount   (numeric or string) The bitcoin address is the key, the numeric amount (can be string) in BTC is the value
/// >       ,...
/// >     }
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SendMany(
    /// The transaction id for the send.
    ///
    /// Only 1 transaction is created regardless of the number of addresses.
    pub String,
);

/// Result of the JSON-RPC method `sendtoaddress`.
///
/// > sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode")
/// >
/// > Send an amount to a given address.
/// >
/// > Arguments:
/// > 1. "address"            (string, required) The bitcoin address to send to.
/// > 2. "amount"             (numeric or string, required) The amount in BTC to send. eg 0.1
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SendToAddress(pub String);

impl SendToAddress {
    /// Converts json straight to a `bitcoin::Txid`.
    pub fn txid(self) -> Result<Txid, hex::HexToArrayError> { Ok(self.into_model()?.txid) }
}

/// Result of the JSON-RPC method `signmessage`.
///
/// > signmessage "address" "message"
/// >
/// > Sign a message with the private key of an address
/// >
/// > Arguments:
/// > 1. "address"         (string, required) The bitcoin address to use for the private key.
/// > 2. "message"         (string, required) The message to create a signature of.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SignMessage(
    /// The signature of the message encoded in base 64.
    pub String,
);

/// Result of the JSON-RPC method `signrawtransactionwithwallet`.
///
/// > signrawtransactionwithwallet "hexstring" ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] sighashtype )
/// >
/// > Sign inputs for raw transaction (serialized, hex-encoded).
/// > The second optional argument (may be null) is an array of previous transaction outputs that
/// > this transaction depends on but may not yet be in the block chain.
/// >
/// >
/// > Arguments:
/// > 1. "hexstring"                      (string, required) The transaction hex string
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SignRawTransactionWithWallet {
    /// The hex-encoded raw transaction with signature(s).
    pub hex: String,
    /// If the transaction has a complete set of signatures.
    pub complete: bool,
    /// Script verification errors (if there are any).
    pub errors: Option<Vec<SignErrorData>>, // 'Data' suffix to differentiate this from a normal error type.
}

/// Returned as part of `signrawtransactionwithwallet`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SignErrorData {
    /// The hash of the referenced, previous transaction.
    pub txid: String,
    /// The index of the output to spent and used as input.
    pub vout: i64,
    /// The hex-encoded signature script.
    #[serde(rename = "ScriptSig")]
    pub script_sig: String,
    /// Script sequence number.
    pub sequence: u32,
    /// Verification or signing error related to the input.
    pub error: String,
}

/// Result of the JSON-RPC method `walletcreatefundedpsbt`.
///
/// > walletcreatefundedpsbt [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable ) ( options bip32derivs )
/// >
/// > Creates and funds a transaction in the Partially Signed Transaction format. Inputs will be added if supplied inputs are not enough
/// > Implements the Creator and Updater roles.
/// >
/// > Arguments:
/// > 1. "inputs"                (array, required) A json array of json objects
/// >      [
/// >        {
/// >          "txid":"id",      (string, required) The transaction id
/// >          "vout":n,         (numeric, required) The output number
/// >          "sequence":n      (numeric, optional) The sequence number
/// >        }
/// >        ,...
/// >      ]
/// > 2. "outputs"               (array, required) a json array with outputs (key-value pairs)
/// >    [
/// >     {
/// >       "address": x.xxx,    (obj, optional) A key-value pair. The key (string) is the bitcoin address, the value (float or string) is the amount in BTC
/// >     },
/// >     {
/// >       "data": "hex"        (obj, optional) A key-value pair. The key must be "data", the value is hex encoded data
/// >     }
/// >     ,...                     More key-value pairs of the above form. For compatibility reasons, a dictionary, which holds the key-value pairs directly, is also
/// >                              accepted as second parameter.
/// >    ]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletCreateFundedPsbt {
    /// The resulting raw transaction (base64-encoded string).
    pub psbt: String,
    /// Fee in BTC the resulting transaction pays.
    pub fee: f64,
    /// The position of the added change output, or -1.
    #[serde(rename = "changepos")]
    pub change_pos: i64,
}

/// Result of the JSON-RPC method `walletprocesspsbt`.
///
/// > walletprocesspsbt "psbt" ( sign "sighashtype" bip32derivs )
/// >
/// > Update a PSBT with input information from our wallet and then sign inputs
/// > that we can sign for.
/// >
/// >
/// > Arguments:
/// > 1. "psbt"                      (string, required) The transaction base64 string
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct WalletProcessPsbt {
    /// The base64-encoded partially signed transaction.
    pub psbt: String,
    /// If the transaction has a complete set of signatures.
    pub complete: bool,
}

/// Result of the JSON-RPC method `abandontransaction`.
///
/// > abandontransaction "txid"
/// >
/// >
/// >
/// > Arguments:
/// > 1. txid    (string, required) The transaction id
/// >
/// > Result null (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AbandonTransaction;

/// Result of JSON-RPC method `abortrescan`.
///
/// > abortrescan
/// >
/// > Returns null
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AbortRescan;

/// Result of JSON-RPC method `backupwallet`.
///
/// > backupwallet
/// >
/// > Returns null (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct BackupWallet;

/// Result of JSON-RPC method `encryptwallet`.
///
/// > encryptwallet
/// >
/// > Arguments:
/// > 1. passphrase (string, required) The pass phrase to encrypt the wallet with. It must be at least 1 character, but should be long.
/// >
/// > Returns null (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct EncryptWallet;

/// Result of JSON-RPC method `importaddress`.
///
/// > importaddress
/// >
/// > Arguments:
/// > 1. address    (string, required) The Bitcoin address (or hex-encoded script)
/// > 2. label      (string, optional, default="") An optional label
/// > 3. rescan     (boolean, optional, default=true) Scan the chain and mempool for wallet transactions.
/// > 4. p2sh       (boolean, optional, default=false) Add the P2SH version of the script as well
/// >
/// > Returns null (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ImportAddress;

/// Result of JSON-RPC method `importprivkey`.
///
/// > importprivkey
/// >
/// > Arguments:
/// > 1. privkey    (string, required) The private key (see dumpprivkey)
/// > 2. label      (string, optional, default=current label if address exists, otherwise "") An optional label
/// > 3. rescan     (boolean, optional, default=true) Rescan the wallet for transactions
/// >
/// > Returns null (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ImportPrivKey;

/// Result of JSON-RPC method `importprunedfunds`.
///
/// > importprunedfunds
/// >
/// > Arguments:
/// > 1. rawtransaction    (string, required) A raw transaction in hex funding an already-existing address in wallet
/// > 2. txoutproof        (string, required) The hex output from gettxoutproof that contains the transaction
/// >
/// > Returns null (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ImportPrunedFunds;

/// Result of JSON-RPC method `importpubkey`.
///
/// > importpubkey
/// >
/// > Arguments:
/// > 1. pubkey    (string, required) The hex-encoded public key
/// > 2. label     (string, optional, default="") An optional label
/// > 3. rescan    (boolean, optional, default=true) Scan the chain and mempool for wallet transactions.
/// >
/// > Returns null (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ImportPubKey;

/// Result of JSON-RPC method `importwallet`.
///
/// > importwallet
/// >
/// > Arguments:
/// > 1. filename    (string, required) The wallet file
/// >
/// > Returns null (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ImportWallet;

/// Result of JSON-RPC method `keypoolrefill`.
///
/// > keypoolrefill
/// >
/// > Arguments:
/// > 1. newsize    (numeric, optional, default=1000, or as set by -keypool) The new keypool size
/// >
/// > Returns null (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct KeypoolRefill;

/// Result of JSON-RPC method `lockunspent`.
///
/// > lockunspent
/// >
/// > Arguments:
/// > 1. unlock                  (boolean, required) Whether to unlock (true) or lock (false) the specified transactions
/// > 2. transactions            (json array, optional, default=[]) The transaction outputs and within each, the txid (string) vout (numeric).
/// >   [
/// >   {                   (json object)
///   > "txid": "hex",    (string, required) The transaction id
///   > "vout": n,        (numeric, required) The output number
/// >   },
/// >   ...
///]
/// > 3. persistent              (boolean, optional, default=false) Whether to write/erase this lock in the wallet database, or keep the change in memory only. Ignored for unlocking.
/// >
/// > Returns null (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LockUnspent(pub bool);

/// Result of JSON-RPC method `removeprunedfunds`.
///
/// > removeprunedfunds
/// >
/// > Arguments:
/// > 1. txid    (string, required) The hex-encoded id of the transaction you are deleting
/// >
/// > Returns null (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RemovePrunedFunds;

/// Result of JSON-RPC method `sethdseed`.
///
/// > sethdseed
/// >
/// > Arguments:
/// > 1. newkeypool    (boolean, optional, default=true) Whether to flush old unused addresses, including change addresses, from the keypool and regenerate it.
/// >   If true, the next address from getnewaddress and change address from getrawchangeaddress will be from this new seed.
/// >   If false, addresses (including change addresses if the wallet already had HD Chain Split enabled) from the existing
/// >   keypool will be used until it has been depleted.
/// > 2. seed          (string, optional, default=random seed) The WIF private key to use as the new HD seed.
/// >   The seed value can be retrieved using the dumpwallet command. It is the private key marked hdseed=1
/// >
/// > Returns null (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SetHdSeed;

/// Result of JSON-RPC method `settxfee`.
///
/// > settxfee
/// >
/// > Arguments:
/// > 1. amount         (numeric or string, required) The transaction fee in BTC/kB
/// >
/// > Returns true|false        (boolean) Returns true if successful
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SetTxFee(pub bool);

/// Result of JSON-RPC method `walletlock`.
///
/// > walletlock
/// >
/// > Returns:
/// > null    (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct WalletLock;

/// Result of JSON-RPC method `walletpassphrase`.
///
/// > walletpassphrase
/// >
/// > Arguments:
/// > 1. passphrase    (string, required) The wallet passphrase
/// > 2. timeout       (numeric, required) The time to keep the decryption key in seconds; capped at 100000000 (~3 years).
///
/// > Returns:
/// > null    (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct WalletPassPhrase;

/// Result of JSON-RPC method `walletpassphrasechange`.
///
/// > walletpassphrasechange
/// >
/// > Arguments:
/// > 1. oldpassphrase    (string, required) The current passphrase
/// > 2. newpassphrase    (string, required) The new passphrase
///
/// > Returns:
/// > null    (json null)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct WalletPassPhraseChange;

/// Result of JSON-RPC method `importmulti`.
///
/// > importmulti requests ( options )
/// >
/// > Arguments:
/// > 1. requests                                                         (json array, required) Data to be imported
/// >   [
/// >   {                                                            (json object)
/// >   "desc": "str",                                             (string, optional) Descriptor to import. If using descriptor, do not also provide address/scriptPubKey, scripts, or pubkeys
/// >   "scriptPubKey": "script" | { "address":"address" },    (string / json, required) Type of scriptPubKey (string for script, json for address). Should not be provided if using a descriptor
/// >   "timestamp": timestamp | "now",                            (integer / string, required) Creation time of the key expressed in UNIX epoch time,or the string "now" to substitute the current synced blockchain time. The timestamp of the oldest key will determine how far back blockchain rescans need to begin for missing wallet transactions. "now" can be specified to bypass scanning, for keys which are known to never have been used, and 0 can be specified to scan the entire blockchain. Blocks up to 2 hours before the earliest key creation time of all keys being imported by the importmulti call will be scanned.
/// >   "redeemscript": "str",                                     (string, optional) Allowed only if the scriptPubKey is a P2SH or P2SH-P2WSH address/scriptPubKey
/// >   "witnessscript": "str",                                    (string, optional) Allowed only if the scriptPubKey is a P2SH-P2WSH or P2WSH address/scriptPubKey
/// >   "pubkeys": [                                               (json array, optional, default=[]) Array of strings giving pubkeys to import. They must occur in P2PKH or P2WPKH scripts. They are not required when the private key is also provided (see the "keys" argument).
/// >   "pubKey",                                                (string)
/// >   ...
/// >   ],
/// >   "keys": [                                                  (json array, optional, default=[]) Array of strings giving private keys to import. The corresponding public keys must occur in the output or redeemscript.
/// >   "key",                                                   (string)
/// >   ...
/// >   ],
/// >   "range": n or \[n,n\],                                       (numeric or array, optional) If a ranged descriptor is used, this specifies the end or the range (in the form \[begin,end\]) to import
/// >   "internal": bool,                                          (boolean, optional, default=false) Stating whether matching outputs should be treated as not incoming payments (also known as change)
/// >   "watchonly": bool,                                         (boolean, optional, default=false) Stating whether matching outputs should be considered watchonly.
/// >   "label": "str",                                            (string, optional, default="") Label to assign to the address, only allowed with internal=false
/// >   "keypool": bool,                                           (boolean, optional, default=false) Stating whether imported public keys should be added to the keypool for when users request new addresses. Only allowed when wallet private keys are disabled
/// >   },
/// >   ...
/// >   ]
/// 2. options                                                          (json object, optional) Options object that can be used to pass named arguments, listed below.
/// > Named Arguments:
/// > rescan    (boolean, optional, default=true) Scan the chain and mempool for wallet transactions after all imports.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ImportMulti(pub Vec<ImportMultiEntry>);

/// Represents a single entry in the importmulti result array.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ImportMultiEntry {
    pub success: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
