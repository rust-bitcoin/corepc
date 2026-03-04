// SPDX-License-Identifier: CC0-1.0

//! Tests for the async client.

#![cfg(not(feature = "v24_and_below"))]
#![allow(non_snake_case)] // Test names intentionally use double underscore.

use bitcoin::address::KnownHrp;
use bitcoin::{Address, CompressedPublicKey, PrivateKey};
use corepc_client::bdk_client::{Auth, Client, Error as AsyncClientError};
use integration_test::{Node, NodeExt as _, Wallet};
use node::mtype;

fn async_client_for(node: &Node) -> Client {
    Client::new_with_auth(&node.rpc_url(), auth_for(node)).expect("async client")
}

#[tokio::test]
async fn async__get_best_block_hash__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let model: Result<bitcoin::BlockHash, AsyncClientError> = client.get_best_block_hash().await;
    let model = model.unwrap();
    let expected = node.client.best_block_hash().expect("best_block_hash");
    assert_eq!(model, expected);
}

#[tokio::test]
async fn async__get_block__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let best_hash = node.client.best_block_hash().expect("best_block_hash");

    let model: Result<bitcoin::Block, AsyncClientError> = client.get_block(&best_hash).await;
    let model = model.unwrap();
    assert_eq!(model.block_hash(), best_hash);

    let model: Result<mtype::GetBlockVerboseOne, AsyncClientError> =
        client.get_block_verbose(&best_hash).await;
    let model = model.unwrap();
    assert_eq!(model.hash, best_hash);
}

#[tokio::test]
async fn async__get_block_count__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let model: Result<u64, AsyncClientError> = client.get_block_count().await;
    let model = model.unwrap();
    assert_eq!(model, 0);
}

#[tokio::test]
#[cfg(not(feature = "v18_and_below"))]
async fn async__get_block_filter__modelled() {
    let node = Node::with_wallet(Wallet::None, &["-blockfilterindex"]);
    let client = async_client_for(&node);

    let best_hash = node.client.best_block_hash().expect("best_block_hash");
    let model: Result<mtype::GetBlockFilter, AsyncClientError> =
        client.get_block_filter(&best_hash).await;
    let model = model.unwrap();

    assert!(!model.filter.is_empty());
    assert_eq!(model.header.to_string().len(), 64);
}

#[tokio::test]
async fn async__get_block_hash__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let model: Result<bitcoin::BlockHash, AsyncClientError> = client.get_block_hash(0).await;
    let model = model.unwrap();
    let expected = node.client.best_block_hash().expect("best_block_hash");
    assert_eq!(model, expected);
}

#[tokio::test]
async fn async__get_block_header__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let best_hash = node.client.best_block_hash().expect("best_block_hash");
    let model: Result<bitcoin::block::Header, AsyncClientError> =
        client.get_block_header(&best_hash).await;
    let model = model.unwrap();
    assert_eq!(model.block_hash(), best_hash);

    let model: Result<mtype::GetBlockHeaderVerbose, AsyncClientError> =
        client.get_block_header_verbose(&best_hash).await;
    let model = model.unwrap();
    assert_eq!(model.hash, best_hash);
    assert_eq!(model.height, 0);
}

#[tokio::test]
async fn async__get_raw_mempool__modelled() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let model: Result<Vec<bitcoin::Txid>, AsyncClientError> = client.get_raw_mempool().await;
    let model = model.unwrap();
    assert!(model.is_empty());
}

#[tokio::test]
async fn async__get_raw_transaction__modelled() {
    let node = Node::with_wallet(Wallet::None, &["-txindex"]);
    let privkey =
        PrivateKey::from_wif("cVt4o7BGAig1UXywgGSmARhxMdzP5qvQsxKkSsc1XEkw3tDTQFpy").expect("wif");
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let pubkey = privkey.public_key(&secp);
    let address = Address::p2wpkh(&CompressedPublicKey(pubkey.inner), KnownHrp::Regtest);
    node.client.generate_to_address(1, &address).expect("generatetoaddress");

    let client = async_client_for(&node);
    let best_hash = node.client.best_block_hash().expect("best_block_hash");
    let block = client.get_block(&best_hash).await.expect("getblock");
    let txid = block.txdata[0].compute_txid();

    let model: Result<bitcoin::Transaction, AsyncClientError> =
        client.get_raw_transaction(&txid).await;
    let model = model.unwrap();
    assert_eq!(model.compute_txid(), txid);
}

fn auth_for(node: &Node) -> Auth { Auth::CookieFile(node.params.cookie_file.clone()) }
