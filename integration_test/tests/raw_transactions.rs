// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Rawtransactions ==` section of the API docs.

use integration_test::{Node, NodeExt as _};

#[test]
#[cfg(feature = "TODO")]
fn send_raw_transaction() {
    let _node = Node::new_no_wallet();
    todo!()
}

#[test]
#[cfg(feature = "v28")]
fn submitpackage() {
    let node = Node::new_with_default_wallet();

    // Submitting the empty package should simply fail.
    assert!(node.client.submit_package(&[], None, None).is_err());

    node.fund_wallet();

    let (_, tx_0) = node.create_mined_transaction();
    let (_, tx_1) = node.create_mined_transaction();

    // The call for submitting this package should succeed, but yield an 'already known'
    // error for all transactions.
    let res = node
        .client
        .submit_package(&[tx_0, tx_1], None, None)
        .expect("failed to submit package")
        .into_model()
        .expect("failed to submit package");
    for (_, tx_result) in &res.tx_results {
        assert!(tx_result.error.is_some());
    }
    assert!(res.replaced_transactions.is_empty());
}
