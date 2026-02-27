// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::blockdata::script::{witness_program, witness_version};
use bitcoin::{address, hex};

/// Error when converting a `CreateMultisig` type into the model type.
#[derive(Debug)]
pub enum CreateMultisigError {
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `redeem_script` field failed.
    RedeemScript(hex::HexToBytesError),
}

impl fmt::Display for CreateMultisigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Address(ref e) => write!(f, "conversion of the `address` field failed: {}", e),
            Self::RedeemScript(ref e) =>
                write!(f, "conversion of the `redeem_script` field failed: {}", e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CreateMultisigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Address(ref e) => Some(e),
            Self::RedeemScript(ref e) => Some(e),
        }
    }
}

/// Error when converting a `ValidateAddress` type into the model type.
#[derive(Debug)]
pub enum ValidateAddressError {
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
}

impl fmt::Display for ValidateAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Address(ref e) => write!(f, "conversion of the `address` field failed: {}", e),
            Self::ScriptPubKey(ref e) =>
                write!(f, "conversion of the `script_pubkey` field failed: {}", e),
            Self::WitnessVersionValue(v) => write!(f, "invalid witness version number: {}", v),
            Self::WitnessVersion(ref e) =>
                write!(f, "conversion of the `witness_version` field failed: {}", e),
            Self::WitnessProgramBytes(ref e) => write!(
                f,
                "conversion of the `witness_program` field hex string to bytes failed: {}",
                e
            ),
            Self::WitnessProgram(ref e) =>
                write!(f, "conversion of the `witness_program` field failed: {}", e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ValidateAddressError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Address(ref e) => Some(e),
            Self::ScriptPubKey(ref e) => Some(e),
            Self::WitnessVersionValue(_) => None,
            Self::WitnessVersion(ref e) => Some(e),
            Self::WitnessProgramBytes(ref e) => Some(e),
            Self::WitnessProgram(ref e) => Some(e),
        }
    }
}
