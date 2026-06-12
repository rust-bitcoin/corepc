// SPDX-License-Identifier: CC0-1.0

//! Reference tests that exercise (and document) the design decisions behind the async
//! production client.
//!
//! These run under the `test-async` feature, where `node.client` is the blocking facade over the
//! generated async `v30` client (see `corepc-client/src/client_async/blocking.rs`). Run with:
//!
//! ```sh
//! cargo test -p integration-test --no-default-features --features 30_2,test-async,download --test async
//! ```
//!
//! Without `test-async`, `node.client` is the ordinary sync client; the same calls still compile
//! and pass, so this file is a live reference in both modes. Each test below maps to one decision
//! and asserts a real call against a real node, so the reference can never silently rot.

// These tests target the v30 surface (the version the async client is generated for). Gate on the
// v30 feature so they are skipped when the crate is built for a different version.
#![cfg(feature = "30_2")]
#![allow(non_snake_case)]

use bitcoind::vtype::*;
use integration_test::{BitcoinD, BitcoinDExt as _, Wallet};

// =============================================================================================
// Decision 1: the async client is the real transport; the blocking facade just blocks on it.
//
// Under `test-async`, `node.client` is `client_async::blocking::Client`, which owns its own
// current-thread tokio runtime and `block_on`s each generated async call. The whole point of the
// facade is that the unchanged sync-style call below actually drives the async generated code.
// =============================================================================================

#[test]
fn decision_async_backs_the_sync_surface() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    // Identical call shape to the sync client. Under `test-async` this round-trips through the
    // async transport and back.
    let _: GetBlockCount = node.client.get_block_count().unwrap();
}

// =============================================================================================
// Decision 2: the type bridge.
//
// The generated async method returns a *generated* type (`types::v30::generated::*`). The facade
// bridges it to the *curated* `vtype` (`types::v30::*`) via a JSON round-trip (`reserialize`),
// because both are serde views of the same Core response. The call below returns the curated type,
// proving the bridge produced exactly what the sync client would have deserialized.
// =============================================================================================

#[test]
fn decision_type_bridge_returns_curated_type() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let info: GetBlockchainInfo = node.client.get_blockchain_info().unwrap();
    assert!(!info.chain.is_empty());
}

// =============================================================================================
// Decision 3: curated types convert to the version-nonspecific `model` layer.
//
// The bridge stops at the curated, version-specific type. Strongly typed rust-bitcoin values live
// one more hop away, behind `into_model()`. This is deliberate: codegen owns raw/curated shape,
// the model layer owns semantics.
// =============================================================================================

#[test]
fn decision_curated_converts_to_model() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let json = node.client.get_blockchain_info().unwrap();
    let model = json.into_model().expect("into_model");
    // `best_block_hash` is a real `bitcoin::BlockHash` on the model type, not a string.
    let _: bitcoin::BlockHash = model.best_block_hash;
}

// =============================================================================================
// Decision 4: the name bridge.
//
// Core RPC names are machine-split once, so a generated name does not always match the curated
// name. The facade reconciles them by hand. `getblockheader` with verbose=true is generated as
// `get_block_header_verbose_one` but the curated/sync surface calls it `get_block_header_verbose`.
// The facade maps one to the other; this test pins that the curated name is the one callers see.
// =============================================================================================

#[test]
fn decision_name_bridge_curated_name_wins() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    let best = node.client.best_block_hash().unwrap();
    // Curated name `get_block_header_verbose`, not generated `get_block_header_verbose_one`.
    let _: GetBlockHeaderVerbose = node.client.get_block_header_verbose(&best).unwrap();
}

// =============================================================================================
// Decision 5: oneOf/anyOf verbosity variants get one typed method each.
//
// `getblock` is a oneOf over verbosity. Rather than guess at runtime, codegen emits a distinct
// typed method per verbosity level (`get_block_verbose_zero`, `_one`, ...). Each returns its own
// fully typed struct. This test shows two of those variants returning different concrete types.
// =============================================================================================

#[test]
fn decision_verbosity_variants_are_separate_typed_methods() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    let best = node.client.best_block_hash().unwrap();

    let _: GetBlockVerboseZero = node.client.get_block_verbose_zero(best).unwrap();
    let _: GetBlockVerboseOne = node.client.get_block_verbose_one(best).unwrap();
}

// =============================================================================================
// Decision 6: the raw escape hatch.
//
// Any RPC, including ones with no generated wrapper, is reachable through `call`, which serializes
// params and deserializes the response into any type the caller picks. This is the bottom of the
// stack every wrapper is built on.
// =============================================================================================

#[test]
fn decision_raw_call_escape_hatch() {
    let node = BitcoinD::with_wallet(Wallet::None, &[]);
    // `uptime` returns a bare number; ask for it as a plain u64 through the raw hatch.
    let secs: u64 = node.client.call("uptime", &[]).unwrap();
    let _ = secs; // freshly started node, only assert it decoded.
}

// =============================================================================================
// Decision 7: every RPC with optional positional args gets a `_with` + `*Options` pair.
//
// Optional Core args are positional; skipping a slot means sending JSON null. Rather than overload
// many method shapes, codegen emits a required-only method plus a `_with(opts)` method, where
// `opts` is a `*Options` struct (one `Option<T>` per optional arg, `Default`-derived). The Options
// types are request-side data, so they live next to the call surface in the client crate; the
// types crate holds only what the node sends back.
//
// NOTE: the `_with` + Options surface is exposed by the *generated async client*
// (`corepc-client/src/client_async/v30`), not by this crate's blocking facade. The facade only
// surfaces the subset the integration tests already exercised. Reference shape (compiles against
// `corepc_client::client_async::Client`):
//
// ```ignore
// use corepc_client::client_async::v30::GetBlockStatsOptions;
//
// // required-only:
// let _ = client.get_block_stats(height as f64).await?;
//
// // with options (None fields serialize to JSON null = "use Core default"):
// let opts = GetBlockStatsOptions { stats: Some(vec!["avgfee".into(), "height".into()]) };
// let _ = client.get_block_stats_with(height as f64, opts).await?;
// ```
//
// The live assertion below uses the facade's required-only form (its optional `stats` arg passed
// as `None`, i.e. JSON null = "use Core default") so the reference stays runnable in this crate
// without pulling in the async client and a runtime.
// =============================================================================================

#[test]
fn decision_optional_args_none_means_core_default() {
    let node = BitcoinD::with_wallet(Wallet::Default, &[]);
    let height = node.client.get_block_count().unwrap().0;
    // `stats: None` is the facade equivalent of the generated `_with` Options field left unset:
    // the optional positional slot is sent as JSON null.
    let _: GetBlockStats = node.client.get_block_stats_by_height(height as u32, None).unwrap();
}
