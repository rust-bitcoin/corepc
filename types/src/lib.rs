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

mod error;
mod psbt;

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

// JSON types that model _all_ `bitcoind` versions.
pub mod model;

use core::fmt;

use bitcoin::address::{self, Address, NetworkUnchecked};
use bitcoin::amount::ParseAmountError;
use bitcoin::hex::{self, FromHex as _};
use bitcoin::{Amount, FeeRate, ScriptBuf, Witness};
use serde::{Deserialize, Serialize};

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
    let sats_per_kb = Amount::from_btc(btc_per_kb)?;
    let sats_per_byte = sats_per_kb.to_sat() / 1000;

    // Virtual bytes equal bytes before segwit.
    let rate = FeeRate::from_sat_per_vb(sats_per_byte);

    Ok(rate)
}

// TODO: Remove this function if a new `Witness` constructor gets added.
// https://github.com/rust-bitcoin/rust-bitcoin/issues/4350
fn witness_from_hex_slice<T: AsRef<str>>(witness: &[T]) -> Result<Witness, hex::HexToBytesError> {
    let bytes: Vec<Vec<u8>> =
        witness.iter().map(|hex| Vec::from_hex(hex.as_ref())).collect::<Result<_, _>>()?;
    Ok(Witness::from_slice(&bytes))
}

/// Gets the compact size encoded value from `slice` and moves slice past the encoding.
///
/// Caller to guarantee that the encoding is well formed. Well formed is defined as:
///
/// * Being at least long enough.
/// * Containing a minimal encoding.
///
/// # Panics
///
/// * Panics in release mode if the `slice` does not contain a valid minimal compact size encoding.
/// * Panics in debug mode if the encoding is not minimal (referred to as "non-canonical" in Core).
// This is copied from the `bitcoin-internals::compact_size` module.
pub fn compact_size_decode(slice: &mut &[u8]) -> u64 {
    if slice.is_empty() {
        panic!("tried to decode an empty slice");
    }

    match slice[0] {
        0xFF => {
            const SIZE: usize = 9;
            if slice.len() < SIZE {
                panic!("slice too short, expected at least 9 bytes");
            };

            let mut bytes = [0_u8; SIZE - 1];
            bytes.copy_from_slice(&slice[1..SIZE]);

            let v = u64::from_le_bytes(bytes);
            debug_assert!(v > u32::MAX.into(), "non-minimal encoding of a u64");
            *slice = &slice[SIZE..];
            v
        }
        0xFE => {
            const SIZE: usize = 5;
            if slice.len() < SIZE {
                panic!("slice too short, expected at least 5 bytes");
            };

            let mut bytes = [0_u8; SIZE - 1];
            bytes.copy_from_slice(&slice[1..SIZE]);

            let v = u32::from_le_bytes(bytes);
            debug_assert!(v > u16::MAX.into(), "non-minimal encoding of a u32");
            *slice = &slice[SIZE..];
            u64::from(v)
        }
        0xFD => {
            const SIZE: usize = 3;
            if slice.len() < SIZE {
                panic!("slice too short, expected at least 3 bytes");
            };

            let mut bytes = [0_u8; SIZE - 1];
            bytes.copy_from_slice(&slice[1..SIZE]);

            let v = u16::from_le_bytes(bytes);
            debug_assert!(v >= 0xFD, "non-minimal encoding of a u16");
            *slice = &slice[SIZE..];
            u64::from(v)
        }
        n => {
            *slice = &slice[1..];
            u64::from(n)
        }
    }
}

/// Data returned by Core for a script pubkey.
///
/// This is used by methods in the blockchain section and in the raw transaction section (i.e raw
/// transaction and psbt methods). The shape changed in Core v22 but the new shape is fully
/// backwards compatible so we only provide it not a v0.17 specific type. The `mtype::ScriptPubkey`
/// mirrors this design (but with concrete `rust-bitcoin` types).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScriptPubkey {
    /// Script assembly.
    pub asm: String,
    /// Inferred descriptor for the output. v23 and later only.
    #[serde(rename = "desc")]
    pub descriptor: Option<String>,
    /// Script hex.
    pub hex: String,
    /// Number of required signatures - deprecated in Core v22.
    ///
    /// Only returned before in versions prior to 22 or for version 22 onwards if
    /// config option `-deprecatedrpc=addresses` is passed.
    #[serde(rename = "reqSigs")]
    pub required_signatures: Option<i64>,
    /// The type, eg pubkeyhash.
    #[serde(rename = "type")]
    pub type_: String,
    /// Bitcoin address (only if a well-defined address exists).
    pub address: Option<String>,
    /// Array of bitcoin addresses - deprecated in Core v22.
    ///
    /// Only returned before in versions prior to 22 or for version 22 onwards if
    /// config option `-deprecatedrpc=addresses` is passed.
    pub addresses: Option<Vec<String>>,
}

impl ScriptPubkey {
    fn script_buf(&self) -> Result<ScriptBuf, hex::HexToBytesError> {
        ScriptBuf::from_hex(&self.hex)
    }

    fn address(&self) -> Option<Result<Address<NetworkUnchecked>, address::ParseError>> {
        self.address.as_ref().map(|addr| addr.parse::<Address<_>>())
    }
}

/// Data returned by Core for a script signature.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "serde-deny-unknown-fields", serde(deny_unknown_fields))]
pub struct ScriptSig {
    /// Assembly representation of the script.
    pub asm: String,
    /// Hex representation of the script.
    pub hex: String,
}

impl ScriptSig {
    pub fn script_buf(&self) -> Result<ScriptBuf, hex::HexToBytesError> {
        ScriptBuf::from_hex(&self.hex)
    }
}
