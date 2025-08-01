// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Network ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use integration_test::{Node, NodeExt as _, Wallet};
use node::vtype::*;             // All the version specific types.
use node::{AddNodeCommand, mtype, SetBanCommand};

#[test]
fn network__add_node() {
    let node = match () {
        #[cfg(feature = "v25_and_below")]
        () => Node::with_wallet(Wallet::None, &[]),
        #[cfg(not(feature = "v25_and_below"))]
        () => Node::with_wallet(Wallet::None, &["-v2transport"]),
    };

    let dummy_peer = "192.0.2.1:8333";

    let _: () = node.client.add_node(dummy_peer, AddNodeCommand::OneTry).expect("addnode onetry");
    let _: () = node.client.add_node(dummy_peer, AddNodeCommand::Add).expect("addnode add");
    let _: () = node.client.add_node(dummy_peer, AddNodeCommand::Remove).expect("addnode remove");
}

#[test]
fn network__clear_banned() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let dummy_subnet = "192.0.2.2";

    let _: () = node.client.set_ban(dummy_subnet, SetBanCommand::Add).expect("setban add");
    let _: () = node.client.clear_banned().expect("clearbanned");
}

#[test]
fn network__disconnect_node() {
    let (_node1, node2, _node3) = integration_test::three_node_network();

    let peers = node2.client.get_peer_info().expect("getpeerinfo");
    let peer = peers.0.first().expect("should have at least one peer");

    let _: () = node2.client.disconnect_node(&peer.address).expect("disconnectnode");
}

#[test]
fn network__get_added_node_info() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _: GetAddedNodeInfo = node.client.get_added_node_info().expect("getaddednodeinfo");
}

#[test]
#[cfg(not(feature = "v25_and_below"))]
fn network__get_addr_man_info() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json: GetAddrManInfo = node.client.get_addr_man_info().expect("getaddrmaninfo");
    assert!(!json.0.is_empty());

    for (network_name, network_info) in &json.0 {
        assert!(!network_name.is_empty());
        assert_eq!(network_info.total, network_info.new + network_info.tried);
    };
}

#[test]
fn network__get_connection_count() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _: GetConnectionCount = node.client.get_connection_count().expect("getconnectioncount");
}

#[test]
fn network__get_net_totals() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _: GetNetTotals = node.client.get_net_totals().expect("getnettotals");
}

#[test]
fn network__get_network_info__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json: GetNetworkInfo = node.client.get_network_info().expect("getnetworkinfo");
    let model: Result<mtype::GetNetworkInfo, GetNetworkInfoError> = json.into_model();
    model.unwrap();

    // Server version is returned as part of the getnetworkinfo method.
    node.client.check_expected_server_version().expect("unexpected version");
}

#[test]
// Core version 18 onwards.
#[cfg(not(feature = "v17"))]
fn network__get_node_addresses() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json: GetNodeAddresses = node.client.get_node_addresses().expect("getnodeaddresses");
    let res: Result<mtype::GetNodeAddresses, _> = json.into_model();
    let model = res.expect("GetNodeAddresses into model");
    assert!(model.0.len() <= 2500);
    if let Some(addr) = model.0.first() {
        assert!(addr.port > 0);
        assert!(addr.time > 1231006505);
    }
}

#[test]
fn network__get_peer_info() {
    get_peer_info_one_node_network();
    get_peer_info_three_node_network();
}

fn get_peer_info_one_node_network() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json: GetPeerInfo = node.client.get_peer_info().expect("getpeerinfo");
    assert_eq!(json.0.len(), 0);
}

fn get_peer_info_three_node_network() {
    let (node1, node2, node3) = integration_test::three_node_network();

    // Just for good measure.
    node1.mine_a_block();
    node2.mine_a_block();
    node3.mine_a_block();

    let json: GetPeerInfo = node1.client.get_peer_info().expect("getpeerinfo");
    // This verifies that we re-exported the correct `PeerInfo` type at the module level.
    let _: PeerInfo = json.0[0];

    // FIXME: Fails if we use equal to 2 ???
    assert!(node1.peers_connected() >= 1);
    assert!(node2.peers_connected() >= 1);
    assert!(node3.peers_connected() >= 1);
}

#[test]
fn network__list_banned() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let dummy_subnet = "192.0.2.5/32";

    node.client.set_ban(dummy_subnet, SetBanCommand::Add).expect("setban add");
    let json: ListBanned = node.client.list_banned().expect("listbanned");
    assert!(json.0.iter().any(|item| item.address == dummy_subnet));

    node.client.set_ban(dummy_subnet, SetBanCommand::Remove).expect("setban remove");
    let json: ListBanned = node.client.list_banned().expect("listbanned");
    assert!(json.0.iter().all(|item| item.address != dummy_subnet));
}

#[test]
fn network__ping() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _: () = node.client.ping().expect("ping");
}

#[test]
fn network__set_ban() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let dummy_subnet = "192.0.2.3";

    let _: () = node.client.set_ban(dummy_subnet, SetBanCommand::Add).expect("setban add");
    let _: () = node.client.set_ban(dummy_subnet, SetBanCommand::Remove).expect("setban remove");
}

#[test]
fn network__set_network_active() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json: SetNetworkActive = node.client.set_network_active(false).expect("setnetworkactive false");
    assert!(!json.0);

    let json: SetNetworkActive = node.client.set_network_active(true).expect("setnetworkactive true");
    assert!(json.0);
}
