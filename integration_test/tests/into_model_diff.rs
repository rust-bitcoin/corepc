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
//!
//! Some conversions are deliberately NOT diffed here because the generated `into_model` is meant to
//! diverge from, or is knowingly incomplete relative to, the hand-written one (see
//! `corepc_bugs_backlog.md`):
//!
//! - `dumptxoutset` / `loadtxoutset`: the generated code routes `coins_written` / `coins_loaded`
//!   through a compat shim because the canonical type mis-types a UTXO count as an `Amount`
//!   (backlog #1). The shim throws the value away, so the models cannot match.
//! - `decodescript`: the canonical `model::DecodeScript` is stale vs Core v30 (backlog #2), so the
//!   two sides legitimately disagree on the nested `segwit` data.
//! - `gettransaction`: the hand-written raw type models `mempool_conflicts` as `Option<Vec<_>>` (so
//!   an omitted empty list is `None`), while the generated raw types it as a defaulting `Vec` (so it
//!   is `Some([])`). Both are valid `Option<Vec<Txid>>` readings of "no mempool conflicts"; the
//!   generated conversion is correct, it just disagrees with the hand-written representation here.
//! - `getblockstats`: the `total_weight` / `segwit_total_weight` fields are diffed with the two
//!   weight fields cleared. Core reports these as raw BIP141 weight units, so the generated code
//!   uses `Weight::from_wu`; the hand-written conversion uses `Weight::from_vb` (which multiplies by
//!   4), giving a 4x-too-large value (backlog #5). Every other field is compared directly.
//! - `getrawmempool`: the hand-written client splits the three response shapes into separate typed
//!   methods (each producing one of `model::GetRawMempool` / `GetRawMempoolVerbose` /
//!   `GetRawMempoolSequence`), while the generated client returns a single untagged
//!   `model::GetRawMempoolResult` enum (the `oneOf` is selected by two parameters, so the verbose
//!   splitter cannot pick a variant at the type level). The two sides have different model types by
//!   design, so they are not diffed here.

#![cfg(all(feature = "v30_and_below", not(feature = "v29_and_below")))]
#![allow(non_snake_case)]

use bitcoin::Amount;
use bitcoind::vtype::*; // Sync raw types named in the helpers below.
use bitcoind::{Input, Output};
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
        assert_eq!(
            sync_model, gen_model,
            "generated into_model disagrees with the hand-written one"
        );
    }};
    (infallible: $call:expr, $gen:ty $(,)?) => {{
        let (sync_raw, gen_raw) = __same_input!($call, $gen);
        let sync_model = sync_raw.into_model();
        let gen_model = gen_raw.into_model().expect("generated into_model");
        assert_eq!(
            sync_model, gen_model,
            "generated into_model disagrees with the hand-written one"
        );
    }};
}

/// One RPC response shared by both converters: the sync raw value, plus the same JSON deserialized
/// into the generated raw type (which also checks the generated type is wire-compatible).
macro_rules! __same_input {
    ($call:expr, $gen:ty) => {{
        let sync_raw = $call;
        let mut value =
            bitcoind::serde_json::to_value(&sync_raw).expect("serialize the sync raw response");
        // The hand-written types are version-flexible: some keep fields that older Core versions
        // returned but v30 does not (e.g. the deprecated `addresses` in a scriptPubKey), modelled
        // as `Option` with no `skip_serializing_if`. Re-serializing emits them as `null`, which the
        // v30-specific generated types reject under `serde-deny-unknown-fields`. Real Core v30 JSON
        // simply omits these, so drop nulls to feed the generated type the same shape Core would.
        strip_nulls(&mut value);
        let gen_raw: $gen = bitcoind::serde_json::from_value(value)
            .expect("generated raw type is wire-compatible with the sync raw type");
        (sync_raw, gen_raw)
    }};
}

/// Recursively remove object members whose value is JSON `null`.
///
/// An absent optional member and a `null` one both deserialize to `None`, so this is shape-
/// preserving for the generated types while matching what Core actually puts on the wire.
fn strip_nulls(value: &mut bitcoind::serde_json::Value) {
    use bitcoind::serde_json::Value;
    match value {
        Value::Object(map) => {
            map.retain(|_, v| !v.is_null());
            for v in map.values_mut() {
                strip_nulls(v);
            }
        }
        Value::Array(items) =>
            for v in items.iter_mut() {
                strip_nulls(v);
            },
        _ => {}
    }
}

