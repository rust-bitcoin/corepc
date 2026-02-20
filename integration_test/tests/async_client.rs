// SPDX-License-Identifier: CC0-1.0

//! Tests for the async client.

#![allow(non_snake_case)] // Test names intentionally use double underscore.

#[cfg(all(feature = "v18_and_below", not(feature = "v17")))]
use corepc_client::client_async::v18 as async_client_v18;
#[cfg(not(feature = "v18_and_below"))]
use corepc_client::client_async::v19 as async_client_v19;
#[cfg(not(feature = "v20_and_below"))]
use corepc_client::client_async::v21 as async_client_v21;
#[cfg(all(not(feature = "v22_and_below"), feature = "v23_and_below"))]
use corepc_client::client_async::v23 as async_client_v23;
#[cfg(not(feature = "v23_and_below"))]
use corepc_client::client_async::v24 as async_client_v24;
#[cfg(not(feature = "v28_and_below"))]
use corepc_client::client_async::v29 as async_client_v29;
use corepc_client::client_async::{v17 as async_client, Auth};
use integration_test::{Node, NodeExt as _, Wallet};

fn async_client_for(node: &Node) -> async_client::Client {
    async_client::Client::new_with_auth(&node.rpc_url(), auth_for(node)).expect("async client")
}

#[tokio::test]
async fn async_client__get_block_count_and_hash() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let count = client.get_block_count().await.expect("getblockcount");
    assert_eq!(count.0, 0);

    let json = client.get_block_hash(0).await.expect("getblockhash");
    let model = json.into_model().expect("getblockhash model");
    let expected = node.client.best_block_hash().expect("best_block_hash");
    assert_eq!(model.0, expected);
}

#[tokio::test]
async fn async_client__get_block_variants() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let best_hash = node.client.best_block_hash().expect("best_block_hash");

    let block = client.get_block(best_hash).await.expect("getblock");
    assert_eq!(block.block_hash(), best_hash);

    let block_v0 = client.get_block_verbose_zero(best_hash).await.expect("getblock verbose=0");
    let block_from_hex = block_v0.block().expect("getblock verbose=0 decode");
    assert_eq!(block_from_hex.block_hash(), best_hash);

    #[cfg(feature = "v28_and_below")]
    {
        let block_v1 = client.get_block_verbose_one(best_hash).await.expect("getblock verbose=1");
        assert_eq!(block_v1.hash, best_hash.to_string());

        let block_info = client.get_block_info(best_hash).await.expect("getblock info");
        assert_eq!(block_info.hash, best_hash.to_string());
    }

    #[cfg(not(feature = "v28_and_below"))]
    {
        let client = async_client_v29::Client::new_with_auth(&node.rpc_url(), auth_for(&node))
            .expect("async client v29");
        let block_v1 =
            client.get_block_verbose_one(best_hash).await.expect("getblock verbose=1 v29");
        assert_eq!(block_v1.hash, best_hash.to_string());

        let block_info = client.get_block_info(best_hash).await.expect("getblock info v29");
        assert_eq!(block_info.hash, best_hash.to_string());
    }
}

#[tokio::test]
async fn async_client__get_block_header_variants() {
    let node = Node::with_wallet(Wallet::None, &[]);
    let client = async_client_for(&node);

    let best_hash = node.client.best_block_hash().expect("best_block_hash");
    let header = client.get_block_header(&best_hash).await.expect("getblockheader");
    assert!(!header.0.is_empty());

    #[cfg(feature = "v28_and_below")]
    {
        let header_verbose =
            client.get_block_header_verbose(&best_hash).await.expect("getblockheader verbose");

        assert_eq!(header_verbose.hash, best_hash.to_string());
        assert_eq!(header_verbose.height, 0);

        let header_info =
            client.get_block_header_info(&best_hash).await.expect("getblockheader info");

        assert_eq!(header_info.hash, best_hash.to_string());
        assert_eq!(header_info.height, 0);
    }

    #[cfg(not(feature = "v28_and_below"))]
    {
        let client = async_client_v29::Client::new_with_auth(&node.rpc_url(), auth_for(&node))
            .expect("async client v29");
        let header_verbose =
            client.get_block_header_verbose(&best_hash).await.expect("getblockheader verbose v29");

        assert_eq!(header_verbose.hash, best_hash.to_string());
        assert_eq!(header_verbose.height, 0);

        let header_info =
            client.get_block_header_info(&best_hash).await.expect("getblockheader info v29");

        assert_eq!(header_info.hash, best_hash.to_string());
        assert_eq!(header_info.height, 0);
    }
}

