// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::parse_int::UnprefixedHexError;

use crate::error::write_err;

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
        match *self {
            Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Self::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
            Self::Next(ref e) =>
                write_err!(f, "conversion of one of the items in field `next` failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetMiningInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Bits(ref e) => Some(e),
            Self::Target(ref e) => Some(e),
            Self::Next(ref e) => Some(e),
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
        match *self {
            Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Self::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NextBlockInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Bits(ref e) => Some(e),
            Self::Target(ref e) => Some(e),
        }
    }
}
