// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Generating ==` section of the API docs.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

use bitcoin::hex;
use bitcoind::mtype;
use bitcoind::vtype::*;
use integration_test::{BitcoinD, BitcoinDExt as _, Wallet}; // All the version specific types.

#[test]
#[cfg(not(feature = "v20_and_below"))]
fn generating__generate_block__modelled() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let mining_addr = node.client.new_address().expect("failed to get new address");
    let dest_addr = node.client.new_address().expect("failed to get new address");
    let amount = bitcoin::Amount::from_sat(1_000_000);
    let txid = node
        .client
        .send_to_address_rbf(&dest_addr, amount)
        .expect("sendtoaddressrbf")
        .txid()
        .expect("txid");
    let transactions = vec![txid.to_string()];

    let json: GenerateBlock;
    #[cfg(feature = "v24_and_below")]
    {
        // No `submit` argument
        json = node
            .client
            .generate_block(&mining_addr.to_string(), &transactions)
            .expect("generateblock");
        let model: Result<mtype::GenerateBlock, hex::HexToArrayError> = json.into_model();
        model.unwrap();
    }

    #[cfg(not(feature = "v24_and_below"))]
    {
        // Check with `submit = false` so that `hex` is returned. v25 and later only.
        json = node
            .client
            .generate_block(&mining_addr.to_string(), &transactions, false)
            .expect("generateblock");
        let model: Result<mtype::GenerateBlock, GenerateBlockError> = json.into_model();
        model.unwrap();
    }
}

#[test]
#[cfg(feature = "v17")]
fn generating__generate__modelled() {
    const NBLOCKS: usize = 10;
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);

    let json: Generate = node.client.generate(NBLOCKS).expect("generate");

    let model: Result<mtype::Generate, hex::HexToArrayError> = json.into_model();
    model.unwrap();
}

#[test]
fn generating__generate_to_address__modelled() {
    const NBLOCKS: usize = 1;

    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    let address = node.client.new_address().expect("failed to get new address");

    let json: GenerateToAddress =
        node.client.generate_to_address(NBLOCKS, &address).expect("generatetoaddress");

    let model: Result<mtype::GenerateToAddress, hex::HexToArrayError> = json.into_model();
    model.unwrap();
}

#[test]
#[cfg(not(feature = "v19_and_below"))]
fn generating__generate_to_descriptor__modelled() {
    const NBLOCKS: usize = 1;

    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    let address = node.client.new_address().expect("failed to get new address");
    let descriptor = format!("addr({})", address);

    let json: GenerateToDescriptor =
        node.client.generate_to_descriptor(NBLOCKS, &descriptor).expect("generatetodescriptor");
    let model: Result<mtype::GenerateToDescriptor, hex::HexToArrayError> = json.into_model();
    model.unwrap();
}

// This method does not appear in the output of `bitcoin-cli help`.
#[test]
fn generating__invalidate_block() {
    const NBLOCKS: usize = 1;

    let node = BitcoinD::with_wallet(Wallet::Default, &[]);

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

    let json: GetBestBlockHash = node.client.get_best_block_hash().expect("getbestblockhash");
    let model: Result<mtype::GetBestBlockHash, hex::HexToArrayError> = json.into_model();
    let best_block = model.unwrap();

    assert_eq!(old_best_block, best_block.0);
}
