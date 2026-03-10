// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::hex::DecodeVariableLengthBytesError;
use bitcoin::parse_int::UnprefixedHexError;
use bitcoin::{consensus, hex};

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `GetBlockTemplate` type into the model type.
#[derive(Debug)]
pub enum GetBlockTemplateError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `previous_block_hash` field failed.
    PreviousBlockHash(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `transactions` field failed.
    Transactions(BlockTemplateTransactionError),
    /// Conversion of the `target` field failed.
    Target(DecodeVariableLengthBytesError),
    /// Conversion of the `bits` field failed.
    Bits(UnprefixedHexError),
}

impl fmt::Display for GetBlockTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::PreviousBlockHash(ref e) =>
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e),
            Self::Transactions(ref e) =>
                write_err!(f, "conversion of the `transactions` field failed"; e),
            Self::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
            Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockTemplateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::PreviousBlockHash(ref e) => Some(e),
            Self::Transactions(ref e) => Some(e),
            Self::Target(ref e) => Some(e),
            Self::Bits(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockTemplateError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `BlockTemplateTransaction` type into the model type.
#[derive(Debug)]
pub enum BlockTemplateTransactionError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `data` field failed.
    Data(consensus::encode::FromHexError),
    /// Conversion of the `txid` field failed.
    Txid(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `hash` field to `wtxid` failed.
    Hash(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
}

impl fmt::Display for BlockTemplateTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Data(ref e) => write_err!(f, "conversion of the `data` field failed"; e),
            Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            Self::Hash(ref e) =>
                write_err!(f, "conversion of the `hash` field to `wtxid` failed"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BlockTemplateTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Data(ref e) => Some(e),
            Self::Txid(ref e) => Some(e),
            Self::Hash(ref e) => Some(e),
            Self::Fee(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for BlockTemplateTransactionError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
