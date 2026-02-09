// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Blockchain ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use bitcoin::consensus::encode;
use bitcoin::hex;
use integration_test::{Node, NodeExt as _, Wallet};
use node::vtype::*; // All the version specific types.
use node::{mtype, Input, Output};

#[test]
#[cfg(not(feature = "v25_and_below"))]
fn blockchain__dump_tx_out_set__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _tx) = node.create_mined_transaction();

    let temp_path = integration_test::random_tmp_file();
    let path = temp_path.to_str().expect("temp path should be valid UTF-8");
    let json: DumpTxOutSet;
    #[cfg(feature = "v28_and_below")]
    {
        json = node.client.dump_tx_out_set(path).expect("dumptxoutset");
    }
    #[cfg(not(feature = "v28_and_below"))]
    {
        json = node.client.dump_tx_out_set(path, "latest").expect("dumptxoutset");
    }
    let model: Result<mtype::DumpTxOutSet, DumpTxOutSetError> = json.into_model();
    let dump = model.unwrap();

    assert!(dump.coins_written.to_sat() > 0);
}

#[test]
fn blockchain__get_best_block_hash__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetBestBlockHash = node.client.get_best_block_hash().expect("getbestblockhash");
    let model: Result<mtype::GetBestBlockHash, hex::HexToArrayError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_block__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");

    let json: GetBlockVerboseZero =
        node.client.get_block_verbose_zero(block_hash).expect("getblock verbose=0");
    let model: Result<mtype::GetBlockVerboseZero, encode::FromHexError> = json.into_model();
    model.unwrap();

    let json: GetBlockVerboseOne =
        node.client.get_block_verbose_one(block_hash).expect("getblock verbose=1");
    let model: Result<mtype::GetBlockVerboseOne, GetBlockVerboseOneError> = json.into_model();
    model.unwrap();

    #[cfg(not(feature = "v28_and_below"))]
    {
        let node = Node::with_wallet(Wallet::Default, &[]);
        node.fund_wallet();
        let (_address, mined_tx) = node.create_mined_transaction();
        let block_hash = node.client.best_block_hash().expect("best_block_hash failed");

        let json: GetBlockVerboseTwo =
            node.client.get_block_verbose_two(block_hash).expect("getblock verbose=2");
        let model: Result<mtype::GetBlockVerboseTwo, GetBlockVerboseTwoError> = json.into_model();
        let block_v2 = model.unwrap();

        assert_eq!(block_v2.tx.len(), block_v2.n_tx as usize);

        let mined_txid = mined_tx.compute_txid();
        let mined_entry = block_v2
            .tx
            .iter()
            .find(|entry| entry.transaction.transaction.compute_txid() == mined_txid)
            .expect("mined transaction should be present in verbosity=2 results");
        assert!(mined_entry.fee.is_some());
        assert!(!mined_entry.transaction.transaction.input.is_empty());
        assert!(!mined_entry.transaction.transaction.output.is_empty());

        let json: GetBlockVerboseThree =
            node.client.get_block_verbose_three(block_hash).expect("getblock verbose=3");
        let model: Result<mtype::GetBlockVerboseThree, GetBlockVerboseThreeError> =
            json.into_model();
        let block_v3 = model.unwrap();

        assert_eq!(block_v3.tx.len(), block_v3.n_tx as usize);

        let mined_entry = block_v3
            .tx
            .iter()
            .find(|entry| entry.transaction.transaction.compute_txid() == mined_txid)
            .expect("mined transaction should be present in verbosity=3 results");
        assert!(
            mined_entry.prevouts.iter().any(|prevout| prevout.is_some()),
            "expected at least one prevout for the mined transaction"
        );

        for prevout in mined_entry.prevouts.iter().flatten() {
            assert!(prevout.value.to_sat() > 0);
            assert!(prevout.height <= block_v3.height);
            assert!(!prevout.script_pubkey.script_pubkey.is_empty());
        }
    }
}

