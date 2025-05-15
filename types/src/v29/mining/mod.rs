// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v29` - mining.
//!
//! Types for methods found under the `== Mining ==` section of the API docs.

mod error;
mod into;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use self::error::{BlockTemplateTransactionError, GetBlockTemplateError};

/// Represents the `next` block information within the GetMiningInfo result.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NextBlockInfo {
    /// The next height
    pub height: i64,
    /// The next nBits
    pub bits: String,
    /// The next difficulty
    pub difficulty: f64,
    /// The next target
    pub target: String,
}

/// Result of the JSON-RPC method `getmininginfo`.
///
/// > getmininginfo
/// >
/// > Returns a json object containing mining-related information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetMiningInfo {
    /// The current block.
    pub blocks: i64,
    /// The last block weight.
    #[serde(rename = "currentblockweight")]
    pub current_block_weight: Option<i64>,
    /// The last block transaction.
    #[serde(rename = "currentblocktx")]
    pub current_block_transaction: Option<i64>,
    /// The current nBits (added in v29)
    pub bits: String,
    /// The current difficulty.
    pub difficulty: f64,
    /// The current target (added in v29)
    pub target: String,
    /// The network hashes per second.
    #[serde(rename = "networkhashps")]
    pub network_hash_ps: i64,
    /// The size of the mempool.
    #[serde(rename = "pooledtx")]
    pub pooled_transactions: i64,
    /// Current network name as defined in BIP70 (main, test, regtest).
    pub chain: String,
    /// The block challenge for signet (optional, added in v29).
    pub signet_challenge: Option<String>,
    /// Information about the next block (added in v29)
    #[serde(rename = "next")]
    pub next_block: NextBlockInfo,
    /// Any network and blockchain warnings.
    pub warnings: Vec<String>,
}

/// Result of the JSON-RPC method `getblocktemplate`.
///
/// > getblocktemplate {"mode":"str","capabilities":["str",...],"rules":["segwit","str",...],"longpollid":"str","data":"hex"}
/// >
/// > If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// > It returns data needed to construct a block to work on.
/// > For full specification, see BIPs 22, 23, 9, and 145:
/// >     <https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki>
/// >     <https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki>
/// >     <https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes>
/// >     <https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki>
/// >
/// > Arguments:
/// > 1. template_request            (json object, required) Format of the template
/// >      {
/// >        "mode": "str",          (string, optional) This must be set to "template", "proposal" (see BIP 23), or omitted
/// >        "capabilities": [       (json array, optional) A list of strings
/// >          "str",                (string) client side supported feature, 'longpoll', 'coinbasevalue', 'proposal', 'serverlist', 'workid'
/// >          ...
/// >        ],
/// >        "rules": [              (json array, required) A list of strings
/// >          "segwit",             (string, required) (literal) indicates client side segwit support
/// >          "str",                (string) other client side supported softfork deployment
/// >          ...
/// >        ],
/// >        "longpollid": "str",    (string, optional) delay processing request until the result would vary significantly from the "longpollid" of a prior template
/// >        "data": "hex",          (string, optional) proposed block data to check, encoded in hexadecimal; valid only for mode="proposal"
/// >      }
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockTemplate {
    /// The preferred block version.
    pub version: i32,
    /// Specific block rules that are to be enforced.
    pub rules: Vec<String>,
    /// Set of pending, supported versionbit (BIP 9) softfork deployments.
    ///
    /// Map of rules name to bit number - identifies the bit number as indicating acceptance and
    /// readiness for the named softfork rule.
    #[serde(rename = "vbavailable")]
    pub version_bits_available: BTreeMap<String, u32>,
    /// Bit mask of versionbits the server requires set in submissions.
    #[serde(rename = "vbrequired")]
    pub version_bits_required: i64,
    /// The hash of current highest block.
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: String,
    /// Contents of non-coinbase transactions that should be included in the next block.
    pub transactions: Vec<BlockTemplateTransaction>,
    /// Data that should be included in the coinbase's scriptSig content.
    ///
    /// Key name is to be ignored, and value included in scriptSig.
    #[serde(rename = "coinbaseaux")]
    pub coinbase_aux: BTreeMap<String, String>,
    /// Maximum allowable input to coinbase transaction, including the generation award and transaction fees (in satoshis).
    #[serde(rename = "coinbasevalue")]
    pub coinbase_value: i64,
    /// A list of supported features,for example `proporsal`
    pub capabilities: Vec<String>,
    /// ID to include with a request to longpool on an update to this template
    #[serde(rename = "longpollid")]
    pub long_pool_id: String,
    // This is in the docs but not actually returned (for v17 and v18 at least).
    // coinbase_txn: ???, // Also I don't know what the JSON object represents: `{ ... }`
    /// The hash target.
    pub target: String,
    /// The minimum timestamp appropriate for next block time in seconds since epoch (Jan 1 1970 GMT).
    #[serde(rename = "mintime")]
    pub min_time: u32,
    /// List of ways the block template may be changed.
    ///
    /// A way the block template may be changed, e.g. 'time', 'transactions', 'prevblock'
    pub mutable: Vec<String>,
    /// A range of valid nonces.
    #[serde(rename = "noncerange")]
    pub nonce_range: String,
    /// Limit of sigops in blocks.
    #[serde(rename = "sigoplimit")]
    pub sigop_limit: i64,
    /// Limit of block size.
    #[serde(rename = "sizelimit")]
    pub size_limit: i64,
    /// Limit of block weight.
    #[serde(rename = "weightlimit")]
    pub weight_limit: i64,
    /// Current timestamp in seconds since epoch (Jan 1 1970 GMT).
    #[serde(rename = "curtime")]
    pub current_time: u64,
    /// Compressed target of next block.
    pub bits: String,
    /// The height of the next block,
    pub height: i64,
    /// Optional signet challenge
    #[serde(rename = "signet_challenge")]
    pub signet_challenge: Option<String>,
    #[serde(rename = "defaultwitnesscommitment")]
    pub default_witness_commitment: Option<String>,
}

/// Contents of non-coinbase transactions that should be included in the next block.
///
/// Returned as part of `getblocktemplate`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BlockTemplateTransaction {
    /// Transaction data encoded in hexadecimal (byte-for-byte).
    pub data: String,
    /// Transaction id encoded in little-endian hexadecimal.
    pub txid: String,
    /// Hash encoded in little-endian hexadecimal (including witness data).
    pub hash: String,
    /// Array of numbers.
    ///
    /// Transactions before this one (by 1-based index in 'transactions' list) that must be present in the final block if this one is.
    pub depends: Vec<i64>,
    /// Difference in value between transaction inputs and outputs (in satoshis); for coinbase
    /// transactions, this is a negative Number of the total collected block fees (ie, not including
    /// the block subsidy); if key is not present, fee is unknown and clients MUST NOT assume there
    /// isn't one.
    pub fee: i64,
    /// Total SigOps cost, as counted for purposes of block limits; if key is not present, sigop
    /// cost is unknown and clients MUST NOT assume it is zero.
    pub sigops: i64,
    /// Total transaction weight, as counted for purposes of block limits.
    pub weight: u64,
}
