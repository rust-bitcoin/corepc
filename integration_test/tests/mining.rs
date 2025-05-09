// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Mining ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use bitcoin::SignedAmount;
use integration_test::{Node, NodeExt as _, Wallet};
use node::{TemplateRequest, TemplateRules};
use node::vtype::*;             // All the version specific types.
use node::mtype;

#[test]
fn mining__get_block_template__modelled() {
    // Requires connected nodes otherwise the RPC call errors.
    let (node1, node2, node3) = integration_test::three_node_network();

    // Use the nodes otherwise they get dropped.
    node1.mine_a_block();
    node2.mine_a_block();
    node3.mine_a_block();

    let options = TemplateRequest { rules: vec![TemplateRules::Segwit] };

    let json: GetBlockTemplate = node1.client.get_block_template(&options).expect("rpc");
    let model: Result<mtype::GetBlockTemplate, GetBlockTemplateError> = json.into_model();
    model.unwrap();
}

#[test]
fn mining__get_mining_info() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let _: GetMiningInfo = node.client.get_mining_info().expect("rpc");
}

#[test]
fn mining__get_network_hash_ps() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let _ = node.client.get_network_hash_ps().expect("rpc");
}

#[test]
// Core version 26 onwards.
#[cfg(all(
    not(feature = "v17"),
    not(feature = "v18"),
    not(feature = "v19"),
    not(feature = "v20"),
    not(feature = "v21"),
    not(feature = "v22"),
    not(feature = "v23"),
    not(feature = "v24"),
    not(feature = "v25"),
))]
fn mining__get_prioritised_transactions() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let _ = node.client.get_prioritised_transactions().expect("getprioritisedtransactions");
}

#[test]
fn mining__prioritise_transaction() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let (_addr, txid) = node.create_mempool_transaction();
    let fee_delta = SignedAmount::from_sat(10_000);
    let res = node.client.prioritise_transaction(&txid, fee_delta).expect("prioritisetransaction");
    assert!(res) // According to docs always returns true.
}

#[test]
#[cfg(feature = "TODO")]        // This test is flaky - no clue why.
fn mining__submit_block() {
    // Requires connected nodes otherwise the RPC call errors.
    let (node1, node2, node3) = integration_test::three_node_network();

    // Use the nodes otherwise they get dropped.
    node1.mine_a_block();
    node2.mine_a_block();
    node3.mine_a_block();

    let options = TemplateRequest { rules: vec![TemplateRules::Segwit] };
    let json = node1.client.get_block_template(&options).expect("getblocktemplate");
    let template = json.into_model().expect("GetBlockTemplate into model");

    submit_empty_block(&node1, &template);
    // submit_block_with_dummy_coinbase(&node1, &template);
}

// Code copied from BDK - thanks!
// FIXME: Submitting this block sometimes works and sometimes returns 'inconclusive'.
#[allow(dead_code)]
fn submit_empty_block(node: &Node, bt: &mtype::GetBlockTemplate) {
    use bitcoin::hashes::Hash as _;
    use bitcoin::{
        absolute, block, transaction, Amount, Block, OutPoint, ScriptBuf, Sequence,
        Transaction, TxIn, TxOut, Witness, ScriptHash, TxMerkleNode,
    };

    let txdata = vec![Transaction {
        version: transaction::Version::ONE,
        lock_time: absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint::default(),
            // FIXME: (Tobin) I don't know if this script is meaningful in anyway other than enabling function reusability in the code copied from BDK?
            script_sig: ScriptBuf::builder()
                .push_int(bt.height as _)
                .push_int(rand::random()) // random number so that re-mining creates unique block
                .into_script(),
            sequence: Sequence::default(),
            witness: Witness::new(),
        }],
        output: vec![TxOut {
            value: Amount::ZERO,
            script_pubkey: ScriptBuf::new_p2sh(&ScriptHash::all_zeros()),
        }],
    }];

    let mut block = Block {
        header: block::Header {
            version: block::Version::default(),
            prev_blockhash: bt.previous_block_hash,
            merkle_root: TxMerkleNode::all_zeros(),
            time: Ord::max(bt.min_time, std::time::UNIX_EPOCH.elapsed().expect("elapsed").as_secs() as u32) as u32,
            bits: bt.bits,
            nonce: 0,
        },
        txdata,
    };

    block.header.merkle_root = block.compute_merkle_root().expect("must compute");

    for nonce in 0..=u32::MAX {
        block.header.nonce = nonce;
        if block.header.target().is_met_by(block.block_hash()) {
            break;
        }
    }

    let _ = node.client.submit_block(&block).expect("submitblock");
}

// FIXME: Submitting this block returns 'inconclusive'.
#[allow(dead_code)]
fn mining__submit_block_with_dummy_coinbase(node: &Node, bt: &mtype::GetBlockTemplate) {
    use bitcoin::hashes::Hash as _;
    use bitcoin::{
        absolute, block, transaction, Amount, Block, OutPoint, ScriptBuf, Sequence,
        Transaction, TxIn, TxOut, Witness, TxMerkleNode,
    };

    let address = node.client.new_address().expect("failed to get new address");

    let coinbase = Transaction {
        version: transaction::Version::ONE,
        lock_time: absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint::null(),
            // FIXME: (Tobin) I don't know what this script means. Core return block invalid without it?
            script_sig: ScriptBuf::builder()
                .push_int(bt.height.into())
                .push_int(rand::random()) // random number so that re-mining creates unique block
                .into_script(),
            sequence: Sequence::default(),
            witness: Witness::new(),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(50 * 100_000_000),
            script_pubkey: address.script_pubkey(),
        }],
    };

    let mut block = Block {
        header: block::Header {
            version: bt.version,
            prev_blockhash: bt.previous_block_hash,
            merkle_root: TxMerkleNode::all_zeros(),
            time: bt.min_time + 3600, // Some arbitrary amount of time.
            bits: bt.bits,
            nonce: 0,
        },
        txdata: vec![coinbase],
    };

    let mut nonces = bt.nonce_range.split(",");
    let nonce_start = match nonces.next() {
        Some(s) => u32::from_str_radix(s, 16).expect("valid nonce"),
        None => 0,
    };

    for nonce in nonce_start..=u32::MAX {
        block.header.nonce = nonce;
        if block.header.target().is_met_by(block.block_hash()) {
            break;
        }
    }

    let _ = node.client.submit_block(&block).expect("submitblock");
}

// TODO: submitheader "hexdata" (v18 onwards)