#[test]
fn blockchain__get_blockchain_info__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetBlockchainInfo = node.client.get_blockchain_info().expect("rpc");
    let model: Result<mtype::GetBlockchainInfo, GetBlockchainInfoError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_block_count__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetBlockCount = node.client.get_block_count().unwrap();
    let _: mtype::GetBlockCount = json.into_model();
}

#[test]
#[cfg(not(feature = "v18_and_below"))]
fn blockchain__get_block_filter__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-blockfilterindex"]);
    node.mine_a_block();
    let hash = node.client.best_block_hash().expect("best_block_hash failed");

    let json: GetBlockFilter = node.client.get_block_filter(hash).expect("getblockfilter");
    let model: Result<mtype::GetBlockFilter, GetBlockFilterError> = json.into_model();
    model.unwrap();
}

#[test]
#[cfg(not(feature = "v22_and_below"))]
fn blockchain__get_block_from_peer() {
    use bitcoin::hashes::Hash;
    let (node1, _node2, _node3) = integration_test::three_node_network();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as u32;

    // Create a dummy header and submit it
    let mut header = bitcoin::block::Header {
        version: bitcoin::block::Version::from_consensus(0x20000000),
        prev_blockhash: node1.client.best_block_hash().expect("best_block_hash failed"),
        merkle_root: bitcoin::TxMerkleNode::all_zeros(),
        time: now,
        bits: bitcoin::CompactTarget::from_consensus(0x207fffff),
        nonce: 0,
    };
    let target = header.target();
    while header.validate_pow(target).is_err() {
        header.nonce += 1;
    }
    node1.client.submit_header(&header).expect("submit_header failed");

    let hash = header.block_hash();
    let peer_id = node1.client.get_peer_info().expect("getpeerinfo").0[0].id;
    let _: () = node1.client.get_block_from_peer(hash, peer_id).expect("getblockfrompeer");
}

#[test]
fn blockchain__get_block_hash__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetBlockHash = node.client.get_block_hash(0).expect("getblockhash");
    let model: Result<mtype::GetBlockHash, hex::HexToArrayError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_block_header__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");

    // verbose = false
    let json: GetBlockHeader = node.client.get_block_header(&block_hash).expect("getblockheader");
    let model: Result<mtype::GetBlockHeader, GetBlockHeaderError> = json.into_model();
    model.unwrap();

    // verbose = true
    let json: GetBlockHeaderVerbose =
        node.client.get_block_header_verbose(&block_hash).expect("getblockheader");
    let model: Result<mtype::GetBlockHeaderVerbose, GetBlockHeaderVerboseError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_block_stats__modelled() {
    // Version 17 and 18 cannot call `getblockstats` if `-txindex` is not enabled.
    #[cfg(not(feature = "v18_and_below"))]
    getblockstats();

    // All versions including 17 and 18 can `getblockstats` if `-txindex` is enabled.
    getblockstats_txindex();
}

#[cfg(not(feature = "v18_and_below"))]
fn getblockstats() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let json: GetBlockStats = node.client.get_block_stats_by_height(1).expect("getblockstats");
    let model: Result<mtype::GetBlockStats, GetBlockStatsError> = json.into_model();
    model.unwrap();

    // No need for explicit types, used explicitly in test below.
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");
    let json: GetBlockStats =
        node.client.get_block_stats_by_block_hash(&block_hash).expect("getblockstats");
    let model: Result<mtype::GetBlockStats, GetBlockStatsError> = json.into_model();
    model.unwrap();
}

fn getblockstats_txindex() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    // Get block stats by height.
    let json: GetBlockStats = node.client.get_block_stats_by_height(101).expect("getblockstats");
    let model: Result<mtype::GetBlockStats, GetBlockStatsError> = json.into_model();
    model.unwrap();

    // Get block stats by block hash.
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");
    let json: GetBlockStats =
        node.client.get_block_stats_by_block_hash(&block_hash).expect("getblockstats");
    let model: Result<mtype::GetBlockStats, GetBlockStatsError> = json.into_model();
    model.unwrap();
}

