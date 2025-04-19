// SPDX-License-Identifier: CC0-1.0

//! Types returned by the JSON-RPC API of Bitcoin Core.
//!
//! Each type has rustdocs copied from Core, bugs and all. Additional docs were only added if things
//! really didn't make sense. Only required arguments are documented. To see what optional arguments
//! are available run `bitcoin-cli help <method>` against the version of Core you are interested in.

/// Re-export the `rust-bitcoin` crate.
pub extern crate bitcoin;

extern crate alloc;

// TODO: Consider updating https://en.bitcoin.it/wiki/API_reference_%28JSON-RPC%29 when this is complete.

// JSON types, for each specific version of `bitcoind`.
pub mod v17;
pub mod v18;
pub mod v19;
pub mod v20;
pub mod v21;
pub mod v22;
pub mod v23;
pub mod v24;
pub mod v25;
pub mod v26;
pub mod v27;
pub mod v28;
pub mod v29;

mod error;
// JSON types that model _all_ `bitcoind` versions.
pub mod model;

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::{Amount, FeeRate};

/// Converts an `i64` numeric type to a `u32`.
///
/// The Bitcoin Core JSONRPC API has fields marked as 'numeric'. It is not obvious what Rust
/// type these fields should be.
///
/// We want the version specific JSON types to just work (TM).
///
/// 1. We use an `i64` because its the biggest signed integer on "common" machines.
/// 2. We use a signed integer because Core sometimes returns -1.
///
/// (2) was discovered in the wild but is hard to test for.
pub fn to_u32(value: i64, field: &str) -> Result<u32, NumericError> {
    if value.is_negative() {
        return Err(NumericError::Negative { value, field: field.to_owned() });
    }
    u32::try_from(value).map_err(|_| NumericError::Overflow { value, field: field.to_owned() })
}

/// Error converting an `i64` to a `u32`.
///
/// If we expect a numeric value to sanely fit inside a `u32` we use that type in the `model`
/// module, this requires converting the `i64` returned by the JSONRPC API into a `u32`, if our
/// expectations are not met this error will be encountered.
#[derive(Debug)]
pub enum NumericError {
    /// Expected an unsigned numeric value however the value was negative.
    Negative { field: String, value: i64 },
    /// A value larger than `u32::MAX` was unexpectedly encountered.
    Overflow { field: String, value: i64 },
}

impl fmt::Display for NumericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NumericError::*;

        match *self {
            Negative{ ref field, value } => write!(f, "expected an unsigned numeric value however the value was negative (field name: {} value: {})", field, value),
            Overflow { ref field, value } => write!(f, "a value larger than `u32::MAX` was unexpectedly encountered (field name: {} Value: {})", field, value),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NumericError {}

/// Converts `fee_rate` in BTC/kB to `FeeRate`.
fn btc_per_kb(btc_per_kb: f64) -> Result<Option<FeeRate>, ParseAmountError> {
    let btc_per_byte = btc_per_kb / 1000_f64;
    let sats_per_byte = Amount::from_btc(btc_per_byte)?;

    // Virtual bytes equal bytes before segwit.
    let rate = FeeRate::from_sat_per_vb(sats_per_byte.to_sat());

    Ok(rate)
}
