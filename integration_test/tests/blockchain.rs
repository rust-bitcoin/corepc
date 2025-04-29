// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Blockchain ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use integration_test::{Node, NodeExt as _, Wallet};
use node::client::client_sync;
use node::vtype::*;             // All the version specific types.
use node::mtype;

#[test]
fn blockchain__get_best_block_hash__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json = node.client.get_best_block_hash().expect("rpc");
    let model: Result<mtype::GetBestBlockHash, _> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_block__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");

    let json: GetBlockVerboseZero = node.client.get_block_verbose_zero(block_hash).expect("getblock verbose=0");
    let model: Result<mtype::GetBlockVerboseZero, _> = json.into_model();
    model.expect("GetBlock into model");

    let json: GetBlockVerboseOne = node.client.get_block_verbose_one(block_hash).expect("getblock verbose=1");
    let model: Result<mtype::GetBlockVerboseOne, GetBlockVerboseOneError> = json.into_model();
    model.expect("GetBlockVerbose into model");

    // TODO: Test getblock 2
    // let json = node.client.get_block_with_verbosity(block_hash, 2).expect("getblock verbosity 2");
    // assert!(json.into_model().is_ok());
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
#[cfg(not(feature = "v17"))]
#[cfg(not(feature = "v18"))]
fn blockchain__get_block_filter__modelled() {
    let node = Node::with_wallet(Wallet::Default, &["-blockfilterindex"]);
    node.mine_a_block();
    let hash = node.client.best_block_hash().expect("best_block_hash failed");

    let json: GetBlockFilter = node.client.get_block_filter(hash).expect("getblockfilter");
    let model: Result<mtype::GetBlockFilter, GetBlockFilterError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_block_hash__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetBlockHash = node.client.get_block_hash(0).expect("getblockhash");
    let model: Result<mtype::GetBlockHash, _> = json.into_model();
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
    let json:GetBlockHeaderVerbose = node.client.get_block_header_verbose(&block_hash).expect("getblockheader");
    let model: Result<mtype::GetBlockHeaderVerbose, GetBlockHeaderVerboseError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_block_stats__modelled() {
    // Version 18 cannot call `getblockstats` if `-txindex` is not enabled.
    #[cfg(not(feature = "v18"))]
    getblockstats();

    // All versions including 18 can `getblockstats` if `-txindex` is enabled.
    getblockstats_txindex();
}

#[cfg(not(feature = "v18"))]
fn getblockstats() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let json = node.client.get_block_stats_by_height(1).expect("getblockstats");
    json.into_model().unwrap();

    // No need for explicit types, used explicitly in test below.
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");
    let json = node.client.get_block_stats_by_block_hash(&block_hash).expect("getblockstats");
    json.into_model().unwrap();
}

fn getblockstats_txindex() {
    let node = Node::with_wallet(Wallet::Default, &["-txindex"]);
    node.fund_wallet();

    // Get block stats by height.
    let json: GetBlockStats = node.client.get_block_stats_by_height(101).expect("getblockstats");
    let model: Result<mtype::GetBlockStats, GetBlockStatsError> = json.into_model();
    model.expect("GetBlockStats into model");

    // Get block stats by block hash.
    let block_hash = node.client.best_block_hash().expect("best_block_hash failed");
    let json = node.client.get_block_stats_by_block_hash(&block_hash).expect("getblockstats");
    json.into_model().unwrap();
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
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetChainTxStats = node.client.get_chain_tx_stats().expect("getchaintxstats");
    let model: Result<mtype::GetChainTxStats, GetChainTxStatsError> = json.into_model();
    model.unwrap();
}

#[test]
fn blockchain__get_difficulty__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let json: GetDifficulty = node.client.get_difficulty().expect("getdifficulty");
    let _: mtype::GetDifficulty = json.into_model();
}

#[test]
#[cfg(feature = "TODO")]
fn blockchain__get_mempool_ancestors__modelled() {
    // We can probably get away with not testing this because it returns the same type as
    // `getmempoolentry` which is tested below (for verbose=true). For verbose=false it
    // just returns a txid.
}

#[test]
#[cfg(feature = "TODO")]
fn blockchain__get_mempool_descendants__modelled() {
    // Same justification as for `blockchain__get_mempool_ancestors__modelled`
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

    // verbose = false
    let json: GetRawMempool = node.client.get_raw_mempool().expect("getrawmempool");
    let model: Result<mtype::GetRawMempool, _> = json.clone().into_model();
    let mempool = model.unwrap();
    // Sanity check.
    assert_eq!(mempool.0.len(), 1);

    // FIXME: Fails: JsonRpc(Json(Error("invalid type: map, expected a sequence", line: 1, column: 0)))
    // verbose = true
    // let json: GetRawMempoolVerbose = node.client.get_raw_mempool_verbose().expect("getrawmempool verbose");
    // let model: Result<mtype::GetRawMempoolVerbose, GetRawMempoolVerboseError> = json.into_model();
    // let mempool = model.unwrap();
    // // Sanity check.
    // assert_eq!(mempool.0.len(), 1);
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
fn blockchain__precious_block() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.mine_a_block();
    let hash = node.client.best_block_hash().expect("best_block_hash failed");
    node.mine_a_block();

    let _ = node.client.precious_block(hash).expect("preciousblock");
}

#[test]
fn blockchain__verify_tx_out_proof__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    verify_tx_out_proof(&node).unwrap();
}