#[test]
#[cfg(not(feature = "v25_and_below"))]
fn blockchain__get_chain_states__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _tx) = node.create_mined_transaction();

    let json: GetChainStates = node.client.get_chain_states().expect("getchainstates");
    let model: Result<mtype::GetChainStates, GetChainStatesError> = json.into_model();
    let chain_states = model.unwrap();

    assert!(chain_states.chain_states[0].blocks > 0);
}

#[test]
fn blockchain__get_chain_tips__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetChainTips = node.client.get_chain_tips().expect("getchaintips");
    let model: Result<mtype::GetChainTips, ChainTipsError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_chain_tx_stats__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _tx) = node.create_mined_transaction();

    let json: GetChainTxStats = node.client.get_chain_tx_stats().expect("getchaintxstats");
    let model: Result<mtype::GetChainTxStats, GetChainTxStatsError> = json.into_model();
    let chain_tx_stats = model.unwrap();

    assert!(chain_tx_stats.tx_rate.unwrap() > 0.0);
}

#[test]
#[cfg(not(feature = "v22_and_below"))]
fn blockchain__get_deployment_info__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");

    let json: GetDeploymentInfo =
        node.client.get_deployment_info(&block_hash).expect("getdeploymentinfo");
    let model: Result<mtype::GetDeploymentInfo, GetDeploymentInfoError> = json.into_model();
    model.unwrap();
}

#[test]
#[cfg(not(feature = "v28_and_below"))]
fn blockchain__get_descriptor_activity__modelled() {
    let node = Node::with_wallet(Wallet::None, &["-coinstatsindex=1", "-txindex=1"]);

    let json: GetDescriptorActivity =
        node.client.get_descriptor_activity().expect("getdescriptoractivity");
    let model: Result<mtype::GetDescriptorActivity, GetDescriptorActivityError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_difficulty__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetDifficulty = node.client.get_difficulty().expect("getdifficulty");
    let _: mtype::GetDifficulty = json.into_model();
}

#[test]
fn blockchain__get_mempool_ancestors__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, parent_txid) = node.create_mempool_transaction();
    let child_txid = create_child_spending_parent(&node, parent_txid);

    let json: GetMempoolAncestors =
        node.client.get_mempool_ancestors(child_txid).expect("getmempoolancestors");
    let model: Result<mtype::GetMempoolAncestors, hex::HexToArrayError> = json.into_model();
    let ancestors = model.unwrap();

    assert!(ancestors.0.contains(&parent_txid));
}

#[test]
fn blockchain__get_mempool_ancestors_verbose__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, parent_txid) = node.create_mempool_transaction();
    let child_txid = create_child_spending_parent(&node, parent_txid);

    let json: GetMempoolAncestorsVerbose =
        node.client.get_mempool_ancestors_verbose(child_txid).expect("getmempoolancestors verbose");
    let model: Result<mtype::GetMempoolAncestorsVerbose, MapMempoolEntryError> = json.into_model();
    let ancestors = model.unwrap();

    assert!(ancestors.0.contains_key(&parent_txid));
}

#[test]
fn blockchain__get_mempool_descendants__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, parent_txid) = node.create_mempool_transaction();
    let child_txid = create_child_spending_parent(&node, parent_txid);

    let json: GetMempoolDescendants =
        node.client.get_mempool_descendants(parent_txid).expect("getmempooldescendants");
    let model: Result<mtype::GetMempoolDescendants, hex::HexToArrayError> = json.into_model();
    let descendants = model.unwrap();

    assert!(descendants.0.contains(&child_txid));
}