#[tokio::test]
async fn async_client__get_raw_mempool_variants() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let (_address, txid) = node.create_mempool_transaction();
    let txid_str = txid.to_string();
    let client = async_client_for(&node);

    let mempool = client.get_raw_mempool().await.expect("getrawmempool");
    assert!(mempool.0.iter().any(|id| id == &txid_str));

    #[cfg(feature = "v17")]
    {
        let mempool_verbose =
            client.get_raw_mempool_verbose().await.expect("getrawmempool verbose");
        assert!(mempool_verbose.0.contains_key(&txid_str));
    }

    #[cfg(all(feature = "v18_and_below", not(feature = "v17")))]
    {
        let client = async_client_v18::Client::new_with_auth(&node.rpc_url(), auth_for(&node))
            .expect("async client v18");
        let mempool_verbose =
            client.get_raw_mempool_verbose().await.expect("getrawmempool verbose v18");
        assert!(mempool_verbose.0.contains_key(&txid_str));
    }

    #[cfg(all(not(feature = "v18_and_below"), feature = "v20_and_below"))]
    {
        let client = async_client_v19::Client::new_with_auth(&node.rpc_url(), auth_for(&node))
            .expect("async client v19");
        let mempool_verbose =
            client.get_raw_mempool_verbose().await.expect("getrawmempool verbose v19");
        assert!(mempool_verbose.0.contains_key(&txid_str));
    }

    #[cfg(all(not(feature = "v20_and_below"), feature = "v22_and_below"))]
    {
        let client = async_client_v21::Client::new_with_auth(&node.rpc_url(), auth_for(&node))
            .expect("async client v21");
        let mempool_verbose =
            client.get_raw_mempool_verbose().await.expect("getrawmempool verbose v21");
        assert!(mempool_verbose.0.contains_key(&txid_str));
    }

    #[cfg(all(not(feature = "v22_and_below"), feature = "v23_and_below"))]
    {
        let client = async_client_v23::Client::new_with_auth(&node.rpc_url(), auth_for(&node))
            .expect("async client v23");
        let mempool_verbose =
            client.get_raw_mempool_verbose().await.expect("getrawmempool verbose v23");
        assert!(mempool_verbose.0.contains_key(&txid_str));
    }

    #[cfg(not(feature = "v23_and_below"))]
    {
        let client = async_client_v24::Client::new_with_auth(&node.rpc_url(), auth_for(&node))
            .expect("async client v24");
        let mempool_verbose =
            client.get_raw_mempool_verbose().await.expect("getrawmempool verbose v24");
        assert!(mempool_verbose.0.contains_key(&txid_str));
    }

    #[cfg(not(feature = "v20_and_below"))]
    {
        let client = async_client_v21::Client::new_with_auth(&node.rpc_url(), auth_for(&node))
            .expect("async client v21");
        let mempool_sequence =
            client.get_raw_mempool_sequence().await.expect("getrawmempool sequence");
        assert!(mempool_sequence.txids.iter().any(|id| id == &txid_str));
        assert!(mempool_sequence.mempool_sequence > 0);
    }
}

#[tokio::test]
async fn async_client__get_raw_transaction_variants() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();

    let (_address, txid) = node.create_mempool_transaction();
    let client = async_client_for(&node);

    let raw = client.get_raw_transaction(txid).await.expect("getrawtransaction");
    assert!(!raw.0.is_empty());

    let verbose =
        client.get_raw_transaction_verbose(txid).await.expect("getrawtransaction verbose");
    assert_eq!(verbose.txid, txid.to_string());
    assert!(!verbose.hex.is_empty());
}

#[tokio::test]
#[cfg(not(feature = "v18_and_below"))]
async fn async_client__get_block_filter() {
    let node = Node::with_wallet(Wallet::Default, &["-blockfilterindex"]);
    node.mine_a_block();
    let client = async_client_v19::Client::new_with_auth(&node.rpc_url(), auth_for(&node))
        .expect("async client v19");

    let best_hash = node.client.best_block_hash().expect("best_block_hash");
    let filter = client.get_block_filter(best_hash).await.expect("getblockfilter");
    assert!(!filter.filter.is_empty());
}

fn auth_for(node: &Node) -> Auth { Auth::CookieFile(node.params.cookie_file.clone()) }
