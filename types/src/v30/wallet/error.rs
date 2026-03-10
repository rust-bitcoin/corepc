// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::hex;

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `GetWalletInfo` type into the model type.
#[derive(Debug)]
pub enum GetWalletInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `pay_tx_fee` field failed.
    PayTxFee(ParseAmountError),
    /// Conversion of the `last_processed_block` field failed.
    LastProcessedBlock(LastProcessedBlockError),
}

impl fmt::Display for GetWalletInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::PayTxFee(ref e) =>
                write_err!(f, "conversion of the `pay_tx_fee` field failed"; e),
            Self::LastProcessedBlock(ref e) =>
                write_err!(f, "conversion of the `last_processed_block` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetWalletInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::PayTxFee(ref e) => Some(e),
            Self::LastProcessedBlock(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetWalletInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `LastProcessedBlock` type into the model type.
#[derive(Debug)]
pub enum LastProcessedBlockError {
    /// Conversion of the `hash` field failed.
    Hash(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `height` field failed.
    Height(NumericError),
}

impl fmt::Display for LastProcessedBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Self::Height(ref e) => write_err!(f, "conversion of the `height` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LastProcessedBlockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Hash(ref e) => Some(e),
            Self::Height(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for LastProcessedBlockError {
    fn from(e: NumericError) -> Self { Self::Height(e) }
}
