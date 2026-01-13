// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core `v0.17`.
//!
//! We ignore option arguments unless they effect the shape of the returned JSON data.

pub mod blockchain;
pub mod control;
pub mod generating;
pub mod hidden;
pub mod mining;
pub mod network;
pub mod raw_transactions;
pub mod util;
pub mod wallet;
pub mod zmq;

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{sign_message, Amount, Block, BlockHash, PublicKey, Txid};
use serde::{Deserialize, Serialize, Serializer};

use crate::client_sync::into_json;
use crate::types::v17::*;

crate::define_jsonrpc_bitreq_client!("v17");
crate::impl_client_check_expected_server_version!({ [170200] });

// == Blockchain ==
crate::impl_client_v17__get_best_block_hash!();
crate::impl_client_v17__get_block!();
crate::impl_client_v17__get_blockchain_info!();
crate::impl_client_v17__get_block_count!();
crate::impl_client_v17__get_block_hash!();
crate::impl_client_v17__get_block_header!();
crate::impl_client_v17__get_block_stats!();
crate::impl_client_v17__get_chain_tips!();
crate::impl_client_v17__get_chain_tx_stats!();
crate::impl_client_v17__get_difficulty!();
crate::impl_client_v17__get_mempool_ancestors!();
crate::impl_client_v17__get_mempool_descendants!();
crate::impl_client_v17__get_mempool_entry!();
crate::impl_client_v17__get_mempool_info!();
crate::impl_client_v17__get_raw_mempool!();
crate::impl_client_v17__get_tx_out!();
crate::impl_client_v17__get_tx_out_proof!();
crate::impl_client_v17__get_tx_out_set_info!();
crate::impl_client_v17__precious_block!();
crate::impl_client_v17__prune_blockchain!();
crate::impl_client_v17__save_mempool!();
crate::impl_client_v17__verify_chain!();
crate::impl_client_v17__verify_tx_out_proof!();

// == Control ==
crate::impl_client_v17__get_memory_info!();
crate::impl_client_v17__help!();
crate::impl_client_v17__logging!();
crate::impl_client_v17__stop!();
crate::impl_client_v17__uptime!();

// == Generating ==
crate::impl_client_v17__generate_to_address!();
crate::impl_client_v17__generate!();
crate::impl_client_v17__invalidate_block!();

// == Hidden ==
crate::impl_client_v17__estimate_raw_fee!();
crate::impl_client_v17__wait_for_block!();
crate::impl_client_v17__wait_for_block_height!();
crate::impl_client_v17__wait_for_new_block!();
crate::impl_client_v17__sync_with_validation_interface_queue!();

// == Mining ==
crate::impl_client_v17__get_block_template!();
crate::impl_client_v17__get_mining_info!();
crate::impl_client_v17__get_network_hashes_per_second!();
crate::impl_client_v17__prioritise_transaction!();
crate::impl_client_v17__submit_block!();

// == Network ==
crate::impl_client_v17__add_node!();
crate::impl_client_v17__clear_banned!();
crate::impl_client_v17__disconnect_node!();
crate::impl_client_v17__get_added_node_info!();
crate::impl_client_v17__get_connection_count!();
crate::impl_client_v17__get_net_totals!();
crate::impl_client_v17__get_network_info!();
crate::impl_client_v17__get_peer_info!();
crate::impl_client_v17__list_banned!();
crate::impl_client_v17__ping!();
crate::impl_client_v17__set_ban!();
crate::impl_client_v17__set_network_active!();

// == Rawtransactions ==
crate::impl_client_v17__combine_psbt!();
crate::impl_client_v17__combine_raw_transaction!();
crate::impl_client_v17__convert_to_psbt!();
crate::impl_client_v17__create_psbt!();
crate::impl_client_v17__create_raw_transaction!();
crate::impl_client_v17__decode_psbt!();
crate::impl_client_v17__decode_raw_transaction!();
crate::impl_client_v17__decode_script!();
crate::impl_client_v17__finalize_psbt!();
crate::impl_client_v17__fund_raw_transaction!();
crate::impl_client_v17__get_raw_transaction!();
crate::impl_client_v17__send_raw_transaction!();
crate::impl_client_v17__sign_raw_transaction!();
crate::impl_client_v17__sign_raw_transaction_with_key!();
crate::impl_client_v17__test_mempool_accept!();

// == Util ==
crate::impl_client_v17__create_multisig!();
crate::impl_client_v17__estimate_smart_fee!();
crate::impl_client_v17__sign_message_with_priv_key!();
crate::impl_client_v17__validate_address!();
crate::impl_client_v17__verify_message!();

