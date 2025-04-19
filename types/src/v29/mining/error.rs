// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::error::UnprefixedHexError;
use bitcoin::hex::HexToBytesError;
use bitcoin::{consensus, hex};

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `GetBlockTemplate` type into the model type.
#[derive(Debug)]
pub enum GetBlockTemplateError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of the `transactions` field failed.
    Transactions(BlockTemplateTransactionError),
    /// Conversion of the `target` field failed.
    Target(HexToBytesError),
    /// Conversion of the `bits` field failed.
    Bits(UnprefixedHexError),
}

impl fmt::Display for GetBlockTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetBlockTemplateError as E;

        match *self {
            E::Numeric(ref e) => write_err!(f, "numeric"; e),
            E::PreviousBlockHash(ref e) => {
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e)
            }
            E::Transactions(ref e) => {
                write_err!(f, "conversion of the `transactions` field failed"; e)
            }
            E::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
            E::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockTemplateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetBlockTemplateError as E;

        match *self {
            E::Numeric(ref e) => Some(e),
            E::PreviousBlockHash(ref e) => Some(e),
            E::Transactions(ref e) => Some(e),
            E::Target(ref e) => Some(e),
            E::Bits(ref e) => Some(e),
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
    Txid(hex::HexToArrayError),
    /// Conversion of the `hash` field failed.
    Hash(hex::HexToArrayError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
}

impl fmt::Display for BlockTemplateTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BlockTemplateTransactionError as E;

        match *self {
            E::Numeric(ref e) => write_err!(f, "numeric"; e),
            E::Data(ref e) => write_err!(f, "conversion of the `data` field failed"; e),
            E::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            E::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            E::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BlockTemplateTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use BlockTemplateTransactionError as E;

        match *self {
            E::Numeric(ref e) => Some(e),
            E::Data(ref e) => Some(e),
            E::Txid(ref e) => Some(e),
            E::Hash(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for BlockTemplateTransactionError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
