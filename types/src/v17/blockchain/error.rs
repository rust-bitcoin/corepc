// SPDX-License-Identifier: CC0-1.0

use core::fmt;

use bitcoin::amount::{self, ParseAmountError};
use bitcoin::consensus::encode;
use bitcoin::error::UnprefixedHexError;
use bitcoin::{address, hex, network};

use crate::error::write_err;
use crate::NumericError;

/// Error when converting a `GetBlockVerboseOne` type into the model type.
#[derive(Debug)]
pub enum GetBlockVerboseOneError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `hash` field failed.
    Hash(hex::HexToArrayError),
    /// Conversion of the transaction `hex` field failed.
    Tx(encode::FromHexError),
    /// Conversion of the transaction `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of the transaction `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of the transaction `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of the transaction `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetBlockVerboseOneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Self::Tx(ref e) => write_err!(f, "conversion of the `tx` field failed"; e),
            Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Self::ChainWork(ref e) =>
                write_err!(f, "conversion of the `chain_work` field failed"; e),
            Self::PreviousBlockHash(ref e) =>
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e),
            Self::NextBlockHash(ref e) =>
                write_err!(f, "conversion of the `next_block_hash` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockVerboseOneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Hash(ref e) => Some(e),
            Self::Tx(ref e) => Some(e),
            Self::Bits(ref e) => Some(e),
            Self::ChainWork(ref e) => Some(e),
            Self::PreviousBlockHash(ref e) => Some(e),
            Self::NextBlockHash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockVerboseOneError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetBlockchainInfo` type into the model type.
#[derive(Debug)]
pub enum GetBlockchainInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `chain` field failed.
    Chain(network::ParseNetworkError),
    /// Conversion of the `best_block_hash` field failed.
    BestBlockHash(hex::HexToArrayError),
    /// Conversion of the `chain_work` field failed.
    ChainWork(UnprefixedHexError),
}

impl fmt::Display for GetBlockchainInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Chain(ref e) => write_err!(f, "conversion of the `chain` field failed"; e),
            Self::BestBlockHash(ref e) =>
                write_err!(f, "conversion of the `best_block_hash` field failed"; e),
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

/// Error when converting a `GetBlockHeader` type into the model type.
#[derive(Debug)]
pub enum GetBlockHeaderError {
    /// Conversion of hex data to bytes failed.
    Hex(hex::HexToBytesError),
    /// Consensus decoding of bytes to header failed.
    Consensus(encode::Error),
}

impl fmt::Display for GetBlockHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Hex(ref e) => write_err!(f, "conversion of hex data to bytes failed"; e),
            Self::Consensus(ref e) =>
                write_err!(f, "consensus decoding of bytes to header failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockHeaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Hex(ref e) => Some(e),
            Self::Consensus(ref e) => Some(e),
        }
    }
}

/// Error when converting a `GetBlockHeader` type into the model type.
#[derive(Debug)]
pub enum GetBlockHeaderVerboseError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of `hash` field failed.
    Hash(hex::HexToArrayError),
    /// Conversion of `merkle_root` field failed.
    MerkleRoot(hex::HexToArrayError),
    /// Conversion of `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetBlockHeaderVerboseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Self::MerkleRoot(ref e) =>
                write_err!(f, "conversion of the `merkle_root` field failed"; e),
            Self::Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            Self::ChainWork(ref e) =>
                write_err!(f, "conversion of the `chain_work` field failed"; e),
            Self::PreviousBlockHash(ref e) =>
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e),
            Self::NextBlockHash(ref e) =>
                write_err!(f, "conversion of the `next_block_hash` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockHeaderVerboseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Hash(ref e) => Some(e),
            Self::MerkleRoot(ref e) => Some(e),
            Self::Bits(ref e) => Some(e),
            Self::ChainWork(ref e) => Some(e),
            Self::PreviousBlockHash(ref e) => Some(e),
            Self::NextBlockHash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockHeaderVerboseError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetBlockStats` type into the model type.
#[derive(Debug)]
pub enum GetBlockStatsError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `block_hash` field failed.
    BlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetBlockStatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::BlockHash(ref e) =>
                write_err!(f, "conversion of the `block_hash` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetBlockStatsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::BlockHash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetBlockStatsError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `ChainTips` type into the model type.
#[derive(Debug)]
pub enum ChainTipsError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `hash` field failed.
    Hash(hex::HexToArrayError),
}

impl fmt::Display for ChainTipsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ChainTipsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::Hash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for ChainTipsError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetChainTxStats` type into the model type.
#[derive(Debug)]
pub enum GetChainTxStatsError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `window_final_block_hash` field failed.
    WindowFinalBlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetChainTxStatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::WindowFinalBlockHash(ref e) =>
                write_err!(f, "conversion of the `window_final_block_hash` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetChainTxStatsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::WindowFinalBlockHash(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetChainTxStatsError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `MapMempoolEntry` into the model type.
#[derive(Debug)]
pub enum MapMempoolEntryError {
    /// Conversion of a `txid` failed.
    Txid(hex::HexToArrayError),
    /// Conversion of a `MempoolEntry` value inside a map failed.
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

/// Error when converting a `MempoolEntry` type into the model type.
#[derive(Debug)]
pub enum MempoolEntryError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the `wtxid` field failed.
    Wtxid(hex::HexToArrayError),
    /// Conversion of the `MempoolEntryFees` type failed.
    Fees(MempoolEntryFeesError),
    /// Conversion of the `depends` field failed.
    Depends(hex::HexToArrayError),
    /// Conversion of the `spent_by` field failed.
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
}

impl fmt::Display for MempoolEntryFeesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Base(ref e) => write_err!(f, "conversion of the `base` field failed"; e),
            Self::Modified(ref e) => write_err!(f, "conversion of the `modified` field failed"; e),
            Self::Ancestor(ref e) => write_err!(f, "conversion of the `ancestor` field failed"; e),
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
            Self::Ancestor(ref e) => Some(e),
            Self::Descendant(ref e) => Some(e),
        }
    }
}

