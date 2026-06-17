// SPDX-License-Identifier: CC0-1.0

//! Tests derived from Bitcoin Core rpc_blockchain.py, rpc_getchaintips.py,
//! rpc_txoutproof.py, rpc_scantxoutset.py, rpc_gettxspendingprevout.py,
//! rpc_dumptxoutset.py and rpc_getblockstats.py

use bitcoind::mtype;
use bitcoind::vtype::*;
use integration_test::{BitcoinD, BitcoinDExt as _, Wallet};

#[test]
fn get_blockchain_info_pruned_node_has_prune_fields() {
    let node = BitcoinD::with_wallet(Wallet::Default, &["-prune=550"]);
    node.fund_wallet();
    let out: GetBlockchainInfo = node.client.get_blockchain_info().unwrap();

    assert!(out.pruned);
    assert!(out.prune_height.is_some());
    assert!(out.automatic_pruning.is_some());
    assert!(out.prune_target_size.is_some());
}

#[test]
fn get_chain_tx_stats_default_window_has_optional_fields() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    let addr = node.client.new_address().unwrap();
    node.client.generate_to_address(200, &addr).unwrap();

    let json: GetChainTxStats = node.client.get_chain_tx_stats().unwrap();
    let stats: mtype::GetChainTxStats = json.into_model().unwrap();

    #[cfg(not(feature = "v18_and_below"))]
    assert!(stats.window_final_block_height.is_some());
    assert!(stats.window_tx_count.is_some());
    assert!(stats.window_interval.is_some());
    assert!(stats.tx_rate.is_some());
}

#[test]
fn get_chain_tx_stats_pinned_to_block1_has_no_window_fields() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    let addr = node.client.new_address().unwrap();
    node.client.generate_to_address(200, &addr).unwrap();

    let b1 = node.client.get_block_hash(1).unwrap().block_hash().unwrap();

    let json: GetChainTxStats = node
        .client
        .call(
            "getchaintxstats",
            &[bitcoind::serde_json::Value::Null, bitcoind::serde_json::json!(b1.to_string())],
        )
        .unwrap();
    let stats: mtype::GetChainTxStats = json.into_model().unwrap();

    assert!(stats.window_tx_count.is_none());
    assert!(stats.window_interval.is_none());
    assert!(stats.tx_rate.is_none());
}

#[test]
fn get_tx_out_set_info_after_mining() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let height = node.client.get_block_count().unwrap().0;
    let json: GetTxOutSetInfo = node.client.get_tx_out_set_info().unwrap();

    assert_eq!(json.height, height as i64);
    #[cfg(not(feature = "v25_and_below"))]
    assert!(json.transactions.is_some());
    #[cfg(all(not(feature = "v25_and_below"), feature = "v28_and_below"))]
    assert!(json.disk_size.is_some());
}

#[test]
fn get_tx_out_set_info_at_genesis() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let b1 = node.client.get_block_hash(1).unwrap().block_hash().unwrap();
    node.client.invalidate_block(b1).unwrap();

    let json: GetTxOutSetInfo = node.client.get_tx_out_set_info().unwrap();

    assert_eq!(json.height, 0);
    #[cfg(not(feature = "v25_and_below"))]
    assert!(json.hash_serialized_3.is_some());
}

#[test]
#[cfg(not(feature = "v25_and_below"))]
fn get_tx_out_set_info_muhash() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let json: GetTxOutSetInfo =
        node.client.call("gettxoutsetinfo", &[bitcoind::serde_json::json!("muhash")]).unwrap();

    assert!(json.muhash.is_some());
}

#[test]
fn get_tx_out_coinbase_fields() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let best = node.client.best_block_hash().unwrap();
    let block = node.client.get_block(best).unwrap();
    let coinbase = block.txdata[0].compute_txid();

    let json: GetTxOut = node.client.get_tx_out(coinbase, 0).unwrap();

    assert_eq!(json.confirmations, 1);
    assert!(json.coinbase);
    assert_eq!(json.best_block, best.to_string());
}

#[test]
fn get_block_header_verbose_fields() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let best = node.client.best_block_hash().unwrap();
    let height = node.client.get_block_count().unwrap().0;
    let prev = node.client.get_block_hash(height - 1).unwrap().block_hash().unwrap();

    let json: GetBlockHeaderVerbose = node.client.get_block_header_verbose(&best).unwrap();

    assert_eq!(json.hash, best.to_string());
    assert_eq!(json.confirmations, 1);
    assert_eq!(json.height, height as i64);
    assert_eq!(json.previous_block_hash.as_deref(), Some(prev.to_string().as_str()));
}

