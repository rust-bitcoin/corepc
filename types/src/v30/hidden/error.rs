// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::hex;

use crate::error::write_err;

/// Error when converting a `GetOrphanTxsVerboseOneEntry` type into the model type.
#[derive(Debug)]
pub enum GetOrphanTxsVerboseOneEntryError {
    /// Conversion of the transaction `txid` field failed.
    Txid(hex::DecodeFixedLengthBytesError),
    /// Conversion of the transaction `wtxid` field failed.
    Wtxid(hex::DecodeFixedLengthBytesError),
}

impl fmt::Display for GetOrphanTxsVerboseOneEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            Self::Wtxid(ref e) => write_err!(f, "conversion of the `wtxid` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetOrphanTxsVerboseOneEntryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Txid(ref e) => Some(e),
            Self::Wtxid(ref e) => Some(e),
        }
    }
}

/// Error when converting a `GetOrphanTxsVerboseTwoEntry` type into the model type.
#[derive(Debug)]
pub enum GetOrphanTxsVerboseTwoEntryError {
    /// Conversion of the transaction `txid` field failed.
    Txid(hex::DecodeFixedLengthBytesError),
    /// Conversion of the transaction `wtxid` field failed.
    Wtxid(hex::DecodeFixedLengthBytesError),
    /// Conversion of hex data to bytes failed.
    Hex(hex::DecodeVariableLengthBytesError),
    /// Consensus decoding of `hex` to transaction failed.
    Consensus(bitcoin::primitives::transaction::TransactionDecoderError),
}

impl fmt::Display for GetOrphanTxsVerboseTwoEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            Self::Wtxid(ref e) => write_err!(f, "conversion of the `wtxid` field failed"; e),
            Self::Hex(ref e) => write_err!(f, "conversion of hex data to bytes failed"; e),
            Self::Consensus(ref e) =>
                write_err!(f, "consensus decoding of `hex` to transaction failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetOrphanTxsVerboseTwoEntryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Txid(ref e) => Some(e),
            Self::Wtxid(ref e) => Some(e),
            Self::Hex(ref e) => Some(e),
            Self::Consensus(ref e) => Some(e),
        }
    }
}
