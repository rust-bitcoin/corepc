//  SPDX-License-Identifier: CC0-1.0
// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::{address, hex};

use crate::error::write_err;
use crate::v19::DecodeScriptSegwitError;
use crate::NumericError;

/// Error when converting a `DecodeScript` type into the model type.
#[derive(Debug)]
pub enum DecodeScriptError {
    /// Conversion of the transaction `hex` field failed.
    Hex(hex::HexToBytesError),
    /// Conversion of the transaction `address` field failed.
    Address(address::ParseError),
    /// Conversion of the transaction `addresses` field failed.
    Addresses(address::ParseError),
    /// Conversion of the transaction `p2sh` field failed.
    P2sh(address::ParseError),
    /// Conversion of the transaction `segwit` field failed.
    Segwit(DecodeScriptSegwitError),
}

impl fmt::Display for DecodeScriptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DecodeScriptError as E;
        match *self {
            E::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            E::Addresses(ref e) => write_err!(f, "conversion of the `addresses` field failed"; e),
            E::P2sh(ref e) => write_err!(f, "conversion of the `p2sh` field failed"; e),
            E::Segwit(ref e) => write_err!(f, "conversion of the `segwit` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodeScriptError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use DecodeScriptError as E;
        match *self {
            E::Hex(ref e) => Some(e),
            E::Address(ref e) => Some(e),
            E::Addresses(ref e) => Some(e),
            E::P2sh(ref e) => Some(e),
            E::Segwit(ref e) => Some(e),
        }
    }
}

/// Error when converting a `TestMempoolAccept` type into the model type.
#[derive(Debug)]
pub enum TestMempoolAcceptError {
    /// Conversion of one of the mempool acceptance results failed.
    MempoolAcceptance(MempoolAcceptanceError),
}

impl fmt::Display for TestMempoolAcceptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::MempoolAcceptance(ref e) =>
                write_err!(f, "conversion of one of the mempool acceptance results failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TestMempoolAcceptError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::MempoolAcceptance(ref e) => Some(e),
        }
    }
}

impl From<MempoolAcceptanceError> for TestMempoolAcceptError {
    fn from(e: MempoolAcceptanceError) -> Self { TestMempoolAcceptError::MempoolAcceptance(e) }
}

/// Error when converting a `MempoolAcceptance` type into the model type.
#[derive(Debug)]
pub enum MempoolAcceptanceError {
    /// Conversion of a numeric field failed.
    Numeric(NumericError),
    /// Conversion of the `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Conversion of the `wtxid` field failed.
    Wtxid(hex::HexToArrayError),
    /// Conversion of the `base` fee field failed.
    Base(ParseAmountError),
}

impl fmt::Display for MempoolAcceptanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "conversion of a numeric field failed"; e),
            Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            Self::Wtxid(ref e) => write_err!(f, "conversion of the `wtxid` field failed"; e),
            Self::Base(ref e) => write_err!(f, "conversion of the `base` fee field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MempoolAcceptanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Txid(ref e) => Some(e),
            Self::Wtxid(ref e) => Some(e),
            Self::Base(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for MempoolAcceptanceError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