// == Blockchain: no extra setup ==

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
fn diff__get_block_hash() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    assert_into_model_agrees!(
        node.client.get_block_hash(0).expect("getblockhash"),
        types::v30::generated::GetBlockHash,
    );
}

#[test]
fn diff__get_block_header() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let hash = node.client.best_block_hash().expect("best_block_hash");
    assert_into_model_agrees!(
        node.client.get_block_header(&hash).expect("getblockheader"),
        types::v30::generated::GetBlockHeaderVerbose0,
    );
}

#[test]
fn diff__get_block_header_verbose() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let hash = node.client.best_block_hash().expect("best_block_hash");
    assert_into_model_agrees!(
        node.client.get_block_header_verbose(&hash).expect("getblockheader verbose"),
        types::v30::generated::GetBlockHeaderVerbose1,
    );
}

#[test]
fn diff__get_block_verbose_zero() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let hash = node.client.best_block_hash().expect("best_block_hash");
    assert_into_model_agrees!(
        node.client.get_block_verbose_zero(hash).expect("getblock verbose=0"),
        types::v30::generated::GetBlockVerbose0,
    );
}

#[test]
fn diff__get_block_verbose_one() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let hash = node.client.best_block_hash().expect("best_block_hash");
    assert_into_model_agrees!(
        node.client.get_block_verbose_one(hash).expect("getblock verbose=1"),
        types::v30::generated::GetBlockVerbose1,
    );
}

#[test]
fn diff__get_tx_out_set_info() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    assert_into_model_agrees!(
        node.client.get_tx_out_set_info().expect("gettxoutsetinfo"),
        types::v30::generated::GetTxOutSetInfo,
    );
}

// == Blockchain: needs a mined transaction ==

#[test]
fn diff__get_chain_states() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _tx) = node.create_mined_transaction();
    assert_into_model_agrees!(
        node.client.get_chain_states().expect("getchainstates"),
        types::v30::generated::GetChainStates,
    );
}

#[test]
fn diff__get_chain_tx_stats() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _tx) = node.create_mined_transaction();
    assert_into_model_agrees!(
        node.client.get_chain_tx_stats().expect("getchaintxstats"),
        types::v30::generated::GetChainTxStats,
    );
}

#[test]
fn diff__get_deployment_info() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    node.mine_a_block();
    let hash = node.client.best_block_hash().expect("best_block_hash");
    assert_into_model_agrees!(
        node.client.get_deployment_info(&hash).expect("getdeploymentinfo"),
        types::v30::generated::GetDeploymentInfo,
    );
}

// == Blockchain: needs a parent/child mempool chain ==

#[test]
fn diff__get_mempool_entry() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, txid) = node.create_mempool_transaction();
    assert_into_model_agrees!(
        node.client.get_mempool_entry(txid).expect("getmempoolentry"),
        types::v30::generated::GetMempoolEntry,
    );
}

#[test]
fn diff__get_mempool_ancestors() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, parent_txid) = node.create_mempool_transaction();
    let child_txid = create_child_spending_parent(&node, parent_txid);
    assert_into_model_agrees!(
        node.client.get_mempool_ancestors(child_txid).expect("getmempoolancestors"),
        types::v30::generated::GetMempoolAncestorsVerbose0,
    );
}

#[test]
fn diff__get_mempool_ancestors_verbose() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, parent_txid) = node.create_mempool_transaction();
    let child_txid = create_child_spending_parent(&node, parent_txid);
    assert_into_model_agrees!(
        node.client.get_mempool_ancestors_verbose(child_txid).expect("getmempoolancestors verbose"),
        types::v30::generated::GetMempoolAncestorsVerbose1,
    );
}

#[test]
fn diff__get_mempool_descendants() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, parent_txid) = node.create_mempool_transaction();
    let _child_txid = create_child_spending_parent(&node, parent_txid);
    assert_into_model_agrees!(
        node.client.get_mempool_descendants(parent_txid).expect("getmempooldescendants"),
        types::v30::generated::GetMempoolDescendantsVerbose0,
    );
}

#[test]
fn diff__get_mempool_descendants_verbose() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, parent_txid) = node.create_mempool_transaction();
    let _child_txid = create_child_spending_parent(&node, parent_txid);
    assert_into_model_agrees!(
        node.client
            .get_mempool_descendants_verbose(parent_txid)
            .expect("getmempooldescendants verbose"),
        types::v30::generated::GetMempoolDescendantsVerbose1,
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

// == Raw transactions ==

#[test]
fn diff__get_raw_transaction() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, txid) = node.create_mempool_transaction();
    assert_into_model_agrees!(
        node.client.get_raw_transaction(txid).expect("getrawtransaction"),
        types::v30::generated::GetRawTransactionVerbose0,
    );
}

