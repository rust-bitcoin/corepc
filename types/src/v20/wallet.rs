// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v20` - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

// use alloc::collections::BTreeMap;

// use bitcoin::amount::ParseAmountError;
// use bitcoin::key::{self, PrivateKey};
// use bitcoin::{hex, Amount, Txid};
use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `abortrescan`.
///
/// > abortrescan
/// >
/// > Returns null
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AbortRescan(pub bool);

/// Result of JSON-RPC method `encryptwallet`.
///
/// > encryptwallet
/// >
/// > Arguments:
/// > 1. passphrase (string, required) The pass phrase to encrypt the wallet with. It must be at least 1 character, but should be long.
/// >
/// > Returns "str" (string) A string with further instructions
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct EncryptWallet(pub String);
