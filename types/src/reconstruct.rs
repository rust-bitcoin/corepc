// SPDX-License-Identifier: CC0-1.0

//! Rebuilds whole rust-bitcoin values from a generated decoded response type.
//!
//! Some `crate::model` types wrap a whole `bitcoin::Transaction` that Core only hands back as
//! decoded fields (`decoderawtransaction`, `decodepsbt`, `gettransaction` with `verbose`). The
//! curated [`crate::psbt::RawTransaction`] already knows how to assemble a `Transaction` from those
//! fields, so the generated `into_model` bridges its value through JSON into `RawTransaction` and
//! reuses [`RawTransaction::to_transaction`] rather than re-deriving the consensus assembly per
//! generated type.

use core::fmt;

use bitcoin::Transaction;
use serde::Serialize;
use serde_json::Value;

use crate::psbt::{RawTransaction, RawTransactionError};

/// Error rebuilding a `Transaction` from a generated decoded response type.
#[derive(Debug)]
pub enum ReconstructError {
    /// Bridging the generated type through JSON into `RawTransaction` failed.
    Bridge(serde_json::Error),
    /// Assembling the `bitcoin::Transaction` from the decoded fields failed.
    Transaction(RawTransactionError),
}

impl fmt::Display for ReconstructError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bridge(e) =>
                write!(f, "bridging the decoded transaction through JSON failed: {e}"),
            Self::Transaction(e) =>
                write!(f, "assembling the transaction from decoded fields failed: {e}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ReconstructError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Bridge(e) => Some(e),
            Self::Transaction(e) => Some(e),
        }
    }
}

/// Rebuilds a `Transaction` from a generated decoded response type.
///
/// Bridges `value` through JSON into the curated [`RawTransaction`] (dropping `null` members so an
/// absent optional field is not rejected by its `deny_unknown_fields`), then assembles the
/// transaction.
pub fn transaction<T: Serialize>(value: &T) -> Result<Transaction, ReconstructError> {
    let mut json = serde_json::to_value(value).map_err(ReconstructError::Bridge)?;
    strip_nulls(&mut json);
    let raw: RawTransaction = serde_json::from_value(json).map_err(ReconstructError::Bridge)?;
    raw.to_transaction().map_err(ReconstructError::Transaction)
}

/// Recursively drops object members whose value is JSON `null`.
fn strip_nulls(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.retain(|_, v| !v.is_null());
            for v in map.values_mut() {
                strip_nulls(v);
            }
        }
        Value::Array(items) => items.iter_mut().for_each(strip_nulls),
        _ => {}
    }
}