#[test]
fn blockchain__get_mempool_descendants_verbose__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, parent_txid) = node.create_mempool_transaction();
    let child_txid = create_child_spending_parent(&node, parent_txid);

    let json: GetMempoolDescendantsVerbose = node
        .client
        .get_mempool_descendants_verbose(parent_txid)
        .expect("getmempooldescendants verbose");
    let model: Result<mtype::GetMempoolDescendantsVerbose, MapMempoolEntryError> =
        json.into_model();
    let descendants = model.unwrap();

    assert!(descendants.0.contains_key(&child_txid));
}

#[test]
fn blockchain__get_mempool_entry__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, txid) = node.create_mempool_transaction();

    let json: GetMempoolEntry = node.client.get_mempool_entry(txid).expect("getmempoolentry");
    let model: Result<mtype::GetMempoolEntry, MempoolEntryError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_mempool_info__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _txid) = node.create_mempool_transaction();

    let json: GetMempoolInfo = node.client.get_mempool_info().expect("getmempoolinfo");
    let model: Result<mtype::GetMempoolInfo, GetMempoolInfoError> = json.clone().into_model();
    let info = model.unwrap();

    // Sanity check.
    assert_eq!(info.size, 1);
}

#[test]
fn blockchain__get_raw_mempool__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _txid) = node.create_mempool_transaction();

    // verbose = false + mempool_sequence = false
    let json: GetRawMempool = node.client.get_raw_mempool().expect("getrawmempool");
    let model: Result<mtype::GetRawMempool, hex::HexToArrayError> = json.clone().into_model();
    let mempool = model.unwrap();
    // Sanity check.
    assert_eq!(mempool.0.len(), 1);

    // verbose = true + mempool_sequence = false
    let json: GetRawMempoolVerbose =
        node.client.get_raw_mempool_verbose().expect("getrawmempool verbose");
    let model: Result<mtype::GetRawMempoolVerbose, MapMempoolEntryError> = json.into_model();
    let mempool = model.unwrap();
    // Sanity check.
    assert_eq!(mempool.0.len(), 1);

    #[cfg(not(feature = "v20_and_below"))]
    {
        // verbose = false + mempool_sequence = true
        let json: GetRawMempoolSequence =
            node.client.get_raw_mempool_sequence().expect("getrawmempool sequence");
        let model: Result<mtype::GetRawMempoolSequence, hex::HexToArrayError> = json.into_model();
        let mempool = model.unwrap();
        // Sanity check.
        assert_eq!(mempool.txids.len(), 1);
    }
}

#[test]
fn blockchain__get_tx_out__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    // Test the type and into model conversion code.
    let json: GetTxOut = node.client.get_tx_out(txid, 1).expect("gettxout");
    let model: Result<mtype::GetTxOut, GetTxOutError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_tx_out_proof() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    let _ = node.client.get_tx_out_proof(&[txid]).expect("gettxoutproof");
}

#[test]
fn blockchain__get_tx_out_set_info__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _tx) = node.create_mined_transaction();

    let json: GetTxOutSetInfo = node.client.get_tx_out_set_info().expect("gettxoutsetinfo");
    let model: Result<mtype::GetTxOutSetInfo, GetTxOutSetInfoError> = json.into_model();
    model.unwrap();
}

#[test]
#[cfg(not(feature = "v23_and_below"))]
fn blockchain__get_tx_spending_prevout__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let (_address1, txid_1) = node.create_mempool_transaction();
    let (_address2, txid_2) = node.create_mempool_transaction();

    let inputs = vec![
        bitcoin::OutPoint { txid: txid_1, vout: 0 },
        bitcoin::OutPoint { txid: txid_2, vout: 0 },
    ];

    let json: GetTxSpendingPrevout =
        node.client.get_tx_spending_prevout(&inputs).expect("gettxspendingprevout");
    let model: Result<mtype::GetTxSpendingPrevout, GetTxSpendingPrevoutError> = json.into_model();
    let spending_prevout = model.unwrap();

    assert_eq!(spending_prevout.0.len(), 2);
    assert_eq!(spending_prevout.0[0].outpoint.txid, txid_1);
    assert_eq!(spending_prevout.0[0].outpoint.vout, 0);
}