#[test]
fn diff__get_raw_transaction_verbose() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, txid) = node.create_mempool_transaction();
    assert_into_model_agrees!(
        node.client.get_raw_transaction_verbose(txid).expect("getrawtransaction verbose"),
        types::v30::generated::GetRawTransactionVerbose1,
    );
}

#[test]
fn diff__decode_raw_transaction() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, txid) = node.create_mempool_transaction();
    let tx =
        node.client.get_raw_transaction(txid).expect("getrawtransaction").transaction().unwrap();
    assert_into_model_agrees!(
        node.client.decode_raw_transaction(&tx).expect("decoderawtransaction"),
        types::v30::generated::DecodeRawTransaction,
    );
}

#[test]
fn diff__combine_raw_transaction() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, txid) = node.create_mempool_transaction();
    let tx =
        node.client.get_raw_transaction(txid).expect("getrawtransaction").transaction().unwrap();
    let txs = vec![tx.clone(), tx];
    assert_into_model_agrees!(
        node.client.combine_raw_transaction(&txs).expect("combinerawtransaction"),
        types::v30::generated::CombineRawTransaction,
    );
}

// == Wallet ==

#[test]
fn diff__get_balance() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    assert_into_model_agrees!(
        node.client.get_balance().expect("getbalance"),
        types::v30::generated::GetBalance,
    );
}

#[test]
fn diff__get_balances() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    assert_into_model_agrees!(
        node.client.get_balances().expect("getbalances"),
        types::v30::generated::GetBalances,
    );
}

#[test]
fn diff__list_wallets() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    node.client.create_wallet("diff_wallet").expect("createwallet");
    assert_into_model_agrees!(
        infallible: node.client.list_wallets().expect("listwallets"),
        types::v30::generated::ListWallets,
    );
}

#[test]
fn diff__get_hd_keys() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    assert_into_model_agrees!(
        node.client.get_hd_keys().expect("gethdkeys"),
        types::v30::generated::GetHdKeys,
    );
}

#[test]
fn diff__list_unspent() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    assert_into_model_agrees!(
        node.client.list_unspent().expect("listunspent"),
        types::v30::generated::ListUnspent,
    );
}

#[test]
fn diff__list_lock_unspent() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let utxos = node.client.list_unspent().expect("listunspent").into_model().unwrap();
    let (txid, vout) = (utxos.0[0].txid, utxos.0[0].vout);
    node.client.lock_unspent(&[(txid, vout)]).expect("lockunspent");
    assert_into_model_agrees!(
        node.client.list_lock_unspent().expect("listlockunspent"),
        types::v30::generated::ListLockUnspent,
    );
}

#[test]
fn diff__get_addresses_by_label() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    let label = "diff-label";
    let _addr = node.client.new_address_with_label(label).expect("newaddress with label");
    assert_into_model_agrees!(
        node.client.get_addresses_by_label(label).expect("getaddressesbylabel"),
        types::v30::generated::GetAddressesByLabel,
    );
}

#[test]
fn diff__get_wallet_info() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    assert_into_model_agrees!(
        node.client.get_wallet_info().expect("getwalletinfo"),
        types::v30::generated::GetWalletInfo,
    );
}

// `gettransaction` is intentionally omitted from the strict diff: see the module docs
// (`mempool_conflicts` is `None` vs `Some([])` between the two raw representations).

#[test]
fn diff__list_transactions() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let addr = node.client.new_address().expect("newaddress");
    node.client.send_to_address(&addr, Amount::from_sat(5_000)).expect("sendtoaddress");
    node.mine_a_block();
    assert_into_model_agrees!(
        node.client.list_transactions().expect("listtransactions"),
        types::v30::generated::ListTransactions,
    );
}

#[test]
fn diff__send_to_address() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let address = node.client.new_address().expect("newaddress");
    assert_into_model_agrees!(
        node.client.send_to_address(&address, Amount::from_sat(10_000)).expect("sendtoaddress"),
        types::v30::generated::SendToAddressVerbose0,
    );
}

// == Mining / network / other blockchain ==

#[test]
fn diff__get_mining_info() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    assert_into_model_agrees!(
        node.client.get_mining_info().expect("getmininginfo"),
        types::v30::generated::GetMiningInfo,
    );
}

