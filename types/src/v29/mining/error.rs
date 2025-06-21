// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::error::UnprefixedHexError;
use bitcoin::{consensus, hex};

use crate::error::write_err;
use crate::NumericError;

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

/// Error when converting a `GetMiningInfo` type into the model type.
#[derive(Debug)]
pub enum GetMiningInfoError {
    /// Conversion of the `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of the `target` field failed.
    Target(UnprefixedHexError),
    /// Conversion of one of the items in field `next` failed.
    Next(NextBlockInfoError),
}

impl fmt::Display for GetMiningInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetMiningInfoError as E;

        match *self {
            E::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            E::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
            E::Next(ref e) => {
                write_err!(f, "conversion of one of the items in field `next` failed"; e)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetMiningInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetMiningInfoError as E;

        match *self {
            E::Bits(ref e) => Some(e),
            E::Target(ref e) => Some(e),
            E::Next(ref e) => Some(e),
        }
    }
}

/// Error when converting a `NextBlockInfo` type into the model type.
#[derive(Debug)]
pub enum NextBlockInfoError {
    /// Conversion of the `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of the `target` field failed.
    Target(UnprefixedHexError),
}

impl fmt::Display for NextBlockInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NextBlockInfoError as E;

        match *self {
            E::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            E::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NextBlockInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use NextBlockInfoError as E;

        match *self {
            E::Bits(ref e) => Some(e),
            E::Target(ref e) => Some(e),
        }
    }
}