#[test]
#[cfg(not(feature = "v25_and_below"))]
fn blockchain__import_mempool() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _tx) = node.create_mined_transaction();

    let mempool_path = node.client.save_mempool().expect("savemempool");

    let _: () = node.client.import_mempool(&mempool_path.filename).expect("importmempool");
}

#[test]
fn blockchain__precious_block() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.mine_a_block();
    let hash = node.client.best_block_hash().expect("best_block_hash failed");
    node.mine_a_block();

    let _: () = node.client.precious_block(hash).expect("preciousblock");
}

#[test]
fn blockchain__prune_blockchain() {
    const NBLOCKS: usize = 1;

    let node = Node::with_wallet(Wallet::Default, &["-prune=550"]);
    let address = node.client.new_address().expect("Failed to get new address");

    let gen_result = node
        .client
        .generate_to_address(NBLOCKS, &address)
        .expect("generate_to_address RPC call failed");
    assert_eq!(
        gen_result.0.len(),
        NBLOCKS,
        "generate_to_address did not return the expected number of block hashes"
    );

    let target_height: u64 = 500;

    let _: Result<PruneBlockchain, _> = node.client.prune_blockchain(target_height);
}

#[test]
fn blockchain__savemempool() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_addr, _txid) = node.create_mempool_transaction();

    #[cfg(feature = "v22_and_below")]
    {
        let _: () = node.client.save_mempool().expect("savemempool");
    }

    #[cfg(not(feature = "v22_and_below"))]
    {
        let _: Result<SaveMempool, _> = node.client.save_mempool();
    }
}

#[test]
#[cfg(not(feature = "v24_and_below"))]
fn blockchain__scan_blocks_modelled() {
    let node = Node::with_wallet(Wallet::None, &["-blockfilterindex=1"]);

    // Arbitrary scan descriptor
    let scan_desc = "pkh(022afc20bf379bc96a2f4e9e63ffceb8652b2b6a097f63fbee6ecec2a49a48010e)";

    let json: ScanBlocksStart =
        node.client.scan_blocks_start(&[scan_desc]).expect("scanblocks start");
    let model: Result<mtype::ScanBlocksStart, ScanBlocksStartError> = json.into_model();
    let model = model.unwrap();

    let _: Option<ScanBlocksStatus> = node.client.scan_blocks_status().expect("scanblocks status");

    let _: ScanBlocksAbort = node.client.scan_blocks_abort().expect("scanblocks abort");

    assert!(model.from_height <= model.to_height);

    #[cfg(not(feature = "v25_and_below"))]
    {
        assert!(model.completed.is_some());
    }
}

#[test]
fn blockchain__scan_tx_out_set_modelled() {
    let node = match () {
        #[cfg(feature = "v21_and_below")]
        () => Node::with_wallet(Wallet::None, &[]),
        #[cfg(not(feature = "v21_and_below"))]
        () => Node::with_wallet(Wallet::None, &["-coinstatsindex=1"]),
    };

    let dummy_pubkey_hex = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    let scan_desc = format!("pkh({})", dummy_pubkey_hex);

    let json: ScanTxOutSetStart =
        node.client.scan_tx_out_set_start(&[&scan_desc]).expect("scantxoutset start");
    let model: Result<mtype::ScanTxOutSetStart, ScanTxOutSetError> = json.into_model();
    model.unwrap();

    let _: Option<ScanTxOutSetStatus> =
        node.client.scan_tx_out_set_status().expect("scantxoutset status");

    let _: ScanTxOutSetAbort = node.client.scan_tx_out_set_abort().expect("scantxoutset abort");
}

#[test]
fn blockchain__verify_chain() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let _: Result<VerifyChain, _> = node.client.verify_chain();
}