#[test]
fn diff__get_prioritised_transactions() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    assert_into_model_agrees!(
        node.client.get_prioritised_transactions().expect("getprioritisedtransactions"),
        types::v30::generated::GetPrioritisedTransactions,
    );
}

#[test]
fn diff__get_network_info() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    assert_into_model_agrees!(
        node.client.get_network_info().expect("getnetworkinfo"),
        types::v30::generated::GetNetworkInfo,
    );
}

#[test]
fn diff__get_chain_tips() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    assert_into_model_agrees!(
        node.client.get_chain_tips().expect("getchaintips"),
        types::v30::generated::GetChainTips,
    );
}

#[test]
fn diff__get_tx_out() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let utxos = node.client.list_unspent().expect("listunspent").into_model().unwrap();
    let (txid, vout) = (utxos.0[0].txid, u64::from(utxos.0[0].vout));
    assert_into_model_agrees!(
        node.client.get_tx_out(txid, vout).expect("gettxout"),
        types::v30::generated::GetTxOutVariant1,
    );
}

#[test]
fn diff__get_tx_spending_prevout() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_a1, txid_1) = node.create_mempool_transaction();
    let (_a2, txid_2) = node.create_mempool_transaction();
    let inputs = vec![
        bitcoin::OutPoint { txid: txid_1, vout: 0 },
        bitcoin::OutPoint { txid: txid_2, vout: 0 },
    ];
    assert_into_model_agrees!(
        node.client.get_tx_spending_prevout(&inputs).expect("gettxspendingprevout"),
        types::v30::generated::GetTxSpendingPrevout,
    );
}

#[test]
fn diff__get_block_filter() {
    let node = BitcoinD::with_wallet(Wallet::Default, &["-blockfilterindex"]);
    node.mine_a_block();
    let hash = node.client.best_block_hash().expect("best_block_hash");
    assert_into_model_agrees!(
        node.client.get_block_filter(hash).expect("getblockfilter"),
        types::v30::generated::GetBlockFilter,
    );
}

#[test]
fn diff__scan_tx_out_set() {
    let node = BitcoinD::with_wallet(Wallet::None, &["-coinstatsindex=1"]);
    let dummy_pubkey = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    let scan_desc = format!("pkh({})", dummy_pubkey);
    assert_into_model_agrees!(
        node.client.scan_tx_out_set_start(&[&scan_desc]).expect("scantxoutset start"),
        types::v30::generated::ScanTxOutSetVariant0,
    );
}

// == Util ==

#[test]
fn diff__validate_address() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    let address = node.client.new_address().expect("newaddress");
    assert_into_model_agrees!(
        node.client.validate_address(&address).expect("validateaddress"),
        types::v30::generated::ValidateAddress,
    );
}

// == Wallet: listsinceblock ==

#[test]
fn diff__list_since_block() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let addr = node.client.new_address().expect("newaddress");
    node.client.send_to_address(&addr, Amount::from_sat(7_000)).expect("sendtoaddress");
    node.mine_a_block();
    assert_into_model_agrees!(
        node.client.list_since_block().expect("listsinceblock"),
        types::v30::generated::ListSinceBlock,
    );
}

// == Blockchain: getblockstats ==
//
// Diffed field-by-field with the two weight fields cleared: the hand-written conversion uses
// `Weight::from_vb` (4x too large) while the generated one correctly uses `Weight::from_wu`. See
// the module docs and `corepc_bugs_backlog.md` #5.
#[test]
fn diff__get_block_stats() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _txid) = node.create_mined_transaction();
    let hash = node.client.best_block_hash().expect("best_block_hash");

    let sync_raw = node.client.get_block_stats_by_block_hash(&hash, None).expect("getblockstats");
    let mut value =
        bitcoind::serde_json::to_value(&sync_raw).expect("serialize the sync raw response");
    strip_nulls(&mut value);
    let gen_raw: types::v30::generated::GetBlockStats = bitcoind::serde_json::from_value(value)
        .expect("generated raw type is wire-compatible with the sync raw type");

    let mut sync_model = sync_raw.into_model().expect("hand-written into_model");
    let mut gen_model = gen_raw.into_model().expect("generated into_model");
    // The weight fields legitimately diverge (hand-written bug, backlog #5); compare the rest.
    sync_model.total_weight = None;
    gen_model.total_weight = None;
    sync_model.segwit_total_weight = None;
    gen_model.segwit_total_weight = None;
    assert_eq!(sync_model, gen_model, "generated into_model disagrees with the hand-written one");
}

