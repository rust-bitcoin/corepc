// SPDX-License-Identifier: CC0-1.0

//! A JSON-RPC client for testing against Bitcoin Core `v26`.
//!
//! We ignore option arguments unless they effect the shape of the returned JSON data.

pub mod blockchain;
pub mod mining;
pub mod raw_transactions;

use std::collections::BTreeMap;
use std::path::Path;

use bitcoin::address::{Address, NetworkChecked};
use bitcoin::{sign_message, Amount, Block, BlockHash, PublicKey, Txid};

use crate::client_sync::into_json;
use crate::types::v26::*;

#[rustfmt::skip]                // Keep public re-exports separate.
pub use crate::client_sync::{
    v17::{
        AddNodeCommand, Input, Output, SetBanCommand, TemplateRequest, TemplateRules,
        WalletCreateFundedPsbtInput
    },
    v23::AddressType,
};

crate::define_jsonrpc_minreq_client!("v26");
crate::impl_client_check_expected_server_version!({ [260000, 260100, 260200] });

// == Blockchain ==
crate::impl_client_v17__get_best_block_hash!();
crate::impl_client_v17__get_block!();
crate::impl_client_v17__get_blockchain_info!();
crate::impl_client_v17__get_block_count!();
crate::impl_client_v19__get_block_filter!();
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
crate::impl_client_v26__get_tx_out_set_info!();
crate::impl_client_v17__precious_block!();
crate::impl_client_v17__prune_blockchain!();
crate::impl_client_v23__save_mempool!();
crate::impl_client_v17__verify_chain!();
crate::impl_client_v17__verify_tx_out_proof!();

// == Control ==
crate::impl_client_v17__get_memory_info!();
crate::impl_client_v18__get_rpc_info!();
crate::impl_client_v17__help!();
crate::impl_client_v17__logging!();
crate::impl_client_v17__stop!();
crate::impl_client_v17__uptime!();

// == Generating ==
crate::impl_client_v17__generate_to_address!();
crate::impl_client_v17__invalidate_block!();

// == Mining ==
crate::impl_client_v17__get_block_template!();
crate::impl_client_v17__get_mining_info!();
crate::impl_client_v17__get_network_hashes_per_second!();
crate::impl_client_v26__get_prioritised_transactions!();
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
crate::impl_client_v18__get_node_addresses!();
crate::impl_client_v17__get_peer_info!();
crate::impl_client_v17__set_ban!();

// == Rawtransactions ==
crate::impl_client_v18__analyze_psbt!();
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
crate::impl_client_v18__join_psbts!();
crate::impl_client_v17__send_raw_transaction!();
crate::impl_client_v17__sign_raw_transaction!();
crate::impl_client_v17__sign_raw_transaction_with_key!();
crate::impl_client_v26__submit_package!();
crate::impl_client_v17__test_mempool_accept!();
crate::impl_client_v18__utxo_update_psbt!();

// == Util ==
crate::impl_client_v17__create_multisig!();
crate::impl_client_v17__estimate_smart_fee!();
crate::impl_client_v17__sign_message_with_priv_key!();
crate::impl_client_v17__validate_address!();
crate::impl_client_v17__verify_message!();

// == Wallet ==
crate::impl_client_v17__add_multisig_address!();
crate::impl_client_v17__bump_fee!();
crate::impl_client_v23__create_wallet!();
crate::impl_client_v17__dump_priv_key!();
crate::impl_client_v17__dump_wallet!();
crate::impl_client_v17__get_addresses_by_label!();
crate::impl_client_v17__get_address_info!();
crate::impl_client_v17__get_balance!();
crate::impl_client_v19__get_balances!();
crate::impl_client_v17__get_new_address!();
crate::impl_client_v17__get_raw_change_address!();
crate::impl_client_v17__get_received_by_address!();
crate::impl_client_v17__get_transaction!();
crate::impl_client_v17__get_unconfirmed_balance!();
crate::impl_client_v17__get_wallet_info!();
crate::impl_client_v17__list_address_groupings!();
crate::impl_client_v17__list_labels!();
crate::impl_client_v17__list_lock_unspent!();
crate::impl_client_v17__list_received_by_address!();
crate::impl_client_v17__list_since_block!();
crate::impl_client_v17__list_transactions!();
crate::impl_client_v17__list_unspent!();
crate::impl_client_v17__list_wallets!();
crate::impl_client_v22__load_wallet!();
crate::impl_client_v17__rescan_blockchain!();
crate::impl_client_v17__send_many!();
crate::impl_client_v17__send_to_address!();
crate::impl_client_v17__sign_message!();
crate::impl_client_v17__sign_raw_transaction_with_wallet!();
crate::impl_client_v21__unload_wallet!();
crate::impl_client_v17__wallet_create_funded_psbt!();
crate::impl_client_v17__wallet_process_psbt!();
