// SPDX-License-Identifier: CC0-1.0

//! Integration tests for the codegen-generated `into_model` conversions.
//!
//! Unlike `integration_test/tests/into_model_diff.rs` (which needs a live Bitcoin Core node and
//! diffs the generated conversion against the hand-written one), these tests are node-free and
//! self-contained: they feed canned JSON (the shape Core puts on the wire) into a generated raw
//! type, run `into_model`, and assert the resulting `crate::model` value.
//!
//! They exercise the conversion module as a black box through its public API, so they pin the
//! observable behaviour of `into_model` before the codebase is refactored. The point is a
//! regression net: change the generator, re-run `just codegen`, and these must still pass. They
//! deliberately cover one case per conversion mechanism rather than every RPC:
//!
//! - newtype + leaf rules (`String -> BlockHash`, `i64 -> u64`),
//! - a multi-field reconstruction (`gettxout` -> `TxOut` + `Address`),
//! - a known-bug compatibility shim (`dumptxoutset.coins_written`, backlog #1),
//! - the `decodepsbt` whole-`Psbt` reconstruction,
//! - the `getrawmempool` one-enum-to-three-models fan-out (all three response shapes),
//! - error propagation out of a failed leaf conversion.

use bitcoin::address::NetworkUnchecked;
use bitcoin::hashes::sha256;
use bitcoin::{Address, Amount, BlockHash, ScriptBuf, Txid, Wtxid};
use corepc_types::model;
use corepc_types::v30::generated::{
    DecodePsbt, DumpTxOutSet, GetBestBlockHash, GetBlockCount, GetRawMempool, GetTxOutVariant1,
};
use serde_json::json;

// Recognisable, always-valid 32-byte hex values (any 64 hex chars parse as these hash types).
const TXID_1: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const TXID_2: &str = "2222222222222222222222222222222222222222222222222222222222222222";
const WTXID: &str = "3333333333333333333333333333333333333333333333333333333333333333";
const BLOCK_HASH: &str = "00000000000000000000000000000000000000000000000000000000000000ab";
const SET_HASH: &str = "4444444444444444444444444444444444444444444444444444444444444444";
// A mainnet P2PKH address; parses fine as `Address<NetworkUnchecked>` regardless of network.
const ADDRESS: &str = "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2";

// == Newtype + leaf rules ==

/// `getbestblockhash`: a raw `String` newtype converts to a `BlockHash` newtype.
#[test]
fn get_best_block_hash_parses_into_block_hash() {
    let raw: GetBestBlockHash = serde_json::from_value(json!(BLOCK_HASH)).unwrap();
    let model = raw.into_model().expect("valid block hash");
    assert_eq!(model.0, BLOCK_HASH.parse::<BlockHash>().unwrap());
}

/// `getblockcount`: a raw `i64` newtype narrows to the `u64` the model holds.
#[test]
fn get_block_count_narrows_i64_to_u64() {
    let raw: GetBlockCount = serde_json::from_value(json!(812_345)).unwrap();
    let model = raw.into_model().expect("non-negative height");
    assert_eq!(model.0, 812_345_u64);
}

/// A leaf conversion that fails surfaces as the typed error, it does not panic or silently drop.
#[test]
fn bad_hash_string_propagates_as_error() {
    let raw: GetBestBlockHash = serde_json::from_value(json!("not-a-valid-hash")).unwrap();
    assert!(raw.into_model().is_err(), "an unparseable hash must be a conversion error");
}

// == Multi-field reconstruction ==

