// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Network ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use integration_test::{AddNodeCommand, SetBanCommand};
use integration_test::{Node, NodeExt as _, Wallet};
use node::vtype::*;             // All the version specific types.
use node::mtype;

#[test]
fn network__get_added_node_info() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _: GetAddedNodeInfo = node.client.get_added_node_info().expect("getaddednodeinfo");
}

#[test]
fn network__get_net_totals() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let _: GetNetTotals = node.client.get_net_totals().expect("getnettotals");
}

#[test]
fn network__get_network_info() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let json: GetNetworkInfo = node.client.get_network_info().expect("getnetworkinfo");
    let model: Result<mtype::GetNetworkInfo, GetNetworkInfoError> = json.into_model();
    model.unwrap();

    // Server version is returned as part of the getnetworkinfo method.
    node.client.check_expected_server_version().expect("unexpected version");
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

    // FIXME: Fails if we use equal to 2 ???
    assert!(node1.peers_connected() >= 1);
    assert!(node2.peers_connected() >= 1);
    assert!(node3.peers_connected() >= 1);
}

#[test]
fn network__add_node() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let dummy_peer = "192.0.2.1:8333";

    #[cfg(any(
        feature = "v17",
        feature = "v18",
        feature = "v19",
        feature = "v20",
        feature = "v21",
        feature = "v22",
        feature = "v23",
        feature = "v24",
        feature = "v25"
    ))] {
        node.client.add_node(dummy_peer, AddNodeCommand::OneTry).expect("addnode onetry failed (v17-v25)");

        node.client.add_node(dummy_peer, AddNodeCommand::Add).expect("addnode add failed (v17-v25");

        node.client.add_node(dummy_peer, AddNodeCommand::Remove).expect("addnode remove failed (v17-v25");
    }

    #[cfg(any(
        feature = "v26",
        feature = "v27",
        feature = "v28"
    ))] {
        node.client.add_node(dummy_peer, AddNodeCommand::OneTry, None).expect("addnode onetry failed (v26+, v2transport=None)");

        node.client.add_node(dummy_peer, AddNodeCommand::Add, Some(false)).expect("addone add failed (v26+, v2transport=false)");

        node.client.add_node(dummy_peer, AddNodeCommand::Remove, Some(true)).expect("addnode remove failed (v26+, v2transport=true)");
    }
}

#[test]
fn network__clear_banned() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let dummy_ip = "192.0.2.2";

    node.client.set_ban(dummy_ip, SetBanCommand::Add, Some(60), None).expect(&format!("set_ban failed for setup for IP: {}", dummy_ip));

    node.client.clear_banned().expect("clearbanned RPC call failed");

    let banned_list = node.client.list_banned().expect("list_banned failed during verification after clear");
    assert!(banned_list.0.is_empty(), "Banned list should be empty after clearbanned");
}

#[test]
fn network__set_ban() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let subnet1 = "192.0.2.3";
    let subnet2 = "192.0.2.0/24";

    // Test Case 1: Add subnet1 with default bantime
    node.client.set_ban(subnet1, SetBanCommand::Add, None, None).expect("setban add default time failed");

    // Test Case 2: Add subnet2 with specific duration (e.g., 10 minutes)
    node.client.set_ban(subnet2, SetBanCommand::Add, Some(600), None).expect("setban add specific duration failed");

    // Test Case 3: Add subnet1 to use absolute time
    node.client.set_ban(subnet1, SetBanCommand::Remove, None, None).expect("setban remove failed for subnet1");
    node.client.set_ban(subnet2, SetBanCommand::Remove, None, None).expect("setban remove failed for subnet2");

    // Verify removal using list_banned
    let list_after_removals = node.client.list_banned().expect("list_banned after both removes failed");
    assert!(list_after_removals.0.is_empty(), "Ban list should be empty after removing both");

    // Re-add subnet1 with absolute time
    let future_timestamp = (std::time::SystemTime::now() + std::time::Duration::from_secs(3600))
        .duration_since(std::time::UNIX_EPOCH).expect("Time went backwards").as_secs();
    node.client.set_ban(subnet1, SetBanCommand::Add, Some(future_timestamp as i64), Some(true)).expect("setban re-add absolute time failed after removing both");

    // Test Case 4: Remove the absolute ban for subnet1
    node.client.set_ban(subnet1, SetBanCommand::Remove, None, None).expect("setban remove failed for subnet1 (after absolute ban)");

    // Test Case 6: Attempt to remove subnet2 again (it was already removed)
    // This should fail as it's not currently banned.
    let remove_subnet2_again_result = node.client.set_ban(subnet2, SetBanCommand::Remove, None, None);
    assert!(
        remove_subnet2_again_result.is_err(),
        "Removing subnet2 again should fail (it's not banned)"
    );

    if let Err(e) = remove_subnet2_again_result {
         println!("  Verified removing {} again fails as expected: {:?}", subnet2, e);
    }

    // Verify final state is empty
    let final_list = node.client.list_banned().expect("Final list_banned call failed");
    assert!(final_list.0.is_empty(), "Final ban list should be empty");
}