// == Wallet ==
crate::impl_client_v17__abandon_transaction!();
crate::impl_client_v17__abort_rescan!();
crate::impl_client_v17__add_multisig_address!();
crate::impl_client_v17__backup_wallet!();
crate::impl_client_v17__bump_fee!();
crate::impl_client_v17__create_wallet!();
crate::impl_client_v17__dump_priv_key!();
crate::impl_client_v17__dump_wallet!();
crate::impl_client_v17__encrypt_wallet!();
crate::impl_client_v17__get_addresses_by_label!();
crate::impl_client_v17__get_address_info!();
crate::impl_client_v17__get_balance!();
crate::impl_client_v17__get_new_address!();
crate::impl_client_v17__get_raw_change_address!();
crate::impl_client_v17__get_received_by_address!();
crate::impl_client_v17__get_transaction!();
crate::impl_client_v17__get_unconfirmed_balance!();
crate::impl_client_v17__get_wallet_info!();
crate::impl_client_v17__import_address!();
crate::impl_client_v17__import_multi!();
crate::impl_client_v17__import_privkey!();
crate::impl_client_v17__import_pruned_funds!();
crate::impl_client_v17__import_pubkey!();
crate::impl_client_v17__import_wallet!();
crate::impl_client_v17__key_pool_refill!();
crate::impl_client_v17__list_address_groupings!();
crate::impl_client_v17__list_labels!();
crate::impl_client_v17__list_lock_unspent!();
crate::impl_client_v17__list_received_by_address!();
crate::impl_client_v17__list_since_block!();
crate::impl_client_v17__list_transactions!();
crate::impl_client_v17__list_unspent!();
crate::impl_client_v17__list_wallets!();
crate::impl_client_v17__load_wallet!();
crate::impl_client_v17__lock_unspent!();
crate::impl_client_v17__remove_pruned_funds!();
crate::impl_client_v17__rescan_blockchain!();
crate::impl_client_v17__send_many!();
crate::impl_client_v17__send_to_address!();
crate::impl_client_v17__set_hd_seed!();
crate::impl_client_v17__set_tx_fee!();
crate::impl_client_v17__sign_message!();
crate::impl_client_v17__sign_raw_transaction_with_wallet!();
crate::impl_client_v17__unload_wallet!();
crate::impl_client_v17__wallet_create_funded_psbt!();
crate::impl_client_v17__wallet_lock!();
crate::impl_client_v17__wallet_passphrase!();
crate::impl_client_v17__wallet_passphrase_change!();
crate::impl_client_v17__wallet_process_psbt!();

// == Zmq ==
crate::impl_client_v17__get_zmq_notifications!();

/// Argument to the `Client::get_new_address_with_type` function.
///
/// For Core versions 0.17 through to v22. For Core v23 and onwards use `v23::AddressType`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AddressType {
    Legacy,
    P2shSegwit,
    Bech32,
}

impl fmt::Display for AddressType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use AddressType::*;

        let s = match *self {
            Legacy => "legacy",
            P2shSegwit => "p2sh-segwit",
            Bech32 => "bech32",
        };
        fmt::Display::fmt(s, f)
    }
}

/// Arg for the `getblocktemplate` method.
///
/// For Core versions 0.17 through to v28. For Core v29 and onwards use `v29::TemplateRequest`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TemplateRequest {
    /// A list of strings.
    pub rules: Vec<TemplateRules>,
}

/// Client side supported softfork deployment.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TemplateRules {
    /// SegWit v0 supported.
    Segwit,
    /// Signet supported.
    Signet,
    /// CSV supported.
    Csv,
    /// Taproot supported.
    Taproot,
}

/// Input used as parameter to `create_raw_transaction`.
#[derive(Debug, Serialize)]
pub struct Input {
    /// The txid of the transaction that contains the UTXO.
    pub txid: bitcoin::Txid,
    /// The vout for the UTXO.
    pub vout: u64,
    /// Sequence number if needed.
    pub sequence: Option<bitcoin::Sequence>,
}

/// Output used as parameter to `create_raw_transaction`.
// Abuse `HashMap` so we can derive serialize to get the correct JSON object.
#[derive(Debug, Serialize)]
pub struct Output(
    /// Map of address to value. Always only has a single item in it.
    HashMap<String, f64>,
);

impl Output {
    /// Creates a single output that serializes as Core expects.
    pub fn new(addr: Address, value: Amount) -> Self {
        let mut map = HashMap::new();
        map.insert(addr.to_string(), value.to_btc());
        Output(map)
    }
}

/// An element in the `inputs` argument of method `walletcreatefundedpsbt`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletCreateFundedPsbtInput {
    txid: Txid,
    vout: u32,
}

impl WalletCreateFundedPsbtInput {
    /// Create a new walletcreatefundedpsbt input entry.
    pub fn new(txid: Txid, vout: u32) -> Self { Self { txid, vout } }
}

/// Args for the `addnode` method.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AddNodeCommand {
    Add,
    Remove,
    OneTry,
}

/// Args for the `setban` method.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SetBanCommand {
    Add,
    Remove,
}

/// Args for the `importmulti` method.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ImportMultiRequest {
    /// Descriptor to import. If using descriptor, do not also provide address/scriptPubKey, scripts, or pubkeys.
    #[serde(rename = "desc", skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>, // from core v18 onwards.
    /// Type of scriptPubKey (string for script, json for address). Should not be provided if using descriptor.
    #[serde(rename = "scriptPubKey", skip_serializing_if = "Option::is_none")]
    pub script_pubkey: Option<ImportMultiScriptPubKey>,
    /// Creation time of the key expressed in UNIX epoch time, or the string "now" to substitute the current synced blockchain time.
    pub timestamp: ImportMultiTimestamp,
}

/// `scriptPubKey` can be a string for script or json for address.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ImportMultiScriptPubKey {
    /// The script.
    Script(String),
    /// The address.
    Address { address: String },
}

/// `timestamp` can be a number (UNIX epoch time) or the string `"now"`.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ImportMultiTimestamp {
    /// The string "now".
    Now,
    /// The UNIX timestamp.
    Time(u64),
}

impl Serialize for ImportMultiTimestamp {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ImportMultiTimestamp::Now => serializer.serialize_str("now"),
            ImportMultiTimestamp::Time(t) => serializer.serialize_u64(*t),
        }
    }
}