/// `gettxout` returns a flat `{ value, scriptPubKey: { hex, address } }`; the model re-assembles it
/// into a strong `TxOut` plus an `Option<Address>` (a `RECONSTRUCT` rule reading several raw fields).
#[test]
fn get_tx_out_reconstructs_txout_and_address() {
    let raw: GetTxOutVariant1 = serde_json::from_value(json!({
        "bestblock": BLOCK_HASH,
        "coinbase": false,
        "confirmations": 6,
        "scriptPubKey": {
            "asm": "OP_TRUE",
            "desc": "raw(51)#abcdef",
            "hex": "51",
            "type": "nonstandard",
            "address": ADDRESS,
        },
        "value": 1.5,
    }))
    .unwrap();

    let model = raw.into_model().expect("valid gettxout");

    assert_eq!(model.best_block, BLOCK_HASH.parse::<BlockHash>().unwrap());
    assert_eq!(model.confirmations, 6);
    assert!(!model.coinbase);
    // `value` is BTC-denominated; 1.5 BTC == 150_000_000 sat. `hex` decodes to the script bytes.
    assert_eq!(model.tx_out.value, Amount::from_sat(150_000_000));
    assert_eq!(model.tx_out.script_pubkey, ScriptBuf::from_hex("51").unwrap());
    assert_eq!(model.address, Some(ADDRESS.parse::<Address<NetworkUnchecked>>().unwrap()));
}

// == Known-bug compatibility shim ==

/// `dumptxoutset.coins_written` is a UTXO count, but the canonical model wrongly types it as an
/// `Amount` (corepc_bugs_backlog.md #1). Codegen routes it through a shim that discards the value,
/// so the model always reports `Amount::from_sat(0)` here. This test pins that deliberate-wrong
/// behaviour: when the canonical type is fixed and the shim removed, it should start failing and be
/// updated, which is exactly the signal we want.
#[test]
fn dump_tx_out_set_coins_written_goes_through_compat_shim() {
    let raw: DumpTxOutSet = serde_json::from_value(json!({
        "base_hash": BLOCK_HASH,
        "base_height": 100,
        "coins_written": 123_456,
        "nchaintx": 789,
        "path": "/tmp/utxo.dat",
        "txoutset_hash": SET_HASH,
    }))
    .unwrap();

    let model = raw.into_model().expect("valid dumptxoutset");

    // The real count (123_456) is thrown away by the compat shim.
    assert_eq!(model.coins_written, Amount::from_sat(0));
    // Every other field converts normally.
    assert_eq!(model.base_hash, BLOCK_HASH.parse::<BlockHash>().unwrap());
    assert_eq!(model.base_height, 100);
    assert_eq!(model.n_chain_tx, 789);
    assert_eq!(model.tx_out_set_hash, SET_HASH.parse::<sha256::Hash>().unwrap());
    assert_eq!(model.path, "/tmp/utxo.dat");
}

// == decodepsbt whole-Psbt reconstruction ==

/// Minimal `decodepsbt` for an empty PSBT (no inputs/outputs). Exercises the
/// `crate::reconstruct::psbt` bridge end to end: the generated raw type round-trips through JSON
/// into the curated `RawPsbt` and assembles a `bitcoin::Psbt`. `fee` is absent here.
#[test]
fn decode_psbt_reconstructs_empty_psbt() {
    let raw: DecodePsbt = serde_json::from_value(json!({
        "tx": {
            "txid": TXID_1,
            "hash": TXID_1,
            "version": 2,
            "size": 10,
            "vsize": 10,
            "weight": 40,
            "locktime": 0,
            "vin": [],
            "vout": [],
        },
        "global_xpubs": [],
        "psbt_version": 0,
        "proprietary": [],
        "unknown": {},
        "inputs": [],
        "outputs": [],
    }))
    .unwrap();

    let model = raw.into_model().expect("valid empty psbt");

    assert!(model.psbt.unsigned_tx.input.is_empty());
    assert!(model.psbt.unsigned_tx.output.is_empty());
    assert_eq!(model.psbt.version, 0);
    assert!(model.psbt.inputs.is_empty());
    assert!(model.psbt.outputs.is_empty());
    assert_eq!(model.fee, None);
}

