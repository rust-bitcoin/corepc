// SPDX-License-Identifier: CC0-1.0

//! Tests for methods that are `== Hidden ==` and not in the API docs of Bitcoin Core.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use integration_test::{Node, NodeExt as _, Wallet};
use node::mtype;
use node::vtype::*; // All the version specific types.

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