/// Error when converting a `GetMempoolInfo` type into the model type.
#[derive(Debug)]
pub enum GetMempoolInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of a fee rate failed.
    FeeRate(ParseAmountError),
}

impl fmt::Display for GetMempoolInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::FeeRate(ref e) => write_err!(f, "fee rate"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetMempoolInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::FeeRate(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetMempoolInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

impl From<ParseAmountError> for GetMempoolInfoError {
    fn from(e: ParseAmountError) -> Self { Self::FeeRate(e) }
}

/// Error when converting a `GetTxOut` type into the model type.
///
/// Note that variants in this type are not named in the usual fashion. The `ScriptBuf` and
/// `Address` variants are named after the functions on [`crate::ScriptPubkey`].
#[derive(Debug)]
pub enum GetTxOutError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `best_block` field failed.
    BestBlock(hex::HexToArrayError),
    /// Conversion of the transaction `value` field failed.
    Value(amount::ParseAmountError),
    /// Conversion of the `ScriptPubkey` hex to a `ScriptBuf` failed.
    ScriptBuf(hex::HexToBytesError),
    /// Conversion of the `ScriptPubkey` `address` field failed.
    Address(address::ParseError),
}

impl fmt::Display for GetTxOutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::BestBlock(ref e) =>
                write_err!(f, "conversion of the `beast_block` field failed"; e),
            Self::Value(ref e) => write_err!(f, "conversion of the `value` field failed"; e),
            Self::ScriptBuf(ref e) =>
                write_err!(f, "conversion of the `ScriptPubkey` hex to a `ScriptBuf` failed"; e),
            Self::Address(ref e) =>
                write_err!(f, "conversion of the `ScriptPubkey` `address` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetTxOutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::BestBlock(ref e) => Some(e),
            Self::Value(ref e) => Some(e),
            Self::ScriptBuf(ref e) => Some(e),
            Self::Address(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetTxOutError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error when converting a `GetTxOutSetInfo` type into the model type.
#[derive(Debug)]
pub enum GetTxOutSetInfoError {
    /// Conversion of numeric type to expected type failed.
    Numeric(NumericError),
    /// Conversion of the transaction `best_block` field failed.
    BestBlock(hex::HexToArrayError),
    /// Conversion of the transaction `total_amount` field failed.
    TotalAmount(amount::ParseAmountError),
}

impl fmt::Display for GetTxOutSetInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Numeric(ref e) => write_err!(f, "numeric"; e),
            Self::BestBlock(ref e) =>
                write_err!(f, "conversion of the `best_block` field failed"; e),
            Self::TotalAmount(ref e) =>
                write_err!(f, "conversion of the `total_amount` field failed"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GetTxOutSetInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Numeric(ref e) => Some(e),
            Self::BestBlock(ref e) => Some(e),
            Self::TotalAmount(ref e) => Some(e),
        }
    }
}

impl From<NumericError> for GetTxOutSetInfoError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}

/// Error that can occur when converting the result of the `scantxoutset`
/// RPC method into the strongly typed `ScanTxOutSet` model.
#[derive(Debug)]
pub enum ScanTxOutSetError {
    /// Failed to parse the `bestblock` field (a block hash) from a hex string.
    BestBlockHash(hex::HexToArrayError),
    /// Failed to parse the `blockhash` field (per unspent) from a hex string.
    BlockHash(hex::HexToArrayError),
    /// Failed to parse the `txid` field from a hex string.
    Txid(hex::HexToArrayError),
    /// Failed to parse the `scriptPubKey` field from hex string into script bytes.
    ScriptPubKey(hex::HexToBytesError),
    /// Failed to convert the `total_amount` field to a valid Bitcoin amount.
    TotalAmount(amount::ParseAmountError),
    /// Failed to convert an `amount` field in an unspent output to a valid Bitcoin amount.
    Amount(amount::ParseAmountError),
    /// A numeric field could not be converted to the expected Rust numeric type.
    Numeric(NumericError),
}

impl fmt::Display for ScanTxOutSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ScanTxOutSetError::*;

        match self {
            BestBlockHash(e) =>
                write_err!(f, "conversion of the `bestblock` field failed"; e),
            BlockHash(e) =>
                write_err!(f, "conversion of the `blockhash` field failed"; e),
            Txid(e) =>
                write_err!(f, "conversion of the `txid` field failed"; e),
            ScriptPubKey(e) =>
                write_err!(f, "conversion of the `scriptPubKey` field failed"; e),
            TotalAmount(e) =>
                write_err!(f, "conversion of the `total_amount` field failed"; e),
            Amount(e) =>
                write_err!(f, "conversion of the `amount` field failed"; e),
            Numeric(e) =>
                write_err!(f, "numeric"; e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ScanTxOutSetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use ScanTxOutSetError::*;

        match self {
            BestBlockHash(e) => Some(e),
            BlockHash(e) => Some(e),
            Txid(e) => Some(e),
            ScriptPubKey(e) => Some(e),
            TotalAmount(e) => Some(e),
            Amount(e) => Some(e),
            Numeric(e) => Some(e),
        }
    }
}

impl From<NumericError> for ScanTxOutSetError {
    fn from(e: NumericError) -> Self { Self::Numeric(e) }
}
