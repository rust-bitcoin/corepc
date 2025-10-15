// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::hex;

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `WaitForBlock` type into the model type.
#[derive(Debug)]
pub enum WaitForBlockError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `hash` field failed.
    Hash(hex::HexToArrayError),
}

impl fmt::Display for WaitForBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WaitForBlockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Hash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for WaitForBlockError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `WaitForBlockHeight` type into the model type.
#[derive(Debug)]
pub enum WaitForBlockHeightError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `hash` field failed.
    Hash(hex::HexToArrayError),
}

impl fmt::Display for WaitForBlockHeightError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WaitForBlockHeightError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Hash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for WaitForBlockHeightError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `WaitForNewBlock` type into the model type.
#[derive(Debug)]
pub enum WaitForNewBlockError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `hash` field failed.
    Hash(hex::HexToArrayError),
}

impl fmt::Display for WaitForNewBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WaitForNewBlockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Hash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for WaitForNewBlockError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
