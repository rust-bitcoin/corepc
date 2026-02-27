// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::{address, bip32, hex, key, witness_program, witness_version};

use super::GetAddressInfoEmbeddedError;
use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `GetAddressInfo` type into the model type.
#[derive(Debug)]
pub enum GetAddressInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `script_pubkey` field failed.
    ScriptPubKey(hex::HexToBytesError),
    /// The `witness_version` field's value was too big for a u8.
    WitnessVersionValue(i64),
    /// Conversion of the `witness_version` field failed.
    WitnessVersion(witness_version::TryFromError),
    /// Conversion of the `witness_program` field hex string to bytes failed.
    WitnessProgramBytes(hex::HexToBytesError),
    /// Conversion of the `witness_program` field failed.
    WitnessProgram(witness_program::Error),
    /// Conversion of the `hex` field failed.
    Hex(hex::HexToBytesError),
    /// Conversion of the `pubkeys` field failed.
    PubKeys(key::ParsePublicKeyError),
    /// Conversion of the `pubkey` field failed.
    PubKey(key::ParsePublicKeyError),
    /// Conversion of the `embedded` field failed.
    Embedded(GetAddressInfoEmbeddedError),
    /// Conversion of the `hd_key_path` field failed.
    HdKeyPath(bip32::Error),
    /// Conversion of the `hd_seed_id` field failed.
    HdSeedId(hex::HexToArrayError),
    /// Conversion of the `hd_master_fingerprint` field failed.
    HdMasterFingerprint(hex::HexToArrayError),
}

impl fmt::Display for GetAddressInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Self::ScriptPubKey(ref e) =>
                write_err!(f, "conversion of the `script_pubkey` field failed"; e),
            Self::WitnessVersionValue(v) => write!(f, "invalid witness version number: {}", v),
            Self::WitnessVersion(ref e) =>
                write_err!(f, "conversion of the `witness_version` field failed"; e),
            Self::WitnessProgramBytes(ref e) =>
                write_err!(f, "conversion of the `witness_program` field hex string to bytes failed"; e),
            Self::WitnessProgram(ref e) =>
                write_err!(f, "conversion of the `witness_program` field failed"; e),
            Self::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            Self::PubKeys(ref e) => write_err!(f, "conversion of the `pubkeys` field failed"; e),
            Self::PubKey(ref e) => write_err!(f, "conversion of the `pubkey` failed"; e),
            Self::Embedded(ref e) => write_err!(f, "conversion of the `embedded` field failed"; e),
            Self::HdKeyPath(ref e) =>
                write_err!(f, "conversion of the `hd_key_path` field failed"; e),
            Self::HdSeedId(ref e) =>
                write_err!(f, "conversion of the `hd_seed_id` field failed"; e),
            Self::HdMasterFingerprint(ref e) =>
                write_err!(f, "conversion of the `hd_master_fingerprint` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetAddressInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Address(ref e) => Some(e),
            Self::ScriptPubKey(ref e) => Some(e),
            Self::WitnessVersionValue(_) => None,
            Self::WitnessVersion(ref e) => Some(e),
            Self::WitnessProgramBytes(ref e) => Some(e),
            Self::WitnessProgram(ref e) => Some(e),
            Self::Hex(ref e) => Some(e),
            Self::PubKeys(ref e) => Some(e),
            Self::PubKey(ref e) => Some(e),
            Self::Embedded(ref e) => Some(e),
            Self::HdKeyPath(ref e) => Some(e),
            Self::HdSeedId(ref e) => Some(e),
            Self::HdMasterFingerprint(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetAddressInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `ListReceivedByLabel` type into the model type.
#[derive(Debug)]
pub enum ListReceivedByLabelError {
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
}

impl fmt::Display for ListReceivedByLabelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ListReceivedByLabelError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Amount(ref e) => Some(e),
            Self::Numeric(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for ListReceivedByLabelError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