#[test]
fn blockchain__verify_tx_out_proof__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let (_address, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    let proof = node.client.get_tx_out_proof(&[txid]).expect("gettxoutproof");

    let json: VerifyTxOutProof = node.client.verify_tx_out_proof(&proof).expect("verifytxoutproof");
    let model: Result<mtype::VerifyTxOutProof, hex::HexToArrayError> = json.into_model();
    let txids = model.unwrap();

    // sanity check
    assert_eq!(txids.0.len(), 1);
}

#[test]
fn blockchain__wait_for_block__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _tx) = node.create_mined_transaction();
    let block_hash = node.client.best_block_hash().expect("bestblockhash");

    let json: WaitForBlock = node.client.wait_for_block(&block_hash).expect("waitforblock");
    let model: Result<mtype::WaitForBlock, WaitForBlockError> = json.into_model();
    let block = model.unwrap();
    assert_eq!(block.hash, block_hash);
}

#[test]
fn blockchain__wait_for_block_height__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _tx) = node.create_mined_transaction();
    let height = node.client.get_block_count().expect("getblockcount").0;
    let block_hash = node.client.best_block_hash().expect("bestblockhash");
    let target_height = height;

    let json: WaitForBlockHeight =
        node.client.wait_for_block_height(target_height).expect("waitforblockheight");
    let model: Result<mtype::WaitForBlockHeight, WaitForBlockHeightError> = json.into_model();
    let block = model.unwrap();
    assert_eq!(block.height, target_height as u32);
    assert_eq!(block.hash, block_hash);
}

#[test]
fn blockchain__wait_for_new_block__modelled() {
    let (node1, node2, _node3) = integration_test::three_node_network();
    node1.fund_wallet();
    node1.mine_a_block();

    let prev_hash = node1.client.best_block_hash().expect("bestblockhash");
    let prev_height = node1.client.get_block_count().expect("getblockcount").0;

    // Start waiting for a new block on node1 in a separate thread.
    let handle = std::thread::spawn(move || {
        let json: WaitForNewBlock = node1.client.wait_for_new_block().expect("waitfornewblock");
        let model: Result<mtype::WaitForNewBlock, WaitForNewBlockError> = json.into_model();
        model.unwrap()
    });
    std::thread::sleep(std::time::Duration::from_millis(200));

    // Trigger a new block on node2.
    node2.mine_a_block();

    let block = handle.join().expect("waitfornewblock thread panicked");
    assert_eq!(block.height, (prev_height + 1) as u32);
    assert_ne!(block.hash, prev_hash);
}

/// Create and broadcast a child transaction spending vout 0 of the given parent mempool txid.
/// Returns the child's txid.
fn create_child_spending_parent(node: &Node, parent_txid: bitcoin::Txid) -> bitcoin::Txid {
    let inputs = vec![Input { txid: parent_txid, vout: 0, sequence: None }];
    let spend_address = node.client.new_address().expect("newaddress");
    let outputs = vec![Output::new(spend_address, bitcoin::Amount::from_sat(100_000))];

    let raw: CreateRawTransaction =
        node.client.create_raw_transaction(&inputs, &outputs).expect("createrawtransaction");
    let unsigned = raw.transaction().expect("raw.transaction");

    let funded: FundRawTransaction =
        node.client.fund_raw_transaction(&unsigned).expect("fundrawtransaction");
    let funded_tx = funded.transaction().expect("funded.transaction");

    let signed: SignRawTransaction = node
        .client
        .sign_raw_transaction_with_wallet(&funded_tx)
        .expect("signrawtransactionwithwallet");
    let sign_raw_transaction =
        signed.into_model().expect("SignRawTransactionWithWallet into model");
    let child_txid = sign_raw_transaction.tx.compute_txid();
    let _ = node.client.send_raw_transaction(&sign_raw_transaction.tx).expect("sendrawtransaction");

    child_txid
}
