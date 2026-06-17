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

/// Error rebuilding a `bitcoin::Psbt` from a generated decoded `decodepsbt` response.
#[derive(Debug)]
pub enum ReconstructPsbtError {
    /// Bridging the generated type through JSON into `RawPsbt` failed.
    Bridge(serde_json::Error),
    /// Assembling the `bitcoin::Psbt` from the decoded parts failed.
    Psbt(crate::psbt::RawPsbtError),
}

impl fmt::Display for ReconstructPsbtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bridge(e) => write!(f, "bridging the decoded psbt through JSON failed: {e}"),
            Self::Psbt(e) => write!(f, "assembling the psbt from decoded parts failed: {e}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ReconstructPsbtError {}

/// Rebuilds a `bitcoin::Psbt` from a generated decoded `decodepsbt` response type.
///
/// Bridges `value` through JSON into the shared [`crate::psbt::RawPsbt`] (dropping `null` members),
/// then assembles the PSBT with [`crate::psbt::RawPsbt::into_psbt`].
pub fn psbt<T: Serialize>(value: &T) -> Result<bitcoin::Psbt, ReconstructPsbtError> {
    let mut json = serde_json::to_value(value).map_err(ReconstructPsbtError::Bridge)?;
    strip_nulls(&mut json);
    let raw: crate::psbt::RawPsbt =
        serde_json::from_value(json).map_err(ReconstructPsbtError::Bridge)?;
    raw.into_psbt().map_err(ReconstructPsbtError::Psbt)
}

/// Re-types a flattened map of decoded fields (a `#[serde(flatten)]` "extra" bag) as a generated
/// raw response type `T`.
///
/// `getblock` verbosity 2 nests each transaction as a flat `getrawtransaction`-verbose object that
/// the generated raw type captures in an untyped `extra` map. This re-serialises that map and
/// deserialises it into `T` (dropping `null` members the curated types emit, which the generated
/// `deny_unknown_fields` types would otherwise reject), so the caller can run `T::into_model`.
pub fn from_flat_fields<T: serde::de::DeserializeOwned>(
    fields: &std::collections::BTreeMap<String, Value>,
) -> Result<T, serde_json::Error> {
    let mut json = serde_json::to_value(fields)?;
    strip_nulls(&mut json);
    serde_json::from_value(json)
}

/// Re-types a `getblock` verbosity-3 transaction as a generated verbose-tx type `T`.
///
/// Verbosity 3 pulls each input's `vin` entry out of the flattened tx body (to attach per-input
/// `prevout` data), so the leftover `extra` map lacks `vin` and carries a block-only `fee` member.
/// This rebuilds the `getrawtransaction`-verbose object: drop `fee`, re-insert `vin` (each input
/// minus its `prevout`), then deserialise into `T` so the caller can run `T::into_model`.
pub fn block_verbose3_tx<T, V>(
    extra: &std::collections::BTreeMap<String, Value>,
    vin: &[V],
) -> Result<T, serde_json::Error>
where
    T: serde::de::DeserializeOwned,
    V: Serialize,
{
    let vins = vin
        .iter()
        .map(|v| {
            let mut vv = serde_json::to_value(v)?;
            if let Value::Object(m) = &mut vv {
                m.remove("prevout");
            }
            Ok(vv)
        })
        .collect::<Result<Vec<_>, serde_json::Error>>()?;
    let mut obj = serde_json::to_value(extra)?;
    if let Value::Object(m) = &mut obj {
        m.remove("fee");
        m.insert("vin".to_owned(), Value::Array(vins));
    }
    strip_nulls(&mut obj);
    serde_json::from_value(obj)
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