#[test]
fn network__list_banned() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let subnet1 = "192.0.2.6";

    let ban_duration_secs = 300i64;

    let initial_list = node.client.list_banned().expect("Initial listbanned call failed");
    assert!(initial_list.0.is_empty(), "Initial banned list should be empty");

    node.client.set_ban(subnet1, SetBanCommand::Add, Some(ban_duration_secs), None).expect("set_ban call failed during setup");

    let banned_list = node.client.list_banned().expect("Second listbanned call failed");

    assert!(!banned_list.0.is_empty(), "Banned list should not be empty after setban");

    let entry = banned_list.0.iter().find(|entry| entry.address == subnet1 || entry.address.starts_with(&format!("{}/", subnet1))).expect(&format!("Did not find banned subnet {} or {}/32 in list", subnet1, subnet1));

    assert!(entry.ban_created.is_some(), "ban_created should be Some for v17+");
    assert!(entry.banned_until.is_some(), "banned_until should be Some for v17+");

    #[cfg(any(
        feature = "v17",
        feature = "v18",
        feature = "v19",
        feature = "v20",
    ))] {
        assert!(entry.ban_reason.is_some(), "ban_reason expected to be Some for v17-v20");
    }

    #[cfg(not(any(
        feature = "v17",
        feature = "v18",
        feature = "v19",
        feature = "v20",
    )))] {
        assert!(entry.ban_reason.is_none(), "ban_reason expected to be None for v21+");
    }

    #[cfg(any(
        feature = "v22",
        feature = "v23",
        feature = "v24",
        feature = "v25",
        feature = "v26",
        feature = "v27",
        feature = "v28",
    ))] {
        assert!(entry.ban_duration.is_some(), "ban_duration expected to be Some for v22+");
        assert!(entry.time_remaining.is_some(), "time_remaining expected to be Some for v22+");
        assert_eq!(entry.ban_duration.unwrap(), ban_duration_secs, "Ban duration mismatch");
        assert!(entry.time_remaining.unwrap() <= ban_duration_secs && entry.time_remaining.unwrap() >= 0, "Time remaining out of range");
    }

    #[cfg(not(any(
        feature = "v22",
        feature = "v23",
        feature = "v24",
        feature = "v25",
        feature = "v26",
        feature = "v27",
        feature = "v28",
    )))] {
        assert!(entry.ban_duration.is_none(), "ban_duration expected to be None for v17-v21");
        assert!(entry.time_remaining.is_none(), "time_remaining expected to be None for v17-v21");
    }

    node.client.clear_banned().expect("clear_banned call failed during cleanup");
    let final_list = node.client.list_banned().expect("Final listbanned call failed");
    assert!(final_list.0.is_empty(), "Banned list should be empty after clearbanned");
}

#[test]
fn network__disconnect_node_error_cases() {
    let node = Node::with_wallet(Wallet::None, &[]);

    // Test providing both - expect specific error
    let result_both = node.client.disconnect_node(Some("127.0.0.1:1"), Some(1));
    assert!(result_both.is_err(), "Expected disconnect_node to return Err when both args are provided");

    // Test providing none - expect specific error
    let result_none = node.client.disconnect_node(None, None);
    assert!(result_none.is_err(), "Expected disconnect_node to return Err when no args are provided");
}

