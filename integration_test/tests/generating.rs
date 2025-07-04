// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Generating ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use integration_test::{Node, NodeExt as _, Wallet};
use node::vtype::*;             // All the version specific types.
use node::mtype;

#[test]
// The `generate` method was deprecated in Core v0.18 and was removed in v0.19.
#[cfg(feature = "v17")]
fn generating__generate__modelled() {
    const NBLOCKS: usize = 10;
    let node = Node::with_wallet(Wallet::Default, &[]);

    let json: Generate = node.client.generate(NBLOCKS).expect("generate");

    let model: Result<mtype::Generate, _> = json.into_model();
    model.unwrap();
}

#[test]
fn generating__generate_to_address__modelled() {
    const NBLOCKS: usize = 1;

    let node = Node::with_wallet(Wallet::Default, &[]);
    let address = node.client.new_address().expect("failed to get new address");

    let json: GenerateToAddress = node.client.generate_to_address(NBLOCKS, &address).expect("generatetoaddress");

    let model: Result<mtype::GenerateToAddress, _>  = json.into_model();
    let _ = model.unwrap();
}

#[cfg(not(feature = "v19_and_below"))]
#[test]
fn generating__generate_to_descriptor__modelled() {
    const NBLOCKS: usize = 1;

    let node = Node::with_wallet(Wallet::Default, &[]);
    let address = node.client.new_address().expect("failed to get new address");
    let descriptor = format!("addr({})", address);

    let json: GenerateToDescriptor = node.client.generate_to_descriptor(NBLOCKS, &descriptor).expect("generatetodescriptor");
    let model: Result<mtype::GenerateToDescriptor, _> = json.into_model();
    let _ = model.unwrap();
}

// This method does not appear in the output of `bitcoin-cli help`.
#[test]
fn generating__invalidate_block() {
    const NBLOCKS: usize = 1;

    let node = Node::with_wallet(Wallet::Default, &[]);

    let address = node.client.new_address().expect("failed to get new address");
    let old_best_block =
        node.client.get_best_block_hash().expect("getbestblockhash").into_model().unwrap().0;
    node.client
        .generate_to_address(NBLOCKS, &address)
        .expect("generatetoaddress")
        .into_model()
        .unwrap();

    let new_best_block =
        node.client.get_best_block_hash().expect("getbestblockhash").into_model().unwrap().0;
    assert_ne!(old_best_block, new_best_block);

    node.client.invalidate_block(new_best_block).expect("invalidateblock");
    let best_block =
        node.client.get_best_block_hash().expect("getbestblockhash").into_model().unwrap().0;
    assert_eq!(old_best_block, best_block);
}
