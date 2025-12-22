// SPDX-License-Identifier: CC0-1.0

//! Tests for methods that are `== Hidden ==` and not in the API docs of Bitcoin Core.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

#[cfg(not(feature = "v28_and_below"))]
use std::collections::HashMap;

#[cfg(not(feature = "v28_and_below"))]
use bitcoin::{
    absolute, transaction, consensus, Amount, OutPoint, ScriptBuf, Sequence, Transaction,
    TxIn, TxOut, Txid, Witness,
};

#[cfg(not(feature = "v28_and_below"))]
use bitcoin::hex::DisplayHex;
#[cfg(not(feature = "v28_and_below"))]
use bitcoin::hashes::Hash;

use integration_test::{Node, NodeExt as _, Wallet};
use node::mtype;
use node::vtype::*; // All the version specific types.
#[cfg(not(feature = "v21_and_below"))]
use node::P2P;

#[test]
#[cfg(not(feature = "v21_and_below"))]
fn hidden__add_connection__modelled() {
    let (listener, dialer, _node3) = integration_test::three_node_network();

    let p2p = listener.p2p_connect(false).expect("p2p address");
    let address = match p2p {
        P2P::Connect(socket, _) => socket.to_string(),
        _ => unreachable!("p2p_connect should return P2P::Connect"),
    };

    let json: AddConnection = {
        #[cfg(feature = "v26_and_below")]
        {
            dialer.client.add_connection(&address, "outbound-full-relay").expect("addconnection")
        }
        #[cfg(not(feature = "v26_and_below"))]
        {
            dialer
                .client
                .add_connection(&address, "outbound-full-relay", false)
                .expect("addconnection")
        }
    };

    let model: mtype::AddConnection = json.into_model();

    assert_eq!(model.address, address);
    assert_eq!(model.connection_type, "outbound-full-relay");
    assert!(dialer.peers_connected() >= 1);
}

#[test]
fn hidden__estimate_raw_fee__modelled() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    node.fund_wallet();

    // Give the fee estimator some confirmation history.
    for _ in 0..10 {
        node.create_mined_transaction();
    }

    let json: EstimateRawFee = node.client.estimate_raw_fee(2).expect("estimaterawfee");
    let json_range: &RawFeeRange = json.long.fail.as_ref().unwrap();

    assert!(json_range.total_confirmed > 0.0);

    let model: Result<mtype::EstimateRawFee, EstimateRawFeeError> = json.into_model();
    let estimate = model.unwrap();

    assert!(estimate.long.scale > 0);
}

#[test]
#[cfg(not(feature = "v28_and_below"))]
fn hidden__get_orphan_txs__modelled() {
    // We use node1 to send node2 orphan transactions via a P2P `tx` message.
    let (node1, node2, _node3) = integration_test::three_node_network();

    // Generate a couple of orphan transactions by spending from non-existing UTXOs.
    const NUM_ORPHANS: u8 = 3;
    let address = node1.client.new_address().expect("failed to get new address");
    let orphans: Vec<Transaction> = (0..NUM_ORPHANS).map(|i| {
        Transaction {
            version: transaction::Version::ONE,
            lock_time: absolute::LockTime::ZERO,
            input: vec![TxIn {
                previous_output: OutPoint {
                    txid: Txid::from_raw_hash(Txid::from_byte_array([i; 32]).into()),
                    vout: 0,
                },
                script_sig: ScriptBuf::new(),
                sequence: Sequence::MAX,
                witness: Witness::new(),
            }],
            output: vec![TxOut {
                value: Amount::from_sat(100_000),
                script_pubkey: address.script_pubkey(),
            }],
        }
    }).collect();

    // The receiving node needs to be out of IBD to start accepting transactions.
    node2.mine_a_block();

    // node2 is peer=0 of node1
    const PEER_ID: u64 = 0;
    for orphan in orphans.iter() {
        let tx_bytes = consensus::encode::serialize(orphan);
        let tx_hex: String = tx_bytes.as_hex().to_string();
        // HACK: We should use sendmsgtopeer directly but it's not implemented yet.
        node1.client
            .call::<HashMap<String, String>>(
                "sendmsgtopeer",
                &[PEER_ID.into(), "tx".into(), tx_hex.into()],
            ).unwrap();
    }

    let json_v0: GetOrphanTxs = node2.client.get_orphan_txs().expect("getorphantxs");
    let json_v1: GetOrphanTxsVerboseOne = node2.client.get_orphan_txs_verbosity_1().expect("getorphantxs 1");
    let json_v2: GetOrphanTxsVerboseTwo = node2.client.get_orphan_txs_verbosity_2().expect("getorphantxs 2");

    let model_v0: mtype::GetOrphanTxs = json_v0.into_model();
    let model_v1: mtype::GetOrphanTxsVerboseOne = json_v1.into_model().unwrap();
    let model_v2: mtype::GetOrphanTxsVerboseTwo = json_v2.into_model().unwrap();


    assert_eq!(model_v0.0.len(), NUM_ORPHANS as usize);
    assert_eq!(model_v1.0.len(), NUM_ORPHANS as usize);
    assert_eq!(model_v2.0.len(), NUM_ORPHANS as usize);

    for orphan in orphans.iter() {
        assert!(model_v0.0.contains(&orphan.compute_txid()));

        match model_v1.0
            .iter()
            .filter(|e| e.txid == orphan.compute_txid())
            .next_back() {
            Some(e) => {
                assert_eq!(e.wtxid, orphan.compute_wtxid());
                assert_eq!(e.bytes as usize, orphan.total_size());
                assert_eq!(e.vsize as usize, orphan.vsize());
                assert_eq!(e.weight, orphan.weight().to_wu());
                // node2 received all orphans from node1, which is node2's peer=0
                assert_eq!(e.from, vec![0]);
            },
            None => {
                panic!("Orphan with txid={} not found in `getorphantxs 1` response", orphan.compute_txid());
            }
        }

        match model_v2.0
            .iter()
            .filter(|e| e.txid == orphan.compute_txid())
            .next_back() {
            Some(e) => {
                assert_eq!(e.wtxid, orphan.compute_wtxid());
                assert_eq!(e.bytes as usize, orphan.total_size());
                assert_eq!(e.vsize as usize, orphan.vsize());
                assert_eq!(e.weight, orphan.weight().to_wu());
                // node2 received all orphans from node1, which is node2's peer=0
                assert_eq!(e.from, vec![0]);
                assert_eq!(e.transaction, *orphan);
            },
            None => {
                panic!("Orphan with txid={} not found in `getorphantxs 2` response", orphan.compute_txid());
            }
        }
    }
}
