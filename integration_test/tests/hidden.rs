// SPDX-License-Identifier: CC0-1.0

//! Tests for methods that are `== Hidden ==` and not in the API docs of Bitcoin Core.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use integration_test::{Node, NodeExt as _, Wallet};
use node::mtype;
use node::vtype::*; // All the version specific types.
#[cfg(not(feature = "v21_and_below"))]
use node::P2P;

#[test]
#[cfg(not(feature = "v21_and_below"))]
fn hidden__add_connection() {
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

    assert_eq!(json.address, address);
    assert_eq!(json.connection_type, "outbound-full-relay");
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
#[cfg(not(feature = "v25_and_below"))]
fn hidden__get_raw_addrman() {
    let node = Node::with_wallet(Wallet::None, &[]);

    // Add a peer address so the address manager has something.
    let peer_address = "1.2.3.4";
    let peer_port = 8333;
    node.client.add_peer_address(peer_address, peer_port).expect("addpeeraddress");

    let json: GetRawAddrman = node.client.get_raw_addrman().expect("getrawaddrman");

    let entry = json
        .new
        .values()
        .find(|e| e.address == peer_address && e.port == peer_port)
        .expect("added peer should appear in the 'new' table");

    assert_eq!(entry.network, "ipv4");

    // mapped_as field added in v28, only present with -asmap config.
    assert!(entry.mapped_as.is_none(), "mapped_as requires -asmap config");
}
