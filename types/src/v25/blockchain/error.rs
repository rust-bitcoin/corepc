// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::hex;

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `ScanBlocksStart` type into the model type.
#[derive(Debug)]
pub enum ScanBlocksStartError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `relevant_blocks` field failed.
    RelevantBlocks(hex::DecodeFixedLengthBytesError),
}

impl fmt::Display for ScanBlocksStartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::RelevantBlocks(ref e) =>
                write_err!(f, "conversion of the `relevant_blocks` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ScanBlocksStartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::RelevantBlocks(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for ScanBlocksStartError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
