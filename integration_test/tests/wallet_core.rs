// SPDX-License-Identifier: CC0-1.0

//! Tests derived from Bitcoin Core's wallet_basic.py, wallet_coinbase_category.py,
//! wallet_address_types.py, wallet_gethdkeys.py, wallet_listtransactions.py,
//! wallet_bumpfee.py, wallet_listdescriptors.py, wallet_listsinceblock.py,
//! wallet_groups.py, and wallet_listreceivedby.py

use bitcoin::Amount;
use bitcoind::vtype::*;
use bitcoind::AddressType;
use integration_test::{BitcoinD, BitcoinDExt as _, Wallet};

#[test]
#[cfg(not(feature = "v19_and_below"))]
fn get_transaction_block_fields_present_after_mining() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let addr = node.client.new_address().unwrap();
    let txid =
        node.client.send_to_address(&addr, Amount::from_sat(500_000)).unwrap().txid().unwrap();
    node.mine_a_block();

    let json: GetTransaction = node.client.get_transaction(txid).unwrap();

    assert!(json.block_hash.is_some());
    assert!(json.block_height.is_some());
    assert!(json.block_index.is_some());
    assert!(json.block_time.is_some());
}

#[test]
#[cfg(not(feature = "v25_and_below"))]
fn get_wallet_info_has_birthtime() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);

    let json: GetWalletInfo = node.client.get_wallet_info().unwrap();

    assert!(json.birthtime.is_some());
}

#[test]
fn get_transaction_send_has_fee_and_details() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let addr = node.client.new_address().unwrap();
    let txid =
        node.client.send_to_address(&addr, Amount::from_sat(500_000)).unwrap().txid().unwrap();
    node.mine_a_block();

    let json: GetTransaction = node.client.get_transaction(txid).unwrap();

    assert!(json.fee.is_some());
    assert!(!json.details.is_empty());
}

#[test]
#[cfg(not(feature = "v23_and_below"))]
fn list_transactions_send_entry_has_address_and_block() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let target = node.client.new_address().unwrap();
    node.client.send_to_address(&target, Amount::from_sat(123_456)).unwrap();
    node.mine_a_block();

    let json: ListTransactions = node.client.list_transactions().unwrap();

    let send = json.0.iter().rev().find(|t| t.address.is_some() && t.fee.is_some()).unwrap();
    assert!(send.block_hash.is_some());
    assert!(send.block_height.is_some());
}

#[test]
fn list_received_by_address_contains_sent_txid() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let target = node.client.new_address().unwrap();
    let txid =
        node.client.send_to_address(&target, Amount::from_sat(321_000)).unwrap().txid().unwrap();
    node.mine_a_block();

    let json: ListReceivedByAddress = node.client.list_received_by_address().unwrap();

    let entry = json.0.iter().find(|e| e.address == target.to_string()).unwrap();
    assert!(entry.txids.iter().any(|t| *t == txid.to_string()));
}

#[test]
#[cfg(not(feature = "v17"))]
fn list_unspent_utxos_have_descriptor() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let json: ListUnspent = node.client.list_unspent().unwrap();

    assert!(json.0.iter().all(|u| u.descriptor.is_some()));
}

#[test]
fn list_since_block_has_coinbase_with_generated_true() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let json: ListSinceBlock = node.client.list_since_block().unwrap();

    assert!(json.transactions.iter().any(|t| t.generated == Some(true)));
}

#[test]
fn get_address_info_bech32_has_witness_fields() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    let addr = node.client.new_address_with_type(AddressType::Bech32).unwrap();

    let json: GetAddressInfo = node.client.get_address_info(&addr).unwrap();

    assert!(json.is_witness);
    assert!(json.witness_version.is_some());
    assert!(json.witness_program.is_some());
}

#[test]
#[cfg(not(feature = "v27_and_below"))]
fn get_hd_keys_returns_xpriv_when_private() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);

    let json: GetHdKeys =
        node.client.call("gethdkeys", &[bitcoind::serde_json::json!({"private": true})]).unwrap();

    assert!(!json.0.is_empty());
    assert!(json.0[0].xpriv.is_some());
}

#[test]
#[cfg(not(feature = "v23_and_below"))]
fn list_transactions_labeled_address_has_label() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let label = "corepc-test";
    let addr = node.client.new_address_with_label(label).unwrap().assume_checked();
    node.client.send_to_address(&addr, Amount::from_sat(10_000)).unwrap();
    node.mine_a_block();

    let json: ListTransactions = node.client.list_transactions().unwrap();

    let entry = json
        .0
        .iter()
        .find(|t| t.address.as_deref() == Some(addr.to_string().as_str()) && t.label.is_some());
    assert!(entry.is_some(), "receive entry with label should exist");
    assert_eq!(entry.unwrap().label.as_deref(), Some(label));
}

#[test]
fn bump_fee_original_fee_rename_and_cross_field() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let addr = node.client.new_address().unwrap();
    let old_txid =
        node.client.send_to_address_rbf(&addr, Amount::from_sat(50_000)).unwrap().txid().unwrap();

    let json: BumpFee = node.client.bump_fee(old_txid).unwrap();

    assert!(json.original_fee > 0.0);
    assert!(json.fee >= json.original_fee);
    assert_ne!(json.txid, old_txid.to_string());
}