// == Wallet: listaddressgroupings ==

#[test]
fn diff__list_address_groupings() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let addr = node.client.new_address().expect("newaddress");
    node.client.send_to_address(&addr, Amount::from_sat(20_000)).expect("sendtoaddress");
    node.mine_a_block();
    assert_into_model_agrees!(
        node.client.list_address_groupings().expect("listaddressgroupings"),
        types::v30::generated::ListAddressGroupings,
    );
}

// == Blockchain: getblock verbosity 2 ==

#[test]
fn diff__get_block_verbose_two() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _txid) = node.create_mined_transaction();
    let hash = node.client.best_block_hash().expect("best_block_hash");
    assert_into_model_agrees!(
        node.client.get_block_verbose_two(hash).expect("getblock verbose=2"),
        types::v30::generated::GetBlockVerbose2,
    );
}

// == Blockchain: getdescriptoractivity ==
//
// Only a `receive` activity is exercised: Core's wire field for a spend is `spend_vin`, but the
// hand-written `SpendActivity` raw type names it `spend_vout` with no rename (backlog #6), so it
// cannot deserialize a real spend entry. A freshly funded, unspent address yields receives only.
#[test]
fn diff__get_descriptor_activity() {
    let node = BitcoinD::with_wallet(Wallet::Default, &["-coinstatsindex=1", "-txindex=1"]);
    node.fund_wallet();
    let addr = node.client.new_address().expect("newaddress");
    node.client.send_to_address(&addr, Amount::from_sat(50_000)).expect("sendtoaddress");
    node.mine_a_block();
    let block_hash = node.client.best_block_hash().expect("best_block_hash");
    let scan = format!("addr({})", addr);
    assert_into_model_agrees!(
        node.client
            .get_descriptor_activity(&[block_hash], &[scan.as_str()])
            .expect("getdescriptoractivity"),
        types::v30::generated::GetDescriptorActivity,
    );
}

// == Blockchain: getblock verbosity 3 ==

#[test]
fn diff__get_block_verbose_three() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let (_address, _txid) = node.create_mined_transaction();
    let hash = node.client.best_block_hash().expect("best_block_hash");
    assert_into_model_agrees!(
        node.client.get_block_verbose_three(hash).expect("getblock verbose=3"),
        types::v30::generated::GetBlockVerbose3,
    );
}

// == Raw transactions: decodepsbt ==

#[test]
fn diff__decode_psbt() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let addr = node.client.new_address().expect("newaddress");
    let outputs = std::collections::BTreeMap::from([(addr, Amount::from_sat(100_000))]);
    let funded = node
        .client
        .wallet_create_funded_psbt(vec![], vec![outputs])
        .expect("walletcreatefundedpsbt");
    assert_into_model_agrees!(
        node.client.decode_psbt(&funded.psbt).expect("decodepsbt"),
        types::v30::generated::DecodePsbt,
    );
}

/// Build, fund, sign and broadcast a transaction that spends `parent_txid`, returning its txid.
///
/// Copied from `tests/blockchain.rs`; used to grow a parent/child mempool chain so the
/// `getmempoolancestors` / `getmempooldescendants` responses are non-empty.
fn create_child_spending_parent(node: &BitcoinD, parent_txid: bitcoin::Txid) -> bitcoin::Txid {
    let inputs = vec![Input { txid: parent_txid, vout: 0, sequence: None }];
    let spend_address = node.client.new_address().expect("newaddress");
    let outputs = vec![Output::new(spend_address, Amount::from_sat(100_000))];

    let raw: CreateRawTransaction =
        node.client.create_raw_transaction(&inputs, &outputs).expect("createrawtransaction");
    let unsigned = raw.transaction().expect("raw.transaction");

    let funded: FundRawTransaction =
        node.client.fund_raw_transaction(&unsigned).expect("fundrawtransaction");
    let funded_tx = funded.transaction().expect("funded.transaction");

    let signed: SignRawTransaction = node
        .client
        .sign_raw_transaction_with_wallet(&funded_tx)
        .expect("signrawtransactionwithwallet");
    let sign_raw_transaction = signed.into_model().expect("SignRawTransaction into model");
    let child_txid = sign_raw_transaction.tx.compute_txid();
    let _ = node.client.send_raw_transaction(&sign_raw_transaction.tx).expect("sendrawtransaction");

    child_txid
}
