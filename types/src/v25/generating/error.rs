// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::consensus::encode;
use bitcoin::hex;

use crate::error::write_err;

/// Error when converting a `GenerateBlock` type into the model type.
#[derive(Debug)]
pub enum GenerateBlockError {
    /// Conversion of the `hash` field failed.
    Hash(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `hex` field failed.
    Hex(encode::FromHexError),
}

impl fmt::Display for GenerateBlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Self::Hex(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GenerateBlockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Hash(ref e) => Some(e),
            Self::Hex(ref e) => Some(e),
        }
    }
}
