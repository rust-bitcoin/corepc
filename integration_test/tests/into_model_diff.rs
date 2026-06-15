// SPDX-License-Identifier: CC0-1.0

//! Differential tests for the codegen-generated `into_model` conversions.
//!
//! For each RPC we take ONE real node response and feed it to two converters:
//!
//! - the hand-written ("sync") raw type in `bitcoind::vtype` and its `into_model`, and
//! - the codegen-generated ("async") raw type in `corepc_types::v30::generated` and its `into_model`.
//!
//! Both target the same `crate::model` type, so a correct generated conversion must produce the
//! exact same model value as the trusted hand-written one. We get a single shared input by calling
//! the RPC once, serializing the sync raw response back to JSON, and deserializing that into the
//! generated raw type (the generated type deserializing it also checks wire compatibility).
//!
//! These only run against a Bitcoin Core v30 node (`--features 30_2`); the generated types are v30.

#![cfg(all(feature = "v30_and_below", not(feature = "v29_and_below")))]
#![allow(non_snake_case)]

use integration_test::{BitcoinD, BitcoinDExt as _, Wallet};

/// Assert the generated `into_model` agrees with the hand-written one on the same node response.
///
/// `$call` evaluates to a hand-written raw response value; `$gen` is the matching generated raw type.
/// The generated `into_model` is always fallible; the hand-written one is fallible by default, or
/// infallible with the `infallible:` form (it returns the model directly).
macro_rules! assert_into_model_agrees {
    ($call:expr, $gen:ty $(,)?) => {{
        let (sync_raw, gen_raw) = __same_input!($call, $gen);
        let sync_model = sync_raw.into_model().expect("hand-written into_model");
        let gen_model = gen_raw.into_model().expect("generated into_model");
        assert_eq!(sync_model, gen_model, "generated into_model disagrees with the hand-written one");
    }};
    (infallible: $call:expr, $gen:ty $(,)?) => {{
        let (sync_raw, gen_raw) = __same_input!($call, $gen);
        let sync_model = sync_raw.into_model();
        let gen_model = gen_raw.into_model().expect("generated into_model");
        assert_eq!(sync_model, gen_model, "generated into_model disagrees with the hand-written one");
    }};
}

/// One RPC response shared by both converters: the sync raw value, plus the same JSON deserialized
/// into the generated raw type (which also checks the generated type is wire-compatible).
macro_rules! __same_input {
    ($call:expr, $gen:ty) => {{
        let sync_raw = $call;
        let value =
            bitcoind::serde_json::to_value(&sync_raw).expect("serialize the sync raw response");
        let gen_raw: $gen = bitcoind::serde_json::from_value(value)
            .expect("generated raw type is wire-compatible with the sync raw type");
        (sync_raw, gen_raw)
    }};
}

#[test]
fn diff__get_best_block_hash() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    assert_into_model_agrees!(
        node.client.get_best_block_hash().expect("getbestblockhash"),
        types::v30::generated::GetBestBlockHash,
    );
}

#[test]
fn diff__get_block_count() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    assert_into_model_agrees!(
        infallible: node.client.get_block_count().expect("getblockcount"),
        types::v30::generated::GetBlockCount,
    );
}

#[test]
fn diff__get_difficulty() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    assert_into_model_agrees!(
        infallible: node.client.get_difficulty().expect("getdifficulty"),
        types::v30::generated::GetDifficulty,
    );
}

#[test]
fn diff__get_blockchain_info() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    assert_into_model_agrees!(
        node.client.get_blockchain_info().expect("getblockchaininfo"),
        types::v30::generated::GetBlockchainInfo,
    );
}

#[test]
fn diff__get_mempool_info() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _txid) = node.create_mempool_transaction();
    assert_into_model_agrees!(
        node.client.get_mempool_info().expect("getmempoolinfo"),
        types::v30::generated::GetMempoolInfo,
    );
}