/// Same shape but with a `fee` present: the second model field converts the BTC float to an
/// `Amount` independently of the reconstructed PSBT.
#[test]
fn decode_psbt_converts_fee_when_present() {
    let raw: DecodePsbt = serde_json::from_value(json!({
        "fee": 0.0001,
        "tx": {
            "txid": TXID_1,
            "hash": TXID_1,
            "version": 2,
            "size": 10,
            "vsize": 10,
            "weight": 40,
            "locktime": 0,
            "vin": [],
            "vout": [],
        },
        "global_xpubs": [],
        "psbt_version": 0,
        "proprietary": [],
        "unknown": {},
        "inputs": [],
        "outputs": [],
    }))
    .unwrap();

    let model = raw.into_model().expect("valid psbt with fee");
    // 0.0001 BTC == 10_000 sat.
    assert_eq!(model.fee, Some(Amount::from_sat(10_000)));
}

// == getrawmempool: one untagged enum fanned out to three model types ==

/// `verbose=false`: a plain id array becomes `GetRawMempoolResult::List`.
#[test]
fn get_raw_mempool_list_variant() {
    let raw: GetRawMempool = serde_json::from_value(json!([TXID_1, TXID_2])).unwrap();
    match raw.into_model().expect("valid id list") {
        model::GetRawMempoolResult::List(model::GetRawMempool(ids)) => {
            assert_eq!(ids, vec![TXID_1.parse::<Txid>().unwrap(), TXID_2.parse::<Txid>().unwrap()]);
        }
        other => panic!("expected List, got {other:?}"),
    }
}

/// `verbose=true`: a map of id to entry becomes `GetRawMempoolResult::Verbose`, each value run
/// through the generated `MempoolEntry` conversion (a nested generated type).
#[test]
fn get_raw_mempool_verbose_variant() {
    let raw: GetRawMempool = serde_json::from_value(json!({
        TXID_1: {
            "ancestorcount": 1,
            "ancestorsize": 200,
            "bip125-replaceable": true,
            "depends": [],
            "descendantcount": 1,
            "descendantsize": 200,
            "fees": { "ancestor": 0.00001, "base": 0.00001, "descendant": 0.00001, "modified": 0.00001 },
            "height": 500,
            "spentby": [],
            "time": 1_700_000_000_i64,
            "unbroadcast": false,
            "vsize": 200,
            "weight": 800,
            "wtxid": WTXID,
        }
    }))
    .unwrap();

    match raw.into_model().expect("valid verbose map") {
        model::GetRawMempoolResult::Verbose(model::GetRawMempoolVerbose(map)) => {
            let entry = map.get(&TXID_1.parse::<Txid>().unwrap()).expect("entry for txid");
            assert_eq!(entry.vsize, Some(200));
            assert_eq!(entry.size, None); // dropped on v0.19+, canonical optional with no raw field.
            assert_eq!(entry.weight, Some(800));
            assert_eq!(entry.height, 500);
            assert_eq!(entry.wtxid, WTXID.parse::<Wtxid>().unwrap());
            assert_eq!(entry.fees.base, Amount::from_sat(1000)); // 0.00001 BTC.
            assert_eq!(entry.bip125_replaceable, Some(true));
        }
        other => panic!("expected Verbose, got {other:?}"),
    }
}

/// `verbose=false, mempool_sequence=true`: the sequence object becomes
/// `GetRawMempoolResult::Sequence` (the untagged enum disambiguates it from the verbose map).
#[test]
fn get_raw_mempool_sequence_variant() {
    let raw: GetRawMempool =
        serde_json::from_value(json!({ "mempool_sequence": 42, "txids": [TXID_1] })).unwrap();
    match raw.into_model().expect("valid sequence object") {
        model::GetRawMempoolResult::Sequence(seq) => {
            assert_eq!(seq.mempool_sequence, 42);
            assert_eq!(seq.txids, vec![TXID_1.parse::<Txid>().unwrap()]);
        }
        other => panic!("expected Sequence, got {other:?}"),
    }
}
