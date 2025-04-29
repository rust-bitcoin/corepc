// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v28` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

use bitcoin::address::NetworkUnchecked;
use bitcoin::{Address, Amount, BlockHash, ScriptBuf, SignedAmount, Txid};
use serde::{Deserialize, Serialize};
extern crate bitcoin;

/// Result of the JSON-RPC method `listunspent`.
///
/// > listunspent ( minconf maxconf  ["addresses",...] `[include_unsafe]` `[query_options]`)
/// >
/// > Returns array of unspent transaction outputs
/// > with between minconf and maxconf (inclusive) confirmations.
/// > Optionally filter to only include txouts paid to specified addresses.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ListUnspent(pub Vec<ListUnspentItem>);

/// Unspent transaction output, returned as part of `listunspent`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")] // Match typical JSON field names
pub struct ListUnspentItem {
    /// The transaction id.
    pub txid: Txid,
    /// The vout value.
    pub vout: u32,
    /// The bitcoin address of the transaction (optional for non-standard scripts).
    pub address: Option<Address<NetworkUnchecked>>,
    /// The associated label, present only if the address is in the address book.
    pub label: Option<String>,
    /// Optional account field.
    pub account: Option<String>,
    /// The script key.
    #[serde(rename = "scriptPubKey")]
    pub script_pubkey: ScriptBuf,
    /// The transaction amount.
    // Use Amount (unsigned) for UTXOs
    #[serde(with = "bitcoin::amount::serde::as_btc")]
    pub amount: Amount,
    /// The number of confirmations.
    pub confirmations: u32,
    /// The redeemScript if scriptPubKey is P2SH.
    #[serde(rename = "redeemScript")]
    pub redeem_script: Option<ScriptBuf>,
    /// The witnessScript if the scriptPubKey is P2WSH or P2SH-P2WSH.
    #[serde(rename = "witnessScript")]
    pub witness_script: Option<ScriptBuf>,
    /// Whether we have the private keys to spend this output.
    pub spendable: bool,
    /// Whether we know how to spend this output, ignoring the lack of keys.
    pub solvable: bool,
    /// Whether this output is considered safe to spend.
    pub safe: bool,
    // Add other optional fields if needed for your version (desc, coinbase etc.)
    /// Output descriptor if available.
    pub desc: Option<String>,
    /// Whether this is a coinbase output (added in v25+).
    pub coinbase: Option<bool>,
}

/// Transaction item returned as part of `listtransactions`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)] // Removed Eq due to SignedAmount potentially not deriving Eq
#[serde(rename_all = "camelCase")] // Match common JSON names if needed
pub struct ListTransactionsItem {
    /// The bitcoin address of the transaction (optional).
    // Using Option<Address> as address might not always be present/decodable
    pub address: Option<Address<NetworkUnchecked>>,
    /// The transaction category.
    pub category: TransactionCategory,
    /// The amount.
    #[serde(with = "bitcoin::amount::serde::as_btc")]
    pub amount: SignedAmount,
    /// A comment for the address/transaction, if any.
    pub label: Option<String>,
    /// The vout value.
    pub vout: u32,
    /// The amount of the fee in BTC. (Made optional)
    pub fee: Option<f64>,
    /// The number of confirmations for the transaction.
    pub confirmations: i64,
    /// Whether we consider the outputs of this unconfirmed transaction safe to spend.
    pub trusted: Option<bool>,
    /// The block hash containing the transaction.
    #[serde(rename = "blockhash")]
    pub block_hash: Option<BlockHash>,
    /// The index of the transaction in the block that includes it.
    #[serde(rename = "blockindex")]
    pub block_index: Option<u32>,
    /// The block time in seconds since epoch (1 Jan 1970 GMT).
    #[serde(rename = "blocktime")]
    pub block_time: Option<u32>,
    /// The transaction id.
    pub txid: Txid,
    /// The transaction time in seconds since epoch (Jan 1 1970 GMT).
    pub time: u32,
    /// The time received in seconds since epoch (Jan 1 1970 GMT).
    #[serde(rename = "timereceived")]
    pub time_received: u32,
    /// If a comment is associated with the transaction.
    pub comment: Option<String>,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee);
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: Bip125Replaceable,
    /// If the transaction has been abandoned (inputs are respendable).
    pub abandoned: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionCategory {
    Send,
    Receive,
    Generate,
    Immature,
    Orphan,
    Move,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Bip125Replaceable {
    Yes,
    No,
    Unknown,
}

// Ensure you have the top-level struct too
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListTransactions(pub Vec<ListTransactionsItem>);
