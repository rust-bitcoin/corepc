// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;
use bitcoin::error::UnprefixedHexError;
use bitcoin::hex;

use crate::error::write_err;
use crate::v17::GetRawTransactionVerboseError;
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

/// Error when converting a `GetTxSpendingPrevout` type into the model type.
#[derive(Debug)]
pub enum GetTxSpendingPrevoutError {
    /// Conversion of the `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Conversion of the `spendingtxid` field failed.
    SpendingTxid(hex::HexToArrayError),
    /// Conversion of the `spendingtx` field failed.
    SpendingTx(encode::FromHexError),
    /// Conversion of the `blockhash` field failed.
    BlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetTxSpendingPrevoutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            Self::SpendingTxid(ref e) =>
                write_err!(f, "conversion of the `spendingtxid` field failed"; e),
            Self::SpendingTx(ref e) =>
                write_err!(f, "conversion of the `spendingtx` field failed"; e),
            Self::BlockHash(ref e) =>
                write_err!(f, "conversion of the `blockhash` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetTxSpendingPrevoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Txid(ref e) => Some(e),
            Self::SpendingTxid(ref e) => Some(e),
            Self::SpendingTx(ref e) => Some(e),
            Self::BlockHash(ref e) => Some(e),
        }
    }
}

/// Error when converting a `CoinbaseTransaction` type into the model type.
#[derive(Debug)]
pub enum CoinbaseTransactionError {
    /// Conversion of the `coinbase` field failed.
    Coinbase(hex::HexToBytesError),
    /// Conversion of the `witness` field failed.
    Witness(hex::HexToBytesError),
}

impl fmt::Display for CoinbaseTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Coinbase(ref e) => write_err!(f, "conversion of the `coinbase` field failed"; e),
            Self::Witness(ref e) => write_err!(f, "conversion of the `witness` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CoinbaseTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Coinbase(ref e) => Some(e),
            Self::Witness(ref e) => Some(e),
        }
    }
}

/// Error when converting a `GetBlockVerboseOne` type into the model type.
#[derive(Debug)]
pub enum GetBlockVerboseOneError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `hash` field failed.
    Hash(hex::HexToArrayError),
    /// Conversion of the transaction `merkle_root` field failed.
    MerkleRoot(hex::HexToArrayError),
    /// Conversion of the transaction `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of the `target` field failed.
    Target(UnprefixedHexError),
    /// Conversion of the transaction `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of the transaction `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of the transaction `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
    /// Conversion of the `coinbase_tx` field failed.
    CoinbaseTx(CoinbaseTransactionError),
}

