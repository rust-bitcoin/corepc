// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Zmq ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.
#![allow(unused_imports)] // Because of feature gated tests.

use bitcoind::vtype::*;
use integration_test::{BitcoinD, BitcoinDExt as _, Wallet}; // All the version specific types.

#[test]
#[cfg(not(feature = "v17"))]
fn zmq__get_zmq_notifications__modelled() {
    // Start node with a ZMQ notification enabled so we have at least one entry.
    // Using hashblock as it is lightweight.
    let node = BitcoinD::with_wallet(Wallet::Default, &["-zmqpubhashblock=tcp://127.0.0.1:29000"]);

    let list: Vec<GetZmqNotifications> =
        node.client.get_zmq_notifications().expect("getzmqnotifications");
    let zmq_notification = &list[0];
    assert_eq!(zmq_notification.type_, "pubhashblock");
    assert_eq!(zmq_notification.address, "tcp://127.0.0.1:29000");
}
