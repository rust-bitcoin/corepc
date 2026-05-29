// SPDX-License-Identifier: CC0-1.0

//! Tests derived from Bitcoin Core mining_basic.py, rpc_generate.py

#[cfg(not(feature = "v25_and_below"))]
use bitcoin::SignedAmount;
use bitcoind::vtype::*;
use bitcoind::{TemplateRequest, TemplateRules};
use integration_test::{BitcoinD, BitcoinDExt as _, Wallet};

#[test]
#[cfg(not(feature = "v25_and_below"))]
fn get_prioritised_transactions_round_trips_fee_delta() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_, txid) = node.create_mempool_transaction();

    let fee_delta = SignedAmount::from_sat(10_000);
    node.client.prioritise_transaction(&txid, fee_delta).unwrap();

    let json: GetPrioritisedTransactions = node.client.get_prioritised_transactions().unwrap();

    let entry = json.0.get(&txid.to_string()).unwrap();
    assert_eq!(entry.fee_delta, fee_delta.to_sat());
}

#[test]
fn generate_to_address_hash_matches_best_block() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    let addr = node.client.new_address().unwrap();

    let json: GenerateToAddress = node.client.generate_to_address(1, &addr).unwrap();
    let hashes = json.into_model().unwrap();

    assert_eq!(hashes.0[0], node.client.best_block_hash().unwrap());
}

#[test]
#[cfg(not(feature = "v20_and_below"))]
fn generate_block_with_empty_tx_list() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    let addr = node.client.new_address().unwrap();

    #[cfg(feature = "v24_and_below")]
    let json: GenerateBlock = node.client.generate_block(&addr.to_string(), &[]).unwrap();

    #[cfg(not(feature = "v24_and_below"))]
    let json: GenerateBlock = node.client.generate_block(&addr.to_string(), &[], true).unwrap();

    let block_hash: bitcoin::BlockHash = json.hash.parse().unwrap();
    assert_eq!(block_hash, node.client.best_block_hash().unwrap());
}

#[test]
fn get_block_template_has_optional_fields() {
    let (node1, _node2, _node3) = integration_test::three_node_network();
    node1.fund_wallet();

    #[cfg(feature = "v28_and_below")]
    let options = TemplateRequest { rules: vec![TemplateRules::Segwit] };
    #[cfg(not(feature = "v28_and_below"))]
    let options = TemplateRequest {
        rules: vec![TemplateRules::Segwit],
        mode: Some("template".to_string()),
        ..Default::default()
    };

    let json: GetBlockTemplate = node1.client.get_block_template(&options).unwrap();

    assert!(json.long_poll_id.is_some());
    assert!(json.default_witness_commitment.is_some());
}

#[test]
fn get_block_template_includes_mempool_tx() {
    let (node1, _node2, _node3) = integration_test::three_node_network();
    node1.fund_wallet();
    let (_, txid) = node1.create_mempool_transaction();

    #[cfg(feature = "v28_and_below")]
    let options = TemplateRequest { rules: vec![TemplateRules::Segwit] };
    #[cfg(not(feature = "v28_and_below"))]
    let options = TemplateRequest {
        rules: vec![TemplateRules::Segwit],
        mode: Some("template".to_string()),
        ..Default::default()
    };

    let json: GetBlockTemplate = node1.client.get_block_template(&options).unwrap();

    assert!(json.transactions.iter().any(|t| t.txid == txid.to_string()));
}
