// SPDX-License-Identifier: CC0-1.0

//! Tests for the async client.

#![cfg(feature = "v30_and_below")]
#![cfg(not(feature = "v24_and_below"))]
#![allow(non_snake_case)] // Test names intentionally use double underscore.

use bitcoin::address::KnownHrp;
use bitcoin::{Address, CompressedPublicKey, PrivateKey};
use bitcoind::mtype;
use corepc_client::client_async::{Auth, BitcoinRpcs as _, Client, Error as AsyncClientError};
use integration_test::{BitcoinD, BitcoinDExt as _, Wallet};

fn async_client_for(node: &BitcoinD) -> Client {
    Client::new_with_auth(&node.rpc_url(), auth_for(node)).expect("async client")
}

#[tokio::test]
async fn async__get_best_block_hash__modelled() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let model: Result<bitcoin::BlockHash, AsyncClientError> = client.get_best_block_hash().await;
    let model = model.unwrap();
    let expected = node.client.best_block_hash().expect("best_block_hash");
    assert_eq!(model, expected);
}

#[tokio::test]
async fn async__get_block__modelled() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
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
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let model: Result<u64, AsyncClientError> = client.get_block_count().await;
    let model = model.unwrap();
    assert_eq!(model, 0);
}

#[tokio::test]
#[cfg(not(feature = "v18_and_below"))]
async fn async__get_block_filter__modelled() {
    let node = BitcoinD::with_wallet(Wallet::None, &["-blockfilterindex"]);
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
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let model: Result<bitcoin::BlockHash, AsyncClientError> = client.get_block_hash(0).await;
    let model = model.unwrap();
    let expected = node.client.best_block_hash().expect("best_block_hash");
    assert_eq!(model, expected);
}

#[tokio::test]
async fn async__get_block_header__modelled() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
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
async fn async__get_blockchain_info__modelled() {
    let node = BitcoinD::with_wallet(Wallet::None, &["-prune=10000"]);
    let client = async_client_for(&node);

    let model: Result<mtype::GetBlockchainInfo, AsyncClientError> =
        client.get_blockchain_info().await;
    let model = model.unwrap();

    assert_eq!(model.blocks, 0);
    assert!(model.pruned);
}

#[tokio::test]
async fn async__get_raw_mempool__modelled() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let model: Result<Vec<bitcoin::Txid>, AsyncClientError> = client.get_raw_mempool().await;
    let model = model.unwrap();
    assert!(model.is_empty());
}

#[tokio::test]
async fn async__get_raw_transaction__modelled() {
    let node = BitcoinD::with_wallet(Wallet::None, &["-txindex"]);
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

#[tokio::test]
async fn async__get_tx_out__modelled() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let client = async_client_for(&node);
    let (_address, tx) = node.create_mined_transaction();
    let txid = tx.compute_txid();

    let model: Result<Option<mtype::GetTxOut>, AsyncClientError> =
        client.get_tx_out(&bitcoin::OutPoint { txid, vout: 1 }, true).await;
    let model = model.unwrap().expect("unspent output");
    assert!(!model.coinbase);

    let missing: Result<Option<mtype::GetTxOut>, AsyncClientError> =
        client.get_tx_out(&bitcoin::OutPoint { txid, vout: 2 }, true).await;
    assert!(missing.unwrap().is_none());
}

fn auth_for(node: &BitcoinD) -> Auth { Auth::CookieFile(node.params.cookie_file.clone()) }

#[tokio::test]
async fn async__server_version__returns_positive_integer() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let version: Result<usize, AsyncClientError> = client.server_version().await;
    let version = version.unwrap();
    assert!(version > 0);
}
