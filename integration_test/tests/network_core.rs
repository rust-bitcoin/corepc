// SPDX-License-Identifier: CC0-1.0

//! Tests derived from Bitcoin Core's rpc_net.py and rpc_setban.py

use bitcoind::vtype::*;
use bitcoind::AddNodeCommand;
#[cfg(not(feature = "v21_and_below"))]
use bitcoind::SetBanCommand;
use integration_test::{BitcoinD, BitcoinDExt as _, Wallet};

#[test]
#[cfg(not(feature = "v20_and_below"))]
fn get_peer_info_has_connection_type() {
    let (node1, _node2, _node3) = integration_test::three_node_network();

    let json: GetPeerInfo = node1.client.get_peer_info().unwrap();
    let peer = json.0.first().unwrap();

    assert!(peer.connection_type.is_some());
}

#[test]
fn get_net_totals_time_millis_is_non_zero() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);

    let json: GetNetTotals = node.client.get_net_totals().unwrap();

    assert!(json.time_millis > 0);
}

#[test]
#[cfg(not(feature = "v20_and_below"))]
fn get_peer_info_bytes_per_msg_non_empty() {
    let (node1, _node2, _node3) = integration_test::three_node_network();

    let json: GetPeerInfo = node1.client.get_peer_info().unwrap();
    let peer = json.0.first().unwrap();

    assert!(!peer.bytes_sent_per_message.is_empty());
    assert!(!peer.bytes_received_per_message.is_empty());
}

#[test]
#[cfg(not(feature = "v22_and_below"))]
fn get_peer_info_synced_blocks_le_headers() {
    let (node1, _node2, _node3) = integration_test::three_node_network();

    let json: GetPeerInfo = node1.client.get_peer_info().unwrap();
    let peer = json.0.first().unwrap();

    #[cfg(feature = "v30_and_below")]
    assert!(peer.starting_height.is_some());
    #[cfg(not(feature = "v30_and_below"))]
    assert!(peer.starting_height.is_none());

    let headers = peer.synced_headers.unwrap();
    let blocks = peer.synced_blocks.unwrap();
    assert!(blocks <= headers);
}

#[test]
#[cfg(not(feature = "v18_and_below"))]
fn get_network_info_local_services_names_non_empty() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);

    let json: GetNetworkInfo = node.client.get_network_info().unwrap();

    assert!(!json.local_services_names.is_empty());
}

#[test]
#[cfg(not(feature = "v20_and_below"))]
fn get_network_info_connections_sum_matches_total() {
    let (node1, _node2, _node3) = integration_test::three_node_network();

    let json: GetNetworkInfo = node1.client.get_network_info().unwrap();

    assert_eq!(json.connections_in + json.connections_out, json.connections);
    assert!(json.connections > 0);
}

#[test]
#[cfg(not(feature = "v21_and_below"))]
fn list_banned_duration_matches_until_minus_created() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let target = "192.0.2.88/32";
    node.client.set_ban(target, SetBanCommand::Add).unwrap();

    let json: ListBanned = node.client.list_banned().unwrap();
    let entry = json.0.iter().find(|e| e.address == target).unwrap();

    assert_eq!(entry.banned_until, entry.ban_created + entry.ban_duration);

    node.client.set_ban(target, SetBanCommand::Remove).unwrap();
}

#[test]
fn get_added_node_info_round_trips_added_node() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);

    let target = "192.0.2.99:18444";
    node.client.add_node(target, AddNodeCommand::Add).unwrap();

    let json: GetAddedNodeInfo = node.client.get_added_node_info().unwrap();
    let entry = json.0.iter().find(|n| n.added_node == target).unwrap();
    assert_eq!(entry.added_node, target);
}