impl fmt::Display for GetBlockVerboseOneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Self::MerkleRoot(ref e) =>
                write_err!(f, "conversion of the `merkle_root` field failed"; e),
            Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Self::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
            Self::ChainWork(ref e) =>
                write_err!(f, "conversion of the `chain_work` field failed"; e),
            Self::PreviousBlockHash(ref e) =>
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e),
            Self::NextBlockHash(ref e) =>
                write_err!(f, "conversion of the `next_block_hash` field failed"; e),
            Self::CoinbaseTx(ref e) =>
                write_err!(f, "conversion of the `coinbase_tx` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockVerboseOneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Hash(ref e) => Some(e),
            Self::MerkleRoot(ref e) => Some(e),
            Self::Bits(ref e) => Some(e),
            Self::Target(ref e) => Some(e),
            Self::ChainWork(ref e) => Some(e),
            Self::PreviousBlockHash(ref e) => Some(e),
            Self::NextBlockHash(ref e) => Some(e),
            Self::CoinbaseTx(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockVerboseOneError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetBlockVerboseTwo` type into the model type.
#[derive(Debug)]
pub enum GetBlockVerboseTwoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `hash` field failed.
    Hash(hex::HexToArrayError),
    /// Conversion of the transaction `merkle_root` field failed.
    MerkleRoot(hex::HexToArrayError),
    /// Conversion of the transaction `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of the `target` field failed.
    Target(UnprefixedHexError),
    /// Conversion of the transaction `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of the transaction `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of the transaction `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
    /// Conversion of a transaction entry failed.
    Transaction(GetRawTransactionVerboseError),
    /// Conversion of the transaction `fee` field failed.
    Fee(ParseAmountError),
    /// Conversion of the `coinbase_tx` field failed.
    CoinbaseTx(CoinbaseTransactionError),
}

impl fmt::Display for GetBlockVerboseTwoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Self::MerkleRoot(ref e) =>
                write_err!(f, "conversion of the `merkle_root` field failed"; e),
            Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Self::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
            Self::ChainWork(ref e) =>
                write_err!(f, "conversion of the `chain_work` field failed"; e),
            Self::PreviousBlockHash(ref e) =>
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e),
            Self::NextBlockHash(ref e) =>
                write_err!(f, "conversion of the `next_block_hash` field failed"; e),
            Self::Transaction(ref e) =>
                write_err!(f, "conversion of a transaction entry failed"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
            Self::CoinbaseTx(ref e) =>
                write_err!(f, "conversion of the `coinbase_tx` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockVerboseTwoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Hash(ref e) => Some(e),
            Self::MerkleRoot(ref e) => Some(e),
            Self::Bits(ref e) => Some(e),
            Self::Target(ref e) => Some(e),
            Self::ChainWork(ref e) => Some(e),
            Self::PreviousBlockHash(ref e) => Some(e),
            Self::NextBlockHash(ref e) => Some(e),
            Self::Transaction(ref e) => Some(e),
            Self::Fee(ref e) => Some(e),
            Self::CoinbaseTx(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockVerboseTwoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetBlockVerboseThree` type into the model type.
#[derive(Debug)]
pub enum GetBlockVerboseThreeError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `hash` field failed.
    Hash(hex::HexToArrayError),
    /// Conversion of the transaction `merkle_root` field failed.
    MerkleRoot(hex::HexToArrayError),
    /// Conversion of the transaction `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of the `target` field failed.
    Target(UnprefixedHexError),
    /// Conversion of the transaction `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of the transaction `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of the transaction `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
    /// Conversion of a transaction entry failed.
    Transaction(crate::v29::GetBlockVerboseThreeError),
    /// Conversion of the transaction `fee` field failed.
    Fee(ParseAmountError),
    /// Conversion of the `coinbase_tx` field failed.
    CoinbaseTx(CoinbaseTransactionError),
}

impl fmt::Display for GetBlockVerboseThreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Self::MerkleRoot(ref e) =>
                write_err!(f, "conversion of the `merkle_root` field failed"; e),
            Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Self::Target(ref e) => write_err!(f, "conversion of the `target` field failed"; e),
            Self::ChainWork(ref e) =>
                write_err!(f, "conversion of the `chain_work` field failed"; e),
            Self::PreviousBlockHash(ref e) =>
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e),
            Self::NextBlockHash(ref e) =>
                write_err!(f, "conversion of the `next_block_hash` field failed"; e),
            Self::Transaction(ref e) =>
                write_err!(f, "conversion of a transaction entry failed"; e),
            Self::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
            Self::CoinbaseTx(ref e) =>
                write_err!(f, "conversion of the `coinbase_tx` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockVerboseThreeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Hash(ref e) => Some(e),
            Self::MerkleRoot(ref e) => Some(e),
            Self::Bits(ref e) => Some(e),
            Self::Target(ref e) => Some(e),
            Self::ChainWork(ref e) => Some(e),
            Self::PreviousBlockHash(ref e) => Some(e),
            Self::NextBlockHash(ref e) => Some(e),
            Self::Transaction(ref e) => Some(e),
            Self::Fee(ref e) => Some(e),
            Self::CoinbaseTx(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockVerboseThreeError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