#[test]
#[cfg(not(feature = "v25_and_below"))]
fn get_wallet_info_has_last_processed_block() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let json: GetWalletInfo = node.client.get_wallet_info().unwrap();

    assert!(json.last_processed_block.is_some());
}

#[test]
#[cfg(not(feature = "v25_and_below"))]
fn get_balances_has_last_processed_block() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let json: GetBalances = node.client.get_balances().unwrap();

    assert!(json.last_processed_block.is_some());
}

#[test]
#[cfg(not(feature = "v24_and_below"))]
fn list_descriptors_ranged_has_range_and_next_index() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let wallet_name = "desc_wallet_range";

    #[cfg(feature = "v22_and_below")]
    node.client.create_descriptor_wallet(wallet_name).unwrap();
    #[cfg(not(feature = "v22_and_below"))]
    node.client.create_wallet(wallet_name).unwrap();

    let json: ListDescriptors = node.client.list_descriptors().unwrap();

    let ranged = json.descriptors.iter().find(|d| d.range.is_some()).unwrap();
    assert!(ranged.next_index.is_some());
    let [start, end] = ranged.range.unwrap();
    assert!(end >= start);
}

#[test]
#[cfg(not(feature = "v21_and_below"))]
fn list_descriptors_active_has_internal_field() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let wallet_name = "desc_wallet_internal";

    #[cfg(feature = "v22_and_below")]
    node.client.create_descriptor_wallet(wallet_name).unwrap();
    #[cfg(not(feature = "v22_and_below"))]
    node.client.create_wallet(wallet_name).unwrap();

    let json: ListDescriptors = node.client.list_descriptors().unwrap();

    assert!(json.descriptors.iter().any(|d| d.active && d.internal.is_some()));
}

#[test]
#[cfg(not(feature = "v27_and_below"))]
fn get_address_info_has_parent_descriptor() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    let addr = node.client.new_address().unwrap();

    let json: GetAddressInfo = node.client.get_address_info(&addr).unwrap();

    assert!(json.parent_descriptor.is_some());
}

#[test]
fn get_transaction_send_detail_has_negative_fee() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let dest = node.client.new_address().unwrap();
    let txid =
        node.client.send_to_address(&dest, Amount::from_sat(50_000)).unwrap().txid().unwrap();
    node.mine_a_block();

    let json: GetTransaction = node.client.get_transaction(txid).unwrap();

    let send = json.details.iter().find(|d| d.category == TransactionCategory::Send).unwrap();
    assert!(send.fee.is_some());
    assert!(send.fee.unwrap() < 0.0);
}

#[test]
#[cfg(not(feature = "v23_and_below"))]
fn get_transaction_received_detail_has_parent_descs() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let dest = node.client.new_address().unwrap();
    let txid =
        node.client.send_to_address(&dest, Amount::from_sat(50_000)).unwrap().txid().unwrap();
    node.mine_a_block();

    let json: GetTransaction = node.client.get_transaction(txid).unwrap();

    let receive = json.details.iter().find(|d| d.category == TransactionCategory::Receive).unwrap();
    assert!(receive.parent_descriptors.is_some());
}

#[test]
fn list_since_block_last_block_is_best() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    node.mine_a_block();

    let best = node.client.best_block_hash().unwrap();
    let json: ListSinceBlock = node.client.list_since_block().unwrap();

    assert_eq!(json.last_block, best.to_string());
}

#[test]
fn list_address_groupings_labeled_uses_three_variant() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let label = "groupings-label";
    let addr = node.client.new_address_with_label(label).unwrap().assume_checked();
    let addr_s = addr.to_string();
    node.client.send_to_address(&addr, Amount::from_sat(25_000)).unwrap();
    node.mine_a_block();

    let json: ListAddressGroupings = node.client.list_address_groupings().unwrap();

    let entry = json
        .0
        .iter()
        .flat_map(|group| group.iter())
        .find(|item| match item {
            ListAddressGroupingsItem::Two(a, _) => a == &addr_s,
            ListAddressGroupingsItem::Three(a, _, _) => a == &addr_s,
        })
        .unwrap();
    match entry {
        ListAddressGroupingsItem::Three(_, _, l) => assert_eq!(l, label),
        ListAddressGroupingsItem::Two(_, _) => {
            panic!("labeled address must deserialize as Three variant, got Two")
        }
    }
}

#[test]
fn list_received_by_address_aggregates_amount_across_txids() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let target = node.client.new_address().unwrap();
    node.client.send_to_address(&target, Amount::from_sat(100_000)).unwrap();
    node.mine_a_block();
    node.client.send_to_address(&target, Amount::from_sat(200_000)).unwrap();
    node.mine_a_block();

    let json: ListReceivedByAddress = node.client.list_received_by_address().unwrap();

    let entry = json.0.iter().find(|e| e.address == target.to_string()).unwrap();

    assert_eq!(entry.txids.len(), 2);
    assert_eq!(entry.amount, 0.003);
}

#[test]
fn list_transactions_yields_both_send_and_receive() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let addr = node.client.new_address().unwrap();

    node.client.send_to_address(&addr, Amount::from_sat(15_000)).unwrap();
    node.mine_a_block();

    let json: ListTransactions = node.client.list_transactions().unwrap();

    assert!(json.0.iter().any(|t| t.category == TransactionCategory::Receive));
    assert!(json.0.iter().any(|t| t.category == TransactionCategory::Send));
}
