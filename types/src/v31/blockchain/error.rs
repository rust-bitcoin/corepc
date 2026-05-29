// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::hex;

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `GetMempoolCluster` type into the model type.
#[derive(Debug)]
pub enum GetMempoolClusterError {
    /// Conversion of a transaction id to a `Txid` failed.
    Txid(hex::HexToArrayError),
    /// Conversion of a chunk fee to an `Amount` failed.
    ChunkFee(ParseAmountError),
}

impl fmt::Display for GetMempoolClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Txid(ref e) => write_err!(f, "conversion of `txid` field failed"; e),
            Self::ChunkFee(ref e) => write_err!(f, "conversion of `chunkfee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetMempoolClusterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Txid(ref e) => Some(e),
            Self::ChunkFee(ref e) => Some(e),
        }
    }
}

/// Error when converting a `MempoolEntryFees` type into the model type.
#[derive(Debug)]
pub enum MempoolEntryFeesError {
    /// Conversion of the `base` field failed.
    Base(ParseAmountError),
    /// Conversion of the `modified` field failed.
    Modified(ParseAmountError),
    /// Conversion of the `ancestor` field failed.
    Ancestor(ParseAmountError),
    /// Conversion of the `descendant` field failed.
    Descendant(ParseAmountError),
    /// Conversion of the `chunk` field failed.
    Chunk(ParseAmountError),
}

impl fmt::Display for MempoolEntryFeesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Base(ref e) => write_err!(f, "conversion of the `base` field failed"; e),
            Self::Modified(ref e) => write_err!(f, "conversion of the `modified` field failed"; e),
            Self::Ancestor(ref e) => write_err!(f, "conversion of the `ancestor` field failed"; e),
            Self::Descendant(ref e) =>
                write_err!(f, "conversion of the `descendant` field failed"; e),
            Self::Chunk(ref e) => write_err!(f, "conversion of the `chunk` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MempoolEntryFeesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Base(ref e) => Some(e),
            Self::Modified(ref e) => Some(e),
            Self::Ancestor(ref e) => Some(e),
            Self::Descendant(ref e) => Some(e),
            Self::Chunk(ref e) => Some(e),
        }
    }
}

/// Error when converting a `MempoolEntry` type into the model type.
#[derive(Debug)]
pub enum MempoolEntryError {
    /// Conversion of a numeric type to an expected type failed.
    Numeric(NumericError),
    /// Conversion of the `wtxid` field failed.
    Wtxid(hex::HexToArrayError),
    /// Conversion of the `fees` field failed.
    Fees(MempoolEntryFeesError),
    /// Conversion of the `depends` field failed.
    Depends(hex::HexToArrayError),
    /// Conversion of the `spentby` field failed.
    SpentBy(hex::HexToArrayError),
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
            Self::SpentBy(ref e) => write_err!(f, "conversion of the `spentby` field failed"; e),
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

/// Error when converting a map of `MempoolEntry`s into the model type.
#[derive(Debug)]
pub enum MapMempoolEntryError {
    /// Conversion of a `txid` failed.
    Txid(hex::HexToArrayError),
    /// Conversion of a `MempoolEntry` failed.
    MempoolEntry(MempoolEntryError),
}

impl fmt::Display for MapMempoolEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Txid(ref e) => write_err!(f, "conversion of a `txid` failed"; e),
            Self::MempoolEntry(ref e) => write_err!(f, "conversion of a `MempoolEntry` failed"; e),
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

/// Error when converting a `GetMempoolFeerateDiagram` type into the model type.
#[derive(Debug)]
pub enum GetMempoolFeerateDiagramError {
    /// Conversion of a numeric type to an expected type failed.
    Numeric(NumericError),
    /// Conversion of a `fee` field failed.
    Fee(ParseAmountError),
}

impl From<NumericError> for GetMempoolFeerateDiagramError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

impl fmt::Display for GetMempoolFeerateDiagramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetMempoolFeerateDiagramError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Fee(ref e) => Some(e),
        }
    }
}