#[test]
fn get_block_hash_at_tip_matches_best() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let best = node.client.best_block_hash().unwrap();
    let height = node.client.get_block_count().unwrap().0;
    let at_height = node.client.get_block_hash(height).unwrap().block_hash().unwrap();

    assert_eq!(at_height, best);
}

#[test]
fn get_block_verbose_one_tx_count_matches_n_tx() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    node.create_mined_transaction();
    let hash = node.client.best_block_hash().unwrap();

    let json: GetBlockVerboseOne = node.client.get_block_verbose_one(hash).unwrap();

    assert_eq!(json.tx.len(), json.n_tx as usize);
    assert!(json.previous_block_hash.is_some());
}

#[test]
fn get_chain_tips_active_tip_matches_best() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let best = node.client.best_block_hash().unwrap();
    let height = node.client.get_block_count().unwrap().0;
    let json: GetChainTips = node.client.get_chain_tips().unwrap();

    let tip = json.0.iter().find(|t| t.status == ChainTipsStatus::Active).unwrap();
    assert_eq!(tip.hash, best.to_string());
    assert_eq!(tip.height, height as i64);
    assert_eq!(tip.branch_length, 0);
}

#[test]
fn get_mempool_entry_height_and_no_parents() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_, txid) = node.create_mempool_transaction();

    let height = node.client.get_block_count().unwrap().0;
    let json: GetMempoolEntry = node.client.get_mempool_entry(txid).unwrap();

    assert_eq!(json.0.height, height as i64);
    assert!(json.0.depends.is_empty());
}

#[test]
fn verify_tx_out_proof_round_trips_txid() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    let proof = node.client.get_tx_out_proof(&[txid]).unwrap();
    let json: VerifyTxOutProof = node.client.verify_tx_out_proof(&proof).unwrap();

    assert_eq!(json.0[0], txid.to_string());
}

#[test]
#[cfg(not(feature = "v18_and_below"))]
fn scan_tx_out_set_best_block_matches_tip() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let best = node.client.best_block_hash().unwrap();
    let height = node.client.get_block_count().unwrap().0;

    let desc = "pkh(0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798)";
    let json: ScanTxOutSetStart = node.client.scan_tx_out_set_start(&[desc]).unwrap();

    assert_eq!(json.best_block, best.to_string());
    assert_eq!(json.height, height);
}

#[test]
#[cfg(not(feature = "v22_and_below"))]
fn get_deployment_info_genesis_vs_tip() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let genesis = node.client.get_block_hash(0).unwrap().block_hash().unwrap();
    let tip = node.client.best_block_hash().unwrap();

    let g: GetDeploymentInfo = node.client.get_deployment_info(&genesis).unwrap();
    let t: GetDeploymentInfo = node.client.get_deployment_info_tip().unwrap();

    assert_eq!(g.hash, genesis.to_string());
    assert_eq!(g.height, 0);
    assert_eq!(t.hash, tip.to_string());
}

#[test]
#[cfg(not(feature = "v23_and_below"))]
fn get_tx_spending_prevout_for_unspent_output() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_, txid) = node.create_mempool_transaction();

    let outpoints = vec![bitcoin::OutPoint { txid, vout: 0 }];
    #[cfg(feature = "v30_and_below")]
    let json: GetTxSpendingPrevout = node.client.get_tx_spending_prevout(&outpoints).unwrap();
    #[cfg(not(feature = "v30_and_below"))]
    let json: GetTxSpendingPrevout = node.client.get_tx_spending_prevout(&outpoints).unwrap();

    assert_eq!(json.0.len(), 1);
    assert_eq!(json.0[0].txid, txid.to_string());
    assert_eq!(json.0[0].vout, 0);
    assert!(json.0[0].spending_txid.is_none());
}

#[test]
#[cfg(not(feature = "v28_and_below"))]
fn get_block_verbose_two_non_coinbase_has_fee() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_, mined) = node.create_mined_transaction();
    let hash = node.client.best_block_hash().unwrap();

    let json: GetBlockVerboseTwo = node.client.get_block_verbose_two(hash).unwrap();

    let mined_id = mined.compute_txid().to_string();
    let entry = json.tx.iter().find(|t| t.transaction.txid == mined_id).unwrap();
    assert!(entry.fee.is_some());
}

#[test]
#[cfg(not(feature = "v28_and_below"))]
fn get_block_verbose_three_non_coinbase_has_prevouts() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_, mined) = node.create_mined_transaction();
    let hash = node.client.best_block_hash().unwrap();

    let json: GetBlockVerboseThree = node.client.get_block_verbose_three(hash).unwrap();

    let mined_id = mined.compute_txid().to_string();
    let entry = json.tx.iter().find(|t| t.transaction.txid == mined_id).unwrap();

    assert!(entry.transaction.inputs.iter().any(|v| v.prevout.is_some()));
}

