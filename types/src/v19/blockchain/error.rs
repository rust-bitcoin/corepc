// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::parse_int::UnprefixedHexError;
use bitcoin::{hex, network};

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `GetBlockchainInfo` type into the model type.
#[derive(Debug)]
pub enum GetBlockchainInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `chain` field failed.
    Chain(network::ParseNetworkError),
    /// Conversion of the `best_block_hash` field failed.
    BestBlockHash(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `chain_work` field failed.
    ChainWork(UnprefixedHexError),
}

impl fmt::Display for GetBlockchainInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Chain(ref e) => write_err!(f, "conversion of the `chain` field failed"; e),
            Self::BestBlockHash(ref e) => {
                write_err!(f, "conversion of the `best_block_hash` field failed"; e)
            }
            Self::ChainWork(ref e) =>
                write_err!(f, "conversion of the `chain_work` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockchainInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Chain(ref e) => Some(e),
            Self::BestBlockHash(ref e) => Some(e),
            Self::ChainWork(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockchainInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetBlockFilter` into the model type.
#[derive(Debug)]
pub enum GetBlockFilterError {
    /// Conversion of the `filter` field failed.
    Filter(hex::DecodeVariableLengthBytesError),
    /// Conversion of the `header` field failed.
    Header(hex::DecodeFixedLengthBytesError),
}

impl fmt::Display for GetBlockFilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Filter(ref e) => write_err!(f, "conversion of the `filter` field failed"; e),
            Self::Header(ref e) => write_err!(f, "conversion of the `header` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockFilterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Filter(ref e) => Some(e),
            Self::Header(ref e) => Some(e),
        }
    }
}

/// Error when converting a `MapMempoolEntry` into the model type.
#[derive(Debug)]
pub enum MapMempoolEntryError {
    /// Conversion of a `txid` failed.
    Txid(hex::DecodeFixedLengthBytesError),
    /// Conversion of a `MempoolEntry` failed.
    MempoolEntry(MempoolEntryError),
}

impl fmt::Display for MapMempoolEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Txid(ref e) => write_err!(f, "conversion of a `txid` failed"; e),
            Self::MempoolEntry(ref e) => write_err!(f, "conversion of an `MempoolEntry` failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MapMempoolEntryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Txid(ref e) => Some(e),
            Self::MempoolEntry(ref e) => Some(e),
        }
    }
}

/// Error when converting a `Mem` type into the model type.
#[derive(Debug)]
pub enum MempoolEntryError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `wtxid` field failed.
    Wtxid(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `MempoolEntryFees` type failed.
    Fees(MempoolEntryFeesError),
    /// Conversion of the `depends` field failed.
    Depends(hex::DecodeFixedLengthBytesError),
    /// Conversion of the `spent_by` field failed.
    SpentBy(hex::DecodeFixedLengthBytesError),
}

impl From<NumericError> for MempoolEntryError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

impl fmt::Display for MempoolEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Wtxid(ref e) => write_err!(f, "conversion of the `wtxid` field failed"; e),
            Self::Fees(ref e) => write_err!(f, "conversion of the `fees` field failed"; e),
            Self::Depends(ref e) => write_err!(f, "conversion of the `depends` field failed"; e),
            Self::SpentBy(ref e) => write_err!(f, "conversion of the `spent_by` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MempoolEntryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Wtxid(ref e) => Some(e),
            Self::Fees(ref e) => Some(e),
            Self::Depends(ref e) => Some(e),
            Self::SpentBy(ref e) => Some(e),
        }
    }
}

/// Error when converting a `MempoolEntryFeesError` type into the model type.
#[derive(Debug)]
pub enum MempoolEntryFeesError {
    /// Conversion of the `base` field failed.
    Base(ParseAmountError),
    /// Conversion of the `modified` field failed.
    Modified(ParseAmountError),
    /// Conversion of the `ancestor` field failed.
    MempoolEntry(ParseAmountError),
    /// Conversion of the `descendant` field failed.
    Descendant(ParseAmountError),
}

impl fmt::Display for MempoolEntryFeesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Base(ref e) => write_err!(f, "conversion of the `base` field failed"; e),
            Self::Modified(ref e) => write_err!(f, "conversion of the `modified` field failed"; e),
            Self::MempoolEntry(ref e) =>
                write_err!(f, "conversion of the `ancestor` field failed"; e),
            Self::Descendant(ref e) =>
                write_err!(f, "conversion of the `descendant` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MempoolEntryFeesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Base(ref e) => Some(e),
            Self::Modified(ref e) => Some(e),
            Self::MempoolEntry(ref e) => Some(e),
            Self::Descendant(ref e) => Some(e),
        }
    }
}
