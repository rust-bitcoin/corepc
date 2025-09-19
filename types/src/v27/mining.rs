// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v27` - mining.
//!
//! Types for methods found under the `== Mining ==` section of the API docs.

use std::collections::BTreeMap;

use bitcoin::{hex, Amount, Txid};
use serde::{Deserialize, Serialize};

use crate::model;

/// Result of the JSON-RPC method `getprioritisedtransactions`.
///
/// > getprioritisedtransactions
/// >
/// > Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct GetPrioritisedTransactions(
    /// prioritisation keyed by txid.
    pub BTreeMap<String, PrioritisedTransaction>,
);

/// An individual prioritised transaction. Part of `getprioritisedtransactions`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct PrioritisedTransaction {
    /// Transaction fee delta in satoshis.
    pub fee_delta: i64,
    /// Whether this transaction is currently in mempool.
    pub in_mempool: bool,
    /// Modified fee in satoshis. Only returned if in_mempool=true.
    pub modified_fee: Option<u64>,
}

impl GetPrioritisedTransactions {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetPrioritisedTransactions, hex::HexToArrayError> {
        let mut map = BTreeMap::new();
        for (k, v) in self.0.into_iter() {
            let txid = k.parse::<Txid>()?;
            map.insert(txid, v.into_model());
        }
        Ok(model::GetPrioritisedTransactions(map))
    }
}

impl PrioritisedTransaction {
    /// Converts version specific type to a version nonspecific, more strongly typed type.
    pub fn into_model(self) -> model::PrioritisedTransaction {
        model::PrioritisedTransaction {
            fee_delta: Amount::from_sat(self.fee_delta as u64),
            in_mempool: self.in_mempool,
            modified_fee: self.modified_fee.map(Amount::from_sat),
        }
    }
}