#[test]
fn blockchain__get_tx_out_proof__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    verify_tx_out_proof(&node).unwrap();
}

fn verify_tx_out_proof(node: &Node) -> Result<(), client_sync::Error> {
    let (_address, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    let proof = node.client.get_tx_out_proof(&[txid])?;

    let json: VerifyTxOutProof = node.client.verify_tx_out_proof(&proof)?;
    let model: Result<mtype::VerifyTxOutProof, _> = json.into_model();
    let txids = model.unwrap();

    // sanity check
    assert_eq!(txids.0.len(), 1);

    Ok(())
}

#[test]
fn blockchain__prune_blockchain() {
    const NBLOCKS: usize = 1;

    let node = Node::with_wallet(Wallet::Default, &["-prune=550"]);
    let address = node.client.new_address().expect("Failed to get new address");

    let gen_result = node.client.generate_to_address(NBLOCKS, &address).expect("generate_to_address RPC call failed");
    assert_eq!(gen_result.0.len(), NBLOCKS, "generate_to_address did not return the expected number of block hashes");

    let target_height: u64 = 500;

    let _ = node.client.prune_blockchain(target_height);
}

#[test]
fn blockchain__savemempool() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    node.fund_wallet();
    let (_addr, _txid) = node.create_mempool_transaction();

    // Give node a moment to process mempool tx
    std::thread::sleep(std::time::Duration::from_millis(200));

    let result = node.client.save_mempool();

    // Assert based on version feature flag active during test run
    #[cfg(any(
        feature = "0_17_2",
        feature = "0_18_1",
        feature = "0_19_1",
        feature = "0_20_2",
        feature = "0_21_2",
        feature = "22_1"
    ))] {
        result.expect("savemempool RPC call failed (v17 - v22");
    }

    #[cfg(any(
        feature = "23_2",
        feature = "24_2",
        feature = "25_2",
        feature = "26_2",
        feature = "27_2",
        feature = "28_0"
    ))] {
        let save_result = result.expect("savemempool RPC call failed (v23 - v28)");
        assert!(!save_result.filename.is_empty(), "Filename returned by savemempool should not be empty (v23 - v28)");
    }
}

#[test]
fn blockchain__verify_chain() {
    let node = Node::with_wallet(Wallet::None, &[]);

    // Test with default parameters
    let result_default = node.client.verify_chain_default().expect("veifychain with defaults failed");
    let result_default_value = result_default.0;
    assert!(result_default_value, "verifychain with defaults should return true on a clean chain");

    // Test with specific parameters (e.g., check first 2 blocks thoroughly)
    let checklevel = Some(4u32);
    let nblocks = Some(2u32);
    let result_specific = node.client.verify_chain(checklevel, nblocks).expect("verifychain with specific args failed");
    let result_specific_value = result_specific.0;
    assert!(result_specific_value, "verifychain with specific args should return true on a clean chain");

    // Test with only nblocks (requires null for checklevel)
    let result_nblocks_only = node.client.verify_chain(None, Some(1)).expect("verifychain with nblocks only failed");
    let result_nblocks_only_value = result_nblocks_only.0;
    assert!(result_nblocks_only_value, "verifychain with nblocks only should return true");
}

#[cfg(not(any(feature = "v17", feature = "v18", feature = "v19", feature = "v20", feature = "v21")))]
#[test]
fn blockchain__scantxoutset_modelled() {
    let node = Node::with_wallet(Wallet::None, &["-coinstatsindex=1"]);


    let dummy_pubkey_hex = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    let scan_desc = format!("pkh({})", dummy_pubkey_hex);

    let scan_objects = [client_sync::ScanObject::Descriptor(scan_desc)];
    let action = client_sync::ScanAction::Start;

    let result = node.client.scan_tx_out_set(action, Some(&scan_objects));

    #[cfg(feature = "v17")]
    {
        let _: ScanTxOutSet = result.expect("scantxoutset(Start) failed (v17)");
    }
    #[cfg(feature = "v18")]
    {
        let _: ScanTxOutSet = result.expect("scantxoutset(Start) failed (v18)");
    }
    #[cfg(all(feature = "v19", not(feature = "v22")))]
    {
        let res: ScanTxOutSet = result.expect("scantxoutset(Start) failed (v19-v21)");
        assert!(res.success == true || res.success == false);
    }
     #[cfg(all(feature = "v22", not(feature = "v25")))]
    {
        let res: ScanTxOutSet = result.expect("scantxoutset(Start) failed (v22-v24)");
        match res {
            ScanTxOutSet::Start(start_res) => {
                 assert!(start_res.success == true || start_res.success == false);
            },
            _ => panic!("Expected Start variant result for scantxoutset (v22-v24)"),
        }
    }
     #[cfg(all(feature = "v25", not(feature = "v28")))]
    {
        let res: ScanTxOutSet = result.expect("scantxoutset(Start) failed (v25-v27)");
         match res {
            ScanTxOutSet::Start(start_res) => {
                 assert!(start_res.success == true || start_res.success == false);
             },
            _ => panic!("Expected Start variant result for scantxoutset (v25-v27)"),
        }
    }
    #[cfg(any(feature = "v28"))]
    {
        let res: ScanTxOutSet = result.expect("scantxoutset(Start) failed (v28+)");
        match res {
            ScanTxOutSet::Start(start_res) => {
                 assert!(start_res.success == true || start_res.success == false);
             },
            _ => panic!("Expected Start variant result for scantxoutset (v28+)"),
        }
    }
}
