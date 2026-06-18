// SPDX-License-Identifier: CC0-1.0

//! Manual compatibility shims for `into_model` conversions that `codegen` produces
//! correctly but that do not match the (buggy) canonical `crate::model` types.
//!
//! Emitted by `codegen` from a fixed override table, so every version carries the same
//! shims. Each one is a deliberate WRONG, compilable placeholder that isolates a known bug
//! in `types/` so the generated code keeps building. Each has a matching entry in
//! `corepc_bugs_backlog.md`. When a canonical type is fixed, drop the override and let
//! codegen emit the correct conversion inline (left commented at each call site).
//!
//! Bitcoin Core `30`.

#![allow(unused_imports)]

use bitcoin::{Address, Amount, BlockHash, FeeRate};

/// WRONG placeholder: canonical types `coins_written` as `Amount` but Core returns a UTXO count (corepc_bugs_backlog.md #1).
///
/// Codegen produces `self.coins_written` (u64); the canonical field wrongly wants `Amount`, so this
/// discards the real value to compile. TODO: fix the canonical type, then delete this shim.
pub fn dump_tx_out_set_coins_written(_v: u64) -> Amount { Amount::from_sat(0) }

/// WRONG placeholder: canonical types `coins_loaded` as `Amount` but Core returns a UTXO count (corepc_bugs_backlog.md #1).
///
/// Codegen produces `self.coins_loaded` (u64); the canonical field wrongly wants `Amount`, so this
/// discards the real value to compile. TODO: fix the canonical type, then delete this shim.
pub fn load_tx_out_set_coins_loaded(_v: u64) -> Amount { Amount::from_sat(0) }