#[test]
fn get_mempool_entry_spent_by_contains_child() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_, parent) = node.create_mempool_transaction();
    let child = create_child_spending_parent(&node, parent);

    let json: GetMempoolEntry = node.client.get_mempool_entry(parent).unwrap();

    assert!(json.0.spent_by.iter().any(|t| *t == child.to_string()));
}

#[test]
#[cfg(not(feature = "v20_and_below"))]
fn get_raw_mempool_sequence_includes_tx() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_, txid) = node.create_mempool_transaction();

    let json: GetRawMempoolSequence = node.client.get_raw_mempool_sequence().unwrap();

    assert!(json.txids.iter().any(|t| *t == txid.to_string()));
    assert!(json.mempool_sequence > 0);
}

#[test]
fn get_mempool_ancestors_verbose_keyed_by_parent_txid() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_, parent) = node.create_mempool_transaction();
    let child = create_child_spending_parent(&node, parent);

    let json: GetMempoolAncestorsVerbose =
        node.client.get_mempool_ancestors_verbose(child).unwrap();

    assert!(json.0.contains_key(&parent.to_string()));
}

#[test]
fn get_block_verbose_one_stripped_size_le_size() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    node.create_mined_transaction();
    let best = node.client.best_block_hash().unwrap();

    let json: GetBlockVerboseOne = node.client.get_block_verbose_one(best).unwrap();

    assert!(json.stripped_size.is_some());
    assert!(json.stripped_size.unwrap() <= json.size);
}

#[test]
fn get_block_verbose_one_next_block_hash_some_for_non_tip() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let b1 = node.client.get_block_hash(1).unwrap().block_hash().unwrap();
    let json: GetBlockVerboseOne = node.client.get_block_verbose_one(b1).unwrap();

    assert!(json.next_block_hash.is_some());
}

#[test]
fn get_chain_tips_after_invalidate_block_has_stale_tip() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let tip = node.client.best_block_hash().unwrap();
    node.client.invalidate_block(tip).unwrap();
    node.mine_a_block();

    let json: GetChainTips = node.client.get_chain_tips().unwrap();

    let stale = json.0.iter().find(|t| t.status == ChainTipsStatus::Invalid).unwrap();
    assert_eq!(stale.branch_length, 1);
}

#[test]
#[cfg(not(feature = "v25_and_below"))]
fn dump_tx_out_set_fields_consistent() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let temp = integration_test::random_tmp_file();
    let path = temp.to_str().unwrap();

    #[cfg(feature = "v28_and_below")]
    let json: DumpTxOutSet = node.client.dump_tx_out_set(path).unwrap();
    #[cfg(not(feature = "v28_and_below"))]
    let json: DumpTxOutSet = node.client.dump_tx_out_set(path, "latest").unwrap();

    let tip = node.client.get_block_count().unwrap().0;

    assert_eq!(json.base_height, tip as i64);
    assert!(json.n_chain_tx > 0);
    assert!(!json.tx_out_set_hash.is_empty());
}

#[test]
#[cfg(not(feature = "v24_and_below"))]
fn get_block_stats_genesis_utxo_fields() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);

    let json: GetBlockStats = node.client.get_block_stats_by_height(0, None).unwrap();

    assert_eq!(json.utxo_increase, Some(1));
    assert!(json.utxo_size_increase.is_some());
}

#[test]
#[cfg(not(feature = "v24_and_below"))]
fn get_block_stats_v25_actual_utxo_fields() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let tip = node.client.get_block_count().unwrap().0;

    let json: GetBlockStats = node.client.get_block_stats_by_height(tip as u32, None).unwrap();

    assert!(json.utxo_increase_actual.is_some());
    assert!(json.utxo_size_increase_actual.is_some());
}

fn create_child_spending_parent(node: &BitcoinD, parent: bitcoin::Txid) -> bitcoin::Txid {
    use bitcoind::{Input, Output};
    let inputs = vec![Input { txid: parent, vout: 0, sequence: None }];
    let addr = node.client.new_address().unwrap();
    let outputs = vec![Output::new(addr, bitcoin::Amount::from_sat(100_000))];

    let raw: CreateRawTransaction = node.client.create_raw_transaction(&inputs, &outputs).unwrap();
    let unsigned = raw.transaction().unwrap();

    let funded: FundRawTransaction = node.client.fund_raw_transaction(&unsigned).unwrap();
    let funded_tx = funded.transaction().unwrap();

    let signed: SignRawTransaction =
        node.client.sign_raw_transaction_with_wallet(&funded_tx).unwrap();
    let model = signed.into_model().unwrap();
    let child = model.tx.compute_txid();
    node.client.send_raw_transaction(&model.tx).unwrap();
    child
}