#[test]
fn network__disconnect_node_success_cases() {
    // Setup 3 connected nodes
    let (node1, node2, _node3) = integration_test::three_node_network();
    println!("Node network setup complete.");

    // Get Peer Info
    let node1_peers = node1.client.get_peer_info().expect("Node1 getpeerinfo failed");

    assert_eq!(node1_peers.0.len(), 1, "Node1 should have exactly one peer (Node2)");
    let node2_info = &node1_peers.0[0]; // Get the first (only) peer info
    let node2_id = node2_info.id;

    let node2_addr_str = &node2_info.address;

    let node2_peers = node2.client.get_peer_info().expect("Node2 getpeerinfo failed");

    let node3_info = node2_peers.0.iter()
         .find(|p| p.id != node1.client.get_network_info().expect("getnetworkinfo").local_addresses.get(0).map_or(0, |a| a.port as u32))
         .or_else(|| node2_peers.0.get(1))
         .expect("Node2 should see Node3");

    let node3_id = node3_info.id;

    // Test Disconnect by Address
    node1.client.disconnect_node(Some(node2_addr_str), None) // Pass &String here
        .expect("disconnect_node by address failed");

    // Verify disconnection
    std::thread::sleep(std::time::Duration::from_millis(500));
    let node1_peers_after_addr_disconnect = node1.client.get_peer_info()
        .expect("Node1 getpeerinfo after addr disconnect failed");
    assert!(
        node1_peers_after_addr_disconnect.0.iter().find(|p| p.id == node2_id).is_none(),
        "Node2 should be disconnected from Node1 after disconnect by address"
    );

    // Test Disconnect by Node ID
    node2.client.disconnect_node(None, Some(node3_id.into()))
         .expect("disconnect_node by nodeid failed");

    // Verify disconnection
    std::thread::sleep(std::time::Duration::from_millis(500));
    let node2_peers_after_id_disconnect = node2.client.get_peer_info()
        .expect("Node2 getpeerinfo after id disconnect failed");
    assert!(
        node2_peers_after_id_disconnect.0.iter().find(|p| p.id == node3_id).is_none(),
        "Node3 should be disconnected from Node2 after disconnect by nodeid"
    );
}

#[test]
fn network__get_connection_count() {
    let node_single = Node::with_wallet(Wallet::None, &[]);
    let count_single = node_single.client.get_connection_count().expect("getconnectioncount failed for single node");
    let count_single_value = count_single.0;

    assert_eq!(count_single_value, 0, "Single node should have 0 connections");

    let (node1, node2, node3) = integration_test::three_node_network();

    // Allow time for connection for establish fully
    std::thread::sleep(std::time::Duration::from_millis(500));

    let count1 = node1.client.get_connection_count().expect("getconnectioncount failed for node1");
    let count1_value = count1.0;
    assert!(count1_value >= 1, "Node1 should have at least 1 connection");

    let count2 = node2.client.get_connection_count().expect("getconnectioncount failed for node2");
    let count2_value = count2.0;
    assert!(count2_value >= 1, "Node2 should have at least 1 connection");

    let count3 = node3.client.get_connection_count().expect("getconnectioncount failed for node3");
    let count3_value = count3.0;
    assert!(count3_value >= 1, "Node3 should have at least 1 connection");
}

#[test]
fn network__ping() {
    let node_single = Node::with_wallet(Wallet::None, &[]);
    node_single.client.ping().expect("ping failed for single node");

    let (node1, node2, _node3) = integration_test::three_node_network();

    // Allow time for connections to establish fully
    std::thread::sleep(std::time::Duration::from_millis(500));

    node1.client.ping().expect("ping failed for node1 (3-node)");
    node2.client.ping().expect("ping failed for node2 (3-node)");
}

#[test]
fn network__set_network_active() {
    let (node1, _node2, _node3) = integration_test::three_node_network();

    // Allow time for initial connections
    std::thread::sleep(std::time::Duration::from_millis(1000));

    // Call set_network_active(false)
    let result_disable = node1.client.set_network_active(false);

    #[cfg(any(
        feature = "v17",
        feature = "v18",
        feature = "v19"
    ))] {
        // For v17-v19, expect Ok(SetNetworkActiveResult {}) (empty struct)
        result_disable.expect("setnetworkactive(false) failed (v17-v19)");
    }

    #[cfg(not(any(
        feature = "v17",
        feature = "v18",
        feature = "v19",
    )))] {
        // For v20+, expect Ok(SetNetworkActiveResult(bool))
        let result_wrapper = result_disable.expect("setnetworkactive(false) failed (v20+)");
        assert!(!result_wrapper.0, "setnetworkactive(false) should return false (v20+)");
    }

    // Wait and verify network is inactive
    std::thread::sleep(std::time::Duration::from_millis(1000));
    let info_after_disable = node1.client.get_network_info().expect("getnetworkinfo after disable failed");
    assert!(!info_after_disable.network_active, "Network should be inactive after setnetworkactive(false)");

    // Call set_network_active(true)
    let result_enable = node1.client.set_network_active(true);

    #[cfg(any(
        feature = "v17",
        feature = "v18",
        feature = "v19",
    ))] {
         // For v17-v19, expect Ok(SetNetworkActiveResult {}) (empty struct)
        result_enable.expect("setnetworkactive(true) failed (v17-v19)");
    }

    #[cfg(not(any(
        feature = "v17",
        feature = "v18",
        feature = "v19", // This block runs for v20+
    )))] {
        // For v20+, expect Ok(SetNetworkActiveResult(bool))
        let result_wrapper = result_enable.expect("setnetworkactive(true) failed (v20+)");
        // Access the inner bool using .0
        assert!(result_wrapper.0, "setnetworkactive(true) should return true (v20+). Got: {}", result_wrapper.0);
    }
}
