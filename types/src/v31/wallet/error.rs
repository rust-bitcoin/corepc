// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use crate::error::write_err;
use crate::v30::LastProcessedBlockError;
use crate::NumericError;

/// Error when converting a `GetWalletInfo` type into the model type.
#[derive(Debug)]
pub enum GetWalletInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `last_processed_block` field failed.
    LastProcessedBlock(LastProcessedBlockError),
}

impl fmt::Display for GetWalletInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
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
            Self::LastProcessedBlock(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetWalletInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
