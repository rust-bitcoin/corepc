// SPDX-License-Identifier: CC0-1.0

//! Tests for the async client. Versions 25 to 31 are currently supported

#![cfg(feature = "v31_and_below")]
#![cfg(not(feature = "v24_and_below"))]

use bitcoin::address::KnownHrp;
use bitcoin::{Address, CompressedPublicKey, PrivateKey};
use corepc_client::client_async::{Auth, Client};
use integration_test::{BitcoinD, BitcoinDExt as _, Wallet};

fn async_client_for(node: &BitcoinD) -> Client {
    Client::new_with_auth(&node.rpc_url(), auth_for(node)).expect("async client")
}

#[tokio::test]
async fn get_best_block_hash() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    // Tests that the async-client has this function.
    let got = client.get_best_block_hash().await.unwrap();
    // Grabs the block hash using the sync client which we know works.
    let want = node.client.best_block_hash().expect("best_block_hash");
    // Tests that the async client returned the same block hash as the sync client.
    assert_eq!(got, want);
}

#[tokio::test]
async fn get_block() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let want = node.client.best_block_hash().expect("best_block_hash");
    let got = client.get_block(&want).await.unwrap();
    assert_eq!(got.block_hash(), want);

    let got = client.get_block_verbose(&want).await.unwrap();
    assert_eq!(got.hash, want);
}

#[tokio::test]
async fn get_block_count() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let got = client.get_block_count().await.unwrap();
    assert_eq!(got, 0);
}

#[tokio::test]
#[cfg(not(feature = "v18_and_below"))]
async fn get_block_filter() {
    let node = BitcoinD::with_wallet(Wallet::None, &["-blockfilterindex"]);
    let client = async_client_for(&node);

    let best_hash = node.client.best_block_hash().expect("best_block_hash");
    let got = client.get_block_filter(&best_hash).await.unwrap();

    assert!(!got.filter.is_empty());
    assert_eq!(got.header.to_string().len(), 64);
}

#[tokio::test]
async fn get_block_hash() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let got = client.get_block_hash(0).await.unwrap();
    let want = node.client.best_block_hash().expect("best_block_hash");
    assert_eq!(got, want);
}

#[tokio::test]
async fn get_block_header() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let want = node.client.best_block_hash().expect("best_block_hash");
    let got = client.get_block_header(&want).await.unwrap();
    assert_eq!(got.block_hash(), want);

    let got = client.get_block_header_verbose(&want).await.unwrap();
    assert_eq!(got.hash, want);
    assert_eq!(got.height, 0);
}

#[tokio::test]
async fn get_raw_mempool() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let got = client.get_raw_mempool().await.unwrap();
    assert!(got.is_empty());
}

#[tokio::test]
async fn get_raw_transaction() {
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

    let got = client.get_raw_transaction(&txid).await.unwrap();
    assert_eq!(got.compute_txid(), txid);
}

#[tokio::test]
async fn get_tx_out() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
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

    let got = client.get_tx_out(&txid, 0).await.unwrap();
    assert!(got.coinbase);
    assert_eq!(got.tx_out, block.txdata[0].output[0]);
}

#[tokio::test]
async fn get_blockchain_info() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let got = client.get_blockchain_info().await.unwrap();
    assert_eq!(got.chain, bitcoin::Network::Regtest);
}

fn auth_for(node: &BitcoinD) -> Auth { Auth::CookieFile(node.params.cookie_file.clone()) }

#[tokio::test]
async fn server_version() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let got = client.server_version().await.unwrap();
    assert!(got > 0);
}
